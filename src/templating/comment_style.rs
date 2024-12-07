use std::borrow::Cow;

use cached::proc_macro::cached;
use regex::Regex;

use crate::templating::COMMENT_START;

use super::element::{Block, Element};

const PREFIX_COMMENT_SYMBOLS: [&str; 5] = ["//", "#", "--", ";", "%"];
const CIRCUMFIX_COMMENT_SYMBOLS: [(&str, &str); 3] = [("/*", "*/"), ("<!--", "-->"), ("{-", "-}")];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommentStyle {
    Prefix(String),
    Circumfix(String, String),
}

impl Default for CommentStyle {
    fn default() -> Self {
        CommentStyle::Prefix("#".to_string())
    }
}

impl CommentStyle {
    /// Try to infer the CommentStyle from a line
    pub fn try_infer(element: &Element<'_>) -> Option<Self> {
        let line = match &element {
            Element::Inline { line, .. }
            | Element::NextLine {
                tagged_line: line, ..
            }
            | Element::MultiLine {
                block: Block {
                    tagged_line: line, ..
                },
                ..
            } => &line,
            Element::Conditional { blocks, .. } => &blocks.first()?.tagged_line,
            Element::Plain(_) => return None,
        };
        let (left, right) = (line.left, line.right);

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

    pub fn try_infer_from_elements(elements: &[Element<'_>]) -> Option<Self> {
        for e in elements {
            if let Some(style) = Self::try_infer(e) {
                return Some(style);
            }
        }
        None
    }

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
        let re = create_regex(format!(
            "{}{}",
            regex::escape(left),
            regex::escape(COMMENT_START)
        ))
        .unwrap();
        let left_done = re.replace_all(line, "");
        if let CommentStyle::Circumfix(_, right) = self {
            let re_right = create_regex(regex::escape(right)).unwrap();
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
        create_regex(re).unwrap().is_match(line)
    }

    pub fn disable_line<'a>(&self, line: &'a str) -> Cow<'a, str> {
        if self.is_disabled(line) || line.trim().is_empty() {
            return line.into();
        }
        let left = self.left();
        let re = create_regex("^(\\s*)(.*)$".to_string()).unwrap();
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

/// Same as [`Regex::new`], but with caching.
/// This is used so we don't have to re-create the same regex for each instance of `CommentStyle`
#[cached]
fn create_regex(s: String) -> Result<Regex, regex::Error> {
    Regex::new(&s)
}

#[cfg(test)]
mod test {
    use testresult::TestResult;

    use crate::templating::element::Element;

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
    pub fn test_infer_comment_syntax() -> TestResult {
        assert_eq!(
            CommentStyle::try_infer(&Element::try_from_str("# {< foo >}")?),
            Some(CommentStyle::prefix("#"))
        );
        assert_eq!(
            CommentStyle::try_infer(&Element::try_from_str("/* {< foo >} */")?),
            Some(CommentStyle::circumfix("/*", "*/"))
        );
        Ok(())
    }
}
