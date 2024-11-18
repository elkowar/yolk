use regex::Regex;

use crate::COMMENT_START;

pub struct RenderingContext {
    pub(crate) comment_prefix: String,
}

impl RenderingContext {
    pub fn enabled_str(&self, s: &str) -> String {
        let re = Regex::new(&format!("{}{}", self.comment_prefix, COMMENT_START)).unwrap();
        let lines: Vec<_> = s.split('\n').map(|line| re.replace_all(line, "")).collect();
        lines.join("\n")
    }
    pub fn disabled_str(&self, s: &str) -> String {
        let re = Regex::new(&format!("^\\s*{}{}", self.comment_prefix, COMMENT_START)).unwrap();
        let lines: Vec<_> = s
            .split('\n')
            .map(|line| {
                if !re.is_match(line) && !line.is_empty() {
                    let indent: String = line
                        .chars()
                        .take_while(|&c| c == ' ' || c == '\t')
                        .collect();
                    format!(
                        "{}{}{}{}",
                        indent,
                        self.comment_prefix,
                        COMMENT_START,
                        line.trim_start_matches(|c| c == ' ' || c == '\t')
                    )
                } else {
                    line.to_string()
                }
            })
            .collect();
        lines.join("\n")
    }
}
