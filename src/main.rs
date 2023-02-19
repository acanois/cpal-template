mod cpal_helpers;
use cpal_helpers::io::{
    enumerate_io,
    get_available_hosts,
};

fn main() {
    match enumerate_io() {
        Err(err) => println!("{:?}", err),
        _ => ()
    }

    let available_hosts = get_available_hosts();
    println!("{:?}", available_hosts);
}
