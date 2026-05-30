//! Target-independent executable image validation for the first Phase 8 loader.
//!
//! This module only parses immutable fixture bytes and returns an image plan.
//! It does not allocate user frames, mutate page tables, create processes,
//! build user stacks, or launch lower-EL code.

use crate::{
    initramfs::{PHASE8_INIT_PATH, ReadOnlyInitramfs},
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
};

pub(crate) const PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY: &str =
    "phase8-program-loader-elf64-aarch64-v1";
pub(crate) const MAX_PROGRAM_IMAGE_BYTES: usize = 16 * 1024;
pub(crate) const MAX_LOAD_SEGMENTS: usize = 4;
pub(crate) const LOADER_PAGE_SIZE: u64 = 0x1000;

const ELF_HEADER_SIZE: usize = 64;
const ELF_PROGRAM_HEADER_SIZE: usize = 56;
const EI_CLASS_64: u8 = 2;
const EI_DATA_LSB: u8 = 1;
const EI_VERSION_CURRENT: u8 = 1;
const ET_EXEC: u16 = 2;
const EM_AARCH64: u16 = 183;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProgramLoaderError {
    Source(PosixError),
    EmptyImage,
    ImageTooLarge,
    BadElfIdentity,
    BadElfType,
    BadElfMachine,
    MalformedHeader,
    DynamicLinkingRequired,
    TooManyLoadSegments,
    BadSegmentRange,
    BadSegmentPermissions,
    SegmentAddressDenied,
    SegmentOverlap,
    FileRangeOverflow,
    BadEntry,
}

impl ProgramLoaderError {
    pub(crate) const fn posix_error(self) -> PosixError {
        match self {
            Self::Source(error) => error,
            Self::EmptyImage
            | Self::BadElfIdentity
            | Self::BadElfType
            | Self::BadElfMachine
            | Self::MalformedHeader
            | Self::FileRangeOverflow
            | Self::BadEntry => PosixError::NotExecutable,
            Self::DynamicLinkingRequired => PosixError::NotSupported,
            Self::ImageTooLarge | Self::TooManyLoadSegments => PosixError::NoMemory,
            Self::BadSegmentRange
            | Self::BadSegmentPermissions
            | Self::SegmentAddressDenied
            | Self::SegmentOverlap => PosixError::AccessDenied,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UserSegmentKind {
    UserText,
    UserData,
}

impl UserSegmentKind {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::UserText => "UserText",
            Self::UserData => "UserData",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlannedUserSegment {
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    virtual_start: u64,
    virtual_end: u64,
    rounded_start: u64,
    rounded_end: u64,
    file_offset: usize,
    file_size: usize,
    zero_fill_start: u64,
    zero_fill_end: u64,
}

impl PlannedUserSegment {
    pub(crate) const fn kind(self) -> UserSegmentKind {
        self.kind
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    pub(crate) const fn virtual_start(self) -> u64 {
        self.virtual_start
    }

    pub(crate) const fn virtual_end(self) -> u64 {
        self.virtual_end
    }

    pub(crate) const fn rounded_start(self) -> u64 {
        self.rounded_start
    }

    pub(crate) const fn rounded_end(self) -> u64 {
        self.rounded_end
    }

    pub(crate) const fn file_offset(self) -> usize {
        self.file_offset
    }

    pub(crate) const fn file_size(self) -> usize {
        self.file_size
    }

    pub(crate) const fn file_end(self) -> usize {
        self.file_offset + self.file_size
    }

    pub(crate) const fn zero_fill_start(self) -> u64 {
        self.zero_fill_start
    }

    pub(crate) const fn zero_fill_end(self) -> u64 {
        self.zero_fill_end
    }

    pub(crate) const fn zero_fill_len(self) -> u64 {
        self.zero_fill_end - self.zero_fill_start
    }

    const fn contains_entry(self, entry: u64) -> bool {
        self.kind as u8 == UserSegmentKind::UserText as u8
            && self.virtual_start <= entry
            && entry < self.virtual_end
    }

    #[cfg(any(
        test,
        talos_boot_scenario = "qemu_process_install_smoke",
        talos_boot_scenario = "qemu_initial_process_launch_smoke",
        talos_boot_scenario = "qemu_initial_user_stack_smoke"
    ))]
    pub(crate) const fn for_test_unchecked(
        kind: UserSegmentKind,
        permissions: UserMappingPermissions,
        virtual_start: u64,
        virtual_end: u64,
        rounded_start: u64,
        rounded_end: u64,
        file_offset: usize,
        file_size: usize,
        zero_fill_start: u64,
        zero_fill_end: u64,
    ) -> Self {
        Self {
            kind,
            permissions,
            virtual_start,
            virtual_end,
            rounded_start,
            rounded_end,
            file_offset,
            file_size,
            zero_fill_start,
            zero_fill_end,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProgramImagePlan {
    source_path: &'static [u8],
    fixture_identity: &'static str,
    source_len: usize,
    source_digest: u64,
    entry: u64,
    segment_count: usize,
    segments: [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
    memory_start: u64,
    memory_end: u64,
    memory_footprint: u64,
}

impl ProgramImagePlan {
    pub(crate) const fn source_path(self) -> &'static [u8] {
        self.source_path
    }

    pub(crate) const fn fixture_identity(self) -> &'static str {
        self.fixture_identity
    }

    pub(crate) const fn source_len(self) -> usize {
        self.source_len
    }

    pub(crate) const fn source_digest(self) -> u64 {
        self.source_digest
    }

    pub(crate) const fn entry(self) -> u64 {
        self.entry
    }

    pub(crate) const fn segment_count(self) -> usize {
        self.segment_count
    }

    pub(crate) const fn segment(self, index: usize) -> Option<PlannedUserSegment> {
        if index >= MAX_LOAD_SEGMENTS {
            None
        } else {
            self.segments[index]
        }
    }

    pub(crate) const fn memory_start(self) -> u64 {
        self.memory_start
    }

    pub(crate) const fn memory_end(self) -> u64 {
        self.memory_end
    }

    pub(crate) const fn memory_footprint(self) -> u64 {
        self.memory_footprint
    }

    #[cfg(any(
        test,
        talos_boot_scenario = "qemu_process_install_smoke",
        talos_boot_scenario = "qemu_initial_process_launch_smoke",
        talos_boot_scenario = "qemu_initial_user_stack_smoke"
    ))]
    pub(crate) const fn for_test_unchecked(
        source_path: &'static [u8],
        fixture_identity: &'static str,
        source_len: usize,
        source_digest: u64,
        entry: u64,
        segment_count: usize,
        segments: [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
        memory_start: u64,
        memory_end: u64,
        memory_footprint: u64,
    ) -> Self {
        Self {
            source_path,
            fixture_identity,
            source_len,
            source_digest,
            entry,
            segment_count,
            segments,
            memory_start,
            memory_end,
            memory_footprint,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ElfHeader {
    entry: u64,
    program_header_offset: usize,
    program_header_count: usize,
}

pub(crate) fn plan_phase8_init_image(
    fs: ReadOnlyInitramfs,
) -> Result<ProgramImagePlan, ProgramLoaderError> {
    let bytes = fs
        .regular_file_bytes(PHASE8_INIT_PATH)
        .map_err(ProgramLoaderError::Source)?;
    plan_elf64_aarch64_image(
        PHASE8_INIT_PATH,
        PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
        bytes,
    )
}

pub(crate) fn plan_elf64_aarch64_image(
    source_path: &'static [u8],
    fixture_identity: &'static str,
    bytes: &[u8],
) -> Result<ProgramImagePlan, ProgramLoaderError> {
    if bytes.is_empty() {
        return Err(ProgramLoaderError::EmptyImage);
    }
    if bytes.len() > MAX_PROGRAM_IMAGE_BYTES {
        return Err(ProgramLoaderError::ImageTooLarge);
    }

    let header = parse_elf_header(bytes)?;
    let mut segments = [None; MAX_LOAD_SEGMENTS];
    let mut segment_count = 0;
    let mut ph_index = 0;

    while ph_index < header.program_header_count {
        let offset = header
            .program_header_offset
            .checked_add(
                ph_index
                    .checked_mul(ELF_PROGRAM_HEADER_SIZE)
                    .ok_or(ProgramLoaderError::MalformedHeader)?,
            )
            .ok_or(ProgramLoaderError::MalformedHeader)?;
        match read_u32(bytes, offset)? {
            PT_LOAD => {
                let segment = parse_load_segment(bytes, offset)?;
                if segment_count == MAX_LOAD_SEGMENTS {
                    return Err(ProgramLoaderError::TooManyLoadSegments);
                }
                insert_ordered_segment(&mut segments, &mut segment_count, segment);
            }
            PT_INTERP | PT_DYNAMIC => return Err(ProgramLoaderError::DynamicLinkingRequired),
            _ => {}
        }
        ph_index += 1;
    }

    if segment_count == 0 {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    reject_rounded_overlaps(&segments, segment_count)?;
    if !entry_in_text(header.entry, &segments, segment_count) || header.entry & 0x3 != 0 {
        return Err(ProgramLoaderError::BadEntry);
    }

    let memory_start = segments[0]
        .expect("segment exists after nonzero count")
        .rounded_start();
    let mut memory_end = memory_start;
    let mut memory_footprint = 0;
    let mut index = 0;
    while index < segment_count {
        let segment = segments[index].expect("ordered segment exists");
        memory_end = core::cmp::max(memory_end, segment.rounded_end());
        memory_footprint += segment.rounded_end() - segment.rounded_start();
        index += 1;
    }

    Ok(ProgramImagePlan {
        source_path,
        fixture_identity,
        source_len: bytes.len(),
        source_digest: stable_digest(bytes),
        entry: header.entry,
        segment_count,
        segments,
        memory_start,
        memory_end,
        memory_footprint,
    })
}

pub(crate) fn stable_digest(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        index += 1;
    }
    hash
}

fn parse_elf_header(bytes: &[u8]) -> Result<ElfHeader, ProgramLoaderError> {
    if bytes.len() < ELF_HEADER_SIZE {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    if bytes[0] != 0x7f
        || bytes[1] != b'E'
        || bytes[2] != b'L'
        || bytes[3] != b'F'
        || bytes[4] != EI_CLASS_64
        || bytes[5] != EI_DATA_LSB
        || bytes[6] != EI_VERSION_CURRENT
    {
        return Err(ProgramLoaderError::BadElfIdentity);
    }
    if read_u16(bytes, 16)? != ET_EXEC {
        return Err(ProgramLoaderError::BadElfType);
    }
    if read_u16(bytes, 18)? != EM_AARCH64 {
        return Err(ProgramLoaderError::BadElfMachine);
    }
    if read_u32(bytes, 20)? != 1 {
        return Err(ProgramLoaderError::BadElfIdentity);
    }

    let entry = read_u64(bytes, 24)?;
    let phoff = checked_usize(read_u64(bytes, 32)?)?;
    let ehsize = read_u16(bytes, 52)? as usize;
    let phentsize = read_u16(bytes, 54)? as usize;
    let phnum = read_u16(bytes, 56)? as usize;
    if ehsize != ELF_HEADER_SIZE || phentsize != ELF_PROGRAM_HEADER_SIZE || phnum == 0 {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    let table_size = phnum
        .checked_mul(ELF_PROGRAM_HEADER_SIZE)
        .ok_or(ProgramLoaderError::MalformedHeader)?;
    let table_end = phoff
        .checked_add(table_size)
        .ok_or(ProgramLoaderError::MalformedHeader)?;
    if phoff < ELF_HEADER_SIZE || table_end > bytes.len() {
        return Err(ProgramLoaderError::MalformedHeader);
    }

    Ok(ElfHeader {
        entry,
        program_header_offset: phoff,
        program_header_count: phnum,
    })
}

fn parse_load_segment(
    bytes: &[u8],
    ph_offset: usize,
) -> Result<PlannedUserSegment, ProgramLoaderError> {
    let flags = read_u32(bytes, ph_offset + 4)?;
    let file_offset = checked_usize(read_u64(bytes, ph_offset + 8)?)?;
    let virtual_start = read_u64(bytes, ph_offset + 16)?;
    let file_size = checked_usize(read_u64(bytes, ph_offset + 32)?)?;
    let memory_size = checked_usize(read_u64(bytes, ph_offset + 40)?)?;
    let alignment = read_u64(bytes, ph_offset + 48)?;

    if memory_size == 0 || file_size > memory_size {
        return Err(ProgramLoaderError::BadSegmentRange);
    }
    if alignment != LOADER_PAGE_SIZE
        || virtual_start % LOADER_PAGE_SIZE != (file_offset as u64) % LOADER_PAGE_SIZE
    {
        return Err(ProgramLoaderError::BadSegmentRange);
    }
    let file_end = file_offset
        .checked_add(file_size)
        .ok_or(ProgramLoaderError::FileRangeOverflow)?;
    if file_end > bytes.len() {
        return Err(ProgramLoaderError::FileRangeOverflow);
    }
    let virtual_end = virtual_start
        .checked_add(memory_size as u64)
        .ok_or(ProgramLoaderError::SegmentAddressDenied)?;
    if virtual_start < USER_NULL_GUARD_END || virtual_end > USER_ADDRESS_SPACE_END {
        return Err(ProgramLoaderError::SegmentAddressDenied);
    }

    let (kind, permissions) = classify_segment(flags, file_size)?;
    let rounded_start = align_down(virtual_start, LOADER_PAGE_SIZE);
    let rounded_end = align_up(virtual_end, LOADER_PAGE_SIZE)?;
    if rounded_start < USER_NULL_GUARD_END || rounded_end > USER_ADDRESS_SPACE_END {
        return Err(ProgramLoaderError::SegmentAddressDenied);
    }

    Ok(PlannedUserSegment {
        kind,
        permissions,
        virtual_start,
        virtual_end,
        rounded_start,
        rounded_end,
        file_offset,
        file_size,
        zero_fill_start: virtual_start + file_size as u64,
        zero_fill_end: virtual_end,
    })
}

fn classify_segment(
    flags: u32,
    file_size: usize,
) -> Result<(UserSegmentKind, UserMappingPermissions), ProgramLoaderError> {
    match flags {
        flags if flags == PF_R | PF_X => {
            Ok((UserSegmentKind::UserText, UserMappingPermissions::USER_TEXT))
        }
        flags if flags == PF_R | PF_W => {
            Ok((UserSegmentKind::UserData, UserMappingPermissions::USER_DATA))
        }
        PF_R if file_size != 0 => Ok((UserSegmentKind::UserData, UserMappingPermissions::READ)),
        _ => Err(ProgramLoaderError::BadSegmentPermissions),
    }
}

fn insert_ordered_segment(
    segments: &mut [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
    segment_count: &mut usize,
    segment: PlannedUserSegment,
) {
    let mut insert_at = *segment_count;
    let mut index = 0;
    while index < *segment_count {
        let current = segments[index].expect("segment exists below count");
        if segment.virtual_start() < current.virtual_start() {
            insert_at = index;
            break;
        }
        index += 1;
    }

    let mut move_index = *segment_count;
    while move_index > insert_at {
        segments[move_index] = segments[move_index - 1];
        move_index -= 1;
    }
    segments[insert_at] = Some(segment);
    *segment_count += 1;
}

fn reject_rounded_overlaps(
    segments: &[Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
    segment_count: usize,
) -> Result<(), ProgramLoaderError> {
    let mut index = 1;
    while index < segment_count {
        let previous = segments[index - 1].expect("previous segment");
        let current = segments[index].expect("current segment");
        if current.rounded_start() < previous.rounded_end() {
            return Err(ProgramLoaderError::SegmentOverlap);
        }
        index += 1;
    }
    Ok(())
}

fn entry_in_text(
    entry: u64,
    segments: &[Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
    segment_count: usize,
) -> bool {
    let mut index = 0;
    while index < segment_count {
        if segments[index]
            .expect("segment exists")
            .contains_entry(entry)
        {
            return true;
        }
        index += 1;
    }
    false
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ProgramLoaderError> {
    let end = offset
        .checked_add(2)
        .ok_or(ProgramLoaderError::MalformedHeader)?;
    if end > bytes.len() {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    Ok(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, ProgramLoaderError> {
    let end = offset
        .checked_add(4)
        .ok_or(ProgramLoaderError::MalformedHeader)?;
    if end > bytes.len() {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    Ok(u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, ProgramLoaderError> {
    let end = offset
        .checked_add(8)
        .ok_or(ProgramLoaderError::MalformedHeader)?;
    if end > bytes.len() {
        return Err(ProgramLoaderError::MalformedHeader);
    }
    Ok(u64::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ]))
}

fn checked_usize(value: u64) -> Result<usize, ProgramLoaderError> {
    if value > usize::MAX as u64 {
        Err(ProgramLoaderError::MalformedHeader)
    } else {
        Ok(value as usize)
    }
}

const fn align_down(value: u64, alignment: u64) -> u64 {
    value & !(alignment - 1)
}

fn align_up(value: u64, alignment: u64) -> Result<u64, ProgramLoaderError> {
    value
        .checked_add(alignment - 1)
        .map(|aligned| align_down(aligned, alignment))
        .ok_or(ProgramLoaderError::SegmentAddressDenied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initramfs::{
        PHASE8_INIT_BYTES, PHASE8_INIT_ELF_LEN, phase8_readonly_initramfs_fixture,
    };

    const PHDR0: usize = ELF_HEADER_SIZE;
    const PHDR1: usize = ELF_HEADER_SIZE + ELF_PROGRAM_HEADER_SIZE;
    const PHASE8_INIT_DIGEST: u64 = 0x3892_eed2_2390_0c65;

    fn fixture_bytes() -> [u8; PHASE8_INIT_ELF_LEN] {
        let mut bytes = [0u8; PHASE8_INIT_ELF_LEN];
        bytes.copy_from_slice(PHASE8_INIT_BYTES);
        bytes
    }

    fn plan_bytes(bytes: &[u8]) -> Result<ProgramImagePlan, ProgramLoaderError> {
        plan_elf64_aarch64_image(
            PHASE8_INIT_PATH,
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            bytes,
        )
    }

    fn write_u16(bytes: &mut [u8], offset: usize, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn write_u64(bytes: &mut [u8], offset: usize, value: u64) {
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }

    #[test_case]
    fn phase8_init_fixture_produces_image_plan_only() {
        let plan = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
            .expect("program loader fixture plan");

        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_eq!(
            plan.fixture_identity(),
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        );
        assert_eq!(plan.source_len(), PHASE8_INIT_BYTES.len());
        assert_eq!(plan.source_digest(), PHASE8_INIT_DIGEST);
        assert_eq!(stable_digest(PHASE8_INIT_BYTES), PHASE8_INIT_DIGEST);
        assert_eq!(plan.entry(), 0x0000_0000_0001_0100);
        assert_eq!(plan.segment_count(), 2);
        assert_eq!(plan.memory_start(), 0x0000_0000_0001_0000);
        assert_eq!(plan.memory_end(), 0x0000_0000_0002_2000);
        assert_eq!(plan.memory_footprint(), 0x3000);

        let text = plan.segment(0).expect("text segment");
        assert_eq!(text.kind(), UserSegmentKind::UserText);
        assert_eq!(text.kind().name(), "UserText");
        assert_eq!(text.permissions(), UserMappingPermissions::USER_TEXT);
        assert_eq!(text.file_offset(), 0x100);
        assert_eq!(text.file_size(), 4);
        assert_eq!(text.file_end(), 0x104);
        assert_eq!(text.virtual_start(), 0x0000_0000_0001_0100);
        assert_eq!(text.virtual_end(), 0x0000_0000_0001_0104);
        assert_eq!(text.zero_fill_len(), 0);

        let data = plan.segment(1).expect("data segment");
        assert_eq!(data.kind(), UserSegmentKind::UserData);
        assert_eq!(data.kind().name(), "UserData");
        assert_eq!(data.permissions(), UserMappingPermissions::USER_DATA);
        assert_eq!(data.file_offset(), 0x200);
        assert_eq!(data.file_size(), 4);
        assert_eq!(data.file_end(), 0x204);
        assert_eq!(data.zero_fill_start(), 0x0000_0000_0002_0204);
        assert_eq!(data.zero_fill_end(), 0x0000_0000_0002_1204);
        assert_eq!(data.zero_fill_len(), 0x1000);
    }

    #[test_case]
    fn rejects_bad_magic_before_plan() {
        let mut bytes = fixture_bytes();
        bytes[0] = 0;

        assert_eq!(plan_bytes(&bytes), Err(ProgramLoaderError::BadElfIdentity));
        assert_eq!(
            ProgramLoaderError::BadElfIdentity.posix_error(),
            PosixError::NotExecutable
        );
    }

    #[test_case]
    fn rejects_unsupported_type_and_machine() {
        let mut dyn_type = fixture_bytes();
        write_u16(&mut dyn_type, 16, 3);
        assert_eq!(plan_bytes(&dyn_type), Err(ProgramLoaderError::BadElfType));

        let mut wrong_machine = fixture_bytes();
        write_u16(&mut wrong_machine, 18, 62);
        assert_eq!(
            plan_bytes(&wrong_machine),
            Err(ProgramLoaderError::BadElfMachine)
        );
    }

    #[test_case]
    fn rejects_dynamic_interpreter_program_header() {
        let mut bytes = fixture_bytes();
        write_u32(&mut bytes, PHDR1, PT_INTERP);

        assert_eq!(
            plan_bytes(&bytes),
            Err(ProgramLoaderError::DynamicLinkingRequired)
        );
        assert_eq!(
            ProgramLoaderError::DynamicLinkingRequired.posix_error(),
            PosixError::NotSupported
        );
    }

    #[test_case]
    fn rejects_malformed_program_header_range() {
        let mut bytes = fixture_bytes();
        write_u64(&mut bytes, 32, (PHASE8_INIT_ELF_LEN - 8) as u64);

        assert_eq!(plan_bytes(&bytes), Err(ProgramLoaderError::MalformedHeader));
    }

    #[test_case]
    fn rejects_writable_executable_segment() {
        let mut bytes = fixture_bytes();
        write_u32(&mut bytes, PHDR0 + 4, PF_R | PF_W | PF_X);

        assert_eq!(
            plan_bytes(&bytes),
            Err(ProgramLoaderError::BadSegmentPermissions)
        );
        assert_eq!(
            ProgramLoaderError::BadSegmentPermissions.posix_error(),
            PosixError::AccessDenied
        );
    }

    #[test_case]
    fn rejects_out_of_user_range_segment() {
        let mut bytes = fixture_bytes();
        write_u64(&mut bytes, PHDR0 + 16, USER_ADDRESS_SPACE_END + 0x100);

        assert_eq!(
            plan_bytes(&bytes),
            Err(ProgramLoaderError::SegmentAddressDenied)
        );
    }

    #[test_case]
    fn rejects_rounded_segment_overlap() {
        let mut bytes = fixture_bytes();
        write_u64(&mut bytes, PHDR1 + 16, 0x0000_0000_0001_0200);

        assert_eq!(plan_bytes(&bytes), Err(ProgramLoaderError::SegmentOverlap));
    }

    #[test_case]
    fn rejects_bad_entry_outside_executable_text() {
        let mut bytes = fixture_bytes();
        write_u64(&mut bytes, 24, 0x0000_0000_0002_0200);

        assert_eq!(plan_bytes(&bytes), Err(ProgramLoaderError::BadEntry));
    }

    #[test_case]
    fn rejects_file_range_overflow_without_partial_plan() {
        let mut bytes = fixture_bytes();
        write_u64(&mut bytes, PHDR1 + 32, 8);

        assert_eq!(
            plan_bytes(&bytes),
            Err(ProgramLoaderError::FileRangeOverflow)
        );
    }
}
