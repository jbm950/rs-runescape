use ratatui::crossterm::event::{self, KeyCode};
use tokio::sync::mpsc;

use crate::message::Message;

pub fn key_events(tx: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        if let Some(key) = event::read()?.as_key_press_event() {
            let message = match key.code {
                KeyCode::Tab => Message::TogglePlot,
                KeyCode::Esc => Message::Exit,
                KeyCode::Enter => Message::LoadPlayer,
                _ => Message::Input(key),
            };

            let should_exit = matches!(message, Message::Exit);

            if tx.blocking_send(message).is_err() {
                return Ok(());
            }

            if should_exit {
                return Ok(());
            }
        }
    }
}
