use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use fundsp::hacker::Shared;
use ratatui::DefaultTerminal;

use crate::audio::{AudioHandle, start_audio};
use crate::state::State;
use crate::ui;

const VOLUME_STEP: f32 = 0.05;

pub struct Sound {
    pub name: &'static str,
    control: Shared,
    muted_volume: Option<f32>,
}

impl Sound {
    fn new(name: &'static str, control: Shared) -> Self {
        Self {
            name,
            control,
            muted_volume: None,
        }
    }

    pub fn volume(&self) -> f32 {
        self.control.value()
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.control.set_value(vol.clamp(0.0, 1.0));
        self.muted_volume = None;
    }

    pub fn adjust_volume(&mut self, delta: f32) {
        self.set_volume(self.volume() + delta);
    }

    pub fn toggle_mute(&mut self) {
        if let Some(vol) = self.muted_volume.take() {
            self.control.set_value(vol);
        } else {
            self.muted_volume = Some(self.volume().max(0.3));
            self.control.set_value(0.0);
        }
    }

    pub fn is_muted(&self) -> bool {
        self.muted_volume.is_some()
    }
}

pub struct App {
    pub master: Sound,
    pub sounds: Vec<Sound>,
    pub selected: usize,
    _audio: AudioHandle,
    should_exit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let state = State::load();
        let (audio, controls) = start_audio()?;

        let mut master = Sound::new("Master", controls.master.clone());
        master.set_volume(state.master);

        let mut sounds = vec![
            Sound::new("Pink Noise", controls.pink.clone()),
            Sound::new("40Hz Sine", controls.sine_40hz.clone()),
            Sound::new("Brown Noise", controls.brown.clone()),
            Sound::new("White Noise", controls.white.clone()),
        ];

        for (i, vol) in state.volumes.iter().enumerate() {
            sounds[i].set_volume(*vol);
        }

        Ok(Self {
            master,
            sounds,
            selected: 0,
            _audio: audio,
            should_exit: false,
        })
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| ui::render(&self, frame))?;
            while event::poll(Duration::ZERO)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key);
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(16));
        }
        self.save_state();
        Ok(())
    }

    fn save_state(&self) {
        let state = State {
            master: self.master.volume(),
            volumes: [
                self.sounds[0].volume(),
                self.sounds[1].volume(),
                self.sounds[2].volume(),
                self.sounds[3].volume(),
            ],
        };
        state.save();
    }

    fn selected_sound(&mut self) -> &mut Sound {
        if self.selected == 0 {
            &mut self.master
        } else {
            &mut self.sounds[self.selected - 1]
        }
    }

    fn total_items(&self) -> usize {
        1 + self.sounds.len()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
            KeyCode::Char('h') | KeyCode::Left => self.selected_sound().adjust_volume(-VOLUME_STEP),
            KeyCode::Char('l') | KeyCode::Right => self.selected_sound().adjust_volume(VOLUME_STEP),
            KeyCode::Char(c @ '0'..='9') => {
                let vol = (c as u8 - b'0') as f32 / 9.0;
                self.selected_sound().set_volume(vol);
            }
            KeyCode::Char('m') => self.master.toggle_mute(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.total_items();
    }

    fn select_prev(&mut self) {
        self.selected = self
            .selected
            .checked_sub(1)
            .unwrap_or(self.total_items() - 1);
    }
}
