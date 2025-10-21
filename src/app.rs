use anyhow::Result;
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_input::Input;

use crate::ansible;
use crate::config::{Config, IpOverride};
use crate::proxmox::{Host, HostType, ProxmoxClient};

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Main,
    Export,
    EditIp,
    Setup,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortColumn {
    Name,
    Type,
    Status,
    IpAddress,
    Node,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetupField {
    Name,
    Host,
    Port,
    ApiTokenId,
    ApiTokenSecret,
    VerifySsl,
}

pub struct App {
    pub hosts: Vec<Host>,
    pub selected_index: usize,
    pub view_mode: ViewMode,
    pub is_loading: bool,
    pub last_error: Option<String>,
    pub config: Config,
    pub config_path: String,
    pub should_quit: bool,
    pub export_content: String,
    pub ip_input: Input,
    pub editing_host_name: String,
    pub sort_column: SortColumn,
    pub sort_direction: SortDirection,
    pub initial_fetch_done: bool,
    pub loading_frame: usize,
    // Setup form fields
    pub setup_field: SetupField,
    pub setup_name: Input,
    pub setup_host: Input,
    pub setup_port: Input,
    pub setup_token_id: Input,
    pub setup_token_secret: Input,
    pub setup_verify_ssl: bool,
}

impl App {
    pub fn new(config: Config, config_path: String) -> Self {
        Self {
            hosts: Vec::new(),
            selected_index: 0,
            view_mode: ViewMode::Main,
            is_loading: true, // Start with loading state
            last_error: None,
            config,
            config_path,
            should_quit: false,
            export_content: String::new(),
            ip_input: Input::default(),
            editing_host_name: String::new(),
            sort_column: SortColumn::Name,
            sort_direction: SortDirection::Ascending,
            initial_fetch_done: false,
            loading_frame: 0,
            setup_field: SetupField::Name,
            setup_name: Input::default(),
            setup_host: Input::default(),
            setup_port: Input::default().with_value("8006".to_string()),
            setup_token_id: Input::default(),
            setup_token_secret: Input::default(),
            setup_verify_ssl: false,
        }
    }

    pub fn tick_loading_animation(&mut self) {
        self.loading_frame = self.loading_frame.wrapping_add(1);
    }

    pub async fn fetch_all_hosts(&mut self) -> Result<()> {
        self.is_loading = true;
        self.last_error = None;

        let mut all_hosts = Vec::new();

        // Fetch from each Proxmox host
        for pve_host in &self.config.proxmox_hosts {
            match ProxmoxClient::new(pve_host) {
                Ok(client) => {
                    match client.fetch_all_hosts().await {
                        Ok(hosts) => {
                            all_hosts.extend(hosts);
                        }
                        Err(e) => {
                            self.last_error = Some(format!("Error fetching from {}: {}", pve_host.name, e));
                        }
                    }
                }
                Err(e) => {
                    self.last_error = Some(format!("Error creating client for {}: {}", pve_host.name, e));
                }
            }
        }

        // Apply IP overrides
        for host in &mut all_hosts {
            if let Some(override_entry) = self.config.ip_overrides.iter().find(|o| o.name == host.name) {
                host.ip = Some(override_entry.ip.clone());
            }
        }

        // Add manual hosts
        for manual_host in &self.config.manual_hosts {
            all_hosts.push(Host {
                name: manual_host.name.clone(),
                host_type: HostType::Physical,
                status: "unknown".to_string(),
                ip: Some(manual_host.ip.clone()),
                node: None,
                vmid: None,
                ansible_user: Some(manual_host.ansible_user.clone()),
            });
        }

        self.hosts = all_hosts;
        self.is_loading = false;

        // Apply current sort
        self.apply_sort();

        // Reset selection if out of bounds
        if self.selected_index >= self.hosts.len() && !self.hosts.is_empty() {
            self.selected_index = self.hosts.len() - 1;
        }

        Ok(())
    }

    pub fn export_ansible_format(&mut self) {
        self.export_content = ansible::generate_ansible_hosts(&self.hosts, &self.config.ansible_defaults);
        self.view_mode = ViewMode::Export;
    }

    pub fn next(&mut self) {
        if self.hosts.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.hosts.len();
    }

    pub fn previous(&mut self) {
        if self.hosts.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.hosts.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn page_down(&mut self) {
        if self.hosts.is_empty() {
            return;
        }
        // Jump by 10 items or to the end
        self.selected_index = (self.selected_index + 10).min(self.hosts.len() - 1);
    }

    pub fn page_up(&mut self) {
        if self.hosts.is_empty() {
            return;
        }
        // Jump by 10 items or to the start
        if self.selected_index >= 10 {
            self.selected_index -= 10;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn go_to_top(&mut self) {
        if !self.hosts.is_empty() {
            self.selected_index = 0;
        }
    }

    pub fn go_to_bottom(&mut self) {
        if !self.hosts.is_empty() {
            self.selected_index = self.hosts.len() - 1;
        }
    }

    pub fn copy_to_clipboard(&mut self) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Err(e) = clipboard.set_text(&self.export_content) {
                self.last_error = Some(format!("Failed to copy to clipboard: {}", e));
            } else {
                self.last_error = Some("Copied to clipboard!".to_string());
            }
        } else {
            self.last_error = Some("Failed to access clipboard".to_string());
        }
    }

    pub fn start_edit_ip(&mut self) {
        if self.hosts.is_empty() {
            return;
        }

        let host = &self.hosts[self.selected_index];
        self.editing_host_name = host.name.clone();

        // Pre-fill with existing IP if available
        let current_ip = host.ip.clone().unwrap_or_default();
        self.ip_input = Input::default().with_value(current_ip);

        self.view_mode = ViewMode::EditIp;
    }

    pub fn save_ip_override(&mut self) -> Result<()> {
        let new_ip = self.ip_input.value().trim().to_string();

        if new_ip.is_empty() {
            // Remove the override if IP is empty
            self.config.ip_overrides.retain(|o| o.name != self.editing_host_name);

            // Also remove from the in-memory host list
            if let Some(host) = self.hosts.iter_mut().find(|h| h.name == self.editing_host_name) {
                host.ip = None;
            }
        } else {
            // Add or update the override
            if let Some(existing) = self.config.ip_overrides.iter_mut().find(|o| o.name == self.editing_host_name) {
                existing.ip = new_ip.clone();
            } else {
                self.config.ip_overrides.push(IpOverride {
                    name: self.editing_host_name.clone(),
                    ip: new_ip.clone(),
                });
            }

            // Immediately update the in-memory host list (instant UI update!)
            if let Some(host) = self.hosts.iter_mut().find(|h| h.name == self.editing_host_name) {
                host.ip = Some(new_ip);
            }
        }

        // Save to file
        self.save_config()?;

        self.last_error = Some(format!("IP saved for {}", self.editing_host_name));
        self.view_mode = ViewMode::Main;
        // No need for needs_refresh - we updated in-memory immediately!

        Ok(())
    }

    fn save_config(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(&self.config_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let yaml = serde_yaml::to_string(&self.config)?;
        std::fs::write(&self.config_path, yaml)?;
        Ok(())
    }

    pub fn start_setup(&mut self) {
        // Reset form fields
        self.setup_name = Input::default();
        self.setup_host = Input::default();
        self.setup_port = Input::default().with_value("8006".to_string());
        self.setup_token_id = Input::default();
        self.setup_token_secret = Input::default();
        self.setup_verify_ssl = false;
        self.setup_field = SetupField::Name;
        self.view_mode = ViewMode::Setup;
    }

    pub fn next_setup_field(&mut self) {
        self.setup_field = match self.setup_field {
            SetupField::Name => SetupField::Host,
            SetupField::Host => SetupField::Port,
            SetupField::Port => SetupField::ApiTokenId,
            SetupField::ApiTokenId => SetupField::ApiTokenSecret,
            SetupField::ApiTokenSecret => SetupField::VerifySsl,
            SetupField::VerifySsl => SetupField::Name,
        };
    }

    pub fn prev_setup_field(&mut self) {
        self.setup_field = match self.setup_field {
            SetupField::Name => SetupField::VerifySsl,
            SetupField::Host => SetupField::Name,
            SetupField::Port => SetupField::Host,
            SetupField::ApiTokenId => SetupField::Port,
            SetupField::ApiTokenSecret => SetupField::ApiTokenId,
            SetupField::VerifySsl => SetupField::ApiTokenSecret,
        };
    }

    pub fn save_proxmox_host(&mut self) -> Result<()> {
        use crate::config::ProxmoxHost;

        // Validate fields
        let name = self.setup_name.value().trim();
        let host = self.setup_host.value().trim();
        let port_str = self.setup_port.value().trim();
        let token_id = self.setup_token_id.value().trim();
        let token_secret = self.setup_token_secret.value().trim();

        if name.is_empty() || host.is_empty() || token_id.is_empty() || token_secret.is_empty() {
            self.last_error = Some("All fields except port are required".to_string());
            return Ok(());
        }

        let port: u16 = port_str.parse().unwrap_or(8006);

        // Check for duplicate name
        if self.config.proxmox_hosts.iter().any(|h| h.name == name) {
            self.last_error = Some(format!("Host '{}' already exists", name));
            return Ok(());
        }

        // Add new host
        let new_host = ProxmoxHost {
            name: name.to_string(),
            host: host.to_string(),
            port,
            api_token_id: token_id.to_string(),
            api_token_secret: token_secret.to_string(),
            verify_ssl: self.setup_verify_ssl,
        };

        self.config.proxmox_hosts.push(new_host);
        self.save_config()?;

        self.last_error = Some(format!("Added Proxmox host '{}'", name));
        self.view_mode = ViewMode::Main;

        Ok(())
    }

    pub fn set_sort_column(&mut self, column: SortColumn) {
        if self.sort_column == column {
            // Toggle direction if clicking same column
            self.sort_direction = match self.sort_direction {
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::Ascending,
            };
        } else {
            // New column, default to ascending
            self.sort_column = column;
            self.sort_direction = SortDirection::Ascending;
        }
        self.apply_sort();
    }

    pub fn apply_sort(&mut self) {
        let direction_multiplier = match self.sort_direction {
            SortDirection::Ascending => 1,
            SortDirection::Descending => -1,
        };

        self.hosts.sort_by(|a, b| {
            let cmp = match self.sort_column {
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortColumn::Type => a.host_type.as_str().cmp(&b.host_type.as_str()),
                SortColumn::Status => a.status.cmp(&b.status),
                SortColumn::IpAddress => {
                    // Sort IPs, putting N/A at the end
                    match (&a.ip, &b.ip) {
                        (Some(ip_a), Some(ip_b)) => {
                            // Try to parse as IP addresses for proper sorting
                            match (ip_a.parse::<std::net::IpAddr>(), ip_b.parse::<std::net::IpAddr>()) {
                                (Ok(addr_a), Ok(addr_b)) => addr_a.cmp(&addr_b),
                                _ => ip_a.cmp(ip_b),
                            }
                        }
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                }
                SortColumn::Node => {
                    // Sort nodes, putting "-" at the end
                    match (&a.node, &b.node) {
                        (Some(node_a), Some(node_b)) => node_a.cmp(node_b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                }
            };

            if direction_multiplier == -1 {
                cmp.reverse()
            } else {
                cmp
            }
        });

        // Reset selection to top after sorting
        self.selected_index = 0;
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.view_mode {
            ViewMode::Main => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.next();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.previous();
                    }
                    KeyCode::PageDown => {
                        self.page_down();
                    }
                    KeyCode::PageUp => {
                        self.page_up();
                    }
                    KeyCode::Home | KeyCode::Char('g') => {
                        self.go_to_top();
                    }
                    KeyCode::End | KeyCode::Char('G') => {
                        self.go_to_bottom();
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.export_ansible_format();
                    }
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        self.start_edit_ip();
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        self.start_setup();
                    }
                    KeyCode::Char('1') => {
                        self.set_sort_column(SortColumn::Name);
                    }
                    KeyCode::Char('2') => {
                        self.set_sort_column(SortColumn::Type);
                    }
                    KeyCode::Char('3') => {
                        self.set_sort_column(SortColumn::Status);
                    }
                    KeyCode::Char('4') => {
                        self.set_sort_column(SortColumn::IpAddress);
                    }
                    KeyCode::Char('5') => {
                        self.set_sort_column(SortColumn::Node);
                    }
                    _ => {}
                }
            }
            ViewMode::Export => {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                        self.view_mode = ViewMode::Main;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        self.copy_to_clipboard();
                    }
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.copy_to_clipboard();
                    }
                    _ => {}
                }
            }
            ViewMode::EditIp => {
                match key.code {
                    KeyCode::Enter => {
                        if let Err(e) = self.save_ip_override() {
                            self.last_error = Some(format!("Failed to save: {}", e));
                            self.view_mode = ViewMode::Main;
                        }
                    }
                    KeyCode::Esc => {
                        self.view_mode = ViewMode::Main;
                    }
                    KeyCode::Char(c) => {
                        self.ip_input.handle(tui_input::InputRequest::InsertChar(c));
                    }
                    KeyCode::Backspace => {
                        self.ip_input.handle(tui_input::InputRequest::DeletePrevChar);
                    }
                    KeyCode::Delete => {
                        self.ip_input.handle(tui_input::InputRequest::DeleteNextChar);
                    }
                    KeyCode::Left => {
                        self.ip_input.handle(tui_input::InputRequest::GoToPrevChar);
                    }
                    KeyCode::Right => {
                        self.ip_input.handle(tui_input::InputRequest::GoToNextChar);
                    }
                    KeyCode::Home => {
                        self.ip_input.handle(tui_input::InputRequest::GoToStart);
                    }
                    KeyCode::End => {
                        self.ip_input.handle(tui_input::InputRequest::GoToEnd);
                    }
                    _ => {}
                }
            }
            ViewMode::Setup => {
                match key.code {
                    KeyCode::Esc => {
                        self.view_mode = ViewMode::Main;
                    }
                    KeyCode::Tab => {
                        self.next_setup_field();
                    }
                    KeyCode::BackTab => {
                        self.prev_setup_field();
                    }
                    KeyCode::Enter if self.setup_field == SetupField::VerifySsl => {
                        // Submit form
                        if let Err(e) = self.save_proxmox_host() {
                            self.last_error = Some(format!("Failed to save: {}", e));
                        }
                    }
                    KeyCode::Char(' ') if self.setup_field == SetupField::VerifySsl => {
                        self.setup_verify_ssl = !self.setup_verify_ssl;
                    }
                    KeyCode::Char(c) => {
                        let input = match self.setup_field {
                            SetupField::Name => &mut self.setup_name,
                            SetupField::Host => &mut self.setup_host,
                            SetupField::Port => &mut self.setup_port,
                            SetupField::ApiTokenId => &mut self.setup_token_id,
                            SetupField::ApiTokenSecret => &mut self.setup_token_secret,
                            SetupField::VerifySsl => return, // Skip for checkbox
                        };
                        input.handle(tui_input::InputRequest::InsertChar(c));
                    }
                    KeyCode::Backspace => {
                        let input = match self.setup_field {
                            SetupField::Name => &mut self.setup_name,
                            SetupField::Host => &mut self.setup_host,
                            SetupField::Port => &mut self.setup_port,
                            SetupField::ApiTokenId => &mut self.setup_token_id,
                            SetupField::ApiTokenSecret => &mut self.setup_token_secret,
                            SetupField::VerifySsl => return,
                        };
                        input.handle(tui_input::InputRequest::DeletePrevChar);
                    }
                    KeyCode::Delete => {
                        let input = match self.setup_field {
                            SetupField::Name => &mut self.setup_name,
                            SetupField::Host => &mut self.setup_host,
                            SetupField::Port => &mut self.setup_port,
                            SetupField::ApiTokenId => &mut self.setup_token_id,
                            SetupField::ApiTokenSecret => &mut self.setup_token_secret,
                            SetupField::VerifySsl => return,
                        };
                        input.handle(tui_input::InputRequest::DeleteNextChar);
                    }
                    KeyCode::Left => {
                        let input = match self.setup_field {
                            SetupField::Name => &mut self.setup_name,
                            SetupField::Host => &mut self.setup_host,
                            SetupField::Port => &mut self.setup_port,
                            SetupField::ApiTokenId => &mut self.setup_token_id,
                            SetupField::ApiTokenSecret => &mut self.setup_token_secret,
                            SetupField::VerifySsl => return,
                        };
                        input.handle(tui_input::InputRequest::GoToPrevChar);
                    }
                    KeyCode::Right => {
                        let input = match self.setup_field {
                            SetupField::Name => &mut self.setup_name,
                            SetupField::Host => &mut self.setup_host,
                            SetupField::Port => &mut self.setup_port,
                            SetupField::ApiTokenId => &mut self.setup_token_id,
                            SetupField::ApiTokenSecret => &mut self.setup_token_secret,
                            SetupField::VerifySsl => return,
                        };
                        input.handle(tui_input::InputRequest::GoToNextChar);
                    }
                    _ => {}
                }
            }
        }
    }
}

