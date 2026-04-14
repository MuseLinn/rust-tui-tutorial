use rand::prelude::*;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn glitch_paragraph(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    lines: &[&str],
    base_color: Color,
    accent_color: Color,
    tick: u64,
) {
    let glitch_chars = ["▓", "▒", "░", "█", "▀", "▄", "▌", "▐"];
    let cycle = 24u64;
    let phase = (tick % cycle) as f64 / cycle as f64;

    // Glitch probability pulsates: high in middle of cycle, low at ends
    let glitch_prob = if phase > 0.25 && phase < 0.75 {
        0.18
    } else {
        0.03
    };

    let mut rng = rand::rng();
    let mut text_lines = vec![];

    for line in lines {
        let mut spans = vec![];
        for ch in line.chars() {
            if ch == ' ' {
                spans.push(Span::raw(" "));
            } else if rng.random::<f64>() < glitch_prob {
                let gch = glitch_chars[rng.random_range(0..glitch_chars.len())];
                spans.push(Span::styled(
                    gch.to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            } else if rng.random::<f64>() < 0.06 {
                // Occasional bright flash (noise)
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(accent_color),
                ));
            } else if rng.random::<f64>() < 0.04 {
                // Rare red glitch
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Red),
                ));
            } else {
                let color = if phase > 0.35 {
                    base_color
                } else {
                    Color::Cyan
                };
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
        }
        text_lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(text_lines);
    frame.render_widget(paragraph, area);
}

pub fn flicker_hint(text: &str, tick: u64) -> Paragraph<'_> {
    let visible = (tick % 30) > 5;
    let color = if visible {
        Color::Green
    } else {
        Color::DarkGray
    };
    Paragraph::new(text).style(Style::default().fg(color))
}

pub fn scanlines_overlay(frame: &mut Frame, tick: u64) {
    let area = frame.area();
    let cycle = 6u64;
    for y in (0..area.height).step_by(3) {
        let active = (y as u64 + tick).is_multiple_of(cycle);
        if active {
            let line = ratatui::layout::Rect {
                x: 0,
                y,
                width: area.width,
                height: 1,
            };
            let span = Span::styled(
                " ".repeat(area.width as usize),
                Style::default().bg(Color::Rgb(10, 10, 15)),
            );
            frame.render_widget(Paragraph::new(Line::from(span)), line);
        }
    }
}
