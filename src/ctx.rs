use std::sync::Arc;

use anyhow::Result;
use regex::Regex;
use rune::runtime::Object;

use crate::COMMENT_START;

pub fn foo() -> Result<()> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources! {
        entry => {
            pub fn calc(input) {
                dbg(input["key"]);
                input["key"] = "World";
                input
            }
        }
    };

    let mut diagnostics = rune::Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer =
            rune::termcolor::StandardStream::stderr(rune::termcolor::ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;

    let mut vm = rune::Vm::new(runtime, Arc::new(unit));

    let mut object = Object::new();
    object.insert(
        rune::alloc::String::try_from("key")?,
        rune::to_value(42i64)?,
    )?;

    let output = vm.call(["calc"], (object,))?;
    let output: Object = rune::from_value(output)?;

    println!("{:?}", output.get("key"));
    Ok(())
}

pub struct Context {
    pub(crate) comment_prefix: String,
}

impl Context {
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
