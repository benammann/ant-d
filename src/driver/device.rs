use std::fmt::Error;
use std::time::Duration;
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext};
use rusb::constants::{LIBUSB_ENDPOINT_IN, LIBUSB_ENDPOINT_OUT};
use crate::driver::messages;
use crate::driver::messages::Message;

pub struct AntDevice {
    device: Device<Context>,
    context: Context,
    descriptor: DeviceDescriptor,
    handle: DeviceHandle<Context>,
    buf: Message,
    is_running: bool
}

impl AntDevice {

    pub fn start(&self) {
        // todo: loop read messages, verify them and send them to the grpc server
    }

    pub fn stop(&mut self) {
       self.is_running = false;
    }

    fn open(&self) -> rusb::Result<usize> {
        return Ok(0);
    }

    fn close(&self) -> rusb::Result<usize> {
        let res1 = self.write(messages::close_channel_message(0));
        let res2 = self.write(messages::system_reset_message());
        return res1.and_then(|_| res2);
    }

    fn read(&self) -> Box<Vec<u8>> {
        let mut msg = vec![];
        return match self.handle.read_bulk(LIBUSB_ENDPOINT_IN,&mut msg, self.get_timeout()) {
            Ok(size) => Box::new(msg),
            _ => {
                println!("cloud not read");
                return Box::new(msg)
            }
        }
    }

    fn write(&self, mut buf: messages::Message) -> rusb::Result<usize> {
        return self.handle.write_bulk(LIBUSB_ENDPOINT_OUT, buf.as_mut_slice(), self.get_timeout());
    }

    fn buffer_size(&self) -> u32 {
        return 64;
    }

    fn get_timeout(&self) -> Duration {
        return Duration::from_secs(20);
    }

}

pub fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
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

pub fn new_ant_device(vendor_id: u16, product_id: u16) -> Result<AntDevice, Error> {
    return match Context::new() {
        Ok(mut context) => match open_device(&mut context, vendor_id, product_id) {
            Some((device, descriptor, handle)) => {
                let mut msg = vec![];
                Ok(AntDevice{
                    device,
                    descriptor,
                    context,
                    handle,
                    is_running: true,
                    buf: Box::new(msg),
                })
            },
            None => panic!("could not find device")
        },
        Err(e) => panic!("could not initialize libusb: {}", e),
    }
}