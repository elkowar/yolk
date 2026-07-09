use std::path::PathBuf;

use crate::{
    eggs_config::{DeploymentStrategy, ShellHooks},
    util::test_util::{TestEnv, TestResult},
    yolk::EvalMode,
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
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    env.egg_file("foo/thing/thing.toml").write_str("")?;
    env.yolk().sync_egg_deployment(
        &env.open_egg(
            "foo",
            EggConfig::default()
                .with_strategy(DeploymentStrategy::Put)
                .with_target("foo.toml", env.home_file("foo.toml"))
                .with_target("thing", env.home_file("thing")),
        )?,
    )?;
    env.home_file("foo.toml").assert(is_symlink());
    env.home_file("thing").assert(is_symlink());
    Ok(())
}

#[test]
fn test_egg_post_deploy_hooks() -> TestResult {
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    let egg = EggConfig::default()
        .with_strategy(DeploymentStrategy::Put)
        .with_target("foo.toml", env.home_file("foo.toml"))
        .with_unsafe_hooks(ShellHooks {
            post_deploy: Some(format!("touch {}/post_deploy_ran", env.home.display())),
            post_undeploy: Some(format!("touch {}/post_undeploy_ran", env.home.display())),
            pre_deploy: Some(format!("touch {}/pre_deploy_ran", env.home.display())),
            pre_undeploy: Some(format!("touch {}/pre_undeploy_ran", env.home.display())),
        });
    let egg_again = egg.clone().with_unsafe_hooks(ShellHooks {
        post_deploy: Some(format!(
            "touch {}/post_deploy_ran_again",
            env.home.display()
        )),
        post_undeploy: Some(format!(
            "touch {}/post_undeploy_ran_again",
            env.home.display()
        )),
        pre_deploy: Some(format!("touch {}/pre_deploy_ran_again", env.home.display())),
        pre_undeploy: Some(format!(
            "touch {}/pre_undeploy_ran_again",
            env.home.display()
        )),
    });
    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg.clone())?)?;
    env.home_file("pre_deploy_ran").assert(exists());
    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg_again.clone())?)?;
    env.home_file("pre_deploy_ran_again").assert(exists().not());

    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg.clone())?)?;
    env.home_file("post_deploy_ran").assert(exists());
    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg_again.clone())?)?;
    env.home_file("post_deploy_ran_again")
        .assert(exists().not());

    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg.clone().with_enabled(false))?)?;
    env.home_file("pre_undeploy_ran").assert(exists());
    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg_again.clone().with_enabled(false))?)?;
    env.home_file("pre_undeploy_ran_again")
        .assert(exists().not());

    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg.clone().with_enabled(false))?)?;
    env.home_file("post_undeploy_ran").assert(exists());
    env.yolk()
        .sync_egg_deployment(&env.open_egg("foo", egg_again.clone().with_enabled(false))?)?;
    env.home_file("post_undeploy_ran_again")
        .assert(exists().not());

    Ok(())
}

#[test]
fn test_adopt_directory_moves_into_egg() -> TestResult {
    let env = TestEnv::init()?;
    env.home_file(".config/noctalia").create_dir_all()?;
    env.home_file(".config/noctalia/config.toml")
        .write_str("theme = 'dark'")?;
    let source = env.home_file(".config/noctalia");

    let targets = env
        .yolk()
        .adopt("noctalia".to_string(), source.to_path_buf())?;

    assert!(targets.is_empty());
    source.assert(exists().not());
    env.egg_file("noctalia/config.toml")
        .assert("theme = 'dark'");
    Ok(())
}

#[test]
fn test_adopt_file_moves_into_egg_and_returns_target_mapping() -> TestResult {
    use crate::util::PathExt as _;
    let env = TestEnv::init()?;
    env.home_file(".zshrc").write_str("source ~/.zprofile")?;
    let source = env.home_file(".zshrc");
    // adopt records the canonical absolute location the file was moved from, so
    // capture it before the move (afterwards `source` no longer exists).
    let expected_target = source.canonical()?;

    let targets = env.yolk().adopt("zsh".to_string(), source.to_path_buf())?;

    source.assert(exists().not());
    env.egg_file("zsh/.zshrc").assert("source ~/.zprofile");
    assert_eq!(
        targets,
        maplit::hashmap! { PathBuf::from(".zshrc") => expected_target },
    );
    Ok(())
}

#[test]
fn test_deploy_merge_mode() -> TestResult {
    cov_mark::check_count!(deploy_merge, 1);
    let env = TestEnv::init()?;
    env.home_file(".config").create_dir_all()?;
    env.egg_file("bar/.config/thing.toml").write_str("")?;
    env.yolk()
        .sync_egg_deployment(&env.open_egg("bar", EggConfig::new_merge(".", &env.home))?)?;

    env.home_file(".config").assert(is_dir());
    env.home_file(".config/thing.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_deploy_put_mode() -> TestResult {
    cov_mark::check_count!(deploy_put_symlink_failed, 0);
    cov_mark::check_count!(deploy_put, 2);
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    env.egg_file("foo/thing/thing.toml").write_str("")?;
    env.yolk().sync_egg_deployment(
        &env.open_egg(
            "foo",
            EggConfig::default()
                .with_target("foo.toml", env.home_file("foo.toml"))
                .with_target("thing", env.home_file("thing"))
                .with_strategy(DeploymentStrategy::Put),
        )?,
    )?;
    env.home_file("foo.toml").assert(is_symlink());
    env.home_file("thing").assert(is_symlink());
    Ok(())
}

#[test]
fn test_moving_put_deploy_cleans_up_old_symlinks() -> TestResult {
    cov_mark::check_count!(delete_stale_symlink, 2);
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    let mut egg = env.open_egg("foo", EggConfig::new("foo.toml", env.home_file("foo.toml")))?;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("foo.toml").assert(is_symlink());

    // now we sync again, to a different location
    *egg.config_mut() = EggConfig::new("foo.toml", env.home_file("bar.toml"));
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("bar.toml").assert(is_symlink());
    env.home_file("foo.toml").assert(exists().not());

    // and back, just to be sure
    *egg.config_mut() = EggConfig::new("foo.toml", env.home_file("foo.toml"));
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("foo.toml").assert(is_symlink());
    env.home_file("bar.toml").assert(exists().not());
    Ok(())
}

#[test]
#[cfg(not(windows))]
fn test_failed_deploy_does_not_cleanup_previous_successful_deploy() -> TestResult {
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    let mut egg = env.open_egg("foo", EggConfig::new("foo.toml", env.home_file("old.toml")))?;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("old.toml").assert(is_symlink());

    env.home_file("new.toml").write_str("conflict")?;
    *egg.config_mut() = EggConfig::new("foo.toml", env.home_file("new.toml"));
    assert!(env.yolk().sync_egg_deployment(&egg).is_err());

    env.home_file("old.toml").assert(is_symlink());
    let cached_deployments = fs_err::read_to_string(env.yolk_file(".deployed_cache/foo"))?;
    assert_eq!(
        cached_deployments,
        env.home_file("old.toml").display().to_string()
    );
    Ok(())
}

#[test]
fn test_moving_merge_deploy_cleans_up_old_symlinks() -> TestResult {
    cov_mark::check_count!(delete_stale_symlink, 2);
    let env = TestEnv::init()?;
    env.home_file("a").create_dir_all()?;
    env.home_file("b").create_dir_all()?;
    env.egg_file("foo/foo/foo.toml").write_str("")?;
    let mut egg = env.open_egg("foo", EggConfig::new_merge(".", env.home_file("a")))?;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("a/foo").assert(is_symlink());

    // now we sync again, to a different location
    *egg.config_mut() = EggConfig::new_merge(".", env.home_file("b"));
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("b/foo").assert(is_symlink());
    env.home_file("a/foo").assert(exists().not());

    // and back, just to be sure
    *egg.config_mut() = EggConfig::new_merge(".", env.home_file("a"));
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("a/foo").assert(is_symlink());
    env.home_file("b/foo").assert(exists().not());
    Ok(())
}

#[test]
fn test_deploy_outside_of_home() -> TestResult {
    let env = TestEnv::init()?;
    let other_dir = TempDir::new()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    env.yolk().sync_egg_deployment(
        &env.open_egg(
            "foo",
            EggConfig::new("foo.toml", other_dir.child("foo.toml"))
                .with_strategy(DeploymentStrategy::Put),
        )?,
    )?;
    other_dir.child("foo.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_deploy_put_mode_fails_with_stow_usage() -> TestResult {
    cov_mark::check_count!(deploy_put, 1);
    let env = TestEnv::init()?;
    env.home_file(".config").create_dir_all()?;
    env.egg_file("bar/.config/thing.toml").write_str("")?;
    let result = env
        .yolk()
        .sync_egg_deployment(&env.open_egg("bar", EggConfig::new(".", &env.home))?);
    env.home_file(".config").assert(is_dir());
    env.home_file(".config/thing.toml").assert(exists().not());
    assert!(format!("{:?}", miette::Report::from(result.unwrap_err()))
        .contains("Failed to create symlink"));
    Ok(())
}

#[test]
fn test_deploy_put_creates_parent_dir() -> TestResult {
    cov_mark::check_count!(deploy_put, 1);
    cov_mark::check_count!(deploy_put_symlink_failed, 0);
    let env = TestEnv::init()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    env.yolk().sync_egg_deployment(&env.open_egg(
        "foo",
        EggConfig::new("foo.toml", env.home_file("a/a/a/foo.toml")),
    )?)?;
    env.home_file("a/a/a")
        .assert(is_dir().and(is_symlink().not()));
    env.home_file("a/a/a/foo.toml").assert(is_symlink());
    Ok(())
}

#[test]
fn test_undeploy() -> TestResult {
    cov_mark::check!(undeploy);
    let env = TestEnv::init()?;
    env.home_file(".config").create_dir_all()?;
    env.egg_file("foo/foo.toml").write_str("")?;
    env.egg_file("bar/.config/thing.toml").write_str("")?;

    let mut egg = env.open_egg(
        "foo",
        EggConfig::new_merge("foo.toml", env.home_file("foo.toml")),
    )?;

    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("foo.toml").assert(is_symlink());

    egg.config_mut().enabled = false;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file("foo.toml").assert(exists().not());

    // Verify stow-style usage works
    env.home_file(".config").create_dir_all()?;
    env.egg_file("bar/.config/thing.toml").write_str("")?;
    let mut egg = env.open_egg("bar", EggConfig::new_merge(".", &env.home))?;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file(".config/thing.toml").assert(is_symlink());
    egg.config_mut().enabled = false;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file(".config/thing.toml").assert(exists().not());
    env.home_file(".config").assert(is_dir());
    Ok(())
}

/// Test that sync_egg_deployment after moving something in the egg dir and changing the deployment configuration,
/// When encountering old, dead symlinks into the same egg, deletes those dead symlinks.
#[test]
fn test_deploy_after_moving_overrides_old_dead_symlinks() -> TestResult {
    cov_mark::check_count!(remove_dead_symlink, 1);

    let env = TestEnv::init()?;
    // We start out with a stow-style situation, where we have eggs/alacritty/.config/alacritty/alacritty.toml
    env.home_file(".config").create_dir_all()?;
    env.egg_file("alacritty/.config/alacritty/alacritty.toml")
        .write_str("")?;
    let mut egg = env.open_egg("alacritty", EggConfig::new_merge(".", &env.home))?;
    env.yolk().sync_egg_deployment(&egg)?;
    env.home_file(".config/alacritty").assert(is_symlink());

    // now we want to change to a simpler structure, where we explicitly declare the target dir for the files.
    // the user first moves the files inside the egg dir
    fs_err::rename(
        env.egg_file("alacritty/.config/alacritty/alacritty.toml"),
        env.egg_file("alacritty/alacritty.toml"),
    )?;
    // deletes the now empty .config structure
    fs_err::remove_dir(env.egg_file("alacritty/.config/alacritty/"))?;
    fs_err::remove_dir(env.egg_file("alacritty/.config/"))?;

    // He now updates his egg configuration to make the alacritty egg dir deploy to .config/alacritty
    egg.config_mut().targets = maplit::hashmap! {
        PathBuf::from(".") => PathBuf::from(".config/alacritty/")
    };
    // And syncs
    env.yolk().sync_egg_deployment(&egg)?;

    env.home_file(".config/alacritty").assert(is_symlink());
    Ok(())
}

#[test]
fn test_syncing() -> TestResult {
    let env = TestEnv::init()?;
    let foo_toml_initial = "{# data.value #}\nfoo";
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            export const data = if LOCAL { #{value: "local"} } else { #{value: "canonical"} };
            export let eggs = #{foo: #{ targets: `~`, strategy: "merge"}};
        "#})?;
    env.egg_file("foo/foo.toml").write_str(foo_toml_initial)?;
    env.yolk().sync_to_mode(EvalMode::Local, true)?;
    // No template set in eggs.rhai, so no templating should happen
    env.home_file("foo.toml").assert(is_symlink());
    env.egg_file("foo/foo.toml").assert(foo_toml_initial);

    // Now we make the file a template, so it should be updated
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "local"}} else {#{value: "canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
        "#})?;

    env.yolk().sync_to_mode(EvalMode::Local, true)?;
    env.egg_file("foo/foo.toml")
        .assert("{# data.value #}\nlocal");

    // Update the state, to see if applying again just works :tm:
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            export const data = if LOCAL {#{value: "new local"}} else {#{value: "new canonical"}};
            export let eggs = #{foo: #{targets: `~`, templates: ["foo.toml"], strategy: "merge"}};
        "#})?;
    env.yolk().sync_to_mode(EvalMode::Local, true)?;
    env.home_file("foo.toml")
        .assert("{# data.value #}\nnew local");
    env.yolk().with_canonical_state(|| {
        env.egg_file("foo/foo.toml")
            .assert("{# data.value #}\nnew canonical");
        Ok(())
    })?;
    Ok(())
}

#[test]
#[cfg(not(windows))]
fn test_sync_eggs_continues_after_failure() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            export let eggs = #{
                foo: #{ targets: `~`, strategy: "merge", templates: ["foo"]},
                bar: #{ targets: `~`, strategy: "merge", templates: ["bar"]},
            };
        "#})?;
    env.egg_file("foo/foo").write_str(r#"{< invalid rhai >}"#)?;
    env.egg_file("bar/bar").write_str(r#"foo # {<if false>}"#)?;
    let result = env.yolk().sync_to_mode(EvalMode::Local, true);
    env.egg_file("foo/foo").assert(r#"{< invalid rhai >}"#);
    env.egg_file("bar/bar")
        .assert(r#"#<yolk> foo # {<if false>}"#);
    assert!(test_util::render_error(result.unwrap_err()).contains("Syntax error"));
    Ok(())
}

#[test]
fn test_access_sysinfo() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(
        r#"
            export const hostname = SYSTEM.hostname;
            export let eggs = #{foo: #{targets: `~/foo`, templates: ["foo.toml"]}};
        "#,
    )?;
    env.egg_file("foo/foo.toml")
        .write_str("{< `host=${hostname}|${SYSTEM.hostname}` >}")?;
    env.yolk().sync_to_mode(EvalMode::Local, true)?;
    env.egg_file("foo/foo.toml").assert(
        "host=canonical-hostname|canonical-hostname{< `host=${hostname}|${SYSTEM.hostname}` >}",
    );
    Ok(())
}

#[test]
fn test_sysinfo_accessible_in_functions() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            fn get_hostname() { SYSTEM.hostname }
            fn is_local() { LOCAL }
        "#})?;
    let mut eval_ctx = env.yolk().prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_eq!(
        "canonical-hostname",
        eval_ctx.eval_rhai::<String>("get_hostname()")?
    );
    assert!(eval_ctx.eval_rhai::<bool>("is_local()")?);
    Ok(())
}

#[test]
fn test_imports_work_in_yolk_rhai() -> TestResult {
    let env = TestEnv::init()?;
    env.home_file("yolk/foo.rhai").write_str(indoc::indoc! {r#"
            fn some_function() { 1 }
            export let some_value = 1;
        "#})?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            import "foo" as foo;
            fn get_value() { foo::some_function() + foo::some_value }
        "#})?;
    let mut eval_ctx = env.yolk().prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_eq!(2, eval_ctx.eval_rhai::<i64>("get_value()")?);
    Ok(())
}

#[test]
pub fn test_custom_functions_in_text_transformer_tag() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            fn scream() { get_yolk_text().to_upper() }
        "#})?;
    let mut eval_ctx = env.yolk().prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_str_eq!(
        "TEST{< scream() >}",
        env.yolk()
            .eval_template(&mut eval_ctx, "", "test{< scream() >}")?
    );

    Ok(())
}

#[test]
fn test_variable_and_import_in_text_transformer_tag() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            import "foo" as foo;
            export let some_value = "a";
        "#})?;
    env.home_file("yolk/foo.rhai").write_str(indoc::indoc! {r#"
            export let imported = "b";
        "#})?;
    let mut eval_ctx = env.yolk().prepare_eval_ctx_for_templates(EvalMode::Local)?;
    assert_str_eq!(
        "foo=ab # {< replace_value(some_value + foo::imported) >}",
        env.yolk().eval_template(
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

    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            fn foo(
        "#})?;
    insta::assert_snapshot!(env
        .yolk()
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
pub fn test_rhai_error_for_known_function_with_wrong_arguments() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            io::path_exists(1);
        "#})?;

    let report = env
        .yolk()
        .prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)
        .map_err(test_util::render_report)
        .unwrap_err();

    assert!(report.contains("Function `io::path_exists` exists"));
    assert!(report.contains("no overload accepts arguments: i64"));
    assert!(report.contains("Available overloads:"));
    assert!(report.contains("io::path_exists(p: string) -> Result<bool>"));

    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_rhai_error_for_unknown_function_suggests_similar_name() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            io::path_exits("foo");
        "#})?;

    let report = env
        .yolk()
        .prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)
        .map_err(test_util::render_report)
        .unwrap_err();

    assert!(report.contains("Unknown function `io::path_exits`"));
    assert!(report.contains("Did you mean `io::path_exists`?"));

    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_rhai_error_for_user_function_with_wrong_arguments() -> TestResult {
    let env = TestEnv::init()?;
    env.yolk_rhai().write_str(indoc::indoc! {r#"
            fn greet(name) {
                "hello, " + name
            }
        "#})?;
    let mut eval_ctx = env
        .yolk()
        .prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)?;

    let report = eval_ctx
        .eval_rhai::<String>("greet(\"leon\", \"extra\")")
        .map_err(|err| err.into_report("expr", "greet(\"leon\", \"extra\")"))
        .map_err(test_util::render_report)
        .unwrap_err();

    assert!(report.contains("Function `greet` exists"), "{report}");
    assert!(
        report.contains(
            "no overload accepts arguments: &str | ImmutableString | String, &str | ImmutableString | String"
        ),
        "{report}"
    );
    assert!(report.contains("greet(name)"), "{report}");

    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_rhai_function_hint_keeps_template_span() -> TestResult {
    let env = TestEnv::init()?;
    let mut eval_ctx = env
        .yolk()
        .prepare_eval_ctx_for_templates(crate::yolk::EvalMode::Local)?;

    let report = env
        .yolk()
        .eval_template(
            &mut eval_ctx,
            "template.conf",
            "before\nvalue = {< io::path_exists(1) >}\nafter\n",
        )
        .map_err(test_util::render_report)
        .unwrap_err();

    assert!(report.contains("Function `io::path_exists` exists"));
    assert!(report.contains("value = {< io::path_exists(1) >}"));
    assert!(report.contains("template.conf"));

    Ok(())
}

#[test]
#[cfg(not(windows))]
pub fn test_deployment_error() -> TestResult {
    let env = TestEnv::init()?;
    env.egg_file("bar/file1").write_str("")?;
    env.egg_file("bar/file2").write_str("")?;
    env.home_file("file1").write_str("")?;
    env.home_file("file2").write_str("")?;
    let egg = env.open_egg(
        "bar",
        EggConfig::default()
            .with_target("file1", env.home_file("file1"))
            .with_target("file2", env.home_file("file2")),
    )?;
    // Normalize platform-specific temp dir prefixes (e.g. macOS uses
    // `/var/folders/.../T` and `/private/var/folders/.../T`) so the snapshot
    // looks the same as on Linux where the prefix is `/tmp`.
    let tmp = std::env::temp_dir();
    let tmp_canonical = tmp.canonicalize().unwrap_or_else(|_| tmp.clone());
    let tmp_re = regex::escape(tmp.to_string_lossy().trim_end_matches('/'));
    let tmp_canonical_re = regex::escape(tmp_canonical.to_string_lossy().trim_end_matches('/'));
    insta::with_settings!({filters => vec![
        (tmp_canonical_re.as_str(), "/tmp"),
        (tmp_re.as_str(), "/tmp"),
        (r"\.tmp[a-zA-Z0-9]{6}", "[tmp-dir]"),
        (r"file\d", "[filename]"),
    ]}, {
        insta::assert_snapshot!(test_util::render_error(
            env.yolk().sync_egg_deployment(&egg).unwrap_err()
        ));
    });
    Ok(())
}

#[test]
pub fn test_only_active_sections_get_evaluated() -> TestResult {
    let env = TestEnv::init()?;
    let mut eval_ctx = env.yolk().prepare_eval_ctx_for_templates(EvalMode::Local)?;

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
        env.yolk().eval_template(&mut eval_ctx, "", template)?
    );

    Ok(())
}
