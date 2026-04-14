use crate::app::App;
use crate::components::progress_bar;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let layout = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Percentage(20),
    ]);
    let [_, title, stat1, stat2, progress, hint] = layout.areas(area);

    let title_para = Paragraph::new("🎉 阶段完成！Phase Complete!")
        .style(Style::default().fg(Color::Green).bold())
        .alignment(Alignment::Center);
    frame.render_widget(title_para, title);

    let completed_lessons = app.progress.completed_lessons.len();
    let total_lessons: usize = app.manifest.phases.iter().map(|p| p.lessons.len()).sum();
    let stat1_text = format!(
        "已完成课程 / Lessons completed: {} / {}",
        completed_lessons, total_lessons
    );
    let stat1_para = Paragraph::new(stat1_text).alignment(Alignment::Center);
    frame.render_widget(stat1_para, stat1);

    let completed_exercises = app.progress.completed_exercises.len();
    let total_exercises: usize = app
        .manifest
        .phases
        .iter()
        .flat_map(|p| p.lessons.iter())
        .filter(|l| l.exercise.is_some())
        .count();
    let stat2_text = format!(
        "已完成练习 / Exercises completed: {} / {}",
        completed_exercises, total_exercises
    );
    let stat2_para = Paragraph::new(stat2_text).alignment(Alignment::Center);
    frame.render_widget(stat2_para, stat2);

    progress_bar::render(
        frame,
        progress,
        completed_lessons,
        total_lessons,
        " 总进度 / Overall Progress ",
    );

    let hint_para = Paragraph::new("按 [Enter] 返回课程目录")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    frame.render_widget(hint_para, hint);
}
