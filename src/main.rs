use std::{thread, time};
use clap::Parser;

mod server;
mod driver;

pub mod antd {
    tonic::include_proto!("antd");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0x0fcf)]
    vendor_id: u16,

    #[arg(short, long, default_value_t = 0x1008)]
    product_id: u16,
}

fn main() {
    let args = Args::parse();
    println!("Reading Device ... Vendor:{:04x} Product:{:04x}", args.vendor_id, args.product_id);

    match driver::device::new_ant_device(args.vendor_id, args.product_id) {
        Ok(mut device) => {
            device.start();
            thread::sleep(time::Duration::from_millis(5000));
            device.stop();
        },
        Err(e) => panic!("{}", e)
    }

    // match server::start_server() {
    //     Err(error) => panic!("could not start server: {:?}", error),
    //     Ok(()) => return,
    // }
}