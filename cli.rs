use std::io::BufWriter;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use pat::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "plist cat")]
pub struct Cli {
    #[arg()]
    path: PathBuf,

    #[arg(short, long)]
    xml: bool,

    #[arg(short, long, conflicts_with = "xml")]
    toml: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let dictionary = plist::from_file::<PathBuf, plist::Dictionary>(args.path.clone())?;
    println!(
        "{}",
        if args.xml {
            let mut writer = BufWriter::new(Vec::<u8>::new());
            plist::to_writer_xml_with_options(
                &mut writer,
                &dictionary,
                &plist::XmlWriteOptions::default().indent(b' ', 4),
            )?;
            let xml = writer.into_inner()?;
            String::from_utf8(xml).unwrap()
        } else if args.toml {
            toml::to_string_pretty(&dictionary)?
        } else {
            serde_json::to_string_pretty(&dictionary)?
        }
    );

    Ok(())
}
