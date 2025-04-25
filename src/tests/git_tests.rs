use assert_cmd::{assert, Command};
use assert_fs::prelude::{FileWriteStr as _, PathChild};

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

    // pub fn config_git(&self) {
    //     self.start_git_command()
    //         .args(["config", "--local", "user.name", "test"])
    //         .assert()
    //         .success();
    //     self.start_git_command()
    //         .args(["config", "--local", "user.email", "test@test.test"])
    //         .assert()
    //         .success();
    // }

    pub fn start_git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env("HOME", self.home.path())
            .current_dir(self.yolk_root().path())
            .args(&[
                "--git-dir",
                &self
                    .yolk
                    .paths()
                    .active_yolk_git_dir()
                    .unwrap()
                    .to_string_lossy(),
                "--work-tree",
                &self.yolk_root().to_string_lossy(),
            ]);
        cmd
    }

    pub fn git_cmd(&self, args: &[&str]) -> assert::Assert {
        let mut cmd = self.start_git_command();
        cmd.args(args);
        cmd.assert()
    }

    pub fn yolk_cmd(&self) -> assert_cmd::Command {
        let mut yolk_command = assert_cmd::Command::cargo_bin("yolk").unwrap();
        yolk_command.current_dir(self.yolk_root()).args([
            "--yolk-dir",
            &self.yolk_root().to_string_lossy(),
            "--home-dir",
            &self.yolk.paths().home_path().to_string_lossy(),
        ]);
        yolk_command
    }

    pub fn yolk_git(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        let mut yolk_command = self.yolk_cmd();
        yolk_command.arg("git").args(args);
        yolk_command
            .timeout(std::time::Duration::from_secs(1))
            .assert()
    }
}

#[test]
fn test_git_add_with_error() -> TestResult {
    let env = TestEnv::init()?;

    env.home
        .child("yolk/yolk.rhai")
        .write_str(indoc::indoc! {r#"
            export let eggs = #{
                foo: #{ targets: `~/foo`, strategy: "put", templates: ["fine", "broken"]},
            };
        "#})?;
    env.eggs
        .child("foo/fine")
        .write_str(r#"{<(1+1).to_string()>}"#)?;
    env.eggs.child("foo/broken").write_str(r#"{< foo >}"#)?;
    assert!(env.yolk.sync_to_mode(EvalMode::Local, false).is_err());
    env.yolk_git(&["add", "--all"]).failure();
    env.git_cmd(&["show", ":eggs/foo/fine"])
        .stdout("")
        .stderr("fatal: path \'eggs/foo/fine\' exists on disk, but not in the index\n");
    env.git_cmd(&["show", ":eggs/foo/broken"])
        .stdout("")
        .stderr("fatal: path \'eggs/foo/broken\' exists on disk, but not in the index\n");
    Ok(())
}
