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
        PlotType::PlayerLvls => player_levels_plot(app),
        PlotType::PlayerXp => player_xp_plot(app),
    };

    let [input_area, plot_area] =
        Layout::vertical([Constraint::Max(3), Constraint::Fill(1)]).areas(frame.area());

    frame.render_widget(&app.input_field, input_area);
    frame.render_widget(bar_chart, plot_area);

    if app.player_loading {
        let popup_area = frame
            .area()
            .centered(Constraint::Percentage(20), Constraint::Length(3));
        let popup_text = Paragraph::new(format!("Loading {}", app.input_field.lines().join("")))
            .block(Block::bordered())
            .centered();
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
    }
}

fn player_levels_plot(app: &App) -> BarChart<'_> {
    let bars: Vec<Bar> = app
        .player
        .iter_levels()
        .map(|(name, level)| Bar::with_label(name, level))
        .collect();

    BarChart::horizontal(bars).bar_width(1).block(
        Block::bordered()
            .title_top("Player Levels")
            .title_bottom(PLOT_BOTTOM),
    )
}

fn player_xp_plot(app: &App) -> BarChart<'_> {
    let bars: Vec<Bar> = app
        .player
        .iter_xp()
        .map(|(name, xp)| Bar::with_label(name, xp).text_value(""))
        .collect();

    BarChart::horizontal(bars).bar_width(1).block(
        Block::bordered()
            .title_top("Player Xp")
            .title_bottom(PLOT_BOTTOM),
    )
}
