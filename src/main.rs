mod cpal_helpers;
use cpal_helpers::io::AudioState;
use cpal_helpers::io::{
    get_available_hosts,
    make_host,
    get_default_input,
    get_default_output,
};

fn main() {
    let available_hosts = get_available_hosts();
    for host_id in available_hosts {
        match host_id.name() {
            "CoreAudio" => {
                let host = make_host(&host_id).unwrap();
                let input = get_default_input(&host).unwrap();
                let output = get_default_output(&host).unwrap();
                let audio_state = AudioState { host, input, output };

                println!("Host:\n\t{:?}", audio_state.host_name());
                println!("Input\n\t{:?}", audio_state.input_name().unwrap());
                println!("Output\n\t{:?}", audio_state.output_name().unwrap());
            },
            _ => ()
        }
    }
}
