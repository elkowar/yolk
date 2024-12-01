use std::borrow::Cow;

use regex::Regex;

use crate::templating::COMMENT_START;

use super::linewise::ParsedLine;

const PREFIX_COMMENT_SYMBOLS: [&str; 5] = ["//", "#", "--", ";", "%"];
const CIRCUMFIX_COMMENT_SYMBOLS: [(&str, &str); 3] = [("/*", "*/"), ("<!--", "-->"), ("{-", "-}")];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommentStyle {
    Prefix(String),
    Circumfix(String, String),
}

impl CommentStyle {
    #[allow(unused)]
    pub fn prefix(left: &str) -> Self {
        CommentStyle::Prefix(left.to_string())
    }
    #[allow(unused)]
    pub fn circumfix(left: &str, right: &str) -> Self {
        CommentStyle::Circumfix(left.to_string(), right.to_string())
    }
    pub fn left(&self) -> &str {
        match self {
            CommentStyle::Prefix(left) => left,
            CommentStyle::Circumfix(left, _) => left,
        }
    }
    pub fn enable_line<'a>(&self, line: &'a str) -> Cow<'a, str> {
        // TODO: Creating a regex every time here is horrible
        let left = self.left();
        let re = Regex::new(&format!(
            "{}{}",
            regex::escape(left),
            regex::escape(COMMENT_START)
        ))
        .unwrap();
        let left_done = re.replace_all(line, "");
        if let CommentStyle::Circumfix(_, right) = self {
            let re_right = Regex::new(&regex::escape(right)).unwrap();
            Cow::Owned(re_right.replace_all(&left_done, "").to_string())
        } else {
            left_done
        }
    }

    pub fn is_disabled(&self, line: &str) -> bool {
        let re = match self {
            CommentStyle::Prefix(left) => {
                format!("^.*{}{}", regex::escape(left), regex::escape(COMMENT_START))
            }
            CommentStyle::Circumfix(left, right) => format!(
                "^.*{}{}.*{}",
                regex::escape(left),
                regex::escape(COMMENT_START),
                regex::escape(right)
            ),
        };
        Regex::new(&re).unwrap().is_match(line)
    }

    pub fn disable_line<'a>(&self, line: &'a str) -> Cow<'a, str> {
        if self.is_disabled(line) || line.trim().is_empty() {
            return line.into();
        }
        let left = self.left();
        let re = Regex::new("^(\\s*)(.*)$").unwrap();
        let (indent, remaining_line) = re
            .captures(line)
            .and_then(|x| (x.get(1).zip(x.get(2))))
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .unwrap_or_default();
        let right = match self {
            CommentStyle::Prefix(_) => "".to_string(),
            CommentStyle::Circumfix(_, right) => right.to_string(),
        };
        format!("{indent}{left}{COMMENT_START}{remaining_line}{right}",).into()
    }
}

pub fn infer_comment_syntax(line: &ParsedLine<'_>) -> Option<CommentStyle> {
    let (left, right) = match line {
        ParsedLine::MultiLineTag { line, .. }
        | ParsedLine::NextLineTag { line, .. }
        | ParsedLine::InlineTag { line, .. } => (line.left, line.right),
        ParsedLine::Raw(_) => return None,
    };

    for (prefix, postfix) in &CIRCUMFIX_COMMENT_SYMBOLS {
        if left.trim_end().ends_with(prefix) && right.trim_start().starts_with(postfix) {
            return Some(CommentStyle::Circumfix(
                prefix.to_string(),
                postfix.to_string(),
            ));
        }
    }
    for prefix in &PREFIX_COMMENT_SYMBOLS {
        if left.trim_end().ends_with(prefix) {
            return Some(CommentStyle::Prefix(prefix.to_string()));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use pest::Span;

    use crate::templating::{
        parser::{
            comment_style::infer_comment_syntax,
            linewise::{ParsedLine, TagKind},
        },
        TaggedLine,
    };

    use super::CommentStyle;

    #[track_caller]
    fn assert_roundtrip_works(start: &str, expected_disabled: &str, comment_style: CommentStyle) {
        let disabled = comment_style.disable_line(start);
        let enabled = comment_style.enable_line(disabled.as_ref());
        assert_eq!(expected_disabled, disabled);
        assert_eq!(start, enabled);
    }

    #[test]
    pub fn test_disable_enable_roundtrip() {
        // This roundtrip (disable -> enable) should _always_ be identity
        assert_roundtrip_works("  foo", "  #<yolk> foo", CommentStyle::prefix("#"));

        assert_roundtrip_works(
            "  foo",
            "  /*<yolk> foo*/",
            CommentStyle::circumfix("/*", "*/"),
        );
    }

    #[test]
    pub fn test_enable_idempodent() {
        let assert_idempotent = |comment_style: CommentStyle, line: &str| {
            let enabled = comment_style.enable_line(line);
            let enabled_again = comment_style.enable_line(enabled.as_ref());
            assert_eq!(enabled, enabled_again);
        };
        assert_idempotent(CommentStyle::prefix("#"), "\tfoo");
        assert_idempotent(CommentStyle::prefix("#"), "foo  ");
        assert_idempotent(CommentStyle::circumfix("/*", "*/"), "  foo  ");
    }

    #[test]
    pub fn test_disable_idempodent() {
        let assert_idempotent = |comment_style: CommentStyle, line: &str| {
            let disabled = comment_style.disable_line(line);
            let disabled_again = comment_style.disable_line(disabled.as_ref());
            assert_eq!(disabled, disabled_again);
        };
        assert_idempotent(CommentStyle::prefix("#"), "\tfoo");
        assert_idempotent(CommentStyle::prefix("#"), "foo  ");
        assert_idempotent(CommentStyle::circumfix("/*", "*/"), "  foo  ");
    }

    #[test]
    pub fn test_infer_comment_syntax() {
        let parsed_line = ParsedLine::InlineTag {
            line: TaggedLine {
                full_line: Span::new("# {# foo #}", 0, 10).unwrap(),
                tag: "{# foo #}",
                left: "# ",
                right: "",
            },
            kind: TagKind::Regular("foo"),
        };
        assert_eq!(
            infer_comment_syntax(&parsed_line),
            Some(CommentStyle::prefix("#"))
        );

        let parsed_line = ParsedLine::InlineTag {
            line: TaggedLine {
                full_line: Span::new("/* {# foo #} */", 0, 14).unwrap(),
                tag: "{# foo #}",
                left: "/* ",
                right: " */",
            },
            kind: TagKind::Regular("foo"),
        };
        assert_eq!(
            infer_comment_syntax(&parsed_line),
            Some(CommentStyle::circumfix("/*", "*/"))
        );
    }
}
