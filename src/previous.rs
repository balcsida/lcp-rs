use configparser::ini::Ini;
use serde_json;
use reqwest;

// Read ini file .lcp from user home directory
fn read_config() -> Ini {
    let mut config = Ini::new();
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".lcp");
    config.load(config_path).unwrap();
    config
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Read config file
    // And get the default_remote from the default section
    // If it doesn't exist, use the first remote
    // If it exists, get it from the 'remote "default_remote"' section
    // Make a GET call to 'api.' + infrastructure with the token to get the API version
    // Make a GET call to 'api.' + infrastructure + '/projects' to get the list of projects

    let config = read_config();

    let default_remote = config.get("default", "default_remote").unwrap();

    println!("Default remote is: {}", default_remote);

    let infrastructure = config.get(&format!("remote \"{}\"",default_remote), "infrastructure").unwrap();
    let token = config.get(&format!("remote \"{}\"",default_remote), "token").unwrap();

    println!("Infrastructure is: {}", infrastructure);
    println!("Token is: {}", token);

    // Make an asyncronus GET call to 'api.' + infrastructure withouth the token to get the API version
    // The response JSON looks like this:
    // {"version":"5.40.1","domains":{"infrastructure":"liferay.cloud","service":"lfr.cloud"}}
    // Print the API version to the console

    let api_url = format!("https://api.{}", infrastructure);
    let client = reqwest::Client::new();

    let api_response = client.get(&api_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("API version is: {}", api_response["version"]);

    // Make an asyncronus GET call to 'api.' + infrastructure + '/projects' with the token to get the list of projects
    // Print the list of projectId to the console

    let projects_url = format!("{}/projects", api_url);

    let projects_response = client.get(&projects_url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("List of projects: {}", projects_response);

    Ok(())
}