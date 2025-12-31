use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;

const BAR_WIDTH: usize = 20;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let constraints: Vec<Constraint> = app.sounds.iter().map(|_| Constraint::Length(1)).collect();
    let rows = Layout::vertical(constraints).split(area);

    for (i, sound) in app.sounds.iter().enumerate() {
        render_row(
            frame,
            rows[i],
            sound.name,
            sound.volume(),
            i == app.selected,
        );
    }
}

fn render_row(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    name: &str,
    volume: f32,
    selected: bool,
) {
    let prefix = if selected { "> " } else { "  " };
    let pct = (volume * 100.0) as u8;
    let filled = (volume * BAR_WIDTH as f32) as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    let style = if selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let line = Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{:<12}", name), style),
        Span::raw(" "),
        Span::styled(bar, style),
        Span::styled(format!(" {:>3}%", pct), style),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}
