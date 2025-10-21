use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
    Frame,
};

use crate::app::{App, ViewMode, SortColumn, SortDirection, SetupField};

pub fn render(f: &mut Frame, app: &mut App) {
    match app.view_mode {
        ViewMode::Main => render_main_view(f, app),
        ViewMode::Export => render_export_view(f, app),
        ViewMode::EditIp => render_edit_ip_view(f, app),
        ViewMode::Setup => render_setup_view(f, app),
    }
}

fn render_main_view(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status/keybindings
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Proxmon - Gotta manage 'em all!")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content - table of hosts with sort indicators
    let sort_indicator = match app.sort_direction {
        SortDirection::Ascending => "↑",
        SortDirection::Descending => "↓",
    };

    let headers = [
        ("Name", SortColumn::Name),
        ("Type", SortColumn::Type),
        ("Status", SortColumn::Status),
        ("IP Address", SortColumn::IpAddress),
        ("Node", SortColumn::Node),
    ];

    let header_cells = headers.iter().map(|(label, col)| {
        let text = if *col == app.sort_column {
            format!("{} {}", label, sort_indicator)
        } else {
            label.to_string()
        };
        Cell::from(text).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    });

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.hosts.iter().map(|host| {
        let cells = vec![
            Cell::from(host.name.clone()),
            Cell::from(host.host_type.as_str()),
            Cell::from(host.status.clone()).style(
                if host.status == "running" {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                }
            ),
            Cell::from(host.ip.clone().unwrap_or_else(|| "N/A".to_string())),
            Cell::from(host.node.clone().unwrap_or_else(|| "-".to_string())),
        ];
        Row::new(cells).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Hosts"))
    .row_highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    let mut table_state = TableState::default();
    table_state.select(Some(app.selected_index));

    f.render_stateful_widget(table, chunks[1], &mut table_state);

    // Status bar and keybindings
    let status_text = if app.is_loading {
        "Loading..."
    } else if let Some(ref error) = app.last_error {
        error.as_str()
    } else {
        "Ready"
    };

    let status_style = if app.last_error.is_some() {
        Style::default().fg(Color::Red)
    } else if app.is_loading {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Green)
    };

    let bottom_text = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(status_text, status_style),
        ]),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit | "),
            Span::styled("↑/↓ PgUp/PgDn g/G", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Nav | "),
            Span::styled("1-5", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Sort | "),
            Span::styled("a", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Add Host | "),
            Span::styled("i", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Edit IP | "),
            Span::styled("e", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Export | "),
            Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Refresh"),
        ]),
    ];

    let bottom = Paragraph::new(bottom_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    f.render_widget(bottom, chunks[2]);
}

fn render_export_view(f: &mut Frame, app: &App) {
    // Create a centered popup
    let area = centered_rect(80, 80, f.area());

    // Clear the background
    let block = Block::default()
        .title("Ansible Hosts Export")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Split the inner area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // Content
            Constraint::Length(2),  // Instructions
        ])
        .split(inner_area);

    // Export content
    let content = Paragraph::new(app.export_content.clone())
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(content, chunks[0]);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("c/y", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(": Copy to clipboard | ", Style::default()),
            Span::styled("Enter/Esc/q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(": Close", Style::default()),
        ]),
    ]);

    f.render_widget(instructions, chunks[1]);
}

fn render_edit_ip_view(f: &mut Frame, app: &App) {
    // Create a compact centered popup (fixed height for better UX)
    let area = centered_rect_fixed(60, 9, f.area());

    // Clear the background
    let block = Block::default()
        .title(format!(" Edit IP: {} ", app.editing_host_name))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Split the inner area - more compact layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Label (single line)
            Constraint::Length(3),  // Input field
            Constraint::Length(1),  // Instructions
        ])
        .split(inner_area);

    // Compact label
    let label = Paragraph::new("IP address (empty to remove):")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(label, chunks[0]);

    // Input field
    let input_text = app.ip_input.value();
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow)));
    f.render_widget(input, chunks[1]);

    // Show cursor position
    let cursor_pos = app.ip_input.cursor();
    f.set_cursor_position((
        chunks[1].x + cursor_pos as u16 + 1,
        chunks[1].y + 1,
    ));

    // Compact instructions
    let instructions = Paragraph::new(
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(": Save | ", Style::default()),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(": Cancel", Style::default()),
        ])
    );
    f.render_widget(instructions, chunks[2]);
}

fn render_setup_view(f: &mut Frame, app: &App) {
    // Create a large centered popup for the form
    let area = centered_rect_fixed(80, 25, f.area());

    // Main block
    let block = Block::default()
        .title(" Add Proxmox Host ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Split the inner area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Title/instructions
            Constraint::Length(3),  // Name field
            Constraint::Length(3),  // Host field
            Constraint::Length(3),  // Port field
            Constraint::Length(3),  // Token ID field
            Constraint::Length(3),  // Token Secret field
            Constraint::Length(3),  // Verify SSL checkbox
            Constraint::Length(1),  // Spacer
            Constraint::Length(4),  // Instructions
        ])
        .split(inner_area);

    // Title/instructions
    let intro = Paragraph::new("Fill in the details for your Proxmox host. Get API tokens from Datacenter → Permissions → API Tokens")
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: false });
    f.render_widget(intro, chunks[0]);

    // Helper to render a field
    let mut render_field = |chunk_idx: usize, label: &str, input: &str, field: SetupField, cursor_pos: usize| {
        let is_active = app.setup_field == field;
        let style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let field_widget = Paragraph::new(input)
            .style(if is_active { Style::default().fg(Color::White) } else { Style::default().fg(Color::DarkGray) })
            .block(Block::default().borders(Borders::ALL).title(label).style(style));

        f.render_widget(field_widget, chunks[chunk_idx]);

        // Show cursor if active
        if is_active && field != SetupField::VerifySsl {
            f.set_cursor_position((
                chunks[chunk_idx].x + cursor_pos as u16 + 1,
                chunks[chunk_idx].y + 1,
            ));
        }
    };

    // Render fields
    render_field(1, "Name (e.g., vs01)", app.setup_name.value(), SetupField::Name, app.setup_name.cursor());
    render_field(2, "Host/IP (e.g., 10.1.2.1)", app.setup_host.value(), SetupField::Host, app.setup_host.cursor());
    render_field(3, "Port (default: 8006)", app.setup_port.value(), SetupField::Port, app.setup_port.cursor());
    render_field(4, "API Token ID (e.g., root@pam!mytoken)", app.setup_token_id.value(), SetupField::ApiTokenId, app.setup_token_id.cursor());
    render_field(5, "API Token Secret", app.setup_token_secret.value(), SetupField::ApiTokenSecret, app.setup_token_secret.cursor());

    // Verify SSL checkbox
    let is_ssl_active = app.setup_field == SetupField::VerifySsl;
    let checkbox_text = if app.setup_verify_ssl { "[X]" } else { "[ ]" };
    let checkbox = Paragraph::new(format!("{} Verify SSL (usually off for self-signed certs)", checkbox_text))
        .style(if is_ssl_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .block(Block::default().borders(Borders::ALL).style(
            if is_ssl_active { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::DarkGray) }
        ));
    f.render_widget(checkbox, chunks[6]);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Tab/Shift+Tab", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(": Navigate fields | ", Style::default()),
            Span::styled("Space", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(": Toggle SSL checkbox", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" on last field: Save | ", Style::default()),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(": Cancel", Style::default()),
        ]),
    ]);
    f.render_widget(instructions, chunks[8]);
}

/// Helper function to create a centered rectangle with percentage sizing
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Helper function to create a centered rectangle with fixed height (better for small popups)
fn centered_rect_fixed(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

