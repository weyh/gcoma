use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde_json;
use std::io::{self, stdout};
use std::{fs, vec};

use super::config::Config;
use super::view_state::ViewState;

const HELP_MENU_TEXT: &str = "\
Navigate:
  j/↑: Down
  k/↓: Up
  Enter: Select

Actions:
  q: Quit
  a: Add Session Group
  r: Remove
  R: Reload";

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn create_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn load_cfg_from_file(cfg_path: &str) -> io::Result<Config> {
    let cfg_str = fs::read_to_string(cfg_path)?;
    let config = serde_json::from_str(&cfg_str)?;
    Ok(config)
}

fn remove_selected(state: &mut ViewState) {
    let mut selected_idx = match state.table_state.selected() {
        Some(i) => i,
        None => return,
    };

    for (i, session_group) in state.config.session_groups.iter_mut().enumerate() {
        if selected_idx == 0 {
            state.config.session_groups.remove(i);
            return;
        }

        selected_idx -= 1;

        for (j, _) in session_group.sessions.iter_mut().enumerate() {
            if selected_idx == 0 {
                session_group.sessions.remove(j);
                return;
            }

            selected_idx -= 1;
        }
    }
}

fn handle_events(cfg_path: &str, state: &mut ViewState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                // Quit
                if key.code == KeyCode::Char('q') {
                    if !state.show_popup {
                        return Ok(true);
                    }

                    state.show_popup = false;
                }

                if state.show_popup {
                    // if popup is shown, don't handle any events below
                    return Ok(false);
                }

                // Add Session Group
                if key.code == KeyCode::Char('a') {
                    state.show_popup = true;
                }

                // Remove
                if key.code == KeyCode::Char('r') {
                    remove_selected(state);
                }

                // Reload
                if key.code == KeyCode::Char('R') {
                    state.config = load_cfg_from_file(cfg_path).unwrap_or(Config::new());
                }

                // Vertical scroll
                if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    state.next();
                } else if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    state.previous();
                }
            }
        }
    }

    Ok(false)
}

fn ui(state: &mut ViewState, frame: &mut Frame) {
    let root_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Min(0)],
    )
    .split(frame.size());

    // Title
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(env!("CARGO_CRATE_NAME"))
            .bold()
            .green(),
        root_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        // table                      help
        [Constraint::Percentage(70), Constraint::Percentage(70)],
    )
    .split(root_layout[1]);

    // Menu
    let paragraph = Paragraph::new(HELP_MENU_TEXT)
        .block(Block::default().title("Help").dim().borders(Borders::ALL));

    frame.render_widget(paragraph, inner_layout[1]);

    // Sessions table
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Group Name", "Session Name", "Username", "IP", "Port"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let mut rows = Vec::new();

    for session_group in state.config.session_groups.iter() {
        let sg_cells = vec![
            Cell::from(session_group.name.clone()),
            Cell::from(" "),
            Cell::from(" "),
            Cell::from(" "),
            Cell::from(" "),
        ];
        rows.push(Row::new(sg_cells));

        for session in session_group.sessions.iter() {
            let s_cells = vec![
                Cell::from(" "),
                Cell::from(session.name.clone()),
                Cell::from(session.get_user_name()),
                Cell::from(session.get_ip()),
                Cell::from(session.get_port()),
            ];
            rows.push(Row::new(s_cells));
        }
    }

    let t = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Sessions"))
    .highlight_style(selected_style)
    .highlight_symbol(">> ");

    frame.render_stateful_widget(t, inner_layout[0], &mut state.table_state);

    // Popup (add session group)
    if state.show_popup {
        let block = Block::default()
            .title("Add session group")
            .borders(Borders::ALL);
        let area = create_centered_rect(50, 50, frame.size());
        frame.render_widget(Clear, area); // this clears out the background
        frame.render_widget(block, area);
    }
}

pub fn display(cfg_path: &str) -> io::Result<()> {
    let mut state = ViewState::new(load_cfg_from_file(cfg_path).unwrap_or(Config::new()));

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(&mut state, frame))?;
        should_quit = handle_events(cfg_path, &mut state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    state.config.save(cfg_path);
    Ok(())
}
