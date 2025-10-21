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

    // Create app
    let mut app = App::new(config, actual_config_path);

    // Fetch initial data (don't fail if this errors - we'll show it in the UI)
    if let Err(e) = app.fetch_all_hosts().await {
        eprintln!("Warning: Failed to fetch initial data: {}", e);
        app.last_error = Some(format!("Failed to fetch data: {}", e));
    }

    // Run the app
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
