mod cpal_helpers;
use cpal_helpers::io::enumerate_io;

fn main() {
    match enumerate_io() {
        Err(err) => println!("{:?}", err),
        _ => ()
    }
}
