use reqwest;
use serde::{Deserialize};

use crate::colors::{RED, NC, DG};
use std::env;

#[derive(Deserialize, Debug)]
struct Item{
    id: String
}


#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Item>
}

pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let vt_api_key : String = env::var("VT_API_KEY").unwrap_or_default();
    if vt_api_key.is_empty(){
        println!("{RED}VT_API_KEY not set.{NC} Cannot scan with VirusTotal.com");
        return ;
    }

    let url = format!("https://www.virustotal.com/api/v3/domains/{}/subdomains?limit=100", domain);

    let response = client.get(&url).header("x-apikey", vt_api_key).send().await.unwrap();

    println!("{DG}Checking {}{NC}", &url);
    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await{
                Ok(parsed) => {
                    for item in parsed.data{
                        let domain_url = item.id;
                        if domain_url.contains(&domain){
                            sub_domains.push(domain_url.replace("*.", "").trim().to_string());
                        }

                    }
                }
                Err(err) => {
                    println!("{RED} Could not parse response {NC}{}", err);
                }
            }

        }
        other => {
            println!("{RED} certspotter.com had an error {other} {NC}");
        }
    };
}