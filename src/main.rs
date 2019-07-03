extern crate structopt;
extern crate futures;
extern crate fantoccini;

use fantoccini::{Client, Locator};
use futures::future::Future;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "t", long = "term")]
    /// The full name or partial name of the business you want to search for.
    search_term: String,

    #[structopt(short = "y", long = "type", default_value = "contains")]
    /// The type of search you want to do could either be 'starts with (sw)' or 'contains (c)'.
    /// Defaults to 'contains'.
    search_type: String,
}

fn main() {
    let radio_button_css_id: & str;
    let search_button_css_id: &str = "MainContent_cmdSearch";
    let search_field_css_id: &str = "MainContent_txtFilingName";
    let search_results_css_class: &str = "search-results";

    let c = Client::new("http://localhost:4444");
    let args = Cli::from_args();

    println!("Search Term: {:?}", args);

    tokio::run(
        c
            .map_err(|e| {
                unimplemented!("failed to connect to WebDriver: {:?}", e)
            })
            .and_then(|c| {
                c.goto("https://wyobiz.wy.gov/business/filingsearch.aspx")
            })
            .and_then(|mut c| {
                c.form(Locator::Id("MainContent_txtFilingName"))
            })
            .and_then(|mut search_field| {
                search_field.set(Locator::Id("MainContent_txtFilingName"), "detroit")
            })
            .and_then(|c| {
                Ok(())
            })
            .map_err(|e| {
                panic!("a WebDriver command failed: {:?}", e);
            })
    );
}
