// src/cli.rs
use crate::config::AppConfig;
use clap::{Arg, ArgMatches, Command};
use std::{path::PathBuf, process::exit};

pub fn build_cli() -> Command {
    Command::new("Hubuum CLI")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Specify a custom configuration file"),
        )
        .arg(
            Arg::new("hostname")
                .long("hostname")
                .value_name("HOST")
                .env("HUBUUM_CLI__SERVER__HOSTNAME")
                .help("Set the server hostname"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .env("HUBUUM_CLI__SERVER__PORT")
                .help("Set the server port"),
        )
        .arg(
            Arg::new("protocol")
                .long("protocol")
                .value_name("PROTOCOL")
                .env("HUBUUM_CLI__SERVER__PROTOCOL")
                .ignore_case(true)
                .value_parser(["http", "https"])
                .help("Set the server protocol (http or https)"),
        )
        .arg(
            Arg::new("ssl_validation")
                .long("ssl-validation")
                .value_name("BOOL")
                .env("HUBUUM_CLI__SERVER__SSL_VALIDATION")
                .help("Enable or disable SSL validation"),
        )
        .arg(
            Arg::new("username")
                .long("username")
                .value_name("NAME")
                .env("HUBUUM_CLI__SERVER__USERNAME")
                .help("Set the username"),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .value_name("PASSWORD")
                .env("HUBUUM_CLI__SERVER__PASSWORD")
                .help("Set the password (ideally use ENV)"),
        )
        .arg(
            Arg::new("cache_time")
                .long("cache-time")
                .value_name("SECONDS")
                .env("HUBUUM_CLI__CACHE__TIME")
                .help("Set the cache time in seconds"),
        )
        .arg(
            Arg::new("cache_size")
                .long("cache-size")
                .value_name("BYTES")
                .env("HUBUUM_CLI__CACHE__SIZE")
                .help("Set the cache size in bytes"),
        )
        .arg(
            Arg::new("cache_disable")
                .long("cache-disable")
                .value_name("BOOL")
                .env("HUBUUM_CLI__CACHE__DISABLE")
                .help("Enable or disable caching"),
        )
        .arg(
            Arg::new("completion_disable_api")
                .long("completion-api-disable")
                .value_name("BOOL")
                .env("HUBUUM_CLI__COMPLETION__DISABLE_API_RELATED")
                .help("Disable API-related completions"),
        )
        .arg(
            Arg::new("command")
                .long("command")
                .value_name("COMMAND")
                .help("Run a command and exit"),
        )
        .arg(
            Arg::new("source")
                .long("source")
                .value_name("FILE")
                .help("Run commands from a file and exit"),
        )
}

pub fn get_cli_config_path(matches: &ArgMatches) -> Option<PathBuf> {
    matches.get_one::<String>("config").map(PathBuf::from)
}

pub fn update_config_from_cli(config: &mut AppConfig, matches: &ArgMatches) {
    if let Some(hostname) = matches.get_one::<String>("hostname") {
        config.server.hostname = hostname.to_string();
    }
    if let Some(port) = matches.get_one::<String>("port") {
        if let Ok(port) = port.parse() {
            config.server.port = port;
        }
    }
    if let Some(protocol) = matches.get_one::<String>("protocol") {
        config.server.protocol = protocol.parse().unwrap_or_else(|_| {
            eprintln!("Invalid protocol. Must be 'http' or 'https'");
            exit(1);
        });
    }
    if let Some(ssl_validation) = matches.get_one::<String>("ssl_validation") {
        if let Ok(ssl_validation) = ssl_validation.parse() {
            config.server.ssl_validation = ssl_validation;
        }
    }
    if let Some(username) = matches.get_one::<String>("username") {
        config.server.username = username.to_string();
    }
    if let Some(password) = matches.get_one::<String>("password") {
        config.server.password = Some(password.to_string());
    }
    if let Some(cache_time) = matches.get_one::<String>("cache_time") {
        if let Ok(cache_time) = cache_time.parse() {
            config.cache.time = cache_time;
        }
    }
    if let Some(cache_size) = matches.get_one::<String>("cache_size") {
        if let Ok(cache_size) = cache_size.parse() {
            config.cache.size = cache_size;
        }
    }
    if let Some(cache_disable) = matches.get_one::<String>("cache_disable") {
        if let Ok(cache_disable) = cache_disable.parse() {
            config.cache.disable = cache_disable;
        }
    }
    if let Some(disable_api_completion) = matches.get_one::<String>("completion_disable_api") {
        if let Ok(completion_disable_api) = disable_api_completion.parse() {
            config.completion.disable_api_related = completion_disable_api;
        }
    }
}
