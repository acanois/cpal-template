mod audio_state;
mod cpal_helpers;

use audio_state::main_state::AudioState;
use cpal::default_host;
use cpal_helpers::io::{
    get_default_input,
    get_default_output,
    get_default_input_config,
    get_default_output_config,
};

fn main() {
    let host = default_host();
    let input_device = get_default_input(&host).unwrap();
    let output_device = get_default_output(&host).unwrap();
    let input_config = get_default_input_config(&input_device).unwrap();
    let output_config = get_default_output_config(&output_device).unwrap();
    let in_sample_rate = input_config.sample_rate().0;
    let out_sample_rate = output_config.sample_rate().0;
    let num_input_channels = input_config.channels();
    let num_output_channels = output_config.channels();
    
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
}
