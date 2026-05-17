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
            events_tx,
            input_field,
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

            Message::Input(key) if !self.player_loading => {
                self.input_field.input_without_shortcuts(key);
            }
            Message::LoadPlayer if !self.player_loading => self.request_load_player(),
            Message::TogglePlot if !self.player_loading => self.plot_type.toggle(),
            _ => {}
        }
    }

    pub fn request_load_player(&mut self) {
        self.player_loading = true;

        let tx = self.events_tx.clone();
        let player_name = self.player_name().clone();

        tokio::spawn(async move {
            let player = fetch_player(player_name).await;
            tx.send(Message::PlayerLoaded(player)).await.unwrap();
        });

    }

    pub fn player_name(&self) -> String {
        self.input_field.lines().join("")
    }
}

pub enum PlotType {
    PlayerLvls,
    PlayerXp,
}

impl PlotType {
    fn toggle(&mut self) {
        *self = match self {
            Self::PlayerLvls => Self::PlayerXp,
            Self::PlayerXp => Self::PlayerLvls,
        }
    }
}
