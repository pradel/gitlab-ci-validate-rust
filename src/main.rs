use std::collections::HashMap;
extern crate clap;
use clap::{App, Arg};
use exitfailure::ExitFailure;
use failure::ResultExt;
extern crate reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

// This struct represent the response from the gitlab endpoint
#[derive(Debug, Serialize, Deserialize)]
struct GitlabResponse {
    status: String,
    errors: Vec<String>,
}

fn main() -> Result<(), ExitFailure> {
    // Define the cli options
    let matches = App::new("myapp")
        .version("0.0.1")
        .about("Checks if your .gitlab-ci.yml file is valid")
        .arg(
            Arg::with_name("host")
                .short("H")
                .long("host")
                .value_name("HOST")
                .help("Sets a custom gitlab host")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("private_token")
                .short("P")
                .long("private-token")
                .value_name("HOST")
                .help("Sets a custom gitlab host, defaults to ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILE_PATH")
                .help("Sets the input file to use")
                .index(1),
        )
        .get_matches();

    // Gets a value for file path if supplied by user, or defaults to ".gitlab-ci.yml"
    let file = matches.value_of("FILE_PATH").unwrap_or(".gitlab-ci.yml");

    // Gets a value for gitlab host if supplied by user
    let host = matches.value_of("host");

    // Gets a value for gitlab private_token if supplied by user
    let private_token = matches.value_of("private_token");

    Ok(check_gitlab(file, host, private_token)?)
}

fn check_gitlab(
    file: &str,
    host_param: Option<&str>,
    private_token: Option<&str>,
) -> Result<(), failure::Error> {
    let host = host_param.unwrap_or("https://gitlab.com");

    // Get the file content or fail if file not found
    let content = std::fs::read_to_string(file)
        .with_context(|_| format!("Could not read file `{}`", file))?;

    // Create the POST request body
    let mut map = HashMap::new();
    map.insert("content", &content);

    let full_url = format!("{}/api/v4/ci/lint", host);

    let mut headers = HeaderMap::new();

    if let Some(private_token) = private_token {
        headers.insert("Private-Token", HeaderValue::from_str(private_token)?);
    }

    let client = reqwest::Client::new();
    let mut res = client.post(&full_url).headers(headers).json(&map).send()?;

    let data: GitlabResponse = res.json()?;

    // We check the response from gitlab
    if data.status == "invalid" {
        let mut error_msg = format!("{} is invalid", file);
        for error in &data.errors {
            error_msg = error_msg + "\n- " + error;
        }
        return Err(failure::err_msg(error_msg));
    }

    println!("{} is valid", file);
    Ok(())
}
