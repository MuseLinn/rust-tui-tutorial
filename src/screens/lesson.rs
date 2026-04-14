use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
    let [header, content] = layout.areas(area);

    // Header
    let (phase_title, lesson_title) = match &app.state.screen {
        crate::models::Screen::Lesson {
            phase_id,
            lesson_id,
            ..
        } => {
            let pt = app
                .manifest
                .phases
                .iter()
                .find(|p| &p.id == phase_id)
                .map(|p| p.title.as_str())
                .unwrap_or("");
            let lt = app
                .manifest
                .phases
                .iter()
                .flat_map(|p| p.lessons.iter())
                .find(|l| &l.id == lesson_id)
                .map(|l| l.title.as_str())
                .unwrap_or("");
            (pt, lt)
        }
        _ => ("", ""),
    };

    let header_text = format!("{} / {}", phase_title, lesson_title);
    let header_para = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::Yellow).bold());
    frame.render_widget(header_para, header);

    // Content
    let lesson = match &app.state.screen {
        crate::models::Screen::Lesson { lesson_id, .. } => app
            .manifest
            .phases
            .iter()
            .flat_map(|p| p.lessons.iter())
            .find(|l| &l.id == lesson_id),
        _ => None,
    };

    if let Some(lesson) = lesson {
        let content_para = Paragraph::new(lesson.content_md.as_str())
            .block(Block::default().borders(Borders::ALL).title(" 课程内容 "))
            .wrap(Wrap { trim: true })
            .scroll((app.state.scroll_offset, 0));
        frame.render_widget(content_para, content);
    }
}
