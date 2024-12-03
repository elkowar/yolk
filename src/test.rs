use assert_fs::prelude::{FileWriteStr as _, PathChild as _};
use testresult::TestResult;

use crate::{yolk::Yolk, yolk_paths::YolkPaths};

#[test]
pub fn test_custom_functions_in_text_transformer_tag() -> TestResult {
    let home = assert_fs::TempDir::new()?;
    let yolk = Yolk::new(YolkPaths::new(home.join("yolk"), home.to_path_buf()));
    yolk.init_yolk()?;
    home.child("yolk/yolk.lua").write_str(indoc::indoc! {r#"
        data = if LOCAL then {} else {}
        function scream() return YOLK_TEXT:upper() end
    "#})?;
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)?;
    assert_eq!(
        "TEST{< scream() >}",
        yolk.eval_template(&mut eval_ctx, "test{< scream() >}")?
    );

    Ok(())
}
