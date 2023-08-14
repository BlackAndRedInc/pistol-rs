use anyhow::Result;
use pnet::packet::udp::{ipv4_checksum, MutableUdpPacket};
use std::net::Ipv4Addr;

use crate::utils::return_layer4_udp6_channel;
use crate::utils::UDP_BUFF_SIZE;
use crate::utils::UDP_DATA_LEN;
use crate::utils::UDP_HEADER_LEN;

pub fn send_udp_flood_packet(
    src_ipv4: Ipv4Addr,
    src_port: u16,
    dst_ipv4: Ipv4Addr,
    dst_port: u16,
    max_same_packet: usize,
) -> Result<()> {
    let (mut udp_tx, _) = return_layer4_udp6_channel(UDP_BUFF_SIZE)?;

    // udp header
    let mut udp_buff = [0u8; UDP_HEADER_LEN + UDP_DATA_LEN];
    let mut udp_header = MutableUdpPacket::new(&mut udp_buff[..]).unwrap();
    udp_header.set_source(src_port);
    udp_header.set_destination(dst_port);
    udp_header.set_length((UDP_HEADER_LEN + UDP_DATA_LEN) as u16);
    let checksum = ipv4_checksum(&udp_header.to_immutable(), &src_ipv4, &dst_ipv4);
    udp_header.set_checksum(checksum);

    for _ in 0..max_same_packet {
        match udp_tx.send_to(&udp_header, dst_ipv4.into()) {
            _ => (),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_udp_flood_packet() {
        let src_ipv4 = Ipv4Addr::new(192, 168, 213, 129);
        let dst_ipv4 = Ipv4Addr::new(192, 168, 213, 128);
        let src_port = 57831;
        let dst_port = 80;
        let ret = send_udp_flood_packet(src_ipv4, src_port, dst_ipv4, dst_port, 1).unwrap();
        println!("{:?}", ret);
    }
}
