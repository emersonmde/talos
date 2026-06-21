//! Private Talos userspace socket ABI helpers.
//!
//! This module mirrors the accepted descriptor-backed socket syscall contract
//! for no_std callers. It is private experimental Talos vocabulary, not a
//! Linux syscall-number or libc compatibility surface.

pub(crate) const SVC_IMMEDIATE: u16 = crate::syscall::STABLE_SVC_IMMEDIATE;

pub(crate) const CLOSE: u64 = crate::syscall::TALOS_CLOSE_SYSCALL;
pub(crate) const SOCKET: u64 = crate::syscall::TALOS_SOCKET_SYSCALL;
pub(crate) const BIND: u64 = crate::syscall::TALOS_BIND_SYSCALL;
pub(crate) const LISTEN: u64 = crate::syscall::TALOS_LISTEN_SYSCALL;
pub(crate) const CONNECT: u64 = crate::syscall::TALOS_CONNECT_SYSCALL;
pub(crate) const ACCEPT: u64 = crate::syscall::TALOS_ACCEPT_SYSCALL;
pub(crate) const SEND: u64 = crate::syscall::TALOS_SEND_SYSCALL;
pub(crate) const RECV: u64 = crate::syscall::TALOS_RECV_SYSCALL;
pub(crate) const POLL: u64 = crate::syscall::TALOS_POLL_SYSCALL;
pub(crate) const POLL_WAIT: u64 = crate::syscall::TALOS_POLL_WAIT_SYSCALL;

pub(crate) const AF_INET: u64 = crate::network::SOCKET_DOMAIN_AF_INET;
pub(crate) const SOCK_STREAM: u64 = crate::network::SOCKET_TYPE_STREAM;
pub(crate) const PROTOCOL_DEFAULT: u64 = crate::network::SOCKET_PROTOCOL_DEFAULT;

pub(crate) const POLL_ENTRY_SIZE: usize = crate::syscall::TALOS_POLL_ENTRY_SIZE;
pub(crate) const POLL_MAX_ENTRIES: usize = crate::syscall::TALOS_POLL_MAX_ENTRIES;
pub(crate) const POLL_WAIT_MAX_TICKS: u64 = crate::syscall::TALOS_POLL_WAIT_MAX_TICKS;
pub(crate) const POLL_READ: u32 = crate::syscall::TALOS_POLL_READ;
pub(crate) const POLL_WRITE: u32 = crate::syscall::TALOS_POLL_WRITE;
pub(crate) const POLL_HANGUP: u32 = crate::syscall::TALOS_POLL_HANGUP;
pub(crate) const POLL_ERROR: u32 = crate::syscall::TALOS_POLL_ERROR;

pub(crate) const ERRNO_EAGAIN: u16 = crate::syscall::EAGAIN;
pub(crate) const ERRNO_EBADF: u16 = crate::syscall::EBADF;
pub(crate) const ERRNO_EFAULT: u16 = crate::syscall::EFAULT;
pub(crate) const ERRNO_EINVAL: u16 = crate::syscall::EINVAL;
pub(crate) const ERRNO_ENOSPC: u16 = crate::syscall::ENOSPC;
pub(crate) const ERRNO_ENOTSUP: u16 = crate::syscall::ENOTSUP;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SocketAbiCall {
    number: u64,
    arguments: [u64; crate::syscall::MAX_SCALAR_ARGUMENTS],
}

impl SocketAbiCall {
    pub(crate) const fn new(
        number: u64,
        arguments: [u64; crate::syscall::MAX_SCALAR_ARGUMENTS],
    ) -> Self {
        Self { number, arguments }
    }

    pub(crate) const fn number(self) -> u64 {
        self.number
    }

    pub(crate) const fn arguments(self) -> [u64; crate::syscall::MAX_SCALAR_ARGUMENTS] {
        self.arguments
    }

    pub(crate) const fn syscall_arguments(self) -> crate::syscall::SyscallArguments {
        crate::syscall::SyscallArguments::new(self.arguments)
    }

    #[cfg(target_arch = "aarch64")]
    #[allow(dead_code)]
    pub(crate) unsafe fn invoke(self) -> u64 {
        unsafe { syscall6(self.number, self.arguments) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PollEntry {
    fd: u64,
    events: u32,
    revents: u32,
}

impl PollEntry {
    pub(crate) const fn new(fd: u64, events: u32) -> Self {
        Self {
            fd,
            events,
            revents: 0,
        }
    }

    pub(crate) const fn with_revents(fd: u64, events: u32, revents: u32) -> Self {
        Self {
            fd,
            events,
            revents,
        }
    }

    pub(crate) const fn fd(self) -> u64 {
        self.fd
    }

    pub(crate) const fn events(self) -> u32 {
        self.events
    }

    pub(crate) const fn revents(self) -> u32 {
        self.revents
    }

    pub(crate) fn encode(self, dst: &mut [u8]) -> Result<(), PollEntryEncodeError> {
        if dst.len() < POLL_ENTRY_SIZE {
            return Err(PollEntryEncodeError::BufferTooSmall);
        }
        dst[..8].copy_from_slice(&self.fd.to_le_bytes());
        dst[8..12].copy_from_slice(&self.events.to_le_bytes());
        dst[12..16].copy_from_slice(&self.revents.to_le_bytes());
        Ok(())
    }

    pub(crate) fn decode(src: &[u8]) -> Result<Self, PollEntryEncodeError> {
        if src.len() < POLL_ENTRY_SIZE {
            return Err(PollEntryEncodeError::BufferTooSmall);
        }
        let mut fd = [0u8; 8];
        let mut events = [0u8; 4];
        let mut revents = [0u8; 4];
        fd.copy_from_slice(&src[..8]);
        events.copy_from_slice(&src[8..12]);
        revents.copy_from_slice(&src[12..16]);
        Ok(Self {
            fd: u64::from_le_bytes(fd),
            events: u32::from_le_bytes(events),
            revents: u32::from_le_bytes(revents),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PollEntryEncodeError {
    BufferTooSmall,
}

pub(crate) const fn socket(domain: u64, socket_type: u64, protocol: u64) -> SocketAbiCall {
    SocketAbiCall::new(SOCKET, [domain, socket_type, protocol, 0, 0, 0])
}

pub(crate) const fn inet_stream_socket() -> SocketAbiCall {
    socket(AF_INET, SOCK_STREAM, PROTOCOL_DEFAULT)
}

pub(crate) const fn bind(fd: u64, ipv4_be: u32, port: u16) -> SocketAbiCall {
    SocketAbiCall::new(BIND, [fd, ipv4_be as u64, port as u64, 0, 0, 0])
}

pub(crate) const fn listen(fd: u64, backlog: u64) -> SocketAbiCall {
    SocketAbiCall::new(LISTEN, [fd, backlog, 0, 0, 0, 0])
}

pub(crate) const fn connect(fd: u64, ipv4_be: u32, port: u16) -> SocketAbiCall {
    SocketAbiCall::new(CONNECT, [fd, ipv4_be as u64, port as u64, 0, 0, 0])
}

pub(crate) const fn accept(fd: u64) -> SocketAbiCall {
    SocketAbiCall::new(ACCEPT, [fd, 0, 0, 0, 0, 0])
}

pub(crate) const fn send(fd: u64, buffer_start: u64, len: u64) -> SocketAbiCall {
    SocketAbiCall::new(SEND, [fd, buffer_start, len, 0, 0, 0])
}

pub(crate) const fn recv(fd: u64, buffer_start: u64, len: u64) -> SocketAbiCall {
    SocketAbiCall::new(RECV, [fd, buffer_start, len, 0, 0, 0])
}

pub(crate) const fn poll(entries_start: u64, entry_count: u64) -> SocketAbiCall {
    SocketAbiCall::new(POLL, [entries_start, entry_count, 0, 0, 0, 0])
}

pub(crate) const fn poll_wait(
    entries_start: u64,
    entry_count: u64,
    timeout_ticks: u64,
) -> SocketAbiCall {
    SocketAbiCall::new(
        POLL_WAIT,
        [entries_start, entry_count, timeout_ticks, 0, 0, 0],
    )
}

pub(crate) const fn close(fd: u64) -> SocketAbiCall {
    SocketAbiCall::new(CLOSE, [fd, 0, 0, 0, 0, 0])
}

#[cfg(target_arch = "aarch64")]
#[allow(dead_code)]
unsafe fn syscall6(number: u64, arguments: [u64; crate::syscall::MAX_SCALAR_ARGUMENTS]) -> u64 {
    let mut x0 = arguments[0];
    unsafe {
        core::arch::asm!(
            "svc #0",
            inout("x0") x0,
            in("x1") arguments[1],
            in("x2") arguments[2],
            in("x3") arguments[3],
            in("x4") arguments[4],
            in("x5") arguments[5],
            in("x8") number,
            options(nostack),
        );
    }
    x0
}

#[cfg(test)]
mod tests {
    use super::*;

    const USER_MEMORY_START: u64 = 0x0000_0000_0011_0000;

    struct NullConsole;

    impl crate::runtime_console::ConsoleBackend for NullConsole {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    #[test_case]
    fn userspace_socket_abi_constants_match_private_kernel_contract() {
        assert_eq!(SVC_IMMEDIATE, 0);
        assert_eq!(CLOSE, 2);
        assert_eq!(SOCKET, 6);
        assert_eq!(BIND, 7);
        assert_eq!(LISTEN, 8);
        assert_eq!(CONNECT, 9);
        assert_eq!(ACCEPT, 10);
        assert_eq!(SEND, 11);
        assert_eq!(RECV, 12);
        assert_eq!(POLL, 13);
        assert_eq!(POLL_WAIT, 14);
        assert_eq!(AF_INET, 2);
        assert_eq!(SOCK_STREAM, 1);
        assert_eq!(PROTOCOL_DEFAULT, 0);
        assert_eq!(POLL_ENTRY_SIZE, 16);
        assert_eq!(POLL_MAX_ENTRIES, 8);
        assert_eq!(POLL_WAIT_MAX_TICKS, 1024);
        assert_eq!(
            POLL_READ,
            crate::network::NetworkSocketReadiness::READ.bits()
        );
        assert_eq!(
            POLL_WRITE,
            crate::network::NetworkSocketReadiness::WRITE.bits()
        );
        assert_eq!(
            POLL_HANGUP,
            crate::network::NetworkSocketReadiness::HANGUP.bits()
        );
        assert_eq!(
            POLL_ERROR,
            crate::network::NetworkSocketReadiness::ERROR.bits()
        );
        assert_eq!(ERRNO_EAGAIN, 11);
        assert_eq!(ERRNO_EBADF, 9);
        assert_eq!(ERRNO_EFAULT, 14);
        assert_eq!(ERRNO_EINVAL, 22);
        assert_eq!(ERRNO_ENOSPC, 28);
        assert_eq!(ERRNO_ENOTSUP, 95);

        let entry = PollEntry::with_revents(4, POLL_READ | POLL_WRITE, POLL_HANGUP);
        assert_eq!(entry.fd(), 4);
        assert_eq!(entry.events(), POLL_READ | POLL_WRITE);
        let mut encoded = [0u8; POLL_ENTRY_SIZE];
        entry.encode(&mut encoded).expect("encode poll entry");
        assert_eq!(PollEntry::decode(&encoded), Ok(entry));
        assert_eq!(socket(1, 2, 3).arguments(), [1, 2, 3, 0, 0, 0]);
    }

    #[test_case]
    fn userspace_socket_abi_wrappers_reach_smoltcp_tcp_socket_bridge() {
        let owner = crate::scheduler::ProcessOwnerId::new(61).expect("owner id");
        let mut store = crate::posix::ProcessDescriptorStore::<1, 8>::new_empty();
        store
            .create_owner_with_inherited_stdio(owner)
            .expect("create owner");
        let mut sockets = crate::network::NetworkSocketDescriptorTable::<4>::new();
        let mut user_memory = [0u8; 128];
        let mut scratch = [0u8; crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY];
        let mappings = [crate::posix::UserMapping::new(
            USER_MEMORY_START,
            user_memory.len(),
            crate::posix::UserMappingPermissions::USER_DATA,
        )
        .expect("user data mapping")];
        let endpoint = crate::network::Ipv4Endpoint::new(0x7f00_0001, 8080);

        let listener_fd = dispatch(
            inet_stream_socket(),
            Some(owner),
            &mut store,
            &mut sockets,
            &mappings,
            &mut user_memory,
            &mut scratch,
        );
        assert_eq!(listener_fd, 3);
        assert_eq!(
            dispatch(
                bind(listener_fd, endpoint.ipv4_be(), endpoint.port()),
                Some(owner),
                &mut store,
                &mut sockets,
                &mappings,
                &mut user_memory,
                &mut scratch,
            ),
            0
        );
        assert_eq!(
            dispatch(
                listen(listener_fd, 1),
                Some(owner),
                &mut store,
                &mut sockets,
                &mappings,
                &mut user_memory,
                &mut scratch,
            ),
            0
        );

        let client_fd = dispatch(
            inet_stream_socket(),
            Some(owner),
            &mut store,
            &mut sockets,
            &mappings,
            &mut user_memory,
            &mut scratch,
        );
        assert_eq!(client_fd, 4);
        assert_eq!(
            dispatch(
                connect(client_fd, endpoint.ipv4_be(), endpoint.port()),
                Some(owner),
                &mut store,
                &mut sockets,
                &mappings,
                &mut user_memory,
                &mut scratch,
            ),
            0
        );

        let client_socket = socket_descriptor(owner, client_fd as usize, &store);
        let connection_id = match sockets
            .socket(client_socket)
            .expect("client socket")
            .state()
        {
            crate::network::NetworkSocketState::Connected { connection_id, .. } => connection_id,
            state => panic!("unexpected client socket state {state:?}"),
        };
        let bridge = sockets
            .smoltcp_bridge_record(connection_id)
            .expect("smoltcp bridge record");
        assert_eq!(
            bridge.handshake().client_state(),
            smoltcp::socket::tcp::State::Established
        );
        assert_eq!(
            bridge.handshake().server_state(),
            smoltcp::socket::tcp::State::Established
        );

        let accepted_fd = dispatch(
            accept(listener_fd),
            Some(owner),
            &mut store,
            &mut sockets,
            &mappings,
            &mut user_memory,
            &mut scratch,
        );
        assert_eq!(accepted_fd, 5);
        let accepted_socket = socket_descriptor(owner, accepted_fd as usize, &store);
        assert_eq!(
            sockets
                .smoltcp_bridge_record(connection_id)
                .expect("bridge after accept")
                .accepted_descriptor(),
            Some(accepted_socket)
        );

        user_memory[0x20..0x26].copy_from_slice(b"talos!");
        assert_eq!(
            dispatch(
                send(client_fd, USER_MEMORY_START + 0x20, 6),
                Some(owner),
                &mut store,
                &mut sockets,
                &mappings,
                &mut user_memory,
                &mut scratch,
            ),
            6
        );
        let bridge = sockets
            .smoltcp_bridge_record(connection_id)
            .expect("bridge after payload");
        assert_eq!(bridge.payload_transfers(), 1);
        assert_eq!(bridge.last_payload().payload_len(), 6);
        assert_eq!(
            dispatch(
                recv(accepted_fd, USER_MEMORY_START + 0x40, 8),
                Some(owner),
                &mut store,
                &mut sockets,
                &mappings,
                &mut user_memory,
                &mut scratch,
            ),
            6
        );
        assert_eq!(&user_memory[0x40..0x46], b"talos!");
    }

    fn dispatch<
        const OWNER_CAPACITY: usize,
        const DESCRIPTOR_CAPACITY: usize,
        const SOCKET_CAPACITY: usize,
    >(
        call: SocketAbiCall,
        current_owner: Option<crate::scheduler::ProcessOwnerId>,
        descriptor_store: &mut crate::posix::ProcessDescriptorStore<
            OWNER_CAPACITY,
            DESCRIPTOR_CAPACITY,
        >,
        socket_table: &mut crate::network::NetworkSocketDescriptorTable<SOCKET_CAPACITY>,
        mappings: &[crate::posix::UserMapping],
        user_memory: &mut [u8],
        kernel_scratch: &mut [u8],
    ) -> u64 {
        crate::syscall::dispatch_process_descriptor_with_socket_table(
            call.number(),
            call.syscall_arguments(),
            current_owner,
            descriptor_store,
            socket_table,
            mappings,
            USER_MEMORY_START,
            user_memory,
            kernel_scratch,
            &mut NullConsole,
        )
        .return_value()
        .x0()
    }

    fn socket_descriptor<const OWNER_CAPACITY: usize, const DESCRIPTOR_CAPACITY: usize>(
        owner: crate::scheduler::ProcessOwnerId,
        process_descriptor: usize,
        store: &crate::posix::ProcessDescriptorStore<OWNER_CAPACITY, DESCRIPTOR_CAPACITY>,
    ) -> crate::network::NetworkSocketDescriptor {
        let entry = store
            .descriptor_table(owner)
            .expect("owner descriptor table")
            .get(process_descriptor)
            .expect("socket process descriptor");
        assert_eq!(
            entry.object().kind(),
            crate::posix::DescriptorObjectKind::Socket
        );
        crate::network::NetworkSocketDescriptor::from_raw(entry.object().reference())
    }
}
