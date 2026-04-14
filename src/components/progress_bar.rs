use ratatui::{
    style::Style,
    widgets::{Block, Borders, Gauge},
    Frame,
};

pub fn render(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    current: usize,
    total: usize,
    label: &str,
) {
    let percent = if total > 0 {
        ((current as f64 / total as f64) * 100.0) as u16
    } else {
        0
    };
    let gauge = Gauge::default()
        .block(Block::default().title(label).borders(Borders::NONE))
        .gauge_style(Style::default().fg(ratatui::style::Color::Green))
        .percent(percent);
    frame.render_widget(gauge, area);
}
