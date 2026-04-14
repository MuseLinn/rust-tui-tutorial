use crate::app::App;
use crate::components::code_block;
use crate::models::{CompileResult, Screen};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: ratatui::layout::Rect) {
    let exercise = match &app.state.screen {
        Screen::Exercise { exercise_id, .. } => app
            .manifest
            .phases
            .iter()
            .flat_map(|p| p.lessons.iter())
            .filter_map(|l| l.exercise.as_ref())
            .find(|e| &e.id == exercise_id),
        _ => None,
    };

    let Some(ex) = exercise else {
        return;
    };

    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ]);
    let [header, instr_area, code_area, result_area] = layout.areas(area);

    // Header
    let header_para = Paragraph::new(format!("══ 练习 // EXERCISE: {} ══", ex.id))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Color::Cyan),
        )
        .style(Style::default().fg(Color::Cyan).bold());
    frame.render_widget(header_para, header);

    // Instructions
    let instr_para = Paragraph::new(ex.instructions.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 说明 (Instructions) "),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(instr_para, instr_area);

    // Code block / Textarea
    if let Some(ref textarea) = app.textarea {
        frame.render_widget(textarea, code_area);
    } else {
        let user_code = match &app.state.screen {
            Screen::Exercise { user_code, .. } => user_code.as_str(),
            _ => ex.starter_code.as_str(),
        };
        code_block::render(
            frame,
            code_area,
            user_code,
            " 代码 (Code) — [Ctrl+R] 运行编译 ",
        );
    }

    // Compile result
    let result_text = match &app.state.screen {
        Screen::Exercise {
            compile_result: Some(result),
            ..
        } => match result {
            CompileResult::Success => Line::from(Span::styled(
                "✅ 编译通过 / Compilation successful!",
                Style::default().fg(Color::Green),
            )),
            CompileResult::CompileError { stderr } => Line::from(vec![
                Span::styled(
                    "❌ 编译错误 / Compile Error:\n",
                    Style::default().fg(Color::Red),
                ),
                Span::raw(stderr.lines().next().unwrap_or("")),
            ]),
            CompileResult::RuntimeMismatch { expected, got } => Line::from(vec![
                Span::styled(
                    "❌ 输出不匹配 / Output mismatch:\n",
                    Style::default().fg(Color::Red),
                ),
                Span::raw(format!("expected: {}, got: {}", expected, got)),
            ]),
            CompileResult::PatternMismatch { expected } => Line::from(vec![
                Span::styled(
                    "❌ 模式不匹配 / Pattern mismatch:\n",
                    Style::default().fg(Color::Red),
                ),
                Span::raw(format!("expected pattern: {}", expected)),
            ]),
            CompileResult::Timeout => Line::from(Span::styled(
                "⏱️ 编译超时 / Timeout",
                Style::default().fg(Color::Yellow),
            )),
        },
        _ => Line::from(Span::styled(
            "按 [Ctrl+R] 运行编译验证",
            Style::default().fg(Color::DarkGray),
        )),
    };

    let result_para = Paragraph::new(result_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 结果 (Result) "),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(result_para, result_area);
}
