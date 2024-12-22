#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: yolk::templating::document::Document| {
    let mut eval_ctx = yolk::script::eval_ctx::EvalCtx::new_empty();
    let _ = data.render(&mut eval_ctx);
});
