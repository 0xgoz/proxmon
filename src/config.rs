use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub proxmox_hosts: Vec<ProxmoxHost>,
    #[serde(default)]
    pub manual_hosts: Vec<ManualHost>,
    #[serde(default)]
    pub ip_overrides: Vec<IpOverride>,
    #[serde(default)]
    pub ansible_defaults: AnsibleDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxHost {
    pub name: String,
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub api_token_id: String,
    pub api_token_secret: String,
    #[serde(default)]
    pub verify_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualHost {
    pub name: String,
    pub ip: String,
    #[serde(rename = "type")]
    pub host_type: String,
    pub ansible_user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpOverride {
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsibleDefaults {
    #[serde(default = "default_python_interpreter")]
    pub python_interpreter: String,
    #[serde(default = "default_true")]
    #[serde(rename = "become")]
    pub ansible_become: bool,
    #[serde(default = "default_become_method")]
    pub become_method: String,
}

impl Default for AnsibleDefaults {
    fn default() -> Self {
        Self {
            python_interpreter: default_python_interpreter(),
            ansible_become: true,
            become_method: default_become_method(),
        }
    }
}

fn default_port() -> u16 {
    8006
}

fn default_python_interpreter() -> String {
    "/usr/bin/python3".to_string()
}

fn default_true() -> bool {
    true
}

fn default_become_method() -> String {
    "sudo".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxmox_hosts: Vec::new(),
            manual_hosts: Vec::new(),
            ip_overrides: Vec::new(),
            ansible_defaults: AnsibleDefaults::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path))?;

        let config: Config = serde_yaml::from_str(&contents)
            .context("Failed to parse config file")?;

        Ok(config)
    }
}

