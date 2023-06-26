use scraper::{ Html, Selector };
use headless_chrome::Browser;
use std::error::Error;
use std::time::Duration;
use std::thread;
use serde_json::{ Value };

const GITHUB_API_URL: &str = "https://api.github.com/repos/code-423n4/";
static mut SOLIDITY_FILES: Vec<String> = Vec::new();

fn main() {
    let url = "https://code4rena.com/contests";
    let mut audits = Vec::new();
    let _ = get_audit_links(url, &mut audits);
    for element in audits {
        let v: Vec<&str> = element.split("/").collect(); 
        let github_url = GITHUB_API_URL.to_owned() + v[v.len() - 1] + "/contents";
        println!("{}", github_url);
        get_repo_info(&github_url);
    }
    unsafe {
        println!("{:?}", SOLIDITY_FILES.len());
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
    //response = response[1..response.len() - 1].to_string();
    let _ = parse_json(&response);
}

fn parse_json(response: &str) -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_str(response).unwrap();
    for i in 0..json.as_array().unwrap().len() {
        let file_json = json.as_array().unwrap().get(i);
        println!("{}", file_json.unwrap()["name"].as_str().unwrap());
        let file_type = file_json.unwrap()["type"].as_str().unwrap();
        if file_type == "file" {
            if file_json.unwrap()["name"].as_str().unwrap().ends_with(".sol") {
                println!("Is a solidity file!");
                unsafe {
                    SOLIDITY_FILES.push(file_json.unwrap()["name"].as_str().unwrap().to_string() + " is a solidity file");
                }
            } else {
                println!("Not a solidity file!");
                unsafe {
                    SOLIDITY_FILES.push(file_json.unwrap()["name"].as_str().unwrap().to_string() + " is not a solidity file!");
                }
            }
        } else {
            let dir_url = file_json.unwrap()["url"].as_str().unwrap();
            println!("{}", dir_url);
        }
        println!("");
    }
    
    Ok(())
}