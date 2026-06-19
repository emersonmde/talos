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
pub(crate) const ICMP_ECHO_HEADER_LEN: usize = 8;

pub(crate) const ETHERTYPE_IPV4: u16 = 0x0800;
pub(crate) const ETHERTYPE_ARP: u16 = 0x0806;
pub(crate) const IPV4_PROTOCOL_ICMP: u8 = 1;

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
    UnsupportedArpOperation,
    InvalidIpv4Version,
    InvalidIpv4HeaderLength,
    InvalidIpv4TotalLength,
    UnsupportedEtherType,
    UnsupportedIpv4Protocol,
    UnsupportedIpv4Options,
    UnsupportedIpv4Fragment,
    InvalidIpv4Checksum,
    InvalidIcmpEcho,
    InvalidIcmpChecksum,
    NotForLocalHost,
    OutputBufferTooSmall,
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
pub(crate) struct ArpNeighbor {
    ipv4: [u8; 4],
    mac: MacAddress,
}

impl ArpNeighbor {
    pub(crate) const fn new(ipv4: [u8; 4], mac: MacAddress) -> Self {
        Self { ipv4, mac }
    }

    pub(crate) const fn ipv4(self) -> [u8; 4] {
        self.ipv4
    }

    pub(crate) const fn mac(self) -> MacAddress {
        self.mac
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArpCacheUpdate {
    Inserted,
    Updated,
    Replaced(ArpNeighbor),
    NoCapacity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ArpCache<const CAPACITY: usize> {
    entries: [Option<ArpNeighbor>; CAPACITY],
    next_replacement: usize,
}

impl<const CAPACITY: usize> ArpCache<CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            next_replacement: 0,
        }
    }

    pub(crate) fn lookup(&self, ipv4: [u8; 4]) -> Option<MacAddress> {
        self.entries
            .iter()
            .flatten()
            .find(|neighbor| neighbor.ipv4() == ipv4)
            .map(|neighbor| neighbor.mac())
    }

    pub(crate) fn insert_or_update(&mut self, ipv4: [u8; 4], mac: MacAddress) -> ArpCacheUpdate {
        let neighbor = ArpNeighbor::new(ipv4, mac);

        if let Some(entry) = self
            .entries
            .iter_mut()
            .flatten()
            .find(|entry| entry.ipv4() == ipv4)
        {
            *entry = neighbor;
            return ArpCacheUpdate::Updated;
        }

        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.is_none()) {
            *entry = Some(neighbor);
            return ArpCacheUpdate::Inserted;
        }

        if CAPACITY == 0 {
            return ArpCacheUpdate::NoCapacity;
        }

        let replaced = self.entries[self.next_replacement].replace(neighbor);
        self.next_replacement = (self.next_replacement + 1) % CAPACITY;

        match replaced {
            Some(replaced) => ArpCacheUpdate::Replaced(replaced),
            None => ArpCacheUpdate::Inserted,
        }
    }

    pub(crate) fn learn_ethernet_ipv4_arp(
        &mut self,
        frame_bytes: &[u8],
    ) -> Result<ArpCacheUpdate, PacketError> {
        let frame = EthernetFrame::parse(frame_bytes)?;
        if frame.ether_type() != EtherType::Arp {
            return Err(PacketError::UnsupportedEtherType);
        }

        let packet = ArpPacket::parse_ethernet_ipv4(frame.payload())?;
        match packet.operation() {
            ArpOperation::Request | ArpOperation::Reply => Ok(self.insert_or_update(
                packet.sender_protocol_address(),
                packet.sender_hardware_address(),
            )),
            ArpOperation::Other(_) => Err(PacketError::UnsupportedArpOperation),
        }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LocalNetworkEndpoint {
    mac: MacAddress,
    ipv4: [u8; 4],
}

impl LocalNetworkEndpoint {
    pub(crate) const fn new(mac: MacAddress, ipv4: [u8; 4]) -> Self {
        Self { mac, ipv4 }
    }

    pub(crate) const fn mac(self) -> MacAddress {
        self.mac
    }

    pub(crate) const fn ipv4(self) -> [u8; 4] {
        self.ipv4
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PacketReplyKind {
    Arp,
    IcmpEcho,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PacketDispatchResult {
    reply_kind: PacketReplyKind,
    frame_len: usize,
}

impl PacketDispatchResult {
    pub(crate) const fn reply_kind(self) -> PacketReplyKind {
        self.reply_kind
    }

    pub(crate) const fn frame_len(self) -> usize {
        self.frame_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalPollStepResult {
    NoFrame,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
    NoReply,
    DispatchError(PacketError),
    TransmitError(DeviceError),
    Replied(PacketDispatchResult),
}

pub(crate) fn poll_local_network_device<D: NetworkDevice>(
    device: &mut D,
    endpoint: LocalNetworkEndpoint,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
) -> LocalPollStepResult {
    let received = match device.receive_frame(receive_buffer) {
        Ok(frame) => frame,
        Err(DeviceError::WouldBlock) => return LocalPollStepResult::NoFrame,
        Err(DeviceError::BufferTooSmall) => return LocalPollStepResult::ReceiveBufferTooSmall,
        Err(error) => return LocalPollStepResult::ReceiveError(error),
    };

    let reply = match dispatch_local_packet(received, endpoint, transmit_buffer) {
        Ok(reply) => reply,
        Err(PacketError::NotForLocalHost) => return LocalPollStepResult::NoReply,
        Err(error) => return LocalPollStepResult::DispatchError(error),
    };

    let frame_len = reply.frame_len();
    match device.transmit_frame(&transmit_buffer[..frame_len]) {
        Ok(()) => LocalPollStepResult::Replied(reply),
        Err(error) => LocalPollStepResult::TransmitError(error),
    }
}

pub(crate) fn poll_local_network_device_with_arp_cache<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
>(
    device: &mut D,
    endpoint: LocalNetworkEndpoint,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
) -> LocalPollStepResult {
    let received = match device.receive_frame(receive_buffer) {
        Ok(frame) => frame,
        Err(DeviceError::WouldBlock) => return LocalPollStepResult::NoFrame,
        Err(DeviceError::BufferTooSmall) => return LocalPollStepResult::ReceiveBufferTooSmall,
        Err(error) => return LocalPollStepResult::ReceiveError(error),
    };

    let reply = match dispatch_local_packet_with_arp_cache(
        received,
        endpoint,
        arp_cache,
        transmit_buffer,
    ) {
        Ok(reply) => reply,
        Err(PacketError::NotForLocalHost) => return LocalPollStepResult::NoReply,
        Err(error) => return LocalPollStepResult::DispatchError(error),
    };

    let frame_len = reply.frame_len();
    match device.transmit_frame(&transmit_buffer[..frame_len]) {
        Ok(()) => LocalPollStepResult::Replied(reply),
        Err(error) => LocalPollStepResult::TransmitError(error),
    }
}

pub(crate) fn dispatch_local_packet(
    frame_bytes: &[u8],
    endpoint: LocalNetworkEndpoint,
    output: &mut [u8],
) -> Result<PacketDispatchResult, PacketError> {
    let frame = EthernetFrame::parse(frame_bytes)?;

    match frame.ether_type() {
        EtherType::Arp => build_arp_reply(frame, endpoint, output),
        EtherType::Ipv4 => build_icmp_echo_reply(frame, endpoint, output),
        EtherType::Other(_) => Err(PacketError::UnsupportedEtherType),
    }
}

pub(crate) fn dispatch_local_packet_with_arp_cache<const ARP_CAPACITY: usize>(
    frame_bytes: &[u8],
    endpoint: LocalNetworkEndpoint,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    output: &mut [u8],
) -> Result<PacketDispatchResult, PacketError> {
    learn_arp_sender_if_present(frame_bytes, arp_cache)?;
    dispatch_local_packet(frame_bytes, endpoint, output)
}

fn learn_arp_sender_if_present<const ARP_CAPACITY: usize>(
    frame_bytes: &[u8],
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
) -> Result<(), PacketError> {
    let frame = EthernetFrame::parse(frame_bytes)?;
    if frame.ether_type() != EtherType::Arp {
        return Ok(());
    }

    let packet = ArpPacket::parse_ethernet_ipv4(frame.payload())?;
    match packet.operation() {
        ArpOperation::Request | ArpOperation::Reply => {
            let _ = arp_cache.insert_or_update(
                packet.sender_protocol_address(),
                packet.sender_hardware_address(),
            );
            Ok(())
        }
        ArpOperation::Other(_) => Err(PacketError::UnsupportedArpOperation),
    }
}

fn build_arp_reply(
    frame: EthernetFrame<'_>,
    endpoint: LocalNetworkEndpoint,
    output: &mut [u8],
) -> Result<PacketDispatchResult, PacketError> {
    let packet = ArpPacket::parse_ethernet_ipv4(frame.payload())?;

    if packet.operation() != ArpOperation::Request {
        return Err(PacketError::UnsupportedArpOperation);
    }
    if !mac_is_local_or_broadcast(frame.destination(), endpoint.mac()) {
        return Err(PacketError::NotForLocalHost);
    }
    if packet.target_protocol_address() != endpoint.ipv4() {
        return Err(PacketError::NotForLocalHost);
    }

    let reply_len = ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN;
    if output.len() < reply_len {
        return Err(PacketError::OutputBufferTooSmall);
    }

    output[..ETHERNET_ADDR_LEN].copy_from_slice(&frame.source().bytes());
    output[ETHERNET_ADDR_LEN..ETHERNET_ADDR_LEN * 2].copy_from_slice(&endpoint.mac().bytes());
    write_be_u16(output, 12, ETHERTYPE_ARP);

    let arp = &mut output[ETHERNET_HEADER_LEN..reply_len];
    write_be_u16(arp, 0, 1);
    write_be_u16(arp, 2, ETHERTYPE_IPV4);
    arp[4] = ETHERNET_ADDR_LEN as u8;
    arp[5] = 4;
    write_be_u16(arp, 6, ArpOperation::Reply.raw());
    arp[8..14].copy_from_slice(&endpoint.mac().bytes());
    arp[14..18].copy_from_slice(&endpoint.ipv4());
    arp[18..24].copy_from_slice(&packet.sender_hardware_address().bytes());
    arp[24..28].copy_from_slice(&packet.sender_protocol_address());

    Ok(PacketDispatchResult {
        reply_kind: PacketReplyKind::Arp,
        frame_len: reply_len,
    })
}

fn build_icmp_echo_reply(
    frame: EthernetFrame<'_>,
    endpoint: LocalNetworkEndpoint,
    output: &mut [u8],
) -> Result<PacketDispatchResult, PacketError> {
    let ipv4_bytes = frame.payload();
    let packet = Ipv4Packet::parse(ipv4_bytes)?;

    if packet.header_len() != IPV4_MIN_HEADER_LEN {
        return Err(PacketError::UnsupportedIpv4Options);
    }
    if ipv4_fragment_field(ipv4_bytes) != 0 {
        return Err(PacketError::UnsupportedIpv4Fragment);
    }
    if !ipv4_header_checksum_is_valid(&ipv4_bytes[..packet.header_len()]) {
        return Err(PacketError::InvalidIpv4Checksum);
    }
    if packet.protocol() != IPV4_PROTOCOL_ICMP {
        return Err(PacketError::UnsupportedIpv4Protocol);
    }
    if frame.destination() != endpoint.mac() {
        return Err(PacketError::NotForLocalHost);
    }
    if packet.destination() != endpoint.ipv4() {
        return Err(PacketError::NotForLocalHost);
    }

    let icmp_request = packet.payload();
    if icmp_request.len() < ICMP_ECHO_HEADER_LEN || icmp_request[0] != 8 || icmp_request[1] != 0 {
        return Err(PacketError::InvalidIcmpEcho);
    }
    if !internet_checksum_is_valid(icmp_request) {
        return Err(PacketError::InvalidIcmpChecksum);
    }

    let reply_len = ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + icmp_request.len();
    if output.len() < reply_len {
        return Err(PacketError::OutputBufferTooSmall);
    }

    output[..ETHERNET_ADDR_LEN].copy_from_slice(&frame.source().bytes());
    output[ETHERNET_ADDR_LEN..ETHERNET_ADDR_LEN * 2].copy_from_slice(&endpoint.mac().bytes());
    write_be_u16(output, 12, ETHERTYPE_IPV4);

    let ipv4_reply = &mut output[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
    ipv4_reply[0] = 0x45;
    ipv4_reply[1] = ipv4_bytes[1];
    write_be_u16(
        ipv4_reply,
        2,
        (IPV4_MIN_HEADER_LEN + icmp_request.len()) as u16,
    );
    ipv4_reply[4] = ipv4_bytes[4];
    ipv4_reply[5] = ipv4_bytes[5];
    write_be_u16(ipv4_reply, 6, 0);
    ipv4_reply[8] = 64;
    ipv4_reply[9] = IPV4_PROTOCOL_ICMP;
    write_be_u16(ipv4_reply, 10, 0);
    ipv4_reply[12..16].copy_from_slice(&endpoint.ipv4());
    ipv4_reply[16..20].copy_from_slice(&packet.source());
    let ipv4_checksum = internet_checksum(ipv4_reply);
    write_be_u16(ipv4_reply, 10, ipv4_checksum);

    let icmp_reply = &mut output[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..reply_len];
    icmp_reply.copy_from_slice(icmp_request);
    icmp_reply[0] = 0;
    icmp_reply[1] = 0;
    write_be_u16(icmp_reply, 2, 0);
    let icmp_checksum = internet_checksum(icmp_reply);
    write_be_u16(icmp_reply, 2, icmp_checksum);

    Ok(PacketDispatchResult {
        reply_kind: PacketReplyKind::IcmpEcho,
        frame_len: reply_len,
    })
}

fn read_be_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn write_be_u16(bytes: &mut [u8], offset: usize, value: u16) {
    let raw = value.to_be_bytes();
    bytes[offset] = raw[0];
    bytes[offset + 1] = raw[1];
}

fn ipv4_fragment_field(bytes: &[u8]) -> u16 {
    read_be_u16(bytes, 6) & 0x3fff
}

fn ipv4_header_checksum_is_valid(header: &[u8]) -> bool {
    internet_checksum_is_valid(header)
}

fn internet_checksum_is_valid(bytes: &[u8]) -> bool {
    ones_complement_sum(bytes) == 0xffff
}

fn internet_checksum(bytes: &[u8]) -> u16 {
    !(ones_complement_sum(bytes))
}

fn mac_is_local_or_broadcast(candidate: MacAddress, local: MacAddress) -> bool {
    candidate == local || candidate.bytes() == [0xff; ETHERNET_ADDR_LEN]
}

fn ones_complement_sum(bytes: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut index = 0;
    while index + 1 < bytes.len() {
        sum += read_be_u16(bytes, index) as u32;
        index += 2;
    }
    if index < bytes.len() {
        sum += (bytes[index] as u32) << 8;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    sum as u16
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
    fn arp_cache_inserts_updates_misses_and_replaces_oldest_slots() {
        let mut cache = ArpCache::<2>::new();
        let mac_a = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mac_b = MacAddress::new([0x02, 0, 0, 0, 0, 11]);
        let mac_c = MacAddress::new([0x02, 0, 0, 0, 0, 12]);
        let mac_d = MacAddress::new([0x02, 0, 0, 0, 0, 13]);

        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], mac_a),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(mac_a));
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], mac_b),
            ArpCacheUpdate::Updated
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(mac_b));

        assert_eq!(
            cache.insert_or_update([192, 0, 2, 11], mac_c),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 12], mac_d),
            ArpCacheUpdate::Replaced(ArpNeighbor::new([192, 0, 2, 10], mac_b))
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
        assert_eq!(cache.lookup([192, 0, 2, 11]), Some(mac_c));
        assert_eq!(cache.lookup([192, 0, 2, 12]), Some(mac_d));
    }

    #[test_case]
    fn arp_cache_zero_capacity_reports_no_capacity_without_state() {
        let mut cache = ArpCache::<0>::new();

        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], MacAddress::new([0x02, 0, 0, 0, 0, 10])),
            ArpCacheUpdate::NoCapacity
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
    }

    #[test_case]
    fn arp_cache_learns_sender_from_valid_arp_requests_and_replies() {
        let mut cache = ArpCache::<4>::new();

        assert_eq!(
            cache.learn_ethernet_ipv4_arp(&arp_request_frame()),
            Ok(ArpCacheUpdate::Inserted)
        );
        assert_eq!(
            cache.lookup([192, 0, 2, 10]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 1]))
        );

        assert_eq!(
            cache.learn_ethernet_ipv4_arp(&arp_reply_frame()),
            Ok(ArpCacheUpdate::Inserted)
        );
        assert_eq!(
            cache.lookup([192, 0, 2, 20]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
    }

    #[test_case]
    fn arp_cache_rejects_malformed_or_unsupported_learning_without_state_change() {
        let mut cache = ArpCache::<2>::new();
        let original_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], original_mac),
            ArpCacheUpdate::Inserted
        );

        let truncated = &arp_request_frame()[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            cache.learn_ethernet_ipv4_arp(truncated),
            Err(PacketError::Truncated)
        );

        let mut unsupported_operation = arp_request_frame();
        write_be_u16(
            &mut unsupported_operation[ETHERNET_HEADER_LEN..],
            6,
            ArpOperation::Other(99).raw(),
        );
        unsupported_operation[ETHERNET_HEADER_LEN + 8..ETHERNET_HEADER_LEN + 14]
            .copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        assert_eq!(
            cache.learn_ethernet_ipv4_arp(&unsupported_operation),
            Err(PacketError::UnsupportedArpOperation)
        );

        let mut unsupported_ethertype = arp_request_frame();
        write_be_u16(&mut unsupported_ethertype, 12, ETHERTYPE_IPV4);
        assert_eq!(
            cache.learn_ethernet_ipv4_arp(&unsupported_ethertype),
            Err(PacketError::UnsupportedEtherType)
        );

        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(original_mac));
        assert_eq!(cache.lookup([192, 0, 2, 99]), None);
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

    #[test_case]
    fn dispatch_builds_arp_reply_for_local_ipv4_identity() {
        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1]);
        let request = arp_request_frame();
        let mut output = [0u8; 64];

        let result = dispatch_local_packet(&request, endpoint, &mut output).expect("dispatch arp");

        assert_eq!(result.reply_kind(), PacketReplyKind::Arp);
        assert_eq!(
            result.frame_len(),
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
        let reply = &output[..result.frame_len()];
        assert_eq!(&reply[0..6], &[0x02, 0, 0, 0, 0, 1]);
        assert_eq!(&reply[6..12], &[0x02, 0, 0, 0, 0, 99]);
        assert_eq!(read_be_u16(reply, 12), ETHERTYPE_ARP);
        assert_eq!(
            read_be_u16(reply, ETHERNET_HEADER_LEN + 6),
            ArpOperation::Reply.raw()
        );
        assert_eq!(
            &reply[ETHERNET_HEADER_LEN + 8..ETHERNET_HEADER_LEN + 14],
            &[0x02, 0, 0, 0, 0, 99]
        );
        assert_eq!(
            &reply[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18],
            &[192, 0, 2, 1]
        );
        assert_eq!(
            &reply[ETHERNET_HEADER_LEN + 18..ETHERNET_HEADER_LEN + 24],
            &[0x02, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            &reply[ETHERNET_HEADER_LEN + 24..ETHERNET_HEADER_LEN + 28],
            &[192, 0, 2, 10]
        );
    }

    #[test_case]
    fn dispatch_rejects_nonlocal_arp_and_small_arp_output_buffer() {
        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 99]);
        let request = arp_request_frame();
        let mut output = [0u8; 64];

        assert_eq!(
            dispatch_local_packet(&request, endpoint, &mut output),
            Err(PacketError::NotForLocalHost)
        );

        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1]);
        let mut wrong_mac = request;
        wrong_mac[0..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 88]);
        assert_eq!(
            dispatch_local_packet(&wrong_mac, endpoint, &mut output),
            Err(PacketError::NotForLocalHost)
        );

        let mut small = [0u8; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            dispatch_local_packet(&request, endpoint, &mut small),
            Err(PacketError::OutputBufferTooSmall)
        );
    }

    #[test_case]
    fn dispatch_builds_icmp_echo_reply_with_valid_ipv4_and_icmp_checksums() {
        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1]);
        let request = icmp_echo_request_frame();
        let mut output = [0u8; 128];

        let result = dispatch_local_packet(&request, endpoint, &mut output).expect("dispatch icmp");

        assert_eq!(result.reply_kind(), PacketReplyKind::IcmpEcho);
        let reply = &output[..result.frame_len()];
        assert_eq!(&reply[0..6], &[0x02, 0, 0, 0, 0, 1]);
        assert_eq!(&reply[6..12], &[0x02, 0, 0, 0, 0, 99]);
        assert_eq!(read_be_u16(reply, 12), ETHERTYPE_IPV4);

        let ipv4 = &reply[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        assert_eq!(read_be_u16(ipv4, 2), 32);
        assert_eq!(ipv4[9], IPV4_PROTOCOL_ICMP);
        assert_eq!(&ipv4[12..16], &[192, 0, 2, 1]);
        assert_eq!(&ipv4[16..20], &[192, 0, 2, 10]);
        assert!(ipv4_header_checksum_is_valid(ipv4));

        let icmp = &reply[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..result.frame_len()];
        assert_eq!(icmp[0], 0);
        assert_eq!(icmp[1], 0);
        assert_eq!(&icmp[4..], &[0x12, 0x34, 0, 7, 1, 2, 3, 4]);
        assert!(internet_checksum_is_valid(icmp));
    }

    #[test_case]
    fn dispatch_rejects_malformed_ipv4_icmp_protocols_and_fragments() {
        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1]);
        let mut output = [0u8; 128];

        let mut bad_ipv4_checksum = icmp_echo_request_frame();
        bad_ipv4_checksum[ETHERNET_HEADER_LEN + 10] ^= 0x01;
        assert_eq!(
            dispatch_local_packet(&bad_ipv4_checksum, endpoint, &mut output),
            Err(PacketError::InvalidIpv4Checksum)
        );

        let mut udp_like = icmp_echo_request_frame();
        udp_like[ETHERNET_HEADER_LEN + 9] = 17;
        rewrite_ipv4_checksum(&mut udp_like);
        assert_eq!(
            dispatch_local_packet(&udp_like, endpoint, &mut output),
            Err(PacketError::UnsupportedIpv4Protocol)
        );

        let mut wrong_mac = icmp_echo_request_frame();
        wrong_mac[0..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 88]);
        assert_eq!(
            dispatch_local_packet(&wrong_mac, endpoint, &mut output),
            Err(PacketError::NotForLocalHost)
        );

        let mut fragmented = icmp_echo_request_frame();
        fragmented[ETHERNET_HEADER_LEN + 6] = 0x20;
        rewrite_ipv4_checksum(&mut fragmented);
        assert_eq!(
            dispatch_local_packet(&fragmented, endpoint, &mut output),
            Err(PacketError::UnsupportedIpv4Fragment)
        );

        let mut options = icmp_echo_request_with_options_frame();
        assert_eq!(
            dispatch_local_packet(&options, endpoint, &mut output),
            Err(PacketError::UnsupportedIpv4Options)
        );

        options[ETHERNET_HEADER_LEN] = 0x45;
        rewrite_ipv4_checksum(&mut options);
        assert_eq!(
            dispatch_local_packet(&options, endpoint, &mut output),
            Err(PacketError::InvalidIcmpEcho)
        );
    }

    #[test_case]
    fn dispatch_rejects_bad_icmp_checksum_unsupported_ethertype_and_small_icmp_output() {
        let endpoint =
            LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1]);
        let mut output = [0u8; 128];

        let mut bad_icmp = icmp_echo_request_frame();
        bad_icmp[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 2] ^= 0x40;
        assert_eq!(
            dispatch_local_packet(&bad_icmp, endpoint, &mut output),
            Err(PacketError::InvalidIcmpChecksum)
        );

        let mut unsupported = icmp_echo_request_frame();
        write_be_u16(&mut unsupported, 12, 0x86dd);
        assert_eq!(
            dispatch_local_packet(&unsupported, endpoint, &mut output),
            Err(PacketError::UnsupportedEtherType)
        );

        let request = icmp_echo_request_frame();
        let mut small = [0u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN - 1];
        assert_eq!(
            dispatch_local_packet(&request, endpoint, &mut small),
            Err(PacketError::OutputBufferTooSmall)
        );
    }

    #[test_case]
    fn cache_aware_dispatch_learns_arp_request_and_preserves_reply_generation() {
        let endpoint = local_endpoint();
        let request = arp_request_frame();
        let mut cache = ArpCache::<4>::new();
        let mut output = [0u8; 64];

        let result =
            dispatch_local_packet_with_arp_cache(&request, endpoint, &mut cache, &mut output)
                .expect("dispatch arp with cache");

        assert_eq!(result.reply_kind(), PacketReplyKind::Arp);
        assert_eq!(
            cache.lookup([192, 0, 2, 10]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 1]))
        );
        assert_eq!(&output[..6], &[0x02, 0, 0, 0, 0, 1]);
        assert_eq!(&output[6..12], &[0x02, 0, 0, 0, 0, 99]);
    }

    #[test_case]
    fn cache_aware_dispatch_learns_arp_reply_without_generating_a_reply() {
        let endpoint = local_endpoint();
        let reply = arp_reply_frame();
        let mut cache = ArpCache::<4>::new();
        let mut output = [0u8; 64];

        assert_eq!(
            dispatch_local_packet_with_arp_cache(&reply, endpoint, &mut cache, &mut output),
            Err(PacketError::UnsupportedArpOperation)
        );
        assert_eq!(
            cache.lookup([192, 0, 2, 20]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
    }

    #[test_case]
    fn cache_aware_dispatch_rejects_bad_arp_without_cache_mutation() {
        let endpoint = local_endpoint();
        let original_mac = MacAddress::new([0x02, 0, 0, 0, 0, 55]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 55], original_mac),
            ArpCacheUpdate::Inserted
        );
        let mut output = [0u8; 64];

        let truncated = &arp_request_frame()[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            dispatch_local_packet_with_arp_cache(truncated, endpoint, &mut cache, &mut output),
            Err(PacketError::Truncated)
        );

        let mut unsupported_operation = arp_request_frame();
        write_be_u16(
            &mut unsupported_operation[ETHERNET_HEADER_LEN..],
            6,
            ArpOperation::Other(99).raw(),
        );
        unsupported_operation[ETHERNET_HEADER_LEN + 8..ETHERNET_HEADER_LEN + 14]
            .copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        assert_eq!(
            dispatch_local_packet_with_arp_cache(
                &unsupported_operation,
                endpoint,
                &mut cache,
                &mut output
            ),
            Err(PacketError::UnsupportedArpOperation)
        );

        assert_eq!(cache.lookup([192, 0, 2, 55]), Some(original_mac));
        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
        assert_eq!(cache.lookup([192, 0, 2, 99]), None);
    }

    #[test_case]
    fn cache_aware_dispatch_preserves_icmp_echo_without_arp_learning() {
        let endpoint = local_endpoint();
        let request = icmp_echo_request_frame();
        let mut cache = ArpCache::<4>::new();
        let mut cache_unaware_output = [0u8; 128];
        let mut cache_aware_output = [0u8; 128];

        let cache_unaware =
            dispatch_local_packet(&request, endpoint, &mut cache_unaware_output).expect("icmp");
        let cache_aware = dispatch_local_packet_with_arp_cache(
            &request,
            endpoint,
            &mut cache,
            &mut cache_aware_output,
        )
        .expect("icmp with cache");

        assert_eq!(cache_aware, cache_unaware);
        assert_eq!(
            &cache_aware_output[..cache_aware.frame_len()],
            &cache_unaware_output[..cache_unaware.frame_len()]
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
    }

    struct PollDevice<'a> {
        frame: Option<&'a [u8]>,
        receive_error: Option<DeviceError>,
        transmit_error: Option<DeviceError>,
        transmitted: [u8; 128],
        transmitted_len: usize,
    }

    impl<'a> PollDevice<'a> {
        fn with_frame(frame: &'a [u8]) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_receive_error(error: DeviceError) -> Self {
            Self {
                frame: None,
                receive_error: Some(error),
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_transmit_error(frame: &'a [u8], error: DeviceError) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                transmit_error: Some(error),
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }
    }

    impl<'a> NetworkDevice for PollDevice<'a> {
        fn receive_frame<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8], DeviceError> {
            if let Some(error) = self.receive_error {
                return Err(error);
            }

            let frame = self.frame.expect("test poll device frame configured");
            if buffer.len() < frame.len() {
                return Err(DeviceError::BufferTooSmall);
            }

            buffer[..frame.len()].copy_from_slice(frame);
            Ok(&buffer[..frame.len()])
        }

        fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError> {
            if let Some(error) = self.transmit_error {
                return Err(error);
            }

            self.transmitted[..frame.len()].copy_from_slice(frame);
            self.transmitted_len = frame.len();
            Ok(())
        }
    }

    #[test_case]
    fn poll_step_transmits_arp_reply_from_caller_owned_buffers() {
        let endpoint = local_endpoint();
        let request = arp_request_frame();
        let mut device = PollDevice::with_frame(&request);
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 64];

        let result = poll_local_network_device(
            &mut device,
            endpoint,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(
            result,
            LocalPollStepResult::Replied(PacketDispatchResult {
                reply_kind: PacketReplyKind::Arp,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            device.transmitted_len,
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
        assert_eq!(
            &device.transmitted[..device.transmitted_len],
            &transmit_buffer[..device.transmitted_len]
        );
        assert_eq!(&device.transmitted[0..6], &[0x02, 0, 0, 0, 0, 1]);
        assert_eq!(&device.transmitted[6..12], &[0x02, 0, 0, 0, 0, 99]);
    }

    #[test_case]
    fn poll_step_transmits_icmp_echo_reply_from_caller_owned_buffers() {
        let endpoint = local_endpoint();
        let request = icmp_echo_request_frame();
        let mut device = PollDevice::with_frame(&request);
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        let result = poll_local_network_device(
            &mut device,
            endpoint,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(
            result,
            LocalPollStepResult::Replied(PacketDispatchResult {
                reply_kind: PacketReplyKind::IcmpEcho,
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12,
            })
        );
        let transmitted = &device.transmitted[..device.transmitted_len];
        assert_eq!(read_be_u16(transmitted, 12), ETHERTYPE_IPV4);
        let ipv4 = &transmitted[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        assert!(ipv4_header_checksum_is_valid(ipv4));
        let icmp = &transmitted[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..];
        assert_eq!(icmp[0], 0);
        assert!(internet_checksum_is_valid(icmp));
    }

    #[test_case]
    fn poll_step_distinguishes_receive_boundaries() {
        let endpoint = local_endpoint();
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        let mut no_frame = PollDevice::with_receive_error(DeviceError::WouldBlock);
        assert_eq!(
            poll_local_network_device(
                &mut no_frame,
                endpoint,
                &mut receive_buffer,
                &mut transmit_buffer
            ),
            LocalPollStepResult::NoFrame
        );

        let request = icmp_echo_request_frame();
        let mut small_rx = [0u8; ETHERNET_HEADER_LEN - 1];
        let mut receive_pressure = PollDevice::with_frame(&request);
        assert_eq!(
            poll_local_network_device(
                &mut receive_pressure,
                endpoint,
                &mut small_rx,
                &mut transmit_buffer
            ),
            LocalPollStepResult::ReceiveBufferTooSmall
        );

        let mut receive_error = PollDevice::with_receive_error(DeviceError::Io);
        assert_eq!(
            poll_local_network_device(
                &mut receive_error,
                endpoint,
                &mut receive_buffer,
                &mut transmit_buffer
            ),
            LocalPollStepResult::ReceiveError(DeviceError::Io)
        );
    }

    #[test_case]
    fn poll_step_distinguishes_no_reply_dispatch_and_transmit_errors() {
        let endpoint = local_endpoint();
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        let mut nonlocal = icmp_echo_request_frame();
        nonlocal[0..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 88]);
        let mut no_reply = PollDevice::with_frame(&nonlocal);
        assert_eq!(
            poll_local_network_device(
                &mut no_reply,
                endpoint,
                &mut receive_buffer,
                &mut transmit_buffer
            ),
            LocalPollStepResult::NoReply
        );
        assert_eq!(no_reply.transmitted_len, 0);

        let mut unsupported = icmp_echo_request_frame();
        write_be_u16(&mut unsupported, 12, 0x86dd);
        let mut dispatch_error = PollDevice::with_frame(&unsupported);
        assert_eq!(
            poll_local_network_device(
                &mut dispatch_error,
                endpoint,
                &mut receive_buffer,
                &mut transmit_buffer
            ),
            LocalPollStepResult::DispatchError(PacketError::UnsupportedEtherType)
        );
        assert_eq!(dispatch_error.transmitted_len, 0);

        let request = arp_request_frame();
        let mut transmit_error = PollDevice::with_transmit_error(&request, DeviceError::Io);
        assert_eq!(
            poll_local_network_device(
                &mut transmit_error,
                endpoint,
                &mut receive_buffer,
                &mut transmit_buffer
            ),
            LocalPollStepResult::TransmitError(DeviceError::Io)
        );
        assert_eq!(transmit_error.transmitted_len, 0);
    }

    #[test_case]
    fn cache_aware_poll_transmits_arp_reply_and_learns_sender() {
        let endpoint = local_endpoint();
        let request = arp_request_frame();
        let mut device = PollDevice::with_frame(&request);
        let mut cache = ArpCache::<4>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 64];

        let result = poll_local_network_device_with_arp_cache(
            &mut device,
            endpoint,
            &mut cache,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(
            result,
            LocalPollStepResult::Replied(PacketDispatchResult {
                reply_kind: PacketReplyKind::Arp,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            cache.lookup([192, 0, 2, 10]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 1]))
        );
        assert_eq!(
            device.transmitted_len,
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
    }

    #[test_case]
    fn cache_aware_poll_learns_arp_reply_without_transmit() {
        let endpoint = local_endpoint();
        let reply = arp_reply_frame();
        let mut device = PollDevice::with_frame(&reply);
        let mut cache = ArpCache::<4>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 64];

        let result = poll_local_network_device_with_arp_cache(
            &mut device,
            endpoint,
            &mut cache,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(
            result,
            LocalPollStepResult::DispatchError(PacketError::UnsupportedArpOperation)
        );
        assert_eq!(
            cache.lookup([192, 0, 2, 20]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
        assert_eq!(device.transmitted_len, 0);
    }

    #[test_case]
    fn cache_aware_poll_no_frame_leaves_cache_unchanged() {
        let endpoint = local_endpoint();
        let original_mac = MacAddress::new([0x02, 0, 0, 0, 0, 55]);
        let mut device = PollDevice::with_receive_error(DeviceError::WouldBlock);
        let mut cache = ArpCache::<4>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 55], original_mac),
            ArpCacheUpdate::Inserted
        );
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 64];

        let result = poll_local_network_device_with_arp_cache(
            &mut device,
            endpoint,
            &mut cache,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(result, LocalPollStepResult::NoFrame);
        assert_eq!(cache.lookup([192, 0, 2, 55]), Some(original_mac));
        assert_eq!(cache.lookup([192, 0, 2, 10]), None);
    }

    #[test_case]
    fn cache_aware_poll_transmit_error_keeps_learned_sender() {
        let endpoint = local_endpoint();
        let request = arp_request_frame();
        let mut device = PollDevice::with_transmit_error(&request, DeviceError::Io);
        let mut cache = ArpCache::<4>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 64];

        let result = poll_local_network_device_with_arp_cache(
            &mut device,
            endpoint,
            &mut cache,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(result, LocalPollStepResult::TransmitError(DeviceError::Io));
        assert_eq!(
            cache.lookup([192, 0, 2, 10]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 1]))
        );
        assert_eq!(device.transmitted_len, 0);
    }

    const fn local_endpoint() -> LocalNetworkEndpoint {
        LocalNetworkEndpoint::new(MacAddress::new([0x02, 0, 0, 0, 0, 99]), [192, 0, 2, 1])
    }

    fn arp_request_frame() -> [u8; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN] {
        let mut frame = [0u8; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN];
        frame[..6].copy_from_slice(&[0xff; 6]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        write_be_u16(&mut frame, 12, ETHERTYPE_ARP);
        frame[ETHERNET_HEADER_LEN..].copy_from_slice(ARP_REQUEST);
        frame
    }

    fn arp_reply_frame() -> [u8; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN] {
        let mut frame = [0u8; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_be_u16(&mut frame, 12, ETHERTYPE_ARP);

        let arp = &mut frame[ETHERNET_HEADER_LEN..];
        write_be_u16(arp, 0, 1);
        write_be_u16(arp, 2, ETHERTYPE_IPV4);
        arp[4] = ETHERNET_ADDR_LEN as u8;
        arp[5] = 4;
        write_be_u16(arp, 6, ArpOperation::Reply.raw());
        arp[8..14].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        arp[14..18].copy_from_slice(&[192, 0, 2, 20]);
        arp[18..24].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        arp[24..28].copy_from_slice(&[192, 0, 2, 10]);
        frame
    }

    fn icmp_echo_request_frame() -> [u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12] {
        let mut frame = [0u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        write_be_u16(&mut frame, 12, ETHERTYPE_IPV4);

        let ipv4 = &mut frame[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        ipv4[0] = 0x45;
        write_be_u16(ipv4, 2, (IPV4_MIN_HEADER_LEN + 12) as u16);
        write_be_u16(ipv4, 4, 0x2222);
        ipv4[8] = 64;
        ipv4[9] = IPV4_PROTOCOL_ICMP;
        ipv4[12..16].copy_from_slice(&[192, 0, 2, 10]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
        let checksum = internet_checksum(ipv4);
        write_be_u16(ipv4, 10, checksum);

        let icmp = &mut frame[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..];
        icmp[0] = 8;
        icmp[4..].copy_from_slice(&[0x12, 0x34, 0, 7, 1, 2, 3, 4]);
        let checksum = internet_checksum(icmp);
        write_be_u16(icmp, 2, checksum);
        frame
    }

    fn icmp_echo_request_with_options_frame() -> [u8; ETHERNET_HEADER_LEN + 24 + 8] {
        let mut frame = [0u8; ETHERNET_HEADER_LEN + 24 + 8];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
        write_be_u16(&mut frame, 12, ETHERTYPE_IPV4);

        let ipv4 = &mut frame[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + 24];
        ipv4[0] = 0x46;
        write_be_u16(ipv4, 2, 32);
        write_be_u16(ipv4, 4, 0x3333);
        ipv4[8] = 64;
        ipv4[9] = IPV4_PROTOCOL_ICMP;
        ipv4[12..16].copy_from_slice(&[192, 0, 2, 10]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
        ipv4[20..24].copy_from_slice(&[1, 2, 3, 4]);
        let checksum = internet_checksum(ipv4);
        write_be_u16(ipv4, 10, checksum);

        let icmp = &mut frame[ETHERNET_HEADER_LEN + 24..];
        icmp[0] = 8;
        let checksum = internet_checksum(icmp);
        write_be_u16(icmp, 2, checksum);
        frame
    }

    fn rewrite_ipv4_checksum(frame: &mut [u8]) {
        let header_len = ((frame[ETHERNET_HEADER_LEN] & 0x0f) as usize) * 4;
        let ipv4 = &mut frame[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + header_len];
        write_be_u16(ipv4, 10, 0);
        let checksum = internet_checksum(ipv4);
        write_be_u16(ipv4, 10, checksum);
    }
}
