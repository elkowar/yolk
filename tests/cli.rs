//! Integration tests that spawn the compiled `yolk` binary.
//!
//! These live here as an integration test (rather than as unit tests under
//! `src/`) so that Cargo provides the `CARGO_BIN_EXE_yolk` environment variable,
//! which `assert_cmd::Command::cargo_bin` uses to locate the binary. Unit tests
//! don't get that variable, which forces `assert_cmd` to guess the binary's
//! location by walking up from the test binary and assuming the default
//! build-artifact layout. That assumption breaks with `build.build-dir`
//! (`CARGO_BUILD_BUILD_DIR`, Cargo 1.91) and the upcoming build-dir layout
//! changes, so any test that spawns the binary must run from here.

use assert_cmd::{assert, Command};
use assert_fs::prelude::{FileWriteStr as _, PathChild as _, PathCreateDir as _};
use predicates::str::contains;

use yolk::{
    yolk::{EvalMode, Yolk},
    yolk_paths::YolkPaths,
};

/// like <https://crates.io/crates/testresult>, but shows the debug output instead of display.
pub type TestResult<T = ()> = std::result::Result<T, TestError>;

#[derive(Debug)]
pub enum TestError {}

impl<T: std::fmt::Debug + std::fmt::Display> From<T> for TestError {
    #[track_caller] // Will show the location of the caller in test failure messages
    fn from(error: T) -> Self {
        // Use alternate format for rich error message for anyhow
        // See: https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
        panic!("error: {} - {:?}", std::any::type_name::<T>(), error);
    }
}

struct TestEnv {
    home: assert_fs::TempDir,
    eggs: assert_fs::fixture::ChildPath,
    yolk: Yolk,
}

impl TestEnv {
    pub fn init() -> TestResult<Self> {
        let home = assert_fs::TempDir::new()?;
        let paths = YolkPaths::new(home.join("yolk"), home.to_path_buf())?;
        let yolk = Yolk::new(paths);
        // Ensure neither the in-process library nor the spawned binary touch the
        // real home directory. The binary is additionally given `--home-dir`
        // explicitly below.
        std::env::set_var("HOME", home.path());

        let eggs = home.child("yolk/eggs");
        yolk.init_yolk(None)?;
        Ok(Self { home, eggs, yolk })
    }

    pub fn yolk_root(&self) -> assert_fs::fixture::ChildPath {
        self.home.child("yolk")
    }

    pub fn start_git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env("HOME", self.home.path())
            .current_dir(self.yolk_root().path())
            .args([
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

#[test]
fn test_adopt_directory_prints_config_snippet() -> TestResult {
    let env = TestEnv::init()?;
    let source = env.home.child(".config/noctalia");
    source.create_dir_all()?;

    let mut cmd = env.yolk_cmd();
    cmd.args(["adopt", "noctalia", &source.to_string_lossy()]);
    cmd.assert().success().stdout(contains(
        r#""noctalia": #{ enabled: true, strategy: "put", targets: "~/.config/noctalia", templates: [] },"#,
    ));

    Ok(())
}

#[test]
fn test_adopt_file_prints_target_map() -> TestResult {
    let env = TestEnv::init()?;
    let source = env.home.child(".zshrc");
    source.write_str("zsh")?;

    let mut cmd = env.yolk_cmd();
    cmd.args([
        "adopt",
        "zsh",
        &source.to_string_lossy(),
        "--strategy",
        "merge",
    ]);
    cmd.assert().success().stdout(contains(
        r#""zsh": #{ enabled: true, strategy: "merge", targets: #{ ".zshrc": "~/.zshrc" }, templates: [] },"#,
    ));

    Ok(())
}
