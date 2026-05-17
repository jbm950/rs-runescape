use ratatui::crossterm::event::KeyEvent;

use crate::runescape::Player;

#[derive(Clone)]
pub enum Message {
    Input(KeyEvent),
    LoadPlayer,
    PlayerLoaded(Player),
    TogglePlot,
    Exit,
}
