//! A simple utility to extract ASN.1 spec files from the docx files (WIP).

use std::io::{Read, Write};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "input '.docx' file.")]
    input_file: String,

    #[arg(short = 'o', long = "output", help = "output '.asn' file.")]
    output_file: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let asn_start = regex::Regex::new("^-- ASN1START$").unwrap();
    let asn_stop = regex::Regex::new("^-- ASN1STOP$").unwrap();

    let mut docx = std::fs::File::open(&cli.input_file)?;
    let mut buffer = Vec::new();
    docx.read_to_end(&mut buffer)?;

    let docx = docx_rs::read_docx(&buffer);

    let mut collect_asn = false;
    let mut asn_text = vec![];
    for child in docx.unwrap().document.children {
        if let docx_rs::DocumentChild::Paragraph(ref p) = child {
            for para_child in &p.children {
                if let docx_rs::ParagraphChild::Run(ref r) = para_child {
                    for run_child in &r.children {
                        if let docx_rs::RunChild::Text(ref t) = run_child {
                            if asn_start.is_match(&t.text) {
                                collect_asn = true;
                            }
                            if asn_stop.is_match(&t.text) {
                                collect_asn = false;
                            }
                            if collect_asn {
                                let replaced = t.text.replace("&amp;", "&").replace('\u{a0}', " ");
                                asn_text.push(replaced);
                            }
                        }
                    }
                }
            }
        }
    }

    let asn_text = asn_text.join("\n");

    let mut output_file = std::fs::File::create(cli.output_file)?;
    output_file.write_all(asn_text.as_bytes())?;

    Ok(())
}
