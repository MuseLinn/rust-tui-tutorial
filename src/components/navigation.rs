use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, hint: &str) {
    let paragraph = Paragraph::new(hint)
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(ratatui::style::Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
