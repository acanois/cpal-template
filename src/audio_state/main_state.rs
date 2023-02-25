use cpal::traits::DeviceTrait;
use cpal::{Device, DeviceNameError, Host, SupportedStreamConfig};

pub struct AudioState {
    pub host: Host,
    pub input_device: Device,
    pub output_device: Device,
    pub input_config: SupportedStreamConfig,
    pub output_config: SupportedStreamConfig,
    pub in_sample_rate: f32,
    pub out_sample_rate: f32,
    pub num_input_channels: usize,
    pub num_output_channels: usize,
}

impl AudioState {
    pub fn new(
        host: Host,
        input_device: Device,
        output_device: Device,
        input_config: SupportedStreamConfig,
        output_config: SupportedStreamConfig,
        in_sample_rate: f32,
        out_sample_rate: f32,
        num_input_channels: usize,
        num_output_channels: usize,
    ) -> AudioState {
        AudioState {
            host,
            input_device,
            output_device,
            input_config,
            output_config,
            in_sample_rate,
            out_sample_rate,
            num_input_channels,
            num_output_channels,
        }
    }

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
        println!("\nIn Sample Rate\n\t{:?}", self.in_sample_rate);
        println!("Out Sample Rate\n\t{:?}", self.out_sample_rate);
    }
}
