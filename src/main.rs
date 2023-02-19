mod cpal_helpers;
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
                let input = get_default_input(&host);
                println!("{:?}", input);
                let output = get_default_output(&host);
                println!("{:?}", output);
            },
            _ => ()
        }
    }
}
