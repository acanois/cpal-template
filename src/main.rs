mod audio_state;
mod cpal_helpers;

use audio_state::main_state::AudioState;
use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SizedSample,
};
use cpal_helpers::io::{
    get_default_input, get_default_input_config, get_default_output, get_default_output_config,
};


fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

pub fn debug_beep<T>(
    device: &Device, 
    config: &cpal::StreamConfig,
    sample_rate: f32,
    channels: usize,
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
 {
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn main() {
    let host = default_host();
    let input_device = get_default_input(&host).unwrap();
    let output_device = get_default_output(&host).unwrap();
    let input_config = get_default_input_config(&input_device).unwrap();
    let output_config = get_default_output_config(&output_device).unwrap();
    let in_sample_rate = input_config.sample_rate().0 as f32;
    let out_sample_rate = output_config.sample_rate().0 as f32;
    let num_input_channels = input_config.channels() as usize;
    let num_output_channels = output_config.channels() as usize;

    let audio_state = AudioState::new(
        host,
        input_device,
        output_device,
        input_config,
        output_config,
        in_sample_rate,
        out_sample_rate,
        num_input_channels,
        num_output_channels,
    );

    audio_state.log_state();

    debug_beep::<f32>(
        &audio_state.output_device, 
        &audio_state.output_config.into(), 
        audio_state.out_sample_rate, 
        audio_state.num_output_channels
    );
}
