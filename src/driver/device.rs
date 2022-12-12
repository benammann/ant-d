use std::borrow::{Borrow, BorrowMut};
use std::fmt::Error;
use std::sync::mpsc::Sender;
use std::time::Duration;
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, Recipient, request_type, RequestType, UsbContext};
use rusb::constants::{LIBUSB_DT_STRING, LIBUSB_ENDPOINT_IN, LIBUSB_ENDPOINT_OUT, LIBUSB_REQUEST_GET_DESCRIPTOR, LIBUSB_REQUEST_SET_DESCRIPTOR};
use crate::driver::messages;
use crate::driver::messages::Message;

pub fn open_device(
    context: &Context,
    vid: u16,
    pid: u16,
) -> Option<(Device<Context>, DeviceDescriptor, DeviceHandle<Context>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    None
}

fn read(handle: &DeviceHandle<Context>) -> Message {
    let mut msg = vec![];
    return match handle.read_control(
        request_type(Direction::In, RequestType::Standard, Recipient::Device),
        LIBUSB_REQUEST_GET_DESCRIPTOR,
        u16::from(LIBUSB_DT_STRING) << 8,
        0,
        &mut msg,
        get_timeout()
    ) {
        Ok(size) => Box::new(msg),
        Err(error) => panic!("could not read {}", error),
        _ => {
        println!("cloud not read");
        return Box::new(msg)
        }
    }
}

fn write(handle: &DeviceHandle<Context>, mut msg: Message) -> rusb::Result<usize> {
    return handle.write_control(
        request_type(Direction::Out, RequestType::Standard, Recipient::Device),
        LIBUSB_REQUEST_GET_DESCRIPTOR,
        u16::from(LIBUSB_DT_STRING) << 8,
        0,
        &mut msg,
        get_timeout()
    );
}

fn write_wrapper(handle: &DeviceHandle<Context>, mut msg: Message) {
    return match write(handle, msg) {
        Err(error) => panic!("could not write: {}", error),
        _ => {}
    }
}

fn get_timeout() -> Duration {
    return Duration::from_millis(100);
}

pub fn stream_rx_scan_mode(vendor_id: u16, product_id: u16, tx: Sender<Message>) -> Result<(), Error> {

    let ctx = match Context::new() {
        Ok(context) => context,
        Err(e) => panic!("could not initialize libusb: {}", e),
    };

    let (_, _, mut handle) = open_device(&ctx, vendor_id, product_id).expect("could not open device");

    match handle.reset() {
        Err(err) => panic!("could not reset {}", err),
        _ => {}
    }

    write_wrapper(&handle, messages::system_reset());
    write_wrapper(&handle, messages::set_network_key(0, messages::constants::ant_plus_network_key()));
    write_wrapper(&handle, messages::assign_channel(0, 0x40));
    write_wrapper(&handle, messages::set_channel_id(0));
    write_wrapper(&handle, messages::set_channel_rf_frequency(0, 2457));
    write_wrapper(&handle, messages::open_rx_scan_mode());

    loop {
        // read next message from device
        let msg = read(&handle);

        // only send sync messages to tx
        if msg.len() <= 0 || msg[0] != messages::constants::MESSAGE_TX_SYNC {
            continue
        }

        // todo: integrity checking but tbh who needs that :trollface:
        tx.send(msg).unwrap();
    }

}