pub mod constants;

struct Messages;

pub type Message = Box<Vec<u8>>;

fn build_message(message_id: u8, payload: Vec<u8>) -> Message {
    let mut msg = vec![];
    msg.push(constants::MESSAGE_TX_SYNC);
    msg.push(payload.len() as u8);
    msg.push(message_id);
    for payload_item in payload {
        msg.push(payload_item.to_owned());
    }
    msg.push(get_checksum(msg.to_owned()));
    return Box::new(msg);
}

fn get_checksum(message: Vec<u8>) -> u8 {
    let mut checksum: u8 = 0;
    for message_item in message {
        checksum ^= message_item;
    }
    return checksum;
}

pub fn close_channel_message(channel: u8) -> Message {
    return build_message(constants::MESSAGE_CHANNEL_CLOSE, vec![channel])
}

pub fn system_reset() -> Message {
    return build_message(constants::MESSAGE_SYSTEM_RESET, vec![0x00])
}

pub fn set_network_key(channel: u8, key: Vec<u8>) -> Message {
    let payload = [vec![channel], key].concat();
    return build_message(constants::MESSAGE_NETWORK_KEY, payload)
}

pub fn assign_channel(channel: u8, typ: u8) -> Message {
    return build_message(constants::MESSAGE_CHANNEL_ASSIGN, vec![channel, typ, 0x00]);
}

pub fn set_channel_id(channel: u8) -> Message {
    return build_message(constants::MESSAGE_CHANNEL_ID, vec![channel, 0x00, 0x00, 0x00, 0x00])
}

pub fn set_channel_rf_frequency(channel: u8, freq: u16) -> Message {
    let ff = (freq-2400) as u8;
    let payload = vec![channel, ff];
    return build_message(constants::MESSAGE_CHANNEL_FREQUENCY, payload);
}

pub fn enable_extended_messages(enable: bool) -> Message {
    let mut opt: u8 = 0x00;
    if enable == true {
        opt = 0x01;
    }
    return build_message(constants::CAPABILITIES_EXT_MESSAGE_ENABLED, vec![opt]);
}

pub fn open_rx_scan_mode() -> Message {
    return build_message(constants::MESSAGE_CHANNEL_OPEN_RX_SCAN, vec![0x00]);
}