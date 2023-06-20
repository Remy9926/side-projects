use scraper::Selector;
use scraper::Html;

fn main() {
    println!("cargo:rerun-if-changed=./.lalrpop");
    let url = "https://code4rena.com/contests";
    let response = reqwest::blocking::get(url).expect("Could not load url");
    let raw_html = response.text().unwrap();
    println!("{}", raw_html);
}
