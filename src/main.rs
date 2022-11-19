use colored::*;
use tokio;
use clap::Parser;
use mime::Mime;
use anyhow::{Result, Ok, anyhow};
use reqwest::{Url, Response};
use std::{collections::HashMap, str::FromStr};

#[derive(Parser, Debug)]
#[clap(version = "1.0.0", author = "yovafeng <github.com/DomineCore>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(about="Send a http post reqwest to a server.")]
    Get(Get),
    #[clap(about="Send a http post reqwest to a server with body.")]
    Post(Post),
}

// get subcommand

#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post subcommand

#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_body))]
    body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
    key: String,
    value: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}, body must like `a=1 b=2···`", s));
        Ok(Self{
            key: (split.next().ok_or_else(err)?).to_string(),
            value: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// validate subcommand paramas url
fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

// validate subcommand paramas body
fn parse_body(body: &str) -> Result<KvPair> {
    Ok(body.parse()?)
}

async fn get(client:reqwest::Client, args:&Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_full_response(resp).await?)
}

async fn post(client:reqwest::Client, args:&Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(pair.key.clone(), pair.value.clone());
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_full_response(resp).await?)
}

// print server version & status information
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// print response headers
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers().iter() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n");
}
// print response body
fn print_body(m: Option<Mime>,body: String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(&body).unwrap().cyan())
        }
        _ => println!("{}",body)
    }
}
// print full response
async fn print_full_response(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, body);
    Ok(())
}

// get response content type
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
       .get("Content-Type")
       .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // build a http client
    let client = reqwest::Client::new();
    // send http post request
    let result = match opts.subcommand {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}