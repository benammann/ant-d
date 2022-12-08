mod constants;

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

pub fn system_reset_message() -> Message {
    return build_message(constants::MESSAGE_SYSTEM_RESET, vec![0x00])
}