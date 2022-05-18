use clap::Parser;
use cli_selector::{extract, ExtractOptions};
use std::{error::Error, io::Read, path::PathBuf};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// input file ( specify "-" for stdin )
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        default_value = "-"
    )]
    input: std::path::PathBuf,

    // css selector to select html elements
    selector: String,

    /// separator between matching elements
    #[clap(long, short,default_value_t = String::from("\n"))]
    separator: String,

    #[clap(long, short = 'n')]
    inner: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        input: input_file,
        selector,
        separator,
        inner,
    } = Args::parse();
    let selector = scraper::Selector::parse(&selector).expect("selector parse error");

    let str = if input_file.eq(&PathBuf::from("-")) {
        let mut vec = vec![];
        std::io::stdin().read_to_end(&mut vec)?;
        String::from_utf8(vec).expect("Unable to convert input to utf-8")
    } else {
        std::fs::read_to_string(input_file)?
    };

    let out = extract(ExtractOptions {
        input: str,
        selector,
        separator,
        inner,
    });

    println!("{}", out);

    Ok(())
}
