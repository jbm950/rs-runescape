use ratatui_textarea::{CursorMove, TextArea};
use tokio::sync::mpsc;

use crate::message::Message;
use crate::runescape::{Player, fetch_player};

pub struct App {
    pub events_tx: mpsc::Sender<Message>,
    pub input_field: TextArea<'static>,
    pub player: Player,
    pub player_loading: bool,
    pub plot_type: PlotType,
    pub exit: bool,
}

impl App {
    pub fn new(events_tx: mpsc::Sender<Message>) -> Self {
        let mut input_field = TextArea::new(vec!["Salvsis2".to_string()]);
        input_field.move_cursor(CursorMove::End);
        Self {
            events_tx: events_tx,
            input_field: input_field,
            player: Player::default(),
            player_loading: false,
            plot_type: PlotType::PlayerLvls,
            exit: false,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Exit => self.exit = true,
            Message::PlayerLoaded(player) => {
                self.player_loading = false;
                self.player = player;
            }

            _ if self.player_loading => {}
            Message::Input(key) => {
                self.input_field.input_without_shortcuts(key);
            }
            Message::LoadPlayer => self.spawn_load_player(),
            Message::TogglePlot => {
                self.plot_type = match self.plot_type {
                    PlotType::PlayerLvls => PlotType::PlayerXp,
                    PlotType::PlayerXp => PlotType::PlayerLvls,
                }
            }
        }
    }

    pub fn spawn_load_player(&mut self) {
        self.player_loading = true;
        tokio::spawn(fetch_player(
            self.events_tx.clone(),
            self.input_field.lines().join(""),
        ));
    }
}

pub enum PlotType {
    PlayerLvls,
    PlayerXp,
}
