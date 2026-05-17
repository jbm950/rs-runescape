mod app;
mod input;
mod message;
mod run;
mod runescape;
mod ui;

use run::run_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal).await;

    ratatui::restore();

    result
}
