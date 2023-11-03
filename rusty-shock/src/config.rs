use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs,io,io::Write,fs::File,str::FromStr};
use swing::{Logger,Config as LoggerConfig};
use log::LevelFilter;


//expect root Table and configure subtables, osc
#[derive(Deserialize)]
pub struct Config {
    pub osc: Osc,
    pub logging: Logging,
}

// Expected OSC config, listen_port,send_port,ip_address
#[derive(Deserialize)]
pub struct Osc {
    pub listen_port: u16,
    pub send_port: u16,
    pub ip_address: String,
}

#[derive(Deserialize)]
pub struct Logging {
    pub level: String,
}

// Make CONFIG a public static so it's accessible from other modules
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path = "config.toml";
    let config_str = fs::read_to_string(config_path).or_else(|e: std::io::Error| {
        if e.kind() == io::ErrorKind::NotFound {
            create_config().and_then(|_| fs::read_to_string(config_path))
        } else {
            Err(e)
        }
    }).expect("Unable to handle config file");
    toml::from_str(&config_str).expect("Failed to parse config")
});

pub fn get_config() -> &'static Config {
    &CONFIG
}

// This function is a placeholder for the actual implementation of creating a config.
fn create_config() -> io::Result<()> {
    log::info!("Creating a new config file...");

    let mut config_file = fs::File::create("config.toml")?;

    // This is the default config data that will be written to the file.
    // My CoDE Is SelF DoCuMeNtInG
    let config_data = r#"
    [osc]
    # This is the port that RustyShock is listening to (EX: VRChat's send port) 
    # Default: 9001 
    listen_port = 9001
    
    # This is the port that RustShock is sending on (EX: VRChat's listen port) 
    # Default: 9000 
    send_port = 9000
    
    # This is the IP Address of the computer with the OSC Client on it (EX: VRChat) 
    # Default: 127.0.0.1 (LocalHost)
    ip_address = "127.0.0.1"
    
    [OpenShock-Legacy]
    # OpenShock Legacy's endpoint to issue commands to.
    # Default: OpenShock.Local
    api_endpoint = "OpenShock.Local"

    [logging]
    # This is the log level that RustyShock will use.
    # Default: info
    level = "info"
    "#;

    //for some odd reason if I dont do the conversion to bytes it wont write to the file even with as_bytes in write_all
    let config_bytes = config_data.as_bytes();
    //println!("{:?}",config_bytes);
    config_file.write_all(config_bytes)?; //write default config
    Ok(())
}

// generate the logging config for each logger implementation across the files
pub fn get_logging_config() -> LoggerConfig {
    // This might be unnecessary and could probably be directly called in the let level line
    let log_level_str = &CONFIG.logging.level;
    
    // Parse the log level from string, defaulting to 'Debug' if there's an error
    let level = LevelFilter::from_str(log_level_str).unwrap_or_else(|err|{ 
        println!("Unable to parse log level from config: {}. Defaulting to 'Debug'", log_level_str);
        LevelFilter::Debug
    });

    LoggerConfig {
        level: level,
        ..Default::default()
    }
}