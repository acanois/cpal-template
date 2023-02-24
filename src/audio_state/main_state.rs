use cpal::traits::DeviceTrait;
use cpal::{Device, DeviceNameError, Host, SupportedStreamConfig};

pub struct AudioState {
    pub host: Host,
    pub input_device: Device,
    pub output_device: Device,
    pub input_config: SupportedStreamConfig,
    pub output_config: SupportedStreamConfig,
}

impl AudioState {
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
