Work in progress ...

Done:
- Connect / Disconnect to stick
- Message builder
- Implemented msg constants
- Minimalistic GRPC server

Todo:
- Read loop from usb and tunnel to grpc server
- Implement list devices

Usage (do not use please):

**Check for ANT2 USB Stick**
````
lsusb
````
should contain
````
Bus 001 Device 002: ID 0fcf:1008 Dynastream Innovations, Inc. ANTUSB2 Stick
````

VendorID `0fcf`, ProductID `1008`

````
Usage: ant-d [OPTIONS]

Options:
  -v, --vendor-id <VENDOR_ID>    [default: 4047]
  -p, --product-id <PRODUCT_ID>  [default: 4104]
  -h, --help                     Print help information
  -V, --version                  Print version information
````