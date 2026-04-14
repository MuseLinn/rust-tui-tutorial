use crate::app::App;
use crate::components::glitch_text::{flicker_hint, glitch_paragraph};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

const LOGO: [&str; 6] = [
    "",
    " ██████╗ ██╗   ██╗███████╗████████╗    ████████╗██╗   ██╗██╗",
    " ██╔══██╗██║   ██║██╔════╝╚══██╔══╝    ╚══██╔══╝██║   ██║██║",
    " ██████╔╝██║   ██║███████╗   ██║          ██║   ██║   ██║██║",
    " ██╔══██╗██║   ██║╚════██║   ██║          ██║   ██║   ██║██║",
    " ██║  ██║╚██████╔╝███████║   ██║          ██║   ╚██████╔╝██║",
];

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let layout = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ]);
    let [_, center, bottom] = layout.areas(area);

    let inner = Layout::vertical([
        Constraint::Length(7), // LOGO height
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);
    let [logo_area, tagline_area, _, hint_area] = inner.areas(center);

    // Glitch ASCII logo
    glitch_paragraph(
        frame,
        logo_area,
        &LOGO,
        Color::Cyan,
        Color::Magenta,
        app.state.frame,
    );

    // Tagline
    let tagline = Paragraph::new("INTERACTIVE RUST TUTORIAL v0.1")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(tagline, tagline_area);

    // Animated Enter hint
    let hint = if (app.state.frame % 40) < 30 {
        ">>> PRESS [ENTER] TO JACK IN <<<"
    } else {
        "                                "
    };
    let hint_para = flicker_hint(hint, app.state.frame).alignment(Alignment::Center);
    frame.render_widget(hint_para, hint_area);

    // Version / credits in bottom corner
    let credits = Paragraph::new("[ ratatui | tokio | rust ]")
        .style(Style::default().fg(Color::Rgb(60, 60, 70)))
        .alignment(Alignment::Center);
    frame.render_widget(credits, bottom);
}
