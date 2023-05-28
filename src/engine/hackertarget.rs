use reqwest;
use crate::colors::{RED, NC, DG};


pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let url = format!("https://api.hackertarget.com/hostsearch/?q={}", domain);
    let response = client.get(&url).send().await.unwrap();
    println!("{DG}Checking {}{NC}", &url);

    match response.status(){
        reqwest::StatusCode::OK => {
            match response.text().await{
                Ok(parsed) => {
                    for item in parsed.lines(){
                        let columns = item.split(",");

                        for col in columns{
                            if col.contains(&domain){
                                sub_domains.push(col.to_string());
                            }
                            break;
                        }

                    }
                }
                _ => {
                    println!("{RED} Could not parse response {NC}");
                }
            }

        }
        other => {
            println!("{RED} hackertarget.com had an error {other} {NC}");
        }
    };
}