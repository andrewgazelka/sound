#![allow(clippy::precedence)]

use cpal::traits::DeviceTrait as _;
use cpal::traits::HostTrait as _;
use cpal::traits::StreamTrait as _;
use fundsp::hacker::*;

fn main() -> eyre::Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| eyre::eyre!("no output device found"))?;
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        _ => eyre::bail!("unsupported sample format"),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> eyre::Result<()>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    // Pink noise + 40Hz sine wave, mixed and attenuated
    let mut graph = (0.3 * pink() + 0.2 * sine_hz(40.0))
        >> pan(0.0)
        >> (declick() | declick())
        >> (dcblock() | dcblock())
        >> limiter_stereo(1.0, 5.0);

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

    stream.play()?;

    println!("Playing pink noise + 40Hz tone. Press Ctrl+C to stop.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
