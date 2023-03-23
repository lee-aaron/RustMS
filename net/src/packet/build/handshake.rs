use crate::error::NetworkError;
use packet::{io::write::PktWrite, Packet};

/// Build the handshake_packet which shares the encryption IVs with the client.
pub fn build_handshake_packet(
    recv_iv: &Vec<u8>,
    send_iv: &Vec<u8>,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();

    packet.write_short(0x0E)?; // Packet length header
    packet.write_short(83)?; // Version
    packet.write_short(1)?;
    packet.write_byte(49)?;
    packet.write_bytes(&recv_iv)?;
    packet.write_bytes(&send_iv)?;
    packet.write_byte(8)?; // Locale byte

    Ok(packet)
}
