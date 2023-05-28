use chrono::{Local};
use clap::Parser;
use reqwest;
use std::fs::File;
use std::io::{BufWriter, Write};
use tokio;

use rassetfinder::colors::{BLUE, DG, GREEN, ITALIC_BLUE, LG, NC, RED, YELLOW};
use rassetfinder::engine::{certspotter, crt, hackertarget, urlscan, virustotal, wayback};



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Domain to lookup
    domain: String,

    /// Save domains to this file as simple list
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<String>
}


/// Prints all domains we gathered
fn print_domains(sub_domaints: &mut Vec<String>) {
    for entry in sub_domaints {
        println!("{}", entry);
    }
}

fn print_banner() {
    let header = "

    ▄▄▄   ▄▄▄· .▄▄ · .▄▄ · ▄▄▄ .▄▄▄▄▄·▄▄▄▪   ▐ ▄ ·▄▄▄▄  ▄▄▄ .▄▄▄
    ▀▄ █·▐█ ▀█ ▐█ ▀. ▐█ ▀. ▀▄.▀·•██  ▐▄▄·██ •█▌▐███▪ ██ ▀▄.▀·▀▄ █·
    ▐▀▀▄ ▄█▀▀█ ▄▀▀▀█▄▄▀▀▀█▄▐▀▀▪▄ ▐█.▪██▪ ▐█·▐█▐▐▌▐█· ▐█▌▐▀▀▪▄▐▀▀▄
    ▐█•█▌▐█ ▪▐▌▐█▄▪▐█▐█▄▪▐█▐█▄▄▌ ▐█▌·██▌.▐█▌██▐█▌██. ██ ▐█▄▄▌▐█•█▌
    .▀  ▀ ▀  ▀  ▀▀▀▀  ▀▀▀▀  ▀▀▀  ▀▀▀ ▀▀▀ ▀▀▀▀▀ █▪▀▀▀▀▀•  ▀▀▀ .▀  ▀

    ";

    println!("{YELLOW}{}{NC}", header);

    let sub_header = format!(
        "
    /--------------------------{ITALIC_BLUE} INFO{NC} ---------------------------\\
    | Github:    https://github.com/richardschwabe/rassetfinder |
    | LinkTree:  https://linktr.ee/richardschwabe               |
    | Author:    Richard Schwabe                                |
    |                                                           |
    |             Feedback Welcome · made with Rust ❤           |
    \\-----------------------------------------------------------/
"
    );
    println!("{LG}{}{NC}", sub_header);

    println!("{DG}Inspired by: https://github.com/tomnomnom/assetfinder{NC}");
    // println!("{BLUE}{ITALIC}$ rassetfinder domain.tld");
    // println!("$ rassetfinder domain.tld --out file_name.txt{NC}");

    println!("\n");
}

/// Writes all Subdomains as a plainlist to a file
fn write_to_file(path: &str, sub_domains: &mut Vec<String>) {

    let output = File::create(path).unwrap();
    let mut stream = BufWriter::new(output);
    for subdomain in sub_domains {
        stream.write_fmt(format_args!("{}\n", subdomain)).unwrap();
    }
    stream.flush().unwrap();
}

#[tokio::main]
async fn main() {
    print_banner();

    // parse Arguments
    let args = Args::parse();

    let domain = args.domain;
    let mut sub_domains: Vec<String> = Vec::new();

    // create a webclient and use that in all the checks
    let now = Local::now();
    let client = reqwest::Client::new();
    println!("{YELLOW}Starting ... please hold the line...{NC}");

    crt::run(&client, &domain, &mut sub_domains).await;
    urlscan::run(&client, &domain, &mut sub_domains).await;
    hackertarget::run(&client, &domain, &mut sub_domains).await;
    certspotter::run(&client, &domain, &mut sub_domains).await;
    wayback::run(&client, &domain, &mut sub_domains).await;
    virustotal::run(&client, &domain, &mut sub_domains).await;

    // remove duplicates, for that the list needs to be sorted
    sub_domains.sort();
    sub_domains.dedup();

    // set default output file or from args
    let mut path: &str =  &format!("{}.txt", &domain.replace("-", "").trim());
    if let Some(output_path) = &args.output.as_deref() {
        path = &output_path;
    }

    let mut highlighted_count = format!("{RED}0{NC}");
    if sub_domains.len() > 0 {
        print_domains(&mut sub_domains);
        highlighted_count = format!("{GREEN}{}{NC}", sub_domains.len());

        //writing to file
        write_to_file(&path, &mut sub_domains);
    } else {
        println!("{RED}No results were found.{NC}");
    }

    println!("{BLUE}╔═════╣ Results");
    println!("{BLUE}╚══╣{NC} Date:              {}", now.to_rfc2822());
    println!("{BLUE}╚══╣{NC} Subdomains Found:  {}", highlighted_count);
    println!("{BLUE}╚══╣{NC} File:              {}", path );

}
