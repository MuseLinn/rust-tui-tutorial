use crate::app::App;
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title(" 课程目录 (Lesson Menu) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut items: Vec<ListItem> = vec![];
    let mut flat_index = 0;

    for phase in &app.manifest.phases {
        items.push(ListItem::new(Line::from(Span::styled(
            format!("📚 {}", phase.title),
            Style::default().fg(Color::Yellow).bold(),
        ))));

        for lesson in &phase.lessons {
            let is_selected = flat_index == app.state.menu_selection;
            let is_completed = app.progress.completed_lessons.contains(&lesson.id);
            let marker = if is_completed { " ✓ " } else { "   " };
            let text = format!("{} {}", marker, lesson.title);

            let style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            items.push(ListItem::new(Line::from(Span::styled(text, style))));
            flat_index += 1;
        }
    }

    let list = List::new(items);
    frame.render_widget(list, inner);
}

pub fn total_lesson_count(app: &App) -> usize {
    app.manifest.phases.iter().map(|p| p.lessons.len()).sum()
}
