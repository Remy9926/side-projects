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
use std::collections::HashMap;

const GITHUB_API_URL: &str = "https://api.github.com/repos/code-423n4/";

fn main() {
    //let _ = create_contract_locally("https://raw.githubusercontent.com/code-423n4/2023-06-lybra/main/contracts/mocks/mockEUSD.sol");
    //let _ = get_contract_bytecode();
    //let url = "https://raw.githubusercontent.com/code-423n4/2023-06-lybra/main/contracts/mocks/chainLinkMock.sol";
    //file.write_all(b"Henlo bera");
    //let mut opened_file = fs::File::open("./contracts/chainLinkMock.sol").unwrap();
    //let mut contents = String::new();
    //opened_file.read_to_string(&mut contents).unwrap();
    //println!("{}", contents);
    //
    dotenv().ok();
    let mut SOLIDITY_FILES: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    let url = "https://code4rena.com/contests";
    let mut audits = Vec::new();
    let _ = get_audit_links(url, &mut audits);
    for element in audits {
        let v: Vec<&str> = element.split("/").collect(); 
        let github_url = GITHUB_API_URL.to_owned() + v[v.len() - 1] + "/contents";
        println!("{}", github_url);
        get_content_info(&github_url, &mut SOLIDITY_FILES);
    }
    unsafe {
        println!("{:?}", SOLIDITY_FILES);
    }
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

fn get_content_info(url: &str, files: &mut HashMap<&str, Vec<Vec<&str>>>) {
    let client = reqwest::blocking::Client::new();
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert("User-Agent", reqwest::header::HeaderValue::from_str("remy9926").unwrap());
    header_map.insert("Authorization", reqwest::header::HeaderValue::from_str(&dotenv::var("GITHUB_API_KEY").unwrap()).unwrap());
    
    let request_builder = client.request(reqwest::Method::GET, url).headers(header_map);
    let response = request_builder.send().unwrap().text().unwrap();
    //response = response[1..response.len() - 1].to_string();
    let _ = parse_json(&response, files);
}

fn parse_json(response: &str, files: &mut HashMap<&str, Vec<Vec<&str>>>) -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_str(response).unwrap();
    for i in 0..json.as_array().unwrap().len() {
        let file_json = json.as_array().unwrap().get(i);
        let file_type = file_json.unwrap()["type"].as_str().unwrap();
        if file_type == "file" {
            if file_json.unwrap()["name"].as_str().unwrap().ends_with(".sol") {
                // create contract locally here pass the download_url field into fn
                let split_string: Vec<_> = file_json.unwrap()["url"].as_str().unwrap().split("/").collect();
                let repo_name = split_string[5];
                let file_name_download_url = vec![file_json.unwrap()["name"].as_str().unwrap(), file_json.unwrap()["download_url"].as_str().unwrap()];
                if files.contains_key(&repo_name) {
                    let mut repo_files = files.get_mut(repo_name).unwrap();
                    repo_files.push(file_name_download_url);
                } else {
                    files.insert(&repo_name, Vec::new());
                }
            }
        } else {
            let dir_url = file_json.unwrap()["url"].as_str().unwrap();
            let _ = get_content_info(dir_url, files);
        }
    }
    
    Ok(())
}

fn get_contract_bytecode() { //push all solidity files into vector and see if they compile to some bytecode then figure out how to import
    unsafe {
        //SOLIDITY_FILES.push("chainLinkMock.sol".to_owned());
        //SOLIDITY_FILES.push("mockEtherPriceOracle.sol".to_owned());
        //SOLIDITY_FILES.push("mockCurve.sol".to_owned());
        let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat("./contracts").unwrap())
        .set_auto_detect(true)
        .build()
        .unwrap();
        //for file in &SOLIDITY_FILES {
        //    let output = project.compile_file("./contracts/".to_owned() + file).unwrap(); //can replace the file with any file that you want
        //    let get_bytecode: Vec<_> = output.into_contract_bytecodes().collect(); //since it compiles file by file, the size of this vector will be 1
        //    let (_, compact_contract_bytecode) = &get_bytecode[0]; //extract the CompactContractBytecode object
        //    println!("{:?}", compact_contract_bytecode.bytecode.clone().unwrap().object.into_bytes().unwrap().to_string()); //finally get the bytecode and print it out
        //}
    }
}

fn create_contract_locally(download_url: &str) { //create the contract locally along with all the imports to then be able to compile + get bytecode
    // send request before creating file 
    // let opened_file = fs::File::open("./contracts/mockEUSD.sol").unwrap();
    // let content = fs::read_to_string("./contracts/mockEUSD.sol").unwrap(); //wont need these two
    // let all_lines: Vec<_> = content.split("\n").collect();                 //lines because will be fetching data directly from github api
    let client = reqwest::blocking::Client::new();
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert("User-Agent", reqwest::header::HeaderValue::from_str("remy9926").unwrap());
    header_map.insert("Authorization", reqwest::header::HeaderValue::from_str(&dotenv::var("GITHUB_API_KEY").unwrap()).unwrap());
    
    let request_builder = client.request(reqwest::Method::GET, download_url).headers(header_map);
    let response = request_builder.send().unwrap().text().unwrap();
    println!("{}", response);
    let line_by_line: Vec<_> = response.split("\n").collect();

    let mut write_to_file = fs::File::create("./contracts/test.sol").unwrap();
    for line in line_by_line {
        let line_content: Vec<_> = line.split(" ").collect();
        if line_content[0] == "import" {
            let path: Vec<_> = line_content[1].split("/").collect();
            if path[0] != r#""@openzeppelin"# {
                let file_path = path[path.len() - 1];
                let import_file = file_path[..file_path.len() - 2].to_owned();
                writeln!(write_to_file, "{}", r#"import "../"#.to_owned() + &import_file + r#"";"#).unwrap();
            } else {
                writeln!(write_to_file, "{}", line).unwrap();
            }
        } else {
            writeln!(write_to_file, "{}", line).unwrap();
        }
    }

    let project = Project::builder()
    .paths(ProjectPathsConfig::hardhat("./contracts").unwrap())
    .set_auto_detect(true)
    .build()
    .unwrap();

    let output = project.compile_file("./contracts/test.sol").unwrap(); //can replace the file with any file that you want
    let get_bytecode: Vec<_> = output.into_contract_bytecodes().collect(); //since it compiles file by file, the size of this vector will be 1
    let (_, compact_contract_bytecode) = &get_bytecode[0]; //extract the CompactContractBytecode object
    println!("{:?}", compact_contract_bytecode.bytecode.clone().unwrap().object.into_bytes().unwrap().to_string());

    //let mut write_to_file = fs::File::create("./contracts/".to_owned() + &file_name).unwrap(); //create a new file replace test.sol with the actual .sol file name

    //for line in all_lines { // need all_lines to be equal to the download_url content.split("\n")
    //    let line_content: Vec<_> = line.split(" ").collect();
    //    if line_content[0] == "import" {
    //        let path: Vec<_> = line_content[1].split("/").collect();
    //        if path[0] != r#""@openzeppelin"# {
    //            let file_path = path[path.len() - 1];
    //            let import_file = file_path[..file_path.len() - 2].to_owned();
    //            writeln!(write_to_file, "{}", r#"import "./"#.to_owned() + &import_file + r#"";"#).unwrap();
    //        } else {
    //            writeln!(write_to_file, "{}", line).unwrap();
    //        }
    //    } else {
    //        writeln!(write_to_file, "{}", line).unwrap();
    //    }
    //}
}