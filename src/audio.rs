#![allow(clippy::precedence)]

use cpal::traits::DeviceTrait as _;
use cpal::traits::HostTrait as _;
use cpal::traits::StreamTrait as _;
use fundsp::hacker::*;

pub struct AudioHandle {
    _stream: cpal::Stream,
}

pub struct SoundControls {
    pub master: Shared,
    pub pink: Shared,
    pub sine_40hz: Shared,
    pub brown: Shared,
    pub white: Shared,
}

impl SoundControls {
    fn new() -> Self {
        Self {
            master: shared(1.0),
            pink: shared(0.3),
            sine_40hz: shared(0.2),
            brown: shared(0.0),
            white: shared(0.0),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Shared> {
        match index {
            0 => Some(&self.pink),
            1 => Some(&self.sine_40hz),
            2 => Some(&self.brown),
            3 => Some(&self.white),
            _ => None,
        }
    }
}

pub fn start_audio() -> color_eyre::Result<(AudioHandle, SoundControls)> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| color_eyre::eyre::eyre!("no output device found"))?;
    let config = device.default_output_config()?;

    let controls = SoundControls::new();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), &controls)?,
        cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), &controls)?,
        cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), &controls)?,
        _ => color_eyre::eyre::bail!("unsupported sample format"),
    };

    stream.play()?;

    Ok((AudioHandle { _stream: stream }, controls))
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    controls: &SoundControls,
) -> color_eyre::Result<cpal::Stream>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let master_vol = var(&controls.master);
    let pink_vol = var(&controls.pink);
    let sine_vol = var(&controls.sine_40hz);
    let brown_vol = var(&controls.brown);
    let white_vol = var(&controls.white);

    let mut graph =
        (pink_vol * pink() + sine_vol * sine_hz(40.0) + brown_vol * brown() + white_vol * white())
            * master_vol
            >> pan(0.0)
            >> (clip() | clip());

    graph.set_sample_rate(sample_rate);
    graph.allocate();

    let mut next_sample = move || graph.get_stereo();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let (left, right) = next_sample();
                let left = T::from_sample(left);
                let right = T::from_sample(right);
                for (i, sample) in frame.iter_mut().enumerate() {
                    *sample = if i % 2 == 0 { left } else { right };
                }
            }
        },
        |err| eprintln!("audio error: {err}"),
        None,
    )?;

    Ok(stream)
}
