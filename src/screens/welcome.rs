use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};

pub fn render(_app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let layout = Layout::vertical([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ]);
    let [_, center, _] = layout.areas(area);

    let inner_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);
    let [title_area, subtitle_area, _, hint_area] = inner_layout.areas(center);

    let title = Paragraph::new("🦀 Rust 交互式教程")
        .style(Style::default().fg(Color::Yellow).bold())
        .alignment(Alignment::Center);
    frame.render_widget(title, title_area);

    let subtitle = Paragraph::new("Interactive Rust Tutorial")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    frame.render_widget(subtitle, subtitle_area);

    let hint = Paragraph::new("按 [Enter] 开始学习")
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    frame.render_widget(hint, hint_area);
}
