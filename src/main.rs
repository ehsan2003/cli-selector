use clap::Parser;

use scraper::ElementRef;
use std::{error::Error, io::Read, path::PathBuf};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
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

    #[clap(subcommand)]
    function: Option<ExtractionFunction>,
}

#[derive(clap::Subcommand)]
pub enum ExtractionFunction {
    /// extracts attribute from html tags matched by selector
    Attr(Attribute),
    /// inner html of matched tags
    Inner,
    /// inner text of matched tags
    Text,
    /// outer html of matched tags
    Outer,
}
impl Default for ExtractionFunction {
    fn default() -> Self {
        Self::Outer
    }
}
#[derive(clap::Args)]
pub struct Attribute {
    attr: String,
}
impl ExtractionFunction {
    pub fn extract(&self, element: ElementRef) -> String {
        match self {
            ExtractionFunction::Attr(Attribute { attr }) => element
                .value()
                .attr(attr)
                .to_owned()
                .unwrap_or_default()
                .to_string(),
            ExtractionFunction::Inner => element.inner_html(),
            ExtractionFunction::Outer => element.html(),
            ExtractionFunction::Text => element.text().collect(),
        }
    }
}
pub struct ExtractOptions {
    pub input: String,
    pub selector: scraper::Selector,
    pub separator: String,
    pub extraction_function: ExtractionFunction,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Arguments {
        input: input_file,
        selector,
        separator,
        function,
    } = Arguments::parse();
    let function = function.unwrap_or_default();
    let selector = scraper::Selector::parse(&selector).expect("selector parse error");

    let str = if input_file.eq(&PathBuf::from("-")) {
        let mut vec = vec![];
        std::io::stdin().read_to_end(&mut vec)?;
        String::from_utf8(vec).expect("Unable to convert input to utf-8")
    } else {
        std::fs::read_to_string(input_file)?
    };

    let out = scraper::Html::parse_fragment(&str)
        .root_element()
        .select(&selector)
        .map(|elem| function.extract(elem))
        .collect::<Vec<_>>()
        .join(&separator);

    println!("{}", out);

    Ok(())
}
