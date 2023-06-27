use scraper::{Html, Selector};
use headless_chrome::Browser;
use std::error::Error;
use std::time::Duration;
use std::thread;
use serde_json::{Value};
use dotenv::dotenv;
use ethers_solc::{Project, ProjectPathsConfig};
use std::fs;
use std::io::prelude::*;

const GITHUB_API_URL: &str = "https://api.github.com/repos/code-423n4/";
static mut SOLIDITY_FILES: Vec<String> = Vec::new();

fn main() {
    let _ = get_contract_bytecode();
    //let output: Vec<_> = project.compile().unwrap().into_contract_bytecodes().collect();
    //println!("{:?}", output);
    //let url = "https://raw.githubusercontent.com/code-423n4/2023-06-lybra/main/contracts/mocks/chainLinkMock.sol";
    //let mut file = fs::File::create("./contracts/chainLinkMock.sol").unwrap();
    //file.write_all(b"Henlo bera");
    //let mut opened_file = fs::File::open("./contracts/chainLinkMock.sol").unwrap();
    //let mut contents = String::new();
    //opened_file.read_to_string(&mut contents).unwrap();
    //println!("{}", contents);
    //
    //dotenv().ok();
    //let url = "https://code4rena.com/contests";
    //let mut audits = Vec::new();
    //let _ = get_audit_links(url, &mut audits);
    //for element in audits {
    //    let v: Vec<&str> = element.split("/").collect(); 
    //    let github_url = GITHUB_API_URL.to_owned() + v[v.len() - 1] + "/contents";
    //    println!("{}", github_url);
    //    get_content_info(&github_url);
    //}
    //unsafe {
    //    println!("{:?}", SOLIDITY_FILES);
    //}
    //println!("{}", dotenv::var("GITHUB_API_KEY").unwrap()); 
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
        audits.push(element.value().attr("href").unwrap().to_owned());
    }

    Ok(())
}

fn get_content_info(url: &str) {
    let client = reqwest::blocking::Client::new();
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert("User-Agent", reqwest::header::HeaderValue::from_str("remy9926").unwrap());
    header_map.insert("Authorization", reqwest::header::HeaderValue::from_str(&dotenv::var("GITHUB_API_KEY").unwrap()).unwrap());
    
    let request_builder = client.request(reqwest::Method::GET, url).headers(header_map);
    let response = request_builder.send().unwrap().text().unwrap();
    //response = response[1..response.len() - 1].to_string();
    let _ = parse_json(&response);
}

fn parse_json(response: &str) -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_str(response).unwrap();
    for i in 0..json.as_array().unwrap().len() {
        let file_json = json.as_array().unwrap().get(i);
        let file_type = file_json.unwrap()["type"].as_str().unwrap();
        if file_type == "file" {
            if file_json.unwrap()["name"].as_str().unwrap().ends_with(".sol") {
                println!("{}", file_json.unwrap()["name"].as_str().unwrap().to_owned() + " is a solidity file!");
                unsafe {
                    SOLIDITY_FILES.push(file_json.unwrap()["name"].as_str().unwrap().to_owned());
                }
            }
        } else {
            println!("{}", file_json.unwrap()["name"].as_str().unwrap().to_owned() + " is not a solidity file");
            let dir_url = file_json.unwrap()["url"].as_str().unwrap();
            let _ = get_content_info(dir_url);
        }
    }
    
    Ok(())
}

fn get_contract_bytecode() { //push all solidity files into vector and see if they compile to some bytecode then figure out how to import
    unsafe {
        let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat("./contracts").unwrap())
        .set_auto_detect(true)
        .build()
        .unwrap();
        for file in &SOLIDITY_FILES {
            let output = project.compile_file("./contracts/".to_owned() + file).unwrap(); //can replace the file with any file that you want
            let get_bytecode: Vec<_> = output.into_contract_bytecodes().collect(); //since it compiles file by file, the size of this vector will be 1
            let (_, compact_contract_bytecode) = &get_bytecode[0]; //extract the CompactContractBytecode object
            println!("{:?}", compact_contract_bytecode.bytecode.clone().unwrap().object.into_bytes().unwrap().to_string()); //finally get the bytecode and print it out
        }
    }
}