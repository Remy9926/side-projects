use scraper::{ Html, Selector };
use headless_chrome::Browser;
use std::error::Error;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

const GITHUB_API_URL: &str = "https://api.github.com/repos/code-423n4/";

fn main() {
    let url = "https://code4rena.com/contests";
    let mut audits = Vec::new();
    let _ = get_audit_links(url, &mut audits);
    for element in audits {
        let v: Vec<&str> = element.split("/").collect(); 
        let github_url = GITHUB_API_URL.to_owned() + v[v.len() - 1] + "/contents";
        println!("{}", github_url);
        get_repo_info(&github_url)
    }
}

fn get_audit_links(url: &str, audits: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    
    tab.navigate_to(&url)?;
    thread::sleep(Duration::from_millis(2000));
    
    let raw_html = tab.get_content().unwrap();
    let document = Html::parse_document(&raw_html);
    let aria_label_selector = Selector::parse(r#"a[aria-label="Go to audit competition repo (Opens in a new window)"]"#).unwrap();
    
    for element in document.select(&aria_label_selector) {
        audits.push(element.value().attr("href").unwrap().to_string());
    }

    Ok(())
}

fn get_repo_info(url: &str) {
    let client = reqwest::blocking::Client::new();
    let request_builder = client.request(reqwest::Method::GET, url).header("User-Agent", "remy9926");
    let response = request_builder.send().unwrap().text().unwrap();
    println!("{}", response);
}