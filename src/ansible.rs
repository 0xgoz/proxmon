use crate::config::AnsibleDefaults;
use crate::proxmox::{Host, HostType};
use std::collections::HashMap;

pub fn generate_ansible_hosts(hosts: &[Host], defaults: &AnsibleDefaults) -> String {
    let mut output = String::new();

    // Group hosts by type
    let mut grouped: HashMap<&str, Vec<&Host>> = HashMap::new();

    for host in hosts {
        let group = match host.host_type {
            HostType::VM => "Proxmox_VM",
            HostType::LXC => "Proxmox_LXC",
            HostType::Physical => "Physical",
        };
        grouped.entry(group).or_insert_with(Vec::new).push(host);
    }

    // Generate output for each group
    for (group_name, group_hosts) in &grouped {
        if group_hosts.is_empty() {
            continue;
        }

        output.push_str(&format!("[{}]\n", group_name));

        for host in group_hosts {
            let mut line = host.name.clone();

            // Add ansible_host if IP is available
            if let Some(ip) = &host.ip {
                line.push_str(&format!(" ansible_host={}", ip));
            }

            // Add ansible_user
            if let Some(user) = &host.ansible_user {
                line.push_str(&format!(" ansible_user={}", user));
            }

            // For Proxmox hosts, don't add ansible_become=false by default
            // They'll inherit from [all:vars]

            output.push_str(&line);
            output.push('\n');
        }

        output.push('\n');
    }

    // Add [all:vars] section
    output.push_str("[all:vars]\n");
    output.push_str(&format!("ansible_python_interpreter={}\n", defaults.python_interpreter));
    output.push_str(&format!("ansible_become={}\n", if defaults.ansible_become { "yes" } else { "no" }));
    output.push_str(&format!("ansible_become_method={}\n", defaults.become_method));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ansible_hosts() {
        let hosts = vec![
            Host {
                name: "test-vm".to_string(),
                host_type: HostType::VM,
                status: "running".to_string(),
                ip: Some("10.1.2.10".to_string()),
                node: Some("pve1".to_string()),
                vmid: Some(100),
                ansible_user: Some("gozy".to_string()),
            },
            Host {
                name: "test-lxc".to_string(),
                host_type: HostType::LXC,
                status: "running".to_string(),
                ip: Some("10.1.2.20".to_string()),
                node: Some("pve1".to_string()),
                vmid: Some(101),
                ansible_user: Some("gozy".to_string()),
            },
        ];

        let defaults = AnsibleDefaults::default();
        let output = generate_ansible_hosts(&hosts, &defaults);

        assert!(output.contains("[Proxmox_VM]"));
        assert!(output.contains("[Proxmox_LXC]"));
        assert!(output.contains("test-vm ansible_host=10.1.2.10 ansible_user=gozy"));
        assert!(output.contains("test-lxc ansible_host=10.1.2.20 ansible_user=gozy"));
        assert!(output.contains("[all:vars]"));
    }
}

