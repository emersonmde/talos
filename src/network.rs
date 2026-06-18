//! Host-testable network packet boundary primitives.
//!
//! This module deliberately separates device-owned packet movement from
//! protocol parsing. Drivers can implement NetworkDevice to move raw frames;
//! the parser functions consume immutable byte slices and make no hardware,
//! allocation, DMA, interrupt, socket, or SSH claims.

pub(crate) const ETHERNET_HEADER_LEN: usize = 14;
pub(crate) const ETHERNET_ADDR_LEN: usize = 6;
pub(crate) const ARP_ETHERNET_IPV4_LEN: usize = 28;
pub(crate) const IPV4_MIN_HEADER_LEN: usize = 20;

pub(crate) const ETHERTYPE_IPV4: u16 = 0x0800;
pub(crate) const ETHERTYPE_ARP: u16 = 0x0806;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DeviceError {
    WouldBlock,
    BufferTooSmall,
    Io,
}

pub(crate) trait NetworkDevice {
    fn receive_frame<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a [u8], DeviceError>;
    fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PacketError {
    Truncated,
    UnsupportedArpHardware,
    UnsupportedArpProtocol,
    InvalidArpHardwareLength,
    InvalidArpProtocolLength,
    InvalidIpv4Version,
    InvalidIpv4HeaderLength,
    InvalidIpv4TotalLength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MacAddress([u8; ETHERNET_ADDR_LEN]);

impl MacAddress {
    pub(crate) const fn new(bytes: [u8; ETHERNET_ADDR_LEN]) -> Self {
        Self(bytes)
    }

    pub(crate) const fn bytes(self) -> [u8; ETHERNET_ADDR_LEN] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EtherType {
    Ipv4,
    Arp,
    Other(u16),
}

impl EtherType {
    pub(crate) const fn from_raw(raw: u16) -> Self {
        match raw {
            ETHERTYPE_IPV4 => Self::Ipv4,
            ETHERTYPE_ARP => Self::Arp,
            value => Self::Other(value),
        }
    }

    pub(crate) const fn raw(self) -> u16 {
        match self {
            Self::Ipv4 => ETHERTYPE_IPV4,
            Self::Arp => ETHERTYPE_ARP,
            Self::Other(value) => value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EthernetFrame<'a> {
    destination: MacAddress,
    source: MacAddress,
    ether_type: EtherType,
    payload: &'a [u8],
}

impl<'a> EthernetFrame<'a> {
    pub(crate) fn parse(bytes: &'a [u8]) -> Result<Self, PacketError> {
        if bytes.len() < ETHERNET_HEADER_LEN {
            return Err(PacketError::Truncated);
        }

        Ok(Self {
            destination: mac_address_at(bytes, 0),
            source: mac_address_at(bytes, 6),
            ether_type: EtherType::from_raw(read_be_u16(bytes, 12)),
            payload: &bytes[ETHERNET_HEADER_LEN..],
        })
    }

    pub(crate) const fn destination(self) -> MacAddress {
        self.destination
    }

    pub(crate) const fn source(self) -> MacAddress {
        self.source
    }

    pub(crate) const fn ether_type(self) -> EtherType {
        self.ether_type
    }

    pub(crate) const fn payload(self) -> &'a [u8] {
        self.payload
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReceivedFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> ReceivedFrame<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub(crate) const fn bytes(self) -> &'a [u8] {
        self.bytes
    }

    pub(crate) fn ethernet(self) -> Result<EthernetFrame<'a>, PacketError> {
        EthernetFrame::parse(self.bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArpOperation {
    Request,
    Reply,
    Other(u16),
}

impl ArpOperation {
    pub(crate) const fn from_raw(raw: u16) -> Self {
        match raw {
            1 => Self::Request,
            2 => Self::Reply,
            value => Self::Other(value),
        }
    }

    pub(crate) const fn raw(self) -> u16 {
        match self {
            Self::Request => 1,
            Self::Reply => 2,
            Self::Other(value) => value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ArpPacket {
    operation: ArpOperation,
    sender_hardware_address: MacAddress,
    sender_protocol_address: [u8; 4],
    target_hardware_address: MacAddress,
    target_protocol_address: [u8; 4],
}

impl ArpPacket {
    pub(crate) fn parse_ethernet_ipv4(bytes: &[u8]) -> Result<Self, PacketError> {
        if bytes.len() < ARP_ETHERNET_IPV4_LEN {
            return Err(PacketError::Truncated);
        }

        let hardware_type = read_be_u16(bytes, 0);
        if hardware_type != 1 {
            return Err(PacketError::UnsupportedArpHardware);
        }

        let protocol_type = read_be_u16(bytes, 2);
        if protocol_type != ETHERTYPE_IPV4 {
            return Err(PacketError::UnsupportedArpProtocol);
        }

        if bytes[4] != ETHERNET_ADDR_LEN as u8 {
            return Err(PacketError::InvalidArpHardwareLength);
        }

        if bytes[5] != 4 {
            return Err(PacketError::InvalidArpProtocolLength);
        }

        Ok(Self {
            operation: ArpOperation::from_raw(read_be_u16(bytes, 6)),
            sender_hardware_address: mac_address_at(bytes, 8),
            sender_protocol_address: ipv4_address_at(bytes, 14),
            target_hardware_address: mac_address_at(bytes, 18),
            target_protocol_address: ipv4_address_at(bytes, 24),
        })
    }

    pub(crate) const fn operation(self) -> ArpOperation {
        self.operation
    }

    pub(crate) const fn sender_hardware_address(self) -> MacAddress {
        self.sender_hardware_address
    }

    pub(crate) const fn sender_protocol_address(self) -> [u8; 4] {
        self.sender_protocol_address
    }

    pub(crate) const fn target_hardware_address(self) -> MacAddress {
        self.target_hardware_address
    }

    pub(crate) const fn target_protocol_address(self) -> [u8; 4] {
        self.target_protocol_address
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Ipv4Packet<'a> {
    header_len: usize,
    total_len: usize,
    protocol: u8,
    source: [u8; 4],
    destination: [u8; 4],
    payload: &'a [u8],
}

impl<'a> Ipv4Packet<'a> {
    pub(crate) fn parse(bytes: &'a [u8]) -> Result<Self, PacketError> {
        if bytes.len() < IPV4_MIN_HEADER_LEN {
            return Err(PacketError::Truncated);
        }

        let version = bytes[0] >> 4;
        if version != 4 {
            return Err(PacketError::InvalidIpv4Version);
        }

        let ihl_words = bytes[0] & 0x0f;
        if ihl_words < 5 {
            return Err(PacketError::InvalidIpv4HeaderLength);
        }
        let header_len = ihl_words as usize * 4;
        if bytes.len() < header_len {
            return Err(PacketError::Truncated);
        }

        let total_len = read_be_u16(bytes, 2) as usize;
        if total_len < header_len {
            return Err(PacketError::InvalidIpv4TotalLength);
        }
        if bytes.len() < total_len {
            return Err(PacketError::Truncated);
        }

        Ok(Self {
            header_len,
            total_len,
            protocol: bytes[9],
            source: ipv4_address_at(bytes, 12),
            destination: ipv4_address_at(bytes, 16),
            payload: &bytes[header_len..total_len],
        })
    }

    pub(crate) const fn header_len(self) -> usize {
        self.header_len
    }

    pub(crate) const fn total_len(self) -> usize {
        self.total_len
    }

    pub(crate) const fn protocol(self) -> u8 {
        self.protocol
    }

    pub(crate) const fn source(self) -> [u8; 4] {
        self.source
    }

    pub(crate) const fn destination(self) -> [u8; 4] {
        self.destination
    }

    pub(crate) const fn payload(self) -> &'a [u8] {
        self.payload
    }
}

fn read_be_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn mac_address_at(bytes: &[u8], offset: usize) -> MacAddress {
    MacAddress::new([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
    ])
}

fn ipv4_address_at(bytes: &[u8], offset: usize) -> [u8; 4] {
    [
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ETHERNET_IPV4_FRAME: &[u8] = &[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x45,
        0x00, 0x00, 0x18, 0x12, 0x34, 0x40, 0x00, 0x40, 0x11, 0x00, 0x00, 192, 0, 2, 1, 192, 0, 2,
        2, 0xde, 0xad, 0xbe, 0xef,
    ];

    const ARP_REQUEST: &[u8] = &[
        0x00, 0x01, 0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01, 192, 0,
        2, 10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 192, 0, 2, 1,
    ];

    #[test_case]
    fn ethernet_parser_splits_header_and_payload() {
        let frame = EthernetFrame::parse(ETHERNET_IPV4_FRAME).expect("parse ethernet frame");

        assert_eq!(frame.destination().bytes(), [0xff; 6]);
        assert_eq!(frame.source().bytes(), [0x02, 0, 0, 0, 0, 1]);
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
        assert_eq!(frame.ether_type().raw(), ETHERTYPE_IPV4);
        assert_eq!(frame.payload().len(), 24);
    }

    #[test_case]
    fn ethernet_parser_rejects_truncated_headers() {
        assert_eq!(
            EthernetFrame::parse(&ETHERNET_IPV4_FRAME[..ETHERNET_HEADER_LEN - 1]),
            Err(PacketError::Truncated)
        );
    }

    #[test_case]
    fn received_frame_keeps_device_movement_separate_from_protocol_parse() {
        let received = ReceivedFrame::new(ETHERNET_IPV4_FRAME);
        let frame = received.ethernet().expect("parse received ethernet frame");

        assert_eq!(received.bytes(), ETHERNET_IPV4_FRAME);
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
    }

    struct SliceDevice<'a> {
        frame: &'a [u8],
        transmitted_len: usize,
    }

    impl<'a> NetworkDevice for SliceDevice<'a> {
        fn receive_frame<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8], DeviceError> {
            if buffer.len() < self.frame.len() {
                return Err(DeviceError::BufferTooSmall);
            }

            buffer[..self.frame.len()].copy_from_slice(self.frame);
            Ok(&buffer[..self.frame.len()])
        }

        fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError> {
            if frame.is_empty() {
                return Err(DeviceError::WouldBlock);
            }
            if frame[0] == 0xff {
                return Err(DeviceError::Io);
            }

            self.transmitted_len = frame.len();
            Ok(())
        }
    }

    #[test_case]
    fn network_device_trait_moves_raw_frames_without_parsing() {
        let mut device = SliceDevice {
            frame: ETHERNET_IPV4_FRAME,
            transmitted_len: 0,
        };
        let mut buffer = [0u8; 64];

        let received = device.receive_frame(&mut buffer).expect("receive frame");
        assert_eq!(received, ETHERNET_IPV4_FRAME);
        assert_eq!(
            EthernetFrame::parse(received)
                .expect("parse received frame")
                .ether_type(),
            EtherType::Ipv4
        );

        assert_eq!(
            device.receive_frame(&mut buffer[..ETHERNET_HEADER_LEN - 1]),
            Err(DeviceError::BufferTooSmall)
        );
        assert_eq!(device.transmit_frame(&[1, 2, 3]), Ok(()));
        assert_eq!(device.transmitted_len, 3);
        assert_eq!(device.transmit_frame(&[]), Err(DeviceError::WouldBlock));
        assert_eq!(device.transmit_frame(&[0xff]), Err(DeviceError::Io));
    }

    #[test_case]
    fn arp_parser_accepts_ethernet_ipv4_request() {
        let packet = ArpPacket::parse_ethernet_ipv4(ARP_REQUEST).expect("parse arp");

        assert_eq!(packet.operation(), ArpOperation::Request);
        assert_eq!(packet.operation().raw(), 1);
        assert_eq!(
            packet.sender_hardware_address().bytes(),
            [0x02, 0, 0, 0, 0, 1]
        );
        assert_eq!(packet.sender_protocol_address(), [192, 0, 2, 10]);
        assert_eq!(packet.target_hardware_address().bytes(), [0; 6]);
        assert_eq!(packet.target_protocol_address(), [192, 0, 2, 1]);
    }

    #[test_case]
    fn arp_parser_rejects_truncated_and_mismatched_shapes() {
        assert_eq!(
            ArpPacket::parse_ethernet_ipv4(&ARP_REQUEST[..ARP_ETHERNET_IPV4_LEN - 1]),
            Err(PacketError::Truncated)
        );

        let mut bad_hardware = [0u8; ARP_ETHERNET_IPV4_LEN];
        bad_hardware.copy_from_slice(ARP_REQUEST);
        bad_hardware[1] = 0x02;
        assert_eq!(
            ArpPacket::parse_ethernet_ipv4(&bad_hardware),
            Err(PacketError::UnsupportedArpHardware)
        );

        let mut bad_protocol = [0u8; ARP_ETHERNET_IPV4_LEN];
        bad_protocol.copy_from_slice(ARP_REQUEST);
        bad_protocol[2] = 0x86;
        bad_protocol[3] = 0xdd;
        assert_eq!(
            ArpPacket::parse_ethernet_ipv4(&bad_protocol),
            Err(PacketError::UnsupportedArpProtocol)
        );

        let mut bad_hlen = [0u8; ARP_ETHERNET_IPV4_LEN];
        bad_hlen.copy_from_slice(ARP_REQUEST);
        bad_hlen[4] = 5;
        assert_eq!(
            ArpPacket::parse_ethernet_ipv4(&bad_hlen),
            Err(PacketError::InvalidArpHardwareLength)
        );

        let mut bad_plen = [0u8; ARP_ETHERNET_IPV4_LEN];
        bad_plen.copy_from_slice(ARP_REQUEST);
        bad_plen[5] = 16;
        assert_eq!(
            ArpPacket::parse_ethernet_ipv4(&bad_plen),
            Err(PacketError::InvalidArpProtocolLength)
        );
    }

    #[test_case]
    fn ipv4_parser_accepts_header_and_payload() {
        let frame = EthernetFrame::parse(ETHERNET_IPV4_FRAME).expect("parse ethernet frame");
        let packet = Ipv4Packet::parse(frame.payload()).expect("parse ipv4");

        assert_eq!(packet.header_len(), IPV4_MIN_HEADER_LEN);
        assert_eq!(packet.total_len(), 24);
        assert_eq!(packet.protocol(), 17);
        assert_eq!(packet.source(), [192, 0, 2, 1]);
        assert_eq!(packet.destination(), [192, 0, 2, 2]);
        assert_eq!(packet.payload(), [0xde, 0xad, 0xbe, 0xef]);
    }

    #[test_case]
    fn ipv4_parser_accepts_options_when_total_length_matches() {
        let bytes = [
            0x46, 0x00, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 64, 1, 0x00, 0x00, 10, 0, 0, 1, 10, 0,
            0, 2, 1, 2, 3, 4,
        ];
        let packet = Ipv4Packet::parse(&bytes).expect("parse ipv4 options");

        assert_eq!(packet.header_len(), 24);
        assert_eq!(packet.total_len(), 24);
        assert_eq!(packet.payload(), []);
    }

    #[test_case]
    fn ipv4_parser_rejects_malformed_headers() {
        assert_eq!(
            Ipv4Packet::parse(
                &ETHERNET_IPV4_FRAME[ETHERNET_HEADER_LEN..][..IPV4_MIN_HEADER_LEN - 1]
            ),
            Err(PacketError::Truncated)
        );

        let mut bad_version = [0u8; 20];
        bad_version.copy_from_slice(&ETHERNET_IPV4_FRAME[ETHERNET_HEADER_LEN..][..20]);
        bad_version[0] = 0x65;
        assert_eq!(
            Ipv4Packet::parse(&bad_version),
            Err(PacketError::InvalidIpv4Version)
        );

        let mut bad_ihl = [0u8; 20];
        bad_ihl.copy_from_slice(&ETHERNET_IPV4_FRAME[ETHERNET_HEADER_LEN..][..20]);
        bad_ihl[0] = 0x44;
        assert_eq!(
            Ipv4Packet::parse(&bad_ihl),
            Err(PacketError::InvalidIpv4HeaderLength)
        );

        let mut short_total = [0u8; 20];
        short_total.copy_from_slice(&ETHERNET_IPV4_FRAME[ETHERNET_HEADER_LEN..][..20]);
        short_total[2] = 0;
        short_total[3] = 19;
        assert_eq!(
            Ipv4Packet::parse(&short_total),
            Err(PacketError::InvalidIpv4TotalLength)
        );

        let mut truncated_total = [0u8; 20];
        truncated_total.copy_from_slice(&ETHERNET_IPV4_FRAME[ETHERNET_HEADER_LEN..][..20]);
        truncated_total[2] = 0;
        truncated_total[3] = 21;
        assert_eq!(
            Ipv4Packet::parse(&truncated_total),
            Err(PacketError::Truncated)
        );
    }
}
