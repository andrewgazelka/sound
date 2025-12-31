use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use fundsp::hacker::Shared;
use ratatui::DefaultTerminal;

use crate::audio::{AudioHandle, start_audio};
use crate::ui;

const VOLUME_STEP: f32 = 0.05;

pub struct Sound {
    pub name: &'static str,
    control: Shared,
}

impl Sound {
    fn new(name: &'static str, control: Shared) -> Self {
        Self { name, control }
    }

    pub fn volume(&self) -> f32 {
        self.control.value()
    }

    pub fn set_volume(&self, vol: f32) {
        self.control.set_value(vol.clamp(0.0, 1.0));
    }

    pub fn adjust_volume(&self, delta: f32) {
        self.set_volume(self.volume() + delta);
    }
}

pub struct App {
    pub sounds: Vec<Sound>,
    pub selected: usize,
    _audio: AudioHandle,
    should_exit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let (audio, controls) = start_audio()?;

        let sounds = vec![
            Sound::new("Pink Noise", controls.pink.clone()),
            Sound::new("40Hz Sine", controls.sine_40hz.clone()),
            Sound::new("Brown Noise", controls.brown.clone()),
            Sound::new("White Noise", controls.white.clone()),
        ];

        Ok(Self {
            sounds,
            selected: 0,
            _audio: audio,
            should_exit: false,
        })
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| ui::render(&self, frame))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
            KeyCode::Char('h') | KeyCode::Left => self.decrease_volume(),
            KeyCode::Char('l') | KeyCode::Right => self.increase_volume(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.sounds.len();
    }

    fn select_prev(&mut self) {
        self.selected = self
            .selected
            .checked_sub(1)
            .unwrap_or(self.sounds.len() - 1);
    }

    fn increase_volume(&mut self) {
        self.sounds[self.selected].adjust_volume(VOLUME_STEP);
    }

    fn decrease_volume(&mut self) {
        self.sounds[self.selected].adjust_volume(-VOLUME_STEP);
    }
}
