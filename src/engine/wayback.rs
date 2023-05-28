use reqwest;
use serde_tuple::{Deserialize_tuple};

use crate::colors::{RED, NC, DG};


#[derive(Deserialize_tuple, Debug)]
struct Response {
    domain: String,
    timestamp: String,
    original: String,
    mimetye: String,
    statuscode: String,
    digest: String,
    length: String,
}

pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let url = format!("https://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&collapse=urlkey", domain);

    let response = client.get(&url).send().await.unwrap();
    println!("{DG}Checking {}{NC}", &url);

    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<Vec<Response>>().await{
                Ok(parsed) => {
                    for item in parsed{
                        let domain_url = item.domain.split(")/");
                        for part in domain_url{
                            let sanitized_part = part.trim().replace(",", ".");
                            let part_split = sanitized_part.split(":");
                            for optimised_part in part_split{
                                let domain_parts = optimised_part.rsplit(".").collect::<Vec<&str>>();
                                let final_domain : String = domain_parts.join(".").to_string();
                                if final_domain.contains(&domain){
                                    sub_domains.push(final_domain);
                                }
                                break;
                            }

                            break;
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