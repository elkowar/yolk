#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: String| {
    let result = yolk::templating::document::Document::parse_string_named("fuzz-input", &data);
    if let Ok(result) = result {
        let mut eval_ctx = yolk::script::eval_ctx::EvalCtx::new_empty();
        let _ = result.render(&mut eval_ctx);
    }
});
