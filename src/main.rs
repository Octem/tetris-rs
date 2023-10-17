// Keybinds:
// rotate right: k
// rotate left: j
// right: l
// left: h
// hard drop: space
// quit: q or esc

pub mod board;
pub mod block;
pub mod game;
pub mod input;
pub mod colors;

use game::Game;
use input::Input;

use crossterm::{cursor, execute};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::{time::Duration, io::stdout};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    execute!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0)).unwrap();
    let mut game = Game::new();

    execute!(stdout, cursor::Hide);

    game.init(&stdout);

    loop {
        game.update(&stdout);
        game.handle_falling(&stdout);

        if poll(Duration::from_millis(500)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    // q to exit
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        break;
                    }
                    // left
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        game.handle_input(Input::Left, &stdout);
                    }
                    // right
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('l'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        game.handle_input(Input::Right, &stdout);
                    }
                    // rotate left
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('j'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        game.handle_input(Input::Counterclockwise, &stdout);
                    }
                    // rotate right
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('k'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        game.handle_input(Input::Clockwise, &stdout);
                    }
                    _ => (),
                }
            }

        }
    }

    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}