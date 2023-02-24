mod cpal_helpers;
use cpal_helpers::io::AudioState;
use cpal_helpers::io::{
    get_available_hosts,
    make_host,
    get_default_input,
    get_default_output,
    show_default_input_config,
};

fn main() {
    let available_hosts = get_available_hosts();
    for host_id in available_hosts {
        match host_id.name() {
            "CoreAudio" => {
                let host = make_host(&host_id).unwrap();
                let input_device = get_default_input(&host).unwrap();
                let output_device = get_default_output(&host).unwrap();
                let audio_state = AudioState { 
                    host, 
                    input_device: &input_device, 
                    output_device: &output_device 
                };

                println!("Host:\n\t{:?}", audio_state.host_name());
                println!("Input\n\t{:?}", audio_state.input_name().unwrap());
                println!("Output\n\t{:?}", audio_state.output_name().unwrap());
                show_default_input_config(&input_device);
            },
            _ => ()
        }
    }
}
