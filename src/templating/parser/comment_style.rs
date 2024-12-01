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
    pub fn enable_line<'a>(&self, line: &'a str) -> Cow<'a, str> {
        match self {
            CommentStyle::Prefix(prefix) => {
                // TODO: Creating a regex every time here is horrible
                let re = Regex::new(&format!("{prefix}{COMMENT_START}")).unwrap();
                re.replace_all(line, "")
            }
            CommentStyle::Circumfix(left, right) => {
                let re_left = Regex::new(&format!("{left}{COMMENT_START}")).unwrap();
                let re_right = Regex::new(right).unwrap();
                let result = re_right.replace_all(line, "");
                Cow::Owned(re_left.replace_all(&result, "").to_string())
            }
        }
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
