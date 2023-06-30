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

fn main() {
    dotenv().ok();
    let url = "https://code4rena.com/contests";
    let mut result: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let mut contract_urls: Vec<String> = Vec::new();
    let mut audits = Vec::new();
    let _ = get_audit_links(url, &mut audits);

    for element in audits {
        let v: Vec<&str> = element.split("/").collect();
        let repo_name =  v[v.len() - 1];
        let github_url = GITHUB_API_URL.to_owned() + &repo_name + "/contents";
        get_repo_info(&github_url, &mut contract_urls);
        //entire repo has been parsed for .sol files
        for contract_url in &contract_urls {
        //each element is the download url
            let _ = create_contract_locally(&contract_url);
        }
        let vector_of_tuples = get_repo_bytecodes(&contract_urls);
        result.push((repo_name.to_owned(), vector_of_tuples));
        // clear contract url vector after each repo
        let _ = contract_urls.clear();
    }
    println!("{:?}", result);
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

fn get_repo_info(url: &str, contract_urls: &mut Vec<String>) {
    let client = reqwest::blocking::Client::new();
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert("User-Agent", reqwest::header::HeaderValue::from_str("remy9926").unwrap());
    header_map.insert("Authorization", reqwest::header::HeaderValue::from_str(&dotenv::var("GITHUB_API_KEY").unwrap()).unwrap());
    
    let request_builder = client.request(reqwest::Method::GET, url).headers(header_map);
    let response = request_builder.send().unwrap().text().unwrap();
    //response = response[1..response.len() - 1].to_string();
    let _ = parse_json(&response, contract_urls);
}

fn parse_json(response: &str, contract_urls: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_str(response).unwrap();
    for i in 0..json.as_array().unwrap().len() {
        let file_json = json.as_array().unwrap().get(i);
        let file_type = file_json.unwrap()["type"].as_str().unwrap();
        if file_type == "file" {
            if file_json.unwrap()["name"].as_str().unwrap().ends_with(".sol") && !file_json.unwrap()["name"].as_str().unwrap().ends_with(".t.sol") {
                contract_urls.push(file_json.unwrap()["download_url"].as_str().unwrap().to_owned());
            }
        } else {
            let dir_url = file_json.unwrap()["url"].as_str().unwrap();
            let _ = get_repo_info(dir_url, contract_urls);
        }
    }
    
    Ok(())
}

fn create_contract_locally(contract_url: &str) { //create the contract locally along with all the imports to then be able to compile + get bytecode
    let client = reqwest::blocking::Client::new();
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert("User-Agent", reqwest::header::HeaderValue::from_str("remy9926").unwrap());
    header_map.insert("Authorization", reqwest::header::HeaderValue::from_str(&dotenv::var("GITHUB_API_KEY").unwrap()).unwrap());

    let request_builder = client.request(reqwest::Method::GET, contract_url).headers(header_map);
    let response = request_builder.send().unwrap().text().unwrap();

    let split_url: Vec<_> = contract_url.split("/").collect();
    let file_name = split_url[split_url.len() - 1];
    let contract_code: Vec<_> = response.split("\n").collect();

    let mut write_to_file = fs::File::create("./contracts/".to_owned() + &file_name).unwrap();
    let mut statement_string = String::new();

    for i in 0..contract_code.len() {
        if contract_code[i].contains("import") && contract_code[i].ends_with("{") {
            statement_string += &(contract_code[i].trim().to_owned() + " ");
        } else if statement_string != "" {
            statement_string += &(contract_code[i].trim().to_owned() + " ");
            if contract_code[i].ends_with(";") {
                statement_string = statement_string.trim().to_owned();
                let split_statement: Vec<_> = statement_string.split(" ").collect();
                let split_line: Vec<_> = split_statement[split_statement.len() - 1].split("/").collect();
                let file_name = split_line[split_line.len() - 1];
                let new_line = split_statement[0..split_statement.len() - 1].join(" ");
                if !split_line[0].contains("@") {
                    let file_name = split_line[split_line.len() - 1];
                    let import_line = get_new_import_line(file_name.to_owned());
                    writeln!(write_to_file, "{}", new_line + &import_line).unwrap();
                } else {
                    writeln!(write_to_file, "{}", statement_string.clone()).unwrap();
                }
                statement_string.clear();
            }
        } else {
            let line_content: Vec<_> = contract_code[i].split(" ").collect();
            if line_content[0] == "import" {
                let split_line: Vec<_> = line_content[line_content.len() - 1].split("/").collect();
                if !split_line[0].contains("@") {
                    let file_name = split_line[split_line.len() - 1];
                    let import_line = get_new_import_line(file_name.to_owned());
                    let new_string = line_content[..line_content.len() - 1].join(" ");
                    writeln!(write_to_file, "{}", new_string + &import_line).unwrap();
                } else {
                    writeln!(write_to_file, "{}", contract_code[i]).unwrap();
                }
            } else {
                writeln!(write_to_file, "{}", contract_code[i]).unwrap();
            }
        }
    }
}

fn get_repo_bytecodes(contract_urls: &Vec<String>) -> Vec<(String, String)> { //push all solidity files into vector and see if they compile to some bytecode then figure out how to import
    let mut results: Vec<(String, String)> = Vec::new();
    let project = Project::builder()
    .paths(ProjectPathsConfig::hardhat("./contracts").unwrap())
    .set_auto_detect(true)
    .build()
    .unwrap();
    for url in contract_urls {
        //
        let split_string: Vec<_> = url.split("/").collect();
        let file_name = split_string[split_string.len() - 1];
        let output = project.compile_file("contracts/".to_owned() + file_name). unwrap();
        if !output.has_compiler_errors() {
            let get_bytecode: Vec<_> = output.into_contract_bytecodes().collect(); //since it compiles file by file, the size of this vector will be 1
            let (_, compact_contract_bytecode) = &get_bytecode[0]; //extract the CompactContractBytecode object
            let bytecode_string = compact_contract_bytecode.bytecode.clone().unwrap().object.into_bytes().unwrap().to_string();
            let tuple = (file_name.to_owned(), bytecode_string);
            results.push(tuple);
        }
    }
    results
}

fn get_new_import_line(file_name: String) -> String {
    let quotation = &file_name[file_name.len() - 2..file_name.len() - 1];
    if quotation == '"'.to_string() {
        return r#" "../"#.to_owned() + &file_name
    } else {
        return r#" '../"#.to_owned() + &file_name
    }
}