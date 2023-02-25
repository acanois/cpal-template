mod audio_state;
mod cpal_helpers;

use audio_state::main_state::AudioState;
use cpal_helpers::io::{
    get_available_hosts,
    make_host,
    get_default_input,
    get_default_output,
    get_default_input_config,
    get_default_output_config,
};

fn main() {
    let available_hosts = get_available_hosts();
    for host_id in available_hosts {
        match host_id.name() {
            "CoreAudio" => {
                let host = make_host(&host_id).unwrap();
                let input_device = get_default_input(&host).unwrap();
                let output_device = get_default_output(&host).unwrap();
                let input_config = get_default_input_config(&input_device).unwrap();
                let output_config = get_default_output_config(&output_device).unwrap();
                let in_sample_rate = input_config.sample_rate().0;
                let out_sample_rate = output_config.sample_rate().0;
                
                let audio_state = AudioState { 
                    host, 
                    input_device, 
                    output_device,
                    input_config,
                    output_config,
                    in_sample_rate,
                    out_sample_rate,
                };

                audio_state.log_state();
            },
            _ => ()
        }
    }
}
