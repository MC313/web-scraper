/// Running rust program: cargo run -- -t Michigan

extern crate clap;
extern crate fantoccini;

use clap::Parser;
use fantoccini::{ClientBuilder, Locator};
use std::collections::HashMap;
use tokio;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    /// The full name or partial name of the business you want to search for.
    search_term: String,

    #[clap(short = 't', long = "type", possible_values = ["c", "sw"], default_value = "c")]
    /// The type of search you want to do could either be 'starts with (sw)' or 'contains (c)'.
    /// Defaults to 'contains'.
    search_type: String,
}


#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let args = Args::parse();
    
    let search_button_css_id: &str = "MainContent_cmdSearch";
    let search_field_css_id: &str = "MainContent_txtFilingName";
    let search_results_css_class: &str = ".search-results";
    let search_result_css_selector: &str = ".search-results li";
    let search_type_contains_css_id: &str = "MainContent_chkSearchIncludes";
    let search_type_startswith_css_id: &str = "MainContent_chkSearchStartWith";

    let mut search_type_ids = HashMap::new();
    search_type_ids.insert(String::from("c"), &search_type_contains_css_id);
    search_type_ids.insert(String::from("sw"), &search_type_startswith_css_id);

    // Set the search_type
    let search_type_css_id: &str = search_type_ids.get(&args.search_type).unwrap();

    let client = ClientBuilder::native().connect("http://localhost:9515").await.expect("failed to connect to WebDriver");

    client.goto("https://wyobiz.wyo.gov/Business/FilingSearch.aspx").await?;

    // Find and set search field with user input
    let search_field = client.form(Locator::Id(search_field_css_id)).await?;
    search_field.set(Locator::Id(search_field_css_id), &args.search_term).await?; 
    
    // Select the specified search type
    client.find(Locator::Id(search_type_css_id)).await?.click().await?;

    // Find search button and submit search form
    client.find(Locator::Id(search_button_css_id)).await?.click().await?;

    // Find search results
    let results = client.wait().for_element(Locator::Css(search_results_css_class)).await?;

    client.close().await
    // let's set up the sequence of steps we want the browser to take
    // .and_then(move |mut c| c.form(Locator::Id(search_field_css_id)))
    // .and_then(move |mut search_field| {
    //     search_field
    //         .set(Locator::Id(search_field_css_id), &args.search_term)
    //         .map(|element| element.client())
    // })
    // .and_then(move |mut c| c.find(Locator::Id(search_button_css_id)))
    // .and_then(|button| button.click())
    // .and_then(move |c| 
    //     c
    //         .wait_for_find(Locator::Css(search_results_css_class))
    //         .map(|e| e.client())
    // )
    // .and_then(move |mut c| c.find_all(Locator::Css(search_result_css_selector)))
    // //.and_then(|mut list_items| list_items.len())
    // .and_then(|c|  {
    //     println!("{}", c.capacity());
    //     Ok(())
    // })
    // .map_err(|e| {
    //     panic!("a WebDriver command failed: {:?}", e);
    // })
    // client.close().await
}
