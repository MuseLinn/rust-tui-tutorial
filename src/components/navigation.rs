use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let help_text = "[q] 退出  [Enter] 继续  [↑↓] 滚动  [?] 提示  [Ctrl+R] 运行编译";
    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(ratatui::style::Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
