import { readFileSync } from "node:fs";

const cargoToml = readFileSync("Cargo.toml", "utf8");
const version = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)?.[1] ?? "0.0.0";
const repo = "https://github.com/elkowar/yolk";
const tag = `v${version}`;
const packageName = "yolk_dots";

const commonUnix = [
  {
    label: "shell",
    language: "shell",
    command: `curl --proto '=https' --tlsv1.2 -LsSf ${repo}/releases/download/${tag}/${packageName}-installer.sh | sh`,
  },
  {
    label: "crates.io",
    language: "shell",
    command: `cargo install ${packageName} --locked --profile=dist`,
  },
  {
    label: "cargo-binstall",
    language: "shell",
    command: `cargo binstall ${packageName}`,
  },
];

export default {
  version,
  options: [
    {
      id: "linux-x64",
      label: "Linux x64",
      commands: commonUnix,
      artifact: `${repo}/releases/download/${tag}/${packageName}-x86_64-unknown-linux-gnu.tar.xz`,
    },
    {
      id: "linux-musl-x64",
      label: "musl Linux x64",
      commands: commonUnix,
      artifact: `${repo}/releases/download/${tag}/${packageName}-x86_64-unknown-linux-musl.tar.xz`,
    },
    {
      id: "macos-arm64",
      label: "macOS Apple Silicon",
      commands: [
        ...commonUnix,
        { label: "Homebrew", language: "shell", command: "brew install elkowar/tap/yolk" },
      ],
      artifact: `${repo}/releases/download/${tag}/${packageName}-aarch64-apple-darwin.tar.xz`,
    },
    {
      id: "macos-x64",
      label: "macOS Intel",
      commands: [
        ...commonUnix,
        { label: "Homebrew", language: "shell", command: "brew install elkowar/tap/yolk" },
      ],
      artifact: `${repo}/releases/download/${tag}/${packageName}-x86_64-apple-darwin.tar.xz`,
    },
    {
      id: "windows-x64",
      label: "Windows x64",
      commands: [
        {
          label: "powershell",
          language: "powershell",
          command: `irm ${repo}/releases/download/${tag}/${packageName}-installer.ps1 | iex`,
        },
        {
          label: "crates.io",
          language: "powershell",
          command: `cargo install ${packageName} --locked --profile=dist`,
        },
      ],
      artifact: `${repo}/releases/download/${tag}/${packageName}-x86_64-pc-windows-msvc.zip`,
    },
  ],
};
