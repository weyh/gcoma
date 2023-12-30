use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde_json;
use std::fmt;
use std::fs;
use std::io::{self, stdout};

use super::config::Config;

struct MenuAction {
    pub name: &'static str,
    pub action_key: char,
    pub action: fn(&mut Config),
}

const menu_actions: [&MenuAction; 4] = [
    &MenuAction {
        name: "Quit",
        action_key: 'q',
        action: |config: &mut Config| {},
    },
    &MenuAction {
        name: "Add",
        action_key: 'a',
        action: |config: &mut Config| panic!("Not implemented!"),
    },
    &MenuAction {
        name: "Remove",
        action_key: 'r',
        action: |config: &mut Config| panic!("Not implemented!"),
    },
    &MenuAction {
        name: "Reload",
        action_key: 'R',
        action: |config: &mut Config| panic!("Not implemented!"),
    },
];

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(config: &mut Config, vertical_scroll: &mut u16, frame: &mut Frame) {
    let root_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Min(0)],
    )
    .split(frame.size());

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(env!("CARGO_CRATE_NAME"))
            .bold(),
        root_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .split(root_layout[1]);

    let right_inner_layout = Layout::new(
        Direction::Vertical,
        menu_actions.map(|_| Constraint::Length(1)),
    )
    .split(inner_layout[0]);

    for (i, menu_action) in menu_actions.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(format!(
                "{0}: {1}",
                menu_action.action_key, menu_action.name
            )),
            right_inner_layout[i],
        );
    }

    let items = vec![
        Line::from("Item 1"),
        Line::from("Item 2"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
        Line::from("Item 3"),
    ];
    let paragraph = Paragraph::new(items.clone())
        .scroll((*vertical_scroll, 0))
        .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state =
        ScrollbarState::new(items.iter().len()).position(*vertical_scroll as usize);

    frame.render_widget(paragraph, inner_layout[1]);
    frame.render_stateful_widget(
        scrollbar,
        inner_layout[1].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }), // using a inner vertical margin of 1 unit makes the scrollbar inside the block
        &mut scrollbar_state,
    );
}

pub fn display(cfg_path: &str) -> io::Result<()> {
    let cfg_str = fs::read_to_string(cfg_path).unwrap_or(String::new());
    let mut config = serde_json::from_str(&cfg_str).unwrap_or(Config::new());

    let mut vertical_scroll: u16 = 0;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(&mut config, &mut vertical_scroll, frame))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
