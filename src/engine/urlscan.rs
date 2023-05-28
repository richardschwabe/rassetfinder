use reqwest;
use serde::{Deserialize};
use crate::colors::{RED, NC, DG};

#[derive(Deserialize, Debug)]
struct Task {
    domain: String,
}
#[derive(Deserialize, Debug)]
struct Item{
    task: Task
}
#[derive(Deserialize, Debug)]
struct ApiResponse{
    results: Vec<Item>
}


pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let url = format!("https://urlscan.io/api/v1/search/?q=domain:{}", domain);
    let response = client.get(&url).send().await.unwrap();
    println!("{DG}Checking {}{NC}", &url);

    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await{
                Ok(parsed) => {
                    for item in parsed.results{

                        if item.task.domain.contains(&domain){
                            sub_domains.push(item.task.domain.trim().replace("*.", "").to_string());
                        }
                    }
                }
                _ => {
                    println!("{RED} Could not parse response {NC}");
                }
            }

        }
        other => {
            println!("{RED} urlscan.io had an error {other} {NC}");
        }
    };
}