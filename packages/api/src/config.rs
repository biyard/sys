use std::collections::HashSet;

use bdk::prelude::*;
use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub auth: AuthConfig,
    pub aws: AwsConfig,
    pub database: DatabaseConfig,
    pub allowed_emails: HashSet<&'static str>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            auth: AuthConfig::default(),
            aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
            allowed_emails: option_env!("ALLOWED_EMAILS")
                .unwrap_or("")
                .split(',')
                .collect::<HashSet<&str>>(),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
