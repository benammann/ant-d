extern crate core;

use std::{thread, time};
use std::sync::mpsc;
use clap::Parser;
use crate::driver::messages::Message;

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

    let (tx, rx) = mpsc::channel();

    driver::device::stream_rx_scan_mode(args.vendor_id, args.product_id, tx).expect("could not start stream");

    for ant_msg  in rx {
        println!("Received Message");
        for ant_byte in ant_msg.as_slice() {
            println!("{}", ant_byte)
        }
    }

    // match driver::device::new_ant_device(args.vendor_id, args.product_id) {
    //     Ok(mut device) => {
    //         device.start();
    //         match server::start_server() {
    //             Err(error) => panic!("could not start server: {:?}", error),
    //             Ok(()) => return,
    //         }
    //     },
    //     Err(e) => panic!("{}", e)
    // }
}