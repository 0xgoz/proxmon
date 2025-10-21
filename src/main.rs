mod ansible;
mod app;
mod config;
mod proxmox;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use std::time::Duration;

use app::App;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Determine config file location
    let config_path = std::env::var("PROXMON_CONFIG")
        .unwrap_or_else(|_| {
            // Try ~/.config/proxmon/config.yml first (Unix-like systems)
            if let Some(home_dir) = dirs::home_dir() {
                let app_config = home_dir.join(".config").join("proxmon").join("config.yml");
                if app_config.exists() {
                    return app_config.to_string_lossy().to_string();
                }
            }
            // Fall back to current directory
            "config.yml".to_string()
        });

    // Load configuration, create empty one if it doesn't exist
    let (config, actual_config_path) = match Config::load(&config_path) {
        Ok(cfg) => (cfg, config_path),
        Err(_) => {
            // Config doesn't exist, create it with defaults
            eprintln!("No config found at {}", config_path);

            // Determine the default config path
            let default_path = if let Some(home_dir) = dirs::home_dir() {
                home_dir.join(".config").join("proxmon").join("config.yml")
            } else {
                std::path::PathBuf::from("config.yml")
            };

            // Create the directory structure if it doesn't exist
            if let Some(parent) = default_path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Error: Failed to create config directory: {}", e);
                    std::process::exit(1);
                }
            }

            // Create an empty config with defaults
            let empty_config = Config::default();
            let config_yaml = serde_yaml::to_string(&empty_config)
                .expect("Failed to serialize default config");

            if let Err(e) = std::fs::write(&default_path, config_yaml) {
                eprintln!("Error: Failed to create config file: {}", e);
                std::process::exit(1);
            }

            eprintln!("✨ Created new config at: {}", default_path.display());
            eprintln!("📝 Tip: Press 'a' in the app to add your first Proxmox host!\n");

            let path_str = default_path.to_string_lossy().to_string();
            (empty_config, path_str)
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app (starts in loading state)
    let mut app = App::new(config, actual_config_path);

    // Run the app (initial data fetch happens inside the loop)
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    // Perform initial fetch with animated loading screen
    if !app.initial_fetch_done {
        // Spawn the fetch task
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let config = app.config.clone();

        tokio::spawn(async move {
            let mut all_hosts = Vec::new();

            // Fetch from each Proxmox host
            for pve_host in &config.proxmox_hosts {
                match crate::proxmox::ProxmoxClient::new(pve_host) {
                    Ok(client) => {
                        match client.fetch_all_hosts().await {
                            Ok(hosts) => {
                                all_hosts.extend(hosts);
                            }
                            Err(e) => {
                                let _ = tx.send(Err(format!("Error fetching from {}: {}", pve_host.name, e))).await;
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error creating client for {}: {}", pve_host.name, e))).await;
                        return;
                    }
                }
            }

            let _ = tx.send(Ok(all_hosts)).await;
        });

        // Keep drawing loading screen until fetch completes
        loop {
            app.tick_loading_animation();
            terminal.draw(|f| ui::render(f, app))?;

            // Check if fetch completed
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(mut hosts) => {
                        // Apply IP overrides
                        for host in &mut hosts {
                            if let Some(override_entry) = app.config.ip_overrides.iter().find(|o| o.name == host.name) {
                                host.ip = Some(override_entry.ip.clone());
                            }
                        }

                        // Add manual hosts
                        for manual_host in &app.config.manual_hosts {
                            hosts.push(crate::proxmox::Host {
                                name: manual_host.name.clone(),
                                host_type: crate::proxmox::HostType::Physical,
                                status: "unknown".to_string(),
                                ip: Some(manual_host.ip.clone()),
                                node: None,
                                vmid: None,
                                ansible_user: Some(manual_host.ansible_user.clone()),
                            });
                        }

                        app.hosts = hosts;
                        app.apply_sort();

                        if app.selected_index >= app.hosts.len() && !app.hosts.is_empty() {
                            app.selected_index = app.hosts.len() - 1;
                        }
                    }
                    Err(e) => {
                        app.last_error = Some(e);
                    }
                }

                app.is_loading = false;
                app.initial_fetch_done = true;
                break;
            }

            // Handle quit during loading
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == crossterm::event::KeyCode::Char('q')
                        || key.code == crossterm::event::KeyCode::Char('Q')
                        || (key.code == crossterm::event::KeyCode::Char('c')
                            && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)) {
                        app.should_quit = true;
                        return Ok(());
                    }
                }
            }
        }
    }

    loop {
        terminal.draw(|f| ui::render(f, app))?;

        // Handle events with a timeout
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key);

                // Check if we need to refresh data
                if key.code == crossterm::event::KeyCode::Char('r')
                    || key.code == crossterm::event::KeyCode::Char('R') {
                    app.fetch_all_hosts().await?;
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
