use scraper::{ Html, Selector };
use headless_chrome::Browser;
use std::error::Error;
use std::time::Duration;
use std::thread;

fn main() {
    let url = "https://code4rena.com/contests";
    let mut audits = Vec::new();
    get_audit_links(url, &mut audits);
    for element in audits {
        println!("{:?}", element);
    }
}

fn get_audit_links(url: &str, mut audits: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
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