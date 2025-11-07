use std::path::PathBuf;

use crate::{
    eggs_config::{DeploymentStrategy, ShellHooks},
    util::test_util::{setup_and_init_test_yolk, TestResult},
    yolk::EvalMode,
    yolk_paths::Egg,
};

#[cfg(not(windows))]
use crate::util::test_util;
use assert_fs::{
    assert::PathAssert,
    prelude::{FileWriteStr, PathChild, PathCreateDir},
    TempDir,
};
use p::path::{exists, is_dir, is_symlink};
use predicates::prelude::PredicateBooleanExt;
use predicates::{self as p};
use pretty_assertions::assert_str_eq;
use test_log::test;

use crate::eggs_config::EggConfig;

// fn is_direct_file(
// ) -> AndPredicate<FileTypePredicate, NotPredicate<FileTypePredicate, Path>, Path> {
//     is_file().and(is_symlink().not())
// }

#[test]
fn test_deploy_egg_put_mode() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("foo/foo.toml").write_str("")?;
    eggs.child("foo/thing/thing.toml").write_str("")?;
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::default()
            .with_strategy(DeploymentStrategy::Put)
            .with_target("foo.toml", home.child("foo.toml"))
            .with_target("thing", home.child("thing")),
    )?)?;
    home.child("foo.toml").assert(is_symlink());
    home.child("thing").assert(is_symlink());
    Ok(())
}

#[test]
fn test_egg_post_deploy_hooks() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("foo/foo.toml").write_str("")?;
    let egg = EggConfig::default()
        .with_strategy(DeploymentStrategy::Put)
        .with_target("foo.toml", home.child("foo.toml"))
        .with_unsafe_hooks(ShellHooks {
            post_deploy: Some(format!("touch {}/post_deploy_ran", home.display())),
            post_undeploy: Some(format!("touch {}/post_undeploy_ran", home.display())),
            pre_deploy: Some(format!("touch {}/pre_deploy_ran", home.display())),
            pre_undeploy: Some(format!("touch {}/pre_undeploy_ran", home.display())),
        });
    let egg_again = egg.clone().with_unsafe_hooks(ShellHooks {
        post_deploy: Some(format!("touch {}/post_deploy_ran_again", home.display())),
        post_undeploy: Some(format!("touch {}/post_undeploy_ran_again", home.display())),
        pre_deploy: Some(format!("touch {}/pre_deploy_ran_again", home.display())),
        pre_undeploy: Some(format!("touch {}/pre_undeploy_ran_again", home.display())),
    });
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg.clone(),
    )?)?;
    home.child("pre_deploy_ran").assert(exists());
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg_again.clone(),
    )?)?;
    home.child("pre_deploy_ran_again").assert(exists().not());

    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg.clone(),
    )?)?;
    home.child("post_deploy_ran").assert(exists());
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg_again.clone(),
    )?)?;
    home.child("post_deploy_ran_again").assert(exists().not());

    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg.clone().with_enabled(false),
    )?)?;
    home.child("pre_undeploy_ran").assert(exists()); //ERROR: why
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg_again.clone().with_enabled(false),
    )?)?;
    home.child("pre_undeploy_ran_again").assert(exists().not());

    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg.clone().with_enabled(false),
    )?)?;
    home.child("post_undeploy_ran").assert(exists());
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        egg_again.clone().with_enabled(false),
    )?)?;
    home.child("post_undeploy_ran_again").assert(exists().not());

    Ok(())
}

#[test]
fn test_deploy_merge_mode() -> TestResult {
    cov_mark::check_count!(deploy_merge, 1);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child(".config").create_dir_all()?;
    eggs.child("bar/.config/thing.toml").write_str("")?;
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("bar").to_path_buf(),
        EggConfig::new_merge(".", &home),
    )?)?;

    home.child(".config").assert(is_dir());
    home.child(".config/thing.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_deploy_put_mode() -> TestResult {
    cov_mark::check_count!(deploy_put_symlink_failed, 0);
    cov_mark::check_count!(deploy_put, 2);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("foo/foo.toml").write_str("")?;
    eggs.child("foo/thing/thing.toml").write_str("")?;
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::default()
            .with_target("foo.toml", home.child("foo.toml"))
            .with_target("thing", home.child("thing"))
            .with_strategy(DeploymentStrategy::Put),
    )?)?;
    home.child("foo.toml").assert(is_symlink());
    home.child("thing").assert(is_symlink());
    Ok(())
}

#[test]
fn test_moving_put_deploy_cleans_up_old_symlinks() -> TestResult {
    cov_mark::check_count!(delete_stale_symlink, 2);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("foo/foo.toml").write_str("")?;
    let mut egg = Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::new("foo.toml", home.child("foo.toml")),
    )?;
    yolk.sync_egg_deployment(&egg)?;
    home.child("foo.toml").assert(is_symlink());

    // now we sync again, to a different location
    *egg.config_mut() = EggConfig::new("foo.toml", home.child("bar.toml"));
    yolk.sync_egg_deployment(&egg)?;
    home.child("bar.toml").assert(is_symlink());
    home.child("foo.toml").assert(exists().not());

    // and back, just to be sure
    *egg.config_mut() = EggConfig::new("foo.toml", home.child("foo.toml"));
    yolk.sync_egg_deployment(&egg)?;
    home.child("foo.toml").assert(is_symlink());
    home.child("bar.toml").assert(exists().not());
    Ok(())
}

#[test]
fn test_moving_merge_deploy_cleans_up_old_symlinks() -> TestResult {
    cov_mark::check_count!(delete_stale_symlink, 2);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child("a").create_dir_all()?;
    home.child("b").create_dir_all()?;
    eggs.child("foo/foo/foo.toml").write_str("")?;
    let mut egg = Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::new_merge(".", home.child("a")),
    )?;
    yolk.sync_egg_deployment(&egg)?;
    home.child("a/foo").assert(is_symlink());

    // now we sync again, to a different location
    *egg.config_mut() = EggConfig::new_merge(".", home.child("b"));
    yolk.sync_egg_deployment(&egg)?;
    home.child("b/foo").assert(is_symlink());
    home.child("a/foo").assert(exists().not());

    // and back, just to be sure
    *egg.config_mut() = EggConfig::new_merge(".", home.child("a"));
    yolk.sync_egg_deployment(&egg)?;
    home.child("a/foo").assert(is_symlink());
    home.child("b/foo").assert(exists().not());
    Ok(())
}

#[test]
fn test_deploy_outside_of_home() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    let other_dir = TempDir::new()?;
    eggs.child("foo/foo.toml").write_str("")?;
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::new("foo.toml", other_dir.child("foo.toml"))
            .with_strategy(DeploymentStrategy::Put),
    )?)?;
    other_dir.child("foo.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_deploy_put_mode_fails_with_stowy_usage() -> TestResult {
    cov_mark::check_count!(deploy_put, 1);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child(".config").create_dir_all()?;
    eggs.child("bar/.config/thing.toml").write_str("")?;
    let result = yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("bar").to_path_buf(),
        EggConfig::new(".", &home),
    )?);
    home.child(".config").assert(is_dir());
    home.child(".config/thing.toml").assert(exists().not());
    assert!(format!("{:?}", miette::Report::from(result.unwrap_err()))
        .contains("Failed to create symlink"));
    Ok(())
}

#[test]
fn test_deploy_put_creates_parent_dir() -> TestResult {
    cov_mark::check_count!(deploy_put, 1);
    cov_mark::check_count!(deploy_put_symlink_failed, 0);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("foo/foo.toml").write_str("")?;
    yolk.sync_egg_deployment(&Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::new("foo.toml", home.child("a/a/a/foo.toml")),
    )?)?;
    home.child("a/a/a").assert(is_dir().and(is_symlink().not()));
    home.child("a/a/a/foo.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_undeploy() -> TestResult {
    cov_mark::check!(undeploy);
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child(".config").create_dir_all()?;
    eggs.child("foo/foo.toml").write_str("")?;
    eggs.child("bar/.config/thing.toml").write_str("")?;

    let mut egg = Egg::open(
        home.to_path_buf(),
        eggs.child("foo").to_path_buf(),
        EggConfig::new_merge("foo.toml", home.child("foo.toml")),
    )?;

    yolk.sync_egg_deployment(&egg)?;
    home.child("foo.toml").assert(is_symlink());

    egg.config_mut().enabled = false;
    yolk.sync_egg_deployment(&egg)?;
    home.child("foo.toml").assert(exists().not());

    // Verify stow-style usage works
    home.child(".config").create_dir_all()?;
    eggs.child("bar/.config/thing.toml").write_str("")?;
    let mut egg = Egg::open(
        home.to_path_buf(),
        eggs.child("bar").to_path_buf(),
        EggConfig::new_merge(".", &home),
    )?;
    yolk.sync_egg_deployment(&egg)?;
    home.child(".config/thing.toml").assert(is_symlink());
    egg.config_mut().enabled = false;
    yolk.sync_egg_deployment(&egg)?;
    home.child(".config/thing.toml").assert(exists().not());
    home.child(".config").assert(is_dir());
    Ok(())
}

/// Test that sync_egg_deployment after moving something in the egg dir and changing the deployment configuration,
/// When encountering old, dead symlinks into the same egg, deletes those dead symlinks.
#[test]
fn test_deploy_after_moving_overrides_old_dead_symlinks() -> TestResult {
    cov_mark::check_count!(remove_dead_symlink, 1);

    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    // We start out with a stow-style situation, where we have eggs/alacritty/.config/alacritty/alacritty.toml
    home.child(".config").create_dir_all()?;
    eggs.child("alacritty/.config/alacritty/alacritty.toml")
        .write_str("")?;
    let mut egg = Egg::open(
        home.to_path_buf(),
        eggs.child("alacritty").to_path_buf(),
        EggConfig::new_merge(".", &home),
    )?;
    yolk.sync_egg_deployment(&egg)?;
    home.child(".config/alacritty").assert(is_symlink());

    // now we want to change to a simpler structure, where we explicitly declare the target dir for the files.
    // the user first moves the files inside the egg dir
    fs_err::rename(
        eggs.child("alacritty/.config/alacritty/alacritty.toml"),
        eggs.child("alacritty/alacritty.toml"),
    )?;
    // deletes the now empty .config structure
    fs_err::remove_dir(eggs.child("alacritty/.config/alacritty/"))?;
    fs_err::remove_dir(eggs.child("alacritty/.config/"))?;

    // He now updates his egg configuration to make the alacritty egg dir deploy to .config/alacritty
    egg.config_mut().targets = maplit::hashmap! {
        PathBuf::from(".") => PathBuf::from(".config/alacritty/")
    };
    // And syncs
    yolk.sync_egg_deployment(&egg)?;

    home.child(".config/alacritty").assert(is_symlink());
    Ok(())
}

#[test]
fn test_syncing() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    let foo_toml_initial = "{# data.value #}\nfoo";
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL { #{value: "local"} } else { #{value: "canonical"} };
            export let eggs = #{foo: #{ targets: `~`, strategy: "merge"}};
        "#})?;
    eggs.child("foo/foo.toml").write_str(foo_toml_initial)?;
    yolk.sync_to_mode(EvalMode::Local, true)?;
    // No template set in eggs.rhai, so no templating should happen
    home.child("foo.toml").assert(is_symlink());
    eggs.child("foo/foo.toml").assert(foo_toml_initial);

    // Now we make the file a template, so it should be updated
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "local"}} else {#{value: "canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
        "#})?;

    yolk.sync_to_mode(EvalMode::Local, true)?;
    eggs.child("foo/foo.toml").assert("{# data.value #}\nlocal");

    // Update the state, to see if applying again just works :tm:
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "new local"}} else {#{value: "new canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
        "#})?;
    yolk.sync_to_mode(EvalMode::Local, true)?;
    home.child("foo.toml").assert("{# data.value #}\nnew local");
    yolk.with_canonical_state(|| {
        eggs.child("foo/foo.toml")
            .assert("{# data.value #}\nnew canonical");
        Ok(())
    })?;
    Ok(())
}

#[test]
#[cfg(not(windows))]
fn test_sync_eggs_continues_after_failure() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            export let eggs = #{
                foo: #{ targets: `~`, strategy: "merge", templates: ["foo"]},
                bar: #{ targets: `~`, strategy: "merge", templates: ["bar"]},
            };
        "#})?;
    eggs.child("foo/foo").write_str(r#"{< invalid rhai >}"#)?;
    eggs.child("bar/bar").write_str(r#"foo # {<if false>}"#)?;
    let result = yolk.sync_to_mode(EvalMode::Local, true);
    eggs.child("foo/foo").assert(r#"{< invalid rhai >}"#);
    eggs.child("bar/bar")
        .assert(r#"#<yolk> foo # {<if false>}"#);
    assert!(test_util::render_error(result.unwrap_err()).contains("Syntax error"));
    Ok(())
}

#[test]
fn test_access_sysinfo() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    home.child("yolk/yolk.rhai").write_str(
        r#"
            export const hostname = SYSTEM.hostname;
            export let eggs = #{foo: #{targets: `~/foo`, templates: ["foo.toml"]}};
        "#,
    )?;
    eggs.child("foo/foo.toml")
        .write_str("{< `host=${hostname}|${SYSTEM.hostname}` >}")?;
    yolk.sync_to_mode(EvalMode::Local, true)?;
    eggs.child("foo/foo.toml").assert(
        "host=canonical-hostname|canonical-hostname{< `host=${hostname}|${SYSTEM.hostname}` >}",
    );
    Ok(())
}

#[test]
fn test_imports_work_in_yolk_rhai() -> TestResult {
    let (home, yolk, _) = setup_and_init_test_yolk()?;
    home.child("yolk/foo.rhai").write_str(indoc::indoc! {r#"
            fn some_function() { 1 }
            export let some_value = 1;
        "#})?;
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            import "foo" as foo;
            fn get_value() { foo::some_function() + foo::some_value }
        "#})?;
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_eq!(2, eval_ctx.eval_rhai::<i64>("get_value()")?);
    Ok(())
}

#[test]
pub fn test_custom_functions_in_text_transformer_tag() -> TestResult {
    let (home, yolk, _) = setup_and_init_test_yolk()?;
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            fn scream() { get_yolk_text().to_upper() }
        "#})?;
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_str_eq!(
        "TEST{< scream() >}",
        yolk.eval_template(&mut eval_ctx, "", "test{< scream() >}")?
    );

    Ok(())
}

#[test]
fn test_variable_and_import_in_text_transformer_tag() -> TestResult {
    let (home, yolk, _) = setup_and_init_test_yolk()?;
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            import "foo" as foo;
            export let some_value = "a";
        "#})?;
    home.child("yolk/foo.rhai").write_str(indoc::indoc! {r#"
            export let imported = "b";
        "#})?;
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_str_eq!(
        "foo=ab # {< replace_value(some_value + foo::imported) >}",
        yolk.eval_template(
            &mut eval_ctx,
            "",
            "foo=x # {< replace_value(some_value + foo::imported) >}"
        )?
    );
    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_syntax_error_in_yolk_rhai() -> TestResult {
    use crate::util::create_regex;

    let (home, yolk, _) = setup_and_init_test_yolk()?;
    home.child("yolk/yolk.rhai").write_str(indoc::indoc! {r#"
            fn foo(
        "#})?;
    insta::assert_snapshot!(yolk
        .prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)
        .map_err(|e| create_regex(r"\[.*.rhai:\d+:\d+]")
            .unwrap()
            .replace(&test_util::render_report(e), "[no-filename-in-test]")
            .to_string())
        .unwrap_err());

    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_deployment_error() -> TestResult {
    let (home, yolk, eggs) = setup_and_init_test_yolk()?;
    eggs.child("bar/file1").write_str("")?;
    eggs.child("bar/file2").write_str("")?;
    home.child("file1").write_str("")?;
    home.child("file2").write_str("")?;
    let egg = Egg::open(
        home.to_path_buf(),
        eggs.child("bar").to_path_buf(),
        EggConfig::default()
            .with_target("file1", home.child("file1"))
            .with_target("file2", home.child("file2")),
    )?;
    insta::with_settings!({filters => vec![
        (r"\.tmp[a-zA-Z0-9]{6}", "[tmp-dir]"),
        (r"file\d", "[filename]")
    ]}, {
        insta::assert_snapshot!(test_util::render_error(
            yolk.sync_egg_deployment(&egg).unwrap_err()
        ));
    });
    Ok(())
}

#[test]
pub fn test_only_active_sections_get_evaluated() -> TestResult {
    let (_home, yolk, _) = setup_and_init_test_yolk()?;
    let mut eval_ctx = yolk.prepare_eval_ctx_for_templates(EvalMode::Local)?;

    let template = indoc::indoc! {r#"
        {% if false %}
        {< bad_code() >}
        {% else %}
        {< "1" >}
        {% end %}
        {# if false #}
        {< bad_code() >}
    "#};
    assert_str_eq!(
        indoc::indoc! {r#"
            {% if false %}
            #<yolk> {< bad_code() >}
            {% else %}
            1{< "1" >}
            {% end %}
            {# if false #}
            #<yolk> {< bad_code() >}
        "#},
        yolk.eval_template(&mut eval_ctx, "", template)?
    );

    Ok(())
}
