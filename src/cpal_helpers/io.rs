extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};

pub fn enumerate_io() -> Result<(), anyhow::Error> {
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