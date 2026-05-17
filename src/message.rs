use ratatui::crossterm::event::KeyEvent;

use crate::runescape::Player;

pub enum Message {
    Input(KeyEvent),
    LoadPlayer,
    PlayerLoaded(Player),
    TogglePlot,
    Exit,
}
