use std::borrow::Cow;

pub const COMMENT_START: &str = "<yolk> ";

use crate::util::create_regex;

use super::element::{Block, Element};

const PREFIX_COMMENT_SYMBOLS: [&str; 8] = ["//", "#", "--", ";", "%", "\"", "'", "rem"];
const CIRCUMFIX_COMMENT_SYMBOLS: [(&str, &str); 5] = [
    ("/*", "*/"),
    ("<!--", "-->"),
    ("{-", "-}"),
    ("--[[", "]]"),
    ("(", ")"),
];

#[derive(Debug, Clone, Eq, PartialEq, arbitrary::Arbitrary)]
pub enum CommentStyle {
    Prefix(String),
    Circumfix(String, String),
}

impl Default for CommentStyle {
    fn default() -> Self {
        CommentStyle::Prefix("#".to_string())
    }
}

// TODO: Technically, a lot of this could already be done in the parser
// We could parse the indent, and yolk-comment-start and end stuff during the main parsing phase already
// That would allow us to avoid having to regex here, which would potentially be noticably more performant,
// and maybe even more elegant.
// However, that would require the parser to be a lot more complex,
// as well as requiring us to add a lot more information into the parsed data structure.
// The performance benefits are likely more than negligible, so not worth it for now.

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
            } => line,
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

    pub fn toggle_string(&self, s: &str, enable: bool) -> String {
        // TODO: Technically this could return Cow<'_, str> instead, but that's hard
        s.split('\n')
            .map(|x| self.toggle_line(x, enable))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn toggle_line<'a>(&self, line: &'a str, enable: bool) -> Cow<'a, str> {
        if enable {
            self.enable_line(line)
        } else {
            self.disable_line(line)
        }
    }

    pub fn enable_line<'a>(&self, line: &'a str) -> Cow<'a, str> {
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
        let re = create_regex("^(\\s*)(.*)$").unwrap();
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

#[cfg(test)]
mod test {
    use crate::util::TestResult;

    use crate::templating::element::Element;

    use super::CommentStyle;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("  foo", "  #<yolk> foo", CommentStyle::prefix("#"))]
    #[case("  foo", "  /*<yolk> foo*/", CommentStyle::circumfix("/*", "*/"))]
    #[case("foo", "#<yolk> foo", CommentStyle::prefix("#"))]
    #[case("foo", "/*<yolk> foo*/", CommentStyle::circumfix("/*", "*/"))]
    fn test_disable_enable_roundtrip(
        #[case] start: &str,
        #[case] expected_disabled: &str,
        #[case] comment_style: CommentStyle,
    ) {
        let disabled = comment_style.disable_line(start);
        let enabled = comment_style.enable_line(disabled.as_ref());
        assert_eq!(expected_disabled, disabled);
        assert_eq!(start, enabled);
    }

    #[rstest]
    #[case(CommentStyle::prefix("#"), "\tfoo")]
    #[case(CommentStyle::prefix("#"), "foo  ")]
    #[case(CommentStyle::circumfix("/*", "*/"), "  foo  ")]
    fn test_enable_idempotent(#[case] comment_style: CommentStyle, #[case] line: &str) {
        let enabled = comment_style.enable_line(line);
        let enabled_again = comment_style.enable_line(enabled.as_ref());
        assert_eq!(enabled, enabled_again);
    }

    #[rstest]
    #[case(CommentStyle::prefix("#"), "\tfoo")]
    #[case(CommentStyle::prefix("#"), "foo  ")]
    #[case(CommentStyle::circumfix("/*", "*/"), "  foo  ")]
    fn test_disable_idempotent(#[case] comment_style: CommentStyle, #[case] line: &str) {
        let disabled = comment_style.disable_line(line);
        let disabled_again = comment_style.disable_line(disabled.as_ref());
        assert_eq!(disabled, disabled_again);
    }

    #[rstest]
    #[case("# {< foo >}", Some(CommentStyle::prefix("#")))]
    #[case("/* {< foo >} */", Some(CommentStyle::circumfix("/*", "*/")))]
    fn test_infer_comment_syntax(
        #[case] input: &str,
        #[case] expected: Option<CommentStyle>,
    ) -> TestResult {
        assert_eq!(
            CommentStyle::try_infer(&Element::try_from_str(input)?),
            expected
        );
        Ok(())
    }
}
