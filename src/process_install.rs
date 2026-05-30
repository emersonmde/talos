//! Target-independent metadata plan for installing a validated program image.
//!
//! This module turns a validated ProgramImagePlan into ordered page-install
//! metadata only. It does not allocate frames, copy bytes into physical
//! memory, create page tables, create processes, build lower-EL frames, or
//! make the image runnable.

use crate::{
    posix::{PosixError, USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END, UserMappingPermissions},
    program_loader::{
        LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, PlannedUserSegment, ProgramImagePlan, UserSegmentKind,
    },
};

pub(crate) const PROCESS_INSTALL_BOUNDARY_IDENTITY: &str = "phase8-process-install-plan-v1";
pub(crate) const MAX_PROCESS_INSTALL_PAGES: usize = 16;
pub(crate) const MAX_PROCESS_INSTALL_FOOTPRINT: u64 =
    LOADER_PAGE_SIZE * MAX_PROCESS_INSTALL_PAGES as u64;
pub(crate) const MAX_ZERO_RANGES_PER_PAGE: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProcessInstallAction {
    AllocateCopyZeroMap,
}

impl ProcessInstallAction {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::AllocateCopyZeroMap => "allocate,copy,zero,map",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PageByteRange {
    offset: u64,
    len: u64,
}

impl PageByteRange {
    pub(crate) const fn offset(self) -> u64 {
        self.offset
    }

    pub(crate) const fn len(self) -> u64 {
        self.len
    }

    pub(crate) const fn end(self) -> u64 {
        self.offset + self.len
    }

    #[cfg(any(
        test,
        talos_boot_scenario = "qemu_process_address_space_smoke",
        talos_boot_scenario = "qemu_process_page_table_materialization_smoke"
    ))]
    pub(crate) const fn for_test_unchecked(offset: u64, len: u64) -> Self {
        Self { offset, len }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessImagePageInstallRecord {
    index: usize,
    segment_index: usize,
    virtual_start: u64,
    virtual_end: u64,
    kind: UserSegmentKind,
    permissions: UserMappingPermissions,
    copy_page_offset: u64,
    copy_file_offset: usize,
    copy_len: u64,
    zero_ranges: [Option<PageByteRange>; MAX_ZERO_RANGES_PER_PAGE],
    zero_range_count: usize,
    zero_len: u64,
    action: ProcessInstallAction,
}

impl ProcessImagePageInstallRecord {
    pub(crate) const fn index(self) -> usize {
        self.index
    }

    pub(crate) const fn segment_index(self) -> usize {
        self.segment_index
    }

    pub(crate) const fn virtual_start(self) -> u64 {
        self.virtual_start
    }

    pub(crate) const fn virtual_end(self) -> u64 {
        self.virtual_end
    }

    pub(crate) const fn kind(self) -> UserSegmentKind {
        self.kind
    }

    pub(crate) const fn permissions(self) -> UserMappingPermissions {
        self.permissions
    }

    pub(crate) const fn copy_page_offset(self) -> u64 {
        self.copy_page_offset
    }

    pub(crate) const fn copy_file_offset(self) -> usize {
        self.copy_file_offset
    }

    pub(crate) const fn copy_len(self) -> u64 {
        self.copy_len
    }

    pub(crate) const fn zero_range_count(self) -> usize {
        self.zero_range_count
    }

    pub(crate) const fn zero_range(self, index: usize) -> Option<PageByteRange> {
        if index >= MAX_ZERO_RANGES_PER_PAGE {
            None
        } else {
            self.zero_ranges[index]
        }
    }

    pub(crate) const fn zero_len(self) -> u64 {
        self.zero_len
    }

    pub(crate) const fn action(self) -> ProcessInstallAction {
        self.action
    }

    pub(crate) const fn permission_flags(self) -> &'static str {
        match self.permissions {
            UserMappingPermissions::USER_TEXT => "R-X",
            UserMappingPermissions::USER_DATA => "RW-",
            UserMappingPermissions::READ => "R--",
            _ => "---",
        }
    }

    #[cfg(any(
        test,
        talos_boot_scenario = "qemu_process_address_space_smoke",
        talos_boot_scenario = "qemu_process_page_table_materialization_smoke",
        talos_boot_scenario = "qemu_initial_process_launch_smoke"
    ))]
    pub(crate) const fn for_test_unchecked(
        index: usize,
        segment_index: usize,
        virtual_start: u64,
        virtual_end: u64,
        kind: UserSegmentKind,
        permissions: UserMappingPermissions,
        copy_page_offset: u64,
        copy_file_offset: usize,
        copy_len: u64,
        zero_ranges: [Option<PageByteRange>; MAX_ZERO_RANGES_PER_PAGE],
        zero_range_count: usize,
        zero_len: u64,
        action: ProcessInstallAction,
    ) -> Self {
        Self {
            index,
            segment_index,
            virtual_start,
            virtual_end,
            kind,
            permissions,
            copy_page_offset,
            copy_file_offset,
            copy_len,
            zero_ranges,
            zero_range_count,
            zero_len,
            action,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessImageInstallPlan {
    fixture_identity: &'static str,
    install_boundary_identity: &'static str,
    source_path: &'static [u8],
    source_digest: u64,
    entry: u64,
    memory_footprint: u64,
    page_count: usize,
    pages: [Option<ProcessImagePageInstallRecord>; MAX_PROCESS_INSTALL_PAGES],
    side_effects: ProcessInstallSideEffects,
    lower_el_launch_blocked: bool,
}

impl ProcessImageInstallPlan {
    pub(crate) const fn fixture_identity(self) -> &'static str {
        self.fixture_identity
    }

    pub(crate) const fn install_boundary_identity(self) -> &'static str {
        self.install_boundary_identity
    }

    pub(crate) const fn source_path(self) -> &'static [u8] {
        self.source_path
    }

    pub(crate) const fn source_digest(self) -> u64 {
        self.source_digest
    }

    pub(crate) const fn entry(self) -> u64 {
        self.entry
    }

    pub(crate) const fn memory_footprint(self) -> u64 {
        self.memory_footprint
    }

    pub(crate) const fn page_count(self) -> usize {
        self.page_count
    }

    pub(crate) const fn page(self, index: usize) -> Option<ProcessImagePageInstallRecord> {
        if index >= MAX_PROCESS_INSTALL_PAGES {
            None
        } else {
            self.pages[index]
        }
    }

    pub(crate) const fn side_effects(self) -> ProcessInstallSideEffects {
        self.side_effects
    }

    pub(crate) const fn lower_el_launch_blocked(self) -> bool {
        self.lower_el_launch_blocked
    }

    #[cfg(any(
        test,
        talos_boot_scenario = "qemu_process_address_space_smoke",
        talos_boot_scenario = "qemu_process_page_table_materialization_smoke",
        talos_boot_scenario = "qemu_initial_process_launch_smoke"
    ))]
    pub(crate) const fn for_test_unchecked(
        fixture_identity: &'static str,
        install_boundary_identity: &'static str,
        source_path: &'static [u8],
        source_digest: u64,
        entry: u64,
        memory_footprint: u64,
        page_count: usize,
        pages: [Option<ProcessImagePageInstallRecord>; MAX_PROCESS_INSTALL_PAGES],
        side_effects: ProcessInstallSideEffects,
        lower_el_launch_blocked: bool,
    ) -> Self {
        Self {
            fixture_identity,
            install_boundary_identity,
            source_path,
            source_digest,
            entry,
            memory_footprint,
            page_count,
            pages,
            side_effects,
            lower_el_launch_blocked,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessInstallSideEffects {
    frames_allocated: usize,
    mappings_installed: usize,
    process_created: bool,
    descriptors_mutated: bool,
    lower_el_frame: bool,
    runnable: bool,
}

impl ProcessInstallSideEffects {
    pub(crate) const NONE: Self = Self {
        frames_allocated: 0,
        mappings_installed: 0,
        process_created: false,
        descriptors_mutated: false,
        lower_el_frame: false,
        runnable: false,
    };

    pub(crate) const fn frames_allocated(self) -> usize {
        self.frames_allocated
    }

    pub(crate) const fn mappings_installed(self) -> usize {
        self.mappings_installed
    }

    pub(crate) const fn process_created(self) -> bool {
        self.process_created
    }

    pub(crate) const fn descriptors_mutated(self) -> bool {
        self.descriptors_mutated
    }

    pub(crate) const fn lower_el_frame(self) -> bool {
        self.lower_el_frame
    }

    pub(crate) const fn runnable(self) -> bool {
        self.runnable
    }
}

pub(crate) fn plan_process_image_install(
    image: ProgramImagePlan,
) -> Result<ProcessImageInstallPlan, PosixError> {
    let segment_count = image.segment_count();
    if segment_count == 0 || segment_count > MAX_LOAD_SEGMENTS {
        return Err(PosixError::InvalidArgument);
    }

    let mut pages = [None; MAX_PROCESS_INSTALL_PAGES];
    let mut page_count = 0;
    let mut derived_footprint = 0u64;
    let mut previous_rounded_end = 0u64;
    let mut entry_in_text = false;
    let mut segment_index = 0;

    while segment_index < segment_count {
        let segment = image
            .segment(segment_index)
            .ok_or(PosixError::InvalidArgument)?;
        validate_segment(image, segment)?;

        if segment_index != 0 && segment.rounded_start() < previous_rounded_end {
            return Err(PosixError::AccessDenied);
        }
        previous_rounded_end = segment.rounded_end();
        derived_footprint = derived_footprint
            .checked_add(segment.rounded_end() - segment.rounded_start())
            .ok_or(PosixError::InvalidArgument)?;
        if derived_footprint > MAX_PROCESS_INSTALL_FOOTPRINT {
            return Err(PosixError::NoMemory);
        }

        if segment.kind() == UserSegmentKind::UserText
            && segment.virtual_start() <= image.entry()
            && image.entry() < segment.virtual_end()
        {
            entry_in_text = true;
        }

        let mut page_start = segment.rounded_start();
        while page_start < segment.rounded_end() {
            if page_count == MAX_PROCESS_INSTALL_PAGES {
                return Err(PosixError::NoMemory);
            }
            pages[page_count] = Some(derive_page_record(
                page_count,
                segment_index,
                segment,
                page_start,
            )?);
            page_count += 1;
            page_start = page_start
                .checked_add(LOADER_PAGE_SIZE)
                .ok_or(PosixError::InvalidArgument)?;
        }

        segment_index += 1;
    }

    if !entry_in_text {
        return Err(PosixError::NotExecutable);
    }
    if image.memory_footprint() != derived_footprint {
        return Err(PosixError::InvalidArgument);
    }

    Ok(ProcessImageInstallPlan {
        fixture_identity: image.fixture_identity(),
        install_boundary_identity: PROCESS_INSTALL_BOUNDARY_IDENTITY,
        source_path: image.source_path(),
        source_digest: image.source_digest(),
        entry: image.entry(),
        memory_footprint: derived_footprint,
        page_count,
        pages,
        side_effects: ProcessInstallSideEffects::NONE,
        lower_el_launch_blocked: true,
    })
}

fn validate_segment(
    image: ProgramImagePlan,
    segment: PlannedUserSegment,
) -> Result<(), PosixError> {
    if segment.virtual_start() >= segment.virtual_end()
        || segment.rounded_start() > segment.virtual_start()
        || segment.virtual_end() > segment.rounded_end()
        || segment.rounded_start() >= segment.rounded_end()
        || segment.rounded_start() % LOADER_PAGE_SIZE != 0
        || segment.rounded_end() % LOADER_PAGE_SIZE != 0
    {
        return Err(PosixError::InvalidArgument);
    }

    if segment.rounded_start() < USER_NULL_GUARD_END
        || segment.rounded_end() > USER_ADDRESS_SPACE_END
    {
        return Err(PosixError::AccessDenied);
    }

    let file_end = segment
        .file_offset()
        .checked_add(segment.file_size())
        .ok_or(PosixError::InvalidArgument)?;
    if file_end > image.source_len() {
        return Err(PosixError::InvalidArgument);
    }

    let file_virtual_end = segment
        .virtual_start()
        .checked_add(segment.file_size() as u64)
        .ok_or(PosixError::InvalidArgument)?;
    if file_virtual_end > segment.virtual_end()
        || segment.zero_fill_start() != file_virtual_end
        || segment.zero_fill_start() > segment.zero_fill_end()
        || segment.zero_fill_end() != segment.virtual_end()
    {
        return Err(PosixError::InvalidArgument);
    }

    match segment.kind() {
        UserSegmentKind::UserText if segment.permissions() == UserMappingPermissions::USER_TEXT => {
            Ok(())
        }
        UserSegmentKind::UserData if segment.permissions() == UserMappingPermissions::USER_DATA => {
            Ok(())
        }
        _ => Err(PosixError::AccessDenied),
    }
}

fn derive_page_record(
    index: usize,
    segment_index: usize,
    segment: PlannedUserSegment,
    page_start: u64,
) -> Result<ProcessImagePageInstallRecord, PosixError> {
    let page_end = page_start
        .checked_add(LOADER_PAGE_SIZE)
        .ok_or(PosixError::InvalidArgument)?;
    let file_virtual_end = segment
        .virtual_start()
        .checked_add(segment.file_size() as u64)
        .ok_or(PosixError::InvalidArgument)?;
    let copy_start = core::cmp::max(page_start, segment.virtual_start());
    let copy_end = core::cmp::min(page_end, file_virtual_end);
    let copy_len = copy_end.saturating_sub(copy_start);
    let (copy_page_offset, copy_file_offset) = if copy_len == 0 {
        (
            0,
            segment
                .file_offset()
                .checked_add(segment.file_size())
                .ok_or(PosixError::InvalidArgument)?,
        )
    } else {
        (
            copy_start - page_start,
            segment
                .file_offset()
                .checked_add((copy_start - segment.virtual_start()) as usize)
                .ok_or(PosixError::InvalidArgument)?,
        )
    };

    let mut zero_ranges = [None; MAX_ZERO_RANGES_PER_PAGE];
    let mut zero_range_count = 0;
    let mut zero_len = 0u64;
    push_zero_range(
        &mut zero_ranges,
        &mut zero_range_count,
        &mut zero_len,
        page_start,
        page_end,
        page_start,
        segment.virtual_start(),
    )?;
    push_zero_range(
        &mut zero_ranges,
        &mut zero_range_count,
        &mut zero_len,
        page_start,
        page_end,
        file_virtual_end,
        page_end,
    )?;

    Ok(ProcessImagePageInstallRecord {
        index,
        segment_index,
        virtual_start: page_start,
        virtual_end: page_end,
        kind: segment.kind(),
        permissions: segment.permissions(),
        copy_page_offset,
        copy_file_offset,
        copy_len,
        zero_ranges,
        zero_range_count,
        zero_len,
        action: ProcessInstallAction::AllocateCopyZeroMap,
    })
}

fn push_zero_range(
    ranges: &mut [Option<PageByteRange>; MAX_ZERO_RANGES_PER_PAGE],
    range_count: &mut usize,
    zero_len: &mut u64,
    page_start: u64,
    page_end: u64,
    start: u64,
    end: u64,
) -> Result<(), PosixError> {
    let clipped_start = core::cmp::max(page_start, start);
    let clipped_end = core::cmp::min(page_end, end);
    if clipped_start >= clipped_end {
        return Ok(());
    }

    let offset = clipped_start - page_start;
    let len = clipped_end - clipped_start;
    if *range_count != 0 {
        let previous = ranges[*range_count - 1].expect("previous zero range");
        if previous.end() == offset {
            ranges[*range_count - 1] = Some(PageByteRange {
                offset: previous.offset(),
                len: previous.len() + len,
            });
            *zero_len += len;
            return Ok(());
        }
    }

    if *range_count == MAX_ZERO_RANGES_PER_PAGE {
        return Err(PosixError::InvalidArgument);
    }
    ranges[*range_count] = Some(PageByteRange { offset, len });
    *range_count += 1;
    *zero_len += len;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initramfs::{PHASE8_INIT_BYTES, PHASE8_INIT_PATH, phase8_readonly_initramfs_fixture},
        program_loader::{PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, plan_phase8_init_image},
    };

    fn install_fixture() -> ProcessImageInstallPlan {
        let image = plan_phase8_init_image(phase8_readonly_initramfs_fixture())
            .expect("program image plan");
        plan_process_image_install(image).expect("process install plan")
    }

    fn segment(
        kind: UserSegmentKind,
        permissions: UserMappingPermissions,
        virtual_start: u64,
        memory_size: u64,
        file_offset: usize,
        file_size: usize,
    ) -> PlannedUserSegment {
        let virtual_end = virtual_start + memory_size;
        PlannedUserSegment::for_test_unchecked(
            kind,
            permissions,
            virtual_start,
            virtual_end,
            virtual_start & !(LOADER_PAGE_SIZE - 1),
            (virtual_end + LOADER_PAGE_SIZE - 1) & !(LOADER_PAGE_SIZE - 1),
            file_offset,
            file_size,
            virtual_start + file_size as u64,
            virtual_end,
        )
    }

    fn unchecked_plan(
        entry: u64,
        segment_count: usize,
        segments: [Option<PlannedUserSegment>; MAX_LOAD_SEGMENTS],
        memory_footprint: u64,
    ) -> ProgramImagePlan {
        ProgramImagePlan::for_test_unchecked(
            PHASE8_INIT_PATH,
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY,
            PHASE8_INIT_BYTES.len(),
            0x3892_eed2_2390_0c65,
            entry,
            segment_count,
            segments,
            0x0000_0000_0001_0000,
            0x0000_0000_0002_2000,
            memory_footprint,
        )
    }

    #[test_case]
    fn derives_metadata_only_install_plan_from_fixture() {
        let plan = install_fixture();

        assert_eq!(
            plan.fixture_identity(),
            PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY
        );
        assert_eq!(
            plan.install_boundary_identity(),
            PROCESS_INSTALL_BOUNDARY_IDENTITY
        );
        assert_eq!(plan.source_path(), PHASE8_INIT_PATH);
        assert_eq!(plan.source_digest(), 0x3892_eed2_2390_0c65);
        assert_eq!(plan.entry(), 0x0000_0000_0001_0100);
        assert_eq!(plan.memory_footprint(), 0x3000);
        assert_eq!(plan.page_count(), 3);
        assert!(plan.lower_el_launch_blocked());
        assert_eq!(plan.side_effects(), ProcessInstallSideEffects::NONE);
        assert_eq!(plan.side_effects().frames_allocated(), 0);
        assert_eq!(plan.side_effects().mappings_installed(), 0);
        assert!(!plan.side_effects().process_created());
        assert!(!plan.side_effects().descriptors_mutated());
        assert!(!plan.side_effects().lower_el_frame());
        assert!(!plan.side_effects().runnable());

        let text = plan.page(0).expect("text page");
        assert_eq!(text.index(), 0);
        assert_eq!(text.segment_index(), 0);
        assert_eq!(text.kind(), UserSegmentKind::UserText);
        assert_eq!(text.permissions(), UserMappingPermissions::USER_TEXT);
        assert_eq!(text.permission_flags(), "R-X");
        assert_eq!(text.virtual_start(), 0x0000_0000_0001_0000);
        assert_eq!(text.virtual_end(), 0x0000_0000_0001_1000);
        assert_eq!(text.copy_page_offset(), 0x100);
        assert_eq!(text.copy_file_offset(), 0x100);
        assert_eq!(text.copy_len(), 4);
        assert_eq!(text.zero_range_count(), 2);
        assert_eq!(
            text.zero_range(0),
            Some(PageByteRange {
                offset: 0,
                len: 0x100
            })
        );
        assert_eq!(
            text.zero_range(1),
            Some(PageByteRange {
                offset: 0x104,
                len: 0xefc,
            })
        );
        assert_eq!(text.zero_len(), 0xffc);
        assert_eq!(text.action().name(), "allocate,copy,zero,map");

        let data0 = plan.page(1).expect("first data page");
        assert_eq!(data0.index(), 1);
        assert_eq!(data0.segment_index(), 1);
        assert_eq!(data0.kind(), UserSegmentKind::UserData);
        assert_eq!(data0.permission_flags(), "RW-");
        assert_eq!(data0.copy_page_offset(), 0x200);
        assert_eq!(data0.copy_file_offset(), 0x200);
        assert_eq!(data0.copy_len(), 4);
        assert_eq!(data0.zero_range_count(), 2);
        assert_eq!(
            data0.zero_range(0),
            Some(PageByteRange {
                offset: 0,
                len: 0x200
            })
        );
        assert_eq!(
            data0.zero_range(1),
            Some(PageByteRange {
                offset: 0x204,
                len: 0xdfc,
            })
        );

        let data1 = plan.page(2).expect("second data page");
        assert_eq!(data1.copy_page_offset(), 0);
        assert_eq!(data1.copy_file_offset(), 0x204);
        assert_eq!(data1.copy_len(), 0);
        assert_eq!(data1.zero_range_count(), 1);
        assert_eq!(
            data1.zero_range(0),
            Some(PageByteRange {
                offset: 0,
                len: LOADER_PAGE_SIZE,
            })
        );
        assert_eq!(plan.page(3), None);
    }

    #[test_case]
    fn rejects_missing_segment_slot_without_partial_install() {
        let image = unchecked_plan(
            0x0000_0000_0001_0100,
            1,
            [None, None, None, None],
            LOADER_PAGE_SIZE,
        );

        assert_eq!(
            plan_process_image_install(image),
            Err(PosixError::InvalidArgument)
        );
    }

    #[test_case]
    fn rejects_overlapping_pages_without_partial_install() {
        let text = segment(
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0x0000_0000_0001_0100,
            4,
            0x100,
            4,
        );
        let data = segment(
            UserSegmentKind::UserData,
            UserMappingPermissions::USER_DATA,
            0x0000_0000_0001_0200,
            4,
            0x200,
            4,
        );
        let image = unchecked_plan(
            0x0000_0000_0001_0100,
            2,
            [Some(text), Some(data), None, None],
            LOADER_PAGE_SIZE * 2,
        );

        assert_eq!(
            plan_process_image_install(image),
            Err(PosixError::AccessDenied)
        );
    }

    #[test_case]
    fn rejects_permission_widening_without_partial_install() {
        let text = segment(
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_DATA,
            0x0000_0000_0001_0100,
            4,
            0x100,
            4,
        );
        let image = unchecked_plan(
            0x0000_0000_0001_0100,
            1,
            [Some(text), None, None, None],
            LOADER_PAGE_SIZE,
        );

        assert_eq!(
            plan_process_image_install(image),
            Err(PosixError::AccessDenied)
        );
    }

    #[test_case]
    fn rejects_bad_entry_without_partial_install() {
        let data = segment(
            UserSegmentKind::UserData,
            UserMappingPermissions::USER_DATA,
            0x0000_0000_0002_0200,
            4,
            0x200,
            4,
        );
        let image = unchecked_plan(
            0x0000_0000_0002_0200,
            1,
            [Some(data), None, None, None],
            LOADER_PAGE_SIZE,
        );

        assert_eq!(
            plan_process_image_install(image),
            Err(PosixError::NotExecutable)
        );
    }

    #[test_case]
    fn rejects_budget_overflow_without_partial_install() {
        let text = segment(
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0x0000_0000_0001_0000,
            MAX_PROCESS_INSTALL_FOOTPRINT + LOADER_PAGE_SIZE,
            0,
            4,
        );
        let image = unchecked_plan(
            0x0000_0000_0001_0000,
            1,
            [Some(text), None, None, None],
            MAX_PROCESS_INSTALL_FOOTPRINT + LOADER_PAGE_SIZE,
        );

        assert_eq!(plan_process_image_install(image), Err(PosixError::NoMemory));
    }

    #[test_case]
    fn rejects_source_range_overflow_without_partial_install() {
        let text = segment(
            UserSegmentKind::UserText,
            UserMappingPermissions::USER_TEXT,
            0x0000_0000_0001_0100,
            4,
            PHASE8_INIT_BYTES.len(),
            4,
        );
        let image = unchecked_plan(
            0x0000_0000_0001_0100,
            1,
            [Some(text), None, None, None],
            LOADER_PAGE_SIZE,
        );

        assert_eq!(
            plan_process_image_install(image),
            Err(PosixError::InvalidArgument)
        );
    }
}
