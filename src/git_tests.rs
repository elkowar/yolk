use assert_cmd::Command;
use assert_fs::{
    assert::PathAssert as _,
    prelude::{FileWriteStr as _, PathChild},
};

use crate::{
    util::test_util::{setup_and_init_test_yolk, TestResult},
    yolk::{EvalMode, Yolk},
};

struct TestEnv {
    home: assert_fs::TempDir,
    eggs: assert_fs::fixture::ChildPath,
    yolk: Yolk,
}

impl TestEnv {
    pub fn init() -> miette::Result<Self> {
        let (home, yolk, eggs) = setup_and_init_test_yolk()?;
        Ok(Self { home, yolk, eggs })
    }
    pub fn yolk_root(&self) -> assert_fs::fixture::ChildPath {
        self.home.child("yolk")
    }

    pub fn start_git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env("HOME", self.home.path())
            .current_dir(self.yolk_root().path());
        cmd
    }

    pub fn git_add_all(&self) -> assert_cmd::assert::Assert {
        self.start_git_command().args(["add", "--all"]).assert()
    }
    pub fn git_show_staged(&self, path: impl ToString) -> assert_cmd::assert::Assert {
        self.start_git_command()
            .args(["show", &format!(":{}", path.to_string())])
            .assert()
    }
}

#[test]
fn test_init_works() -> TestResult {
    let env = TestEnv::init()?;
    let yolk_binary_path = assert_cmd::cargo::cargo_bin("yolk");
    env.start_git_command()
        .args(["config", "--local", "--get-all", "filter.yolk.process"])
        .assert()
        .success()
        .stdout(format!(
            "{} --yolk-dir '{}' --home-dir '{}' git-filter\n",
            yolk_binary_path.display().to_string().replace(r"\", r"\\"),
            env.yolk_root()
                .path()
                .display()
                .to_string()
                .replace(r"\", r"\\"),
            env.home.path().display().to_string().replace(r"\", r"\\"),
        ));
    Ok(())
}

#[test]
fn test_git_add() -> TestResult {
    let env = TestEnv::init()?;

    env.home
        .child("yolk/yolk.rhai")
        .write_str(indoc::indoc! {r#"
        export let eggs = #{
            foo: #{ targets: `~/foo`, strategy: "put", templates: ["file"]},
            bar: #{ targets: `~/bar`, strategy: "put", templates: ["file"]},
        };
    "#})?;
    env.eggs
        .child("foo/file")
        .write_str(r#"foo=1 # {< replace_value(LOCAL.to_string())>}"#)?;
    env.eggs.child("foo/non-template").write_str(r#"{<1+1>}"#)?;
    env.eggs
        .child("bar/file")
        .write_str(r#"#<yolk> foo # {<if LOCAL>}"#)?;
    env.yolk.sync_to_mode(EvalMode::Local)?;
    env.eggs
        .child("foo/file")
        .assert("foo=true # {< replace_value(LOCAL.to_string())>}");
    env.eggs.child("bar/file").assert("foo # {<if LOCAL>}");
    env.eggs.child("foo/non-template").assert(r#"{<1+1>}"#);

    env.git_add_all().success();
    env.git_show_staged("eggs/foo/file")
        .stdout("foo=false # {< replace_value(LOCAL.to_string())>}");
    env.git_show_staged("eggs/bar/file")
        .stdout("#<yolk> foo # {<if LOCAL>}");
    env.git_show_staged("eggs/foo/non-template")
        .stdout("{<1+1>}");
    Ok(())
}

#[test]
fn test_git_add_with_error() -> TestResult {
    let env = TestEnv::init()?;

    env.home
        .child("yolk/yolk.rhai")
        .write_str(indoc::indoc! {r#"
        export let eggs = #{
            foo: #{ targets: `~/foo`, strategy: "put", templates: ["bad"]},
            bar: #{ targets: `~/bar`, strategy: "put", templates: ["file"]},
        };
    "#})?;
    env.eggs
        .child("foo/bad")
        .write_str(r#"foo=1 # {< bad syntax >}"#)?;
    env.eggs
        .child("bar/file")
        .write_str(r#"#<yolk> foo # {<if LOCAL>}"#)?;
    env.eggs.child("foo/bad").assert("foo=1 # {< bad syntax >}");
    env.eggs
        .child("bar/file")
        .assert("#<yolk> foo # {<if LOCAL>}");

    env.git_add_all().failure();
    env.git_show_staged("eggs/foo/bar").failure();
    env.git_show_staged("eggs/bar/file").failure();
    Ok(())
}
