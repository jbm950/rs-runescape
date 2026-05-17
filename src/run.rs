use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::app::App;
use crate::input::key_events;
use crate::ui::ui;

pub async fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    let mut app = App::new(tx.clone());
    let input_tx = tx.clone();
    std::thread::spawn(move || key_events(input_tx));
    app.request_load_player();

    terminal.draw(|frame| ui(frame, &mut app))?;

    while let Some(message) = rx.recv().await {
        app.update(message);

        terminal.draw(|frame| ui(frame, &mut app))?;

        if app.exit {
            break;
        }
    }

    Ok(())
}
