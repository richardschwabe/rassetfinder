use reqwest;
use serde::{Deserialize};
use crate::colors::{RED, NC, DG};

#[derive(Deserialize, Debug)]
struct CrtEntry {
    name_value: String,
}

impl CrtEntry{
    fn get_trimmed_names(&self) -> Vec<String>{
        let trimmed_name = self.name_value.trim().to_string();
        let lines = trimmed_name.lines().collect::<Vec<_>>();
        let mut names : Vec<String> = Vec::new();

        for item in lines{
            names.push(item.to_string());
        }
        return names;
    }
}


pub async fn run(client : &reqwest::Client , domain: &str, sub_domains : &mut Vec<String> ) {
    let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    let response = client.get(&url).send().await.unwrap();
    println!("{DG}Checking {}{NC}", &url);
    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<Vec<CrtEntry>>().await{
                Ok(parsed) => {
                    for element in parsed{
                        let mut names: Vec<String> = element.get_trimmed_names();
                        sub_domains.append(&mut names);
                    }
                }
                _ => {
                    println!("{RED} Could not parse response {NC}");
                }
            }

        }
        other => {
            println!("{RED} crt.sh had an error {other} {NC}");
        }
    };
}