use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml;
use url::Url;

#[derive(Debug, Deserialize)]
struct Browser {
    command: String,
    domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ConfigData {
    default: Option<String>,
    browsers: Vec<Browser>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the URL from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ./surf-chooser <URL>");
        return Ok(());
    }
    let url_arg = Url::parse(&args[1]).unwrap();

    //Default Browser from env or firefox as fallback
    let mut default_browser = env::var("BROWSER").unwrap_or("firefox".to_string());

    if let Some(config_folder) = dirs::config_dir() {
        let relative_path = Path::new("surf-chooser/config.toml");
        let config_path = config_folder.join(relative_path);

        //Check if config file exists
        if config_path.exists() {
            // Read the TOML file contents
            let toml_contents =
                fs::read_to_string(config_path).expect("Failed to read config file");
            // Parse the TOML contents into a Value
            let parsed_toml: ConfigData =
                toml::from_str(&toml_contents).expect("Failed to parse TOML");

            let browsers = parsed_toml.browsers;

            for browser in browsers {
                for domain in browser.domains {
                    if url_arg.host_str() == Some(&domain) {
                        Command::new(&browser.command)
                            .arg(url_arg.as_str())
                            .spawn()
                            .expect("Failed to execute command");
                        return Ok(());
                    }
                }
            }

            // Patch default browser from config if exists
            if let Some(default) = parsed_toml.default {
                default_browser = default;
            }
        } else {
            println!("Config file not found use default browser");
        }
    }

    // Open the URL in a predefined browser
    Command::new(&default_browser)
        .arg(url_arg.as_str())
        .spawn()
        .expect("Failed to execute command");

    return Ok(());
}
