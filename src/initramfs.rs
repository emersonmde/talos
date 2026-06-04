//! Target-independent read-only initramfs/VFS primitives.
//!
//! This module owns only the immutable fixture object model, normalized path
//! lookup, regular-file open-file descriptions, and all-or-nothing byte reads.
//! It does not parse boot archives, publish filesystems to descriptor syscalls,
//! load programs, or perform target I/O.

use crate::posix::{
    DEFAULT_PATH_LIMITS, DEFAULT_USER_COPY_LIMIT, DescriptorAccess, DescriptorEntry,
    DescriptorFlags, DescriptorObject, DescriptorObjectKind, DescriptorTable, PathLimits,
    PosixError, UserMapping, copy_to_user, normalize_path,
};

pub(crate) const DEFAULT_MAX_PATH_COMPONENTS: usize = 64;
pub(crate) const PHASE8_FIXTURE_NAME: &str = "phase8-readonly-initramfs-vfs-v1";
pub(crate) const PHASE8_BANNER_PATH: &[u8] = b"/etc/banner.txt";
pub(crate) const PHASE8_BANNER_BYTES: &[u8] = b"Talos initramfs fixture\n";
pub(crate) const PHASE8_INIT_PATH: &[u8] = b"/bin/init";
pub(crate) const PHASE8_INIT_ELF_LEN: usize = 0x204;
pub(crate) const PHASE8_INIT_BYTES: &[u8] = &PHASE8_INIT_ELF_BYTES;
pub(crate) const PHASE8_INIT_TEXT_OFFSET: usize = 0x100;
pub(crate) const PHASE8_INIT_EXIT_STATUS: u64 = 0;
pub(crate) const PHASE8_INIT_SVC_MARKER: u64 = 0x7a10;
pub(crate) const PHASE10_ZERO_PATH: &[u8] = b"/bin/zero";
pub(crate) const PHASE10_ZERO_BYTES: &[u8] = &PHASE10_ZERO_ELF_BYTES;
pub(crate) const PHASE10_STATUS42_PATH: &[u8] = b"/bin/status42";
pub(crate) const PHASE10_STATUS42_EXIT_STATUS: u64 = 42;
pub(crate) const PHASE10_STATUS42_BYTES: &[u8] = &PHASE10_STATUS42_ELF_BYTES;
pub(crate) const PHASE10_STDOUT_PATH: &[u8] = b"/bin/stdout";
pub(crate) const PHASE10_STDOUT_PAYLOAD: &[u8] = b"Talos userspace stdout fixture\n";
pub(crate) const PHASE10_STDOUT_BYTES: &[u8] = &PHASE10_STDOUT_ELF_BYTES;
pub(crate) const PHASE10_STDIN_PATH: &[u8] = b"/bin/stdin";
pub(crate) const PHASE10_STDIN_INPUT_BYTES: &[u8] = b"talos-fd0\n";
pub(crate) const PHASE10_STDIN_STDOUT_PREFIX: &[u8] = b"Talos userspace stdin fixture read: ";
pub(crate) const PHASE10_STDIN_READINESS_STDOUT: &[u8] =
    b"Talos userspace stdin fixture no-data: readiness\n";
pub(crate) const PHASE10_STDIN_TERMINAL_EOF_STDOUT: &[u8] =
    b"Talos userspace stdin fixture read-result: terminal-eof\n";
pub(crate) const PHASE10_STDIN_BYTES: &[u8] = &PHASE10_STDIN_ELF_BYTES;
pub(crate) const PHASE10_STDERR_PATH: &[u8] = b"/bin/stderr";
pub(crate) const PHASE10_STDERR_PAYLOAD: &[u8] = b"Talos userspace stderr fixture\n";
pub(crate) const PHASE10_STDERR_BYTES: &[u8] = &PHASE10_STDERR_ELF_BYTES;
pub(crate) const PHASE8_EMPTY_PATH: &[u8] = b"/empty";
pub(crate) const PHASE8_NESTED_PATH: &[u8] = b"/dir/nested.txt";
pub(crate) const PHASE8_NESTED_BYTES: &[u8] = b"nested fixture\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VfsNodeKind {
    Directory,
    RegularFile,
}

impl VfsNodeKind {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Directory => "directory",
            Self::RegularFile => "regular",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DirectoryEntry {
    name: &'static [u8],
    node_index: usize,
}

impl DirectoryEntry {
    pub(crate) const fn new(name: &'static [u8], node_index: usize) -> Self {
        Self { name, node_index }
    }

    pub(crate) const fn name(self) -> &'static [u8] {
        self.name
    }

    pub(crate) const fn node_index(self) -> usize {
        self.node_index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitramfsNodeData {
    Directory(&'static [DirectoryEntry]),
    RegularFile(&'static [u8]),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InitramfsNode {
    id: usize,
    data: InitramfsNodeData,
}

impl InitramfsNode {
    pub(crate) const fn directory(id: usize, entries: &'static [DirectoryEntry]) -> Self {
        Self {
            id,
            data: InitramfsNodeData::Directory(entries),
        }
    }

    pub(crate) const fn regular_file(id: usize, bytes: &'static [u8]) -> Self {
        Self {
            id,
            data: InitramfsNodeData::RegularFile(bytes),
        }
    }

    pub(crate) const fn id(self) -> usize {
        self.id
    }

    pub(crate) const fn kind(self) -> VfsNodeKind {
        match self.data {
            InitramfsNodeData::Directory(_) => VfsNodeKind::Directory,
            InitramfsNodeData::RegularFile(_) => VfsNodeKind::RegularFile,
        }
    }

    pub(crate) const fn len(self) -> usize {
        match self.data {
            InitramfsNodeData::Directory(entries) => entries.len(),
            InitramfsNodeData::RegularFile(bytes) => bytes.len(),
        }
    }

    pub(crate) const fn is_directory(self) -> bool {
        matches!(self.data, InitramfsNodeData::Directory(_))
    }

    pub(crate) const fn is_regular_file(self) -> bool {
        matches!(self.data, InitramfsNodeData::RegularFile(_))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct VfsMetadata {
    node_id: usize,
    kind: VfsNodeKind,
    len: usize,
    read_only: bool,
}

impl VfsMetadata {
    pub(crate) const fn node_id(self) -> usize {
        self.node_id
    }

    pub(crate) const fn kind(self) -> VfsNodeKind {
        self.kind
    }

    pub(crate) const fn len(self) -> usize {
        self.len
    }

    pub(crate) const fn read_only(self) -> bool {
        self.read_only
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct VfsNodeHandle {
    index: usize,
    metadata: VfsMetadata,
}

impl VfsNodeHandle {
    pub(crate) const fn index(self) -> usize {
        self.index
    }

    pub(crate) const fn metadata(self) -> VfsMetadata {
        self.metadata
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadOnlyFileDescription {
    node_index: usize,
    offset: usize,
}

impl ReadOnlyFileDescription {
    pub(crate) const fn offset(self) -> usize {
        self.offset
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadOnlyFileDescriptions<const CAPACITY: usize> {
    entries: [Option<ReadOnlyFileDescription>; CAPACITY],
}

impl<const CAPACITY: usize> ReadOnlyFileDescriptions<CAPACITY> {
    pub(crate) const fn new_empty() -> Self {
        Self {
            entries: [None; CAPACITY],
        }
    }

    pub(crate) fn insert(
        &mut self,
        reference: usize,
        description: ReadOnlyFileDescription,
    ) -> Result<(), PosixError> {
        let Some(slot) = self.entries.get_mut(reference) else {
            return Err(PosixError::InvalidArgument);
        };
        if slot.is_some() {
            return Err(PosixError::InvalidArgument);
        }
        *slot = Some(description);
        Ok(())
    }

    pub(crate) fn allocate(
        &mut self,
        description: ReadOnlyFileDescription,
    ) -> Result<usize, PosixError> {
        let mut reference = 0;
        while reference < CAPACITY {
            if self.entries[reference].is_none() {
                self.entries[reference] = Some(description);
                return Ok(reference);
            }
            reference += 1;
        }
        Err(PosixError::TooManyOpenFiles)
    }

    pub(crate) fn remove(
        &mut self,
        reference: usize,
    ) -> Result<ReadOnlyFileDescription, PosixError> {
        self.entries
            .get_mut(reference)
            .and_then(Option::take)
            .ok_or(PosixError::BadDescriptor)
    }

    pub(crate) fn get_mut(
        &mut self,
        reference: usize,
    ) -> Result<&mut ReadOnlyFileDescription, PosixError> {
        self.entries
            .get_mut(reference)
            .and_then(Option::as_mut)
            .ok_or(PosixError::BadDescriptor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadOnlyInitramfs {
    nodes: &'static [InitramfsNode],
    root_index: usize,
}

impl ReadOnlyInitramfs {
    pub(crate) const fn new(nodes: &'static [InitramfsNode], root_index: usize) -> Self {
        Self { nodes, root_index }
    }

    pub(crate) fn validate(self) -> Result<(), PosixError> {
        let root = self.node(self.root_index)?;
        if !root.is_directory() {
            return Err(PosixError::InvalidArgument);
        }

        let mut index = 0;
        while index < self.nodes.len() {
            let node = self.nodes[index];
            if node.id() != index {
                return Err(PosixError::InvalidArgument);
            }
            if let InitramfsNodeData::Directory(entries) = node.data {
                validate_directory_entries(entries, self.nodes.len())?;
            }
            index += 1;
        }

        Ok(())
    }

    pub(crate) fn lookup(
        self,
        path: &[u8],
        limits: PathLimits,
    ) -> Result<VfsNodeHandle, PosixError> {
        self.validate()?;

        let normalized = normalize_path::<DEFAULT_MAX_PATH_COMPONENTS>(path, limits)?;
        let mut cursor = self.root_index;

        for component in normalized.components() {
            let current = self.node(cursor)?;
            let entries = match current.data {
                InitramfsNodeData::Directory(entries) => entries,
                InitramfsNodeData::RegularFile(_) => return Err(PosixError::NotDirectory),
            };
            cursor = find_entry(entries, component.bytes()).ok_or(PosixError::NoEntry)?;
        }

        let node = self.node(cursor)?;
        if normalized.requires_directory() && node.is_regular_file() {
            return Err(PosixError::NotDirectory);
        }

        Ok(self.handle_for(cursor, node))
    }

    pub(crate) fn lookup_default(self, path: &[u8]) -> Result<VfsNodeHandle, PosixError> {
        self.lookup(path, DEFAULT_PATH_LIMITS)
    }

    pub(crate) fn open_regular_file(
        self,
        path: &[u8],
    ) -> Result<ReadOnlyFileDescription, PosixError> {
        let handle = self.lookup_default(path)?;
        if handle.metadata().kind() == VfsNodeKind::Directory {
            return Err(PosixError::IsDirectory);
        }

        Ok(ReadOnlyFileDescription {
            node_index: handle.index(),
            offset: 0,
        })
    }

    pub(crate) fn regular_file_bytes(self, path: &[u8]) -> Result<&'static [u8], PosixError> {
        let handle = self.lookup_default(path)?;
        let node = self.node(handle.index())?;
        match node.data {
            InitramfsNodeData::Directory(_) => Err(PosixError::IsDirectory),
            InitramfsNodeData::RegularFile(bytes) => Ok(bytes),
        }
    }

    pub(crate) fn unsupported_operation(self) -> Result<(), PosixError> {
        Err(PosixError::NotSupported)
    }

    pub(crate) fn open_regular_descriptor<
        const DESCRIPTOR_CAPACITY: usize,
        const FILE_CAPACITY: usize,
    >(
        self,
        descriptor_table: &mut DescriptorTable<DESCRIPTOR_CAPACITY>,
        file_descriptions: &mut ReadOnlyFileDescriptions<FILE_CAPACITY>,
        path: &[u8],
    ) -> Result<usize, PosixError> {
        let description = self.open_regular_file(path)?;
        let reference = file_descriptions.allocate(description)?;
        let entry = DescriptorEntry::new(
            DescriptorAccess::ReadOnly,
            DescriptorFlags::EMPTY,
            DescriptorObject::new(DescriptorObjectKind::RegularFile, reference),
        );

        match descriptor_table.allocate(entry) {
            Ok(descriptor) => Ok(descriptor),
            Err(error) => {
                let _ = file_descriptions.remove(reference);
                Err(error)
            }
        }
    }

    pub(crate) fn read_descriptor<const DESCRIPTOR_CAPACITY: usize, const FILE_CAPACITY: usize>(
        self,
        descriptor_table: &DescriptorTable<DESCRIPTOR_CAPACITY>,
        file_descriptions: &mut ReadOnlyFileDescriptions<FILE_CAPACITY>,
        descriptor: usize,
        mappings: &[UserMapping],
        user_memory_start: u64,
        user_memory: &mut [u8],
        user_start: u64,
        len: usize,
        kernel_scratch: &mut [u8],
    ) -> Result<usize, PosixError> {
        let entry = descriptor_table.get(descriptor)?;
        entry.require_readable()?;

        match entry.object().kind() {
            DescriptorObjectKind::RegularFile => {}
            DescriptorObjectKind::Directory => return Err(PosixError::IsDirectory),
            _ => return Err(PosixError::NotSupported),
        }

        let description = file_descriptions.get_mut(entry.object().reference())?;
        self.read_regular_file(
            description,
            mappings,
            user_memory_start,
            user_memory,
            user_start,
            len,
            kernel_scratch,
        )
    }

    pub(crate) fn read_regular_file(
        self,
        description: &mut ReadOnlyFileDescription,
        mappings: &[UserMapping],
        user_memory_start: u64,
        user_memory: &mut [u8],
        user_start: u64,
        len: usize,
        kernel_scratch: &mut [u8],
    ) -> Result<usize, PosixError> {
        let node = self.node(description.node_index)?;
        let bytes = match node.data {
            InitramfsNodeData::Directory(_) => return Err(PosixError::IsDirectory),
            InitramfsNodeData::RegularFile(bytes) => bytes,
        };
        if description.offset > bytes.len() {
            return Err(PosixError::InvalidArgument);
        }
        if len == 0 || description.offset == bytes.len() {
            return Ok(0);
        }
        if len > DEFAULT_USER_COPY_LIMIT {
            return Err(PosixError::Fault);
        }

        let selected_len = core::cmp::min(len, bytes.len() - description.offset);
        if kernel_scratch.len() < selected_len {
            return Err(PosixError::InvalidArgument);
        }

        let end = description.offset + selected_len;
        kernel_scratch[..selected_len].copy_from_slice(&bytes[description.offset..end]);
        copy_to_user(
            mappings,
            user_memory_start,
            user_memory,
            user_start,
            selected_len,
            &kernel_scratch[..selected_len],
        )?;
        description.offset = end;
        Ok(selected_len)
    }

    pub(crate) fn read_regular_file_to_kernel(
        self,
        description: &mut ReadOnlyFileDescription,
        kernel_buffer: &mut [u8],
    ) -> Result<usize, PosixError> {
        let node = self.node(description.node_index)?;
        let bytes = match node.data {
            InitramfsNodeData::Directory(_) => return Err(PosixError::IsDirectory),
            InitramfsNodeData::RegularFile(bytes) => bytes,
        };
        if description.offset > bytes.len() {
            return Err(PosixError::InvalidArgument);
        }
        if description.offset == bytes.len() {
            return Ok(0);
        }

        let selected_len = core::cmp::min(kernel_buffer.len(), bytes.len() - description.offset);
        let end = description.offset + selected_len;
        kernel_buffer[..selected_len].copy_from_slice(&bytes[description.offset..end]);
        description.offset = end;
        Ok(selected_len)
    }

    fn node(self, index: usize) -> Result<InitramfsNode, PosixError> {
        self.nodes
            .get(index)
            .copied()
            .ok_or(PosixError::InvalidArgument)
    }

    fn handle_for(self, index: usize, node: InitramfsNode) -> VfsNodeHandle {
        VfsNodeHandle {
            index,
            metadata: VfsMetadata {
                node_id: node.id(),
                kind: node.kind(),
                len: node.len(),
                read_only: true,
            },
        }
    }
}

pub(crate) fn phase8_readonly_initramfs_fixture() -> ReadOnlyInitramfs {
    ReadOnlyInitramfs::new(&PHASE8_NODES, PHASE8_ROOT_INDEX)
}

fn validate_directory_entries(
    entries: &[DirectoryEntry],
    node_count: usize,
) -> Result<(), PosixError> {
    let mut index = 0;
    while index < entries.len() {
        let entry = entries[index];
        if entry.name().is_empty()
            || entry.name().contains(&b'/')
            || entry.name().contains(&0)
            || entry.node_index() >= node_count
        {
            return Err(PosixError::InvalidArgument);
        }

        let mut other = index + 1;
        while other < entries.len() {
            if entries[other].name() == entry.name() {
                return Err(PosixError::InvalidArgument);
            }
            other += 1;
        }
        index += 1;
    }

    Ok(())
}

fn find_entry(entries: &[DirectoryEntry], name: &[u8]) -> Option<usize> {
    let mut index = 0;
    while index < entries.len() {
        if entries[index].name() == name {
            return Some(entries[index].node_index());
        }
        index += 1;
    }
    None
}

const PHASE8_ROOT_INDEX: usize = 0;
const PHASE8_ETC_INDEX: usize = 1;
const PHASE8_BANNER_INDEX: usize = 2;
const PHASE8_BIN_INDEX: usize = 3;
const PHASE8_INIT_INDEX: usize = 4;
const PHASE10_ZERO_INDEX: usize = 5;
const PHASE10_STATUS42_INDEX: usize = 6;
const PHASE10_STDOUT_INDEX: usize = 7;
const PHASE10_STDIN_INDEX: usize = 8;
const PHASE10_STDERR_INDEX: usize = 9;
const PHASE8_EMPTY_INDEX: usize = 10;
const PHASE8_DIR_INDEX: usize = 11;
const PHASE8_NESTED_INDEX: usize = 12;

static PHASE8_ROOT_ENTRIES: [DirectoryEntry; 4] = [
    DirectoryEntry::new(b"etc", PHASE8_ETC_INDEX),
    DirectoryEntry::new(b"bin", PHASE8_BIN_INDEX),
    DirectoryEntry::new(b"empty", PHASE8_EMPTY_INDEX),
    DirectoryEntry::new(b"dir", PHASE8_DIR_INDEX),
];

static PHASE8_ETC_ENTRIES: [DirectoryEntry; 1] =
    [DirectoryEntry::new(b"banner.txt", PHASE8_BANNER_INDEX)];

static PHASE8_BIN_ENTRIES: [DirectoryEntry; 6] = [
    DirectoryEntry::new(b"init", PHASE8_INIT_INDEX),
    DirectoryEntry::new(b"zero", PHASE10_ZERO_INDEX),
    DirectoryEntry::new(b"status42", PHASE10_STATUS42_INDEX),
    DirectoryEntry::new(b"stdout", PHASE10_STDOUT_INDEX),
    DirectoryEntry::new(b"stdin", PHASE10_STDIN_INDEX),
    DirectoryEntry::new(b"stderr", PHASE10_STDERR_INDEX),
];

static PHASE8_DIR_ENTRIES: [DirectoryEntry; 1] =
    [DirectoryEntry::new(b"nested.txt", PHASE8_NESTED_INDEX)];

static PHASE8_INIT_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE8_INIT_EXIT_STATUS);
static PHASE10_ZERO_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE8_INIT_EXIT_STATUS);
static PHASE10_STATUS42_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE10_STATUS42_EXIT_STATUS);
static PHASE10_STDOUT_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE8_INIT_EXIT_STATUS);
static PHASE10_STDIN_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE8_INIT_EXIT_STATUS);
static PHASE10_STDERR_ELF_BYTES: [u8; PHASE8_INIT_ELF_LEN] =
    build_phase8_exit_elf_bytes(PHASE8_INIT_EXIT_STATUS);

static PHASE8_NODES: [InitramfsNode; 13] = [
    InitramfsNode::directory(PHASE8_ROOT_INDEX, &PHASE8_ROOT_ENTRIES),
    InitramfsNode::directory(PHASE8_ETC_INDEX, &PHASE8_ETC_ENTRIES),
    InitramfsNode::regular_file(PHASE8_BANNER_INDEX, PHASE8_BANNER_BYTES),
    InitramfsNode::directory(PHASE8_BIN_INDEX, &PHASE8_BIN_ENTRIES),
    InitramfsNode::regular_file(PHASE8_INIT_INDEX, PHASE8_INIT_BYTES),
    InitramfsNode::regular_file(PHASE10_ZERO_INDEX, PHASE10_ZERO_BYTES),
    InitramfsNode::regular_file(PHASE10_STATUS42_INDEX, PHASE10_STATUS42_BYTES),
    InitramfsNode::regular_file(PHASE10_STDOUT_INDEX, PHASE10_STDOUT_BYTES),
    InitramfsNode::regular_file(PHASE10_STDIN_INDEX, PHASE10_STDIN_BYTES),
    InitramfsNode::regular_file(PHASE10_STDERR_INDEX, PHASE10_STDERR_BYTES),
    InitramfsNode::regular_file(PHASE8_EMPTY_INDEX, b""),
    InitramfsNode::directory(PHASE8_DIR_INDEX, &PHASE8_DIR_ENTRIES),
    InitramfsNode::regular_file(PHASE8_NESTED_INDEX, PHASE8_NESTED_BYTES),
];

const fn build_phase8_exit_elf_bytes(exit_status: u64) -> [u8; PHASE8_INIT_ELF_LEN] {
    const EHDR_LEN: usize = 64;
    const PHENT_LEN: usize = 56;
    const DATA_OFFSET: usize = 0x200;
    const TEXT_VADDR: u64 = 0x0000_0000_0001_0100;
    const DATA_VADDR: u64 = 0x0000_0000_0002_0200;
    const ENTRY: u64 = TEXT_VADDR;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const PF_R: u32 = 4;
    const PAGE_ALIGN: u64 = 0x1000;

    let mut bytes = [0u8; PHASE8_INIT_ELF_LEN];

    bytes[0] = 0x7f;
    bytes[1] = b'E';
    bytes[2] = b'L';
    bytes[3] = b'F';
    bytes[4] = 2;
    bytes[5] = 1;
    bytes[6] = 1;
    bytes[7] = 0;

    write_le_u16(&mut bytes, 16, 2);
    write_le_u16(&mut bytes, 18, 183);
    write_le_u32(&mut bytes, 20, 1);
    write_le_u64(&mut bytes, 24, ENTRY);
    write_le_u64(&mut bytes, 32, EHDR_LEN as u64);
    write_le_u16(&mut bytes, 52, EHDR_LEN as u16);
    write_le_u16(&mut bytes, 54, PHENT_LEN as u16);
    write_le_u16(&mut bytes, 56, 2);

    write_load_phdr(
        &mut bytes,
        EHDR_LEN,
        PF_R | PF_X,
        PHASE8_INIT_TEXT_OFFSET as u64,
        TEXT_VADDR,
        8,
        8,
        PAGE_ALIGN,
    );
    write_load_phdr(
        &mut bytes,
        EHDR_LEN + PHENT_LEN,
        PF_R | PF_W,
        DATA_OFFSET as u64,
        DATA_VADDR,
        4,
        0x1004,
        PAGE_ALIGN,
    );

    let exit_status = (exit_status & 0xffff) as u32;
    let movz_x0 = 0xd280_0000u32 | (exit_status << 5);
    bytes[PHASE8_INIT_TEXT_OFFSET] = movz_x0 as u8;
    bytes[PHASE8_INIT_TEXT_OFFSET + 1] = (movz_x0 >> 8) as u8;
    bytes[PHASE8_INIT_TEXT_OFFSET + 2] = (movz_x0 >> 16) as u8;
    bytes[PHASE8_INIT_TEXT_OFFSET + 3] = 0xd2;
    bytes[PHASE8_INIT_TEXT_OFFSET + 4] = 0x01;
    bytes[PHASE8_INIT_TEXT_OFFSET + 5] = 0x42;
    bytes[PHASE8_INIT_TEXT_OFFSET + 6] = 0x0f;
    bytes[PHASE8_INIT_TEXT_OFFSET + 7] = 0xd4;
    bytes[DATA_OFFSET] = b'D';
    bytes[DATA_OFFSET + 1] = b'A';
    bytes[DATA_OFFSET + 2] = b'T';
    bytes[DATA_OFFSET + 3] = b'A';

    bytes
}

const fn write_load_phdr(
    bytes: &mut [u8; PHASE8_INIT_ELF_LEN],
    offset: usize,
    flags: u32,
    file_offset: u64,
    virtual_address: u64,
    file_size: u64,
    memory_size: u64,
    alignment: u64,
) {
    write_le_u32(bytes, offset, 1);
    write_le_u32(bytes, offset + 4, flags);
    write_le_u64(bytes, offset + 8, file_offset);
    write_le_u64(bytes, offset + 16, virtual_address);
    write_le_u64(bytes, offset + 24, virtual_address);
    write_le_u64(bytes, offset + 32, file_size);
    write_le_u64(bytes, offset + 40, memory_size);
    write_le_u64(bytes, offset + 48, alignment);
}

const fn write_le_u16(bytes: &mut [u8; PHASE8_INIT_ELF_LEN], offset: usize, value: u16) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
}

const fn write_le_u32(bytes: &mut [u8; PHASE8_INIT_ELF_LEN], offset: usize, value: u32) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
    bytes[offset + 2] = (value >> 16) as u8;
    bytes[offset + 3] = (value >> 24) as u8;
}

const fn write_le_u64(bytes: &mut [u8; PHASE8_INIT_ELF_LEN], offset: usize, value: u64) {
    bytes[offset] = value as u8;
    bytes[offset + 1] = (value >> 8) as u8;
    bytes[offset + 2] = (value >> 16) as u8;
    bytes[offset + 3] = (value >> 24) as u8;
    bytes[offset + 4] = (value >> 32) as u8;
    bytes[offset + 5] = (value >> 40) as u8;
    bytes[offset + 6] = (value >> 48) as u8;
    bytes[offset + 7] = (value >> 56) as u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posix::{
        DescriptorAccess, DescriptorEntry, DescriptorFlags, DescriptorObject, DescriptorObjectKind,
        UserMapping, UserMappingPermissions,
    };

    const USER_BASE: u64 = 0x10000;

    fn writable_mapping(len: usize) -> [UserMapping; 1] {
        [
            UserMapping::new(USER_BASE, len, UserMappingPermissions::USER_DATA)
                .expect("writable mapping"),
        ]
    }

    fn read_file_case(
        fs: ReadOnlyInitramfs,
        description: &mut ReadOnlyFileDescription,
        user_memory: &mut [u8],
        user_start: u64,
        len: usize,
        scratch: &mut [u8],
    ) -> Result<usize, PosixError> {
        let mappings = writable_mapping(user_memory.len());
        fs.read_regular_file(
            description,
            &mappings,
            USER_BASE,
            user_memory,
            user_start,
            len,
            scratch,
        )
    }

    #[test_case]
    fn phase8_fixture_name_is_stable() {
        assert_eq!(PHASE8_FIXTURE_NAME, "phase8-readonly-initramfs-vfs-v1");
        assert_eq!(VfsNodeKind::Directory.name(), "directory");
        assert_eq!(VfsNodeKind::RegularFile.name(), "regular");
    }

    #[test_case]
    fn fixture_validates_root_and_unique_directory_entries() {
        let fs = phase8_readonly_initramfs_fixture();
        assert_eq!(fs.validate(), Ok(()));

        static DUPLICATE_ENTRIES: [DirectoryEntry; 2] = [
            DirectoryEntry::new(b"dup", 0),
            DirectoryEntry::new(b"dup", 0),
        ];
        static DUPLICATE_NODES: [InitramfsNode; 1] =
            [InitramfsNode::directory(0, &DUPLICATE_ENTRIES)];
        assert_eq!(
            ReadOnlyInitramfs::new(&DUPLICATE_NODES, 0).validate(),
            Err(PosixError::InvalidArgument)
        );

        static BAD_REFERENCE_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"missing", 3)];
        static BAD_REFERENCE_NODES: [InitramfsNode; 1] =
            [InitramfsNode::directory(0, &BAD_REFERENCE_ENTRIES)];
        assert_eq!(
            ReadOnlyInitramfs::new(&BAD_REFERENCE_NODES, 0).validate(),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn lookup_reports_root_directory_and_contract_files() {
        let fs = phase8_readonly_initramfs_fixture();

        let root = fs.lookup_default(b"/").expect("root lookup");
        assert_eq!(root.metadata().kind(), VfsNodeKind::Directory);
        assert_eq!(root.metadata().len(), 4);
        assert!(root.metadata().read_only());

        let banner = fs
            .lookup_default(PHASE8_BANNER_PATH)
            .expect("banner lookup");
        assert_eq!(banner.metadata().kind(), VfsNodeKind::RegularFile);
        assert_eq!(banner.metadata().len(), PHASE8_BANNER_BYTES.len());

        let init = fs.lookup_default(PHASE8_INIT_PATH).expect("init lookup");
        assert_eq!(init.metadata().kind(), VfsNodeKind::RegularFile);
        assert_eq!(init.metadata().len(), PHASE8_INIT_BYTES.len());

        let empty = fs.lookup_default(PHASE8_EMPTY_PATH).expect("empty lookup");
        assert_eq!(empty.metadata().kind(), VfsNodeKind::RegularFile);
        assert_eq!(empty.metadata().len(), 0);

        let nested = fs
            .lookup_default(PHASE8_NESTED_PATH)
            .expect("nested lookup");
        assert_eq!(nested.metadata().kind(), VfsNodeKind::RegularFile);
        assert_eq!(nested.metadata().len(), PHASE8_NESTED_BYTES.len());
    }

    #[test_case]
    fn lookup_uses_accepted_path_normalization_from_root_and_cwd() {
        let fs = phase8_readonly_initramfs_fixture();
        let absolute = fs
            .lookup_default(b"/etc/./../dir//nested.txt")
            .expect("normalized absolute path");
        let relative = fs
            .lookup_default(b"./dir/nested.txt")
            .expect("normalized relative path");

        assert_eq!(absolute.metadata().node_id(), PHASE8_NESTED_INDEX);
        assert_eq!(relative.metadata().node_id(), PHASE8_NESTED_INDEX);
    }

    #[test_case]
    fn lookup_reports_contract_error_precedence() {
        let fs = phase8_readonly_initramfs_fixture();

        assert_eq!(fs.lookup_default(b""), Err(PosixError::NoEntry));
        assert_eq!(fs.lookup_default(b"/missing"), Err(PosixError::NoEntry));
        assert_eq!(
            fs.lookup_default(b"/etc/banner.txt/child"),
            Err(PosixError::NotDirectory)
        );
        assert_eq!(
            fs.lookup_default(b"/etc/banner.txt/"),
            Err(PosixError::NotDirectory)
        );
        assert_eq!(
            fs.lookup(b"/abcde", PathLimits::new(4, 8, 4)),
            Err(PosixError::NameTooLong)
        );
        assert_eq!(
            fs.lookup_default(b"/etc/ban\0ner.txt"),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn open_regular_file_rejects_directories_as_eisdir() {
        let fs = phase8_readonly_initramfs_fixture();
        assert_eq!(fs.open_regular_file(b"/etc"), Err(PosixError::IsDirectory));
        assert_eq!(fs.unsupported_operation(), Err(PosixError::NotSupported));
    }

    #[test_case]
    fn regular_file_bytes_returns_immutable_init_fixture() {
        let fs = phase8_readonly_initramfs_fixture();
        let init = fs
            .regular_file_bytes(PHASE8_INIT_PATH)
            .expect("init fixture bytes");

        assert_eq!(init, PHASE8_INIT_BYTES);
        assert_eq!(init.len(), PHASE8_INIT_ELF_LEN);
        assert_eq!(&init[..4], b"\x7fELF");
        assert_eq!(
            &init[PHASE8_INIT_TEXT_OFFSET..PHASE8_INIT_TEXT_OFFSET + 8],
            &[0x00, 0x00, 0x80, 0xd2, 0x01, 0x42, 0x0f, 0xd4]
        );
        assert_eq!(PHASE8_INIT_EXIT_STATUS, 0);
        assert_eq!(PHASE8_INIT_SVC_MARKER, 0x7a10);
        let status42 = fs
            .regular_file_bytes(PHASE10_STATUS42_PATH)
            .expect("status42 fixture bytes");
        assert_eq!(
            &status42[PHASE8_INIT_TEXT_OFFSET..PHASE8_INIT_TEXT_OFFSET + 8],
            &[0x40, 0x05, 0x80, 0xd2, 0x01, 0x42, 0x0f, 0xd4]
        );
        assert_eq!(PHASE10_STATUS42_EXIT_STATUS, 42);
        let stdout = fs
            .regular_file_bytes(PHASE10_STDOUT_PATH)
            .expect("stdout fixture bytes");
        assert_eq!(stdout, PHASE10_STDOUT_BYTES);
        assert_eq!(
            &stdout[PHASE8_INIT_TEXT_OFFSET..PHASE8_INIT_TEXT_OFFSET + 8],
            &[0x00, 0x00, 0x80, 0xd2, 0x01, 0x42, 0x0f, 0xd4]
        );
        assert_eq!(PHASE10_STDOUT_PAYLOAD, b"Talos userspace stdout fixture\n");
        let stdin = fs
            .regular_file_bytes(PHASE10_STDIN_PATH)
            .expect("stdin fixture bytes");
        assert_eq!(stdin, PHASE10_STDIN_BYTES);
        assert_eq!(
            &stdin[PHASE8_INIT_TEXT_OFFSET..PHASE8_INIT_TEXT_OFFSET + 8],
            &[0x00, 0x00, 0x80, 0xd2, 0x01, 0x42, 0x0f, 0xd4]
        );
        assert_eq!(PHASE10_STDIN_INPUT_BYTES, b"talos-fd0\n");
        assert_eq!(
            PHASE10_STDIN_STDOUT_PREFIX,
            b"Talos userspace stdin fixture read: "
        );
        assert_eq!(
            PHASE10_STDIN_READINESS_STDOUT,
            b"Talos userspace stdin fixture no-data: readiness\n"
        );
        assert_eq!(
            PHASE10_STDIN_TERMINAL_EOF_STDOUT,
            b"Talos userspace stdin fixture read-result: terminal-eof\n"
        );
        let stderr = fs
            .regular_file_bytes(PHASE10_STDERR_PATH)
            .expect("stderr fixture bytes");
        assert_eq!(stderr, PHASE10_STDERR_BYTES);
        assert_eq!(
            &stderr[PHASE8_INIT_TEXT_OFFSET..PHASE8_INIT_TEXT_OFFSET + 8],
            &[0x00, 0x00, 0x80, 0xd2, 0x01, 0x42, 0x0f, 0xd4]
        );
        assert_eq!(PHASE10_STDERR_PAYLOAD, b"Talos userspace stderr fixture\n");
        assert_eq!(fs.regular_file_bytes(b"/etc"), Err(PosixError::IsDirectory));
    }

    #[test_case]
    fn open_regular_descriptor_attaches_file_description_to_lowest_fd() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut descriptor_table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio");
        let mut file_descriptions = ReadOnlyFileDescriptions::<2>::new_empty();

        let descriptor = fs
            .open_regular_descriptor(
                &mut descriptor_table,
                &mut file_descriptions,
                PHASE8_BANNER_PATH,
            )
            .expect("open banner through descriptor");

        assert_eq!(descriptor, 3);
        let entry = descriptor_table
            .get(descriptor)
            .expect("regular file descriptor");
        assert_eq!(entry.access(), DescriptorAccess::ReadOnly);
        assert_eq!(entry.flags(), DescriptorFlags::EMPTY);
        assert_eq!(entry.object().kind(), DescriptorObjectKind::RegularFile);
        assert_eq!(
            file_descriptions
                .get_mut(entry.object().reference())
                .expect("open file description")
                .offset(),
            0
        );
    }

    #[test_case]
    fn descriptor_reads_opened_files_and_duplicates_share_offset() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut descriptor_table = DescriptorTable::<5>::with_inherited_stdio().expect("stdio");
        let mut file_descriptions = ReadOnlyFileDescriptions::<1>::new_empty();
        let descriptor = fs
            .open_regular_descriptor(
                &mut descriptor_table,
                &mut file_descriptions,
                PHASE8_BANNER_PATH,
            )
            .expect("open banner through descriptor");
        let duplicate = descriptor_table.dup(descriptor).expect("dup regular file");
        let mut user_memory = [0u8; 64];
        let mappings = writable_mapping(user_memory.len());
        let mut scratch = [0u8; 64];

        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                descriptor,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                5,
                &mut scratch,
            ),
            Ok(5)
        );
        assert_eq!(&user_memory[..5], b"Talos");

        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                duplicate,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE + 8,
                64,
                &mut scratch,
            ),
            Ok(PHASE8_BANNER_BYTES.len() - 5)
        );
        assert_eq!(
            &user_memory[8..8 + PHASE8_BANNER_BYTES.len() - 5],
            &PHASE8_BANNER_BYTES[5..]
        );

        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                descriptor,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                64,
                &mut scratch,
            ),
            Ok(0)
        );
    }

    #[test_case]
    fn open_regular_descriptor_rolls_back_file_description_on_descriptor_failure() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut descriptor_table = DescriptorTable::<3>::with_inherited_stdio().expect("stdio");
        let mut file_descriptions = ReadOnlyFileDescriptions::<1>::new_empty();

        assert_eq!(
            fs.open_regular_descriptor(
                &mut descriptor_table,
                &mut file_descriptions,
                PHASE8_BANNER_PATH,
            ),
            Err(PosixError::TooManyOpenFiles)
        );

        let mut descriptor_table = DescriptorTable::<4>::with_inherited_stdio().expect("stdio");
        assert_eq!(
            fs.open_regular_descriptor(
                &mut descriptor_table,
                &mut file_descriptions,
                PHASE8_INIT_PATH,
            ),
            Ok(3)
        );
    }

    #[test_case]
    fn regular_file_reads_copy_bytes_advance_offset_and_report_eof() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut description = fs
            .open_regular_file(PHASE8_BANNER_PATH)
            .expect("banner regular file");
        let mut user_memory = [0u8; 64];
        let mut scratch = [0u8; 64];

        let copied = read_file_case(
            fs,
            &mut description,
            &mut user_memory,
            USER_BASE,
            64,
            &mut scratch,
        )
        .expect("read banner");

        assert_eq!(copied, PHASE8_BANNER_BYTES.len());
        assert_eq!(description.offset(), PHASE8_BANNER_BYTES.len());
        assert_eq!(
            &user_memory[..PHASE8_BANNER_BYTES.len()],
            PHASE8_BANNER_BYTES
        );

        let eof = read_file_case(
            fs,
            &mut description,
            &mut user_memory,
            USER_BASE,
            64,
            &mut scratch,
        )
        .expect("read eof");
        assert_eq!(eof, 0);
        assert_eq!(description.offset(), PHASE8_BANNER_BYTES.len());
    }

    #[test_case]
    fn regular_file_kernel_reads_copy_bytes_advance_offset_and_report_eof() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut description = fs
            .open_regular_file(PHASE8_INIT_PATH)
            .expect("init regular file");
        let mut first = [0u8; 4];
        let mut rest = [0u8; PHASE8_INIT_ELF_LEN];

        assert_eq!(
            fs.read_regular_file_to_kernel(&mut description, &mut first),
            Ok(4)
        );
        assert_eq!(&first, b"\x7fELF");
        assert_eq!(description.offset(), 4);

        let copied = fs
            .read_regular_file_to_kernel(&mut description, &mut rest)
            .expect("read remaining init bytes");
        assert_eq!(copied, PHASE8_INIT_ELF_LEN - 4);
        assert_eq!(&rest[..copied], &PHASE8_INIT_BYTES[4..]);
        assert_eq!(
            fs.read_regular_file_to_kernel(&mut description, &mut rest),
            Ok(0)
        );
        assert_eq!(description.offset(), PHASE8_INIT_ELF_LEN);
    }

    #[test_case]
    fn zero_length_file_and_zero_length_reads_do_not_mutate_offsets() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut empty = fs.open_regular_file(PHASE8_EMPTY_PATH).expect("empty file");
        let mut banner = fs
            .open_regular_file(PHASE8_BANNER_PATH)
            .expect("banner regular file");
        let mut user_memory = [0x55u8; 32];
        let mut scratch = [0u8; 32];

        assert_eq!(
            read_file_case(
                fs,
                &mut empty,
                &mut user_memory,
                USER_BASE,
                32,
                &mut scratch
            ),
            Ok(0)
        );
        assert_eq!(empty.offset(), 0);

        assert_eq!(
            read_file_case(
                fs,
                &mut banner,
                &mut user_memory,
                USER_BASE,
                0,
                &mut scratch
            ),
            Ok(0)
        );
        assert_eq!(banner.offset(), 0);
        assert_eq!(user_memory, [0x55u8; 32]);
    }

    #[test_case]
    fn reads_are_short_only_at_fixture_eof() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut nested = fs
            .open_regular_file(PHASE8_NESTED_PATH)
            .expect("nested regular file");
        let mut user_memory = [0u8; 16];
        let mut scratch = [0u8; 16];

        assert_eq!(
            read_file_case(
                fs,
                &mut nested,
                &mut user_memory,
                USER_BASE,
                6,
                &mut scratch
            ),
            Ok(6)
        );
        assert_eq!(nested.offset(), 6);
        assert_eq!(&user_memory[..6], b"nested");
    }

    #[test_case]
    fn copy_faults_and_invalid_scratch_do_not_mutate_offsets_or_user_bytes() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut description = fs
            .open_regular_file(PHASE8_BANNER_PATH)
            .expect("banner regular file");
        let mut user_memory = [0x77u8; 16];
        let mut scratch = [0u8; 8];

        assert_eq!(
            read_file_case(
                fs,
                &mut description,
                &mut user_memory,
                USER_BASE + 128,
                8,
                &mut scratch,
            ),
            Err(PosixError::Fault)
        );
        assert_eq!(description.offset(), 0);
        assert_eq!(user_memory, [0x77u8; 16]);

        assert_eq!(
            read_file_case(
                fs,
                &mut description,
                &mut user_memory,
                USER_BASE,
                9,
                &mut scratch,
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(description.offset(), 0);
        assert_eq!(user_memory, [0x77u8; 16]);
    }

    #[test_case]
    fn malformed_open_file_description_reports_einval() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut user_memory = [0u8; 16];
        let mut scratch = [0u8; 16];
        let mut missing = ReadOnlyFileDescription {
            node_index: 99,
            offset: 0,
        };
        let mut beyond_eof = ReadOnlyFileDescription {
            node_index: PHASE8_BANNER_INDEX,
            offset: PHASE8_BANNER_BYTES.len() + 1,
        };

        assert_eq!(
            read_file_case(
                fs,
                &mut missing,
                &mut user_memory,
                USER_BASE,
                1,
                &mut scratch
            ),
            Err(PosixError::InvalidArgument)
        );
        assert_eq!(
            read_file_case(
                fs,
                &mut beyond_eof,
                &mut user_memory,
                USER_BASE,
                1,
                &mut scratch,
            ),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn descriptor_facing_read_reports_ebadf_before_copy_or_offset_mutation() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut descriptor_table = crate::posix::DescriptorTable::<4>::new_empty();
        descriptor_table
            .allocate_at(
                3,
                DescriptorEntry::new(
                    DescriptorAccess::ReadOnly,
                    DescriptorFlags::EMPTY,
                    DescriptorObject::new(DescriptorObjectKind::RegularFile, 0),
                ),
            )
            .expect("regular file descriptor");
        let mut file_descriptions = ReadOnlyFileDescriptions::<1>::new_empty();
        let mut banner = fs
            .open_regular_file(PHASE8_BANNER_PATH)
            .expect("banner regular file");
        file_descriptions
            .insert(0, banner)
            .expect("file description slot");
        let mut user_memory = [0x44u8; 32];
        let mut scratch = [0u8; 32];
        let mappings = writable_mapping(user_memory.len());

        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                2,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                8,
                &mut scratch,
            ),
            Err(PosixError::BadDescriptor)
        );
        banner = *file_descriptions
            .get_mut(0)
            .expect("retained file description");
        assert_eq!(banner.offset(), 0);
        assert_eq!(user_memory, [0x44u8; 32]);

        descriptor_table
            .close(3)
            .expect("close regular file descriptor");
        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                3,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                8,
                &mut scratch,
            ),
            Err(PosixError::BadDescriptor)
        );
        assert_eq!(
            file_descriptions
                .get_mut(0)
                .expect("retained file description")
                .offset(),
            0
        );
        assert_eq!(user_memory, [0x44u8; 32]);
    }

    #[test_case]
    fn descriptor_facing_read_rejects_directory_and_unsupported_objects() {
        let fs = phase8_readonly_initramfs_fixture();
        let mut descriptor_table = crate::posix::DescriptorTable::<5>::new_empty();
        descriptor_table
            .allocate_at(
                3,
                DescriptorEntry::new(
                    DescriptorAccess::ReadOnly,
                    DescriptorFlags::EMPTY,
                    DescriptorObject::new(DescriptorObjectKind::Directory, 0),
                ),
            )
            .expect("directory descriptor");
        descriptor_table
            .allocate_at(
                4,
                DescriptorEntry::new(
                    DescriptorAccess::ReadOnly,
                    DescriptorFlags::EMPTY,
                    DescriptorObject::new(DescriptorObjectKind::Device, 0),
                ),
            )
            .expect("device descriptor");
        let mut file_descriptions = ReadOnlyFileDescriptions::<1>::new_empty();
        let mut user_memory = [0u8; 32];
        let mut scratch = [0u8; 32];
        let mappings = writable_mapping(user_memory.len());

        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                3,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                8,
                &mut scratch,
            ),
            Err(PosixError::IsDirectory)
        );
        assert_eq!(
            fs.read_descriptor(
                &descriptor_table,
                &mut file_descriptions,
                4,
                &mappings,
                USER_BASE,
                &mut user_memory,
                USER_BASE,
                8,
                &mut scratch,
            ),
            Err(PosixError::NotSupported)
        );
    }
}
