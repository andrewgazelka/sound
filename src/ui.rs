use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, Sound};

const BAR_WIDTH: usize = 20;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let total = 1 + app.sounds.len();
    let constraints: Vec<Constraint> = (0..total).map(|_| Constraint::Length(1)).collect();
    let rows = Layout::vertical(constraints).split(area);

    render_row(frame, rows[0], &app.master, app.selected == 0);

    for (i, sound) in app.sounds.iter().enumerate() {
        render_row(frame, rows[i + 1], sound, app.selected == i + 1);
    }
}

fn render_row(frame: &mut Frame, area: ratatui::layout::Rect, sound: &Sound, selected: bool) {
    let prefix = if selected { "> " } else { "  " };
    let volume = sound.volume();
    let pct = (volume * 100.0) as u8;
    let filled = (volume * BAR_WIDTH as f32) as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    let mute_indicator = if sound.is_muted() { " [M]" } else { "" };

    let style = if selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if sound.is_muted() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    };

    let line = Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{:<12}", sound.name), style),
        Span::raw(" "),
        Span::styled(bar, style),
        Span::styled(format!(" {:>3}%{}", pct, mute_indicator), style),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}
