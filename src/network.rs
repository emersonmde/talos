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
pub(crate) enum SmoltcpDependencyCorePollResult {
    NoDeviceBound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SmoltcpDependencyCore {
    hardware_address: smoltcp::wire::EthernetAddress,
    ipv4_cidr: smoltcp::wire::Ipv4Cidr,
    tcp_state: smoltcp::socket::tcp::State,
}

impl SmoltcpDependencyCore {
    pub(crate) fn new(mac: MacAddress, ipv4: [u8; 4], prefix_len: u8) -> Self {
        Self {
            hardware_address: smoltcp::wire::EthernetAddress::from_bytes(&mac.bytes()),
            ipv4_cidr: smoltcp::wire::Ipv4Cidr::new(
                smoltcp::wire::Ipv4Address::new(ipv4[0], ipv4[1], ipv4[2], ipv4[3]),
                prefix_len,
            ),
            tcp_state: smoltcp::socket::tcp::State::Closed,
        }
    }

    pub(crate) const fn hardware_address(&self) -> smoltcp::wire::EthernetAddress {
        self.hardware_address
    }

    pub(crate) const fn ipv4_cidr(&self) -> smoltcp::wire::Ipv4Cidr {
        self.ipv4_cidr
    }

    pub(crate) const fn tcp_state(&self) -> smoltcp::socket::tcp::State {
        self.tcp_state
    }

    pub(crate) const fn poll_without_device(&self) -> SmoltcpDependencyCorePollResult {
        SmoltcpDependencyCorePollResult::NoDeviceBound
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SmoltcpPacketDeviceAdapterReceiveResult {
    Idle,
    Received { frame_len: usize },
    NoFrame,
    TransmitQueueFull,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SmoltcpPacketDeviceAdapterTransmitResult {
    Idle,
    Ready,
    Transmitted { frame_len: usize },
    TransmitQueueFull,
    FrameTooLarge { required_len: usize, max_len: usize },
    TransmitError(DeviceError),
}

pub(crate) struct SmoltcpPacketDeviceAdapter<
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
> {
    device: PacketQueueNetworkDevice<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
    receive_buffer: [u8; FRAME_CAPACITY],
    last_receive_result: SmoltcpPacketDeviceAdapterReceiveResult,
    last_transmit_result: SmoltcpPacketDeviceAdapterTransmitResult,
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            device: PacketQueueNetworkDevice::new(),
            receive_buffer: [0; FRAME_CAPACITY],
            last_receive_result: SmoltcpPacketDeviceAdapterReceiveResult::Idle,
            last_transmit_result: SmoltcpPacketDeviceAdapterTransmitResult::Idle,
        }
    }

    pub(crate) fn inject_received(&mut self, frame: &[u8]) -> Result<(), PacketQueueError> {
        self.device.inject_received(frame)
    }

    pub(crate) fn pop_transmitted(&mut self) -> Option<PacketQueueFrame<FRAME_CAPACITY>> {
        self.device.pop_transmitted()
    }

    pub(crate) fn received_len(&self) -> usize {
        self.device.received_len()
    }

    pub(crate) fn transmitted_len(&self) -> usize {
        self.device.transmitted_len()
    }

    pub(crate) fn set_receive_error(&mut self, error: Option<DeviceError>) {
        self.device.set_receive_error(error);
    }

    pub(crate) fn set_transmit_error(&mut self, error: Option<DeviceError>) {
        self.device.set_transmit_error(error);
    }

    pub(crate) const fn last_receive_result(&self) -> SmoltcpPacketDeviceAdapterReceiveResult {
        self.last_receive_result
    }

    pub(crate) const fn last_transmit_result(&self) -> SmoltcpPacketDeviceAdapterTransmitResult {
        self.last_transmit_result
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DriverPacketAdapterReceiveStep {
    NoFrame,
    Received { frame_len: usize },
    TransmitQueueFull,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DriverPacketAdapterTransmitStep {
    Transmitted { frame_len: usize },
    TransmitQueueFull,
    FrameTooLarge { required_len: usize, max_len: usize },
    TransmitError(DeviceError),
}

pub(crate) struct DriverPacketAdapter<
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
> {
    smoltcp_device: SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    DriverPacketAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            smoltcp_device: SmoltcpPacketDeviceAdapter::new(),
        }
    }

    pub(crate) fn inject_driver_rx(&mut self, frame: &[u8]) -> Result<(), PacketQueueError> {
        self.smoltcp_device.inject_received(frame)
    }

    pub(crate) fn pop_driver_tx(&mut self) -> Option<PacketQueueFrame<FRAME_CAPACITY>> {
        self.smoltcp_device.pop_transmitted()
    }

    pub(crate) fn driver_rx_len(&self) -> usize {
        self.smoltcp_device.received_len()
    }

    pub(crate) fn driver_tx_len(&self) -> usize {
        self.smoltcp_device.transmitted_len()
    }

    pub(crate) fn set_receive_error(&mut self, error: Option<DeviceError>) {
        self.smoltcp_device.set_receive_error(error);
    }

    pub(crate) fn set_transmit_error(&mut self, error: Option<DeviceError>) {
        self.smoltcp_device.set_transmit_error(error);
    }

    pub(crate) fn receive_one_for_smoltcp(
        &mut self,
        timestamp: smoltcp::time::Instant,
    ) -> DriverPacketAdapterReceiveStep {
        match <SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY> as smoltcp::phy::Device>::receive(
            &mut self.smoltcp_device,
            timestamp,
        ) {
            Some((rx, _tx)) => {
                let frame_len = smoltcp::phy::RxToken::consume(rx, |frame| frame.len());
                DriverPacketAdapterReceiveStep::Received { frame_len }
            }
            None => match self.smoltcp_device.last_receive_result() {
                SmoltcpPacketDeviceAdapterReceiveResult::Idle
                | SmoltcpPacketDeviceAdapterReceiveResult::NoFrame => {
                    DriverPacketAdapterReceiveStep::NoFrame
                }
                SmoltcpPacketDeviceAdapterReceiveResult::Received { frame_len } => {
                    DriverPacketAdapterReceiveStep::Received { frame_len }
                }
                SmoltcpPacketDeviceAdapterReceiveResult::TransmitQueueFull => {
                    DriverPacketAdapterReceiveStep::TransmitQueueFull
                }
                SmoltcpPacketDeviceAdapterReceiveResult::ReceiveBufferTooSmall => {
                    DriverPacketAdapterReceiveStep::ReceiveBufferTooSmall
                }
                SmoltcpPacketDeviceAdapterReceiveResult::ReceiveError(error) => {
                    DriverPacketAdapterReceiveStep::ReceiveError(error)
                }
            },
        }
    }

    pub(crate) fn transmit_one_from_smoltcp(
        &mut self,
        timestamp: smoltcp::time::Instant,
        frame: &[u8],
    ) -> DriverPacketAdapterTransmitStep {
        match <SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY> as smoltcp::phy::Device>::transmit(
            &mut self.smoltcp_device,
            timestamp,
        ) {
            Some(tx) => {
                smoltcp::phy::TxToken::consume(tx, frame.len(), |tx_buffer| {
                    if tx_buffer.len() == frame.len() {
                        tx_buffer.copy_from_slice(frame);
                    }
                });
                self.last_transmit_step()
            }
            None => self.last_transmit_step(),
        }
    }

    fn last_transmit_step(&self) -> DriverPacketAdapterTransmitStep {
        match self.smoltcp_device.last_transmit_result() {
            SmoltcpPacketDeviceAdapterTransmitResult::Idle
            | SmoltcpPacketDeviceAdapterTransmitResult::Ready => {
                DriverPacketAdapterTransmitStep::TransmitQueueFull
            }
            SmoltcpPacketDeviceAdapterTransmitResult::Transmitted { frame_len } => {
                DriverPacketAdapterTransmitStep::Transmitted { frame_len }
            }
            SmoltcpPacketDeviceAdapterTransmitResult::TransmitQueueFull => {
                DriverPacketAdapterTransmitStep::TransmitQueueFull
            }
            SmoltcpPacketDeviceAdapterTransmitResult::FrameTooLarge {
                required_len,
                max_len,
            } => DriverPacketAdapterTransmitStep::FrameTooLarge {
                required_len,
                max_len,
            },
            SmoltcpPacketDeviceAdapterTransmitResult::TransmitError(error) => {
                DriverPacketAdapterTransmitStep::TransmitError(error)
            }
        }
    }
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    smoltcp::phy::Device for DriverPacketAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    type RxToken<'a>
        = SmoltcpPacketDeviceRxToken<FRAME_CAPACITY>
    where
        Self: 'a;
    type TxToken<'a>
        = SmoltcpPacketDeviceTxToken<'a, RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
    where
        Self: 'a;

    fn receive(
        &mut self,
        timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        self.smoltcp_device.receive(timestamp)
    }

    fn transmit(&mut self, timestamp: smoltcp::time::Instant) -> Option<Self::TxToken<'_>> {
        self.smoltcp_device.transmit(timestamp)
    }

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        self.smoltcp_device.capabilities()
    }
}

pub(crate) struct SmoltcpPacketDeviceRxToken<const FRAME_CAPACITY: usize> {
    frame: PacketQueueFrame<FRAME_CAPACITY>,
}

impl<const FRAME_CAPACITY: usize> smoltcp::phy::RxToken
    for SmoltcpPacketDeviceRxToken<FRAME_CAPACITY>
{
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        f(self.frame.as_bytes())
    }
}

pub(crate) struct SmoltcpPacketDeviceTxToken<
    'a,
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
> {
    device: &'a mut PacketQueueNetworkDevice<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
    status: &'a mut SmoltcpPacketDeviceAdapterTransmitResult,
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    smoltcp::phy::TxToken
    for SmoltcpPacketDeviceTxToken<'_, RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut frame = [0; FRAME_CAPACITY];
        if len > FRAME_CAPACITY {
            let result = f(&mut frame[..0]);
            *self.status = SmoltcpPacketDeviceAdapterTransmitResult::FrameTooLarge {
                required_len: len,
                max_len: FRAME_CAPACITY,
            };
            return result;
        }

        let result = f(&mut frame[..len]);
        match self.device.transmit_frame(&frame[..len]) {
            Ok(()) => {
                *self.status =
                    SmoltcpPacketDeviceAdapterTransmitResult::Transmitted { frame_len: len };
            }
            Err(error) => {
                *self.status = SmoltcpPacketDeviceAdapterTransmitResult::TransmitError(error);
            }
        }
        result
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PacketQueueError {
    Full,
    FrameTooLarge { required_len: usize, max_len: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PacketQueueDriverPumpStep {
    NoFrame,
    Transmitted {
        frame_len: usize,
    },
    Received {
        frame_len: usize,
    },
    ReceiveQueueFull,
    ReceiveFrameTooLarge {
        required_len: usize,
        max_len: usize,
    },
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
    TransmitError {
        frame_len: usize,
        error: DeviceError,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PacketQueueFrame<const FRAME_CAPACITY: usize> {
    bytes: [u8; FRAME_CAPACITY],
    len: usize,
}

impl<const FRAME_CAPACITY: usize> PacketQueueFrame<FRAME_CAPACITY> {
    pub(crate) fn new(frame: &[u8]) -> Result<Self, PacketQueueError> {
        if frame.len() > FRAME_CAPACITY {
            return Err(PacketQueueError::FrameTooLarge {
                required_len: frame.len(),
                max_len: FRAME_CAPACITY,
            });
        }

        let mut bytes = [0; FRAME_CAPACITY];
        bytes[..frame.len()].copy_from_slice(frame);
        Ok(Self {
            bytes,
            len: frame.len(),
        })
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub(crate) const fn len(self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FixedPacketQueue<const QUEUE_CAPACITY: usize, const FRAME_CAPACITY: usize> {
    frames: [Option<PacketQueueFrame<FRAME_CAPACITY>>; QUEUE_CAPACITY],
    len: usize,
}

impl<const QUEUE_CAPACITY: usize, const FRAME_CAPACITY: usize>
    FixedPacketQueue<QUEUE_CAPACITY, FRAME_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            frames: [None; QUEUE_CAPACITY],
            len: 0,
        }
    }

    pub(crate) const fn len(self) -> usize {
        self.len
    }

    pub(crate) const fn is_full(self) -> bool {
        self.len == QUEUE_CAPACITY
    }

    pub(crate) fn push(&mut self, frame: &[u8]) -> Result<(), PacketQueueError> {
        if self.len == QUEUE_CAPACITY {
            return Err(PacketQueueError::Full);
        }

        self.frames[self.len] = Some(PacketQueueFrame::new(frame)?);
        self.len += 1;
        Ok(())
    }

    pub(crate) fn front(&self) -> Option<&PacketQueueFrame<FRAME_CAPACITY>> {
        self.frames.get(0).and_then(Option::as_ref)
    }

    pub(crate) fn pop(&mut self) -> Option<PacketQueueFrame<FRAME_CAPACITY>> {
        if self.len == 0 {
            return None;
        }

        let frame = self.frames[0].take();
        let mut index = 1;
        while index < self.len {
            self.frames[index - 1] = self.frames[index].take();
            index += 1;
        }
        self.len -= 1;
        frame
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PacketQueueNetworkDevice<
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
> {
    rx: FixedPacketQueue<RX_CAPACITY, FRAME_CAPACITY>,
    tx: FixedPacketQueue<TX_CAPACITY, FRAME_CAPACITY>,
    receive_error: Option<DeviceError>,
    transmit_error: Option<DeviceError>,
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    PacketQueueNetworkDevice<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            rx: FixedPacketQueue::new(),
            tx: FixedPacketQueue::new(),
            receive_error: None,
            transmit_error: None,
        }
    }

    pub(crate) fn inject_received(&mut self, frame: &[u8]) -> Result<(), PacketQueueError> {
        self.rx.push(frame)
    }

    pub(crate) fn pop_transmitted(&mut self) -> Option<PacketQueueFrame<FRAME_CAPACITY>> {
        self.tx.pop()
    }

    pub(crate) fn transmitted_len(&self) -> usize {
        self.tx.len()
    }

    pub(crate) fn received_len(&self) -> usize {
        self.rx.len()
    }

    pub(crate) fn set_receive_error(&mut self, error: Option<DeviceError>) {
        self.receive_error = error;
    }

    pub(crate) fn set_transmit_error(&mut self, error: Option<DeviceError>) {
        self.transmit_error = error;
    }

    pub(crate) fn pump_driver<D: NetworkDevice>(
        &mut self,
        driver: &mut D,
        receive_buffer: &mut [u8],
    ) -> PacketQueueDriverPumpStep {
        if let Some(frame) = self.tx.front().copied() {
            let frame_len = frame.len();
            return match driver.transmit_frame(frame.as_bytes()) {
                Ok(()) => {
                    let _ = self.tx.pop();
                    PacketQueueDriverPumpStep::Transmitted { frame_len }
                }
                Err(error) => PacketQueueDriverPumpStep::TransmitError { frame_len, error },
            };
        }

        if self.rx.is_full() {
            return PacketQueueDriverPumpStep::ReceiveQueueFull;
        }

        match driver.receive_frame(receive_buffer) {
            Ok(frame) => {
                let frame_len = frame.len();
                match self.rx.push(frame) {
                    Ok(()) => PacketQueueDriverPumpStep::Received { frame_len },
                    Err(PacketQueueError::FrameTooLarge {
                        required_len,
                        max_len,
                    }) => PacketQueueDriverPumpStep::ReceiveFrameTooLarge {
                        required_len,
                        max_len,
                    },
                    Err(PacketQueueError::Full) => PacketQueueDriverPumpStep::ReceiveQueueFull,
                }
            }
            Err(DeviceError::WouldBlock) => PacketQueueDriverPumpStep::NoFrame,
            Err(DeviceError::BufferTooSmall) => PacketQueueDriverPumpStep::ReceiveBufferTooSmall,
            Err(error) => PacketQueueDriverPumpStep::ReceiveError(error),
        }
    }
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize> NetworkDevice
    for PacketQueueNetworkDevice<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    fn receive_frame<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a [u8], DeviceError> {
        if let Some(error) = self.receive_error {
            return Err(error);
        }

        let frame = self.rx.front().ok_or(DeviceError::WouldBlock)?;
        if buffer.len() < frame.len() {
            return Err(DeviceError::BufferTooSmall);
        }

        let frame = self.rx.pop().expect("front frame remains queued");
        buffer[..frame.len()].copy_from_slice(frame.as_bytes());
        Ok(&buffer[..frame.len()])
    }

    fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError> {
        if let Some(error) = self.transmit_error {
            return Err(error);
        }

        self.tx.push(frame).map_err(|_| DeviceError::BufferTooSmall)
    }
}

impl<const RX_CAPACITY: usize, const TX_CAPACITY: usize, const FRAME_CAPACITY: usize>
    smoltcp::phy::Device for SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
{
    type RxToken<'a>
        = SmoltcpPacketDeviceRxToken<FRAME_CAPACITY>
    where
        Self: 'a;
    type TxToken<'a>
        = SmoltcpPacketDeviceTxToken<'a, RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>
    where
        Self: 'a;

    fn receive(
        &mut self,
        _timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        if self.device.transmitted_len() == TX_CAPACITY {
            self.last_receive_result = SmoltcpPacketDeviceAdapterReceiveResult::TransmitQueueFull;
            return None;
        }

        match self.device.receive_frame(&mut self.receive_buffer) {
            Ok(frame) => {
                let frame_len = frame.len();
                let frame = PacketQueueFrame::new(frame).expect("receive buffer bounds frame");
                self.last_receive_result =
                    SmoltcpPacketDeviceAdapterReceiveResult::Received { frame_len };
                self.last_transmit_result = SmoltcpPacketDeviceAdapterTransmitResult::Ready;
                Some((
                    SmoltcpPacketDeviceRxToken { frame },
                    SmoltcpPacketDeviceTxToken {
                        device: &mut self.device,
                        status: &mut self.last_transmit_result,
                    },
                ))
            }
            Err(DeviceError::WouldBlock) => {
                self.last_receive_result = SmoltcpPacketDeviceAdapterReceiveResult::NoFrame;
                None
            }
            Err(DeviceError::BufferTooSmall) => {
                self.last_receive_result =
                    SmoltcpPacketDeviceAdapterReceiveResult::ReceiveBufferTooSmall;
                None
            }
            Err(error) => {
                self.last_receive_result =
                    SmoltcpPacketDeviceAdapterReceiveResult::ReceiveError(error);
                None
            }
        }
    }

    fn transmit(&mut self, _timestamp: smoltcp::time::Instant) -> Option<Self::TxToken<'_>> {
        if self.device.transmitted_len() == TX_CAPACITY {
            self.last_transmit_result = SmoltcpPacketDeviceAdapterTransmitResult::TransmitQueueFull;
            return None;
        }

        self.last_transmit_result = SmoltcpPacketDeviceAdapterTransmitResult::Ready;
        Some(SmoltcpPacketDeviceTxToken {
            device: &mut self.device,
            status: &mut self.last_transmit_result,
        })
    }

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut capabilities = smoltcp::phy::DeviceCapabilities::default();
        capabilities.medium = smoltcp::phy::Medium::Ethernet;
        capabilities.max_transmission_unit = FRAME_CAPACITY;
        capabilities.max_burst_size = Some(1);
        capabilities
    }
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
pub(crate) enum InflightIcmpEchoResult {
    InflightRequestTracked,
    IcmpEchoReplyMatched { payload_len: usize },
    NoInflightRequest,
    InflightRequestAlreadyTracked { destination_ipv4: [u8; 4] },
    InflightPayloadTooLarge { required_len: usize, max_len: usize },
    NonMatchingIcmpEchoReply { destination_ipv4: [u8; 4] },
    ReplyError(PacketError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SinglePingTransactionStartResult {
    IcmpEchoRequestTransmitted { frame_len: usize },
    ArpRequestTransmittedAndPending { frame_len: usize },
    PendingResult(PendingIcmpEchoResult),
    InflightResult(InflightIcmpEchoResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SinglePingTransactionPollResult {
    NoTransaction,
    PendingResult(PendingIcmpEchoPollResult),
    InflightResult(InflightIcmpEchoPollResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SinglePingTransactionRetryResult {
    NoTransaction,
    PendingResult(PendingIcmpEchoResult),
    InflightResult(InflightIcmpEchoResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SinglePingTransactionTimeoutResult {
    NoTransaction,
    PendingTimedOut {
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
    },
    InflightTimedOut {
        destination_ipv4: [u8; 4],
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SinglePingTransactionStatus {
    Idle,
    PendingArp {
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
        arp_retries_remaining: usize,
    },
    Inflight {
        destination_ipv4: [u8; 4],
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InflightIcmpEchoRequest<const PAYLOAD_CAPACITY: usize> {
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    payload: [u8; PAYLOAD_CAPACITY],
    payload_len: usize,
}

impl<const PAYLOAD_CAPACITY: usize> InflightIcmpEchoRequest<PAYLOAD_CAPACITY> {
    pub(crate) fn new(
        endpoint: LocalNetworkEndpoint,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        payload: &[u8],
    ) -> Result<Self, InflightIcmpEchoResult> {
        if payload.len() > PAYLOAD_CAPACITY {
            return Err(InflightIcmpEchoResult::InflightPayloadTooLarge {
                required_len: payload.len(),
                max_len: PAYLOAD_CAPACITY,
            });
        }

        let mut stored_payload = [0; PAYLOAD_CAPACITY];
        stored_payload[..payload.len()].copy_from_slice(payload);

        Ok(Self {
            endpoint,
            destination_ipv4,
            identifier,
            sequence_number,
            payload: stored_payload,
            payload_len: payload.len(),
        })
    }

    pub(crate) const fn endpoint(self) -> LocalNetworkEndpoint {
        self.endpoint
    }

    pub(crate) const fn destination_ipv4(self) -> [u8; 4] {
        self.destination_ipv4
    }

    pub(crate) const fn identifier(self) -> u16 {
        self.identifier
    }

    pub(crate) const fn sequence_number(self) -> u16 {
        self.sequence_number
    }

    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload[..self.payload_len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SingleInflightIcmpEcho<const PAYLOAD_CAPACITY: usize> {
    inflight: Option<InflightIcmpEchoRequest<PAYLOAD_CAPACITY>>,
}

impl<const PAYLOAD_CAPACITY: usize> SingleInflightIcmpEcho<PAYLOAD_CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self { inflight: None }
    }

    pub(crate) const fn inflight(&self) -> Option<InflightIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.inflight
    }

    pub(crate) const fn inflight_destination_ipv4(&self) -> Option<[u8; 4]> {
        match self.inflight {
            Some(request) => Some(request.destination_ipv4()),
            None => None,
        }
    }

    fn store(
        &mut self,
        request: InflightIcmpEchoRequest<PAYLOAD_CAPACITY>,
    ) -> Result<(), InflightIcmpEchoResult> {
        if let Some(existing) = self.inflight {
            return Err(InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                destination_ipv4: existing.destination_ipv4(),
            });
        }

        self.inflight = Some(request);
        Ok(())
    }

    fn take(&mut self) -> Option<InflightIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.inflight.take()
    }

    fn restore(&mut self, request: InflightIcmpEchoRequest<PAYLOAD_CAPACITY>) {
        self.inflight = Some(request);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SinglePingTransaction<const PAYLOAD_CAPACITY: usize> {
    pending: SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    inflight: SingleInflightIcmpEcho<PAYLOAD_CAPACITY>,
}

impl<const PAYLOAD_CAPACITY: usize> SinglePingTransaction<PAYLOAD_CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self {
            pending: SinglePendingIcmpEcho::new(),
            inflight: SingleInflightIcmpEcho::new(),
        }
    }

    pub(crate) const fn pending(&self) -> Option<PendingIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.pending.pending()
    }

    pub(crate) const fn inflight(&self) -> Option<InflightIcmpEchoRequest<PAYLOAD_CAPACITY>> {
        self.inflight.inflight()
    }

    pub(crate) const fn pending_destination_ipv4(&self) -> Option<[u8; 4]> {
        self.pending.pending_destination_ipv4()
    }

    pub(crate) const fn inflight_destination_ipv4(&self) -> Option<[u8; 4]> {
        self.inflight.inflight_destination_ipv4()
    }

    pub(crate) fn status(&self) -> SinglePingTransactionStatus {
        if let Some(request) = self.pending.pending() {
            return SinglePingTransactionStatus::PendingArp {
                destination_ipv4: request.destination_ipv4(),
                next_hop_ipv4: request.next_hop_ipv4(),
                arp_retries_remaining: request.arp_retries_remaining(),
            };
        }

        if let Some(request) = self.inflight.inflight() {
            return SinglePingTransactionStatus::Inflight {
                destination_ipv4: request.destination_ipv4(),
            };
        }

        SinglePingTransactionStatus::Idle
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SinglePingPacketService<const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize>
{
    arp_cache: ArpCache<ARP_CAPACITY>,
    transaction: SinglePingTransaction<PAYLOAD_CAPACITY>,
}

impl<const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize>
    SinglePingPacketService<ARP_CAPACITY, PAYLOAD_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            arp_cache: ArpCache::new(),
            transaction: SinglePingTransaction::new(),
        }
    }

    pub(crate) const fn with_arp_cache(arp_cache: ArpCache<ARP_CAPACITY>) -> Self {
        Self {
            arp_cache,
            transaction: SinglePingTransaction::new(),
        }
    }

    pub(crate) const fn arp_cache(&self) -> &ArpCache<ARP_CAPACITY> {
        &self.arp_cache
    }

    pub(crate) fn status(&self) -> SinglePingTransactionStatus {
        self.transaction.status()
    }

    pub(crate) fn start_ping<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        endpoint: LocalNetworkEndpoint,
        route_policy: Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        transmit_buffer: &mut [u8],
        arp_retry_budget: usize,
    ) -> SinglePingTransactionStartResult {
        start_routed_single_ping_transaction_with_arp_retry_budget(
            device,
            &self.arp_cache,
            &mut self.transaction,
            endpoint,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            transmit_buffer,
            arp_retry_budget,
        )
    }

    pub(crate) fn pump<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        receive_buffer: &mut [u8],
        transmit_buffer: &mut [u8],
    ) -> SinglePingTransactionPollResult {
        poll_single_ping_transaction(
            device,
            &mut self.arp_cache,
            &mut self.transaction,
            receive_buffer,
            transmit_buffer,
        )
    }

    pub(crate) fn pump_received<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        received_frame: &[u8],
        transmit_buffer: &mut [u8],
    ) -> SinglePingTransactionPollResult {
        poll_single_ping_transaction_received(
            device,
            &mut self.arp_cache,
            &mut self.transaction,
            received_frame,
            transmit_buffer,
        )
    }

    pub(crate) fn retry_arp<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        transmit_buffer: &mut [u8],
    ) -> SinglePingTransactionRetryResult {
        retry_single_ping_transaction_arp_request(device, &mut self.transaction, transmit_buffer)
    }

    pub(crate) fn timeout(&mut self) -> SinglePingTransactionTimeoutResult {
        timeout_single_ping_transaction(&mut self.transaction)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UserspacePingOperationStatus {
    Idle,
    PendingArp {
        destination_ipv4: [u8; 4],
        next_hop_ipv4: [u8; 4],
        arp_retries_remaining: usize,
    },
    Inflight {
        destination_ipv4: [u8; 4],
    },
    Completed {
        destination_ipv4: [u8; 4],
        payload_len: usize,
    },
    TimedOut {
        destination_ipv4: [u8; 4],
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UserspacePingOperationStep {
    StartedPendingArp { frame_len: usize },
    StartedInflight { frame_len: usize },
    NoFrame,
    AdvancedToInflight { frame_len: usize },
    RetryTransmitted { frame_len: usize },
    Completed { payload_len: usize },
    TimedOut { destination_ipv4: [u8; 4] },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UserspacePingOperation<const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize> {
    service: SinglePingPacketService<ARP_CAPACITY, PAYLOAD_CAPACITY>,
    terminal_status: Option<UserspacePingOperationStatus>,
}

impl<const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize>
    UserspacePingOperation<ARP_CAPACITY, PAYLOAD_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            service: SinglePingPacketService::new(),
            terminal_status: None,
        }
    }

    pub(crate) const fn with_service(
        service: SinglePingPacketService<ARP_CAPACITY, PAYLOAD_CAPACITY>,
    ) -> Self {
        Self {
            service,
            terminal_status: None,
        }
    }

    pub(crate) const fn service(&self) -> &SinglePingPacketService<ARP_CAPACITY, PAYLOAD_CAPACITY> {
        &self.service
    }

    pub(crate) fn status(&self) -> UserspacePingOperationStatus {
        match self.service.status() {
            SinglePingTransactionStatus::Idle => self
                .terminal_status
                .unwrap_or(UserspacePingOperationStatus::Idle),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4,
                next_hop_ipv4,
                arp_retries_remaining,
            } => UserspacePingOperationStatus::PendingArp {
                destination_ipv4,
                next_hop_ipv4,
                arp_retries_remaining,
            },
            SinglePingTransactionStatus::Inflight { destination_ipv4 } => {
                UserspacePingOperationStatus::Inflight { destination_ipv4 }
            }
        }
    }

    pub(crate) fn start<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        endpoint: LocalNetworkEndpoint,
        route_policy: Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        transmit_buffer: &mut [u8],
        arp_retry_budget: usize,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.terminal_status = None;
        match self.service.start_ping(
            device,
            endpoint,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            transmit_buffer,
            arp_retry_budget,
        ) {
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted { frame_len } => {
                Ok(UserspacePingOperationStep::StartedInflight { frame_len })
            }
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending { frame_len } => {
                Ok(UserspacePingOperationStep::StartedPendingArp { frame_len })
            }
            SinglePingTransactionStartResult::PendingResult(result) => {
                userspace_step_from_pending_result(
                    result,
                    UserspacePendingSuccess::StartedPendingArp,
                )
            }
            SinglePingTransactionStartResult::InflightResult(result) => {
                userspace_step_from_inflight_result(result)
            }
        }
    }

    pub(crate) fn pump<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        receive_buffer: &mut [u8],
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        let before_status = self.status();
        match self.service.pump(device, receive_buffer, transmit_buffer) {
            SinglePingTransactionPollResult::NoTransaction => {
                Err(crate::posix::PosixError::InvalidArgument)
            }
            SinglePingTransactionPollResult::PendingResult(result) => {
                userspace_step_from_pending_poll_result(result)
            }
            SinglePingTransactionPollResult::InflightResult(result) => {
                self.inflight_poll_step(before_status, result)
            }
        }
    }

    pub(crate) fn pump_received<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        received_frame: &[u8],
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        let before_status = self.status();
        match self
            .service
            .pump_received(device, received_frame, transmit_buffer)
        {
            SinglePingTransactionPollResult::NoTransaction => {
                Err(crate::posix::PosixError::InvalidArgument)
            }
            SinglePingTransactionPollResult::PendingResult(result) => {
                userspace_step_from_pending_poll_result(result)
            }
            SinglePingTransactionPollResult::InflightResult(result) => {
                self.inflight_poll_step(before_status, result)
            }
        }
    }

    pub(crate) fn retry_arp<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        match self.service.retry_arp(device, transmit_buffer) {
            SinglePingTransactionRetryResult::NoTransaction => {
                Err(crate::posix::PosixError::InvalidArgument)
            }
            SinglePingTransactionRetryResult::PendingResult(result) => {
                userspace_step_from_pending_result(
                    result,
                    UserspacePendingSuccess::RetryTransmitted,
                )
            }
            SinglePingTransactionRetryResult::InflightResult(result) => {
                userspace_step_from_inflight_result(result)
            }
        }
    }

    pub(crate) fn timeout(
        &mut self,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        match self.service.timeout() {
            SinglePingTransactionTimeoutResult::NoTransaction => {
                Err(crate::posix::PosixError::InvalidArgument)
            }
            SinglePingTransactionTimeoutResult::PendingTimedOut {
                destination_ipv4, ..
            }
            | SinglePingTransactionTimeoutResult::InflightTimedOut { destination_ipv4 } => {
                self.terminal_status =
                    Some(UserspacePingOperationStatus::TimedOut { destination_ipv4 });
                Ok(UserspacePingOperationStep::TimedOut { destination_ipv4 })
            }
        }
    }

    fn inflight_poll_step(
        &mut self,
        before_status: UserspacePingOperationStatus,
        result: InflightIcmpEchoPollResult,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        match result {
            InflightIcmpEchoPollResult::ObservationResult(
                InflightIcmpEchoResult::IcmpEchoReplyMatched { payload_len },
            ) => {
                self.terminal_status = Some(UserspacePingOperationStatus::Completed {
                    destination_ipv4: terminal_destination_from_status(before_status),
                    payload_len,
                });
                Ok(UserspacePingOperationStep::Completed { payload_len })
            }
            result => userspace_step_from_inflight_poll_result(result),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkPingOperationDescriptor {
    raw: usize,
}

impl NetworkPingOperationDescriptor {
    pub(crate) const fn from_raw(raw: usize) -> Self {
        Self { raw }
    }

    pub(crate) const fn raw(self) -> usize {
        self.raw
    }
}

pub(crate) const SOCKET_DOMAIN_AF_INET: u64 = 2;
pub(crate) const SOCKET_TYPE_STREAM: u64 = 1;
pub(crate) const SOCKET_PROTOCOL_DEFAULT: u64 = 0;
pub(crate) const SOCKET_LISTEN_BACKLOG_MIN: u64 = 1;
pub(crate) const SOCKET_LISTEN_BACKLOG_MAX: u64 = 4;
pub(crate) const SOCKET_LISTEN_BACKLOG_CAPACITY: usize = 4;
pub(crate) const SOCKET_SYNTHETIC_LOCAL_IPV4_BE: u32 = 0x7f00_0001;
pub(crate) const SOCKET_SYNTHETIC_CLIENT_PORT_BASE: u16 = 49152;
pub(crate) const SOCKET_PAYLOAD_QUEUE_CAPACITY: usize = 64;
pub(crate) const SOCKET_CONNECTION_ID_START: u64 = 1;
const SMOLTCP_SOCKET_BRIDGE_FRAME_CAPACITY: usize = 256;
const SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY: usize = 8;
const SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY: usize = 128;
const SMOLTCP_SOCKET_BRIDGE_CLIENT_MAC: [u8; ETHERNET_ADDR_LEN] = [0x02, 0, 0, 0, 0, 0x10];
const SMOLTCP_SOCKET_BRIDGE_SERVER_MAC: [u8; ETHERNET_ADDR_LEN] = [0x02, 0, 0, 0, 0, 0x20];
const SMOLTCP_SOCKET_BRIDGE_CLIENT_IPV4: [u8; 4] = [192, 0, 2, 10];
const SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4: [u8; 4] = [192, 0, 2, 20];
const SMOLTCP_SOCKET_BRIDGE_PREFIX_LEN: u8 = 24;
const SMOLTCP_SOCKET_BRIDGE_CLIENT_PORT: u16 = SOCKET_SYNTHETIC_CLIENT_PORT_BASE;
const SMOLTCP_SOCKET_BRIDGE_SERVER_PORT: u16 = 8080;
const SMOLTCP_SOCKET_BRIDGE_MAX_STEPS: usize = 48;
const LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD: &[u8] = b"runtime-device";
pub(crate) const LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER: &str =
    "blocked-no-live-frame-provider";
pub(crate) const LIVE_PACKET_INGRESS_DISCRIMINATOR_DETERMINISTIC_HOST_ONLY: &str =
    "deterministic-driver-packet-adapter-host-only";
pub(crate) const LIVE_PACKET_INGRESS_DISCRIMINATOR_PROVIDER_LINK_NOT_READY: &str =
    "blocked-hardware-frame-provider-link-not-ready";
pub(crate) const LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING: &str =
    "blocked-runtime-prerequisite-missing";
pub(crate) const LIVE_PACKET_STIMULUS_CONTRACT_ID: &str =
    "phase12-bounded-packet-stimulus-contract-v1";
pub(crate) const LIVE_PACKET_STIMULUS_READY_CLASSIFICATION: &str =
    "bounded-packet-stimulus-contract-ready";
pub(crate) const LIVE_PACKET_STIMULUS_BLOCKED_CLASSIFICATION: &str =
    "blocked-bounded-packet-stimulus-prerequisite-missing";
pub(crate) const LIVE_PACKET_STIMULUS_PERMITTED_SOURCE: &str =
    "lab-network-peer-icmp-echo-to-documented-talos-pi5-target";
pub(crate) const LIVE_PACKET_STIMULUS_NONCE_STRATEGY: &str =
    "run-unique-ascii-nonce-in-icmp-echo-payload-retain-only-sha256-and-length";
pub(crate) const LIVE_PACKET_STIMULUS_PAYLOAD_REDACTION_POLICY: &str =
    "retain-protocol-length-nonce-sha256-and-descriptor-metadata-no-payload-bytes";
pub(crate) const LIVE_PACKET_STIMULUS_TIMING_WINDOW: &str =
    "after-runtime-ready-marker-and-serial-cursor-before-final-pre-restore-identity";
pub(crate) const LIVE_PACKET_STIMULUS_EXPECTED_REPORT_FIELDS: &[&str] = &[
    "contract-id",
    "permitted-stimulus-source",
    "nonce-sha256",
    "nonce-length",
    "stimulus-protocol",
    "descriptor-index",
    "frame-length",
    "ring-wrap",
    "rp1-descriptor-ring-classification",
    "host-only-frame-count",
    "live-packet-io-accepted",
];
pub(crate) const LIVE_PACKET_STIMULUS_DISTINGUISHING_RULES: &[&str] = &[
    "stimulus source must be the lab network peer, not DriverPacketAdapter injection",
    "run-unique nonce hash must be absent from deterministic host-only frames",
    "RP1 descriptor metadata must be recorded from the source-owned RX descriptor/ring handoff",
    "packet payload bytes are never retained",
    "deterministic host-only DriverPacketAdapter delivery remains a regression/control surface only",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketDescriptor {
    raw: usize,
}

impl NetworkSocketDescriptor {
    pub(crate) const fn from_raw(raw: usize) -> Self {
        Self { raw }
    }

    pub(crate) const fn raw(self) -> usize {
        self.raw
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Ipv4Endpoint {
    ipv4_be: u32,
    port: u16,
}

impl Ipv4Endpoint {
    pub(crate) const fn new(ipv4_be: u32, port: u16) -> Self {
        Self { ipv4_be, port }
    }

    pub(crate) const fn ipv4_be(self) -> u32 {
        self.ipv4_be
    }

    pub(crate) const fn port(self) -> u16 {
        self.port
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketPendingLocalPeer {
    client_owner: crate::scheduler::ProcessOwnerId,
    client_descriptor: NetworkSocketDescriptor,
    client_endpoint: Ipv4Endpoint,
    connection_id: u64,
}

impl NetworkSocketPendingLocalPeer {
    pub(crate) const fn new(
        client_owner: crate::scheduler::ProcessOwnerId,
        client_descriptor: NetworkSocketDescriptor,
        client_endpoint: Ipv4Endpoint,
        connection_id: u64,
    ) -> Self {
        Self {
            client_owner,
            client_descriptor,
            client_endpoint,
            connection_id,
        }
    }

    pub(crate) const fn client_owner(self) -> crate::scheduler::ProcessOwnerId {
        self.client_owner
    }

    pub(crate) const fn client_descriptor(self) -> NetworkSocketDescriptor {
        self.client_descriptor
    }

    pub(crate) const fn client_endpoint(self) -> Ipv4Endpoint {
        self.client_endpoint
    }

    pub(crate) const fn connection_id(self) -> u64 {
        self.connection_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketPendingQueue {
    peers: [Option<NetworkSocketPendingLocalPeer>; SOCKET_LISTEN_BACKLOG_CAPACITY],
    len: u8,
}

impl NetworkSocketPendingQueue {
    pub(crate) const fn new() -> Self {
        Self {
            peers: [None; SOCKET_LISTEN_BACKLOG_CAPACITY],
            len: 0,
        }
    }

    pub(crate) const fn len(self) -> u8 {
        self.len
    }

    const fn is_full(self, backlog: u8) -> bool {
        self.len >= backlog || self.len as usize >= SOCKET_LISTEN_BACKLOG_CAPACITY
    }

    fn push(
        &mut self,
        backlog: u8,
        peer: NetworkSocketPendingLocalPeer,
    ) -> Result<(), crate::posix::PosixError> {
        if self.is_full(backlog) {
            return Err(crate::posix::PosixError::NoSpace);
        }

        self.peers[self.len as usize] = Some(peer);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<NetworkSocketPendingLocalPeer, crate::posix::PosixError> {
        if self.len == 0 {
            return Err(crate::posix::PosixError::Again);
        }

        let peer = self.peers[0]
            .take()
            .expect("non-empty pending socket queue has a front peer");
        let mut index = 1;
        while index < self.len as usize {
            self.peers[index - 1] = self.peers[index].take();
            index += 1;
        }
        self.len -= 1;
        Ok(peer)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketPayloadQueue {
    bytes: [u8; SOCKET_PAYLOAD_QUEUE_CAPACITY],
    len: u8,
}

impl NetworkSocketPayloadQueue {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: [0; SOCKET_PAYLOAD_QUEUE_CAPACITY],
            len: 0,
        }
    }

    pub(crate) const fn len(self) -> usize {
        self.len as usize
    }

    pub(crate) const fn remaining_capacity(self) -> usize {
        SOCKET_PAYLOAD_QUEUE_CAPACITY - self.len()
    }

    fn push_all(&mut self, payload: &[u8]) -> Result<(), crate::posix::PosixError> {
        if payload.len() > self.remaining_capacity() {
            return Err(crate::posix::PosixError::NoSpace);
        }

        let start = self.len();
        let end = start + payload.len();
        self.bytes[start..end].copy_from_slice(payload);
        self.len = end as u8;
        Ok(())
    }

    fn peek(&self, dst: &mut [u8]) -> usize {
        let count = core::cmp::min(dst.len(), self.len());
        dst[..count].copy_from_slice(&self.bytes[..count]);
        count
    }

    fn consume(&mut self, count: usize) {
        let len = self.len();
        debug_assert!(count <= len);
        let remaining = len - count;
        self.bytes.copy_within(count..len, 0);
        self.len = remaining as u8;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NetworkSocketState {
    OpenUnbound,
    Bound {
        local_endpoint: Ipv4Endpoint,
    },
    Listening {
        local_endpoint: Ipv4Endpoint,
        backlog: u8,
        pending: NetworkSocketPendingQueue,
    },
    Connected {
        local_endpoint: Ipv4Endpoint,
        remote_endpoint: Ipv4Endpoint,
        connection_id: u64,
        recv_queue: NetworkSocketPayloadQueue,
    },
    Accepted {
        local_endpoint: Ipv4Endpoint,
        remote_endpoint: Ipv4Endpoint,
        connection_id: u64,
        recv_queue: NetworkSocketPayloadQueue,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketReadiness {
    bits: u32,
}

impl NetworkSocketReadiness {
    pub(crate) const EMPTY: Self = Self { bits: 0 };
    pub(crate) const READ: Self = Self { bits: 0x01 };
    pub(crate) const WRITE: Self = Self { bits: 0x02 };
    pub(crate) const HANGUP: Self = Self { bits: 0x04 };
    pub(crate) const ERROR: Self = Self { bits: 0x08 };

    pub(crate) const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    pub(crate) const fn bits(self) -> u32 {
        self.bits
    }

    pub(crate) const fn contains(self, other: Self) -> bool {
        self.bits & other.bits != 0
    }

    fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocket {
    owner: crate::scheduler::ProcessOwnerId,
    domain: u64,
    socket_type: u64,
    protocol: u64,
    state: NetworkSocketState,
}

impl NetworkSocket {
    const fn new(
        owner: crate::scheduler::ProcessOwnerId,
        domain: u64,
        socket_type: u64,
        protocol: u64,
    ) -> Self {
        Self {
            owner,
            domain,
            socket_type,
            protocol,
            state: NetworkSocketState::OpenUnbound,
        }
    }

    pub(crate) const fn owner(self) -> crate::scheduler::ProcessOwnerId {
        self.owner
    }

    pub(crate) const fn domain(self) -> u64 {
        self.domain
    }

    pub(crate) const fn socket_type(self) -> u64 {
        self.socket_type
    }

    pub(crate) const fn protocol(self) -> u64 {
        self.protocol
    }

    pub(crate) const fn state(self) -> NetworkSocketState {
        self.state
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SmoltcpSocketBridgeObservation {
    client_state: smoltcp::socket::tcp::State,
    server_state: smoltcp::socket::tcp::State,
    steps: usize,
    client_to_server_frames: usize,
    server_to_client_frames: usize,
    payload_len: usize,
}

impl SmoltcpSocketBridgeObservation {
    pub(crate) const fn client_state(self) -> smoltcp::socket::tcp::State {
        self.client_state
    }

    pub(crate) const fn server_state(self) -> smoltcp::socket::tcp::State {
        self.server_state
    }

    pub(crate) const fn steps(self) -> usize {
        self.steps
    }

    pub(crate) const fn client_to_server_frames(self) -> usize {
        self.client_to_server_frames
    }

    pub(crate) const fn server_to_client_frames(self) -> usize {
        self.server_to_client_frames
    }

    pub(crate) const fn payload_len(self) -> usize {
        self.payload_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SmoltcpSocketBridgeRecord {
    client_owner: crate::scheduler::ProcessOwnerId,
    client_descriptor: NetworkSocketDescriptor,
    listener_owner: crate::scheduler::ProcessOwnerId,
    listener_descriptor: NetworkSocketDescriptor,
    accepted_owner: Option<crate::scheduler::ProcessOwnerId>,
    accepted_descriptor: Option<NetworkSocketDescriptor>,
    connection_id: u64,
    handshake: SmoltcpSocketBridgeObservation,
    payload_transfers: u64,
    last_payload: SmoltcpSocketBridgeObservation,
}

impl SmoltcpSocketBridgeRecord {
    pub(crate) const fn connection_id(self) -> u64 {
        self.connection_id
    }

    pub(crate) const fn accepted_descriptor(self) -> Option<NetworkSocketDescriptor> {
        self.accepted_descriptor
    }

    pub(crate) const fn handshake(self) -> SmoltcpSocketBridgeObservation {
        self.handshake
    }

    pub(crate) const fn payload_transfers(self) -> u64 {
        self.payload_transfers
    }

    pub(crate) const fn last_payload(self) -> SmoltcpSocketBridgeObservation {
        self.last_payload
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTcpListenerDescriptorBoundary {
    AcceptedLocalSourceBoundary,
    BlockedNoDeviceInterfaceBinding,
    BlockedNoDescriptorBridge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTcpDeviceInterfaceOwnershipModel {
    NetworkOwnedSmoltcpInterfaceWithDriverPacketAdapterIngressAndDescriptorTableDelivery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTcpDeviceInterfaceBindingState {
    LocalSourceBoundaryDoesNotRequireDeviceInterface,
    BlockedMissingDeviceInterfaceBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTcpAcceptedConnectionDeliveryState {
    AcceptedLocalDescriptorDelivery,
    BlockedMissingDeviceInterfaceBinding,
    BlockedNoDescriptorBridge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveTcpNetworkDeviceRuntimeBindingState {
    AcceptedDeterministicDeviceInterfaceDelivery,
    BlockedMissingDescriptorDelivery,
    BlockedMissingDeviceInterfaceBinding,
    BlockedMissingHardwareFrameProvider,
    BlockedHardwareFrameProviderLinkNotReady,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTcpListenerDescriptorBoundaryReport {
    boundary: LiveTcpListenerDescriptorBoundary,
    ownership_model: LiveTcpDeviceInterfaceOwnershipModel,
    connection_id: u64,
    descriptor_bridge_established: bool,
    accepted_descriptor_attached: bool,
    payload_transfers: u64,
    last_payload_len: usize,
    device_interface_binding_state: LiveTcpDeviceInterfaceBindingState,
    device_interface_bound: bool,
    live_packet_io_accepted: bool,
    live_reachability_accepted: bool,
    remote_receipt_accepted: bool,
    compatibility_accepted: bool,
    ssh_ready: bool,
}

impl LiveTcpListenerDescriptorBoundaryReport {
    pub(crate) const fn boundary(self) -> LiveTcpListenerDescriptorBoundary {
        self.boundary
    }

    pub(crate) const fn ownership_model(self) -> LiveTcpDeviceInterfaceOwnershipModel {
        self.ownership_model
    }

    pub(crate) const fn connection_id(self) -> u64 {
        self.connection_id
    }

    pub(crate) const fn descriptor_bridge_established(self) -> bool {
        self.descriptor_bridge_established
    }

    pub(crate) const fn accepted_descriptor_attached(self) -> bool {
        self.accepted_descriptor_attached
    }

    pub(crate) const fn payload_transfers(self) -> u64 {
        self.payload_transfers
    }

    pub(crate) const fn last_payload_len(self) -> usize {
        self.last_payload_len
    }

    pub(crate) const fn device_interface_binding_state(self) -> LiveTcpDeviceInterfaceBindingState {
        self.device_interface_binding_state
    }

    pub(crate) const fn device_interface_bound(self) -> bool {
        self.device_interface_bound
    }

    pub(crate) const fn live_packet_io_accepted(self) -> bool {
        self.live_packet_io_accepted
    }

    pub(crate) const fn live_reachability_accepted(self) -> bool {
        self.live_reachability_accepted
    }

    pub(crate) const fn remote_receipt_accepted(self) -> bool {
        self.remote_receipt_accepted
    }

    pub(crate) const fn compatibility_accepted(self) -> bool {
        self.compatibility_accepted
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTcpListenerDescriptorAcceptReport {
    boundary: LiveTcpListenerDescriptorBoundaryReport,
    delivery_state: LiveTcpAcceptedConnectionDeliveryState,
    accepted_descriptor: Option<NetworkSocketDescriptor>,
    accepted_descriptor_state: Option<NetworkSocketState>,
    descriptor_facing_connection_delivered: bool,
    live_packet_io_accepted: bool,
    live_reachability_accepted: bool,
    remote_receipt_accepted: bool,
    compatibility_accepted: bool,
    ssh_ready: bool,
}

impl LiveTcpListenerDescriptorAcceptReport {
    pub(crate) const fn boundary(self) -> LiveTcpListenerDescriptorBoundaryReport {
        self.boundary
    }

    pub(crate) const fn delivery_state(self) -> LiveTcpAcceptedConnectionDeliveryState {
        self.delivery_state
    }

    pub(crate) const fn accepted_descriptor(self) -> Option<NetworkSocketDescriptor> {
        self.accepted_descriptor
    }

    pub(crate) const fn accepted_descriptor_state(self) -> Option<NetworkSocketState> {
        self.accepted_descriptor_state
    }

    pub(crate) const fn descriptor_facing_connection_delivered(self) -> bool {
        self.descriptor_facing_connection_delivered
    }

    pub(crate) const fn live_packet_io_accepted(self) -> bool {
        self.live_packet_io_accepted
    }

    pub(crate) const fn live_reachability_accepted(self) -> bool {
        self.live_reachability_accepted
    }

    pub(crate) const fn remote_receipt_accepted(self) -> bool {
        self.remote_receipt_accepted
    }

    pub(crate) const fn compatibility_accepted(self) -> bool {
        self.compatibility_accepted
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTcpNetworkDeviceRuntimeReport {
    accept_report: LiveTcpListenerDescriptorAcceptReport,
    binding_state: LiveTcpNetworkDeviceRuntimeBindingState,
    runtime_observation: Option<SmoltcpSocketBridgeObservation>,
    descriptor_facing_connection_delivered: bool,
    deterministic_device_interface_bound: bool,
    hardware_frame_provider_bound: bool,
    hardware_frame_provider_classification: Option<&'static str>,
    live_packet_ingress_discriminator_classification: &'static str,
    driver_packet_rx_frames: usize,
    driver_packet_tx_frames: usize,
    live_packet_io_accepted: bool,
    live_reachability_accepted: bool,
    remote_receipt_accepted: bool,
    compatibility_accepted: bool,
    ssh_ready: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTcpRuntimeMarkerRouteReport {
    runtime_report: LiveTcpNetworkDeviceRuntimeReport,
    marker_route_ready: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveTcpRp1DmaRxDescriptorRingHandoffReport {
    runtime_report: LiveTcpNetworkDeviceRuntimeReport,
    descriptor_ring_classification: &'static str,
    descriptor_ring_owner: &'static str,
    metadata_handoff_ready: bool,
    driver_packet_adapter_handoff_ready: bool,
    frame_metadata_len: usize,
    packet_payload_available: bool,
    redaction_policy: &'static str,
    live_packet_io_accepted: bool,
    live_reachability_accepted: bool,
    remote_receipt_accepted: bool,
    compatibility_accepted: bool,
    ssh_ready: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LivePacketStimulusContractReport {
    contract_id: &'static str,
    classification: &'static str,
    permitted_stimulus_source: &'static str,
    nonce_strategy: &'static str,
    payload_redaction_policy: &'static str,
    timing_window: &'static str,
    expected_report_fields: &'static [&'static str],
    distinguishing_rules: &'static [&'static str],
    descriptor_ring_handoff_ready: bool,
    deterministic_host_only_discriminator: &'static str,
    distinguishes_lab_stimulus_from_host_only: bool,
    packet_payload_retained: bool,
    live_packet_io_accepted: bool,
    live_reachability_accepted: bool,
    remote_receipt_accepted: bool,
    compatibility_accepted: bool,
    ssh_ready: bool,
}

impl LiveTcpRuntimeMarkerRouteReport {
    pub(crate) const fn runtime_report(self) -> LiveTcpNetworkDeviceRuntimeReport {
        self.runtime_report
    }

    pub(crate) const fn marker_route_ready(self) -> bool {
        self.marker_route_ready
    }
}

impl LiveTcpRp1DmaRxDescriptorRingHandoffReport {
    pub(crate) const fn runtime_report(self) -> LiveTcpNetworkDeviceRuntimeReport {
        self.runtime_report
    }

    pub(crate) const fn descriptor_ring_classification(self) -> &'static str {
        self.descriptor_ring_classification
    }

    pub(crate) const fn descriptor_ring_owner(self) -> &'static str {
        self.descriptor_ring_owner
    }

    pub(crate) const fn metadata_handoff_ready(self) -> bool {
        self.metadata_handoff_ready
    }

    pub(crate) const fn driver_packet_adapter_handoff_ready(self) -> bool {
        self.driver_packet_adapter_handoff_ready
    }

    pub(crate) const fn frame_metadata_len(self) -> usize {
        self.frame_metadata_len
    }

    pub(crate) const fn packet_payload_available(self) -> bool {
        self.packet_payload_available
    }

    pub(crate) const fn redaction_policy(self) -> &'static str {
        self.redaction_policy
    }

    pub(crate) const fn live_packet_io_accepted(self) -> bool {
        self.live_packet_io_accepted
    }

    pub(crate) const fn live_reachability_accepted(self) -> bool {
        self.live_reachability_accepted
    }

    pub(crate) const fn remote_receipt_accepted(self) -> bool {
        self.remote_receipt_accepted
    }

    pub(crate) const fn compatibility_accepted(self) -> bool {
        self.compatibility_accepted
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

impl LivePacketStimulusContractReport {
    pub(crate) const fn contract_id(self) -> &'static str {
        self.contract_id
    }

    pub(crate) const fn classification(self) -> &'static str {
        self.classification
    }

    pub(crate) const fn permitted_stimulus_source(self) -> &'static str {
        self.permitted_stimulus_source
    }

    pub(crate) const fn nonce_strategy(self) -> &'static str {
        self.nonce_strategy
    }

    pub(crate) const fn payload_redaction_policy(self) -> &'static str {
        self.payload_redaction_policy
    }

    pub(crate) const fn timing_window(self) -> &'static str {
        self.timing_window
    }

    pub(crate) const fn expected_report_fields(self) -> &'static [&'static str] {
        self.expected_report_fields
    }

    pub(crate) const fn distinguishing_rules(self) -> &'static [&'static str] {
        self.distinguishing_rules
    }

    pub(crate) const fn descriptor_ring_handoff_ready(self) -> bool {
        self.descriptor_ring_handoff_ready
    }

    pub(crate) const fn deterministic_host_only_discriminator(self) -> &'static str {
        self.deterministic_host_only_discriminator
    }

    pub(crate) const fn distinguishes_lab_stimulus_from_host_only(self) -> bool {
        self.distinguishes_lab_stimulus_from_host_only
    }

    pub(crate) const fn packet_payload_retained(self) -> bool {
        self.packet_payload_retained
    }

    pub(crate) const fn live_packet_io_accepted(self) -> bool {
        self.live_packet_io_accepted
    }

    pub(crate) const fn live_reachability_accepted(self) -> bool {
        self.live_reachability_accepted
    }

    pub(crate) const fn remote_receipt_accepted(self) -> bool {
        self.remote_receipt_accepted
    }

    pub(crate) const fn compatibility_accepted(self) -> bool {
        self.compatibility_accepted
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

impl LiveTcpNetworkDeviceRuntimeReport {
    pub(crate) const fn accept_report(self) -> LiveTcpListenerDescriptorAcceptReport {
        self.accept_report
    }

    pub(crate) const fn binding_state(self) -> LiveTcpNetworkDeviceRuntimeBindingState {
        self.binding_state
    }

    pub(crate) const fn runtime_observation(self) -> Option<SmoltcpSocketBridgeObservation> {
        self.runtime_observation
    }

    pub(crate) const fn descriptor_facing_connection_delivered(self) -> bool {
        self.descriptor_facing_connection_delivered
    }

    pub(crate) const fn deterministic_device_interface_bound(self) -> bool {
        self.deterministic_device_interface_bound
    }

    pub(crate) const fn hardware_frame_provider_bound(self) -> bool {
        self.hardware_frame_provider_bound
    }

    pub(crate) const fn hardware_frame_provider_classification(self) -> Option<&'static str> {
        self.hardware_frame_provider_classification
    }

    pub(crate) const fn live_packet_ingress_discriminator_classification(self) -> &'static str {
        self.live_packet_ingress_discriminator_classification
    }

    pub(crate) const fn driver_packet_rx_frames(self) -> usize {
        self.driver_packet_rx_frames
    }

    pub(crate) const fn driver_packet_tx_frames(self) -> usize {
        self.driver_packet_tx_frames
    }

    pub(crate) const fn live_packet_io_accepted(self) -> bool {
        self.live_packet_io_accepted
    }

    pub(crate) const fn live_reachability_accepted(self) -> bool {
        self.live_reachability_accepted
    }

    pub(crate) const fn remote_receipt_accepted(self) -> bool {
        self.remote_receipt_accepted
    }

    pub(crate) const fn compatibility_accepted(self) -> bool {
        self.compatibility_accepted
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

fn live_tcp_runtime_marker_route_report_for_rp1_provider(
    provider_report: Option<crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderBindingReport>,
) -> Result<LiveTcpRuntimeMarkerRouteReport, crate::posix::PosixError> {
    let owner = crate::scheduler::ProcessOwnerId::new(79)
        .ok_or(crate::posix::PosixError::InvalidArgument)?;
    let endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, 22);
    let mut sockets = NetworkSocketDescriptorTable::<4>::new();
    let listener = sockets.open(
        owner,
        SOCKET_DOMAIN_AF_INET,
        SOCKET_TYPE_STREAM,
        SOCKET_PROTOCOL_DEFAULT,
    )?;
    sockets.bind(owner, listener, endpoint)?;
    sockets.listen(owner, listener, 1)?;
    let client = sockets.open(
        owner,
        SOCKET_DOMAIN_AF_INET,
        SOCKET_TYPE_STREAM,
        SOCKET_PROTOCOL_DEFAULT,
    )?;
    sockets.connect(owner, client, endpoint)?;
    let connection_id = match sockets.socket(client)?.state() {
        NetworkSocketState::Connected { connection_id, .. } => connection_id,
        _ => return Err(crate::posix::PosixError::Pipe),
    };
    let accepted = sockets.accept(owner, listener)?;
    sockets.send(owner, client, LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD)?;
    let mut recv = [0u8; LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD.len()];
    if sockets.recv_peek(owner, accepted, &mut recv)?
        != LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD.len()
        || recv != *LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD
    {
        return Err(crate::posix::PosixError::Io);
    }

    let runtime_report = match provider_report {
        Some(provider_report) => sockets
            .live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
                connection_id,
                true,
                provider_report,
            )?,
        None => {
            sockets.live_tcp_network_device_smoltcp_runtime_binding(connection_id, true, false)?
        }
    };
    let provider_route_ready = match provider_report {
        Some(provider_report) => {
            provider_report.provider_bound()
                && provider_report.link_ready()
                && runtime_report.hardware_frame_provider_bound()
                && runtime_report.hardware_frame_provider_classification()
                    == Some(provider_report.classification)
        }
        None => {
            !runtime_report.hardware_frame_provider_bound()
                && runtime_report
                    .hardware_frame_provider_classification()
                    .is_none()
        }
    };
    let marker_route_ready = runtime_report.binding_state()
        == LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        && runtime_report.descriptor_facing_connection_delivered()
        && runtime_report.deterministic_device_interface_bound()
        && provider_route_ready
        && runtime_report.driver_packet_rx_frames() > 0
        && runtime_report.driver_packet_rx_frames() == runtime_report.driver_packet_tx_frames()
        && !runtime_report.live_packet_io_accepted()
        && !runtime_report.live_reachability_accepted()
        && !runtime_report.remote_receipt_accepted()
        && !runtime_report.compatibility_accepted()
        && !runtime_report.ssh_ready();

    Ok(LiveTcpRuntimeMarkerRouteReport {
        runtime_report,
        marker_route_ready,
    })
}

pub(crate) fn live_tcp_runtime_marker_route_report()
-> Result<LiveTcpRuntimeMarkerRouteReport, crate::posix::PosixError> {
    live_tcp_runtime_marker_route_report_for_rp1_provider(None)
}

pub(crate) fn live_tcp_runtime_marker_route_report_with_source_bound_rp1_provider()
-> Result<LiveTcpRuntimeMarkerRouteReport, crate::posix::PosixError> {
    let provider_report =
        crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(Some(
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_contract_evidence(
                crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderState::SourceBoundLinkReady,
            ),
        ));
    live_tcp_runtime_marker_route_report_for_rp1_provider(Some(provider_report))
}

fn live_tcp_runtime_marker_route_report_with_rp1_dma_rx_descriptor_ring_report(
    ring_report: crate::rp1_ethernet::Rp1EthernetDmaRxDescriptorRingReport,
) -> Result<LiveTcpRp1DmaRxDescriptorRingHandoffReport, crate::posix::PosixError> {
    let provider_report =
        crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(Some(
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_contract_evidence(
                crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderState::SourceBoundLinkReady,
            ),
        ));
    let route = live_tcp_runtime_marker_route_report_for_rp1_provider(Some(provider_report))?;
    let frame_metadata_len = match ring_report.frame_metadata() {
        Some(metadata) => metadata.frame_len,
        None => 0,
    };
    let metadata_handoff_ready = ring_report.metadata_handoff_ready()
        && route.marker_route_ready()
        && frame_metadata_len > 0
        && !ring_report.packet_payload_available;

    Ok(LiveTcpRp1DmaRxDescriptorRingHandoffReport {
        runtime_report: route.runtime_report(),
        descriptor_ring_classification: ring_report.classification,
        descriptor_ring_owner: ring_report.contract.owner,
        metadata_handoff_ready,
        driver_packet_adapter_handoff_ready: metadata_handoff_ready,
        frame_metadata_len,
        packet_payload_available: ring_report.packet_payload_available,
        redaction_policy: ring_report.contract.redaction_policy,
        live_packet_io_accepted: false,
        live_reachability_accepted: false,
        remote_receipt_accepted: false,
        compatibility_accepted: false,
        ssh_ready: false,
    })
}

pub(crate) fn live_tcp_runtime_marker_route_report_with_source_owned_rp1_dma_rx_descriptor_ring()
-> Result<LiveTcpRp1DmaRxDescriptorRingHandoffReport, crate::posix::PosixError> {
    let ring_report = crate::rp1_ethernet::rp1_ethernet_dma_rx_descriptor_ring_report(
        crate::rp1_ethernet::Rp1EthernetDmaRxDescriptorRingState::SourceOwnedCompletedFrame,
        Some(crate::rp1_ethernet::rp1_ethernet_dma_rx_frame_metadata(
            0, 64, false,
        )),
    );
    live_tcp_runtime_marker_route_report_with_rp1_dma_rx_descriptor_ring_report(ring_report)
}

fn bounded_packet_stimulus_contract_report_for_handoff(
    handoff: LiveTcpRp1DmaRxDescriptorRingHandoffReport,
) -> LivePacketStimulusContractReport {
    let ready = handoff.metadata_handoff_ready()
        && handoff.driver_packet_adapter_handoff_ready()
        && !handoff.packet_payload_available()
        && !handoff.live_packet_io_accepted()
        && !handoff.remote_receipt_accepted()
        && handoff
            .runtime_report()
            .live_packet_ingress_discriminator_classification()
            == LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER;
    LivePacketStimulusContractReport {
        contract_id: LIVE_PACKET_STIMULUS_CONTRACT_ID,
        classification: if ready {
            LIVE_PACKET_STIMULUS_READY_CLASSIFICATION
        } else {
            LIVE_PACKET_STIMULUS_BLOCKED_CLASSIFICATION
        },
        permitted_stimulus_source: LIVE_PACKET_STIMULUS_PERMITTED_SOURCE,
        nonce_strategy: LIVE_PACKET_STIMULUS_NONCE_STRATEGY,
        payload_redaction_policy: LIVE_PACKET_STIMULUS_PAYLOAD_REDACTION_POLICY,
        timing_window: LIVE_PACKET_STIMULUS_TIMING_WINDOW,
        expected_report_fields: LIVE_PACKET_STIMULUS_EXPECTED_REPORT_FIELDS,
        distinguishing_rules: LIVE_PACKET_STIMULUS_DISTINGUISHING_RULES,
        descriptor_ring_handoff_ready: ready,
        deterministic_host_only_discriminator:
            LIVE_PACKET_INGRESS_DISCRIMINATOR_DETERMINISTIC_HOST_ONLY,
        distinguishes_lab_stimulus_from_host_only: ready,
        packet_payload_retained: false,
        live_packet_io_accepted: false,
        live_reachability_accepted: false,
        remote_receipt_accepted: false,
        compatibility_accepted: false,
        ssh_ready: false,
    }
}

pub(crate) fn live_tcp_bounded_packet_stimulus_contract_report()
-> Result<LivePacketStimulusContractReport, crate::posix::PosixError> {
    let handoff =
        live_tcp_runtime_marker_route_report_with_source_owned_rp1_dma_rx_descriptor_ring()?;
    Ok(bounded_packet_stimulus_contract_report_for_handoff(handoff))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkSocketDescriptorTable<const CAPACITY: usize> {
    entries: [Option<NetworkSocket>; CAPACITY],
    smoltcp_bridges: [Option<SmoltcpSocketBridgeRecord>; CAPACITY],
    next_connection_id: u64,
}

impl<const CAPACITY: usize> NetworkSocketDescriptorTable<CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            smoltcp_bridges: [None; CAPACITY],
            next_connection_id: SOCKET_CONNECTION_ID_START,
        }
    }

    pub(crate) fn open(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        domain: u64,
        socket_type: u64,
        protocol: u64,
    ) -> Result<NetworkSocketDescriptor, crate::posix::PosixError> {
        if domain != SOCKET_DOMAIN_AF_INET
            || socket_type != SOCKET_TYPE_STREAM
            || protocol != SOCKET_PROTOCOL_DEFAULT
        {
            return Err(crate::posix::PosixError::NotSupported);
        }

        let mut raw = 0;
        while raw < CAPACITY {
            if self.entries[raw].is_none() {
                self.entries[raw] = Some(NetworkSocket::new(owner, domain, socket_type, protocol));
                return Ok(NetworkSocketDescriptor::from_raw(raw));
            }
            raw += 1;
        }

        Err(crate::posix::PosixError::NoSpace)
    }

    pub(crate) fn close(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        self.entries[descriptor.raw()] = None;
        self.remove_smoltcp_bridge_for_descriptor(owner, descriptor);
        Ok(())
    }

    pub(crate) fn close_owner(&mut self, owner: crate::scheduler::ProcessOwnerId) -> usize {
        let mut closed = 0usize;
        let mut raw = 0usize;
        while raw < CAPACITY {
            if let Some(socket) = self.entries[raw] {
                if socket.owner == owner {
                    self.remove_smoltcp_bridge_for_descriptor(
                        owner,
                        NetworkSocketDescriptor::from_raw(raw),
                    );
                    self.entries[raw] = None;
                    closed += 1;
                }
            }
            raw += 1;
        }
        closed
    }

    pub(crate) fn bind(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        local_endpoint: Ipv4Endpoint,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let socket = self
            .entries
            .get_mut(descriptor.raw())
            .and_then(Option::as_mut)
            .ok_or(crate::posix::PosixError::BadDescriptor)?;
        match socket.state {
            NetworkSocketState::OpenUnbound => {
                socket.state = NetworkSocketState::Bound { local_endpoint };
                Ok(())
            }
            NetworkSocketState::Bound { .. }
            | NetworkSocketState::Listening { .. }
            | NetworkSocketState::Connected { .. }
            | NetworkSocketState::Accepted { .. } => Err(crate::posix::PosixError::InvalidArgument),
        }
    }

    pub(crate) fn listen(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        backlog: u8,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let state = self.socket(descriptor)?.state;
        match state {
            NetworkSocketState::Bound { local_endpoint } => {
                if self.active_listener_endpoint_in_use(descriptor, local_endpoint) {
                    return Err(crate::posix::PosixError::Exists);
                }
                let socket = self
                    .entries
                    .get_mut(descriptor.raw())
                    .and_then(Option::as_mut)
                    .ok_or(crate::posix::PosixError::BadDescriptor)?;
                socket.state = NetworkSocketState::Listening {
                    local_endpoint,
                    backlog,
                    pending: NetworkSocketPendingQueue::new(),
                };
                Ok(())
            }
            NetworkSocketState::Listening {
                local_endpoint,
                pending,
                ..
            } => {
                if pending.len() > backlog {
                    return Err(crate::posix::PosixError::InvalidArgument);
                }
                if self.active_listener_endpoint_in_use(descriptor, local_endpoint) {
                    return Err(crate::posix::PosixError::Exists);
                }
                let socket = self
                    .entries
                    .get_mut(descriptor.raw())
                    .and_then(Option::as_mut)
                    .ok_or(crate::posix::PosixError::BadDescriptor)?;
                socket.state = NetworkSocketState::Listening {
                    local_endpoint,
                    backlog,
                    pending,
                };
                Ok(())
            }
            NetworkSocketState::OpenUnbound => Err(crate::posix::PosixError::InvalidArgument),
            NetworkSocketState::Connected { .. } | NetworkSocketState::Accepted { .. } => {
                Err(crate::posix::PosixError::InvalidArgument)
            }
        }
    }

    pub(crate) fn connect(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        client_descriptor: NetworkSocketDescriptor,
        remote_endpoint: Ipv4Endpoint,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, client_descriptor)?;
        let client = self.socket(client_descriptor)?;
        if client.state != NetworkSocketState::OpenUnbound {
            return Err(crate::posix::PosixError::InvalidArgument);
        }

        let client_port = synthetic_client_port(client_descriptor)?;
        let client_endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, client_port);
        let mut listener_descriptor = None;
        let mut match_count = 0usize;
        let mut raw = 0;
        while raw < CAPACITY {
            if let Some(socket) = self.entries[raw] {
                if let NetworkSocketState::Listening { local_endpoint, .. } = socket.state {
                    if local_endpoint == remote_endpoint {
                        listener_descriptor = Some(NetworkSocketDescriptor::from_raw(raw));
                        match_count += 1;
                    }
                }
            }
            raw += 1;
        }
        if match_count != 1 {
            return Err(crate::posix::PosixError::InvalidArgument);
        }

        let listener_descriptor =
            listener_descriptor.expect("single listener match records a descriptor");
        let mut listener = self.socket(listener_descriptor)?;
        let NetworkSocketState::Listening {
            local_endpoint,
            backlog,
            mut pending,
        } = listener.state
        else {
            return Err(crate::posix::PosixError::InvalidArgument);
        };
        let connection_id = self.allocate_connection_id()?;
        pending.push(
            backlog,
            NetworkSocketPendingLocalPeer::new(
                owner,
                client_descriptor,
                client_endpoint,
                connection_id,
            ),
        )?;
        let bridge_record = self.create_smoltcp_bridge_record(
            owner,
            client_descriptor,
            listener.owner(),
            listener_descriptor,
            connection_id,
        )?;

        listener.state = NetworkSocketState::Listening {
            local_endpoint,
            backlog,
            pending,
        };
        self.entries[listener_descriptor.raw()] = Some(listener);

        let mut connected_client = client;
        connected_client.state = NetworkSocketState::Connected {
            local_endpoint: client_endpoint,
            remote_endpoint,
            connection_id,
            recv_queue: NetworkSocketPayloadQueue::new(),
        };
        self.entries[client_descriptor.raw()] = Some(connected_client);
        self.insert_smoltcp_bridge_record(bridge_record)?;
        Ok(())
    }

    pub(crate) fn accept(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        listener_descriptor: NetworkSocketDescriptor,
    ) -> Result<NetworkSocketDescriptor, crate::posix::PosixError> {
        self.require_owner(owner, listener_descriptor)?;
        let mut listener = self.socket(listener_descriptor)?;
        let NetworkSocketState::Listening {
            local_endpoint,
            backlog,
            mut pending,
        } = listener.state
        else {
            return Err(crate::posix::PosixError::InvalidArgument);
        };
        if pending.len() == 0 {
            return Err(crate::posix::PosixError::Again);
        }
        let mut accepted_raw = None;
        let mut raw = 0;
        while raw < CAPACITY {
            if self.entries[raw].is_none() {
                accepted_raw = Some(raw);
                break;
            }
            raw += 1;
        }
        let accepted_raw = accepted_raw.ok_or(crate::posix::PosixError::NoSpace)?;
        let mut selected_peer = None;
        while pending.len() != 0 {
            let peer = pending.pop()?;
            if self.pending_peer_is_connected(peer) {
                selected_peer = Some(peer);
                break;
            }
        }

        listener.state = NetworkSocketState::Listening {
            local_endpoint,
            backlog,
            pending,
        };
        self.entries[listener_descriptor.raw()] = Some(listener);
        let peer = selected_peer.ok_or(crate::posix::PosixError::Again)?;
        self.entries[accepted_raw] = Some(NetworkSocket {
            owner,
            domain: SOCKET_DOMAIN_AF_INET,
            socket_type: SOCKET_TYPE_STREAM,
            protocol: SOCKET_PROTOCOL_DEFAULT,
            state: NetworkSocketState::Accepted {
                local_endpoint,
                remote_endpoint: peer.client_endpoint(),
                connection_id: peer.connection_id(),
                recv_queue: NetworkSocketPayloadQueue::new(),
            },
        });
        self.attach_smoltcp_accepted_descriptor(
            peer.connection_id(),
            owner,
            NetworkSocketDescriptor::from_raw(accepted_raw),
        )?;
        Ok(NetworkSocketDescriptor::from_raw(accepted_raw))
    }

    pub(crate) fn send(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        payload: &[u8],
    ) -> Result<usize, crate::posix::PosixError> {
        self.send_ready(owner, descriptor, payload.len())?;
        if payload.is_empty() {
            return Ok(0);
        }

        let socket = self.socket(descriptor)?;
        let (local_endpoint, remote_endpoint, connection_id) = connected_endpoints(socket.state)?;
        let peer_descriptor =
            self.unique_peer_descriptor(connection_id, local_endpoint, remote_endpoint)?;
        self.record_smoltcp_payload_transfer(connection_id, payload)?;
        let mut peer = self.socket(peer_descriptor)?;
        connected_recv_queue_mut(&mut peer.state)?.push_all(payload)?;
        self.entries[peer_descriptor.raw()] = Some(peer);
        Ok(payload.len())
    }

    pub(crate) fn send_ready(
        &self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        len: usize,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let socket = self.socket(descriptor)?;
        let (local_endpoint, remote_endpoint, connection_id) = connected_endpoints(socket.state)?;
        if len == 0 {
            return Ok(());
        }
        if len > SOCKET_PAYLOAD_QUEUE_CAPACITY {
            return Err(crate::posix::PosixError::NoSpace);
        }

        let peer_descriptor =
            self.unique_peer_descriptor(connection_id, local_endpoint, remote_endpoint)?;
        let peer = self.socket(peer_descriptor)?;
        let recv_queue = connected_recv_queue(peer.state)?;
        if len > recv_queue.remaining_capacity() {
            return Err(crate::posix::PosixError::NoSpace);
        }
        Ok(())
    }

    pub(crate) fn recv_peek(
        &self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        dst: &mut [u8],
    ) -> Result<usize, crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let socket = self.socket(descriptor)?;
        let (local_endpoint, remote_endpoint, connection_id) = connected_endpoints(socket.state)?;
        if dst.is_empty() {
            return Ok(0);
        }

        let recv_queue = connected_recv_queue(socket.state)?;
        if recv_queue.len() != 0 {
            return Ok(recv_queue.peek(dst));
        }
        match self.unique_peer_descriptor(connection_id, local_endpoint, remote_endpoint) {
            Ok(_) => Err(crate::posix::PosixError::Again),
            Err(crate::posix::PosixError::Pipe) => Err(crate::posix::PosixError::Pipe),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn recv_commit(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        count: usize,
    ) -> Result<(), crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let mut socket = self.socket(descriptor)?;
        connected_recv_queue_mut(&mut socket.state)?.consume(count);
        self.entries[descriptor.raw()] = Some(socket);
        Ok(())
    }

    pub(crate) fn readiness(
        &self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
        requested: NetworkSocketReadiness,
    ) -> Result<NetworkSocketReadiness, crate::posix::PosixError> {
        self.require_owner(owner, descriptor)?;
        let socket = self.socket(descriptor)?;
        let mut readiness = NetworkSocketReadiness::EMPTY;

        match socket.state {
            NetworkSocketState::OpenUnbound | NetworkSocketState::Bound { .. } => {}
            NetworkSocketState::Listening { pending, .. } => {
                if self.pending_queue_has_connected_peer(pending)
                    && requested.contains(NetworkSocketReadiness::READ)
                {
                    readiness.insert(NetworkSocketReadiness::READ);
                }
            }
            NetworkSocketState::Connected {
                local_endpoint,
                remote_endpoint,
                connection_id,
                recv_queue,
            }
            | NetworkSocketState::Accepted {
                local_endpoint,
                remote_endpoint,
                connection_id,
                recv_queue,
            } => {
                let peer_descriptor =
                    self.unique_peer_descriptor(connection_id, local_endpoint, remote_endpoint);
                if recv_queue.len() != 0 || peer_descriptor.is_err() {
                    if requested.contains(NetworkSocketReadiness::READ) {
                        readiness.insert(NetworkSocketReadiness::READ);
                    }
                }

                match peer_descriptor {
                    Ok(peer_descriptor) => {
                        if requested.contains(NetworkSocketReadiness::WRITE) {
                            let peer = self.socket(peer_descriptor)?;
                            let peer_recv_queue = connected_recv_queue(peer.state)?;
                            if peer_recv_queue.remaining_capacity() != 0 {
                                readiness.insert(NetworkSocketReadiness::WRITE);
                            }
                        }
                    }
                    Err(_) => readiness.insert(NetworkSocketReadiness::HANGUP),
                }
            }
        }

        Ok(readiness)
    }

    pub(crate) fn socket(
        &self,
        descriptor: NetworkSocketDescriptor,
    ) -> Result<NetworkSocket, crate::posix::PosixError> {
        self.entries
            .get(descriptor.raw())
            .and_then(|entry| *entry)
            .ok_or(crate::posix::PosixError::BadDescriptor)
    }

    pub(crate) fn smoltcp_bridge_record(
        &self,
        connection_id: u64,
    ) -> Result<SmoltcpSocketBridgeRecord, crate::posix::PosixError> {
        self.smoltcp_bridges
            .iter()
            .flatten()
            .copied()
            .find(|record| record.connection_id == connection_id)
            .ok_or(crate::posix::PosixError::BadDescriptor)
    }

    pub(crate) fn live_tcp_listener_descriptor_boundary(
        &self,
        connection_id: u64,
        require_device_interface_binding: bool,
    ) -> Result<LiveTcpListenerDescriptorBoundaryReport, crate::posix::PosixError> {
        let record = self.smoltcp_bridge_record(connection_id)?;
        let descriptor_bridge_established = record.handshake().client_state()
            == smoltcp::socket::tcp::State::Established
            && record.handshake().server_state() == smoltcp::socket::tcp::State::Established;
        let accepted_descriptor_attached = record.accepted_descriptor().is_some();
        let boundary = if !descriptor_bridge_established || !accepted_descriptor_attached {
            LiveTcpListenerDescriptorBoundary::BlockedNoDescriptorBridge
        } else if require_device_interface_binding {
            LiveTcpListenerDescriptorBoundary::BlockedNoDeviceInterfaceBinding
        } else {
            LiveTcpListenerDescriptorBoundary::AcceptedLocalSourceBoundary
        };
        let device_interface_binding_state = if require_device_interface_binding {
            LiveTcpDeviceInterfaceBindingState::BlockedMissingDeviceInterfaceBinding
        } else {
            LiveTcpDeviceInterfaceBindingState::LocalSourceBoundaryDoesNotRequireDeviceInterface
        };

        Ok(LiveTcpListenerDescriptorBoundaryReport {
            boundary,
            ownership_model: LiveTcpDeviceInterfaceOwnershipModel::NetworkOwnedSmoltcpInterfaceWithDriverPacketAdapterIngressAndDescriptorTableDelivery,
            connection_id,
            descriptor_bridge_established,
            accepted_descriptor_attached,
            payload_transfers: record.payload_transfers(),
            last_payload_len: record.last_payload().payload_len(),
            device_interface_binding_state,
            device_interface_bound: false,
            live_packet_io_accepted: false,
            live_reachability_accepted: false,
            remote_receipt_accepted: false,
            compatibility_accepted: false,
            ssh_ready: false,
        })
    }

    pub(crate) fn live_tcp_listener_descriptor_accept_delivery(
        &self,
        connection_id: u64,
        require_device_interface_binding: bool,
    ) -> Result<LiveTcpListenerDescriptorAcceptReport, crate::posix::PosixError> {
        let boundary = self.live_tcp_listener_descriptor_boundary(
            connection_id,
            require_device_interface_binding,
        )?;
        let record = self.smoltcp_bridge_record(connection_id)?;
        let accepted_descriptor = record.accepted_descriptor();
        let accepted_descriptor_state = accepted_descriptor
            .and_then(|descriptor| self.socket(descriptor).ok().map(|socket| socket.state()));
        let descriptor_facing_connection_delivered = matches!(
            accepted_descriptor_state,
            Some(NetworkSocketState::Accepted {
                connection_id: accepted_connection_id,
                ..
            }) if accepted_connection_id == connection_id
        );
        let delivery_state = match boundary.boundary() {
            LiveTcpListenerDescriptorBoundary::AcceptedLocalSourceBoundary
                if descriptor_facing_connection_delivered =>
            {
                LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
            }
            LiveTcpListenerDescriptorBoundary::BlockedNoDeviceInterfaceBinding => {
                LiveTcpAcceptedConnectionDeliveryState::BlockedMissingDeviceInterfaceBinding
            }
            LiveTcpListenerDescriptorBoundary::AcceptedLocalSourceBoundary
            | LiveTcpListenerDescriptorBoundary::BlockedNoDescriptorBridge => {
                LiveTcpAcceptedConnectionDeliveryState::BlockedNoDescriptorBridge
            }
        };
        let descriptor_facing_connection_delivered = delivery_state
            == LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery;

        Ok(LiveTcpListenerDescriptorAcceptReport {
            boundary,
            delivery_state,
            accepted_descriptor,
            accepted_descriptor_state,
            descriptor_facing_connection_delivered,
            live_packet_io_accepted: false,
            live_reachability_accepted: false,
            remote_receipt_accepted: false,
            compatibility_accepted: false,
            ssh_ready: false,
        })
    }

    pub(crate) fn live_tcp_network_device_smoltcp_runtime_binding(
        &self,
        connection_id: u64,
        bind_deterministic_device_interface: bool,
        require_hardware_frame_provider: bool,
    ) -> Result<LiveTcpNetworkDeviceRuntimeReport, crate::posix::PosixError> {
        let accept_report =
            self.live_tcp_listener_descriptor_accept_delivery(connection_id, false)?;
        if accept_report.delivery_state()
            != LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
        {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDescriptorDelivery,
                runtime_observation: None,
                descriptor_facing_connection_delivered: false,
                deterministic_device_interface_bound: false,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: None,
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        if !bind_deterministic_device_interface {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDeviceInterfaceBinding,
                runtime_observation: None,
                descriptor_facing_connection_delivered: true,
                deterministic_device_interface_bound: false,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: None,
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        if require_hardware_frame_provider {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingHardwareFrameProvider,
                runtime_observation: None,
                descriptor_facing_connection_delivered: true,
                deterministic_device_interface_bound: true,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: Some(
                    crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_MISSING_CLASSIFICATION,
                ),
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        let runtime_observation =
            driver_packet_smoltcp_listener_transfer(LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD)?;

        Ok(LiveTcpNetworkDeviceRuntimeReport {
            accept_report,
            binding_state: LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery,
            runtime_observation: Some(runtime_observation),
            descriptor_facing_connection_delivered: true,
            deterministic_device_interface_bound: true,
            hardware_frame_provider_bound: false,
            hardware_frame_provider_classification: None,
            live_packet_ingress_discriminator_classification:
                LIVE_PACKET_INGRESS_DISCRIMINATOR_DETERMINISTIC_HOST_ONLY,
            driver_packet_rx_frames: runtime_observation.client_to_server_frames()
                + runtime_observation.server_to_client_frames(),
            driver_packet_tx_frames: runtime_observation.client_to_server_frames()
                + runtime_observation.server_to_client_frames(),
            live_packet_io_accepted: false,
            live_reachability_accepted: false,
            remote_receipt_accepted: false,
            compatibility_accepted: false,
            ssh_ready: false,
        })
    }

    pub(crate) fn live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
        &self,
        connection_id: u64,
        bind_deterministic_device_interface: bool,
        provider_report: crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderBindingReport,
    ) -> Result<LiveTcpNetworkDeviceRuntimeReport, crate::posix::PosixError> {
        let accept_report =
            self.live_tcp_listener_descriptor_accept_delivery(connection_id, false)?;
        if accept_report.delivery_state()
            != LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
        {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDescriptorDelivery,
                runtime_observation: None,
                descriptor_facing_connection_delivered: false,
                deterministic_device_interface_bound: false,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: None,
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        if !bind_deterministic_device_interface {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDeviceInterfaceBinding,
                runtime_observation: None,
                descriptor_facing_connection_delivered: true,
                deterministic_device_interface_bound: false,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: None,
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        if !provider_report.provider_bound() {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingHardwareFrameProvider,
                runtime_observation: None,
                descriptor_facing_connection_delivered: true,
                deterministic_device_interface_bound: true,
                hardware_frame_provider_bound: false,
                hardware_frame_provider_classification: Some(provider_report.classification),
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        if !provider_report.link_ready() {
            return Ok(LiveTcpNetworkDeviceRuntimeReport {
                accept_report,
                binding_state:
                    LiveTcpNetworkDeviceRuntimeBindingState::BlockedHardwareFrameProviderLinkNotReady,
                runtime_observation: None,
                descriptor_facing_connection_delivered: true,
                deterministic_device_interface_bound: true,
                hardware_frame_provider_bound: true,
                hardware_frame_provider_classification: Some(provider_report.classification),
                live_packet_ingress_discriminator_classification:
                    LIVE_PACKET_INGRESS_DISCRIMINATOR_PROVIDER_LINK_NOT_READY,
                driver_packet_rx_frames: 0,
                driver_packet_tx_frames: 0,
                live_packet_io_accepted: false,
                live_reachability_accepted: false,
                remote_receipt_accepted: false,
                compatibility_accepted: false,
                ssh_ready: false,
            });
        }

        let runtime_observation =
            driver_packet_smoltcp_listener_transfer(LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD)?;

        Ok(LiveTcpNetworkDeviceRuntimeReport {
            accept_report,
            binding_state:
                LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery,
            runtime_observation: Some(runtime_observation),
            descriptor_facing_connection_delivered: true,
            deterministic_device_interface_bound: true,
            hardware_frame_provider_bound: true,
            hardware_frame_provider_classification: Some(provider_report.classification),
            live_packet_ingress_discriminator_classification:
                LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER,
            driver_packet_rx_frames: runtime_observation.client_to_server_frames()
                + runtime_observation.server_to_client_frames(),
            driver_packet_tx_frames: runtime_observation.client_to_server_frames()
                + runtime_observation.server_to_client_frames(),
            live_packet_io_accepted: false,
            live_reachability_accepted: false,
            remote_receipt_accepted: false,
            compatibility_accepted: false,
            ssh_ready: false,
        })
    }

    pub(crate) fn require_owner(
        &self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
    ) -> Result<(), crate::posix::PosixError> {
        let socket = self.socket(descriptor)?;
        if socket.owner() == owner {
            Ok(())
        } else {
            Err(crate::posix::PosixError::BadDescriptor)
        }
    }

    fn unique_peer_descriptor(
        &self,
        connection_id: u64,
        local_endpoint: Ipv4Endpoint,
        remote_endpoint: Ipv4Endpoint,
    ) -> Result<NetworkSocketDescriptor, crate::posix::PosixError> {
        let mut peer_descriptor = None;
        let mut match_count = 0usize;
        let mut raw = 0;
        while raw < CAPACITY {
            if let Some(socket) = self.entries[raw] {
                if let Ok((peer_local, peer_remote, peer_connection_id)) =
                    connected_endpoints(socket.state)
                {
                    if peer_connection_id == connection_id
                        && peer_local == remote_endpoint
                        && peer_remote == local_endpoint
                    {
                        peer_descriptor = Some(NetworkSocketDescriptor::from_raw(raw));
                        match_count += 1;
                    }
                }
            }
            raw += 1;
        }

        if match_count == 1 {
            Ok(peer_descriptor.expect("single peer match records a descriptor"))
        } else {
            Err(crate::posix::PosixError::Pipe)
        }
    }

    fn allocate_connection_id(&mut self) -> Result<u64, crate::posix::PosixError> {
        let connection_id = self.next_connection_id;
        self.next_connection_id = self
            .next_connection_id
            .checked_add(1)
            .ok_or(crate::posix::PosixError::NoSpace)?;
        Ok(connection_id)
    }

    fn active_listener_endpoint_in_use(
        &self,
        descriptor: NetworkSocketDescriptor,
        endpoint: Ipv4Endpoint,
    ) -> bool {
        let mut raw = 0usize;
        while raw < CAPACITY {
            if raw != descriptor.raw() {
                if let Some(socket) = self.entries[raw] {
                    if let NetworkSocketState::Listening { local_endpoint, .. } = socket.state {
                        if local_endpoint == endpoint {
                            return true;
                        }
                    }
                }
            }
            raw += 1;
        }
        false
    }

    fn pending_peer_is_connected(&self, peer: NetworkSocketPendingLocalPeer) -> bool {
        match self.socket(peer.client_descriptor()) {
            Ok(socket) if socket.owner() == peer.client_owner() => {
                matches!(
                    socket.state(),
                    NetworkSocketState::Connected { connection_id, .. }
                        if connection_id == peer.connection_id()
                )
            }
            _ => false,
        }
    }

    fn pending_queue_has_connected_peer(&self, pending: NetworkSocketPendingQueue) -> bool {
        let mut index = 0usize;
        while index < pending.len() as usize {
            if let Some(peer) = pending.peers[index] {
                if self.pending_peer_is_connected(peer) {
                    return true;
                }
            }
            index += 1;
        }
        false
    }

    fn create_smoltcp_bridge_record(
        &self,
        client_owner: crate::scheduler::ProcessOwnerId,
        client_descriptor: NetworkSocketDescriptor,
        listener_owner: crate::scheduler::ProcessOwnerId,
        listener_descriptor: NetworkSocketDescriptor,
        connection_id: u64,
    ) -> Result<SmoltcpSocketBridgeRecord, crate::posix::PosixError> {
        if self.smoltcp_bridges.iter().all(Option::is_some) {
            return Err(crate::posix::PosixError::NoSpace);
        }
        let handshake = smoltcp_socket_bridge_transfer(&[])?;
        Ok(SmoltcpSocketBridgeRecord {
            client_owner,
            client_descriptor,
            listener_owner,
            listener_descriptor,
            accepted_owner: None,
            accepted_descriptor: None,
            connection_id,
            handshake,
            payload_transfers: 0,
            last_payload: handshake,
        })
    }

    fn insert_smoltcp_bridge_record(
        &mut self,
        record: SmoltcpSocketBridgeRecord,
    ) -> Result<(), crate::posix::PosixError> {
        let Some(slot) = self.smoltcp_bridges.iter_mut().find(|slot| slot.is_none()) else {
            return Err(crate::posix::PosixError::NoSpace);
        };
        *slot = Some(record);
        Ok(())
    }

    fn attach_smoltcp_accepted_descriptor(
        &mut self,
        connection_id: u64,
        accepted_owner: crate::scheduler::ProcessOwnerId,
        accepted_descriptor: NetworkSocketDescriptor,
    ) -> Result<(), crate::posix::PosixError> {
        let Some(record) = self
            .smoltcp_bridges
            .iter_mut()
            .flatten()
            .find(|record| record.connection_id == connection_id)
        else {
            return Err(crate::posix::PosixError::BadDescriptor);
        };
        record.accepted_owner = Some(accepted_owner);
        record.accepted_descriptor = Some(accepted_descriptor);
        Ok(())
    }

    fn record_smoltcp_payload_transfer(
        &mut self,
        connection_id: u64,
        payload: &[u8],
    ) -> Result<(), crate::posix::PosixError> {
        if payload.is_empty() {
            return Ok(());
        }
        let observation = smoltcp_socket_bridge_transfer(payload)?;
        let Some(record) = self
            .smoltcp_bridges
            .iter_mut()
            .flatten()
            .find(|record| record.connection_id == connection_id)
        else {
            return Err(crate::posix::PosixError::BadDescriptor);
        };
        record.payload_transfers = record
            .payload_transfers
            .checked_add(1)
            .ok_or(crate::posix::PosixError::NoSpace)?;
        record.last_payload = observation;
        Ok(())
    }

    fn remove_smoltcp_bridge_for_descriptor(
        &mut self,
        owner: crate::scheduler::ProcessOwnerId,
        descriptor: NetworkSocketDescriptor,
    ) {
        for slot in &mut self.smoltcp_bridges {
            let Some(record) = slot else {
                continue;
            };
            let accepted_matches = record.accepted_owner == Some(owner)
                && record.accepted_descriptor == Some(descriptor);
            if (record.client_owner == owner && record.client_descriptor == descriptor)
                || (record.listener_owner == owner && record.listener_descriptor == descriptor)
                || accepted_matches
            {
                *slot = None;
            }
        }
    }
}

fn smoltcp_socket_bridge_transfer(
    payload: &[u8],
) -> Result<SmoltcpSocketBridgeObservation, crate::posix::PosixError> {
    if payload.len() > SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY {
        return Err(crate::posix::PosixError::NoSpace);
    }

    let mut client_adapter = SmoltcpPacketDeviceAdapter::<
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_FRAME_CAPACITY,
    >::new();
    let mut server_adapter = SmoltcpPacketDeviceAdapter::<
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_FRAME_CAPACITY,
    >::new();
    let mut client_iface = smoltcp_bridge_interface(
        &mut client_adapter,
        SMOLTCP_SOCKET_BRIDGE_CLIENT_MAC,
        SMOLTCP_SOCKET_BRIDGE_CLIENT_IPV4,
    );
    let mut server_iface = smoltcp_bridge_interface(
        &mut server_adapter,
        SMOLTCP_SOCKET_BRIDGE_SERVER_MAC,
        SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4,
    );
    let mut client_rx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut client_tx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut server_rx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut server_tx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let client_socket = smoltcp::socket::tcp::Socket::new(
        smoltcp::socket::tcp::SocketBuffer::new(&mut client_rx_storage[..]),
        smoltcp::socket::tcp::SocketBuffer::new(&mut client_tx_storage[..]),
    );
    let server_socket = smoltcp::socket::tcp::Socket::new(
        smoltcp::socket::tcp::SocketBuffer::new(&mut server_rx_storage[..]),
        smoltcp::socket::tcp::SocketBuffer::new(&mut server_tx_storage[..]),
    );
    let mut client_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
    let mut server_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
    let mut client_sockets = smoltcp::iface::SocketSet::new(&mut client_socket_storage[..]);
    let mut server_sockets = smoltcp::iface::SocketSet::new(&mut server_socket_storage[..]);
    let client_handle = client_sockets.add(client_socket);
    let server_handle = server_sockets.add(server_socket);
    server_sockets
        .get_mut::<smoltcp::socket::tcp::Socket>(server_handle)
        .listen(SMOLTCP_SOCKET_BRIDGE_SERVER_PORT)
        .map_err(|_| crate::posix::PosixError::InvalidArgument)?;
    client_sockets
        .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
        .connect(
            client_iface.context(),
            (
                smoltcp::wire::IpAddress::v4(
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[0],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[1],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[2],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[3],
                ),
                SMOLTCP_SOCKET_BRIDGE_SERVER_PORT,
            ),
            SMOLTCP_SOCKET_BRIDGE_CLIENT_PORT,
        )
        .map_err(|_| crate::posix::PosixError::InvalidArgument)?;

    let mut observation = smoltcp_bridge_drive(
        &mut client_adapter,
        &mut server_adapter,
        &mut client_iface,
        &mut server_iface,
        &mut client_sockets,
        &mut server_sockets,
        client_handle,
        server_handle,
        payload,
    )?;
    observation.payload_len = payload.len();
    Ok(observation)
}

fn driver_packet_smoltcp_listener_transfer(
    payload: &[u8],
) -> Result<SmoltcpSocketBridgeObservation, crate::posix::PosixError> {
    if payload.len() > SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY {
        return Err(crate::posix::PosixError::NoSpace);
    }

    let mut client_adapter = DriverPacketAdapter::<
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_FRAME_CAPACITY,
    >::new();
    let mut server_adapter = DriverPacketAdapter::<
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_PACKET_QUEUE_CAPACITY,
        SMOLTCP_SOCKET_BRIDGE_FRAME_CAPACITY,
    >::new();
    let mut client_iface = smoltcp_driver_packet_interface(
        &mut client_adapter,
        SMOLTCP_SOCKET_BRIDGE_CLIENT_MAC,
        SMOLTCP_SOCKET_BRIDGE_CLIENT_IPV4,
    );
    let mut server_iface = smoltcp_driver_packet_interface(
        &mut server_adapter,
        SMOLTCP_SOCKET_BRIDGE_SERVER_MAC,
        SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4,
    );
    let mut client_rx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut client_tx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut server_rx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let mut server_tx_storage = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];
    let client_socket = smoltcp::socket::tcp::Socket::new(
        smoltcp::socket::tcp::SocketBuffer::new(&mut client_rx_storage[..]),
        smoltcp::socket::tcp::SocketBuffer::new(&mut client_tx_storage[..]),
    );
    let server_socket = smoltcp::socket::tcp::Socket::new(
        smoltcp::socket::tcp::SocketBuffer::new(&mut server_rx_storage[..]),
        smoltcp::socket::tcp::SocketBuffer::new(&mut server_tx_storage[..]),
    );
    let mut client_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
    let mut server_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
    let mut client_sockets = smoltcp::iface::SocketSet::new(&mut client_socket_storage[..]);
    let mut server_sockets = smoltcp::iface::SocketSet::new(&mut server_socket_storage[..]);
    let client_handle = client_sockets.add(client_socket);
    let server_handle = server_sockets.add(server_socket);
    server_sockets
        .get_mut::<smoltcp::socket::tcp::Socket>(server_handle)
        .listen(SMOLTCP_SOCKET_BRIDGE_SERVER_PORT)
        .map_err(|_| crate::posix::PosixError::InvalidArgument)?;
    client_sockets
        .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
        .connect(
            client_iface.context(),
            (
                smoltcp::wire::IpAddress::v4(
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[0],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[1],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[2],
                    SMOLTCP_SOCKET_BRIDGE_SERVER_IPV4[3],
                ),
                SMOLTCP_SOCKET_BRIDGE_SERVER_PORT,
            ),
            SMOLTCP_SOCKET_BRIDGE_CLIENT_PORT,
        )
        .map_err(|_| crate::posix::PosixError::InvalidArgument)?;

    let mut observation = smoltcp_driver_packet_drive(
        &mut client_adapter,
        &mut server_adapter,
        &mut client_iface,
        &mut server_iface,
        &mut client_sockets,
        &mut server_sockets,
        client_handle,
        server_handle,
        payload,
    )?;
    observation.payload_len = payload.len();
    Ok(observation)
}

fn smoltcp_bridge_interface<
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    adapter: &mut SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
    mac: [u8; ETHERNET_ADDR_LEN],
    ipv4: [u8; 4],
) -> smoltcp::iface::Interface {
    let config = smoltcp::iface::Config::new(smoltcp::wire::EthernetAddress(mac).into());
    let mut iface = smoltcp::iface::Interface::new(config, adapter, smoltcp::time::Instant::ZERO);
    iface.update_ip_addrs(|addresses| {
        addresses
            .push(smoltcp::wire::IpCidr::new(
                smoltcp::wire::IpAddress::v4(ipv4[0], ipv4[1], ipv4[2], ipv4[3]),
                SMOLTCP_SOCKET_BRIDGE_PREFIX_LEN,
            ))
            .expect("single smoltcp IPv4 address slot remains available");
    });
    iface
}

fn smoltcp_driver_packet_interface<
    const RX_CAPACITY: usize,
    const TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    adapter: &mut DriverPacketAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
    mac: [u8; ETHERNET_ADDR_LEN],
    ipv4: [u8; 4],
) -> smoltcp::iface::Interface {
    let config = smoltcp::iface::Config::new(smoltcp::wire::EthernetAddress(mac).into());
    let mut iface = smoltcp::iface::Interface::new(config, adapter, smoltcp::time::Instant::ZERO);
    iface.update_ip_addrs(|addresses| {
        addresses
            .push(smoltcp::wire::IpCidr::new(
                smoltcp::wire::IpAddress::v4(ipv4[0], ipv4[1], ipv4[2], ipv4[3]),
                SMOLTCP_SOCKET_BRIDGE_PREFIX_LEN,
            ))
            .expect("single smoltcp IPv4 address slot remains available");
    });
    iface
}

fn smoltcp_bridge_move_frames<
    const FROM_RX_CAPACITY: usize,
    const FROM_TX_CAPACITY: usize,
    const TO_RX_CAPACITY: usize,
    const TO_TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    from: &mut SmoltcpPacketDeviceAdapter<FROM_RX_CAPACITY, FROM_TX_CAPACITY, FRAME_CAPACITY>,
    to: &mut SmoltcpPacketDeviceAdapter<TO_RX_CAPACITY, TO_TX_CAPACITY, FRAME_CAPACITY>,
) -> Result<usize, crate::posix::PosixError> {
    let mut moved = 0usize;
    while let Some(frame) = from.pop_transmitted() {
        to.inject_received(frame.as_bytes())
            .map_err(|_| crate::posix::PosixError::NoSpace)?;
        moved += 1;
    }
    Ok(moved)
}

fn smoltcp_driver_packet_move_frames<
    const FROM_RX_CAPACITY: usize,
    const FROM_TX_CAPACITY: usize,
    const TO_RX_CAPACITY: usize,
    const TO_TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    from: &mut DriverPacketAdapter<FROM_RX_CAPACITY, FROM_TX_CAPACITY, FRAME_CAPACITY>,
    to: &mut DriverPacketAdapter<TO_RX_CAPACITY, TO_TX_CAPACITY, FRAME_CAPACITY>,
) -> Result<usize, crate::posix::PosixError> {
    let mut moved = 0usize;
    while let Some(frame) = from.pop_driver_tx() {
        to.inject_driver_rx(frame.as_bytes())
            .map_err(|_| crate::posix::PosixError::NoSpace)?;
        moved += 1;
    }
    Ok(moved)
}

fn smoltcp_bridge_drive<
    const CLIENT_RX_CAPACITY: usize,
    const CLIENT_TX_CAPACITY: usize,
    const SERVER_RX_CAPACITY: usize,
    const SERVER_TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    client_adapter: &mut SmoltcpPacketDeviceAdapter<
        CLIENT_RX_CAPACITY,
        CLIENT_TX_CAPACITY,
        FRAME_CAPACITY,
    >,
    server_adapter: &mut SmoltcpPacketDeviceAdapter<
        SERVER_RX_CAPACITY,
        SERVER_TX_CAPACITY,
        FRAME_CAPACITY,
    >,
    client_iface: &mut smoltcp::iface::Interface,
    server_iface: &mut smoltcp::iface::Interface,
    client_sockets: &mut smoltcp::iface::SocketSet<'_>,
    server_sockets: &mut smoltcp::iface::SocketSet<'_>,
    client_handle: smoltcp::iface::SocketHandle,
    server_handle: smoltcp::iface::SocketHandle,
    payload: &[u8],
) -> Result<SmoltcpSocketBridgeObservation, crate::posix::PosixError> {
    let mut observation = SmoltcpSocketBridgeObservation {
        client_state: smoltcp::socket::tcp::State::Closed,
        server_state: smoltcp::socket::tcp::State::Closed,
        steps: 0,
        client_to_server_frames: 0,
        server_to_client_frames: 0,
        payload_len: 0,
    };
    let mut payload_sent = payload.is_empty();
    let mut payload_received = payload.is_empty();
    let mut receive_buffer = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];

    let mut step = 0usize;
    while step < SMOLTCP_SOCKET_BRIDGE_MAX_STEPS {
        let now = smoltcp::time::Instant::from_millis(step as i64);
        client_iface.poll(now, client_adapter, client_sockets);
        observation.client_to_server_frames +=
            smoltcp_bridge_move_frames(client_adapter, server_adapter)?;
        server_iface.poll(now, server_adapter, server_sockets);
        observation.server_to_client_frames +=
            smoltcp_bridge_move_frames(server_adapter, client_adapter)?;
        client_iface.poll(now, client_adapter, client_sockets);
        observation.client_to_server_frames +=
            smoltcp_bridge_move_frames(client_adapter, server_adapter)?;
        server_iface.poll(now, server_adapter, server_sockets);
        observation.server_to_client_frames +=
            smoltcp_bridge_move_frames(server_adapter, client_adapter)?;

        observation.client_state = client_sockets
            .get::<smoltcp::socket::tcp::Socket>(client_handle)
            .state();
        observation.server_state = server_sockets
            .get::<smoltcp::socket::tcp::Socket>(server_handle)
            .state();
        if observation.client_state == smoltcp::socket::tcp::State::Established
            && observation.server_state == smoltcp::socket::tcp::State::Established
        {
            if !payload_sent {
                let sent = client_sockets
                    .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
                    .send_slice(payload)
                    .map_err(|_| crate::posix::PosixError::NoSpace)?;
                if sent != payload.len() {
                    return Err(crate::posix::PosixError::NoSpace);
                }
                payload_sent = true;
            }
            if payload_sent && !payload_received {
                let server_socket =
                    server_sockets.get_mut::<smoltcp::socket::tcp::Socket>(server_handle);
                if server_socket.can_recv() {
                    match server_socket.recv_slice(&mut receive_buffer[..payload.len()]) {
                        Ok(received) if received == payload.len() => {
                            if &receive_buffer[..payload.len()] != payload {
                                return Err(crate::posix::PosixError::Io);
                            }
                            payload_received = true;
                        }
                        Ok(_) => return Err(crate::posix::PosixError::Io),
                        Err(_) => return Err(crate::posix::PosixError::Pipe),
                    }
                }
            }
            if payload_received {
                observation.steps = step + 1;
                return Ok(observation);
            }
        }
        step += 1;
    }

    Err(crate::posix::PosixError::Again)
}

fn smoltcp_driver_packet_drive<
    const CLIENT_RX_CAPACITY: usize,
    const CLIENT_TX_CAPACITY: usize,
    const SERVER_RX_CAPACITY: usize,
    const SERVER_TX_CAPACITY: usize,
    const FRAME_CAPACITY: usize,
>(
    client_adapter: &mut DriverPacketAdapter<
        CLIENT_RX_CAPACITY,
        CLIENT_TX_CAPACITY,
        FRAME_CAPACITY,
    >,
    server_adapter: &mut DriverPacketAdapter<
        SERVER_RX_CAPACITY,
        SERVER_TX_CAPACITY,
        FRAME_CAPACITY,
    >,
    client_iface: &mut smoltcp::iface::Interface,
    server_iface: &mut smoltcp::iface::Interface,
    client_sockets: &mut smoltcp::iface::SocketSet<'_>,
    server_sockets: &mut smoltcp::iface::SocketSet<'_>,
    client_handle: smoltcp::iface::SocketHandle,
    server_handle: smoltcp::iface::SocketHandle,
    payload: &[u8],
) -> Result<SmoltcpSocketBridgeObservation, crate::posix::PosixError> {
    let mut observation = SmoltcpSocketBridgeObservation {
        client_state: smoltcp::socket::tcp::State::Closed,
        server_state: smoltcp::socket::tcp::State::Closed,
        steps: 0,
        client_to_server_frames: 0,
        server_to_client_frames: 0,
        payload_len: 0,
    };
    let mut payload_sent = payload.is_empty();
    let mut payload_received = payload.is_empty();
    let mut receive_buffer = [0u8; SMOLTCP_SOCKET_BRIDGE_TCP_BUFFER_CAPACITY];

    let mut step = 0usize;
    while step < SMOLTCP_SOCKET_BRIDGE_MAX_STEPS {
        let now = smoltcp::time::Instant::from_millis(step as i64);
        client_iface.poll(now, client_adapter, client_sockets);
        observation.client_to_server_frames +=
            smoltcp_driver_packet_move_frames(client_adapter, server_adapter)?;
        server_iface.poll(now, server_adapter, server_sockets);
        observation.server_to_client_frames +=
            smoltcp_driver_packet_move_frames(server_adapter, client_adapter)?;
        client_iface.poll(now, client_adapter, client_sockets);
        observation.client_to_server_frames +=
            smoltcp_driver_packet_move_frames(client_adapter, server_adapter)?;
        server_iface.poll(now, server_adapter, server_sockets);
        observation.server_to_client_frames +=
            smoltcp_driver_packet_move_frames(server_adapter, client_adapter)?;

        observation.client_state = client_sockets
            .get::<smoltcp::socket::tcp::Socket>(client_handle)
            .state();
        observation.server_state = server_sockets
            .get::<smoltcp::socket::tcp::Socket>(server_handle)
            .state();
        if observation.client_state == smoltcp::socket::tcp::State::Established
            && observation.server_state == smoltcp::socket::tcp::State::Established
        {
            if !payload_sent {
                let sent = client_sockets
                    .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
                    .send_slice(payload)
                    .map_err(|_| crate::posix::PosixError::NoSpace)?;
                if sent != payload.len() {
                    return Err(crate::posix::PosixError::NoSpace);
                }
                payload_sent = true;
            }
            if payload_sent && !payload_received {
                let server_socket =
                    server_sockets.get_mut::<smoltcp::socket::tcp::Socket>(server_handle);
                if server_socket.can_recv() {
                    match server_socket.recv_slice(&mut receive_buffer[..payload.len()]) {
                        Ok(received) if received == payload.len() => {
                            if &receive_buffer[..payload.len()] != payload {
                                return Err(crate::posix::PosixError::Io);
                            }
                            payload_received = true;
                        }
                        Ok(_) => return Err(crate::posix::PosixError::Io),
                        Err(_) => return Err(crate::posix::PosixError::Pipe),
                    }
                }
            }
        }

        observation.steps = step + 1;
        if observation.client_state == smoltcp::socket::tcp::State::Established
            && observation.server_state == smoltcp::socket::tcp::State::Established
            && payload_sent
            && payload_received
        {
            return Ok(observation);
        }
        step += 1;
    }

    Err(crate::posix::PosixError::TimedOut)
}

fn connected_endpoints(
    state: NetworkSocketState,
) -> Result<(Ipv4Endpoint, Ipv4Endpoint, u64), crate::posix::PosixError> {
    match state {
        NetworkSocketState::Connected {
            local_endpoint,
            remote_endpoint,
            connection_id,
            ..
        }
        | NetworkSocketState::Accepted {
            local_endpoint,
            remote_endpoint,
            connection_id,
            ..
        } => Ok((local_endpoint, remote_endpoint, connection_id)),
        NetworkSocketState::OpenUnbound
        | NetworkSocketState::Bound { .. }
        | NetworkSocketState::Listening { .. } => Err(crate::posix::PosixError::InvalidArgument),
    }
}

fn connected_recv_queue(
    state: NetworkSocketState,
) -> Result<NetworkSocketPayloadQueue, crate::posix::PosixError> {
    match state {
        NetworkSocketState::Connected { recv_queue, .. }
        | NetworkSocketState::Accepted { recv_queue, .. } => Ok(recv_queue),
        NetworkSocketState::OpenUnbound
        | NetworkSocketState::Bound { .. }
        | NetworkSocketState::Listening { .. } => Err(crate::posix::PosixError::InvalidArgument),
    }
}

fn connected_recv_queue_mut(
    state: &mut NetworkSocketState,
) -> Result<&mut NetworkSocketPayloadQueue, crate::posix::PosixError> {
    match state {
        NetworkSocketState::Connected { recv_queue, .. }
        | NetworkSocketState::Accepted { recv_queue, .. } => Ok(recv_queue),
        NetworkSocketState::OpenUnbound
        | NetworkSocketState::Bound { .. }
        | NetworkSocketState::Listening { .. } => Err(crate::posix::PosixError::InvalidArgument),
    }
}

fn synthetic_client_port(
    descriptor: NetworkSocketDescriptor,
) -> Result<u16, crate::posix::PosixError> {
    let raw = u16::try_from(descriptor.raw()).map_err(|_| crate::posix::PosixError::NoSpace)?;
    SOCKET_SYNTHETIC_CLIENT_PORT_BASE
        .checked_add(raw)
        .ok_or(crate::posix::PosixError::NoSpace)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkPingOperationDescriptorTable<
    const DESCRIPTOR_CAPACITY: usize,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    entries: [Option<UserspacePingOperation<ARP_CAPACITY, PAYLOAD_CAPACITY>>; DESCRIPTOR_CAPACITY],
}

impl<const DESCRIPTOR_CAPACITY: usize, const ARP_CAPACITY: usize, const PAYLOAD_CAPACITY: usize>
    NetworkPingOperationDescriptorTable<DESCRIPTOR_CAPACITY, ARP_CAPACITY, PAYLOAD_CAPACITY>
{
    pub(crate) const fn new() -> Self {
        Self {
            entries: [None; DESCRIPTOR_CAPACITY],
        }
    }

    pub(crate) fn open(
        &mut self,
    ) -> Result<NetworkPingOperationDescriptor, crate::posix::PosixError> {
        self.open_with_operation(UserspacePingOperation::new())
    }

    pub(crate) fn open_with_service(
        &mut self,
        service: SinglePingPacketService<ARP_CAPACITY, PAYLOAD_CAPACITY>,
    ) -> Result<NetworkPingOperationDescriptor, crate::posix::PosixError> {
        self.open_with_operation(UserspacePingOperation::with_service(service))
    }

    pub(crate) fn close(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<(), crate::posix::PosixError> {
        let entry = self
            .entries
            .get_mut(descriptor.raw())
            .ok_or(crate::posix::PosixError::BadDescriptor)?;
        if entry.take().is_some() {
            Ok(())
        } else {
            Err(crate::posix::PosixError::BadDescriptor)
        }
    }

    pub(crate) fn status(
        &self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<UserspacePingOperationStatus, crate::posix::PosixError> {
        Ok(self.operation(descriptor)?.status())
    }

    pub(crate) fn start<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        endpoint: LocalNetworkEndpoint,
        route_policy: Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        transmit_buffer: &mut [u8],
        arp_retry_budget: usize,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.operation_mut(descriptor)?.start(
            device,
            endpoint,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            transmit_buffer,
            arp_retry_budget,
        )
    }

    pub(crate) fn pump<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        receive_buffer: &mut [u8],
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.operation_mut(descriptor)?
            .pump(device, receive_buffer, transmit_buffer)
    }

    pub(crate) fn pump_received<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        received_frame: &[u8],
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.operation_mut(descriptor)?
            .pump_received(device, received_frame, transmit_buffer)
    }

    pub(crate) fn retry_arp<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.operation_mut(descriptor)?
            .retry_arp(device, transmit_buffer)
    }

    pub(crate) fn timeout(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.operation_mut(descriptor)?.timeout()
    }

    fn open_with_operation(
        &mut self,
        operation: UserspacePingOperation<ARP_CAPACITY, PAYLOAD_CAPACITY>,
    ) -> Result<NetworkPingOperationDescriptor, crate::posix::PosixError> {
        if self.entries.iter().any(Option::is_some) {
            return Err(crate::posix::PosixError::Busy);
        }

        let mut raw = 0;
        while raw < DESCRIPTOR_CAPACITY {
            if self.entries[raw].is_none() {
                self.entries[raw] = Some(operation);
                return Ok(NetworkPingOperationDescriptor::from_raw(raw));
            }
            raw += 1;
        }

        Err(crate::posix::PosixError::TooManyOpenFiles)
    }

    fn operation(
        &self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<&UserspacePingOperation<ARP_CAPACITY, PAYLOAD_CAPACITY>, crate::posix::PosixError>
    {
        self.entries
            .get(descriptor.raw())
            .and_then(|entry| entry.as_ref())
            .ok_or(crate::posix::PosixError::BadDescriptor)
    }

    fn operation_mut(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<&mut UserspacePingOperation<ARP_CAPACITY, PAYLOAD_CAPACITY>, crate::posix::PosixError>
    {
        self.entries
            .get_mut(descriptor.raw())
            .and_then(|entry| entry.as_mut())
            .ok_or(crate::posix::PosixError::BadDescriptor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UserspacePendingSuccess {
    StartedPendingArp,
    RetryTransmitted,
}

fn terminal_destination_from_status(status: UserspacePingOperationStatus) -> [u8; 4] {
    match status {
        UserspacePingOperationStatus::Inflight { destination_ipv4 }
        | UserspacePingOperationStatus::PendingArp {
            destination_ipv4, ..
        }
        | UserspacePingOperationStatus::Completed {
            destination_ipv4, ..
        }
        | UserspacePingOperationStatus::TimedOut { destination_ipv4 } => destination_ipv4,
        UserspacePingOperationStatus::Idle => [0; 4],
    }
}

fn userspace_step_from_pending_poll_result(
    result: PendingIcmpEchoPollResult,
) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
    match result {
        PendingIcmpEchoPollResult::NoPendingRequest => {
            Err(crate::posix::PosixError::InvalidArgument)
        }
        PendingIcmpEchoPollResult::NoFrame => Ok(UserspacePingOperationStep::NoFrame),
        PendingIcmpEchoPollResult::ReceiveBufferTooSmall => Err(crate::posix::PosixError::NoSpace),
        PendingIcmpEchoPollResult::ReceiveError(error) => Err(posix_error_from_device_error(error)),
        PendingIcmpEchoPollResult::PendingResult(result) => {
            userspace_step_from_pending_result(result, UserspacePendingSuccess::StartedPendingArp)
        }
    }
}

fn userspace_step_from_inflight_poll_result(
    result: InflightIcmpEchoPollResult,
) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
    match result {
        InflightIcmpEchoPollResult::NoInflightRequest => {
            Err(crate::posix::PosixError::InvalidArgument)
        }
        InflightIcmpEchoPollResult::NoFrame => Ok(UserspacePingOperationStep::NoFrame),
        InflightIcmpEchoPollResult::ReceiveBufferTooSmall => Err(crate::posix::PosixError::NoSpace),
        InflightIcmpEchoPollResult::ReceiveError(error) => {
            Err(posix_error_from_device_error(error))
        }
        InflightIcmpEchoPollResult::ObservationResult(result) => {
            userspace_step_from_inflight_result(result)
        }
    }
}

fn userspace_step_from_pending_result(
    result: PendingIcmpEchoResult,
    success: UserspacePendingSuccess,
) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
    match result {
        PendingIcmpEchoResult::IcmpEchoRequestTransmitted { frame_len } => {
            Ok(UserspacePingOperationStep::AdvancedToInflight { frame_len })
        }
        PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len } => match success {
            UserspacePendingSuccess::StartedPendingArp => {
                Ok(UserspacePingOperationStep::StartedPendingArp { frame_len })
            }
            UserspacePendingSuccess::RetryTransmitted => {
                Ok(UserspacePingOperationStep::RetryTransmitted { frame_len })
            }
        },
        PendingIcmpEchoResult::NoPendingRequest => Err(crate::posix::PosixError::InvalidArgument),
        PendingIcmpEchoResult::PendingRequestAlreadyQueued { .. } => {
            Err(crate::posix::PosixError::Busy)
        }
        PendingIcmpEchoResult::PendingPayloadTooLarge { .. } => {
            Err(crate::posix::PosixError::Range)
        }
        PendingIcmpEchoResult::PendingNeighborUnresolved { .. }
        | PendingIcmpEchoResult::ArpRetryBudgetExhausted { .. }
        | PendingIcmpEchoResult::NonMatchingArp { .. } => Err(crate::posix::PosixError::Again),
        PendingIcmpEchoResult::RouteError(error) => Err(posix_error_from_route_error(error)),
        PendingIcmpEchoResult::RequestError(error) => Err(posix_error_from_frame_error(error)),
        PendingIcmpEchoResult::ArpError(error) => Err(posix_error_from_packet_error(error)),
        PendingIcmpEchoResult::TransmitError { error, .. } => {
            Err(posix_error_from_device_error(error))
        }
    }
}

fn userspace_step_from_inflight_result(
    result: InflightIcmpEchoResult,
) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
    match result {
        InflightIcmpEchoResult::InflightRequestTracked => {
            Ok(UserspacePingOperationStep::StartedInflight { frame_len: 0 })
        }
        InflightIcmpEchoResult::IcmpEchoReplyMatched { payload_len } => {
            Ok(UserspacePingOperationStep::Completed { payload_len })
        }
        InflightIcmpEchoResult::NoInflightRequest => Err(crate::posix::PosixError::InvalidArgument),
        InflightIcmpEchoResult::InflightRequestAlreadyTracked { .. } => {
            Err(crate::posix::PosixError::Busy)
        }
        InflightIcmpEchoResult::InflightPayloadTooLarge { .. } => {
            Err(crate::posix::PosixError::Range)
        }
        InflightIcmpEchoResult::NonMatchingIcmpEchoReply { .. } => {
            Err(crate::posix::PosixError::Again)
        }
        InflightIcmpEchoResult::ReplyError(error) => Err(posix_error_from_packet_error(error)),
    }
}

pub(crate) fn posix_error_from_device_error(error: DeviceError) -> crate::posix::PosixError {
    match error {
        DeviceError::WouldBlock => crate::posix::PosixError::Again,
        DeviceError::BufferTooSmall => crate::posix::PosixError::NoSpace,
        DeviceError::Io => crate::posix::PosixError::Io,
    }
}

fn posix_error_from_route_error(error: OutboundRouteError) -> crate::posix::PosixError {
    match error {
        OutboundRouteError::NoRouteToDestination { .. } => crate::posix::PosixError::NoEntry,
        OutboundRouteError::Frame(error) => posix_error_from_frame_error(error),
    }
}

fn posix_error_from_frame_error(error: OutboundFrameError) -> crate::posix::PosixError {
    match error {
        OutboundFrameError::NeighborUnresolved { .. } => crate::posix::PosixError::Again,
        OutboundFrameError::PayloadTooLarge { .. } => crate::posix::PosixError::Range,
        OutboundFrameError::OutputBufferTooSmall { .. } => crate::posix::PosixError::NoSpace,
    }
}

pub(crate) fn posix_error_from_packet_error(error: PacketError) -> crate::posix::PosixError {
    match error {
        PacketError::OutputBufferTooSmall => crate::posix::PosixError::NoSpace,
        PacketError::UnsupportedEtherType
        | PacketError::UnsupportedArpHardware
        | PacketError::UnsupportedArpProtocol
        | PacketError::UnsupportedArpOperation
        | PacketError::UnsupportedIpv4Protocol
        | PacketError::UnsupportedIpv4Options
        | PacketError::UnsupportedIpv4Fragment => crate::posix::PosixError::NotSupported,
        PacketError::NotForLocalHost => crate::posix::PosixError::Again,
        PacketError::Truncated
        | PacketError::InvalidArpHardwareLength
        | PacketError::InvalidArpProtocolLength
        | PacketError::InvalidIpv4Version
        | PacketError::InvalidIpv4HeaderLength
        | PacketError::InvalidIpv4TotalLength
        | PacketError::InvalidIpv4Checksum
        | PacketError::InvalidIcmpEcho
        | PacketError::InvalidIcmpChecksum => crate::posix::PosixError::InvalidArgument,
    }
}

pub(crate) fn record_single_inflight_ipv4_icmp_echo_request<const PAYLOAD_CAPACITY: usize>(
    inflight: &mut SingleInflightIcmpEcho<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    payload: &[u8],
) -> InflightIcmpEchoResult {
    let request = match InflightIcmpEchoRequest::new(
        endpoint,
        destination_ipv4,
        identifier,
        sequence_number,
        payload,
    ) {
        Ok(request) => request,
        Err(error) => return error,
    };

    match inflight.store(request) {
        Ok(()) => InflightIcmpEchoResult::InflightRequestTracked,
        Err(error) => error,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PendingIcmpEchoPollResult {
    NoPendingRequest,
    NoFrame,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
    PendingResult(PendingIcmpEchoResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InflightIcmpEchoPollResult {
    NoInflightRequest,
    NoFrame,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
    ObservationResult(InflightIcmpEchoResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NetworkRuntimeDevicePumpStepResult {
    NoFrame,
    ReceiveBufferTooSmall,
    ReceiveError(DeviceError),
    LocalNoReply,
    LocalDispatchError(PacketError),
    LocalTransmitError(DeviceError),
    LocalReply(PacketDispatchResult),
    ActivePingStep {
        descriptor: NetworkPingOperationDescriptor,
        step: UserspacePingOperationStep,
    },
    ActivePingError {
        descriptor: NetworkPingOperationDescriptor,
        error: crate::posix::PosixError,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NetworkRuntimeDevicePump<
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
> {
    endpoint: LocalNetworkEndpoint,
    local_arp_cache: ArpCache<LOCAL_ARP_CAPACITY>,
    ping_operations: NetworkPingOperationDescriptorTable<
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >,
}

impl<
    const LOCAL_ARP_CAPACITY: usize,
    const DESCRIPTOR_CAPACITY: usize,
    const OPERATION_ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>
    NetworkRuntimeDevicePump<
        LOCAL_ARP_CAPACITY,
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    >
{
    pub(crate) const fn new(endpoint: LocalNetworkEndpoint) -> Self {
        Self {
            endpoint,
            local_arp_cache: ArpCache::new(),
            ping_operations: NetworkPingOperationDescriptorTable::new(),
        }
    }

    pub(crate) const fn with_local_arp_cache(
        endpoint: LocalNetworkEndpoint,
        local_arp_cache: ArpCache<LOCAL_ARP_CAPACITY>,
    ) -> Self {
        Self {
            endpoint,
            local_arp_cache,
            ping_operations: NetworkPingOperationDescriptorTable::new(),
        }
    }

    pub(crate) const fn local_arp_cache(&self) -> &ArpCache<LOCAL_ARP_CAPACITY> {
        &self.local_arp_cache
    }

    pub(crate) const fn ping_operations(
        &self,
    ) -> &NetworkPingOperationDescriptorTable<
        DESCRIPTOR_CAPACITY,
        OPERATION_ARP_CAPACITY,
        PAYLOAD_CAPACITY,
    > {
        &self.ping_operations
    }

    pub(crate) fn open_ping_operation(
        &mut self,
    ) -> Result<NetworkPingOperationDescriptor, crate::posix::PosixError> {
        self.ping_operations.open()
    }

    pub(crate) fn close_ping_operation(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<(), crate::posix::PosixError> {
        self.ping_operations.close(descriptor)
    }

    pub(crate) fn ping_status(
        &self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<UserspacePingOperationStatus, crate::posix::PosixError> {
        self.ping_operations.status(descriptor)
    }

    pub(crate) fn start_ping<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        route_policy: Ipv4EgressRoutePolicy,
        destination_ipv4: [u8; 4],
        identifier: u16,
        sequence_number: u16,
        ttl: u8,
        payload: &[u8],
        transmit_buffer: &mut [u8],
        arp_retry_budget: usize,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.ping_operations.start(
            descriptor,
            device,
            self.endpoint,
            route_policy,
            destination_ipv4,
            identifier,
            sequence_number,
            ttl,
            payload,
            transmit_buffer,
            arp_retry_budget,
        )
    }

    pub(crate) fn retry_ping_arp<D: NetworkDevice>(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
        device: &mut D,
        transmit_buffer: &mut [u8],
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.ping_operations
            .retry_arp(descriptor, device, transmit_buffer)
    }

    pub(crate) fn timeout_ping(
        &mut self,
        descriptor: NetworkPingOperationDescriptor,
    ) -> Result<UserspacePingOperationStep, crate::posix::PosixError> {
        self.ping_operations.timeout(descriptor)
    }

    pub(crate) fn pump<D: NetworkDevice>(
        &mut self,
        device: &mut D,
        active_ping: Option<NetworkPingOperationDescriptor>,
        receive_buffer: &mut [u8],
        transmit_buffer: &mut [u8],
    ) -> NetworkRuntimeDevicePumpStepResult {
        let received = match device.receive_frame(receive_buffer) {
            Ok(frame) => frame,
            Err(DeviceError::WouldBlock) => return NetworkRuntimeDevicePumpStepResult::NoFrame,
            Err(DeviceError::BufferTooSmall) => {
                return NetworkRuntimeDevicePumpStepResult::ReceiveBufferTooSmall;
            }
            Err(error) => return NetworkRuntimeDevicePumpStepResult::ReceiveError(error),
        };

        match dispatch_local_packet_with_arp_cache(
            received,
            self.endpoint,
            &mut self.local_arp_cache,
            transmit_buffer,
        ) {
            Ok(reply) => {
                let frame_len = reply.frame_len();
                match device.transmit_frame(&transmit_buffer[..frame_len]) {
                    Ok(()) => NetworkRuntimeDevicePumpStepResult::LocalReply(reply),
                    Err(error) => NetworkRuntimeDevicePumpStepResult::LocalTransmitError(error),
                }
            }
            Err(error) if network_runtime_may_offer_to_active_ping(error) => {
                self.pump_active_ping_received(active_ping, device, received, transmit_buffer)
            }
            Err(error) => NetworkRuntimeDevicePumpStepResult::LocalDispatchError(error),
        }
    }

    fn pump_active_ping_received<D: NetworkDevice>(
        &mut self,
        active_ping: Option<NetworkPingOperationDescriptor>,
        device: &mut D,
        received_frame: &[u8],
        transmit_buffer: &mut [u8],
    ) -> NetworkRuntimeDevicePumpStepResult {
        let descriptor = match active_ping {
            Some(descriptor) => descriptor,
            None => return NetworkRuntimeDevicePumpStepResult::LocalNoReply,
        };

        match self.ping_operations.pump_received(
            descriptor,
            device,
            received_frame,
            transmit_buffer,
        ) {
            Ok(step) => NetworkRuntimeDevicePumpStepResult::ActivePingStep { descriptor, step },
            Err(error) => NetworkRuntimeDevicePumpStepResult::ActivePingError { descriptor, error },
        }
    }
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

pub(crate) fn poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    pending: &mut SinglePendingIcmpEcho<PAYLOAD_CAPACITY>,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
) -> PendingIcmpEchoPollResult {
    if pending.pending().is_none() {
        return PendingIcmpEchoPollResult::NoPendingRequest;
    }

    let received = match device.receive_frame(receive_buffer) {
        Ok(frame) => frame,
        Err(DeviceError::WouldBlock) => return PendingIcmpEchoPollResult::NoFrame,
        Err(DeviceError::BufferTooSmall) => {
            return PendingIcmpEchoPollResult::ReceiveBufferTooSmall;
        }
        Err(error) => return PendingIcmpEchoPollResult::ReceiveError(error),
    };

    PendingIcmpEchoPollResult::PendingResult(
        learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
            device,
            arp_cache,
            pending,
            received,
            transmit_buffer,
        ),
    )
}

pub(crate) fn observe_single_inflight_ipv4_icmp_echo_reply<const PAYLOAD_CAPACITY: usize>(
    inflight: &mut SingleInflightIcmpEcho<PAYLOAD_CAPACITY>,
    reply_frame: &[u8],
) -> InflightIcmpEchoResult {
    let request = match inflight.take() {
        Some(request) => request,
        None => return InflightIcmpEchoResult::NoInflightRequest,
    };

    let result = match icmp_echo_reply_matches_inflight(reply_frame, &request) {
        Ok(()) => InflightIcmpEchoResult::IcmpEchoReplyMatched {
            payload_len: request.payload().len(),
        },
        Err(InflightIcmpEchoResult::NonMatchingIcmpEchoReply { .. }) => {
            InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
                destination_ipv4: request.destination_ipv4(),
            }
        }
        Err(error) => error,
    };

    if !matches!(result, InflightIcmpEchoResult::IcmpEchoReplyMatched { .. }) {
        inflight.restore(request);
    }

    result
}

pub(crate) fn poll_single_inflight_ipv4_icmp_echo_reply<
    D: NetworkDevice,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    inflight: &mut SingleInflightIcmpEcho<PAYLOAD_CAPACITY>,
    receive_buffer: &mut [u8],
) -> InflightIcmpEchoPollResult {
    if inflight.inflight().is_none() {
        return InflightIcmpEchoPollResult::NoInflightRequest;
    }

    let received = match device.receive_frame(receive_buffer) {
        Ok(frame) => frame,
        Err(DeviceError::WouldBlock) => return InflightIcmpEchoPollResult::NoFrame,
        Err(DeviceError::BufferTooSmall) => {
            return InflightIcmpEchoPollResult::ReceiveBufferTooSmall;
        }
        Err(error) => return InflightIcmpEchoPollResult::ReceiveError(error),
    };

    InflightIcmpEchoPollResult::ObservationResult(observe_single_inflight_ipv4_icmp_echo_reply(
        inflight, received,
    ))
}

pub(crate) fn start_routed_single_ping_transaction<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route_policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> SinglePingTransactionStartResult {
    start_routed_single_ping_transaction_with_arp_retry_budget(
        device,
        arp_cache,
        transaction,
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

pub(crate) fn start_routed_single_ping_transaction_with_arp_retry_budget<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route_policy: Ipv4EgressRoutePolicy,
    destination_ipv4: [u8; 4],
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
    arp_retry_budget: usize,
) -> SinglePingTransactionStartResult {
    if let Some(destination_ipv4) = transaction.inflight_destination_ipv4() {
        return SinglePingTransactionStartResult::InflightResult(
            InflightIcmpEchoResult::InflightRequestAlreadyTracked { destination_ipv4 },
        );
    }

    if let Some(destination_ipv4) = transaction.pending_destination_ipv4() {
        return SinglePingTransactionStartResult::PendingResult(
            PendingIcmpEchoResult::PendingRequestAlreadyQueued { destination_ipv4 },
        );
    }

    let route = match route_ipv4_egress(endpoint, route_policy, destination_ipv4) {
        Ok(route) => route,
        Err(error) => {
            return SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::RouteError(error),
            );
        }
    };

    match resolve_outbound_neighbor(arp_cache, route.next_hop_ipv4()) {
        OutboundNeighborResolution::Resolved { .. } => {
            start_resolved_routed_single_ping_transaction(
                device,
                arp_cache,
                transaction,
                endpoint,
                route,
                identifier,
                sequence_number,
                ttl,
                payload,
                output,
            )
        }
        OutboundNeighborResolution::Unresolved { .. } => {
            let result = transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget(
                device,
                arp_cache,
                &mut transaction.pending,
                endpoint,
                route_policy,
                destination_ipv4,
                identifier,
                sequence_number,
                ttl,
                payload,
                output,
                arp_retry_budget,
            );
            match result {
                PendingIcmpEchoResult::ArpRequestTransmittedAndPending { frame_len } => {
                    SinglePingTransactionStartResult::ArpRequestTransmittedAndPending { frame_len }
                }
                result => SinglePingTransactionStartResult::PendingResult(result),
            }
        }
    }
}

pub(crate) fn poll_single_ping_transaction<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    receive_buffer: &mut [u8],
    transmit_buffer: &mut [u8],
) -> SinglePingTransactionPollResult {
    if transaction.pending().is_none() && transaction.inflight().is_none() {
        return SinglePingTransactionPollResult::NoTransaction;
    }

    if let Some(request) = transaction.pending() {
        if transaction.inflight().is_some() {
            return SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                        destination_ipv4: request.destination_ipv4(),
                    },
                ),
            );
        }
    }

    let received = match device.receive_frame(receive_buffer) {
        Ok(frame) => frame,
        Err(DeviceError::WouldBlock) => {
            return match transaction.status() {
                SinglePingTransactionStatus::PendingArp { .. } => {
                    SinglePingTransactionPollResult::PendingResult(
                        PendingIcmpEchoPollResult::NoFrame,
                    )
                }
                SinglePingTransactionStatus::Inflight { .. } => {
                    SinglePingTransactionPollResult::InflightResult(
                        InflightIcmpEchoPollResult::NoFrame,
                    )
                }
                SinglePingTransactionStatus::Idle => SinglePingTransactionPollResult::NoTransaction,
            };
        }
        Err(DeviceError::BufferTooSmall) => {
            return match transaction.status() {
                SinglePingTransactionStatus::PendingArp { .. } => {
                    SinglePingTransactionPollResult::PendingResult(
                        PendingIcmpEchoPollResult::ReceiveBufferTooSmall,
                    )
                }
                SinglePingTransactionStatus::Inflight { .. } => {
                    SinglePingTransactionPollResult::InflightResult(
                        InflightIcmpEchoPollResult::ReceiveBufferTooSmall,
                    )
                }
                SinglePingTransactionStatus::Idle => SinglePingTransactionPollResult::NoTransaction,
            };
        }
        Err(error) => {
            return match transaction.status() {
                SinglePingTransactionStatus::PendingArp { .. } => {
                    SinglePingTransactionPollResult::PendingResult(
                        PendingIcmpEchoPollResult::ReceiveError(error),
                    )
                }
                SinglePingTransactionStatus::Inflight { .. } => {
                    SinglePingTransactionPollResult::InflightResult(
                        InflightIcmpEchoPollResult::ReceiveError(error),
                    )
                }
                SinglePingTransactionStatus::Idle => SinglePingTransactionPollResult::NoTransaction,
            };
        }
    };

    poll_single_ping_transaction_received(device, arp_cache, transaction, received, transmit_buffer)
}

pub(crate) fn poll_single_ping_transaction_received<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &mut ArpCache<ARP_CAPACITY>,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    received_frame: &[u8],
    transmit_buffer: &mut [u8],
) -> SinglePingTransactionPollResult {
    if let Some(request) = transaction.pending() {
        let result = PendingIcmpEchoPollResult::PendingResult(
            learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                device,
                arp_cache,
                &mut transaction.pending,
                received_frame,
                transmit_buffer,
            ),
        );
        if let PendingIcmpEchoPollResult::PendingResult(
            PendingIcmpEchoResult::IcmpEchoRequestTransmitted { .. },
        ) = result
        {
            let inflight_request = match InflightIcmpEchoRequest::new(
                request.endpoint(),
                request.destination_ipv4(),
                request.identifier(),
                request.sequence_number(),
                request.payload(),
            ) {
                Ok(request) => request,
                Err(error) => {
                    return SinglePingTransactionPollResult::InflightResult(
                        InflightIcmpEchoPollResult::ObservationResult(error),
                    );
                }
            };
            if let Err(error) = transaction.inflight.store(inflight_request) {
                return SinglePingTransactionPollResult::InflightResult(
                    InflightIcmpEchoPollResult::ObservationResult(error),
                );
            }
        }
        return SinglePingTransactionPollResult::PendingResult(result);
    }

    if transaction.inflight().is_some() {
        return SinglePingTransactionPollResult::InflightResult(
            InflightIcmpEchoPollResult::ObservationResult(
                observe_single_inflight_ipv4_icmp_echo_reply(
                    &mut transaction.inflight,
                    received_frame,
                ),
            ),
        );
    }

    SinglePingTransactionPollResult::NoTransaction
}

pub(crate) fn retry_single_ping_transaction_arp_request<
    D: NetworkDevice,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    output: &mut [u8],
) -> SinglePingTransactionRetryResult {
    if transaction.pending().is_some() {
        return SinglePingTransactionRetryResult::PendingResult(
            retry_single_pending_ipv4_icmp_echo_arp_request(
                device,
                &mut transaction.pending,
                output,
            ),
        );
    }

    if let Some(destination_ipv4) = transaction.inflight_destination_ipv4() {
        return SinglePingTransactionRetryResult::InflightResult(
            InflightIcmpEchoResult::InflightRequestAlreadyTracked { destination_ipv4 },
        );
    }

    SinglePingTransactionRetryResult::NoTransaction
}

pub(crate) fn timeout_single_ping_transaction<const PAYLOAD_CAPACITY: usize>(
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
) -> SinglePingTransactionTimeoutResult {
    if let Some(request) = transaction.pending.take() {
        return SinglePingTransactionTimeoutResult::PendingTimedOut {
            destination_ipv4: request.destination_ipv4(),
            next_hop_ipv4: request.next_hop_ipv4(),
        };
    }

    if let Some(request) = transaction.inflight.take() {
        return SinglePingTransactionTimeoutResult::InflightTimedOut {
            destination_ipv4: request.destination_ipv4(),
        };
    }

    SinglePingTransactionTimeoutResult::NoTransaction
}

fn icmp_echo_reply_matches_inflight<const PAYLOAD_CAPACITY: usize>(
    reply_frame: &[u8],
    request: &InflightIcmpEchoRequest<PAYLOAD_CAPACITY>,
) -> Result<(), InflightIcmpEchoResult> {
    let frame = EthernetFrame::parse(reply_frame).map_err(InflightIcmpEchoResult::ReplyError)?;
    if frame.ether_type() != EtherType::Ipv4 {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::UnsupportedEtherType,
        ));
    }
    if frame.destination() != request.endpoint().mac() {
        return Err(InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
            destination_ipv4: request.destination_ipv4(),
        });
    }

    let ipv4_bytes = frame.payload();
    let packet = Ipv4Packet::parse(ipv4_bytes).map_err(InflightIcmpEchoResult::ReplyError)?;
    if packet.header_len() != IPV4_MIN_HEADER_LEN {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::UnsupportedIpv4Options,
        ));
    }
    if ipv4_fragment_field(ipv4_bytes) != 0 {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::UnsupportedIpv4Fragment,
        ));
    }
    if !ipv4_header_checksum_is_valid(&ipv4_bytes[..packet.header_len()]) {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::InvalidIpv4Checksum,
        ));
    }
    if packet.protocol() != IPV4_PROTOCOL_ICMP {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::UnsupportedIpv4Protocol,
        ));
    }
    if packet.source() != request.destination_ipv4()
        || packet.destination() != request.endpoint().ipv4()
    {
        return Err(InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
            destination_ipv4: request.destination_ipv4(),
        });
    }

    let icmp_reply = packet.payload();
    if icmp_reply.len() < ICMP_ECHO_HEADER_LEN || icmp_reply[0] != 0 || icmp_reply[1] != 0 {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::InvalidIcmpEcho,
        ));
    }
    if !internet_checksum_is_valid(icmp_reply) {
        return Err(InflightIcmpEchoResult::ReplyError(
            PacketError::InvalidIcmpChecksum,
        ));
    }
    if read_be_u16(icmp_reply, 4) != request.identifier()
        || read_be_u16(icmp_reply, 6) != request.sequence_number()
        || &icmp_reply[ICMP_ECHO_HEADER_LEN..] != request.payload()
    {
        return Err(InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
            destination_ipv4: request.destination_ipv4(),
        });
    }

    Ok(())
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

fn network_runtime_may_offer_to_active_ping(error: PacketError) -> bool {
    matches!(
        error,
        PacketError::NotForLocalHost
            | PacketError::UnsupportedArpOperation
            | PacketError::InvalidIcmpEcho
            | PacketError::InvalidIcmpChecksum
    )
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

fn start_resolved_routed_single_ping_transaction<
    D: NetworkDevice,
    const ARP_CAPACITY: usize,
    const PAYLOAD_CAPACITY: usize,
>(
    device: &mut D,
    arp_cache: &ArpCache<ARP_CAPACITY>,
    transaction: &mut SinglePingTransaction<PAYLOAD_CAPACITY>,
    endpoint: LocalNetworkEndpoint,
    route: Ipv4EgressRouteDecision,
    identifier: u16,
    sequence_number: u16,
    ttl: u8,
    payload: &[u8],
    output: &mut [u8],
) -> SinglePingTransactionStartResult {
    let request = match InflightIcmpEchoRequest::new(
        endpoint,
        route.destination_ipv4(),
        identifier,
        sequence_number,
        payload,
    ) {
        Ok(request) => request,
        Err(error) => return SinglePingTransactionStartResult::InflightResult(error),
    };

    let resolution = resolve_outbound_neighbor(arp_cache, route.next_hop_ipv4());
    let frame_len = match build_outbound_routed_ipv4_icmp_echo_request(
        resolution,
        route.destination_ipv4(),
        endpoint,
        identifier,
        sequence_number,
        ttl,
        payload,
        output,
    ) {
        Ok(frame_len) => frame_len,
        Err(OutboundFrameError::NeighborUnresolved { destination_ipv4 }) => {
            return SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::PendingNeighborUnresolved { destination_ipv4 },
            );
        }
        Err(error) => {
            return SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::RouteError(OutboundRouteError::Frame(error)),
            );
        }
    };

    match device.transmit_frame(&output[..frame_len]) {
        Ok(()) => match transaction.inflight.store(request) {
            Ok(()) => SinglePingTransactionStartResult::IcmpEchoRequestTransmitted { frame_len },
            Err(error) => SinglePingTransactionStartResult::InflightResult(error),
        },
        Err(error) => {
            SinglePingTransactionStartResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
                frame_len,
                error,
            })
        }
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
    fn smoltcp_dependency_core_keeps_tcp_closed_until_device_adapter_exists() {
        let core =
            SmoltcpDependencyCore::new(MacAddress::new([0x02, 0, 0, 0, 0, 1]), [192, 0, 2, 10], 24);

        assert_eq!(
            core.hardware_address(),
            smoltcp::wire::EthernetAddress([0x02, 0, 0, 0, 0, 1])
        );
        assert_eq!(
            core.ipv4_cidr(),
            smoltcp::wire::Ipv4Cidr::new(smoltcp::wire::Ipv4Address::new(192, 0, 2, 10), 24)
        );
        assert_eq!(core.tcp_state(), smoltcp::socket::tcp::State::Closed);
        assert_eq!(
            core.poll_without_device(),
            SmoltcpDependencyCorePollResult::NoDeviceBound
        );
    }

    #[test_case]
    fn smoltcp_packet_device_adapter_moves_receive_and_reply_frames() {
        let mut adapter = SmoltcpPacketDeviceAdapter::<2, 2, 64>::new();
        adapter
            .inject_received(ETHERNET_IPV4_FRAME)
            .expect("inject frame");

        let (rx, tx) =
            smoltcp::phy::Device::receive(&mut adapter, smoltcp::time::Instant::from_millis(1))
                .expect("smoltcp receive token");
        smoltcp::phy::RxToken::consume(rx, |frame| {
            assert_eq!(frame, ETHERNET_IPV4_FRAME);
        });
        smoltcp::phy::TxToken::consume(tx, 3, |frame| {
            frame.copy_from_slice(&[0xaa, 0xbb, 0xcc]);
        });
        assert_eq!(
            adapter.last_receive_result(),
            SmoltcpPacketDeviceAdapterReceiveResult::Received {
                frame_len: ETHERNET_IPV4_FRAME.len(),
            }
        );
        assert_eq!(
            adapter.last_transmit_result(),
            SmoltcpPacketDeviceAdapterTransmitResult::Transmitted { frame_len: 3 }
        );
        assert_eq!(adapter.received_len(), 0);
        assert_eq!(adapter.transmitted_len(), 1);
        assert_eq!(
            adapter
                .pop_transmitted()
                .expect("transmitted reply")
                .as_bytes(),
            &[0xaa, 0xbb, 0xcc]
        );
    }

    #[test_case]
    fn smoltcp_packet_device_adapter_reports_no_frame_and_transmit_queue_pressure() {
        let mut adapter = SmoltcpPacketDeviceAdapter::<2, 1, 64>::new();

        assert!(
            smoltcp::phy::Device::receive(&mut adapter, smoltcp::time::Instant::from_millis(1),)
                .is_none()
        );
        assert_eq!(
            adapter.last_receive_result(),
            SmoltcpPacketDeviceAdapterReceiveResult::NoFrame
        );

        let tx =
            smoltcp::phy::Device::transmit(&mut adapter, smoltcp::time::Instant::from_millis(2))
                .expect("transmit token");
        smoltcp::phy::TxToken::consume(tx, 2, |frame| {
            frame.copy_from_slice(&[1, 2]);
        });
        assert_eq!(
            adapter.last_transmit_result(),
            SmoltcpPacketDeviceAdapterTransmitResult::Transmitted { frame_len: 2 }
        );
        assert!(
            smoltcp::phy::Device::transmit(&mut adapter, smoltcp::time::Instant::from_millis(3))
                .is_none()
        );
        assert_eq!(
            adapter.last_transmit_result(),
            SmoltcpPacketDeviceAdapterTransmitResult::TransmitQueueFull
        );

        adapter
            .inject_received(&[0x10, 0x20])
            .expect("inject held receive frame");
        assert!(
            smoltcp::phy::Device::receive(&mut adapter, smoltcp::time::Instant::from_millis(4),)
                .is_none()
        );
        assert_eq!(
            adapter.last_receive_result(),
            SmoltcpPacketDeviceAdapterReceiveResult::TransmitQueueFull
        );
        assert_eq!(adapter.received_len(), 1);
    }

    #[test_case]
    fn smoltcp_packet_device_adapter_maps_device_errors_and_frame_bounds() {
        let mut adapter = SmoltcpPacketDeviceAdapter::<2, 2, 64>::new();
        adapter
            .inject_received(&[1, 2, 3])
            .expect("inject frame before receive error");
        adapter.set_receive_error(Some(DeviceError::Io));

        assert!(
            smoltcp::phy::Device::receive(&mut adapter, smoltcp::time::Instant::from_millis(1),)
                .is_none()
        );
        assert_eq!(
            adapter.last_receive_result(),
            SmoltcpPacketDeviceAdapterReceiveResult::ReceiveError(DeviceError::Io)
        );
        assert_eq!(adapter.received_len(), 1);

        adapter.set_receive_error(None);
        let (rx, _reply_tx) =
            smoltcp::phy::Device::receive(&mut adapter, smoltcp::time::Instant::from_millis(2))
                .expect("receive after clearing error");
        smoltcp::phy::RxToken::consume(rx, |frame| assert_eq!(frame, &[1, 2, 3]));

        adapter.set_transmit_error(Some(DeviceError::Io));
        let tx =
            smoltcp::phy::Device::transmit(&mut adapter, smoltcp::time::Instant::from_millis(3))
                .expect("transmit token");
        smoltcp::phy::TxToken::consume(tx, 2, |frame| {
            frame.copy_from_slice(&[9, 8]);
        });
        assert_eq!(
            adapter.last_transmit_result(),
            SmoltcpPacketDeviceAdapterTransmitResult::TransmitError(DeviceError::Io)
        );
        assert_eq!(adapter.transmitted_len(), 0);

        adapter.set_transmit_error(None);
        let tx =
            smoltcp::phy::Device::transmit(&mut adapter, smoltcp::time::Instant::from_millis(4))
                .expect("bounded transmit token");
        smoltcp::phy::TxToken::consume(tx, 65, |frame| {
            assert!(frame.is_empty());
        });
        assert_eq!(
            adapter.last_transmit_result(),
            SmoltcpPacketDeviceAdapterTransmitResult::FrameTooLarge {
                required_len: 65,
                max_len: 64,
            }
        );
        assert_eq!(adapter.transmitted_len(), 0);
    }

    #[test_case]
    fn driver_packet_adapter_moves_driver_rx_and_smoltcp_tx_with_copied_frames() {
        let mut adapter = DriverPacketAdapter::<2, 2, 64>::new();
        adapter
            .inject_driver_rx(ETHERNET_IPV4_FRAME)
            .expect("inject driver rx frame");

        assert_eq!(
            adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_millis(10)),
            DriverPacketAdapterReceiveStep::Received {
                frame_len: ETHERNET_IPV4_FRAME.len()
            }
        );
        assert_eq!(adapter.driver_rx_len(), 0);

        assert_eq!(
            adapter.transmit_one_from_smoltcp(
                smoltcp::time::Instant::from_millis(11),
                &[0xde, 0xad, 0xbe, 0xef],
            ),
            DriverPacketAdapterTransmitStep::Transmitted { frame_len: 4 }
        );
        assert_eq!(adapter.driver_tx_len(), 1);
        assert_eq!(
            adapter.pop_driver_tx().expect("driver tx frame").as_bytes(),
            &[0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test_case]
    fn driver_packet_adapter_preserves_rx_when_tx_backpressure_blocks_smoltcp_receive() {
        let mut adapter = DriverPacketAdapter::<1, 1, 64>::new();

        assert_eq!(
            adapter
                .transmit_one_from_smoltcp(smoltcp::time::Instant::from_millis(20), &[0xaa, 0xbb],),
            DriverPacketAdapterTransmitStep::Transmitted { frame_len: 2 }
        );
        adapter
            .inject_driver_rx(&[0x10, 0x20, 0x30])
            .expect("inject driver rx behind full tx queue");

        assert_eq!(
            adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_millis(21)),
            DriverPacketAdapterReceiveStep::TransmitQueueFull
        );
        assert_eq!(adapter.driver_rx_len(), 1);
        assert_eq!(adapter.driver_tx_len(), 1);

        assert_eq!(
            adapter
                .pop_driver_tx()
                .expect("first driver tx frame")
                .as_bytes(),
            &[0xaa, 0xbb]
        );
        assert_eq!(
            adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_millis(22)),
            DriverPacketAdapterReceiveStep::Received { frame_len: 3 }
        );
        assert_eq!(adapter.driver_rx_len(), 0);
    }

    #[test_case]
    fn driver_packet_adapter_maps_capacity_and_device_errors_deterministically() {
        let mut adapter = DriverPacketAdapter::<1, 1, 4>::new();

        assert_eq!(
            adapter.inject_driver_rx(&[1, 2, 3, 4, 5]),
            Err(PacketQueueError::FrameTooLarge {
                required_len: 5,
                max_len: 4,
            })
        );
        assert_eq!(
            adapter.transmit_one_from_smoltcp(
                smoltcp::time::Instant::from_millis(30),
                &[1, 2, 3, 4, 5],
            ),
            DriverPacketAdapterTransmitStep::FrameTooLarge {
                required_len: 5,
                max_len: 4,
            }
        );
        assert_eq!(adapter.driver_tx_len(), 0);

        adapter.set_transmit_error(Some(DeviceError::Io));
        assert_eq!(
            adapter.transmit_one_from_smoltcp(smoltcp::time::Instant::from_millis(31), &[9]),
            DriverPacketAdapterTransmitStep::TransmitError(DeviceError::Io)
        );
        assert_eq!(adapter.driver_tx_len(), 0);
        adapter.set_transmit_error(None);

        adapter.inject_driver_rx(&[7]).expect("inject one rx frame");
        adapter.set_receive_error(Some(DeviceError::Io));
        assert_eq!(
            adapter.receive_one_for_smoltcp(smoltcp::time::Instant::from_millis(32)),
            DriverPacketAdapterReceiveStep::ReceiveError(DeviceError::Io)
        );
        assert_eq!(adapter.driver_rx_len(), 1);
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum SmoltcpHandshakeOutcome {
        Established,
        TimedOut,
        ClientTransmitBackpressure,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct SmoltcpHandshakeObservation {
        outcome: SmoltcpHandshakeOutcome,
        client_state: smoltcp::socket::tcp::State,
        server_state: smoltcp::socket::tcp::State,
        client_to_server_frames: usize,
        server_to_client_frames: usize,
        poll_steps: usize,
    }

    fn make_smoltcp_interface<
        const RX_CAPACITY: usize,
        const TX_CAPACITY: usize,
        const FRAME_CAPACITY: usize,
    >(
        mac: [u8; ETHERNET_ADDR_LEN],
        ipv4: [u8; 4],
        adapter: &mut SmoltcpPacketDeviceAdapter<RX_CAPACITY, TX_CAPACITY, FRAME_CAPACITY>,
    ) -> smoltcp::iface::Interface {
        let mut config = smoltcp::iface::Config::new(smoltcp::wire::EthernetAddress(mac).into());
        config.random_seed = 0x5441_4c4f_5300_0001;
        let mut iface =
            smoltcp::iface::Interface::new(config, adapter, smoltcp::time::Instant::ZERO);
        iface.update_ip_addrs(|ip_addrs| {
            ip_addrs
                .push(smoltcp::wire::IpCidr::new(
                    smoltcp::wire::IpAddress::v4(ipv4[0], ipv4[1], ipv4[2], ipv4[3]),
                    24,
                ))
                .expect("single smoltcp IPv4 address slot remains available");
        });
        iface
    }

    fn move_smoltcp_frames<
        const FROM_RX_CAPACITY: usize,
        const FROM_TX_CAPACITY: usize,
        const TO_RX_CAPACITY: usize,
        const TO_TX_CAPACITY: usize,
        const FRAME_CAPACITY: usize,
    >(
        from: &mut SmoltcpPacketDeviceAdapter<FROM_RX_CAPACITY, FROM_TX_CAPACITY, FRAME_CAPACITY>,
        to: &mut SmoltcpPacketDeviceAdapter<TO_RX_CAPACITY, TO_TX_CAPACITY, FRAME_CAPACITY>,
    ) -> Result<usize, PacketQueueError> {
        let mut moved = 0;
        while let Some(frame) = from.pop_transmitted() {
            to.inject_received(frame.as_bytes())?;
            moved += 1;
        }
        Ok(moved)
    }

    #[allow(clippy::too_many_arguments)]
    fn drive_smoltcp_handshake<
        const CLIENT_RX_CAPACITY: usize,
        const CLIENT_TX_CAPACITY: usize,
        const SERVER_RX_CAPACITY: usize,
        const SERVER_TX_CAPACITY: usize,
        const FRAME_CAPACITY: usize,
    >(
        client_adapter: &mut SmoltcpPacketDeviceAdapter<
            CLIENT_RX_CAPACITY,
            CLIENT_TX_CAPACITY,
            FRAME_CAPACITY,
        >,
        server_adapter: &mut SmoltcpPacketDeviceAdapter<
            SERVER_RX_CAPACITY,
            SERVER_TX_CAPACITY,
            FRAME_CAPACITY,
        >,
        client_iface: &mut smoltcp::iface::Interface,
        server_iface: &mut smoltcp::iface::Interface,
        client_sockets: &mut smoltcp::iface::SocketSet<'_>,
        server_sockets: &mut smoltcp::iface::SocketSet<'_>,
        client_handle: smoltcp::iface::SocketHandle,
        server_handle: smoltcp::iface::SocketHandle,
        max_poll_steps: usize,
    ) -> SmoltcpHandshakeObservation {
        let mut client_to_server_frames = 0;
        let mut server_to_client_frames = 0;

        for step in 0..max_poll_steps {
            let now = smoltcp::time::Instant::from_millis(step as i64);

            client_iface.poll(now, client_adapter, client_sockets);
            if client_adapter.last_transmit_result()
                == SmoltcpPacketDeviceAdapterTransmitResult::TransmitQueueFull
            {
                return SmoltcpHandshakeObservation {
                    outcome: SmoltcpHandshakeOutcome::ClientTransmitBackpressure,
                    client_state: client_sockets
                        .get::<smoltcp::socket::tcp::Socket>(client_handle)
                        .state(),
                    server_state: server_sockets
                        .get::<smoltcp::socket::tcp::Socket>(server_handle)
                        .state(),
                    client_to_server_frames,
                    server_to_client_frames,
                    poll_steps: step + 1,
                };
            }
            client_to_server_frames += move_smoltcp_frames(client_adapter, server_adapter)
                .expect("server rx has capacity");

            server_iface.poll(now, server_adapter, server_sockets);
            server_to_client_frames += move_smoltcp_frames(server_adapter, client_adapter)
                .expect("client rx has capacity");

            client_iface.poll(now, client_adapter, client_sockets);
            client_to_server_frames += move_smoltcp_frames(client_adapter, server_adapter)
                .expect("server rx has capacity");

            server_iface.poll(now, server_adapter, server_sockets);

            let client_state = client_sockets
                .get::<smoltcp::socket::tcp::Socket>(client_handle)
                .state();
            let server_state = server_sockets
                .get::<smoltcp::socket::tcp::Socket>(server_handle)
                .state();
            if client_state == smoltcp::socket::tcp::State::Established
                && server_state == smoltcp::socket::tcp::State::Established
            {
                return SmoltcpHandshakeObservation {
                    outcome: SmoltcpHandshakeOutcome::Established,
                    client_state,
                    server_state,
                    client_to_server_frames,
                    server_to_client_frames,
                    poll_steps: step + 1,
                };
            }
        }

        SmoltcpHandshakeObservation {
            outcome: SmoltcpHandshakeOutcome::TimedOut,
            client_state: client_sockets
                .get::<smoltcp::socket::tcp::Socket>(client_handle)
                .state(),
            server_state: server_sockets
                .get::<smoltcp::socket::tcp::Socket>(server_handle)
                .state(),
            client_to_server_frames,
            server_to_client_frames,
            poll_steps: max_poll_steps,
        }
    }

    #[test_case]
    fn smoltcp_loopback_tcp_handshake_establishes_over_packet_device_adapters() {
        let mut client_adapter = SmoltcpPacketDeviceAdapter::<8, 8, 1536>::new();
        let mut server_adapter = SmoltcpPacketDeviceAdapter::<8, 8, 1536>::new();
        let mut client_iface = make_smoltcp_interface(
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x11],
            [192, 0, 2, 11],
            &mut client_adapter,
        );
        let mut server_iface = make_smoltcp_interface(
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x22],
            [192, 0, 2, 22],
            &mut server_adapter,
        );

        let mut client_rx_storage = [0u8; 256];
        let mut client_tx_storage = [0u8; 256];
        let mut server_rx_storage = [0u8; 256];
        let mut server_tx_storage = [0u8; 256];
        let client_socket = smoltcp::socket::tcp::Socket::new(
            smoltcp::socket::tcp::SocketBuffer::new(&mut client_rx_storage[..]),
            smoltcp::socket::tcp::SocketBuffer::new(&mut client_tx_storage[..]),
        );
        let server_socket = smoltcp::socket::tcp::Socket::new(
            smoltcp::socket::tcp::SocketBuffer::new(&mut server_rx_storage[..]),
            smoltcp::socket::tcp::SocketBuffer::new(&mut server_tx_storage[..]),
        );
        let mut client_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
        let mut server_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
        let mut client_sockets = smoltcp::iface::SocketSet::new(&mut client_socket_storage[..]);
        let mut server_sockets = smoltcp::iface::SocketSet::new(&mut server_socket_storage[..]);
        let client_handle = client_sockets.add(client_socket);
        let server_handle = server_sockets.add(server_socket);

        server_sockets
            .get_mut::<smoltcp::socket::tcp::Socket>(server_handle)
            .listen(8080)
            .expect("server listen succeeds");
        client_sockets
            .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
            .connect(
                client_iface.context(),
                (smoltcp::wire::IpAddress::v4(192, 0, 2, 22), 8080),
                49152,
            )
            .expect("client connect enters SYN-SENT");

        let observation = drive_smoltcp_handshake(
            &mut client_adapter,
            &mut server_adapter,
            &mut client_iface,
            &mut server_iface,
            &mut client_sockets,
            &mut server_sockets,
            client_handle,
            server_handle,
            8,
        );

        assert_eq!(
            observation,
            SmoltcpHandshakeObservation {
                outcome: SmoltcpHandshakeOutcome::Established,
                client_state: smoltcp::socket::tcp::State::Established,
                server_state: smoltcp::socket::tcp::State::Established,
                client_to_server_frames: 3,
                server_to_client_frames: 2,
                poll_steps: 2,
            }
        );
    }

    #[test_case]
    fn smoltcp_loopback_tcp_handshake_reports_client_transmit_backpressure() {
        let mut client_adapter = SmoltcpPacketDeviceAdapter::<4, 0, 1536>::new();
        let mut server_adapter = SmoltcpPacketDeviceAdapter::<4, 4, 1536>::new();
        let mut client_iface = make_smoltcp_interface(
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x31],
            [192, 0, 2, 31],
            &mut client_adapter,
        );
        let mut server_iface = make_smoltcp_interface(
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x32],
            [192, 0, 2, 32],
            &mut server_adapter,
        );

        let mut client_rx_storage = [0u8; 128];
        let mut client_tx_storage = [0u8; 128];
        let mut server_rx_storage = [0u8; 128];
        let mut server_tx_storage = [0u8; 128];
        let client_socket = smoltcp::socket::tcp::Socket::new(
            smoltcp::socket::tcp::SocketBuffer::new(&mut client_rx_storage[..]),
            smoltcp::socket::tcp::SocketBuffer::new(&mut client_tx_storage[..]),
        );
        let server_socket = smoltcp::socket::tcp::Socket::new(
            smoltcp::socket::tcp::SocketBuffer::new(&mut server_rx_storage[..]),
            smoltcp::socket::tcp::SocketBuffer::new(&mut server_tx_storage[..]),
        );
        let mut client_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
        let mut server_socket_storage = [smoltcp::iface::SocketStorage::EMPTY];
        let mut client_sockets = smoltcp::iface::SocketSet::new(&mut client_socket_storage[..]);
        let mut server_sockets = smoltcp::iface::SocketSet::new(&mut server_socket_storage[..]);
        let client_handle = client_sockets.add(client_socket);
        let server_handle = server_sockets.add(server_socket);

        server_sockets
            .get_mut::<smoltcp::socket::tcp::Socket>(server_handle)
            .listen(8081)
            .expect("server listen succeeds");
        client_sockets
            .get_mut::<smoltcp::socket::tcp::Socket>(client_handle)
            .connect(
                client_iface.context(),
                (smoltcp::wire::IpAddress::v4(192, 0, 2, 32), 8081),
                49153,
            )
            .expect("client connect enters SYN-SENT");

        let observation = drive_smoltcp_handshake(
            &mut client_adapter,
            &mut server_adapter,
            &mut client_iface,
            &mut server_iface,
            &mut client_sockets,
            &mut server_sockets,
            client_handle,
            server_handle,
            1,
        );

        assert_eq!(
            observation,
            SmoltcpHandshakeObservation {
                outcome: SmoltcpHandshakeOutcome::ClientTransmitBackpressure,
                client_state: smoltcp::socket::tcp::State::SynSent,
                server_state: smoltcp::socket::tcp::State::Listen,
                client_to_server_frames: 0,
                server_to_client_frames: 0,
                poll_steps: 1,
            }
        );
    }

    #[test_case]
    fn live_tcp_listener_descriptor_boundary_accepts_local_source_bridge_only() {
        let owner = crate::scheduler::ProcessOwnerId::new(77).expect("owner id");
        let endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, 22);
        let mut sockets = NetworkSocketDescriptorTable::<4>::new();
        let listener = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("listener socket");
        sockets
            .bind(owner, listener, endpoint)
            .expect("bind listener");
        sockets
            .listen(owner, listener, 1)
            .expect("listen on local endpoint");
        let client = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("client socket");
        sockets
            .connect(owner, client, endpoint)
            .expect("connect local client to listener");
        let connection_id = match sockets.socket(client).expect("client state").state() {
            NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected client state {state:?}"),
        };

        let pre_accept = sockets
            .live_tcp_listener_descriptor_boundary(connection_id, false)
            .expect("pre-accept boundary report");
        assert_eq!(
            pre_accept.boundary(),
            LiveTcpListenerDescriptorBoundary::BlockedNoDescriptorBridge
        );
        assert!(pre_accept.descriptor_bridge_established());
        assert!(!pre_accept.accepted_descriptor_attached());
        assert!(!pre_accept.ssh_ready());
        let pre_accept_delivery = sockets
            .live_tcp_listener_descriptor_accept_delivery(connection_id, false)
            .expect("pre-accept descriptor delivery report");
        assert_eq!(
            pre_accept_delivery.delivery_state(),
            LiveTcpAcceptedConnectionDeliveryState::BlockedNoDescriptorBridge
        );
        assert!(!pre_accept_delivery.descriptor_facing_connection_delivered());
        assert_eq!(pre_accept_delivery.accepted_descriptor(), None);
        assert_eq!(pre_accept_delivery.accepted_descriptor_state(), None);
        assert!(!pre_accept_delivery.ssh_ready());

        let accepted = sockets
            .accept(owner, listener)
            .expect("accept local client");
        sockets
            .send(owner, client, b"tcp?")
            .expect("send over descriptor bridge");
        let mut recv = [0u8; 4];
        assert_eq!(
            sockets
                .recv_peek(owner, accepted, &mut recv)
                .expect("accepted descriptor receives payload"),
            4
        );
        assert_eq!(&recv, b"tcp?");

        let report = sockets
            .live_tcp_listener_descriptor_boundary(connection_id, false)
            .expect("local source boundary report");
        assert_eq!(
            report.boundary(),
            LiveTcpListenerDescriptorBoundary::AcceptedLocalSourceBoundary
        );
        assert_eq!(
            report.ownership_model(),
            LiveTcpDeviceInterfaceOwnershipModel::NetworkOwnedSmoltcpInterfaceWithDriverPacketAdapterIngressAndDescriptorTableDelivery
        );
        assert_eq!(report.connection_id(), connection_id);
        assert!(report.descriptor_bridge_established());
        assert!(report.accepted_descriptor_attached());
        assert_eq!(report.payload_transfers(), 1);
        assert_eq!(report.last_payload_len(), 4);
        assert_eq!(
            report.device_interface_binding_state(),
            LiveTcpDeviceInterfaceBindingState::LocalSourceBoundaryDoesNotRequireDeviceInterface
        );
        assert!(!report.device_interface_bound());
        assert!(!report.live_packet_io_accepted());
        assert!(!report.live_reachability_accepted());
        assert!(!report.remote_receipt_accepted());
        assert!(!report.compatibility_accepted());
        assert!(!report.ssh_ready());

        let accept_delivery = sockets
            .live_tcp_listener_descriptor_accept_delivery(connection_id, false)
            .expect("local descriptor delivery report");
        assert_eq!(
            accept_delivery.delivery_state(),
            LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
        );
        assert_eq!(
            accept_delivery.boundary().boundary(),
            LiveTcpListenerDescriptorBoundary::AcceptedLocalSourceBoundary
        );
        assert_eq!(accept_delivery.accepted_descriptor(), Some(accepted));
        assert!(matches!(
            accept_delivery.accepted_descriptor_state(),
            Some(NetworkSocketState::Accepted {
                connection_id: accepted_connection_id,
                ..
            }) if accepted_connection_id == connection_id
        ));
        assert!(accept_delivery.descriptor_facing_connection_delivered());
        assert!(!accept_delivery.live_packet_io_accepted());
        assert!(!accept_delivery.live_reachability_accepted());
        assert!(!accept_delivery.remote_receipt_accepted());
        assert!(!accept_delivery.compatibility_accepted());
        assert!(!accept_delivery.ssh_ready());

        let live_required = sockets
            .live_tcp_listener_descriptor_boundary(connection_id, true)
            .expect("device-required boundary report");
        assert_eq!(
            live_required.boundary(),
            LiveTcpListenerDescriptorBoundary::BlockedNoDeviceInterfaceBinding
        );
        assert_eq!(
            live_required.ownership_model(),
            LiveTcpDeviceInterfaceOwnershipModel::NetworkOwnedSmoltcpInterfaceWithDriverPacketAdapterIngressAndDescriptorTableDelivery
        );
        assert_eq!(
            live_required.device_interface_binding_state(),
            LiveTcpDeviceInterfaceBindingState::BlockedMissingDeviceInterfaceBinding
        );
        assert!(!live_required.device_interface_bound());
        assert!(!live_required.live_packet_io_accepted());
        assert!(!live_required.live_reachability_accepted());
        assert!(!live_required.ssh_ready());

        let live_required_delivery = sockets
            .live_tcp_listener_descriptor_accept_delivery(connection_id, true)
            .expect("device-required descriptor delivery report");
        assert_eq!(
            live_required_delivery.delivery_state(),
            LiveTcpAcceptedConnectionDeliveryState::BlockedMissingDeviceInterfaceBinding
        );
        assert_eq!(live_required_delivery.accepted_descriptor(), Some(accepted));
        assert!(!live_required_delivery.descriptor_facing_connection_delivered());
        assert!(!live_required_delivery.live_packet_io_accepted());
        assert!(!live_required_delivery.live_reachability_accepted());
        assert!(!live_required_delivery.remote_receipt_accepted());
        assert!(!live_required_delivery.compatibility_accepted());
        assert!(!live_required_delivery.ssh_ready());
    }

    #[test_case]
    fn live_tcp_network_device_smoltcp_runtime_binding_reaches_descriptor_delivery() {
        let owner = crate::scheduler::ProcessOwnerId::new(78).expect("owner id");
        let endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, 22);
        let mut sockets = NetworkSocketDescriptorTable::<4>::new();
        let listener = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("listener socket");
        sockets
            .bind(owner, listener, endpoint)
            .expect("bind listener");
        sockets.listen(owner, listener, 1).expect("listen");
        let client = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("client socket");
        sockets
            .connect(owner, client, endpoint)
            .expect("connect local client to listener");
        let connection_id = match sockets.socket(client).expect("client state").state() {
            NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected client state {state:?}"),
        };
        let accepted = sockets
            .accept(owner, listener)
            .expect("accept local client");
        sockets
            .send(owner, client, b"tcp-runtime")
            .expect("send over descriptor bridge");
        let mut recv = [0u8; 11];
        assert_eq!(
            sockets
                .recv_peek(owner, accepted, &mut recv)
                .expect("accepted descriptor receives payload"),
            recv.len()
        );

        let host_only = sockets
            .live_tcp_listener_descriptor_accept_delivery(connection_id, false)
            .expect("host-only descriptor delivery report");
        assert_eq!(
            host_only.delivery_state(),
            LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
        );
        assert!(host_only.descriptor_facing_connection_delivered());
        assert!(!host_only.boundary().device_interface_bound());
        assert!(!host_only.live_packet_io_accepted());
        assert!(!host_only.ssh_ready());

        let runtime = sockets
            .live_tcp_network_device_smoltcp_runtime_binding(connection_id, true, false)
            .expect("deterministic runtime binding report");
        assert_eq!(
            runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        );
        assert_eq!(
            runtime.accept_report().delivery_state(),
            LiveTcpAcceptedConnectionDeliveryState::AcceptedLocalDescriptorDelivery
        );
        assert!(runtime.descriptor_facing_connection_delivered());
        assert!(runtime.deterministic_device_interface_bound());
        assert!(!runtime.hardware_frame_provider_bound());
        assert_eq!(runtime.driver_packet_rx_frames(), 6);
        assert_eq!(
            runtime.driver_packet_rx_frames(),
            runtime.driver_packet_tx_frames()
        );
        let observation = runtime
            .runtime_observation()
            .expect("runtime observation present");
        assert_eq!(
            observation.client_state(),
            smoltcp::socket::tcp::State::Established
        );
        assert_eq!(
            observation.server_state(),
            smoltcp::socket::tcp::State::Established
        );
        assert_eq!(
            observation.payload_len(),
            LIVE_TCP_RUNTIME_DRIVER_PACKET_PAYLOAD.len()
        );
        assert!(!runtime.live_packet_io_accepted());
        assert!(!runtime.live_reachability_accepted());
        assert!(!runtime.remote_receipt_accepted());
        assert!(!runtime.compatibility_accepted());
        assert!(!runtime.ssh_ready());
    }

    #[test_case]
    fn live_tcp_network_device_smoltcp_runtime_binding_accepts_source_bound_rp1_provider_only() {
        let owner = crate::scheduler::ProcessOwnerId::new(80).expect("owner id");
        let endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, 22);
        let mut sockets = NetworkSocketDescriptorTable::<4>::new();
        let listener = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("listener socket");
        sockets
            .bind(owner, listener, endpoint)
            .expect("bind listener");
        sockets.listen(owner, listener, 1).expect("listen");
        let client = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("client socket");
        sockets
            .connect(owner, client, endpoint)
            .expect("connect local client to listener");
        let connection_id = match sockets.socket(client).expect("client state").state() {
            NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected client state {state:?}"),
        };
        let accepted = sockets
            .accept(owner, listener)
            .expect("accept local client");
        sockets
            .send(owner, client, b"tcp-runtime")
            .expect("send over descriptor bridge");
        let mut recv = [0u8; 11];
        assert_eq!(
            sockets
                .recv_peek(owner, accepted, &mut recv)
                .expect("accepted descriptor receives payload"),
            recv.len()
        );

        let provider = crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(
            Some(
                crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_contract_evidence(
                    crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderState::SourceBoundLinkReady,
                ),
            ),
        );
        let runtime = sockets
            .live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
                connection_id,
                true,
                provider,
            )
            .expect("source-bound RP1 provider report");
        assert_eq!(
            runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        );
        assert!(runtime.descriptor_facing_connection_delivered());
        assert!(runtime.deterministic_device_interface_bound());
        assert!(runtime.hardware_frame_provider_bound());
        assert_eq!(
            runtime.hardware_frame_provider_classification(),
            Some(crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_BOUND_CLASSIFICATION)
        );
        assert_eq!(
            runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert_eq!(runtime.driver_packet_rx_frames(), 6);
        assert_eq!(
            runtime.driver_packet_rx_frames(),
            runtime.driver_packet_tx_frames()
        );
        assert!(!runtime.live_packet_io_accepted());
        assert!(!runtime.live_reachability_accepted());
        assert!(!runtime.remote_receipt_accepted());
        assert!(!runtime.compatibility_accepted());
        assert!(!runtime.ssh_ready());
    }

    #[test_case]
    fn live_tcp_runtime_marker_route_report_reaches_fail_closed_runtime_path() {
        let report = live_tcp_runtime_marker_route_report().expect("runtime marker route");
        let runtime = report.runtime_report();
        assert!(report.marker_route_ready());
        assert_eq!(
            runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        );
        assert!(runtime.descriptor_facing_connection_delivered());
        assert!(runtime.deterministic_device_interface_bound());
        assert!(!runtime.hardware_frame_provider_bound());
        assert_eq!(
            runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_DETERMINISTIC_HOST_ONLY
        );
        assert!(runtime.driver_packet_rx_frames() > 0);
        assert_eq!(
            runtime.driver_packet_rx_frames(),
            runtime.driver_packet_tx_frames()
        );
        assert!(!runtime.live_packet_io_accepted());
        assert!(!runtime.live_reachability_accepted());
        assert!(!runtime.remote_receipt_accepted());
        assert!(!runtime.compatibility_accepted());
        assert!(!runtime.ssh_ready());
    }

    #[test_case]
    fn live_tcp_runtime_marker_route_report_accepts_source_bound_rp1_provider_metadata_only() {
        let report = live_tcp_runtime_marker_route_report_with_source_bound_rp1_provider()
            .expect("provider-bound runtime marker route");
        let runtime = report.runtime_report();

        assert!(report.marker_route_ready());
        assert_eq!(
            runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        );
        assert!(runtime.descriptor_facing_connection_delivered());
        assert!(runtime.deterministic_device_interface_bound());
        assert!(runtime.hardware_frame_provider_bound());
        assert_eq!(
            runtime.hardware_frame_provider_classification(),
            Some(crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_BOUND_CLASSIFICATION)
        );
        assert_eq!(
            runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert!(runtime.driver_packet_rx_frames() > 0);
        assert_eq!(
            runtime.driver_packet_rx_frames(),
            runtime.driver_packet_tx_frames()
        );
        assert!(!runtime.live_packet_io_accepted());
        assert!(!runtime.live_reachability_accepted());
        assert!(!runtime.remote_receipt_accepted());
        assert!(!runtime.compatibility_accepted());
        assert!(!runtime.ssh_ready());
    }

    #[test_case]
    fn live_tcp_runtime_marker_route_report_accepts_rp1_dma_rx_descriptor_ring_metadata_handoff() {
        let handoff =
            live_tcp_runtime_marker_route_report_with_source_owned_rp1_dma_rx_descriptor_ring()
                .expect("descriptor-ring metadata handoff route");
        let runtime = handoff.runtime_report();

        assert_eq!(
            handoff.descriptor_ring_classification(),
            crate::rp1_ethernet::RP1_ETHERNET_DMA_RX_DESCRIPTOR_RING_SOURCE_READY_CLASSIFICATION
        );
        assert_eq!(
            handoff.descriptor_ring_owner(),
            crate::rp1_ethernet::RP1_ETHERNET_DMA_RX_DESCRIPTOR_RING_OWNER
        );
        assert!(handoff.metadata_handoff_ready());
        assert!(handoff.driver_packet_adapter_handoff_ready());
        assert_eq!(handoff.frame_metadata_len(), 64);
        assert!(!handoff.packet_payload_available());
        assert_eq!(
            handoff.redaction_policy(),
            crate::rp1_ethernet::RP1_ETHERNET_DMA_RX_REDACTION_POLICY
        );
        assert_eq!(
            runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::AcceptedDeterministicDeviceInterfaceDelivery
        );
        assert!(runtime.descriptor_facing_connection_delivered());
        assert!(runtime.deterministic_device_interface_bound());
        assert!(runtime.hardware_frame_provider_bound());
        assert_eq!(
            runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert!(runtime.driver_packet_rx_frames() > 0);
        assert_eq!(
            runtime.driver_packet_rx_frames(),
            runtime.driver_packet_tx_frames()
        );
        assert!(!handoff.live_packet_io_accepted());
        assert!(!handoff.live_reachability_accepted());
        assert!(!handoff.remote_receipt_accepted());
        assert!(!handoff.compatibility_accepted());
        assert!(!handoff.ssh_ready());
    }

    #[test_case]
    fn live_tcp_runtime_marker_route_report_fails_closed_for_rp1_dma_rx_descriptor_ring_without_metadata()
     {
        let ring_report = crate::rp1_ethernet::rp1_ethernet_dma_rx_descriptor_ring_report(
            crate::rp1_ethernet::Rp1EthernetDmaRxDescriptorRingState::NoCompletedFrame,
            None,
        );
        let handoff = live_tcp_runtime_marker_route_report_with_rp1_dma_rx_descriptor_ring_report(
            ring_report,
        )
        .expect("no-frame descriptor-ring route");

        assert_eq!(
            handoff.descriptor_ring_classification(),
            crate::rp1_ethernet::RP1_ETHERNET_DMA_RX_DESCRIPTOR_RING_NO_COMPLETED_FRAME_CLASSIFICATION
        );
        assert!(!handoff.metadata_handoff_ready());
        assert!(!handoff.driver_packet_adapter_handoff_ready());
        assert_eq!(handoff.frame_metadata_len(), 0);
        assert!(!handoff.live_packet_io_accepted());
        assert!(!handoff.ssh_ready());
    }

    #[test_case]
    fn bounded_packet_stimulus_contract_accepts_metadata_only_lab_stimulus_boundary() {
        let report = live_tcp_bounded_packet_stimulus_contract_report()
            .expect("bounded packet stimulus contract");

        assert_eq!(report.contract_id(), LIVE_PACKET_STIMULUS_CONTRACT_ID);
        assert_eq!(
            report.classification(),
            LIVE_PACKET_STIMULUS_READY_CLASSIFICATION
        );
        assert_eq!(
            report.permitted_stimulus_source(),
            LIVE_PACKET_STIMULUS_PERMITTED_SOURCE
        );
        assert_eq!(report.nonce_strategy(), LIVE_PACKET_STIMULUS_NONCE_STRATEGY);
        assert_eq!(
            report.payload_redaction_policy(),
            LIVE_PACKET_STIMULUS_PAYLOAD_REDACTION_POLICY
        );
        assert_eq!(report.timing_window(), LIVE_PACKET_STIMULUS_TIMING_WINDOW);
        assert!(report.expected_report_fields().contains(&"nonce-sha256"));
        assert!(report
            .distinguishing_rules()
            .contains(&"deterministic host-only DriverPacketAdapter delivery remains a regression/control surface only"));
        assert!(report.descriptor_ring_handoff_ready());
        assert_eq!(
            report.deterministic_host_only_discriminator(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_DETERMINISTIC_HOST_ONLY
        );
        assert!(report.distinguishes_lab_stimulus_from_host_only());
        assert!(!report.packet_payload_retained());
        assert!(!report.live_packet_io_accepted());
        assert!(!report.live_reachability_accepted());
        assert!(!report.remote_receipt_accepted());
        assert!(!report.compatibility_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn bounded_packet_stimulus_contract_fails_closed_without_descriptor_metadata_handoff() {
        let ring_report = crate::rp1_ethernet::rp1_ethernet_dma_rx_descriptor_ring_report(
            crate::rp1_ethernet::Rp1EthernetDmaRxDescriptorRingState::NoCompletedFrame,
            None,
        );
        let handoff = live_tcp_runtime_marker_route_report_with_rp1_dma_rx_descriptor_ring_report(
            ring_report,
        )
        .expect("no-frame descriptor-ring route");
        let report = bounded_packet_stimulus_contract_report_for_handoff(handoff);

        assert_eq!(
            report.classification(),
            LIVE_PACKET_STIMULUS_BLOCKED_CLASSIFICATION
        );
        assert!(!report.descriptor_ring_handoff_ready());
        assert!(!report.distinguishes_lab_stimulus_from_host_only());
        assert!(!report.live_packet_io_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn live_tcp_runtime_marker_route_report_fails_closed_for_missing_or_paused_rp1_provider() {
        let missing_provider =
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(None);
        let missing_report =
            live_tcp_runtime_marker_route_report_for_rp1_provider(Some(missing_provider))
                .expect("missing provider runtime marker route");
        let missing_runtime = missing_report.runtime_report();
        assert!(!missing_report.marker_route_ready());
        assert_eq!(
            missing_runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingHardwareFrameProvider
        );
        assert!(!missing_runtime.hardware_frame_provider_bound());
        assert_eq!(
            missing_runtime.hardware_frame_provider_classification(),
            Some(crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_MISSING_CLASSIFICATION)
        );
        assert_eq!(
            missing_runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert!(!missing_runtime.ssh_ready());

        let link_not_ready =
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(Some(
                crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_contract_evidence(
                    crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderState::SourceBoundLinkNotReady,
                ),
            ));
        let link_not_ready_report =
            live_tcp_runtime_marker_route_report_for_rp1_provider(Some(link_not_ready))
                .expect("link-not-ready provider runtime marker route");
        let link_not_ready_runtime = link_not_ready_report.runtime_report();
        assert!(!link_not_ready_report.marker_route_ready());
        assert_eq!(
            link_not_ready_runtime.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedHardwareFrameProviderLinkNotReady
        );
        assert!(link_not_ready_runtime.hardware_frame_provider_bound());
        assert_eq!(
            link_not_ready_runtime.hardware_frame_provider_classification(),
            Some(
                crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_LINK_NOT_READY_CLASSIFICATION
            )
        );
        assert_eq!(
            link_not_ready_runtime.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_PROVIDER_LINK_NOT_READY
        );
        assert_eq!(link_not_ready_runtime.driver_packet_rx_frames(), 0);
        assert!(!link_not_ready_runtime.live_packet_io_accepted());
        assert!(!link_not_ready_runtime.ssh_ready());
    }

    #[test_case]
    fn live_tcp_network_device_smoltcp_runtime_binding_fails_closed_without_runtime_prerequisites()
    {
        let owner = crate::scheduler::ProcessOwnerId::new(79).expect("owner id");
        let endpoint = Ipv4Endpoint::new(SOCKET_SYNTHETIC_LOCAL_IPV4_BE, 22);
        let mut sockets = NetworkSocketDescriptorTable::<4>::new();
        let listener = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("listener socket");
        sockets
            .bind(owner, listener, endpoint)
            .expect("bind listener");
        sockets.listen(owner, listener, 1).expect("listen");
        let client = sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("client socket");
        sockets
            .connect(owner, client, endpoint)
            .expect("connect local client to listener");
        let connection_id = match sockets.socket(client).expect("client state").state() {
            NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected client state {state:?}"),
        };

        let missing_descriptor = sockets
            .live_tcp_network_device_smoltcp_runtime_binding(connection_id, true, false)
            .expect("missing descriptor report");
        assert_eq!(
            missing_descriptor.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDescriptorDelivery
        );
        assert_eq!(
            missing_descriptor.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING
        );
        assert!(!missing_descriptor.descriptor_facing_connection_delivered());
        assert!(!missing_descriptor.deterministic_device_interface_bound());
        assert!(!missing_descriptor.live_reachability_accepted());
        assert!(!missing_descriptor.ssh_ready());

        let accepted = sockets
            .accept(owner, listener)
            .expect("accept local client");
        sockets
            .send(owner, client, b"tcp-runtime")
            .expect("send over descriptor bridge");
        let mut recv = [0u8; 11];
        assert_eq!(
            sockets
                .recv_peek(owner, accepted, &mut recv)
                .expect("accepted descriptor receives payload"),
            recv.len()
        );

        let missing_interface = sockets
            .live_tcp_network_device_smoltcp_runtime_binding(connection_id, false, false)
            .expect("missing interface report");
        assert_eq!(
            missing_interface.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDeviceInterfaceBinding
        );
        assert!(missing_interface.descriptor_facing_connection_delivered());
        assert!(!missing_interface.deterministic_device_interface_bound());
        assert_eq!(missing_interface.driver_packet_rx_frames(), 0);
        assert_eq!(
            missing_interface.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_RUNTIME_PREREQUISITE_MISSING
        );
        assert!(!missing_interface.live_packet_io_accepted());
        assert!(!missing_interface.ssh_ready());

        let missing_hardware = sockets
            .live_tcp_network_device_smoltcp_runtime_binding(connection_id, true, true)
            .expect("missing hardware provider report");
        assert_eq!(
            missing_hardware.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingHardwareFrameProvider
        );
        assert!(missing_hardware.descriptor_facing_connection_delivered());
        assert!(missing_hardware.deterministic_device_interface_bound());
        assert!(!missing_hardware.hardware_frame_provider_bound());
        assert_eq!(missing_hardware.driver_packet_tx_frames(), 0);
        assert_eq!(
            missing_hardware.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert!(!missing_hardware.remote_receipt_accepted());
        assert!(!missing_hardware.compatibility_accepted());
        assert!(!missing_hardware.ssh_ready());

        let missing_provider =
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(None);
        let missing_provider_report = sockets
            .live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
                connection_id,
                true,
                missing_provider,
            )
            .expect("missing RP1 provider report");
        assert_eq!(
            missing_provider_report.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingHardwareFrameProvider
        );
        assert!(!missing_provider_report.hardware_frame_provider_bound());
        assert_eq!(
            missing_provider_report.hardware_frame_provider_classification(),
            Some(crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_MISSING_CLASSIFICATION)
        );
        assert_eq!(
            missing_provider_report.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_NO_LIVE_FRAME_PROVIDER
        );
        assert!(!missing_provider_report.ssh_ready());

        let link_not_ready =
            crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_binding_report(Some(
                crate::rp1_ethernet::rp1_ethernet_hardware_frame_provider_contract_evidence(
                    crate::rp1_ethernet::Rp1EthernetHardwareFrameProviderState::SourceBoundLinkNotReady,
                ),
            ));
        let link_not_ready_report = sockets
            .live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
                connection_id,
                true,
                link_not_ready,
            )
            .expect("link-not-ready RP1 provider report");
        assert_eq!(
            link_not_ready_report.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedHardwareFrameProviderLinkNotReady
        );
        assert!(link_not_ready_report.hardware_frame_provider_bound());
        assert_eq!(
            link_not_ready_report.hardware_frame_provider_classification(),
            Some(
                crate::rp1_ethernet::RP1_ETHERNET_HARDWARE_FRAME_PROVIDER_LINK_NOT_READY_CLASSIFICATION
            )
        );
        assert_eq!(link_not_ready_report.driver_packet_rx_frames(), 0);
        assert_eq!(
            link_not_ready_report.live_packet_ingress_discriminator_classification(),
            LIVE_PACKET_INGRESS_DISCRIMINATOR_PROVIDER_LINK_NOT_READY
        );
        assert!(!link_not_ready_report.live_packet_io_accepted());
        assert!(!link_not_ready_report.ssh_ready());

        let mut no_accept_sockets = NetworkSocketDescriptorTable::<4>::new();
        let no_accept_listener = no_accept_sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("no-accept listener socket");
        no_accept_sockets
            .bind(owner, no_accept_listener, endpoint)
            .expect("bind no-accept listener");
        no_accept_sockets
            .listen(owner, no_accept_listener, 1)
            .expect("listen no-accept");
        let no_accept_client = no_accept_sockets
            .open(
                owner,
                SOCKET_DOMAIN_AF_INET,
                SOCKET_TYPE_STREAM,
                SOCKET_PROTOCOL_DEFAULT,
            )
            .expect("no-accept client socket");
        no_accept_sockets
            .connect(owner, no_accept_client, endpoint)
            .expect("connect no-accept client");
        let no_accept_connection_id = match no_accept_sockets
            .socket(no_accept_client)
            .expect("no-accept client state")
            .state()
        {
            NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected no-accept client state {state:?}"),
        };
        let missing_descriptor_report = no_accept_sockets
            .live_tcp_network_device_smoltcp_runtime_binding_with_rp1_provider(
                no_accept_connection_id,
                true,
                link_not_ready,
            )
            .expect("missing descriptor report");
        assert_eq!(
            missing_descriptor_report.binding_state(),
            LiveTcpNetworkDeviceRuntimeBindingState::BlockedMissingDescriptorDelivery
        );
        assert!(!missing_descriptor_report.hardware_frame_provider_bound());
    }

    #[test_case]
    fn packet_queue_driver_pump_drains_outbound_before_polling_receive_fifo() {
        let mut queue = PacketQueueNetworkDevice::<2, 2, 64>::new();
        let mut driver = PacketQueueNetworkDevice::<2, 2, 64>::new();
        let mut receive_buffer = [0u8; 64];

        assert_eq!(queue.transmit_frame(&[1, 2, 3]), Ok(()));
        assert_eq!(queue.transmit_frame(&[4, 5]), Ok(()));
        assert_eq!(driver.inject_received(&[9, 8]), Ok(()));

        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::Transmitted { frame_len: 3 }
        );
        assert_eq!(queue.transmitted_len(), 1);
        assert_eq!(
            driver
                .pop_transmitted()
                .expect("first driver transmit")
                .as_bytes(),
            &[1, 2, 3]
        );

        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::Transmitted { frame_len: 2 }
        );
        assert_eq!(queue.transmitted_len(), 0);
        assert_eq!(
            driver
                .pop_transmitted()
                .expect("second driver transmit")
                .as_bytes(),
            &[4, 5]
        );

        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::Received { frame_len: 2 }
        );
        assert_eq!(queue.received_len(), 1);
        assert_eq!(driver.received_len(), 0);
        assert_eq!(
            queue
                .receive_frame(&mut receive_buffer)
                .expect("queued receive"),
            &[9, 8]
        );
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::NoFrame
        );
    }

    #[test_case]
    fn packet_queue_driver_pump_reports_backpressure_and_device_errors_deterministically() {
        let mut queue = PacketQueueNetworkDevice::<1, 1, 2>::new();
        let mut driver = PacketQueueNetworkDevice::<2, 2, 4>::new();
        let mut receive_buffer = [0u8; 4];

        assert_eq!(queue.transmit_frame(&[1, 2]), Ok(()));
        driver.set_transmit_error(Some(DeviceError::Io));
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::TransmitError {
                frame_len: 2,
                error: DeviceError::Io,
            }
        );
        assert_eq!(queue.transmitted_len(), 1);
        driver.set_transmit_error(None);
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::Transmitted { frame_len: 2 }
        );

        assert_eq!(queue.inject_received(&[0xaa]), Ok(()));
        assert_eq!(driver.inject_received(&[3, 4]), Ok(()));
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::ReceiveQueueFull
        );
        assert_eq!(queue.received_len(), 1);
        assert_eq!(driver.received_len(), 1);
        assert_eq!(
            queue.receive_frame(&mut receive_buffer).expect("drain rx"),
            &[0xaa]
        );

        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer[..1]),
            PacketQueueDriverPumpStep::ReceiveBufferTooSmall
        );
        assert_eq!(driver.received_len(), 1);
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::Received { frame_len: 2 }
        );

        assert_eq!(
            queue.receive_frame(&mut receive_buffer).expect("drain rx"),
            &[3, 4]
        );
        assert_eq!(driver.inject_received(&[5, 6, 7]), Ok(()));
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::ReceiveFrameTooLarge {
                required_len: 3,
                max_len: 2,
            }
        );
        assert_eq!(driver.received_len(), 0);

        driver.set_receive_error(Some(DeviceError::Io));
        assert_eq!(
            queue.pump_driver(&mut driver, &mut receive_buffer),
            PacketQueueDriverPumpStep::ReceiveError(DeviceError::Io)
        );
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
        receive_attempts: usize,
        transmit_error: Option<DeviceError>,
        transmitted: [u8; 128],
        transmitted_len: usize,
    }

    impl<'a> PollDevice<'a> {
        fn with_frame(frame: &'a [u8]) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                receive_attempts: 0,
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_receive_error(error: DeviceError) -> Self {
            Self {
                frame: None,
                receive_error: Some(error),
                receive_attempts: 0,
                transmit_error: None,
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }

        fn with_transmit_error(frame: &'a [u8], error: DeviceError) -> Self {
            Self {
                frame: Some(frame),
                receive_error: None,
                receive_attempts: 0,
                transmit_error: Some(error),
                transmitted: [0; 128],
                transmitted_len: 0,
            }
        }
    }

    impl<'a> NetworkDevice for PollDevice<'a> {
        fn receive_frame<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8], DeviceError> {
            self.receive_attempts += 1;
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
    fn integrated_single_ping_starts_resolved_route_and_completes_matching_reply() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update(destination, destination_mac),
            ArpCacheUpdate::Inserted
        );
        let mut transaction = SinglePingTransaction::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let payload = [1, 2, 3, 4];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();

        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut output,
            ),
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted {
                frame_len: expected_len,
            }
        );
        assert_eq!(transaction.pending(), None);
        let inflight = transaction.inflight().expect("inflight recorded");
        assert_eq!(inflight.destination_ipv4(), destination);
        assert_eq!(inflight.identifier(), 0x1234);
        assert_eq!(inflight.sequence_number(), 7);
        assert_eq!(inflight.payload(), &payload);
        assert_eq!(device.transmit_attempts, 1);

        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("icmp");
        assert_eq!(frame.destination(), destination_mac);
        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("ipv4");
        assert_eq!(ipv4.destination(), destination);
        assert_eq!(frame.payload()[8], 61);

        let reply = icmp_echo_reply_frame();
        let mut poll_device = PollDevice::with_frame(&reply);
        let mut receive_buffer = [0u8; 128];
        assert_eq!(
            poll_single_ping_transaction(
                &mut poll_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::IcmpEchoReplyMatched {
                        payload_len: payload.len(),
                    }
                )
            )
        );
        assert_eq!(transaction.inflight(), None);
    }

    #[test_case]
    fn integrated_single_ping_arps_unresolved_route_then_records_inflight_after_icmp_transmit() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let mut cache = ArpCache::<2>::new();
        let mut transaction = SinglePingTransaction::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let payload = [1, 2, 3, 4];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);

        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut output,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(transaction.inflight(), None);
        let pending = transaction.pending().expect("pending recorded");
        assert_eq!(pending.destination_ipv4(), destination);
        assert_eq!(pending.next_hop_ipv4(), destination);

        let arp_frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        assert_eq!(arp_frame.ether_type(), EtherType::Arp);
        let arp = ArpPacket::parse_ethernet_ipv4(arp_frame.payload()).expect("arp packet");
        assert_eq!(arp.target_protocol_address(), destination);

        let reply = arp_reply_frame();
        let mut poll_device = PollDevice::with_frame(&reply);
        let mut receive_buffer = [0u8; 128];
        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        assert_eq!(
            poll_single_ping_transaction(
                &mut poll_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(
                    PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                        frame_len: expected_len,
                    }
                )
            )
        );
        assert_eq!(transaction.pending(), None);
        let inflight = transaction.inflight().expect("inflight recorded");
        assert_eq!(inflight.destination_ipv4(), destination);
        assert_eq!(inflight.payload(), &payload);
        assert_eq!(
            cache.lookup(destination),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
        let icmp_frame =
            EthernetFrame::parse(&poll_device.transmitted[..poll_device.transmitted_len])
                .expect("icmp");
        assert_eq!(
            icmp_frame.destination(),
            MacAddress::new([0x02, 0, 0, 0, 0, 20])
        );

        let reply = icmp_echo_reply_frame();
        let mut reply_device = PollDevice::with_frame(&reply);
        assert_eq!(
            poll_single_ping_transaction(
                &mut reply_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::IcmpEchoReplyMatched {
                        payload_len: payload.len(),
                    }
                )
            )
        );
        assert_eq!(transaction.pending(), None);
        assert_eq!(transaction.inflight(), None);
    }

    #[test_case]
    fn integrated_single_ping_preserves_state_on_start_and_pending_error_boundaries() {
        let endpoint = local_endpoint();
        let destination = [198, 51, 100, 7];
        let mut cache = ArpCache::<2>::new();
        let mut transaction = SinglePingTransaction::<2>::new();
        let mut output = [0xaa; 128];
        let mut device = OutboundTransmitDevice::new();
        let no_gateway = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);

        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                no_gateway,
                destination,
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            SinglePingTransactionStartResult::PendingResult(PendingIcmpEchoResult::RouteError(
                OutboundRouteError::NoRouteToDestination {
                    destination_ipv4: destination,
                }
            ))
        );
        assert_eq!(transaction.pending(), None);
        assert_eq!(transaction.inflight(), None);
        assert_eq!(device.transmit_attempts, 0);
        assert_eq!(output, [0xaa; 128]);

        let gateway = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let gateway_mac = MacAddress::new([0x02, 0, 0, 0, 0, 254]);
        assert_eq!(
            cache.insert_or_update([192, 0, 2, 254], gateway_mac),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                gateway,
                destination,
                1,
                2,
                64,
                &[1, 2, 3],
                &mut output,
            ),
            SinglePingTransactionStartResult::InflightResult(
                InflightIcmpEchoResult::InflightPayloadTooLarge {
                    required_len: 3,
                    max_len: 2,
                }
            )
        );
        assert_eq!(transaction.inflight(), None);
        assert_eq!(device.transmit_attempts, 0);

        let mut small_output = [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                gateway,
                destination,
                1,
                2,
                64,
                &[1, 2],
                &mut small_output,
            ),
            SinglePingTransactionStartResult::PendingResult(PendingIcmpEchoResult::RouteError(
                OutboundRouteError::Frame(OutboundFrameError::OutputBufferTooSmall {
                    required_len: ETHERNET_HEADER_LEN
                        + IPV4_MIN_HEADER_LEN
                        + ICMP_ECHO_HEADER_LEN
                        + 2,
                    available_len: small_output.len(),
                })
            ))
        );
        assert_eq!(transaction.inflight(), None);
        assert_eq!(device.transmit_attempts, 0);

        let mut failing = OutboundTransmitDevice::with_transmit_error(DeviceError::Io);
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut failing,
                &cache,
                &mut transaction,
                endpoint,
                gateway,
                destination,
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            SinglePingTransactionStartResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 2,
                error: DeviceError::Io,
            })
        );
        assert_eq!(transaction.inflight(), None);
        assert_eq!(failing.transmit_attempts, 1);

        let mut unresolved = SinglePingTransaction::<4>::new();
        let mut arp_device = OutboundTransmitDevice::new();
        let same_subnet = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut arp_device,
                &ArpCache::<0>::new(),
                &mut unresolved,
                endpoint,
                same_subnet,
                [192, 0, 2, 44],
                1,
                2,
                64,
                &[1, 2],
                &mut output,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut arp_device,
                &ArpCache::<0>::new(),
                &mut unresolved,
                endpoint,
                same_subnet,
                [192, 0, 2, 45],
                3,
                4,
                64,
                &[3, 4],
                &mut output,
            ),
            SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::PendingRequestAlreadyQueued {
                    destination_ipv4: [192, 0, 2, 44],
                }
            )
        );

        let mut receive_error = PollDevice::with_receive_error(DeviceError::Io);
        let mut receive_buffer = [0u8; 128];
        assert_eq!(
            poll_single_ping_transaction(
                &mut receive_error,
                &mut ArpCache::<2>::new(),
                &mut unresolved,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::ReceiveError(DeviceError::Io)
            )
        );
        assert_eq!(unresolved.pending_destination_ipv4(), Some([192, 0, 2, 44]));

        let nonmatching_arp_frame = arp_reply_frame();
        let mut nonmatching_arp = PollDevice::with_frame(&nonmatching_arp_frame);
        assert_eq!(
            poll_single_ping_transaction(
                &mut nonmatching_arp,
                &mut ArpCache::<2>::new(),
                &mut unresolved,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::NonMatchingArp {
                    pending_destination_ipv4: [192, 0, 2, 44],
                    arp_sender_ipv4: [192, 0, 2, 20],
                })
            )
        );
        assert_eq!(unresolved.pending_destination_ipv4(), Some([192, 0, 2, 44]));

        let truncated = &arp_reply_frame()[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        let mut malformed_arp = PollDevice::with_frame(truncated);
        assert_eq!(
            poll_single_ping_transaction(
                &mut malformed_arp,
                &mut ArpCache::<2>::new(),
                &mut unresolved,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::ArpError(
                    PacketError::Truncated
                ))
            )
        );
        assert_eq!(unresolved.pending_destination_ipv4(), Some([192, 0, 2, 44]));
    }

    #[test_case]
    fn integrated_single_ping_preserves_inflight_on_duplicate_and_nonmatching_replies() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update(destination, destination_mac),
            ArpCacheUpdate::Inserted
        );
        let mut transaction = SinglePingTransaction::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        let payload = [1, 2, 3, 4];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);

        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut output,
            ),
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 4,
            }
        );
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                [192, 0, 2, 44],
                0x1234,
                7,
                61,
                &payload,
                &mut output,
            ),
            SinglePingTransactionStartResult::InflightResult(
                InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                    destination_ipv4: destination,
                }
            )
        );
        assert_eq!(transaction.inflight_destination_ipv4(), Some(destination));

        let mut nonmatching_reply = icmp_echo_reply_frame();
        nonmatching_reply[ETHERNET_HEADER_LEN + 12..ETHERNET_HEADER_LEN + 16]
            .copy_from_slice(&[192, 0, 2, 21]);
        rewrite_ipv4_checksum(&mut nonmatching_reply);
        let mut poll_device = PollDevice::with_frame(&nonmatching_reply);
        let mut receive_buffer = [0u8; 128];
        assert_eq!(
            poll_single_ping_transaction(
                &mut poll_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
                        destination_ipv4: destination,
                    }
                )
            )
        );
        assert_eq!(transaction.inflight_destination_ipv4(), Some(destination));

        let mut receive_error = PollDevice::with_receive_error(DeviceError::Io);
        assert_eq!(
            poll_single_ping_transaction(
                &mut receive_error,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ReceiveError(DeviceError::Io)
            )
        );
        assert_eq!(transaction.inflight_destination_ipv4(), Some(destination));
    }

    #[test_case]
    fn integrated_single_ping_retries_pending_arp_with_caller_owned_budget() {
        let endpoint = local_endpoint();
        let destination = [198, 51, 100, 7];
        let gateway = [192, 0, 2, 254];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some(gateway));
        let mut transaction = SinglePingTransaction::<4>::new();
        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();

        assert_eq!(
            start_routed_single_ping_transaction_with_arp_retry_budget(
                &mut device,
                &ArpCache::<0>::new(),
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut output,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            transaction.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: gateway,
                arp_retries_remaining: 1,
            }
        );

        assert_eq!(
            retry_single_ping_transaction_arp_request(&mut device, &mut transaction, &mut output),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                    frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                }
            )
        );
        assert_eq!(device.transmit_attempts, 2);
        assert_eq!(
            transaction.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: gateway,
                arp_retries_remaining: 0,
            }
        );
        let arp_frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp");
        let arp = ArpPacket::parse_ethernet_ipv4(arp_frame.payload()).expect("arp packet");
        assert_eq!(arp.target_protocol_address(), gateway);

        assert_eq!(
            retry_single_ping_transaction_arp_request(&mut device, &mut transaction, &mut output),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRetryBudgetExhausted {
                    destination_ipv4: destination,
                    next_hop_ipv4: gateway,
                }
            )
        );
        assert_eq!(device.transmit_attempts, 2);

        let mut failing_transaction = SinglePingTransaction::<4>::new();
        let mut setup_device = OutboundTransmitDevice::new();
        assert_eq!(
            start_routed_single_ping_transaction_with_arp_retry_budget(
                &mut setup_device,
                &ArpCache::<0>::new(),
                &mut failing_transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut output,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        let mut failing_retry = OutboundTransmitDevice::with_transmit_error(DeviceError::Io);
        assert_eq!(
            retry_single_ping_transaction_arp_request(
                &mut failing_retry,
                &mut failing_transaction,
                &mut output,
            ),
            SinglePingTransactionRetryResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::ArpRequest,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                error: DeviceError::Io,
            })
        );
        assert_eq!(
            failing_transaction.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: gateway,
                arp_retries_remaining: 1,
            }
        );
    }

    #[test_case]
    fn integrated_single_ping_timeout_and_status_are_caller_driven() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let mut transaction = SinglePingTransaction::<4>::new();

        assert_eq!(transaction.status(), SinglePingTransactionStatus::Idle);
        assert_eq!(
            timeout_single_ping_transaction(&mut transaction),
            SinglePingTransactionTimeoutResult::NoTransaction
        );
        assert_eq!(
            retry_single_ping_transaction_arp_request(
                &mut OutboundTransmitDevice::new(),
                &mut transaction,
                &mut [0u8; 128],
            ),
            SinglePingTransactionRetryResult::NoTransaction
        );

        let mut output = [0u8; 128];
        let mut device = OutboundTransmitDevice::new();
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &ArpCache::<0>::new(),
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut output,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            timeout_single_ping_transaction(&mut transaction),
            SinglePingTransactionTimeoutResult::PendingTimedOut {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
            }
        );
        assert_eq!(transaction.status(), SinglePingTransactionStatus::Idle);
        let late_arp = arp_reply_frame();
        let mut late_arp_device = PollDevice::with_frame(&late_arp);
        let mut receive_buffer = [0u8; 128];
        assert_eq!(
            poll_single_ping_transaction(
                &mut late_arp_device,
                &mut ArpCache::<2>::new(),
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::NoTransaction
        );
        assert_eq!(late_arp_device.receive_attempts, 0);

        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update(destination, destination_mac),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut output,
            ),
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 4,
            }
        );
        assert_eq!(
            retry_single_ping_transaction_arp_request(&mut device, &mut transaction, &mut output),
            SinglePingTransactionRetryResult::InflightResult(
                InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                    destination_ipv4: destination,
                }
            )
        );
        assert_eq!(
            timeout_single_ping_transaction(&mut transaction),
            SinglePingTransactionTimeoutResult::InflightTimedOut {
                destination_ipv4: destination,
            }
        );
        assert_eq!(transaction.status(), SinglePingTransactionStatus::Idle);
        let late_reply = icmp_echo_reply_frame();
        let mut late_reply_device = PollDevice::with_frame(&late_reply);
        assert_eq!(
            poll_single_ping_transaction(
                &mut late_reply_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::NoTransaction
        );
        assert_eq!(late_reply_device.receive_attempts, 0);

        assert_eq!(
            start_routed_single_ping_transaction(
                &mut device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut output,
            ),
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 4,
            }
        );
        let reply = icmp_echo_reply_frame();
        let mut reply_device = PollDevice::with_frame(&reply);
        assert_eq!(
            poll_single_ping_transaction(
                &mut reply_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::IcmpEchoReplyMatched { payload_len: 4 }
                )
            )
        );
        assert_eq!(transaction.status(), SinglePingTransactionStatus::Idle);
        assert_eq!(
            timeout_single_ping_transaction(&mut transaction),
            SinglePingTransactionTimeoutResult::NoTransaction
        );
    }

    #[test_case]
    fn qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let expected_icmp_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut cache = ArpCache::<2>::new();
        let mut transaction = SinglePingTransaction::<4>::new();
        let mut output = [0u8; 128];
        let mut receive_buffer = [0u8; 128];
        let mut start_device = OutboundTransmitDevice::new();

        assert_eq!(
            start_routed_single_ping_transaction_with_arp_retry_budget(
                &mut start_device,
                &cache,
                &mut transaction,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut output,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(start_device.transmit_attempts, 1);
        assert_eq!(
            transaction.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            }
        );
        let arp_frame =
            EthernetFrame::parse(&start_device.transmitted[..start_device.transmitted_len])
                .expect("arp request frame");
        assert_eq!(arp_frame.ether_type(), EtherType::Arp);
        let arp_request =
            ArpPacket::parse_ethernet_ipv4(arp_frame.payload()).expect("arp request packet");
        assert_eq!(arp_request.target_protocol_address(), destination);

        let arp_reply = arp_reply_frame();
        let mut arp_reply_device = PollDevice::with_frame(&arp_reply);
        assert_eq!(
            poll_single_ping_transaction(
                &mut arp_reply_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(
                    PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                        frame_len: expected_icmp_len,
                    }
                )
            )
        );
        assert_eq!(transaction.pending(), None);
        assert_eq!(
            transaction.status(),
            SinglePingTransactionStatus::Inflight {
                destination_ipv4: destination,
            }
        );
        assert_eq!(cache.lookup(destination), Some(destination_mac));
        let icmp_frame =
            EthernetFrame::parse(&arp_reply_device.transmitted[..arp_reply_device.transmitted_len])
                .expect("icmp request frame");
        assert_eq!(icmp_frame.destination(), destination_mac);
        assert_eq!(icmp_frame.ether_type(), EtherType::Ipv4);
        let ipv4 = Ipv4Packet::parse(icmp_frame.payload()).expect("ipv4 request");
        assert_eq!(ipv4.destination(), destination);
        assert_eq!(ipv4.protocol(), IPV4_PROTOCOL_ICMP);

        let icmp_reply = icmp_echo_reply_frame();
        let mut icmp_reply_device = PollDevice::with_frame(&icmp_reply);
        assert_eq!(
            poll_single_ping_transaction(
                &mut icmp_reply_device,
                &mut cache,
                &mut transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::IcmpEchoReplyMatched {
                        payload_len: payload.len(),
                    }
                )
            )
        );
        assert_eq!(transaction.status(), SinglePingTransactionStatus::Idle);

        let retry_destination = [198, 51, 100, 7];
        let gateway = [192, 0, 2, 254];
        let gateway_policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some(gateway));
        let mut retry_transaction = SinglePingTransaction::<4>::new();
        let mut retry_device = OutboundTransmitDevice::new();
        assert_eq!(
            start_routed_single_ping_transaction_with_arp_retry_budget(
                &mut retry_device,
                &ArpCache::<0>::new(),
                &mut retry_transaction,
                endpoint,
                gateway_policy,
                retry_destination,
                0x5678,
                9,
                60,
                &payload,
                &mut output,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            retry_single_ping_transaction_arp_request(
                &mut retry_device,
                &mut retry_transaction,
                &mut output,
            ),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                    frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                }
            )
        );
        assert_eq!(retry_device.transmit_attempts, 2);
        assert_eq!(
            retry_single_ping_transaction_arp_request(
                &mut retry_device,
                &mut retry_transaction,
                &mut output,
            ),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRetryBudgetExhausted {
                    destination_ipv4: retry_destination,
                    next_hop_ipv4: gateway,
                }
            )
        );
        assert_eq!(
            timeout_single_ping_transaction(&mut retry_transaction),
            SinglePingTransactionTimeoutResult::PendingTimedOut {
                destination_ipv4: retry_destination,
                next_hop_ipv4: gateway,
            }
        );
        assert_eq!(
            poll_single_ping_transaction(
                &mut PollDevice::with_frame(&arp_reply),
                &mut ArpCache::<2>::new(),
                &mut retry_transaction,
                &mut receive_buffer,
                &mut output,
            ),
            SinglePingTransactionPollResult::NoTransaction
        );
        assert_eq!(
            retry_transaction.status(),
            SinglePingTransactionStatus::Idle
        );
    }

    #[test_case]
    fn single_ping_packet_service_owns_one_transaction_lifecycle() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let expected_icmp_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut service = SinglePingPacketService::<2, 4>::new();
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];
        let mut start_device = OutboundTransmitDevice::new();

        assert_eq!(service.status(), SinglePingTransactionStatus::Idle);
        assert_eq!(
            service.start_ping(
                &mut start_device,
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            service.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            }
        );

        let mut no_frame_device = PollDevice::with_receive_error(DeviceError::WouldBlock);
        assert_eq!(
            service.pump(
                &mut no_frame_device,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(PendingIcmpEchoPollResult::NoFrame)
        );
        assert_eq!(
            service.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            }
        );

        let arp_reply = arp_reply_frame();
        let mut arp_reply_device = PollDevice::with_frame(&arp_reply);
        assert_eq!(
            service.pump(
                &mut arp_reply_device,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(
                    PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                        frame_len: expected_icmp_len,
                    }
                )
            )
        );
        assert_eq!(
            service.arp_cache().lookup(destination),
            Some(destination_mac)
        );
        assert_eq!(
            service.status(),
            SinglePingTransactionStatus::Inflight {
                destination_ipv4: destination,
            }
        );

        let icmp_reply = icmp_echo_reply_frame();
        let mut icmp_reply_device = PollDevice::with_frame(&icmp_reply);
        assert_eq!(
            service.pump(
                &mut icmp_reply_device,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::IcmpEchoReplyMatched {
                        payload_len: payload.len(),
                    }
                )
            )
        );
        assert_eq!(service.status(), SinglePingTransactionStatus::Idle);
    }

    #[test_case]
    fn single_ping_packet_service_preserves_state_across_edge_cases() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        let mut small_payload_service = SinglePingPacketService::<0, 2>::new();
        assert_eq!(
            small_payload_service.start_ping(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::PendingPayloadTooLarge {
                    required_len: payload.len(),
                    max_len: 2,
                }
            )
        );
        assert_eq!(
            small_payload_service.status(),
            SinglePingTransactionStatus::Idle
        );

        let mut transmit_error_service = SinglePingPacketService::<0, 4>::new();
        assert_eq!(
            transmit_error_service.start_ping(
                &mut OutboundTransmitDevice::with_transmit_error(DeviceError::Io),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            SinglePingTransactionStartResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::ArpRequest,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                error: DeviceError::Io,
            })
        );
        assert_eq!(
            transmit_error_service.status(),
            SinglePingTransactionStatus::Idle
        );

        let mut pending_service = SinglePingPacketService::<2, 4>::new();
        assert_eq!(
            pending_service.start_ping(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            SinglePingTransactionStartResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );
        assert_eq!(
            pending_service.start_ping(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                [192, 0, 2, 21],
                0x1234,
                8,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            SinglePingTransactionStartResult::PendingResult(
                PendingIcmpEchoResult::PendingRequestAlreadyQueued {
                    destination_ipv4: destination,
                }
            )
        );

        let arp_reply = arp_reply_frame();
        let mut tiny_receive = [0u8; ETHERNET_HEADER_LEN - 1];
        assert_eq!(
            pending_service.pump(
                &mut PollDevice::with_frame(&arp_reply),
                &mut tiny_receive,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::ReceiveBufferTooSmall
            )
        );
        assert_eq!(
            pending_service.pump(
                &mut PollDevice::with_receive_error(DeviceError::Io),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::ReceiveError(DeviceError::Io)
            )
        );
        assert_eq!(
            pending_service.pump(
                &mut PollDevice::with_frame(ETHERNET_IPV4_FRAME),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::ArpError(
                    PacketError::UnsupportedEtherType,
                ))
            )
        );

        let mut nonmatching_arp = arp_reply_frame();
        nonmatching_arp[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18]
            .copy_from_slice(&[192, 0, 2, 99]);
        assert_eq!(
            pending_service.pump(
                &mut PollDevice::with_frame(&nonmatching_arp),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::NonMatchingArp {
                    pending_destination_ipv4: destination,
                    arp_sender_ipv4: [192, 0, 2, 99],
                })
            )
        );
        assert_eq!(
            pending_service.status(),
            SinglePingTransactionStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            }
        );

        assert_eq!(
            pending_service.pump(
                &mut PollDevice::with_transmit_error(&arp_reply, DeviceError::Io),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::PendingResult(
                PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                    request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
                    frame_len: ETHERNET_HEADER_LEN
                        + IPV4_MIN_HEADER_LEN
                        + ICMP_ECHO_HEADER_LEN
                        + payload.len(),
                    error: DeviceError::Io,
                })
            )
        );
        assert_eq!(
            pending_service.retry_arp(&mut OutboundTransmitDevice::new(), &mut transmit_buffer),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                    frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
                }
            )
        );
        assert_eq!(
            pending_service.retry_arp(&mut OutboundTransmitDevice::new(), &mut transmit_buffer),
            SinglePingTransactionRetryResult::PendingResult(
                PendingIcmpEchoResult::ArpRetryBudgetExhausted {
                    destination_ipv4: destination,
                    next_hop_ipv4: destination,
                }
            )
        );
        assert_eq!(
            pending_service.timeout(),
            SinglePingTransactionTimeoutResult::PendingTimedOut {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
            }
        );
        let mut late_arp_device = PollDevice::with_frame(&arp_reply);
        assert_eq!(
            pending_service.pump(
                &mut late_arp_device,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::NoTransaction
        );
        assert_eq!(late_arp_device.receive_attempts, 0);

        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update(destination, MacAddress::new([0x02, 0, 0, 0, 0, 20])),
            ArpCacheUpdate::Inserted
        );
        let mut inflight_service = SinglePingPacketService::<2, 4>::with_arp_cache(cache);
        assert_eq!(
            inflight_service.start_ping(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            SinglePingTransactionStartResult::IcmpEchoRequestTransmitted {
                frame_len: ETHERNET_HEADER_LEN
                    + IPV4_MIN_HEADER_LEN
                    + ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            }
        );
        assert_eq!(
            inflight_service.start_ping(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                [192, 0, 2, 21],
                0x1234,
                8,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            SinglePingTransactionStartResult::InflightResult(
                InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                    destination_ipv4: destination,
                }
            )
        );
        let mut unsupported_protocol_reply = icmp_echo_reply_frame();
        unsupported_protocol_reply[ETHERNET_HEADER_LEN + 9] = 17;
        rewrite_ipv4_checksum(&mut unsupported_protocol_reply);
        assert_eq!(
            inflight_service.pump(
                &mut PollDevice::with_frame(&unsupported_protocol_reply),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(InflightIcmpEchoResult::ReplyError(
                    PacketError::UnsupportedIpv4Protocol
                ))
            )
        );

        let mut nonmatching_reply = icmp_echo_reply_frame();
        nonmatching_reply[..ETHERNET_ADDR_LEN].copy_from_slice(&[0x02, 0, 0, 0, 0, 98]);
        assert_eq!(
            inflight_service.pump(
                &mut PollDevice::with_frame(&nonmatching_reply),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::InflightResult(
                InflightIcmpEchoPollResult::ObservationResult(
                    InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
                        destination_ipv4: destination,
                    }
                )
            )
        );
        assert_eq!(
            inflight_service.timeout(),
            SinglePingTransactionTimeoutResult::InflightTimedOut {
                destination_ipv4: destination,
            }
        );
        let late_reply = icmp_echo_reply_frame();
        let mut late_reply_device = PollDevice::with_frame(&late_reply);
        assert_eq!(
            inflight_service.pump(
                &mut late_reply_device,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            SinglePingTransactionPollResult::NoTransaction
        );
        assert_eq!(late_reply_device.receive_attempts, 0);
    }

    #[test_case]
    fn userspace_ping_operation_completes_unresolved_arp_to_echo_reply() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let expected_icmp_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let mut operation = UserspacePingOperation::<2, 4>::new();
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        assert_eq!(operation.status(), UserspacePingOperationStatus::Idle);
        assert_eq!(
            operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            operation.status(),
            UserspacePingOperationStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            }
        );
        assert_eq!(
            operation.pump(
                &mut PollDevice::with_receive_error(DeviceError::WouldBlock),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::NoFrame)
        );
        assert_eq!(
            operation.pump(
                &mut PollDevice::with_frame(&arp_reply_frame()),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::AdvancedToInflight {
                frame_len: expected_icmp_len,
            })
        );
        assert_eq!(
            operation.service().arp_cache().lookup(destination),
            Some(destination_mac)
        );
        assert_eq!(
            operation.status(),
            UserspacePingOperationStatus::Inflight {
                destination_ipv4: destination,
            }
        );
        assert_eq!(
            operation.pump(
                &mut PollDevice::with_frame(&icmp_echo_reply_frame()),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::Completed {
                payload_len: payload.len(),
            })
        );
        assert_eq!(
            operation.status(),
            UserspacePingOperationStatus::Completed {
                destination_ipv4: destination,
                payload_len: payload.len(),
            }
        );
    }

    #[test_case]
    fn userspace_ping_operation_maps_busy_retry_timeout_and_io_errors() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut operation = UserspacePingOperation::<2, 4>::new();
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        assert_eq!(
            operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                [192, 0, 2, 21],
                0x1234,
                8,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Err(crate::posix::PosixError::Busy)
        );
        assert_eq!(
            operation.retry_arp(&mut OutboundTransmitDevice::new(), &mut transmit_buffer),
            Ok(UserspacePingOperationStep::RetryTransmitted {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            operation.retry_arp(&mut OutboundTransmitDevice::new(), &mut transmit_buffer),
            Err(crate::posix::PosixError::Again)
        );
        assert_eq!(
            operation.timeout(),
            Ok(UserspacePingOperationStep::TimedOut {
                destination_ipv4: destination,
            })
        );
        assert_eq!(
            operation.status(),
            UserspacePingOperationStatus::TimedOut {
                destination_ipv4: destination,
            }
        );

        let mut transmit_error_operation = UserspacePingOperation::<0, 4>::new();
        assert_eq!(
            transmit_error_operation.start(
                &mut OutboundTransmitDevice::with_transmit_error(DeviceError::Io),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Err(crate::posix::PosixError::Io)
        );
        assert_eq!(
            transmit_error_operation.status(),
            UserspacePingOperationStatus::Idle
        );

        let mut receive_error_operation = UserspacePingOperation::<2, 4>::new();
        assert_eq!(
            receive_error_operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            receive_error_operation.pump(
                &mut PollDevice::with_receive_error(DeviceError::Io),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Err(crate::posix::PosixError::Io)
        );

        let mut cache = ArpCache::<2>::new();
        assert_eq!(
            cache.insert_or_update(destination, MacAddress::new([0x02, 0, 0, 0, 0, 20])),
            ArpCacheUpdate::Inserted
        );
        let service = SinglePingPacketService::<2, 4>::with_arp_cache(cache);
        let mut inflight_operation = UserspacePingOperation::with_service(service);
        assert_eq!(
            inflight_operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Ok(UserspacePingOperationStep::StartedInflight {
                frame_len: ETHERNET_HEADER_LEN
                    + IPV4_MIN_HEADER_LEN
                    + ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            })
        );
        assert_eq!(
            inflight_operation.start(
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                [192, 0, 2, 21],
                0x1234,
                8,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Err(crate::posix::PosixError::Busy)
        );
        assert_eq!(
            inflight_operation.pump(
                &mut PollDevice::with_receive_error(DeviceError::Io),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Err(crate::posix::PosixError::Io)
        );
    }

    #[test_case]
    fn network_ping_descriptor_lifecycle_completes_unresolved_arp_to_echo_reply() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let destination_mac = MacAddress::new([0x02, 0, 0, 0, 0, 20]);
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut table = NetworkPingOperationDescriptorTable::<1, 2, 4>::new();
        let descriptor = table.open().expect("open descriptor");
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        assert_eq!(descriptor.raw(), 0);
        assert_eq!(
            table.status(descriptor),
            Ok(UserspacePingOperationStatus::Idle)
        );
        assert_eq!(
            table.start(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            table.status(descriptor),
            Ok(UserspacePingOperationStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            })
        );
        assert_eq!(
            table.pump(
                descriptor,
                &mut PollDevice::with_frame(&arp_reply_frame()),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::AdvancedToInflight {
                frame_len: ETHERNET_HEADER_LEN
                    + IPV4_MIN_HEADER_LEN
                    + ICMP_ECHO_HEADER_LEN
                    + payload.len(),
            })
        );
        assert_eq!(
            table.status(descriptor),
            Ok(UserspacePingOperationStatus::Inflight {
                destination_ipv4: destination,
            })
        );
        assert_eq!(
            table.pump(
                descriptor,
                &mut PollDevice::with_frame(&icmp_echo_reply_frame()),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::Completed {
                payload_len: payload.len(),
            })
        );
        assert_eq!(
            table.status(descriptor),
            Ok(UserspacePingOperationStatus::Completed {
                destination_ipv4: destination,
                payload_len: payload.len(),
            })
        );
        let mut seeded_cache = ArpCache::<2>::new();
        assert_eq!(
            seeded_cache.insert_or_update(destination, destination_mac),
            ArpCacheUpdate::Inserted
        );
        assert_eq!(table.close(descriptor), Ok(()));
        assert_eq!(
            table.status(descriptor),
            Err(crate::posix::PosixError::BadDescriptor)
        );
        assert_eq!(
            table.open_with_service(SinglePingPacketService::<2, 4>::with_arp_cache(
                seeded_cache,
            )),
            Ok(descriptor)
        );
    }

    #[test_case]
    fn network_ping_descriptor_maps_invalid_closed_capacity_and_busy_edges() {
        let mut empty_table = NetworkPingOperationDescriptorTable::<0, 2, 4>::new();
        assert_eq!(
            empty_table.open(),
            Err(crate::posix::PosixError::TooManyOpenFiles)
        );
        assert_eq!(
            empty_table.status(NetworkPingOperationDescriptor::from_raw(0)),
            Err(crate::posix::PosixError::BadDescriptor)
        );

        let mut table = NetworkPingOperationDescriptorTable::<1, 2, 4>::new();
        let descriptor = table.open().expect("open descriptor");
        assert_eq!(table.open(), Err(crate::posix::PosixError::Busy));
        assert_eq!(
            table.status(NetworkPingOperationDescriptor::from_raw(7)),
            Err(crate::posix::PosixError::BadDescriptor)
        );
        assert_eq!(table.close(descriptor), Ok(()));
        assert_eq!(
            table.close(descriptor),
            Err(crate::posix::PosixError::BadDescriptor)
        );
        assert_eq!(
            table.start(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                local_endpoint(),
                Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None),
                [192, 0, 2, 20],
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut [0u8; 128],
                0,
            ),
            Err(crate::posix::PosixError::BadDescriptor)
        );
    }

    #[test_case]
    fn network_ping_descriptor_maps_retry_timeout_and_io_errors() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let payload = [1, 2, 3, 4];
        let mut table = NetworkPingOperationDescriptorTable::<1, 2, 4>::new();
        let descriptor = table.open().expect("open descriptor");
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        assert_eq!(
            table.start(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            table.retry_arp(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                &mut transmit_buffer,
            ),
            Err(crate::posix::PosixError::Again)
        );
        assert_eq!(
            table.timeout(descriptor),
            Ok(UserspacePingOperationStep::TimedOut {
                destination_ipv4: destination,
            })
        );
        assert_eq!(
            table.status(descriptor),
            Ok(UserspacePingOperationStatus::TimedOut {
                destination_ipv4: destination,
            })
        );
        assert_eq!(table.close(descriptor), Ok(()));

        let transmit_error_descriptor = table.open().expect("reopen descriptor");
        assert_eq!(
            table.start(
                transmit_error_descriptor,
                &mut OutboundTransmitDevice::with_transmit_error(DeviceError::Io),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                0,
            ),
            Err(crate::posix::PosixError::Io)
        );
        assert_eq!(
            table.status(transmit_error_descriptor),
            Ok(UserspacePingOperationStatus::Idle)
        );
        assert_eq!(table.close(transmit_error_descriptor), Ok(()));

        let receive_error_descriptor = table.open().expect("reopen descriptor");
        assert_eq!(
            table.start(
                receive_error_descriptor,
                &mut OutboundTransmitDevice::new(),
                endpoint,
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            table.pump(
                receive_error_descriptor,
                &mut PollDevice::with_receive_error(DeviceError::Io),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            Err(crate::posix::PosixError::Io)
        );
    }

    #[test_case]
    fn pending_arp_reply_poll_advances_gateway_pending_to_single_icmp_transmit() {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<8>::new();
        let mut queue_output = [0u8; 64];
        let mut queue_device = OutboundTransmitDevice::new();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], Some([192, 0, 2, 254]));
        let payload = [0x11, 0x22, 0x33];

        assert_eq!(
            transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request(
                &mut queue_device,
                &cache,
                &mut pending,
                local_endpoint(),
                policy,
                [198, 51, 100, 7],
                0x1234,
                7,
                61,
                &payload,
                &mut queue_output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let gateway_mac = MacAddress::new([0x02, 0, 0, 0, 0, 254]);
        let mut gateway_reply = arp_reply_frame();
        gateway_reply[6..12].copy_from_slice(&gateway_mac.bytes());
        gateway_reply[ETHERNET_HEADER_LEN + 8..ETHERNET_HEADER_LEN + 14]
            .copy_from_slice(&gateway_mac.bytes());
        gateway_reply[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18]
            .copy_from_slice(&[192, 0, 2, 254]);
        let mut poll_device = PollDevice::with_frame(&gateway_reply);
        let mut learned_cache = ArpCache::<4>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 128];
        let expected_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();

        let result = poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
            &mut poll_device,
            &mut learned_cache,
            &mut pending,
            &mut receive_buffer,
            &mut transmit_buffer,
        );

        assert_eq!(
            result,
            PendingIcmpEchoPollResult::PendingResult(
                PendingIcmpEchoResult::IcmpEchoRequestTransmitted {
                    frame_len: expected_len,
                }
            )
        );
        assert_eq!(poll_device.receive_attempts, 1);
        assert_eq!(poll_device.transmitted_len, expected_len);
        assert_eq!(pending.pending(), None);
        assert_eq!(learned_cache.lookup([192, 0, 2, 254]), Some(gateway_mac));
        assert_eq!(learned_cache.lookup([198, 51, 100, 7]), None);
        let frame = EthernetFrame::parse(&poll_device.transmitted[..poll_device.transmitted_len])
            .expect("icmp");
        assert_eq!(frame.destination(), gateway_mac);
        let ipv4 = Ipv4Packet::parse(frame.payload()).expect("ipv4");
        assert_eq!(ipv4.destination(), [198, 51, 100, 7]);
        assert_eq!(&ipv4.payload()[ICMP_ECHO_HEADER_LEN..], &payload);
    }

    #[test_case]
    fn pending_arp_reply_poll_distinguishes_no_pending_no_frame_and_receive_errors() {
        let mut cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 128];
        let reply = arp_reply_frame();
        let mut no_pending = PollDevice::with_frame(&reply);

        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut no_pending,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::NoPendingRequest
        );
        assert_eq!(no_pending.receive_attempts, 0);
        assert_eq!(no_pending.transmitted_len, 0);

        let mut queue_device = OutboundTransmitDevice::new();
        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut queue_device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 20],
                1,
                2,
                64,
                &[1, 2],
                &mut transmit_buffer,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let mut no_frame = PollDevice::with_receive_error(DeviceError::WouldBlock);
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut no_frame,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::NoFrame
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut receive_pressure = PollDevice::with_frame(&reply);
        let mut small_receive = [0u8; ETHERNET_HEADER_LEN - 1];
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut receive_pressure,
                &mut cache,
                &mut pending,
                &mut small_receive,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::ReceiveBufferTooSmall
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut receive_error = PollDevice::with_receive_error(DeviceError::Io);
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut receive_error,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::ReceiveError(DeviceError::Io)
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 20]));
    }

    #[test_case]
    fn pending_arp_reply_poll_preserves_pending_on_nonmatch_malformed_pressure_and_transmit_error()
    {
        let cache = ArpCache::<0>::new();
        let mut pending = SinglePendingIcmpEcho::<4>::new();
        let mut queue_output = [0u8; 64];
        let mut queue_device = OutboundTransmitDevice::new();
        assert_eq!(
            transmit_or_queue_single_pending_ipv4_icmp_echo_request(
                &mut queue_device,
                &cache,
                &mut pending,
                local_endpoint(),
                [192, 0, 2, 44],
                1,
                2,
                64,
                &[1, 2],
                &mut queue_output,
            ),
            PendingIcmpEchoResult::ArpRequestTransmittedAndPending {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            }
        );

        let nonmatching_reply = arp_reply_frame();
        let mut poll_device = PollDevice::with_frame(&nonmatching_reply);
        let mut cache = ArpCache::<2>::new();
        let mut receive_buffer = [0u8; 64];
        let mut transmit_buffer = [0u8; 128];
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut poll_device,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::NonMatchingArp {
                pending_destination_ipv4: [192, 0, 2, 44],
                arp_sender_ipv4: [192, 0, 2, 20],
            })
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 44]));
        assert_eq!(poll_device.transmitted_len, 0);

        let truncated = &arp_reply_frame()[..ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN - 1];
        let mut malformed = PollDevice::with_frame(truncated);
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut malformed,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::ArpError(
                PacketError::Truncated
            ))
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 44]));

        let matching_reply = {
            let mut frame = arp_reply_frame();
            frame[ETHERNET_HEADER_LEN + 14..ETHERNET_HEADER_LEN + 18]
                .copy_from_slice(&[192, 0, 2, 44]);
            frame
        };
        let mut output_pressure = PollDevice::with_frame(&matching_reply);
        let mut small_transmit = [0xaa; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut output_pressure,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut small_transmit,
            ),
            PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::RequestError(
                OutboundFrameError::OutputBufferTooSmall {
                    required_len: ETHERNET_HEADER_LEN
                        + IPV4_MIN_HEADER_LEN
                        + ICMP_ECHO_HEADER_LEN
                        + 2,
                    available_len: small_transmit.len(),
                }
            ))
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 44]));

        let mut transmit_error = PollDevice::with_transmit_error(&matching_reply, DeviceError::Io);
        assert_eq!(
            poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request(
                &mut transmit_error,
                &mut cache,
                &mut pending,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            PendingIcmpEchoPollResult::PendingResult(PendingIcmpEchoResult::TransmitError {
                request_kind: OutboundRequestKind::Ipv4IcmpEchoRequest,
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 2,
                error: DeviceError::Io,
            })
        );
        assert_eq!(pending.pending_destination_ipv4(), Some([192, 0, 2, 44]));
        assert_eq!(
            cache.lookup([192, 0, 2, 44]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 20]))
        );
    }

    #[test_case]
    fn single_inflight_icmp_echo_reply_poll_matches_and_clears_request() {
        let mut inflight = SingleInflightIcmpEcho::<8>::new();
        let payload = [1, 2, 3, 4];
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &payload,
            ),
            InflightIcmpEchoResult::InflightRequestTracked
        );

        let reply = icmp_echo_reply_frame();
        let mut device = PollDevice::with_frame(&reply);
        let mut receive_buffer = [0u8; 128];

        assert_eq!(
            poll_single_inflight_ipv4_icmp_echo_reply(
                &mut device,
                &mut inflight,
                &mut receive_buffer,
            ),
            InflightIcmpEchoPollResult::ObservationResult(
                InflightIcmpEchoResult::IcmpEchoReplyMatched {
                    payload_len: payload.len(),
                }
            )
        );
        assert_eq!(device.receive_attempts, 1);
        assert_eq!(device.transmitted_len, 0);
        assert_eq!(inflight.inflight(), None);
    }

    #[test_case]
    fn single_inflight_icmp_echo_reply_observation_rejects_nonmatching_reply_fields() {
        let payload = [1, 2, 3, 4];
        let expected_nonmatch = InflightIcmpEchoResult::NonMatchingIcmpEchoReply {
            destination_ipv4: [192, 0, 2, 20],
        };

        let mut source_mismatch = icmp_echo_reply_frame();
        source_mismatch[ETHERNET_HEADER_LEN + 12..ETHERNET_HEADER_LEN + 16]
            .copy_from_slice(&[192, 0, 2, 21]);
        rewrite_ipv4_checksum(&mut source_mismatch);
        assert_inflight_observation_preserves_request(source_mismatch, expected_nonmatch);

        let mut destination_mismatch = icmp_echo_reply_frame();
        destination_mismatch[ETHERNET_HEADER_LEN + 16..ETHERNET_HEADER_LEN + 20]
            .copy_from_slice(&[192, 0, 2, 99]);
        rewrite_ipv4_checksum(&mut destination_mismatch);
        assert_inflight_observation_preserves_request(destination_mismatch, expected_nonmatch);

        let mut identifier_mismatch = icmp_echo_reply_frame();
        identifier_mismatch[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 4] = 0xab;
        rewrite_icmp_checksum(&mut identifier_mismatch);
        assert_inflight_observation_preserves_request(identifier_mismatch, expected_nonmatch);

        let mut sequence_mismatch = icmp_echo_reply_frame();
        sequence_mismatch[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 7] = 0x08;
        rewrite_icmp_checksum(&mut sequence_mismatch);
        assert_inflight_observation_preserves_request(sequence_mismatch, expected_nonmatch);

        let mut payload_mismatch = icmp_echo_reply_frame();
        payload_mismatch[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + 3] =
            0x99;
        rewrite_icmp_checksum(&mut payload_mismatch);
        assert_inflight_observation_preserves_request(payload_mismatch, expected_nonmatch);

        let mut inflight = SingleInflightIcmpEcho::<4>::new();
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &payload,
            ),
            InflightIcmpEchoResult::InflightRequestTracked
        );
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 44],
                0x1234,
                7,
                &payload,
            ),
            InflightIcmpEchoResult::InflightRequestAlreadyTracked {
                destination_ipv4: [192, 0, 2, 20],
            }
        );
    }

    #[test_case]
    fn single_inflight_icmp_echo_reply_poll_distinguishes_empty_and_receive_errors() {
        let mut inflight = SingleInflightIcmpEcho::<4>::new();
        let mut receive_buffer = [0u8; 128];
        let reply = icmp_echo_reply_frame();
        let mut no_inflight = PollDevice::with_frame(&reply);

        assert_eq!(
            poll_single_inflight_ipv4_icmp_echo_reply(
                &mut no_inflight,
                &mut inflight,
                &mut receive_buffer,
            ),
            InflightIcmpEchoPollResult::NoInflightRequest
        );
        assert_eq!(no_inflight.receive_attempts, 0);

        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &[1, 2, 3, 4],
            ),
            InflightIcmpEchoResult::InflightRequestTracked
        );

        let mut no_frame = PollDevice::with_receive_error(DeviceError::WouldBlock);
        assert_eq!(
            poll_single_inflight_ipv4_icmp_echo_reply(
                &mut no_frame,
                &mut inflight,
                &mut receive_buffer,
            ),
            InflightIcmpEchoPollResult::NoFrame
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut small_rx = [0u8; ETHERNET_HEADER_LEN - 1];
        let mut receive_pressure = PollDevice::with_frame(&reply);
        assert_eq!(
            poll_single_inflight_ipv4_icmp_echo_reply(
                &mut receive_pressure,
                &mut inflight,
                &mut small_rx,
            ),
            InflightIcmpEchoPollResult::ReceiveBufferTooSmall
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut receive_error = PollDevice::with_receive_error(DeviceError::Io);
        assert_eq!(
            poll_single_inflight_ipv4_icmp_echo_reply(
                &mut receive_error,
                &mut inflight,
                &mut receive_buffer,
            ),
            InflightIcmpEchoPollResult::ReceiveError(DeviceError::Io)
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));
    }

    #[test_case]
    fn single_inflight_icmp_echo_reply_observation_reports_malformed_and_payload_pressure() {
        let mut inflight = SingleInflightIcmpEcho::<2>::new();
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &[1, 2, 3],
            ),
            InflightIcmpEchoResult::InflightPayloadTooLarge {
                required_len: 3,
                max_len: 2,
            }
        );
        assert_eq!(inflight.inflight(), None);

        let mut inflight = SingleInflightIcmpEcho::<4>::new();
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &[1, 2, 3, 4],
            ),
            InflightIcmpEchoResult::InflightRequestTracked
        );

        assert_eq!(
            observe_single_inflight_ipv4_icmp_echo_reply(
                &mut inflight,
                &icmp_echo_reply_frame()[..ETHERNET_HEADER_LEN - 1],
            ),
            InflightIcmpEchoResult::ReplyError(PacketError::Truncated)
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));

        let mut unsupported = icmp_echo_reply_frame();
        write_be_u16(&mut unsupported, 12, 0x86dd);
        assert_eq!(
            observe_single_inflight_ipv4_icmp_echo_reply(&mut inflight, &unsupported),
            InflightIcmpEchoResult::ReplyError(PacketError::UnsupportedEtherType)
        );

        let mut bad_icmp_checksum = icmp_echo_reply_frame();
        bad_icmp_checksum[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 2] ^= 0xff;
        assert_eq!(
            observe_single_inflight_ipv4_icmp_echo_reply(&mut inflight, &bad_icmp_checksum),
            InflightIcmpEchoResult::ReplyError(PacketError::InvalidIcmpChecksum)
        );

        let mut echo_request_not_reply = icmp_echo_reply_frame();
        echo_request_not_reply[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN] = 8;
        rewrite_icmp_checksum(&mut echo_request_not_reply);
        assert_eq!(
            observe_single_inflight_ipv4_icmp_echo_reply(&mut inflight, &echo_request_not_reply),
            InflightIcmpEchoResult::ReplyError(PacketError::InvalidIcmpEcho)
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));
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

    #[test_case]
    fn network_runtime_device_pump_reports_no_frame_receive_pressure_and_receive_error() {
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_receive_error(DeviceError::WouldBlock),
                None,
                &mut [0u8; 128],
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::NoFrame
        );

        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_frame(&arp_request_frame()),
                None,
                &mut [0u8; ETHERNET_HEADER_LEN - 1],
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::ReceiveBufferTooSmall
        );

        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_receive_error(DeviceError::Io),
                None,
                &mut [0u8; 128],
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::ReceiveError(DeviceError::Io)
        );
    }

    #[test_case]
    fn network_runtime_device_pump_reports_nonlocal_no_reply_without_active_operation() {
        let mut seeded_cache = ArpCache::<2>::new();
        assert_eq!(
            seeded_cache.insert_or_update([192, 0, 2, 55], MacAddress::new([0x02, 0, 0, 0, 0, 55])),
            ArpCacheUpdate::Inserted
        );
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::with_local_arp_cache(
            local_endpoint(),
            seeded_cache,
        );
        let mut nonlocal = icmp_echo_request_frame();
        nonlocal[0..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 88]);
        let mut device = PollDevice::with_frame(&nonlocal);
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.pump(&mut device, None, &mut receive_buffer, &mut transmit_buffer),
            NetworkRuntimeDevicePumpStepResult::LocalNoReply
        );
        assert_eq!(device.transmitted_len, 0);
        assert_eq!(
            runtime.local_arp_cache().lookup([192, 0, 2, 55]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 55]))
        );
    }

    #[test_case]
    fn network_runtime_device_pump_transmits_local_arp_reply_from_caller_buffers() {
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let request = arp_request_frame();
        let mut device = PollDevice::with_frame(&request);
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.pump(&mut device, None, &mut receive_buffer, &mut transmit_buffer),
            NetworkRuntimeDevicePumpStepResult::LocalReply(PacketDispatchResult {
                reply_kind: PacketReplyKind::Arp,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            device.transmitted_len,
            ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN
        );
        let frame =
            EthernetFrame::parse(&device.transmitted[..device.transmitted_len]).expect("arp reply");
        assert_eq!(frame.destination(), MacAddress::new([0x02, 0, 0, 0, 0, 1]));
        assert_eq!(frame.source(), local_endpoint().mac());
        assert_eq!(frame.ether_type(), EtherType::Arp);
        assert_eq!(
            runtime.local_arp_cache().lookup([192, 0, 2, 10]),
            Some(MacAddress::new([0x02, 0, 0, 0, 0, 1]))
        );
    }

    #[test_case]
    fn network_runtime_device_pump_transmits_local_icmp_echo_reply_from_caller_buffers() {
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let request = icmp_echo_request_frame();
        let mut device = PollDevice::with_frame(&request);
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.pump(&mut device, None, &mut receive_buffer, &mut transmit_buffer),
            NetworkRuntimeDevicePumpStepResult::LocalReply(PacketDispatchResult {
                reply_kind: PacketReplyKind::IcmpEcho,
                frame_len: ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12,
            })
        );
        assert_eq!(
            device.transmitted_len,
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12
        );
        let frame = EthernetFrame::parse(&device.transmitted[..device.transmitted_len])
            .expect("icmp reply");
        assert_eq!(frame.destination(), MacAddress::new([0x02, 0, 0, 0, 0, 1]));
        assert_eq!(frame.source(), local_endpoint().mac());
        assert_eq!(frame.ether_type(), EtherType::Ipv4);
        assert_eq!(frame.payload()[IPV4_MIN_HEADER_LEN], 0);
    }

    #[test_case]
    fn network_runtime_device_pump_advances_active_ping_from_arp_to_completed_status() {
        let endpoint = local_endpoint();
        let destination = [192, 0, 2, 20];
        let payload = [1, 2, 3, 4];
        let expected_icmp_len =
            ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + ICMP_ECHO_HEADER_LEN + payload.len();
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(endpoint);
        let descriptor = runtime.open_ping_operation().expect("open ping op");
        let mut transmit_buffer = [0u8; 128];
        let mut receive_buffer = [0u8; 128];

        assert_eq!(
            runtime.start_ping(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &payload,
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            runtime.ping_operations().status(descriptor),
            Ok(UserspacePingOperationStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            })
        );

        let arp_reply = arp_reply_frame();
        let mut arp_device = PollDevice::with_frame(&arp_reply);
        assert_eq!(
            runtime.pump(
                &mut arp_device,
                Some(descriptor),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::ActivePingStep {
                descriptor,
                step: UserspacePingOperationStep::AdvancedToInflight {
                    frame_len: expected_icmp_len,
                },
            }
        );
        assert_eq!(arp_device.receive_attempts, 1);
        assert_eq!(arp_device.transmitted_len, expected_icmp_len);
        assert_eq!(
            runtime.ping_status(descriptor),
            Ok(UserspacePingOperationStatus::Inflight {
                destination_ipv4: destination,
            })
        );

        let icmp_reply = icmp_echo_reply_frame();
        let mut reply_device = PollDevice::with_frame(&icmp_reply);
        assert_eq!(
            runtime.pump(
                &mut reply_device,
                Some(descriptor),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::ActivePingStep {
                descriptor,
                step: UserspacePingOperationStep::Completed {
                    payload_len: payload.len(),
                },
            }
        );
        assert_eq!(
            runtime.ping_status(descriptor),
            Ok(UserspacePingOperationStatus::Completed {
                destination_ipv4: destination,
                payload_len: payload.len(),
            })
        );
        assert_eq!(runtime.close_ping_operation(descriptor), Ok(()));
    }

    #[test_case]
    fn network_runtime_device_pump_preserves_retry_timeout_and_terminal_observation() {
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let descriptor = runtime.open_ping_operation().expect("open ping op");
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.start_ping(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            runtime.retry_ping_arp(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                &mut transmit_buffer,
            ),
            Ok(UserspacePingOperationStep::RetryTransmitted {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            runtime.retry_ping_arp(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                &mut transmit_buffer,
            ),
            Err(crate::posix::PosixError::Again)
        );
        assert_eq!(
            runtime.timeout_ping(descriptor),
            Ok(UserspacePingOperationStep::TimedOut {
                destination_ipv4: destination,
            })
        );
        assert_eq!(
            runtime.ping_status(descriptor),
            Ok(UserspacePingOperationStatus::TimedOut {
                destination_ipv4: destination,
            })
        );
    }

    #[test_case]
    fn network_runtime_device_pump_reports_local_and_active_transmit_errors() {
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_transmit_error(&arp_request_frame(), DeviceError::Io),
                None,
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::LocalTransmitError(DeviceError::Io)
        );

        let descriptor = runtime.open_ping_operation().expect("open ping op");
        assert_eq!(
            runtime.start_ping(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None),
                [192, 0, 2, 20],
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut transmit_buffer,
                0,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_transmit_error(&arp_reply_frame(), DeviceError::Io),
                Some(descriptor),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::ActivePingError {
                descriptor,
                error: crate::posix::PosixError::Io,
            }
        );
    }

    #[test_case]
    fn network_runtime_device_pump_prioritizes_local_reply_over_active_operation() {
        let destination = [192, 0, 2, 20];
        let policy = Ipv4EgressRoutePolicy::new([255, 255, 255, 0], None);
        let mut runtime = NetworkRuntimeDevicePump::<2, 1, 2, 4>::new(local_endpoint());
        let descriptor = runtime.open_ping_operation().expect("open ping op");
        let mut receive_buffer = [0u8; 128];
        let mut transmit_buffer = [0u8; 128];

        assert_eq!(
            runtime.start_ping(
                descriptor,
                &mut OutboundTransmitDevice::new(),
                policy,
                destination,
                0x1234,
                7,
                61,
                &[1, 2, 3, 4],
                &mut transmit_buffer,
                1,
            ),
            Ok(UserspacePingOperationStep::StartedPendingArp {
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );

        assert_eq!(
            runtime.pump(
                &mut PollDevice::with_frame(&arp_request_frame()),
                Some(descriptor),
                &mut receive_buffer,
                &mut transmit_buffer,
            ),
            NetworkRuntimeDevicePumpStepResult::LocalReply(PacketDispatchResult {
                reply_kind: PacketReplyKind::Arp,
                frame_len: ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN,
            })
        );
        assert_eq!(
            runtime.ping_status(descriptor),
            Ok(UserspacePingOperationStatus::PendingArp {
                destination_ipv4: destination,
                next_hop_ipv4: destination,
                arp_retries_remaining: 1,
            })
        );
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

    fn icmp_echo_reply_frame() -> [u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12] {
        let mut frame = [0u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12];
        frame[..6].copy_from_slice(&[0x02, 0, 0, 0, 0, 99]);
        frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 20]);
        write_be_u16(&mut frame, 12, ETHERTYPE_IPV4);

        let ipv4 = &mut frame[ETHERNET_HEADER_LEN..ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN];
        ipv4[0] = 0x45;
        write_be_u16(ipv4, 2, (IPV4_MIN_HEADER_LEN + 12) as u16);
        write_be_u16(ipv4, 4, 0x4444);
        ipv4[8] = 64;
        ipv4[9] = IPV4_PROTOCOL_ICMP;
        ipv4[12..16].copy_from_slice(&[192, 0, 2, 20]);
        ipv4[16..20].copy_from_slice(&[192, 0, 2, 1]);
        let checksum = internet_checksum(ipv4);
        write_be_u16(ipv4, 10, checksum);

        let icmp = &mut frame[ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN..];
        icmp[0] = 0;
        icmp[4..].copy_from_slice(&[0x12, 0x34, 0, 7, 1, 2, 3, 4]);
        let checksum = internet_checksum(icmp);
        write_be_u16(icmp, 2, checksum);
        frame
    }

    fn assert_inflight_observation_preserves_request(
        frame: [u8; ETHERNET_HEADER_LEN + IPV4_MIN_HEADER_LEN + 12],
        expected: InflightIcmpEchoResult,
    ) {
        let mut inflight = SingleInflightIcmpEcho::<4>::new();
        assert_eq!(
            record_single_inflight_ipv4_icmp_echo_request(
                &mut inflight,
                local_endpoint(),
                [192, 0, 2, 20],
                0x1234,
                7,
                &[1, 2, 3, 4],
            ),
            InflightIcmpEchoResult::InflightRequestTracked
        );

        assert_eq!(
            observe_single_inflight_ipv4_icmp_echo_reply(&mut inflight, &frame),
            expected
        );
        assert_eq!(inflight.inflight_destination_ipv4(), Some([192, 0, 2, 20]));
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

    fn rewrite_icmp_checksum(frame: &mut [u8]) {
        let header_len = ((frame[ETHERNET_HEADER_LEN] & 0x0f) as usize) * 4;
        let total_len = read_be_u16(frame, ETHERNET_HEADER_LEN + 2) as usize;
        let icmp_start = ETHERNET_HEADER_LEN + header_len;
        let icmp_end = ETHERNET_HEADER_LEN + total_len;
        let icmp = &mut frame[icmp_start..icmp_end];
        write_be_u16(icmp, 2, 0);
        let checksum = internet_checksum(icmp);
        write_be_u16(icmp, 2, checksum);
    }
}
