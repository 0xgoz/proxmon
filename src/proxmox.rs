use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

use crate::config::ProxmoxHost;

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub host_type: HostType,
    pub status: String,
    pub ip: Option<String>,
    pub node: Option<String>,
    #[allow(dead_code)] // Kept for future features (start/stop VMs)
    pub vmid: Option<u32>,
    pub ansible_user: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HostType {
    VM,
    LXC,
    Physical,
}

impl HostType {
    pub fn as_str(&self) -> &str {
        match self {
            HostType::VM => "VM",
            HostType::LXC => "LXC",
            HostType::Physical => "Physical",
        }
    }
}

#[derive(Debug, Deserialize)]
struct ProxmoxResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct Node {
    node: String,
}

#[derive(Debug, Deserialize)]
struct VmInfo {
    vmid: u32,
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct NetworkInterface {
    #[serde(rename = "ip-address")]
    ip_address: Option<String>,
}

pub struct ProxmoxClient {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl ProxmoxClient {
    pub fn new(config: &ProxmoxHost) -> Result<Self> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(!config.verify_ssl)
            .build()
            .context("Failed to create HTTP client")?;

        let base_url = format!("https://{}:{}/api2/json", config.host, config.port);
        let token = format!("{}={}", config.api_token_id, config.api_token_secret);

        Ok(Self {
            client,
            base_url,
            token,
        })
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("PVEAPIToken={}", self.token))
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, text);
        }

        let data: ProxmoxResponse<T> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(data.data)
    }

    pub async fn list_nodes(&self) -> Result<Vec<String>> {
        let nodes: Vec<Node> = self.get("/nodes").await?;
        Ok(nodes.into_iter().map(|n| n.node).collect())
    }

    pub async fn list_vms(&self, node: &str) -> Result<Vec<Host>> {
        let vms: Vec<VmInfo> = self
            .get(&format!("/nodes/{}/qemu", node))
            .await
            .unwrap_or_default();

        let mut hosts = Vec::new();
        for vm in vms {
            // Try to get IP address from the VM's network configuration
            let ip = self.get_vm_ip(node, vm.vmid).await;

            hosts.push(Host {
                name: vm.name,
                host_type: HostType::VM,
                status: vm.status,
                ip,
                node: Some(node.to_string()),
                vmid: Some(vm.vmid),
                ansible_user: Some("gozy".to_string()), // Default user
            });
        }

        Ok(hosts)
    }

    pub async fn list_lxc(&self, node: &str) -> Result<Vec<Host>> {
        let containers: Vec<VmInfo> = self
            .get(&format!("/nodes/{}/lxc", node))
            .await
            .unwrap_or_default();

        let mut hosts = Vec::new();
        for container in containers {
            // Try to get IP address from the container's network configuration
            let ip = self.get_lxc_ip(node, container.vmid).await;

            hosts.push(Host {
                name: container.name,
                host_type: HostType::LXC,
                status: container.status,
                ip,
                node: Some(node.to_string()),
                vmid: Some(container.vmid),
                ansible_user: Some("gozy".to_string()), // Default user
            });
        }

        Ok(hosts)
    }

    async fn get_vm_ip(&self, node: &str, vmid: u32) -> Option<String> {
        // Try to get IP from agent network interfaces
        let interfaces: Result<Vec<NetworkInterface>> = self
            .get(&format!("/nodes/{}/qemu/{}/agent/network-get-interfaces", node, vmid))
            .await;

        if let Ok(interfaces) = interfaces {
            for interface in interfaces {
                if let Some(ip) = interface.ip_address {
                    // Filter out loopback and IPv6 addresses
                    if !ip.starts_with("127.") && !ip.contains(':') {
                        return Some(ip);
                    }
                }
            }
        }

        // Fallback: Try to get IP from the configuration
        let config: Result<HashMap<String, serde_json::Value>> = self
            .get(&format!("/nodes/{}/qemu/{}/config", node, vmid))
            .await;

        if let Ok(config) = config {
            // Look for net0, net1, etc. and try to parse IP from the configuration
            // This is a simplified approach and might need refinement based on actual config
            for (key, value) in config {
                if key.starts_with("net") {
                    if let Some(net_str) = value.as_str() {
                        // Parse network configuration string (format varies)
                        // This is a basic implementation
                        if let Some(ip_start) = net_str.find("ip=") {
                            let ip_part = &net_str[ip_start + 3..];
                            if let Some(ip_end) = ip_part.find(&[',', ' '][..]) {
                                return Some(ip_part[..ip_end].split('/').next()?.to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }

    async fn get_lxc_ip(&self, node: &str, vmid: u32) -> Option<String> {
        // Try to get IP from network interfaces
        let interfaces: Result<Vec<HashMap<String, serde_json::Value>>> = self
            .get(&format!("/nodes/{}/lxc/{}/interfaces", node, vmid))
            .await;

        if let Ok(interfaces) = interfaces {
            for interface in interfaces {
                if let Some(inet) = interface.get("inet") {
                    if let Some(ip) = inet.as_str() {
                        // Remove CIDR notation if present
                        let ip = ip.split('/').next().unwrap_or(ip);
                        if !ip.starts_with("127.") && !ip.is_empty() {
                            return Some(ip.to_string());
                        }
                    }
                }
            }
        }

        // Fallback: Try to get IP from the configuration
        let config: Result<HashMap<String, serde_json::Value>> = self
            .get(&format!("/nodes/{}/lxc/{}/config", node, vmid))
            .await;

        if let Ok(config) = config {
            // Look for net0, net1, etc.
            for (key, value) in config {
                if key.starts_with("net") {
                    if let Some(net_str) = value.as_str() {
                        if let Some(ip_start) = net_str.find("ip=") {
                            let ip_part = &net_str[ip_start + 3..];
                            if let Some(ip_end) = ip_part.find(&[',', ' '][..]) {
                                return Some(ip_part[..ip_end].split('/').next()?.to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub async fn fetch_all_hosts(&self) -> Result<Vec<Host>> {
        let nodes = self.list_nodes().await?;
        let mut all_hosts = Vec::new();

        for node in nodes {
            // Fetch VMs
            if let Ok(vms) = self.list_vms(&node).await {
                all_hosts.extend(vms);
            }

            // Fetch LXC containers
            if let Ok(lxc) = self.list_lxc(&node).await {
                all_hosts.extend(lxc);
            }
        }

        Ok(all_hosts)
    }
}

