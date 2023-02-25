extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{DefaultStreamConfigError, Device, Host, HostId};

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
