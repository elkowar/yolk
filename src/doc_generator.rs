use std::{collections::HashMap, sync::Arc};

use miette::IntoDiagnostic;
use rhai_autodocs::{item::Item, module::Documentation};

use crate::{
    util::create_regex,
    yolk::{EvalMode, Yolk},
};

pub fn generate_docs(yolk: Yolk) -> miette::Result<HashMap<String, String>> {
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(EvalMode::Canonical)?;

    let docs = rhai_autodocs::export::options()
        .include_standard_packages(false)
        .format_sections_with(rhai_autodocs::export::SectionFormat::Rust)
        .export(&mut eval_ctx.engine_mut())
        .into_diagnostic()?;

    let mut docs = render_docs(&docs);

    let mut empty_module = rhai::Module::new();
    empty_module.set_doc(indoc::indoc! { "
        # Rhai Standard Library builtins
        Rhai standard library functions.

        Note that the typesignatures here do look a bit weird.
        This is simply a result of how we generate the documentation,
        and can't easily be improved.

        Just try your best to ignore it...
        " });
    let mut empty_engine = rhai::Engine::new();
    empty_engine.register_global_module(Arc::new(empty_module));

    let mut stdlib_docs = rhai_autodocs::export::options()
        .include_standard_packages(true)
        .format_sections_with(rhai_autodocs::export::SectionFormat::Rust)
        .export(&mut empty_engine)
        .into_diagnostic()?;

    stdlib_docs.items.retain(|x| match x {
        Item::Function {
            root_metadata: _,
            metadata: _,
            name,
            index: _,
        } => {
            name.starts_with(|x| char::is_ascii_alphanumeric(&x) && x != '?')
                && !name.starts_with("i8.")
                && !name.starts_with("i16.")
                && !name.starts_with("i32.")
                && !name.starts_with("i64.")
                && !name.starts_with("i128.")
                && !name.starts_with("u8.")
                && !name.starts_with("u16.")
                && !name.starts_with("u32.")
                && !name.starts_with("u64.")
                && !name.starts_with("u128.")
                && !name.starts_with("Range.")
                && !name.starts_with("RangeInclusive.")
                && !name.starts_with("FnPtr.")
        }

        Item::CustomType { .. } => true,
    });

    docs.extend(render_docs(&stdlib_docs));
    Ok(docs)
}

fn render_docs(docs: &Documentation) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for module in &docs.sub_modules {
        let entries = render_docs(module);
        map.extend(entries);
    }

    let mut mine = format!(
        "{}\n\n---\n\n**namespace**: `{}`\n\n---\n\n",
        docs.documentation,
        docs.namespace.trim_start_matches("global/")
    );

    for item in &docs.items {
        mine.push_str(&format!("{}\n---\n", render_item_docs(item)))
    }
    map.insert(docs.name.to_string(), mine);

    map
}

fn render_item_docs(item: &Item) -> String {
    match item {
        Item::Function {
            ref root_metadata,
            ref metadata,
            name,
            index: _,
        } => {
            // dbg!(&metadata);
            let docs = root_metadata
                .doc_comments
                .clone()
                .unwrap_or_default()
                .join("\n\n")
                .lines()
                .map(|x| x.trim().trim_start_matches("///").trim().to_string())
                .map(|x| {
                    create_regex("^# ")
                        .unwrap()
                        .replace_all(&x, "#### ")
                        .to_string()
                })
                // .map(|x| format!("> {x}"))
                .collect::<Vec<String>>()
                .join("\n");
            indoc::formatdoc! {r#"
                    ## {name}

                    ```rust,ignore
                    {}
                    ```

                    {}
                "#,
                metadata.iter().map(|x| x.signature.to_string()).collect::<Vec<String>>().join("\n"),
                docs.lines().map(|x| format!("> {x}")).collect::<Vec<_>>().join("\n"),
            }
        }
        // TODO: render these, once we have some
        Item::CustomType { .. } => "custom type".to_string(),
    }
}
