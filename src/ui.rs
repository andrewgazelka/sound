use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

const BAR_WIDTH: usize = 20;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .split(area);

    render_header(frame, layout[0]);
    render_sounds(app, frame, layout[1]);
    render_footer(frame, layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Sound Mixer ");
    frame.render_widget(block, area);
}

fn render_sounds(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default().borders(Borders::LEFT | Borders::RIGHT);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let constraints: Vec<Constraint> = app.sounds.iter().map(|_| Constraint::Length(1)).collect();

    let rows = Layout::vertical(constraints).split(inner);

    for (i, sound) in app.sounds.iter().enumerate() {
        let is_selected = i == app.selected;
        render_sound_row(frame, rows[i], sound.name, sound.volume(), is_selected);
    }
}

fn render_sound_row(frame: &mut Frame, area: Rect, name: &str, volume: f32, selected: bool) {
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

fn render_footer(frame: &mut Frame, area: Rect) {
    let help = " j/k: navigate | h/l: volume | q: quit ";
    let block = Block::default().borders(Borders::ALL).title(help);
    frame.render_widget(block, area);
}
