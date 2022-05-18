pub struct ExtractOptions {
    pub input: String,
    pub selector: scraper::Selector,
    pub separator: String,
    pub inner: bool,
}
pub fn extract(options: ExtractOptions) -> String {
    scraper::Html::parse_fragment(&options.input)
        .root_element()
        .select(&options.selector)
        .map(|elem| {
            if options.inner {
                elem.inner_html()
            } else {
                elem.html()
            }
        })
        .collect::<Vec<_>>()
        .join(&options.separator)
}
