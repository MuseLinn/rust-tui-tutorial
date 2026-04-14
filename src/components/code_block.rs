use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, code: &str, title: &str) {
    let mut lines = vec![];
    for line in code.lines() {
        let spans = vec![
            Span::raw("  "),
            Span::styled(
                line.to_string(),
                Style::default().fg(ratatui::style::Color::Cyan),
            ),
        ];
        lines.push(Line::from(spans));
    }

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ratatui::style::Color::DarkGray));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
