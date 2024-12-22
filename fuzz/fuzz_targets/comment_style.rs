#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|x: (yolk::templating::element::Element, String)| {
    let (element, data) = x;
    let comment_style = yolk::templating::comment_style::CommentStyle::try_infer(&element);
    if let Some(comment_style) = comment_style {
        let _ = comment_style.toggle_string(&data, true);
        let _ = comment_style.toggle_string(&data, false);
    }
});
