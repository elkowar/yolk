//! Logic to handle the [git long-running filter process](https://git-scm.com/docs/gitattributes#_long_running_filter_process) connection.
//!
//! This is used to implement the `yolk git-filter` command, which gets called by git whenever the user checks out or checks in some files.
//! This allows doing the template processing (canonicalization) in-memory within git, rather than having to change the files on-disk before and after interacting with them through git.

use std::str::FromStr;

use miette::{Context, IntoDiagnostic};
use proto::{GitWriter, PacketKind};

pub struct GitFilterServer<P, R = std::io::Stdin, W = std::io::Stdout> {
    processor: P,
    input: R,
    output: GitWriter<W>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GitFilterMode {
    Clean,
    Smudge,
}

impl FromStr for GitFilterMode {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clean" => Ok(GitFilterMode::Clean),
            "smudge" => Ok(GitFilterMode::Smudge),
            _ => Err(miette::miette!("Unknown git filter mode: {}", s)),
        }
    }
}

pub trait GitFilterProcessor {
    fn process(
        &mut self,
        path: &str,
        mode: GitFilterMode,
        input: Vec<u8>,
    ) -> miette::Result<Vec<u8>>;
}

impl<P: GitFilterProcessor, R: std::io::Read, W: std::io::Write> GitFilterServer<P, R, W> {
    pub fn new(input: R, output: W, processor: P) -> Self {
        Self {
            processor,
            input,
            output: GitWriter(output),
        }
    }

    pub fn run(&mut self) -> miette::Result<()> {
        self.handle_handshake()?;
        loop {
            let Some((command, pathname)) = self.read_command_header()? else {
                return Ok(());
            };
            match command.as_str() {
                t @ "clean" | t @ "smudge" => {
                    let mode = GitFilterMode::from_str(t)?;
                    let content = read_bin_until_flush(&mut self.input)
                        .context("Failed to read content from git")?;
                    match self.processor.process(&pathname, mode, content) {
                        Ok(success) => self.send_processing_success(&success)?,
                        Err(error) => {
                            eprintln!("Error in git filter: {error:?}");
                            self.output.write_all(b"status=error")?;
                            self.output.send_flush()?;
                        }
                    }
                }
                _ => {
                    miette::bail!("Unknown command: {}", command);
                }
            }
        }
    }

    fn send_processing_success(&mut self, success: &[u8]) -> Result<(), miette::Error> {
        self.output
            .write_all(b"status=success")
            .context("failed to send status=success")?;
        self.output.send_flush()?;
        self.output
            .write_all(success)
            .context("Failed to write processing output")?;
        self.output.send_flush()?;
        self.output.send_flush()?;
        Ok(())
    }

    fn expect_text_packet(&mut self, expected: &str) -> miette::Result<()> {
        if read_text_packet(&mut self.input)
            .with_context(|| format!("Expected text packet: {}", expected))?
            .is_some_and(|x| x != expected)
        {
            miette::bail!("Expected text packet: {}", expected);
        }
        Ok(())
    }

    fn handle_handshake(&mut self) -> miette::Result<()> {
        self.expect_text_packet("git-filter-client")?;
        self.expect_text_packet("version=2")?;
        if proto::read_packet(&mut self.input)? != Some(PacketKind::Flush) {
            miette::bail!("Expected flush after client hello");
        };

        self.output.write_all(b"git-filter-server")?;
        self.output.write_all(b"version=2")?;
        self.output.flush()?;
        self.output.send_flush()?;

        let mut filter = false;
        let mut smudge = false;
        while let Some(command) = read_text_packet(&mut self.input)? {
            match command.as_str() {
                "capability=clean" => filter = true,
                "capability=smudge" => smudge = true,
                _ => {}
            }
        }
        if filter {
            self.output.write_all(b"capability=clean")?;
        }
        if smudge {
            self.output.write_all(b"capability=smudge")?;
        }
        self.output.send_flush()?;
        Ok(())
    }

    fn read_command_header(&mut self) -> miette::Result<Option<(String, String)>> {
        let mut command = None;
        let mut pathname = None;
        let mut got_something = false;
        while let Some(input) =
            read_text_packet(&mut self.input).context("failed to start reading new file data")?
        {
            got_something = true;
            if let Some(input_command) = input.strip_prefix("command=") {
                command = Some(input_command.to_string());
            } else if let Some(input_pathname) = input.strip_prefix("pathname=") {
                pathname = Some(input_pathname.to_string());
            }
        }
        if !got_something {
            return Ok(None);
        }
        match (command, pathname) {
            (Some(command), Some(pathname)) => Ok(Some((command, pathname))),
            (None, _) => miette::bail!("Missing command"),
            (_, None) => miette::bail!("Missing pathname"),
        }
    }
}

fn read_bin_packet(read: &mut impl std::io::Read) -> miette::Result<Option<Vec<u8>>> {
    match proto::read_packet(read)? {
        Some(PacketKind::Data(x)) => Ok(Some(x)),
        Some(PacketKind::Flush) => Ok(None),
        None => Ok(None),
    }
}

fn read_bin_until_flush(read: &mut impl std::io::Read) -> miette::Result<Vec<u8>> {
    let mut result = Vec::new();
    while let Some(bin) = proto::read_packet(read).context("Failed to read packet")? {
        match bin {
            PacketKind::Data(x) => result.extend(x),
            PacketKind::Flush => return Ok(result),
        }
    }
    Ok(result)
}

fn read_text_packet(read: &mut impl std::io::Read) -> miette::Result<Option<String>> {
    let Some(bin) = read_bin_packet(read).context("Failed to read binary text data")? else {
        return Ok(None);
    };

    if !bin.ends_with(b"\n") {
        miette::bail!("Expected text packet to end with a newline");
    }
    Ok(Some(
        String::from_utf8(bin[..bin.len() - 1].to_vec()).into_diagnostic()?,
    ))
}

mod proto {
    use miette::{Context, IntoDiagnostic, Result};

    pub const MAX_PACKET_LEN: usize = 65516;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PacketKind {
        Data(Vec<u8>),
        Flush,
    }

    pub struct GitWriter<T>(pub T);

    impl<T: std::io::Write> GitWriter<T> {
        pub fn write_all(&mut self, buf: &[u8]) -> miette::Result<()> {
            for chunk in buf.chunks(MAX_PACKET_LEN - 4) {
                let len_bytes = (chunk.len() as u16 + 4).to_be_bytes();
                let mut len_hex = [0; 4];
                hex::encode_to_slice(len_bytes, &mut len_hex).unwrap();
                self.0.write_all(&len_hex).into_diagnostic()?;
                self.0.write_all(chunk).into_diagnostic()?;
            }
            Ok(())
        }

        pub(super) fn flush(&mut self) -> miette::Result<()> {
            self.0.flush().into_diagnostic()
        }
        pub(super) fn send_flush(&mut self) -> miette::Result<()> {
            self.0
                .write_all(b"0000")
                .into_diagnostic()
                .context("Failed to send flush packet")?;
            self.0
                .flush()
                .into_diagnostic()
                .context("Failed to flush after sending flush packet")
        }
    }

    pub fn read_packet(read: &mut impl std::io::Read) -> Result<Option<PacketKind>> {
        let mut len_hex = [0; 4];
        match read.read_exact(&mut len_hex) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            other => other
                .into_diagnostic()
                .wrap_err("Failed to read packet length")?,
        }
        let mut len = [0; 2];
        hex::decode_to_slice(len_hex, &mut len)
            .into_diagnostic()
            .wrap_err("Bad hex length received")?;
        let len = u16::from_be_bytes(len) as usize;
        if len == 0 {
            return Ok(Some(PacketKind::Flush));
        }
        let len = len - 4;
        if len > MAX_PACKET_LEN {
            miette::bail!("Packet too long: {}", len);
        } else if len == 0 {
            miette::bail!("Packet size must never be 0");
        }
        let mut result = vec![0; len];
        read.read_exact(&mut result[..]).into_diagnostic()?;
        Ok(Some(PacketKind::Data(result)))
    }
}
