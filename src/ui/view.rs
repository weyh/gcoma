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
  a: Add
  r: Remove
  R: Reload";

fn handle_events(cfg_path: &str, state: &mut ViewState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                // Quit
                if key.code == KeyCode::Char('q') {
                    return Ok(true);
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
}

pub fn display(cfg_path: &str) -> io::Result<()> {
    let cfg_str = fs::read_to_string(cfg_path).unwrap_or(String::new());
    let mut config = serde_json::from_str(&cfg_str).unwrap_or(Config::new());

    let mut state = ViewState::new(&mut config);

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

    Ok(())
}
