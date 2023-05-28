use reqwest;
use serde::{Deserialize};
use crate::colors::{RED, NC, DG};

#[derive(Deserialize, Debug)]
struct Domains {
    dns_names: Vec<String>,
}

pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let url = format!("https://api.certspotter.com/v1/issuances?domain={}&include_subdomains=true&match_wildcards=true&expand=dns_names", domain);

    let response = client.get(&url).send().await.unwrap();
    println!("{DG}Checking {}{NC}", &url);

    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<Vec<Domains>>().await{
                Ok(parsed) => {
                    for item in parsed{
                        for dns_name in item.dns_names{
                            if dns_name.contains(&domain){
                                sub_domains.push(dns_name.replace("*.", "").trim().to_string());
                            }
                        }
                    }
                }
                _ => {
                    println!("{RED} Could not parse response {NC}");
                }
            }

        }
        other => {
            println!("{RED} certspotter.com had an error {other} {NC}");
        }
    };
}