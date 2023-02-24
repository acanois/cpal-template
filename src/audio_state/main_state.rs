use cpal::traits::DeviceTrait;
use cpal::{Device, DeviceNameError, Host, SupportedStreamConfig};

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

    pub fn input_name(&self) -> Result<String, DeviceNameError> {
        self.input_device.name()
    }

    pub fn output_name(&self) -> Result<String, DeviceNameError> {
        self.output_device.name()
    }

    pub fn log_state(&self) {
        println!("Host:\n\t{:?}", self.host_name());
        println!("Input\n\t{:?}", self.input_name().unwrap());
        println!("Output\n\t{:?}", self.output_name().unwrap());
        println!("\nInput Config\n\t{:?}", self.input_config);
        println!("\nOutput Config\n\t{:?}", self.output_config);
    }
}
