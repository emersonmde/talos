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
pub(crate) enum OutboundFrameError {
    NeighborUnresolved {
        destination_ipv4: [u8; 4],
    },
    PayloadTooLarge {
        required_len: usize,
        max_len: usize,
    },
    OutputBufferTooSmall {
        required_len: usize,
        available_len: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutboundRouteError {
    NoRouteToDestination { destination_ipv4: [u8; 4] },
    Frame(OutboundFrameError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Ipv4EgressRoutePolicy {
    subnet_mask: [u8; 4],
    gateway_ipv4: Option<[u8; 4]>,
}

impl Ipv4EgressRoutePolicy {
    pub(crate) const fn new(subnet_mask: [u8; 4], gateway_ipv4: Option<[u8; 4]>) -> Self {
        Self {
            subnet_mask,
            gateway_ipv4,
        }
    }

    pub(crate) const fn subnet_mask(self) -> [u8; 4] {
        self.subnet_mask
    }

    pub(crate) const fn gateway_ipv4(self) -> Option<[u8; 4]> {
        self.gateway_ipv4
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Ipv4EgressRouteKind {
    SameSubnet,
    Gateway,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Ipv4EgressRouteDecision {
    destination_ipv4: [u8; 4],
    next_hop_ipv4: [u8; 4],
    route_kind: Ipv4EgressRouteKind,
}

impl Ipv4EgressRouteDecision {
    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        self.destination_ipv4
    }

    pub(crate) const fn next_hop_ipv4(self) -> [u8; 4] {
        self.next_hop_ipv4
    }

    pub(crate) const fn route_kind(self) -> Ipv4EgressRouteKind {
        self.route_kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutboundRequestKind {
    Ipv4IcmpEchoRequest,
    ArpRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OutboundRequestSelection {
    request_kind: OutboundRequestKind,
    frame_len: usize,
}

impl OutboundRequestSelection {
    pub(crate) const fn request_kind(self) -> OutboundRequestKind {
        self.request_kind
    }

    pub(crate) const fn frame_len(self) -> usize {
        self.frame_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutboundTransmitResult {
    Ipv4IcmpEchoRequestTransmitted {
        frame_len: usize,
    },
    ArpRequestTransmitted {
        frame_len: usize,
    },
    RequestError(OutboundFrameError),
    TransmitError {
        request_kind: OutboundRequestKind,
        frame_len: usize,
        error: DeviceError,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PendingIcmpEchoResult {
    IcmpEchoRequestTransmitted {
        frame_len: usize,
    },
    ArpRequestTransmittedAndPending {
        frame_len: usize,
    },
    NoPendingRequest,
    PendingRequestAlreadyQueued {
        destination_ipv4: [u8; 4],
    },
    PendingPayloadTooLarge {
        required_len: usize,
        max_len: usize,
    },
    PendingNeighborUnresolved {
        destination_ipv4: [u8; 4],
    },
    ArpRetryBudgetExhausted {
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
    },
    RouteError(OutboundRouteError),
    NonMatchingArp {
        pending_destination_ipv4: [u8; 4],
        arp_sender_ipv4: [u8; 4],
    },
    RequestError(OutboundFrameError),
    ArpError(PacketError),
    TransmitError {
        request_kind: OutboundRequestKind,
        frame_len: usize,
        error: DeviceError,
    },
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
pub(crate) enum OutboundNeighborResolution {
    Resolved {
        destination_ipv4: [u8; 4],
        destination_mac: MacAddress,
    },
    Unresolved {
        destination_ipv4: [u8; 4],
    },
}

impl OutboundNeighborResolution {
    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        match self {
            Self::Resolved {
                destination_ipv4, ..
            }
            | Self::Unresolved { destination_ipv4 } => destination_ipv4,
        }
    }

    pub(crate) const fn destination_mac(self) -> Option<MacAddress> {
        match self {
            Self::Resolved {
                destination_mac, ..
            } => Some(destination_mac),
            Self::Unresolved { .. } => None,
        }
    }
}

pub(crate) fn resolve_outbound_neighbor<const ARP_CAPACITY: usize>(
    arp_cache: &ArpCache<ARP_CAPACITY>,
    destination_ipv4: [u8; 4],
) -> OutboundNeighborResolution {
    match arp_cache.lookup(destination_ipv4) {
        Some(destination_mac) => OutboundNeighborResolution::Resolved {
            destination_ipv4,
            destination_mac,
        },
        None => OutboundNeighborResolution::Unresolved { destination_ipv4 },
    }
}

pub(crate) fn route_ipv4_egress(
    endpoint: LocalNetworkEndpoint,
    policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
) -> Result<Ipv4EgressRouteDecision, OutboundRouteError> {
    if ipv4_same_subnet(endpoint.ipv4(), destination_ipv4, policy.subnet_mask()) {
        return Ok(Ipv4EgressRouteDecision {
            destination_ipv4,
            next_hop_ipv4: destination_ipv4,
            route_kind: Ipv4EgressRouteKind::SameSubnet,
        });
    }

    match policy.gateway_ipv4() {
        Some(gateway_ipv4) => Ok(Ipv4EgressRouteDecision {
            destination_ipv4,
            next_hop_ipv4: gateway_ipv4,
            route_kind: Ipv4EgressRouteKind::Gateway,
        }),
        None => Err(OutboundRouteError::NoRouteToDestination { destination_ipv4 }),
    }
}

pub(crate) fn build_outbound_ethernet_frame(
    resolution: OutboundNeighborResolution,
    source_mac: MacAddress,
    ether_type: EtherType,
    payload: &[u8],
    output: &mut [u8],
) -> Result<usize, OutboundFrameError> {
    let destination_mac = resolved_destination_mac(resolution)?;
    let frame_len = ETHERNET_HEADER_LEN + payload.len();
    if output.len() < frame_len {
        return Err(OutboundFrameError::OutputBufferTooSmall {
            required_len: frame_len,
            available_len: output.len(),
        });
    }

    write_ethernet_header(output, destination_mac, source_mac, ether_type);
    output[ETHERNET_HEADER_LEN..frame_len].copy_from_slice(payload);

    Ok(frame_len)
}

pub(crate) fn build_outbound_arp_request(
    endpoint: LocalNetworkEndpoint,
    target_ipv4: [u8; 4],
    output: &mut [u8],
) -> Result<usize, OutboundFrameError> {
    let frame_len = ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN;
    if output.len() < frame_len {
        return Err(OutboundFrameError::OutputBufferTooSmall {
            required_len: frame_len,
            available_len: output.len(),
        });
    }

    write_ethernet_header(
        output,
        MacAddress::new([0xff; ETHERNET_ADDR_LEN]),
        endpoint.mac(),
        EtherType::Arp,
    );

    let arp = &mut output[ETHERNET_HEADER_LEN..frame_len];
    write_be_u16(arp, 0, 1);
    write_be_u16(arp, 2, ETHERTYPE_IPV4);
    arp[4] = ETHERNET_ADDR_LEN as u8;
    arp[5] = 4;
    write_be_u16(arp, 6, ArpOperation::Request.raw());
    arp[8..14].copy_from_slice(&endpoint.mac().bytes());
    arp[14..18].copy_from_slice(&endpoint.ipv4());
    arp[18..24].copy_from_slice(&[0; ETHERNET_ADDR_LEN]);
    arp[24..28].copy_from_slice(&target_ipv4);

    Ok(frame_len)
}

pub(crate) fn build_outbound_ipv4_icmp_echo_request(
    resolution: OutboundNeighborResolution,
    endpoint: LocalNetworkEndpoint,
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> Result<usize, OutboundFrameError> {
    build_outbound_routed_ipv4_icmp_echo_request(
        resolution,
        resolution.destination_ipv4(),
        endpoint,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
    )
}

pub(crate) fn build_outbound_routed_ipv4_icmp_echo_request(
    next_hop_resolution: OutboundNeighborResolution,
    destination_ipv4: [u8; 4],
    endpoint: LocalNetworkEndpoint,
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> Result<usize, OutboundFrameError> {
    let destination_mac = resolved_destination_mac(next_hop_resolution)?;
    let icmp_len = ICMP_ECHO_HEADER_LEN + payload.len();
    let ipv4_len = IPV4_MIN_HEADER_LEN + icmp_len;
    if ipv4_len > u16::MAX as usize {
        return Err(OutboundFrameError::PayloadTooLarge {
            required_len: ipv4_len,
            max_len: u16::MAX as usize,
        });
    }

    let frame_len = ETHERNET_HEADER_LEN + ipv4_len;
    if output.len() < frame_len {
        return Err(OutboundFrameError::OutputBufferTooSmall {
            required_len: frame_len,
            available_len: output.len(),
        });
    }

    write_ethernet_header(output, destination_mac, endpoint.mac(), EtherType::Ipv4);

    let ipv4 = &mut output[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
    ipv4[0] = 0x45;
    ipv4[1] = 0;
    write_be_u16(ipv4, 2, ipv4_len as u16);
    write_be_u16(ipv4, 4, 0);
    write_be_u16(ipv4, 6, 0);
    ipv4[8] = ttl;
    ipv4[9] = IPV4_PROTOCOL_ICMP;
    write_be_u16(ipv4, 10, 0);
    ipv4[12..16].copy_from_slice(&endpoint.ipv4());
    ipv4[16..20].copy_from_slice(&destination_ipv4);
    let ipv4_checksum = internet_checksum(ipv4);
    write_be_u16(ipv4, 10, ipv4_checksum);

    let icmp = &mut output[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..frame_len];
    icmp[0] = 8;
    icmp[1] = 0;
    write_be_u16(icmp, 2, 0);
    write_be_u16(icmp, 4, identifier);
    write_be_u16(icmp, 6, sequence_number);
    icmp[ICMP_ECHO_HEADER_LEN..].copy_from_slice(payload);
    let icmp_checksum = internet_checksum(icmp);
    write_be_u16(icmp, 2, icmp_checksum);

    Ok(frame_len)
}

pub(crate) fn select_outbound_ipv4_icmp_echo_request<const ARP_CAPACITY: usize>(
    arp_cache: &ArpCache<ARP_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> Result<OutboundRequestSelection, OutboundFrameError> {
    let resolution = resolve_outbound_neighbor(arp_cache, destination_ipv4);
    let (request_kind, frame_len) = match resolution {
        OutboundNeighborResolution::Resolved { .. } => (
            OutboundRequestKind::Ipv4IcmpEchoRequest,
            build_outbound_ipv4_icmp_echo_request(
                resolution,
                endpoint,
                identifier,
                sequence_number,
                ttl,
                payload,
                output,
            )?,
        ),
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => (
            OutboundRequestKind::ArpRequest,
            build_outbound_arp_request(endpoint, destination_ipv4, output)?,
        ),
    };

    Ok(OutboundRequestSelection {
        request_kind,
        frame_len,
    })
}

pub(crate) fn select_routed_outbound_ipv4_icmp_echo_request<const ARP_CAPACITY: usize>(
    arp_cache: &ArpCache<ARP_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route_policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> Result<OutboundRequestSelection, OutboundRouteError> {
    let route = route_ipv4_egress(endpoint, route_policy, destination_ipv4)?;
    let next_hop_resolution = resolve_outbound_neighbor(arp_cache, route.next_hop_ipv4());
    let (request_kind, frame_len) = match next_hop_resolution {
        OutboundNeighborResolution::Resolved { .. } => (
            OutboundRequestKind::Ipv4IcmpEchoRequest,
            build_outbound_routed_ipv4_icmp_echo_request(
                next_hop_resolution,
                route.destination_ipv4(),
                endpoint,
                identifier,
                sequence_number,
                ttl,
                payload,
                output,
            )
            .map_err(OutboundRouteError::Frame)?,
        ),
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => (
            OutboundRequestKind::ArpRequest,
            build_outbound_arp_request(endpoint, destination_ipv4, output)
                .map_err(OutboundRouteError::Frame)?,
        ),
    };

    Ok(OutboundRequestSelection {
        request_kind,
        frame_len,
    })
}

pub(crate) fn transmit_one_outbound_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> OutboundTransmitResult {
    let selection = match select_outbound_ipv4_icmp_echo_request(
        arp_cache,
        endpoint,
        destination_ipv4,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
    ) {
        Ok(selection) => selection,
        Err(error) => return OutboundTransmitResult::RequestError(error),
    };

    let frame_len = selection.frame_len();
    match device.transmit_frame(&output[..frame_len]) {
        Ok(()) => match selection.request_kind() {
            OutboundRequestKind::Ipv4IcmpEchoRequest => {
                OutboundTransmitResult::Ipv4IcmpEchoRequestTransmitted { frame_len }
            }
            OutboundRequestKind::ArpRequest => {
                OutboundTransmitResult::ArpRequestTransmitted { frame_len }
            }
        },
        Err(error) => OutboundTransmitResult::TransmitError {
            request_kind: selection.request_kind(),
            frame_len,
            error,
        },
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PendingIcmpEchoRequest<const PAYLOAD_CAPACITY: usize> {
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    next_hop_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: [u8; PAYLOAD_CAPACITY],
    payload_len: usize,
    arp_retries_remaining: usize,
}

impl<const PAYLOAD_CAPACITY: usize> PendingIcmpEchoRequest<PAYLOAD_CAPACITY> {
    pub(crate) fn new(
        endpoint: LocalNetworkEndpoint,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
    ) -> Result<Self, PendingIcmpEchoResult> {
        Self::new_with_next_hop(
            endpoint,
            destination_ipv4,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
        )
    }

    pub(crate) fn new_with_next_hop(
        endpoint: LocalNetworkEndpoint,
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
    ) -> Result<Self, PendingIcmpEchoResult> {
        Self::new_with_next_hop_and_arp_retry_budget(
            endpoint,
            destination_ipv4,
            next_hop_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            0,
        )
    }

    pub(crate) fn new_with_next_hop_and_arp_retry_budget(
        endpoint: LocalNetworkEndpoint,
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        arp_retries_remaining: usize,
    ) -> Result<Self, PendingIcmpEchoResult> {
        if payload.len() > PAYLOAD_CAPACITY {
            return Err(PendingIcmpEchoResult::PendingPayloadTooLarge {
                required_len: payload.len(),
                max_len: PAYLOAD_CAPACITY,
            });
        }

        let mut stored_payload = [0; PAYLOAD_CAPACITY];
        stored_payload[..payload.len()].copy_from_slice(payload);

        Ok(Self {
            endpoint,
            destination_ipv4,
            next_hop_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload: stored_payload,
            payload_len: payload.len(),
            arp_retries_remaining,
        })
    }

    pub(crate) const fn endpoint(self) -> LocalNetworkEndpoint {
        self.endpoint
    }

    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        self.destination_ipv4
    }

    pub(crate) const fn next_hop_ipv4(self) -> [u8; 4] {
        self.next_hop_ipv4
    }

    pub(crate) const fn identifier(self) -> u16 {
        self.identifier
    }

    pub(crate) const fn sequence_number(self) -> u16 {
        self.sequence_number
    }

    pub(crate) const fn ttl(self) -> u8 {
        self.ttl
    }

    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload[..self.payload_len]
    }

    pub(crate) const fn arp_retries_remaining(self) -> usize {
        self.arp_retries_remaining
    }

    fn consume_arp_retry(&mut self) -> bool {
        if self.arp_retries_remaining == 0 {
            return false;
        }

        self.arp_retries_remaining -= 1;
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SinglePendingIcmpEcho<const PAYLOAD_CAPACITY: usize> {
    pending: Option<PendingIcmpEchoRequest<PAYLOAD_CAPACITY>>,
}

impl<const PAYLOAD_CAPACITY: usize> SinglePendingIcmpEcho<PAYLOAD_CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self { pending: None }
    }

    pub(crate) const fn pending(&self) -> Option<PendingIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.pending
    }

    pub(crate) const fn pending_destination_ipv4(&self) -> Option<[u8; 4]> {
        match self.pending {
            Some(request) => Some(request.destination_ipv4()),
            None => None,
        }
    }

    fn store(
        &mut self,
        request: PendingIcmpEchoRequest<PAYLOAD_CAPACITY>,
    ) -> Result<(), PendingIcmpEchoResult> {
        if let Some(existing) = self.pending {
            return Err(PendingIcmpEchoResult::PendingRequestAlreadyQueued {
                destination_ipv4: existing.destination_ipv4(),
            });
        }

        self.pending = Some(request);
        Ok(())
    }

    fn take(&mut self) -> Option<PendingIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.pending.take()
    }

    fn restore(&mut self, request: PendingIcmpEchoRequest<PAYLOAD_CAPACITY>) {
        self.pending = Some(request);
    }
}

pub(crate) fn transmit_or_queue_single_pending_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    if let Some(existing_destination) = pending.pending_destination_ipv4() {
        return PendingIcmpEchoResult::PendingRequestAlreadyQueued {
            destination_ipv4: existing_destination,
        };
    }

    match resolve_outbound_neighbor(arp_cache, destination_ipv4) {
        OutboundNeighborResolution::Resolved { .. } => transmit_resolved_ipv4_icmp_echo_request(
            device,
            arp_cache,
            endpoint,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            output,
        ),
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => {
            let request = match PendingIcmpEchoRequest::new(
                endpoint,
                destination_ipv4,
                identifier,
                sequence_number,
                ttl,
                payload,
            ) {
                Ok(request) => request,
                Err(error) => return error,
            };

            let frame_len = match build_outbound_arp_request(endpoint, destination_ipv4, output) {
                Ok(frame_len) => frame_len,
                Err(error) => return PendingIcmpEchoResult::RequestError(error),
            };

            match device.transmit_frame(&output[..frame_len]) {
                Ok(()) => match pending.store(request) {
                    Ok(()) => PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len },
                    Err(error) => error,
                },
                Err(error) => PendingIcmpEchoResult::TransmitError {
                    request_kind: OutboundRequestKind::ArpRequest,
                    frame_len,
                    error,
                },
            }
        }
    }
}

pub(crate) fn transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route_policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget(
        device,
        arp_cache,
        pending,
        endpoint,
        route_policy,
        destination_ipv4,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
        0,
    )
}

pub(crate) fn transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route_policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
    arp_retry_budget: usize,
) -> PendingIcmpEchoResult {
    let route = match route_ipv4_egress(endpoint, route_policy, destination_ipv4) {
        Ok(route) => route,
        Err(error) => return PendingIcmpEchoResult::RouteError(error),
    };

    if let Some(existing_destination) = pending.pending_destination_ipv4() {
        return PendingIcmpEchoResult::PendingRequestAlreadyQueued {
            destination_ipv4: existing_destination,
        };
    }

    match resolve_outbound_neighbor(arp_cache, route.next_hop_ipv4()) {
        OutboundNeighborResolution::Resolved { .. } => {
            transmit_resolved_routed_ipv4_icmp_echo_request(
                device,
                arp_cache,
                endpoint,
                route.destination_ipv4(),
                route.next_hop_ipv4(),
                identifier,
                sequence_number,
                ttl,
                payload,
                output,
            )
        }
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => {
            let request = match PendingIcmpEchoRequest::new_with_next_hop_and_arp_retry_budget(
                endpoint,
                route.destination_ipv4(),
                route.next_hop_ipv4(),
                identifier,
                sequence_number,
                ttl,
                payload,
                arp_retry_budget,
            ) {
                Ok(request) => request,
                Err(error) => return error,
            };

            let frame_len = match build_outbound_arp_request(endpoint, destination_ipv4, output) {
                Ok(frame_len) => frame_len,
                Err(error) => return PendingIcmpEchoResult::RequestError(error),
            };

            match device.transmit_frame(&output[..frame_len]) {
                Ok(()) => match pending.store(request) {
                    Ok(()) => PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len },
                    Err(error) => error,
                },
                Err(error) => PendingIcmpEchoResult::TransmitError {
                    request_kind: OutboundRequestKind::ArpRequest,
                    frame_len,
                    error,
                },
            }
        }
    }
}

pub(crate) fn retry_single_pending_ipv4_icmp_echo_arp_request<
    D: NetworkDevice,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let mut request = match pending.take() {
        Some(request) => request,
        None => return PendingIcmpEchoResult::NoPendingRequest,
    };

    if request.arp_retries_remaining() == 0 {
        let result = PendingIcmpEchoResult::ArpRetryBudgetExhausted {
            destination_ipv4: request.destination_ipv4(),
            next_hop_ipv4: request.next_hop_ipv4(),
        };
        pending.restore(request);
        return result;
    }

    let frame_len =
        match build_outbound_arp_request(request.endpoint(), request.next_hop_ipv4(), output) {
            Ok(frame_len) => frame_len,
            Err(error) => {
                pending.restore(request);
                return PendingIcmpEchoResult::RequestError(error);
            }
        };

    match device.transmit_frame(&output[..frame_len]) {
        Ok(()) => {
            let _ = request.consume_arp_retry();
            pending.restore(request);
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len }
        }
        Err(error) => {
            pending.restore(request);
            PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::ArpRequest,
                frame_len,
                error,
            }
        }
    }
}

pub(crate) fn transmit_single_pending_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let request = match pending.take() {
        Some(request) => request,
        None => return PendingIcmpEchoResult::NoPendingRequest,
    };

    match resolve_outbound_neighbor(arp_cache, request.next_hop_ipv4()) {
        OutboundNeighborResolution::Resolved { .. } => {
            let result =
                transmit_pending_request_with_resolution(device, request, arp_cache, output);
            if !matches!(
                result,
                PendingIcmpEchoResult::IcmpEchoRequestTransmitted { .. }
            ) {
                pending.restore(request);
            }
            result
        }
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => {
            pending.restore(request);
            PendingIcmpEchoResult::PendingNeighborUnresolved { destination_ipv4 }
        }
    }
}

pub(crate) fn learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    arp_reply_frame: &[u8],
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let request = match pending.take() {
        Some(request) => request,
        None => return PendingIcmpEchoResult::NoPendingRequest,
    };

    let frame = match EthernetFrame::parse(arp_reply_frame) {
        Ok(frame) => frame,
        Err(error) => {
            pending.restore(request);
            return PendingIcmpEchoResult::ArpError(error);
        }
    };
    if frame.ether_type() != EtherType::Arp {
        pending.restore(request);
        return PendingIcmpEchoResult::ArpError(PacketError::UnsupportedEtherType);
    }

    let arp = match ArpPacket::parse_ethernet_ipv4(frame.payload()) {
        Ok(arp) => arp,
        Err(error) => {
            pending.restore(request);
            return PendingIcmpEchoResult::ArpError(error);
        }
    };
    if arp.operation() != ArpOperation::Reply {
        pending.restore(request);
        return PendingIcmpEchoResult::ArpError(PacketError::UnsupportedArpOperation);
    }
    if arp.sender_protocol_address() != request.next_hop_ipv4() {
        let pending_destination_ipv4 = request.destination_ipv4();
        let arp_sender_ipv4 = arp.sender_protocol_address();
        pending.restore(request);
        return PendingIcmpEchoResult::NonMatchingArp {
            pending_destination_ipv4,
            arp_sender_ipv4,
        };
    }

    let _ =
        arp_cache.insert_or_update(arp.sender_protocol_address(), arp.sender_hardware_address());
    let resolution = OutboundNeighborResolution::Resolved {
        destination_ipv4: request.next_hop_ipv4(),
        destination_mac: arp.sender_hardware_address(),
    };
    let result =
        transmit_pending_request_with_direct_resolution(device, request, resolution, output);
    if !matches!(
        result,
        PendingIcmpEchoResult::IcmpEchoRequestTransmitted { .. }
    ) {
        pending.restore(request);
    }
    result
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

fn ipv4_same_subnet(local_ipv4: [u8; 4], destination_ipv4: [u8; 4], subnet_mask: [u8; 4]) -> bool {
    let local = u32::from_be_bytes(local_ipv4);
    let destination = u32::from_be_bytes(destination_ipv4);
    let mask = u32::from_be_bytes(subnet_mask);
    (local & mask) == (destination & mask)
}

fn resolved_destination_mac(
    resolution: OutboundNeighborResolution,
) -> Result<MacAddress, OutboundFrameError> {
    match resolution {
        OutboundNeighborResolution::Resolved {
            destination_mac, ..
        } => Ok(destination_mac),
        OutboundNeighborResolution::Unresolved { destination_ipv4 } => {
            Err(OutboundFrameError::NeighborUnresolved { destination_ipv4 })
        }
    }
}

fn write_ethernet_header(
    output: &mut [u8],
    destination_mac: MacAddress,
    source_mac: MacAddress,
    ether_type: EtherType,
) {
    output[..ETHERNET_ADDR_LEN].copy_from_slice(&destination_mac.bytes());
    output[ETHERNET_ADDR_LEN..ETHERNET_ADDR_LEN * 2].copy_from_slice(&source_mac.bytes());
    write_be_u16(output, 12, ether_type.raw());
}

fn transmit_resolved_ipv4_icmp_echo_request<D: NetworkDevice, const ARP_CAPACITY: usize>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    match transmit_one_outbound_ipv4_icmp_echo_request(
        device,
        arp_cache,
        endpoint,
        destination_ipv4,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
    ) {
        OutboundTransmitResult::Ipv4IcmpEchoRequestTransmitted { frame_len } => {
            PendingIcmpEchoResult::IcmpEchoRequestTransmitted { frame_len }
        }
        OutboundTransmitResult::ArpRequestTransmitted { frame_len } => {
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len }
        }
        OutboundTransmitResult::RequestError(error) => PendingIcmpEchoResult::RequestError(error),
        OutboundTransmitResult::TransmitError {
            request_kind,
            frame_len,
            error,
        } => PendingIcmpEchoResult::TransmitError {
            request_kind,
            frame_len,
            error,
        },
    }
}

fn transmit_resolved_routed_ipv4_icmp_echo_request<D: NetworkDevice, const ARP_CAPACITY: usize>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    next_hop_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let resolution = resolve_outbound_neighbor(arp_cache, next_hop_ipv4);
    let frame_len = match build_outbound_routed_ipv4_icmp_echo_request(
        resolution,
        destination_ipv4,
        endpoint,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
    ) {
        Ok(frame_len) => frame_len,
        Err(OutboundFrameError::NeighborUnresolved { destination_ipv4 }) => {
            return PendingIcmpEchoResult::PendingNeighborUnresolved { destination_ipv4 };
        }
        Err(error) => return PendingIcmpEchoResult::RequestError(error),
    };

    match device.transmit_frame(&output[..frame_len]) {
        Ok(()) => PendingIcmpEchoResult::IcmpEchoRequestTransmitted { frame_len },
        Err(error) => PendingIcmpEchoResult::TransmitError {
            request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
            frame_len,
            error,
        },
    }
}

fn transmit_pending_request_with_resolution<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    request: PendingIcmpEchoRequest<PAYLOAD_CAPACITY>,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let resolution = resolve_outbound_neighbor(arp_cache, request.next_hop_ipv4());
    transmit_pending_request_with_direct_resolution(device, request, resolution, output)
}

fn transmit_pending_request_with_direct_resolution<
    D: NetworkDevice,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    request: PendingIcmpEchoRequest<PAYLOAD_CAPACITY>,
    resolution: OutboundNeighborResolution,
    output: &mut [u8],
) -> PendingIcmpEchoResult {
    let frame_len = match build_outbound_routed_ipv4_icmp_echo_request(
        resolution,
        request.destination_ipv4(),
        request.endpoint(),
        request.identifier(),
        request.sequence_number(),
        request.ttl(),
        request.payload(),
        output,
    ) {
        Ok(frame_len) => frame_len,
        Err(OutboundFrameError::NeighborUnresolved { destination_ipv4 }) => {
            return PendingIcmpEchoResult::PendingNeighborUnresolved { destination_ipv4 };
        }
        Err(error) => return PendingIcmpEchoResult::RequestError(error),
    };

    match device.transmit_frame(&output[..frame_len]) {
        Ok(()) => PendingIcmpEchoResult::IcmpEchoRequestTransmitted { frame_len },
        Err(error) => PendingIcmpEchoResult::TransmitError {
            request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
            frame_len,
            error,
        },
    }
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
    fn outbound_neighbor_resolution_returns_cached_mac_for_known_destination() {
        let mut cache = ArpCache::<2>::new();
        let mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], mac),
            ArpCacheUpdate::Inserted
        );

        let resolution = resolve_outbound_neighbor(&cache, [192, 0, 2, 10]);

        assert_eq!(
            resolution,
            OutboundNeighborResolution::Resolved {
                destination_ipv4: [192, 0, 2, 10],
                destination_mac: mac,
            }
        );
        assert_eq!(resolution.destination_ipv4(), [192, 0, 2, 10]);
        assert_eq!(resolution.destination_mac(), Some(mac));
    }

    #[test_case]
    fn outbound_neighbor_resolution_returns_destination_on_miss() {
        let cache = ArpCache::<2>::new();

        let resolution = resolve_outbound_neighbor(&cache, [192, 0, 2, 44]);

        assert_eq!(
            resolution,
            OutboundNeighborResolution::Unresolved {
                destination_ipv4: [192, 0, 2, 44],
            }
        );
        assert_eq!(resolution.destination_ipv4(), [192, 0, 2, 44]);
        assert_eq!(resolution.destination_mac(), None);
    }

    #[test_case]
    fn outbound_neighbor_resolution_uses_updated_cache_entries() {
        let mut cache = ArpCache::<2>::new();
        let old_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let new_mac = MacAddress::new([0x02, 0, 0, 0, 0, 11]);

        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], old_mac),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], new_mac),
            ArpCacheUpdate::Updated
        );

        assert_eq!(
            resolve_outbound_neighbor(&cache, [192, 0, 2, 10]).destination_mac(),
            Some(new_mac)
        );
    }

    #[test_case]
    fn outbound_neighbor_resolution_zero_capacity_cache_is_deterministic_miss() {
        let mut cache = ArpCache::<0>::new();
        let mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);

        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], mac),
            ArpCacheUpdate::NoCapacity
        );

        assert_eq!(
            resolve_outbound_neighbor(&cache, [192, 0, 2, 10]),
            OutboundNeighborResolution::Unresolved {
                destination_ipv4: [192, 0, 2, 10],
            }
        );
    }

    #[test_case]
    fn outbound_frame_construction_writes_resolved_neighbor_frame() {
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let source_mac = MacAddress::new([0x02, 0, 0, 0, 0, 99]);
        let payload = [0xde, 0xad, 0xbe, 0xef];
        let mut output = [0u8; 64];

        let frame_len = build_outbound_ethernet_frame(
            OutboundNeighborResolution::Resolved {
                destination_ipv4: [192, 0, 2, 10],
                destination_mac,
            },
            source_mac,
            EtherType::Ipv4,
            &payload,
            &mut output,
        )
        .expect("build outbound frame");

        assert_eq!(frame_len, ETHERNET_HEADER_LEN + payload.len());
        let frame = EthernetFrame::parse(&output[..frame_len]).expect("parse outbound frame");
        assert_eq!(frame.destination(), destination_mac);
        assert_eq!(frame.source(), source_mac);
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
        assert_eq!(frame.payload(), &payload);
    }

    #[test_case]
    fn outbound_frame_construction_rejects_unresolved_neighbor_without_side_effects() {
        let cache = ArpCache::<2>::new();
        let resolution = resolve_outbound_neighbor(&cache, [192, 0, 2, 44]);
        let mut output = [0xaa; 32];

        assert_eq!(
            build_outbound_ethernet_frame(
                resolution,
                MacAddress::new([0x02, 0, 0, 0, 0, 99]),
                EtherType::Ipv4,
                &[0xde, 0xad],
                &mut output,
            ),
            Err(OutboundFrameError::NeighborUnresolved {
                destination_ipv4: [192, 0, 2, 44],
            })
        );
        assert_eq!(cache.lookup([192, 0, 2, 44]), None);
        assert_eq!(output, [0xaa; 32]);
    }

    #[test_case]
    fn outbound_frame_construction_rejects_too_small_output_without_partial_frame() {
        let payload = [0xde, 0xad, 0xbe, 0xef];
        let mut output = [0xaa; ETHERNET_HEADER_LEN + 3];

        assert_eq!(
            build_outbound_ethernet_frame(
                OutboundNeighborResolution::Resolved {
                    destination_ipv4: [192, 0, 2, 10],
                    destination_mac: MacAddress::new([0x02, 0, 0, 0, 0, 10]),
                },
                MacAddress::new([0x02, 0, 0, 0, 0, 99]),
                EtherType::Ipv4,
                &payload,
                &mut output,
            ),
            Err(OutboundFrameError::OutputBufferTooSmall {
                required_len: ETHERNET_HEADER_LEN + payload.len(),
                available_len: ETHERNET_HEADER_LEN + 3,
            })
        );
        assert_eq!(output, [0xaa; ETHERNET_HEADER_LEN + 3]);
    }

    #[test_case]
    fn outbound_frame_construction_composes_with_cached_neighbor_resolution() {
        let mut cache = ArpCache::<2>::new();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let source_mac = local_endpoint().mac();
        let payload = [1, 2, 3, 4, 5];
        let mut output = [0u8; 64];

        let frame_len = build_outbound_ethernet_frame(
            resolve_outbound_neighbor(&cache, [192, 0, 2, 10]),
            source_mac,
            EtherType::Other(0x88b5),
            &payload,
            &mut output,
        )
        .expect("build cached outbound frame");

        assert_eq!(frame_len, ETHERNET_HEADER_LEN + payload.len());
        assert_eq!(&output[0..6], &destination_mac.bytes());
        assert_eq!(&output[6..12], &source_mac.bytes());
        assert_eq!(read_be_u16(&output, 12), 0x88b5);
        assert_eq!(&output[ETHERNET_HEADER_LEN..frame_len], &payload);
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(destination_mac));
    }

    #[test_case]
    fn outbound_arp_request_builds_complete_broadcast_request_frame() {
        let endpoint = local_endpoint();
        let mut output = [0u8; 64];

        let frame_len = build_outbound_arp_request(endpoint, [192, 0, 2, 44], &mut output)
            .expect("build outbound arp request");

        assert_eq!(frame_len, ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN);
        let frame = EthernetFrame::parse(&output[..frame_len]).expect("parse outbound arp frame");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);

        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("parse outbound arp");
        assert_eq!(arp.operation(), ArpOperation::Request);
        assert_eq!(arp.sender_hardware_address(), endpoint.mac());
        assert_eq!(arp.sender_protocol_address(), endpoint.ipv4());
        assert_eq!(arp.target_hardware_address(), MacAddress::new([0; 6]));
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 44]);
    }

    #[test_case]
    fn outbound_arp_request_rejects_too_small_output_without_partial_frame() {
        let mut output = [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];

        assert_eq!(
            build_outbound_arp_request(local_endpoint(), [192, 0, 2, 44], &mut output),
            Err(OutboundFrameError::OutputBufferTooSmall {
                required_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                available_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1,
            })
        );
        assert_eq!(
            output,
            [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1]
        );
    }

    #[test_case]
    fn outbound_arp_request_composes_with_unresolved_neighbor_resolution_without_cache_mutation() {
        let cache = ArpCache::<2>::new();
        let resolution = resolve_outbound_neighbor(&cache, [192, 0, 2, 44]);
        let mut output = [0u8; 64];

        let frame_len = build_outbound_arp_request(
            local_endpoint(),
            resolution.destination_ipv4(),
            &mut output,
        )
        .expect("build outbound arp request for unresolved neighbor");

        assert_eq!(frame_len, ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN);
        assert_eq!(&output[0..6], &[0xff; 6]);
        assert_eq!(read_be_u16(&output, 12), ETHERTYPE_ARP);
        assert_eq!(
            read_be_u16(&output[ETHERNET_HEADER_LEN..], 6),
            ArpOperation::Request.raw()
        );
        assert_eq!(
            &output[ETHERNET_HEADER_LEN + 24..ETHERNET_HEADER_LEN + 28],
            &[192, 0, 2, 44]
        );
        assert_eq!(cache.lookup([192, 0, 2, 44]), None);
    }

    #[test_case]
    fn outbound_ipv4_icmp_echo_request_builds_complete_resolved_neighbor_frame() {
        let endpoint = local_endpoint();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let payload = [1, 2, 3, 4, 5];
        let mut output = [0u8; 128];

        let frame_len = build_outbound_ipv4_icmp_echo_request(
            OutboundNeighborResolution::Resolved {
                destination_ipv4: [192, 0, 2, 10],
                destination_mac,
            },
            endpoint,
            0x1234,
            7,
            37,
            &payload,
            &mut output,
        )
        .expect("build outbound icmp echo request");

        assert_eq!(
            frame_len,
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len()
        );
        let frame = EthernetFrame::parse(&output[..frame_len]).expect("parse outbound frame");
        assert_eq!(frame.destination(), destination_mac);
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Ipv4);

        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("parse outbound ipv4");
        assert_eq!(ipv4.header_len(), IPV4_MIN_HEADER_LEN);
        assert_eq!(
            ipv4.total_len(),
            IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len()
        );
        assert_eq!(ipv4.protocol(), IPV4_PROTOCOL_ICMP);
        assert_eq!(ipv4.source(), endpoint.ipv4());
        assert_eq!(ipv4.destination(), [192, 0, 2, 10]);
        assert_eq!(frame.payload()[8], 37);
        assert!(ipv4_header_checksum_is_valid(
            &frame.payload()[..IPV4_MIN_HEADER_LEN]
        ));

        let icmp = ipv4.payload();
        assert_eq!(icmp[0], 8);
        assert_eq!(icmp[1], 0);
        assert_eq!(read_be_u16(icmp, 4), 0x1234);
        assert_eq!(read_be_u16(icmp, 6), 7);
        assert_eq!(&icmp[ICMP_ECHO_HEADER_LEN..], &payload);
        assert!(internet_checksum_is_valid(icmp));
    }

    #[test_case]
    fn outbound_ipv4_icmp_echo_request_composes_with_cached_resolution() {
        let endpoint = local_endpoint();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let mut output = [0u8; 128];

        let frame_len = build_outbound_ipv4_icmp_echo_request(
            resolve_outbound_neighbor(&cache, [192, 0, 2, 10]),
            endpoint,
            0xabcd,
            0x0102,
            64,
            &[0xaa, 0xbb],
            &mut output,
        )
        .expect("build cached outbound icmp echo request");

        assert_eq!(&output[0..6], &destination_mac.bytes());
        assert_eq!(&output[6..12], &endpoint.mac().bytes());
        assert_eq!(read_be_u16(&output, 12), ETHERTYPE_IPV4);
        assert_eq!(
            read_be_u16(
                &output[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..frame_len],
                4
            ),
            0xabcd
        );
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(destination_mac));
    }

    #[test_case]
    fn outbound_ipv4_icmp_echo_request_rejects_unresolved_neighbor_without_side_effects() {
        let mut output = [0xaa; 128];

        assert_eq!(
            build_outbound_ipv4_icmp_echo_request(
                OutboundNeighborResolution::Unresolved {
                    destination_ipv4: [192, 0, 2, 44],
                },
                local_endpoint(),
                1,
                2,
                64,
                &[1, 2, 3],
                &mut output,
            ),
            Err(OutboundFrameError::NeighborUnresolved {
                destination_ipv4: [192, 0, 2, 44],
            })
        );
        assert_eq!(output, [0xaa; 128]);
    }

    #[test_case]
    fn outbound_ipv4_icmp_echo_request_rejects_small_output_without_partial_frame() {
        let payload = [1, 2, 3, 4];
        let required_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut output =
            [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN - 1];

        assert_eq!(
            build_outbound_ipv4_icmp_echo_request(
                OutboundNeighborResolution::Resolved {
                    destination_ipv4: [192, 0, 2, 10],
                    destination_mac: MacAddress::new([0x02, 0, 0, 0, 0, 10]),
                },
                local_endpoint(),
                1,
                2,
                64,
                &payload,
                &mut output,
            ),
            Err(OutboundFrameError::OutputBufferTooSmall {
                required_len,
                available_len: output.len(),
            })
        );
        assert_eq!(
            output,
            [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN - 1]
        );
    }

    #[test_case]
    fn outbound_ipv4_icmp_echo_request_rejects_payloads_too_large_for_ipv4() {
        let payload = [0u8; u16::MAX as usize - IPV4_MIN_HEADER_LEN - ICMP_ECHO_HEADER_LEN + 1];
        let mut output = [0xaa; 64];

        assert_eq!(
            build_outbound_ipv4_icmp_echo_request(
                OutboundNeighborResolution::Resolved {
                    destination_ipv4: [192, 0, 2, 10],
                    destination_mac: MacAddress::new([0x02, 0, 0, 0, 0, 10]),
                },
                local_endpoint(),
                1,
                2,
                64,
                &payload,
                &mut output,
            ),
            Err(OutboundFrameError::PayloadTooLarge {
                required_len: u16::MAX as usize + 1,
                max_len: u16::MAX as usize,
            })
        );
        assert_eq!(output, [0xaa; 64]);
    }

    #[test_case]
    fn outbound_request_selection_builds_icmp_for_resolved_neighbor_without_cache_mutation() {
        let endpoint = local_endpoint();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let payload = [0xaa, 0xbb, 0xcc];
        let mut output = [0u8; 128];

        let selection = select_outbound_ipv4_icmp_echo_request(
            &cache,
            endpoint,
            [192, 0, 2, 10],
            0x1234,
            9,
            63,
            &payload,
            &mut output,
        )
        .expect("select resolved outbound request");

        assert_eq!(
            selection.request_kind(),
            OutboundRequestKind::Ipv4IcmpEchoRequest
        );
        assert_eq!(
            selection.frame_len(),
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len()
        );
        let frame =
            EthernetFrame::parse(&output[..selection.frame_len()]).expect("parse selected frame");
        assert_eq!(frame.destination(), destination_mac);
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Ipv4);

        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("parse selected ipv4");
        assert_eq!(ipv4.destination(), [192, 0, 2, 10]);
        assert_eq!(frame.payload()[8], 63);
        let icmp = ipv4.payload();
        assert_eq!(icmp[0], 8);
        assert_eq!(read_be_u16(icmp, 4), 0x1234);
        assert_eq!(read_be_u16(icmp, 6), 9);
        assert_eq!(&icmp[ICMP_ECHO_HEADER_LEN..], &payload);
        assert!(internet_checksum_is_valid(icmp));
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(destination_mac));
    }

    #[test_case]
    fn outbound_request_selection_builds_arp_for_unresolved_neighbor_without_cache_mutation() {
        let endpoint = local_endpoint();
        let cached_mac = MacAddress::new([0x02, 0, 0, 0, 0, 55]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 55], cached_mac),
            ArpCacheUpdate::Inserted
        );
        let mut output = [0u8; 64];

        let selection = select_outbound_ipv4_icmp_echo_request(
            &cache,
            endpoint,
            [192, 0, 2, 44],
            0x1234,
            9,
            63,
            &[1, 2, 3],
            &mut output,
        )
        .expect("select unresolved outbound request");

        assert_eq!(selection.request_kind(), OutboundRequestKind::ArpRequest);
        assert_eq!(
            selection.frame_len(),
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
        let frame =
            EthernetFrame::parse(&output[..selection.frame_len()]).expect("parse selected arp");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("parse selected arp");
        assert_eq!(arp.operation(), ArpOperation::Request);
        assert_eq!(arp.sender_hardware_address(), endpoint.mac());
        assert_eq!(arp.sender_protocol_address(), endpoint.ipv4());
        assert_eq!(arp.target_hardware_address(), MacAddress::new([0; 6]));
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 44]);
        assert_eq!(cache.lookup([192, 0, 2, 55]), Some(cached_mac));
        assert_eq!(cache.lookup([192, 0, 2, 44]), None);
    }

    #[test_case]
    fn outbound_request_selection_rejects_resolved_buffer_pressure_without_partial_frame() {
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let payload = [1, 2, 3, 4];
        let required_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut output = [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];

        assert_eq!(
            select_outbound_ipv4_icmp_echo_request(
                &cache,
                local_endpoint(),
                [192, 0, 2, 10],
                1,
                2,
                64,
                &payload,
                &mut output,
            ),
            Err(OutboundFrameError::OutputBufferTooSmall {
                required_len,
                available_len: output.len(),
            })
        );
        assert_eq!(output, [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN]);
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(destination_mac));
    }

    #[test_case]
    fn outbound_request_selection_rejects_unresolved_buffer_pressure_without_partial_frame() {
        let cache = ArpCache::<2>::new();
        let mut output = [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];

        assert_eq!(
            select_outbound_ipv4_icmp_echo_request(
                &cache,
                local_endpoint(),
                [192, 0, 2, 44],
                1,
                2,
                64,
                &[1, 2, 3],
                &mut output,
            ),
            Err(OutboundFrameError::OutputBufferTooSmall {
                required_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                available_len: output.len(),
            })
        );
        assert_eq!(
            output,
            [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1]
        );
        assert_eq!(cache.lookup([192, 0, 2, 44]), None);
    }

    #[test_case]
    fn outbound_request_selection_rejects_resolved_payloads_too_large_for_ipv4() {
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let payload = [0u8; u16::MAX as usize - IPV4_MIN_HEADER_LEN - ICMP_ECHO_HEADER_LEN + 1];
        let mut output = [0xaa; 64];

        assert_eq!(
            select_outbound_ipv4_icmp_echo_request(
                &cache,
                local_endpoint(),
                [192, 0, 2, 10],
                1,
                2,
                64,
                &payload,
                &mut output,
            ),
            Err(OutboundFrameError::PayloadTooLarge {
                required_len: u16::MAX as usize + 1,
                max_len: u16::MAX as usize,
            })
        );
        assert_eq!(output, [0xaa; 64]);
        assert_eq!(cache.lookup([192, 0, 2, 10]), Some(destination_mac));
    }

    #[test_case]
    fn ipv4_egress_route_policy_uses_destination_for_same_subnet_next_hop() {
        let endpoint = local_endpoint();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        let route =
            route_ipv4_egress(endpoint, policy, [192, 0, 2, 44]).expect("same-subnet route");

        assert_eq!(route.destination_ipv4(), [192, 0, 2, 44]);
        assert_eq!(route.next_hop_ipv4(), [192, 0, 2, 44]);
        assert_eq!(route.route_kind(), Ipv4EgressRouteKind::SameSubnet);
    }

    #[test_case]
    fn ipv4_egress_route_policy_uses_gateway_for_off_subnet_destination() {
        let endpoint = local_endpoint();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        let route = route_ipv4_egress(endpoint, policy, [198, 51, 100, 7]).expect("gateway route");

        assert_eq!(route.destination_ipv4(), [198, 51, 100, 7]);
        assert_eq!(route.next_hop_ipv4(), [192, 0, 2, 254]);
        assert_eq!(route.route_kind(), Ipv4EgressRouteKind::Gateway);
    }

    #[test_case]
    fn ipv4_egress_route_policy_reports_no_route_without_gateway() {
        let endpoint = local_endpoint();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);

        assert_eq!(
            route_ipv4_egress(endpoint, policy, [198, 51, 100, 7]),
            Err(OutboundRouteError::NoRouteToDestination {
                destination_ipv4: [198, 51, 100, 7],
            })
        );
    }

    #[test_case]
    fn ipv4_egress_route_policy_handles_zero_and_host_mask_boundaries() {
        let endpoint = local_endpoint();
        let zero_mask = Ipv4EgressRoutePolicy::new([0, 0, 0, 0], None);
        let host_mask = Ipv4EgressRoutePolicy::new([255, 255, 255, 255], Some([192, 0, 2, 254]));

        assert_eq!(
            route_ipv4_egress(endpoint, zero_mask, [203, 0, 113, 9])
                .expect("zero mask is direct")
                .next_hop_ipv4(),
            [203, 0, 113, 9]
        );
        assert_eq!(
            route_ipv4_egress(endpoint, host_mask, endpoint.ipv4())
                .expect("host route to local address")
                .route_kind(),
            Ipv4EgressRouteKind::SameSubnet
        );
        assert_eq!(
            route_ipv4_egress(endpoint, host_mask, [192, 0, 2, 44])
                .expect("host mask uses gateway for any other destination")
                .next_hop_ipv4(),
            [192, 0, 2, 254]
        );
    }

    #[test_case]
    fn routed_outbound_selection_resolves_gateway_mac_without_mutating_cache() {
        let endpoint = local_endpoint();
        let gateway_mac = MacAddress::new([0x02, 0, 0, 0, 0, 254]);
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 7]);
        let mut cache = ArpCache::<4>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 254], gateway_mac),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(
            cache.insert_or_update([198, 51, 100, 99], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let payload = [0x99, 0x88];
        let mut output = [0u8; 128];

        let selection = select_routed_outbound_ipv4_icmp_echo_request(
            &cache,
            endpoint,
            policy,
            [198, 51, 100, 7],
            0x2222,
            3,
            62,
            &payload,
            &mut output,
        )
        .expect("select routed icmp");

        assert_eq!(
            selection.request_kind(),
            OutboundRequestKind::Ipv4IcmpEchoRequest
        );
        let frame = EthernetFrame::parse(&output[..selection.frame_len()]).expect("routed frame");
        assert_eq!(frame.destination(), gateway_mac);
        assert_eq!(frame.source(), endpoint.mac());
        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("routed ipv4");
        assert_eq!(ipv4.destination(), [198, 51, 100, 7]);
        assert_eq!(frame.payload()[8], 62);
        assert_eq!(&ipv4.payload()[ICMP_ECHO_HEADER_LEN..], &payload);
        assert_eq!(cache.lookup([192, 0, 2, 254]), Some(gateway_mac));
        assert_eq!(cache.lookup([198, 51, 100, 7]), None);
        assert_eq!(cache.lookup([198, 51, 100, 99]), Some(destination_mac));
    }

    #[test_case]
    fn routed_outbound_selection_arps_for_unresolved_gateway_next_hop() {
        let endpoint = local_endpoint();
        let cache = ArpCache::<2>::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let mut output = [0u8; 64];

        let selection = select_routed_outbound_ipv4_icmp_echo_request(
            &cache,
            endpoint,
            policy,
            [198, 51, 100, 7],
            0x2222,
            3,
            62,
            &[1, 2],
            &mut output,
        )
        .expect("select gateway arp");

        assert_eq!(selection.request_kind(), OutboundRequestKind::ArpRequest);
        let frame = EthernetFrame::parse(&output[..selection.frame_len()]).expect("arp frame");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("arp");
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 254]);
        assert_eq!(cache.lookup([192, 0, 2, 254]), None);
        assert_eq!(cache.lookup([198, 51, 100, 7]), None);
    }

    #[test_case]
    fn routed_outbound_selection_reports_no_route_before_touching_output() {
        let endpoint = local_endpoint();
        let cache = ArpCache::<0>::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let mut output = [0xaa; 64];

        assert_eq!(
            select_routed_outbound_ipv4_icmp_echo_request(
                &cache,
                endpoint,
                policy,
                [198, 51, 100, 7],
                0x2222,
                3,
                62,
                &[1, 2],
                &mut output,
            ),
            Err(OutboundRouteError::NoRouteToDestination {
                destination_ipv4: [198, 51, 100, 7],
            })
        );
        assert_eq!(output, [0xaa; 64]);
    }

    #[test_case]
    fn routed_outbound_selection_wraps_frame_errors_without_cache_mutation() {
        let endpoint = local_endpoint();
        let gateway_mac = MacAddress::new([0x02, 0, 0, 0, 0, 254]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 254], gateway_mac),
            ArpCacheUpdate::Inserted
        );
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let payload = [1, 2, 3, 4];
        let required_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut output = [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];

        assert_eq!(
            select_routed_outbound_ipv4_icmp_echo_request(
                &cache,
                endpoint,
                policy,
                [198, 51, 100, 7],
                1,
                2,
                64,
                &payload,
                &mut output,
            ),
            Err(OutboundRouteError::Frame(
                OutboundFrameError::OutputBufferTooSmall {
                    required_len,
                    available_len: output.len(),
                },
            ))
        );
        assert_eq!(output, [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN]);
        assert_eq!(cache.lookup([192, 0, 2, 254]), Some(gateway_mac));
        assert_eq!(cache.lookup([198, 51, 100, 7]), None);
    }

    struct OutboundTransmitDevice {
        transmit_error: Option<DeviceError>,
        transmit_attempts: usize,
        transmitted: [u8; 128],
        transmitted_len: usize,
    }

    impl OutboundTransmitDevice {
        const fn new() -> Self {
            Self {
                transmit_error: None,
                transmit_attempts: 0,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        const fn with_transmit_error(error: DeviceError) -> Self {
            Self {
                transmit_error: Some(error),
                transmit_attempts: 0,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }
    }

    impl NetworkDevice for OutboundTransmitDevice {
        fn receive_frame<'a>(&mut self, _buffer: &'a mut [u8]) -> Result<&'a [u8], DeviceError> {
            Err(DeviceError::WouldBlock)
        }

        fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError> {
            self.transmit_attempts += 1;
            if let Some(error) = self.transmit_error {
                return Err(error);
            }

            self.transmitted[..frame.len()].copy_from_slice(frame);
            self.transmitted_len = frame.len();
            Ok(())
        }
    }

    #[test_case]
    fn outbound_one_shot_transmits_icmp_request_once_for_resolved_neighbor() {
        let endpoint = local_endpoint();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let payload = [0x11, 0x22, 0x33];
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();

        let result = transmit_one_outbound_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            endpoint,
            [192, 0, 2, 10],
            0x1234,
            9,
            63,
            &payload,
            &mut output,
        );

        assert_eq!(
            result,
            OutboundTransmitResult::Ipv4IcmpEchoRequestTransmitted {
                frame_len: expected_len,
            }
        );
        assert_eq!(device.transmit_attempts, 1);
        assert_eq!(device.transmitted_len, expected_len);
        assert_eq!(
            &device.transmitted[..device.transmitted_len],
            &output[..expected_len]
        );
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("frame");
        assert_eq!(frame.destination(), destination_mac);
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("ipv4");
        assert_eq!(ipv4.destination(), [192, 0, 2, 10]);
        assert_eq!(frame.payload()[8], 63);
        assert_eq!(&ipv4.payload()[ICMP_ECHO_HEADER_LEN..], &payload);
    }

    #[test_case]
    fn outbound_one_shot_transmits_arp_request_once_for_unresolved_neighbor() {
        let endpoint = local_endpoint();
        let cached_mac = MacAddress::new([0x02, 0, 0, 0, 0, 55]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 55], cached_mac),
            ArpCacheUpdate::Inserted
        );
        let mut output = [0u8; 64];
        let mut device = OutboundTransmitDevice::new();

        let result = transmit_one_outbound_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            endpoint,
            [192, 0, 2, 44],
            0x1234,
            9,
            63,
            &[1, 2, 3],
            &mut output,
        );

        assert_eq!(
            result,
            OutboundTransmitResult::ArpRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(device.transmit_attempts, 1);
        assert_eq!(
            device.transmitted_len,
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
        assert_eq!(
            &device.transmitted[..device.transmitted_len],
            &output[..device.transmitted_len]
        );
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("frame");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.source(), endpoint.mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("arp");
        assert_eq!(arp.operation(), ArpOperation::Request);
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 44]);
        assert_eq!(cache.lookup([192, 0, 2, 55]), Some(cached_mac));
        assert_eq!(cache.lookup([192, 0, 2, 44]), None);
    }

    #[test_case]
    fn outbound_one_shot_does_not_transmit_when_request_building_fails() {
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 10]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 10], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let payload = [1, 2, 3, 4];
        let required_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut output = [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        let mut device = OutboundTransmitDevice::new();

        let result = transmit_one_outbound_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            local_endpoint(),
            [192, 0, 2, 10],
            1,
            2,
            64,
            &payload,
            &mut output,
        );

        assert_eq!(
            result,
            OutboundTransmitResult::RequestError(OutboundFrameError::OutputBufferTooSmall {
                required_len,
                available_len: output.len(),
            })
        );
        assert_eq!(device.transmit_attempts, 0);
        assert_eq!(device.transmitted_len, 0);
        assert_eq!(output, [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN]);
    }

    #[test_case]
    fn outbound_one_shot_reports_transmit_error_after_successful_build() {
        let cache = ArpCache::<0>::new();
        let mut output = [0u8; 64];
        let mut device = OutboundTransmitDevice::with_transmit_error(DeviceError::Io);

        let result = transmit_one_outbound_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            local_endpoint(),
            [192, 0, 2, 44],
            1,
            2,
            64,
            &[1, 2, 3],
            &mut output,
        );

        assert_eq!(
            result,
            OutboundTransmitResult::TransmitError {
                request_kind: OutboundRequestKind::ArpRequest,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                error: DeviceError::Io,
            }
        );
        assert_eq!(device.transmit_attempts, 1);
        assert_eq!(device.transmitted_len, 0);
        let frame = EthernetFrame::parse(&output[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN])
            .expect("built frame remains in caller buffer");
        assert_eq!(frame.ether_type(), EtherType::Arp);
    }

    #[test_case]
    fn single_pending_icmp_unresolved_neighbor_transmits_arp_and_records_request() {
        let cache = ArpCache::<2>::new();
        let mut pending = SinglePendingIcmpEcho::<8>::new();
        let mut output = [0u8; 64];
        let mut device = OutboundTransmitDevice::new();
        let payload = [0x11, 0x22, 0x33];

        let result = transmit_or_queue_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            &mut pending,
            local_endpoint(),
            [192, 0, 2, 20],
            0x1234,
            7,
            61,
            &payload,
            &mut output,
        );

        assert_eq!(
            result,
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(device.transmit_attempts, 1);
        let request = pending.pending().expect("pending request stored");
        assert_eq!(request.endpoint(), local_endpoint());
        assert_eq!(request.destination_ipv4(), [192, 0, 2, 20]);
        assert_eq!(request.identifier(), 0x1234);
        assert_eq!(request.sequence_number(), 7);
        assert_eq!(request.ttl(), 61);
        assert_eq!(request.payload(), payload);

        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.source(), local_endpoint().mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("arp packet");
        assert_eq!(arp.operation(), ArpOperation::Request);
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 20]);
    }

    #[test_case]
    fn single_pending_icmp_matching_arp_reply_transmits_icmp_and_clears_pending() {
        let mut cache = ArpCache::<2>::new();
        let mut pending = SinglePendingIcmpEcho::<8>::new();
        let mut arp_output = [0u8; 64];
        let mut device = OutboundTransmitDevice::new();
        let payload = [0x11, 0x22, 0x33];

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                61,
                &payload,
                &mut arp_output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let mut icmp_output = [0u8; 128];
        let result = learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &mut cache,
            &mut pending,
            &arp_reply_frame(),
            &mut icmp_output,
        );

        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        assert_eq!(
            result,
            PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                frame_len: expected_len,
            }
        );
        assert_eq!(device.transmit_attempts, 2);
        assert_eq!(pending.pending(), None);
        assert_eq!(
            cache.lookup([192, 0, 2, 20]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );

        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("icmp");
        assert_eq!(frame.destination(), MacAddress::new([0x02, 0, 0, 0, 0, 20]));
        assert_eq!(frame.source(), local_endpoint().mac());
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("ipv4");
        assert_eq!(ipv4.destination(), [192, 0, 2, 20]);
        assert_eq!(frame.payload()[8], 61);
        let icmp = ipv4.payload();
        assert_eq!(read_be_u16(icmp, 4), 0x1234);
        assert_eq!(read_be_u16(icmp, 6), 7);
        assert_eq!(&icmp[ICMP_ECHO_HEADER_LEN..], &payload);
        assert!(internet_checksum_is_valid(icmp));
    }

    #[test_case]
    fn single_pending_icmp_can_advance_from_existing_cache_resolution() {
        let mut cache = ArpCache::<2>::new();
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 20], destination_mac),
            ArpCacheUpdate::Inserted
        );
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let payload = [1, 2, 3, 4];

        let result = transmit_or_queue_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            &mut pending,
            local_endpoint(),
            [192, 0, 2, 20],
            0x1111,
            1,
            64,
            &payload,
            &mut output,
        );

        assert_eq!(
            result,
            PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN
                    + IPV4_MIN_HEADER_LEN
                    + ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            }
        );
        assert_eq!(pending.pending(), None);
        assert_eq!(device.transmit_attempts, 1);
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("icmp");
        assert_eq!(frame.destination(), destination_mac);
    }

    #[test_case]
    fn routed_single_pending_icmp_same_subnet_unresolved_arps_destination() {
        let cache = ArpCache::<2>::new();
        let mut pending = SinglePendingIcmpEcho::<8>::new();
        let mut output = [0u8; 64];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        let result = transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            &mut pending,
            local_endpoint(),
            policy,
            [192, 0, 2, 20],
            0x1234,
            7,
            61,
            &[0x11, 0x22],
            &mut output,
        );

        assert_eq!(
            result,
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        let request = pending.pending().expect("pending request stored");
        assert_eq!(request.destination_ipv4(), [192, 0, 2, 20]);
        assert_eq!(request.next_hop_ipv4(), [192, 0, 2, 20]);
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("arp packet");
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 20]);
    }

    #[test_case]
    fn routed_single_pending_icmp_gateway_route_arps_gateway_and_transmits_to_final_destination() {
        let mut cache = ArpCache::<4>::new();
        let mut pending = SinglePendingIcmpEcho::<8>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let payload = [0x11, 0x22, 0x33];

        let result = transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            &mut pending,
            local_endpoint(),
            policy,
            [198, 51, 100, 7],
            0x1234,
            7,
            61,
            &payload,
            &mut output,
        );

        assert_eq!(
            result,
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        let request = pending.pending().expect("gateway pending request stored");
        assert_eq!(request.destination_ipv4(), [198, 51, 100, 7]);
        assert_eq!(request.next_hop_ipv4(), [192, 0, 2, 254]);
        let arp_frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        let arp = ArpPacket::parse_ethernet_ipv4(arp_frame.payload()).expect("arp packet");
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 254]);

        let gateway_mac = MacAddress::new([0x02, 0, 0, 0, 0, 254]);
        let mut gateway_reply = arp_reply_frame();
        gateway_reply[6..12].copy_from_slice(&gateway_mac.bytes());
        gateway_reply[ETHERNET_HEADER_LEN + 8..ETHERNET_HEADER_LEN + 14]
            .copy_from_slice(&gateway_mac.bytes());
        gateway_reply[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18]
            .copy_from_slice(&[192, 0, 2, 254]);

        let result = learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &mut cache,
            &mut pending,
            &gateway_reply,
            &mut output,
        );

        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        assert_eq!(
            result,
            PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                frame_len: expected_len,
            }
        );
        assert_eq!(pending.pending(), None);
        assert_eq!(cache.lookup([192, 0, 2, 254]), Some(gateway_mac));
        assert_eq!(cache.lookup([198, 51, 100, 7]), None);
        let icmp_frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("icmp");
        assert_eq!(icmp_frame.destination(), gateway_mac);
        let ipv4 = Ipv4Packet::parse(icmp_frame.payload()).expect("ipv4");
        assert_eq!(ipv4.destination(), [198, 51, 100, 7]);
        assert_eq!(&ipv4.payload()[ICMP_ECHO_HEADER_LEN..], &payload);
    }

    #[test_case]
    fn routed_single_pending_icmp_reports_no_route_without_pending_or_transmit() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0xaa; 64];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);

        let result = transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request(
            &mut device,
            &cache,
            &mut pending,
            local_endpoint(),
            policy,
            [198, 51, 100, 7],
            1,
            2,
            64,
            &[1, 2],
            &mut output,
        );

        assert_eq!(
            result,
            PendingIcmpEchoResult::RouteError(OutboundRouteError::NoRouteToDestination {
                destination_ipv4: [198, 51, 100, 7],
            })
        );
        assert_eq!(pending.pending(), None);
        assert_eq!(device.transmit_attempts, 0);
        assert_eq!(output, [0xaa; 64]);
    }

    #[test_case]
    fn routed_single_pending_icmp_preserves_gateway_pending_on_nonmatching_arp() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        assert_eq!(
            transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                policy,
                [198, 51, 100, 7],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        assert_eq!(
            learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &mut ArpCache::<2>::new(),
                &mut pending,
                &arp_reply_frame(),
                &mut output,
            ),
            PendingIcmpEchoResult::NonMatchingArp {
                pending_destination_ipv4: [198, 51, 100, 7],
                arp_sender_ipv4: [192, 0, 2, 20],
            }
        );
        let request = pending.pending().expect("pending request preserved");
        assert_eq!(request.destination_ipv4(), [198, 51, 100, 7]);
        assert_eq!(request.next_hop_ipv4(), [192, 0, 2, 254]);
    }

    #[test_case]
    fn single_pending_arp_retry_reemits_stored_gateway_next_hop_and_decrements_budget() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        assert_eq!(
            transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                policy,
                [198, 51, 100, 7],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
                2,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            pending.pending().expect("pending").arp_retries_remaining(),
            2
        );

        assert_eq!(
            retry_single_pending_ipv4_icmp_echo_arp_request(&mut device, &mut pending, &mut output),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let request = pending.pending().expect("pending request preserved");
        assert_eq!(request.destination_ipv4(), [198, 51, 100, 7]);
        assert_eq!(request.next_hop_ipv4(), [192, 0, 2, 254]);
        assert_eq!(request.arp_retries_remaining(), 1);
        assert_eq!(device.transmit_attempts, 2);
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        assert_eq!(frame.destination(), MacAddress::new([0xff; 6]));
        assert_eq!(frame.source(), local_endpoint().mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(frame.payload()).expect("arp packet");
        assert_eq!(arp.target_protocol_address(), [192, 0, 2, 254]);
    }

    #[test_case]
    fn single_pending_arp_retry_reports_budget_exhaustion_without_clearing_pending() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        assert_eq!(
            transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                policy,
                [198, 51, 100, 7],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
                0,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        assert_eq!(
            retry_single_pending_ipv4_icmp_echo_arp_request(&mut device, &mut pending, &mut output),
            PendingIcmpEchoResult::ArpRetryBudgetExhausted {
                destination_ipv4: [198, 51, 100, 7],
                next_hop_ipv4: [192, 0, 2, 254],
            }
        );
        let request = pending.pending().expect("pending request preserved");
        assert_eq!(request.destination_ipv4(), [198, 51, 100, 7]);
        assert_eq!(request.next_hop_ipv4(), [192, 0, 2, 254]);
        assert_eq!(request.arp_retries_remaining(), 0);
        assert_eq!(device.transmit_attempts, 1);
    }

    #[test_case]
    fn single_pending_arp_retry_reports_no_pending_buffer_pressure_and_transmit_error_boundaries() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));

        assert_eq!(
            retry_single_pending_ipv4_icmp_echo_arp_request(&mut device, &mut pending, &mut output),
            PendingIcmpEchoResult::NoPendingRequest
        );

        assert_eq!(
            transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                policy,
                [198, 51, 100, 7],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
                1,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let mut small_output = [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            retry_single_pending_ipv4_icmp_echo_arp_request(
                &mut device,
                &mut pending,
                &mut small_output
            ),
            PendingIcmpEchoResult::RequestError(OutboundFrameError::OutputBufferTooSmall {
                required_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                available_len: small_output.len(),
            })
        );
        assert_eq!(
            pending.pending().expect("pending").arp_retries_remaining(),
            1
        );
        assert_eq!(device.transmit_attempts, 1);

        let mut failing_device = OutboundTransmitDevice::with_transmit_error(DeviceError::Io);
        assert_eq!(
            retry_single_pending_ipv4_icmp_echo_arp_request(
                &mut failing_device,
                &mut pending,
                &mut output
            ),
            PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::ArpRequest,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                error: DeviceError::Io,
            }
        );
        assert_eq!(
            pending.pending().expect("pending").arp_retries_remaining(),
            1
        );
        assert_eq!(failing_device.transmit_attempts, 1);
    }

    #[test_case]
    fn single_pending_icmp_reports_backpressure_without_replacing_pending_request() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 64];
        let mut device = OutboundTransmitDevice::new();

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 44],
                3,
                4,
                64,
                &[3, 4],
                &mut output,
            ),
            PendingIcmpEchoResult::PendingRequestAlreadyQueued {
                destination_ipv4: [192, 0, 2, 20],
            }
        );
        assert_eq!(device.transmit_attempts, 1);
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));
    }

    #[test_case]
    fn single_pending_icmp_reports_payload_and_output_pressure_without_state_change() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<2>::new();
        let mut output = [0xaa; 64];
        let mut device = OutboundTransmitDevice::new();

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                1,
                2,
                64,
                &[1, 2, 3],
                &mut output,
            ),
            PendingIcmpEchoResult::PendingPayloadTooLarge {
                required_len: 3,
                max_len: 2,
            }
        );
        assert_eq!(pending.pending(), None);
        assert_eq!(device.transmit_attempts, 0);
        assert_eq!(output, [0xaa; 64]);

        let mut small_output = [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                1,
                2,
                64,
                &[1, 2],
                &mut small_output,
            ),
            PendingIcmpEchoResult::RequestError(OutboundFrameError::OutputBufferTooSmall {
                required_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                available_len: small_output.len(),
            })
        );
        assert_eq!(pending.pending(), None);
        assert_eq!(device.transmit_attempts, 0);
        assert_eq!(
            small_output,
            [0xaa; ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1]
        );
    }

    #[test_case]
    fn single_pending_icmp_reports_no_pending_unresolved_and_nonmatching_arp_boundaries() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();

        assert_eq!(
            transmit_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                &mut output
            ),
            PendingIcmpEchoResult::NoPendingRequest
        );

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 44],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            transmit_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                &mut output
            ),
            PendingIcmpEchoResult::PendingNeighborUnresolved {
                destination_ipv4: [192, 0, 2, 44],
            }
        );

        let mut nonmatching = arp_reply_frame();
        nonmatching[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18]
            .copy_from_slice(&[192, 0, 2, 20]);
        assert_eq!(
            learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &mut ArpCache::<2>::new(),
                &mut pending,
                &nonmatching,
                &mut output,
            ),
            PendingIcmpEchoResult::NonMatchingArp {
                pending_destination_ipv4: [192, 0, 2, 44],
                arp_sender_ipv4: [192, 0, 2, 20],
            }
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 44]));
    }

    #[test_case]
    fn single_pending_icmp_reports_malformed_arp_and_transmit_errors_without_clearing_pending() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();

        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let truncated = &arp_reply_frame()[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        assert_eq!(
            learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut device,
                &mut ArpCache::<2>::new(),
                &mut pending,
                truncated,
                &mut output,
            ),
            PendingIcmpEchoResult::ArpError(PacketError::Truncated)
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut failing_device = OutboundTransmitDevice::with_transmit_error(DeviceError::Io);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut failing_device,
                &mut cache,
                &mut pending,
                &arp_reply_frame(),
                &mut output,
            ),
            PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 2,
                error: DeviceError::Io,
            }
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));
        assert_eq!(
            cache.lookup([192, 0, 2, 20]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
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
            resolve_outbound_neighbor(&cache, [192, 0, 2, 10]).destination_mac(),
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
