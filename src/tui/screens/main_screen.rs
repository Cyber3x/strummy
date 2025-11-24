use std::rc::Rc;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};
use serde::de::value::UsizeDeserializer;

use crate::{
    core::strumming_pattern::{Stroke, StrummingPattern},
    tui::{
        app_state::AppState,
        screen::{Screen, ScreenCommand},
    },
};

//const INFO_TEXT: [&str; 2] = ["(Esc\\q) quit | (j) move down | (k) move up", ""];

#[derive(Debug, Default)]
pub struct MainScreen {
    current_stroke: Option<u8>,
}

impl Screen for MainScreen {
    fn draw(&mut self, frame: &mut ratatui::Frame, state: &AppState) {
        self.render(frame, state);
    }

    fn handle_event(&mut self, key_event: KeyEvent, state: &mut AppState) -> Option<ScreenCommand> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => Some(ScreenCommand::Quit(true)),
            KeyEvent {
                code: KeyCode::Char('n'),
                ..
            } => {
                new_random_pattern(state);
                None
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                if let None = self.current_stroke {
                    self.current_stroke = Some(0)
                }
                None
            }
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => {
                if let Some(i) = self.current_stroke {
                    self.current_stroke = Some((i + 1) % state.strumming_pattern.len() as u8);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => {
                if let Some(mut i) = self.current_stroke {
                    if i == 0 {
                        i = state.strumming_pattern.len() as u8 - 1;
                    } else {
                        i -= 1
                    }
                    self.current_stroke = Some(i);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Esc, ..
            } => {
                self.current_stroke = None;
                None
            }
            KeyEvent {
                code: KeyCode::Up, ..
            } => {
                if let Some(i) = self.current_stroke {
                    state.strumming_pattern.set_stroke(i as usize, Stroke::Up);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => {
                if let Some(i) = self.current_stroke {
                    state.strumming_pattern.set_stroke(i as usize, Stroke::Down);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Char('m'),
                ..
            } => {
                if let Some(i) = self.current_stroke {
                    state.strumming_pattern.set_stroke(i as usize, Stroke::Miss);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Char('x'),
                ..
            } => {
                if let Some(i) = self.current_stroke {
                    state.strumming_pattern.set_stroke(i as usize, Stroke::Mute);
                }
                None
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                state
                    .strumming_pattern
                    .save_to_file("./patterns/test.json")
                    .unwrap();
                None
            }
            KeyEvent {
                code: KeyCode::Char('o'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                state.strumming_pattern =
                    StrummingPattern::load_from_file("./patterns/test.json").unwrap();
                None
            }
            _ => None,
        }
    }
}

fn new_random_pattern(app_state: &mut AppState) {
    app_state.strumming_pattern = StrummingPattern::new_random(app_state.strumming_pattern.len());
}

impl MainScreen {
    pub fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, frame: &mut Frame, app_state: &AppState) {
        let area = frame.area();

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta))
            .title("Strumming Practice Tool").bold();

        frame.render_widget(&outer_block, area);

        let inner_area = outer_block.inner(area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ])
            .split(inner_area);

        render_strumming_pattern(app_state, frame, layout, self.current_stroke);
    }
}

fn render_strumming_pattern(
    app_state: &AppState,
    frame: &mut Frame,
    layout: Rc<[Rect]>,
    current_stroke: Option<u8>,
) {
    let style_for = |i: usize| {
        if Some(i as u8) == current_stroke {
            Style::default().fg(Color::Red).bold()
        } else {
            Style::default().fg(Color::White)
        }
    };

    let numbers_spans: Vec<Span> = app_state
        .strumming_pattern
        .strokes
        .iter()
        .enumerate()
        .flat_map(|(i, _)| {
            let number = if i % 2 == 0 {
                ((i as f64 / 2.0).ceil() + 1.0).to_string()
            } else {
                "+".to_owned()
            };
            vec![Span::styled(number, style_for(i)), Span::raw(" ")]
        })
        .collect();

    let strokes_spans: Vec<Span> = app_state
        .strumming_pattern
        .strokes
        .iter()
        .enumerate()
        .flat_map(|(i, s)| vec![Span::styled(s.shorthand(), style_for(i)), Span::raw(" ")])
        .collect();

    let strokes_line = Line::from(strokes_spans);
    let numbers_line = Line::from(numbers_spans);

    let strokes_paragraph = Paragraph::new(strokes_line).alignment(Alignment::Center);
    let numbers_paragraph = Paragraph::new(numbers_line).alignment(Alignment::Center);

    frame.render_widget(numbers_paragraph, layout[1]);
    frame.render_widget(strokes_paragraph, layout[3]);
}
