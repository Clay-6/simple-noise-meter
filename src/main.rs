use std::io::Read;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use num_traits::NumCast;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available D:");
    let config = device
        .default_input_config()
        .expect("Failed to find default config");

    let err_fn = move |err| eprintln!("Error on stream: {err}");
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| input_callback::<i8>(data),
                err_fn,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| input_callback::<i16>(data),
                err_fn,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::I32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| input_callback::<i32>(data),
                err_fn,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| input_callback::<f32>(data),
                err_fn,
                None,
            )
            .unwrap(),
        sample_format => {
            panic!("Unsupported sample format '{sample_format}'")
        }
    };
    stream.play().unwrap();
    // Make the main thread wait until enter is pressed
    std::io::stdin().read_exact(&mut [0]).unwrap();
}

fn input_callback<T>(data: &[T])
where
    T: std::fmt::Debug + Copy + NumCast,
{
    let rms = (data
        .iter()
        .map(|s| {
            let float_sample: f64 = num_traits::cast(*s).unwrap(); // Cast the sample to a float
            float_sample * float_sample
        })
        .sum::<f64>()
        / num_traits::cast::<usize, f64>(data.len()).unwrap()) // Cast length to a float
    .sqrt();
    let db_level = 15.0 * (rms).log10().abs();
    if db_level.is_finite() {
        // Sometimes `db_level` ends up being NaN & this causes problems
        // I have elected to ignore the root cause in the name of simplicity
        print!("\r{db_level:.2}   ");
    }
}
