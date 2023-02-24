extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{DefaultStreamConfigError, Device, Host, HostId, SupportedStreamConfig};

pub struct AudioState<'a> {
    pub host: Host,
    pub input_device: &'a Device,
    pub output_device: &'a Device,
    pub input_config: &'a SupportedStreamConfig,
    pub output_config: &'a SupportedStreamConfig,
}

impl AudioState<'_> {
    pub fn host_name(&self) -> &str {
        self.host.id().name()
    }

    pub fn input_name(&self) -> Result<String, cpal::DeviceNameError> {
        self.input_device.name()
    }

    pub fn output_name(&self) -> Result<String, cpal::DeviceNameError> {
        self.output_device.name()
    }
}

pub fn get_available_hosts() -> Vec<HostId> {
    cpal::available_hosts()
}

pub fn make_host(host_id: &HostId) -> Result<cpal::Host, cpal::HostUnavailable> {
    cpal::host_from_id(*host_id)
}

pub fn get_default_input(host: &Host) -> Option<Device> {
    host.default_input_device()
}

pub fn get_default_output(host: &Host) -> Option<Device> {
    host.default_output_device()
}

pub fn get_default_input_config(
    device: &Device,
) -> Result<cpal::SupportedStreamConfig, DefaultStreamConfigError> {
    device.default_input_config()
}

pub fn get_default_output_config(
    device: &Device,
) -> Result<cpal::SupportedStreamConfig, DefaultStreamConfigError> {
    device.default_output_config()
}

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
                    println!("\t{}.{}. {:?}", device_index + 1, config_index + 1, config);
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
                    println!("\t{}.{}. {:?}", device_index + 1, config_index + 1, config);
                }
            }
        }
    }
    Ok(())
}
