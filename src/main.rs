extern crate fantoccini;
extern crate futures;
extern crate structopt;

use fantoccini::{Client, Locator};
use futures::future::Future;
use structopt::StructOpt;
use tokio::timer::Delay;

use std::time::{Duration, Instant};

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
    let search_button_css_id: &str = "MainContent_cmdSearch";
    let search_field_css_id: &str = "MainContent_txtFilingName";
    let search_results_css_class: &str = ".search-results";
    let search_result_css_selector: &str = ".search-results li";

    let c = Client::new("http://localhost:4444");
    let args = Cli::from_args();

    // let's set up the sequence of steps we want the browser to take
    tokio::run(
        c.map_err(|e| unimplemented!("failed to connect to WebDriver: {:?}", e))
            .and_then(|c| c.goto("https://wyobiz.wy.gov/business/filingsearch.aspx"))
            .and_then(move |mut c| c.form(Locator::Id(search_field_css_id)))
            .and_then(move |mut search_field| {
                search_field
                    .set(Locator::Id(search_field_css_id), &args.search_term)
                    .map(|element| element.client())
            })
            .and_then(move |mut c| c.find(Locator::Id(search_button_css_id)))
            .and_then(|button| button.click())
            .and_then(move |c| {
                c.wait_for_find(Locator::Css(search_results_css_class))
                    .map(|e| e.client())
            })
            .and_then(move |mut c| c.find_all(Locator::Css(search_result_css_selector)))
            .and_then(|mut list_items| {
                list_items.map(|li| li.html())
            })
            .and_then(|c| {
                Ok(())
            })
            .map_err(|e| {
                panic!("a WebDriver command failed: {:?}", e);
            }),
    );
}
