extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

fn enumerate_io() -> Result<(), anyhow::Error> {
    println!("All hosts: {:?}", cpal::ALL_HOSTS);
    println!("Available hosts: {:?}", cpal::available_hosts());
    
    let available_hosts = cpal::available_hosts();  

    for host_id in available_hosts {
        println!("{}", host_id.name());
        
        let host = cpal::host_from_id(host_id)?;
        let default_in = host.default_input_device().map(|e| e.name().unwrap());
        let default_out = host.default_output_device().map(|e| e.name().unwrap());
        println!("Default Input Device:\n\t    {:?}", default_in);
        println!("Default Output Device:\n\t    {:?}", default_out);

        let devices = host.devices()?;
        println!("Devices: ");
        for (device_index, device) in devices.enumerate() {
            println!("  {}. \"{}\"", device_index, device.name()?);
            
            // Input configs
            if let Ok(conf) = device.default_input_config() {
                println!("\nDefault input stream config:\n      {:?}", conf);
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("Error getting supported input configs: {:?}", e);
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                println!("All supported input stream configs:");
                for (config_index, config) in input_configs.into_iter().enumerate() {
                    println!(
                        "\t{}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                println!("\nDefault output stream config:\n      {:?}", conf);
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("Error getting supported output configs: {:?}", e);
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                println!("All supported output stream configs:");
                for (config_index, config) in output_configs.into_iter().enumerate() {
                    println!(
                        "\t{}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }
        }
    }

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f64, f64))
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = cpal::Sample::from::<f32>(&(sample.0 as f32));
        let right: T = cpal::Sample::from::<f32>(&(sample.1 as f32));

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;
    //let c = mls();
    //let c = (mls() | dc(400.0) | dc(50.0)) >> resonator();
    //let c = pink();

    // FM synthesis.
    //let f = 110.0;
    //let m = 5.0;
    //let c = oversample(sine_hz(f) * f * m + f >> sine());

    // Pulse wave.
    //let c = lfo(|t| {
    //    let pitch = 220.0;
    //    let duty = lerp11(0.01, 0.99, sin_hz(0.05, t));
    //    (pitch, duty)
    //}) >> pulse();

    //let c = zero() >> pluck(220.0, 0.8, 0.8);
    //let c = dc(110.0) >> dsf_saw_r(0.99);
    //let c = dc(110.0) >> triangle();
    //let c = dc(110.0) >> soft_saw();
    //let c = lfo(|t| xerp11(20.0, 2000.0, sin_hz(0.1, t))) >> dsf_square_r(0.99) >> lowpole_hz(1000.0);
    //let c = dc(110.0) >> square();
    let c = 0.2 * (organ_hz(midi_hz(57.0)) + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    //let c = organ_hz(110.1) + organ_hz(54.9);

    // Filtered noise tone.
    //let c = (noise() | dc((440.0, 50.0))) >> !resonator() >> resonator();

    // Test ease_noise.
    //let c = lfo(|t| xerp11(50.0, 5000.0, ease_noise(smooth9, 0, t))) >> triangle();

    // Bandpass filtering.
    let c = c >> (pass() | envelope(|t| xerp11(500.0, 5000.0, sin_hz(0.05, t)))) >> bandpass_q(5.0);

    // Waveshaper.
    //let c = c >> shape(Shape::Crush(20.0));

    // Add feedback delay.
    //let c = c >> (pass() & feedback(butterpass_hz(1000.0) >> delay(1.0) * 0.5));

    // Apply Moog filter.
    //let c = (c | lfo(|t| (xerp11(110.0, 11000.0, sin_hz(0.1, t)), 0.6))) >> moog();

    let c = c >> pan(0.0);

    //let c = fundsp::sound::risset_glissando(false);

    // Add chorus.
    let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    // Add flanger.
    //let c = c
    //    >> (flanger(0.6, 0.005, 0.01, |t| lerp11(0.005, 0.01, sin_hz(0.1, t)))
    //        | flanger(0.6, 0.005, 0.01, |t| lerp11(0.005, 0.01, cos_hz(0.1, t))));

    // Add phaser.
    //let c = c
    //    >> (phaser(0.5, |t| sin_hz(0.1, t) * 0.5 + 0.5)
    //        | phaser(0.5, |t| cos_hz(0.1, t) * 0.5 + 0.5));

    let mut c = c
        >> (declick() | declick())
        >> (dcblock() | dcblock())
        //>> (multipass() & 0.2 * reverb_stereo(10.0, 3.0))
        >> limiter_stereo((1.0, 5.0));
    //let mut c = c * 0.1;
    c.reset(Some(sample_rate));

    let mut next_value = move || c.get_stereo();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(50000));
    Ok(())
}

fn main() {
    
}
