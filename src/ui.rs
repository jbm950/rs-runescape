use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Bar, BarChart, Block, Clear, Paragraph},
};

use crate::app::{App, PlotType};

const PLOT_BOTTOM: &str = "Press <ctrl>-t to toggle plot";

pub fn ui(frame: &mut Frame, app: &mut App) {
    app.input_field.set_block(Block::bordered());

    let bar_chart = match app.plot_type {
        PlotType::PlayerLvls => player_plot(
            "Player Levels",
            app.player
                .iter_levels()
                .map(|(name, level)| Bar::with_label(name, level))
                .collect(),
        ),
        PlotType::PlayerXp => player_plot(
            "Player Xp",
            app.player
                .iter_xp()
                .map(|(name, level)| Bar::with_label(name, level))
                .collect(),
        ),
    };

    let [input_area, plot_area] =
        Layout::vertical([Constraint::Max(3), Constraint::Fill(1)]).areas(frame.area());

    frame.render_widget(&app.input_field, input_area);
    frame.render_widget(bar_chart, plot_area);

    if app.player_loading {
        loading_popup(frame, app);
    }
}

fn player_plot<'a>(title: &'a str, bars: Vec<Bar<'a>>) -> BarChart<'a> {
    BarChart::horizontal(bars)
        .bar_width(1)
        .block(Block::bordered().title_top(title).title_bottom(PLOT_BOTTOM))
}

fn loading_popup(frame: &mut Frame, app: &App) {
    let popup_area = frame
        .area()
        .centered(Constraint::Percentage(20), Constraint::Length(3));
    let popup_text = Paragraph::new(format!("Loading {}", app.player_name()))
        .block(Block::bordered())
        .centered();
    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_text, popup_area);
}
