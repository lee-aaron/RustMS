use crate::packet::op::SendOpcode::{GuestIdLogin, LoginStatus};
use packet::{io::write::PktWrite, Packet};
use std::time::SystemTime;

/// Build a login status packet that gets sent upon login failure, relaying the
/// reason.
pub fn build_login_status_packet(status: u8) -> Packet {
    // TODO: Need to create an enum for the status...
    let mut packet = Packet::new_empty();
    let opcode = LoginStatus as i16;

    packet.write_short(opcode).unwrap();
    packet.write_byte(status).unwrap();
    packet.write_byte(0).unwrap();
    packet.write_int(0).unwrap();

    packet
}

pub fn build_successful_login_packet() -> Packet {
    let mut packet = Packet::new_empty();
    let opcode = LoginStatus as i16;

    let account_id = 1;
    let gender = 0;
    let account_name = "neeerp";

    packet.write_short(opcode).unwrap();
    packet.write_int(0).unwrap();
    packet.write_short(opcode).unwrap();
    packet.write_int(account_id).unwrap();
    packet.write_byte(gender).unwrap();

    packet.write_byte(0).unwrap();
    packet.write_byte(0).unwrap();
    packet.write_byte(0).unwrap();

    packet.write_str_with_length(account_name).unwrap();
    packet.write_byte(0).unwrap();

    packet.write_byte(0).unwrap();
    packet.write_long(0).unwrap();
    packet.write_long(0).unwrap();

    packet.write_int(1).unwrap();

    packet.write_byte(1).unwrap();
    packet.write_byte(1).unwrap();

    return packet;
}

pub fn build_guest_login_packet() -> Packet {
    let mut packet = Packet::new_empty();
    let opcode = GuestIdLogin as i16;

    let now: i64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    packet.write_short(opcode).unwrap();
    packet.write_short(0x100).unwrap();
    packet.write_int(0).unwrap(); // TODO: Should be random
    packet.write_long(0).unwrap();
    packet.write_long(0).unwrap();
    packet.write_long(now).unwrap();
    packet.write_int(0).unwrap();
    packet
        .write_str_with_length("https://github.com/neeerp")
        .unwrap();

    packet
}