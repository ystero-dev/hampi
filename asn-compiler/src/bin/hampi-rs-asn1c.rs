//! A simple utility to tokenize ASN files.

use anyhow::Result;
use clap::Parser;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

#[derive(Parser, Debug)]
#[command(
    name = "rs-asn1c",
    author,
    version,
    about,
    help_template = "\
{name}: v{version} by {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}\n"
)]
struct Cli {
    /// ASN.1 files to compile
    #[arg(last = true, required = true)]
    files: Vec<String>,

    /// Name of the Rust Module to write generated code to
    #[arg(short, long)]
    module: String,

    /// Visibility of Generated Structures and members:
    #[arg(long, value_enum, default_value_t=Visibility::Public)]
    visibility: Visibility,

    /// ASN.1 Codecs to be Supported during code generation
    /// Specify multiple times for multiple codecs. (eg. --codec aper --codec uper)
    #[arg(long, required = true)]
    codec: Vec<Codec>,

    /// Generate code for these derive macros during code generation
    #[arg(long)]
    derive: Vec<Derive>,

    /// Log verbosity (default: info, -d: debug, -dd...: trace)
    #[arg(short, action=clap::ArgAction::Count)]
    debug: u8,

    /// Whether to disable `rustfmt` formatting of the generated files.
    #[arg(long, default_value_t = false)]
    no_rustfmt: bool,
}

fn main() -> Result<()> {
    let mut cli = Cli::parse();

    if cli.files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No Input files Specified",
        )
        .into());
    }

    let derives = if cli.derive.contains(&Derive::All) {
        cli.derive
            .into_iter()
            .filter(|t| t == &Derive::All)
            .collect::<Vec<Derive>>()
    } else {
        if !cli.derive.contains(&Derive::Debug) {
            cli.derive.push(Derive::Debug);
        }
        cli.derive
    };

    let level = if cli.debug > 0 {
        if cli.debug == 1 {
            "debug"
        } else {
            "trace"
        }
    } else {
        "info"
    };

    let env = env_logger::Env::default().filter_or("MY_LOG_LEVEL", level);
    env_logger::init_from_env(env);

    let mut compiler = Asn1Compiler::new(
        &cli.module,
        &cli.visibility,
        cli.codec.clone(),
        derives.clone(),
    );

    if cli.no_rustfmt {
        compiler.set_rustfmt_generated_code(false);
    }

    compiler.compile_files(&cli.files)?;

    Ok(())
}
