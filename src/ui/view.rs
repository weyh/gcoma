use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use serde_json;
use std::io::{self, stdout};
use std::{fs, vec};
use tui_textarea::{Input, Key};

use super::view_state::ViewState;
use super::{config::Config, view_state::PopupBuilderState};

const HELP_MENU_TEXT: &str = "\
Navigate:
  j/↑: down
  k/↓: up
  Enter: select

Actions:
  q/ESC: quit
      a: add Session Group
      r: remove
      R: Reload config";

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

fn handle_normal_mode_events(state: &mut ViewState, cfg_path: &str) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    // Quit
                    return Ok(true);
                }
                KeyCode::Char('a') => state.popup_state.show(),
                KeyCode::Char('r') => remove_selected(state),
                KeyCode::Char('R') => {
                    state.config = load_cfg_from_file(cfg_path).unwrap_or(Config::new());
                }
                KeyCode::Enter => panic!("Not implemented yet!"),
                KeyCode::Down | KeyCode::Char('j') => state.next(),
                KeyCode::Up | KeyCode::Char('k') => state.previous(),
                _ => {}
            }
        }
    }

    Ok(false)
}

fn handle_edit_mode_events(state: &mut ViewState) -> io::Result<bool> {
    let popup_state = &mut state.popup_state;

    // this is horrible, the state machine needs to be reworked or something :(
    match event::read()?.into() {
        Input { key: Key::Esc, .. } => {
            // Close popup
            popup_state.reset_state();
        }
        Input {
            // Submit
            key: Key::Enter,
            ..
        } => match popup_state.get_state() {
            PopupBuilderState::SessionGroupConfirm
            | PopupBuilderState::Done
            | PopupBuilderState::SessionAddConfirm
            | PopupBuilderState::SessionAddMore => {}
            _ => {
                let line = popup_state.textarea.lines()[0].clone();
                if line.is_empty() {
                    return Ok(false);
                }

                popup_state.push_data(&line);
                popup_state.increment_prompt();

                if popup_state.get_state() == PopupBuilderState::Done {
                    state.add_temp_session_group_to_cfg();
                }
            }
        },
        input => match popup_state.get_state() {
            PopupBuilderState::SessionGroupConfirm | PopupBuilderState::Done => {
                if input.key == Key::Char('y') {
                    let tmp = String::new();

                    popup_state.push_data(&tmp);
                    state.add_temp_session_group_to_cfg();
                }

                // Close popup
                state.popup_state.reset_state();
            }
            PopupBuilderState::SessionAddConfirm => {
                if input.key == Key::Char('y') {
                    let tmp = String::new();
                    popup_state.push_data(&tmp);
                    popup_state.increment_prompt();
                } else if input.key == Key::Char('n') {
                    popup_state.increment_prompt();
                }
            }
            PopupBuilderState::SessionAddMore => {
                if input.key == Key::Char('y') {
                    let tmp = String::new();
                    popup_state.push_data(&tmp);
                } else if input.key == Key::Char('n') {
                    popup_state.increment_prompt();
                }
            }
            _ => {
                popup_state.textarea.input(input);
            }
        },
    }

    Ok(false)
}

fn handle_events(cfg_path: &str, state: &mut ViewState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if !state.popup_state.is_open() {
            return handle_normal_mode_events(state, cfg_path);
        } else {
            return handle_edit_mode_events(state);
        }
    }

    Ok(false)
}

fn table_ui(state: &mut ViewState, frame: &mut Frame, area: &Rect) {
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

    frame.render_stateful_widget(t, *area, &mut state.table_state);
}

fn popup_ui(state: &mut ViewState, frame: &mut Frame) {
    let create_block = |title, border, modifier, color| {
        Block::default().borders(border).gray().title(Span::styled(
            title,
            Style::default().add_modifier(modifier).fg(color),
        ))
    };

    let area = create_centered_rect(50, 50, frame.size());
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(100), Constraint::Percentage(100)],
    )
    .split(area);

    frame.render_widget(Clear, area); // clears out the background
    frame.render_widget(
        create_block(
            "Create new session group",
            Borders::ALL,
            Modifier::BOLD,
            Color::Reset,
        ),
        area,
    );

    let (prompt, placeholder) = match state.popup_state.get_prompt() {
        Some((prompt, placeholder)) => (prompt, placeholder),
        None => panic!("Unimplemented!"),
    };

    // conditional rendering
    // textbox or paragraph
    match state.popup_state.get_state() {
        PopupBuilderState::SessionGroupName
        | PopupBuilderState::SessionName
        | PopupBuilderState::SessionConnectionType
        | PopupBuilderState::SessionData => {
            let textarea = &mut state.popup_state.textarea;

            textarea.set_block(create_block(
                prompt,
                Borders::NONE,
                Modifier::BOLD,
                Color::LightBlue,
            ));
            textarea.set_placeholder_text(placeholder);
            frame.render_widget(
                textarea.widget(),
                chunks[0].inner(&Margin {
                    vertical: 1,
                    horizontal: 2,
                }),
            );
        }
        PopupBuilderState::SessionAddConfirm
        | PopupBuilderState::SessionAddMore
        | PopupBuilderState::SessionGroupConfirm
        | PopupBuilderState::Done => {
            let line = Line::from(prompt);
            let paragraph = Paragraph::new(line).bold();
            frame.render_widget(
                paragraph,
                chunks[0].inner(&Margin {
                    vertical: 1,
                    horizontal: 2,
                }),
            );
        }
    }
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
            .title_alignment(Alignment::Center)
            .bold()
            .green(),
        root_layout[0],
    );

    // --------- --------
    // | table | | help |
    // --------- --------
    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(70), Constraint::Percentage(70)],
    )
    .split(root_layout[1]);

    // Menu
    let paragraph = Paragraph::new(HELP_MENU_TEXT)
        .block(Block::default().title("Help").dim().borders(Borders::ALL));

    frame.render_widget(paragraph, inner_layout[1]);

    // Sessions table
    table_ui(state, frame, &inner_layout[0]);

    // Popup (add session group)
    if state.popup_state.is_open() {
        popup_ui(state, frame)
    }
}

pub fn display(cfg_path: &str) -> io::Result<()> {
    let mut state = ViewState::new(load_cfg_from_file(cfg_path).unwrap_or(Config::new()));

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    state
        .popup_state
        .textarea
        .set_cursor_line_style(Style::default());

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(&mut state, frame))?;
        should_quit = handle_events(cfg_path, &mut state)?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    state.config.save(cfg_path);
    Ok(())
}
