use super::page_frames::EarlyPageFrameSpan;

pub(super) fn reserve_after(
    candidate_start: u64,
    bank_start: u64,
    bank_end: u64,
    reserved_start: u64,
    reserved_end: u64,
) -> Option<u64> {
    if reserved_end <= reserved_start {
        return Some(candidate_start);
    }

    if !ranges_intersect(bank_start, bank_end, reserved_start, reserved_end) {
        return Some(candidate_start);
    }

    Some(core::cmp::max(
        candidate_start,
        core::cmp::min(reserved_end, bank_end),
    ))
}

pub(super) fn contains_address(start: u64, end: u64, address: u64) -> bool {
    start <= address && address < end
}

pub(super) fn ranges_intersect(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    a_start < b_end && b_start < a_end
}

pub(super) fn align_up(value: u64, alignment: u64) -> Option<u64> {
    debug_assert!(alignment.is_power_of_two());
    value
        .checked_add(alignment - 1)
        .map(|value| value & !(alignment - 1))
}

pub(super) fn align_down(value: u64, alignment: u64) -> u64 {
    debug_assert!(alignment.is_power_of_two());
    value & !(alignment - 1)
}

pub(super) fn is_aligned(value: u64, alignment: u64) -> bool {
    debug_assert!(alignment.is_power_of_two());
    value & (alignment - 1) == 0
}

#[allow(dead_code)]
pub(super) fn page_frame_span(start: u64, end: u64, page_size: u64) -> Option<EarlyPageFrameSpan> {
    if page_size == 0
        || start >= end
        || !is_aligned(start, page_size)
        || !is_aligned(end, page_size)
    {
        return None;
    }

    let size = end.checked_sub(start)?;
    let page_count = size.checked_div(page_size)?;
    if page_count == 0 || page_count.checked_mul(page_size)? != size {
        return None;
    }

    Some(EarlyPageFrameSpan {
        start,
        end,
        page_size,
        page_count,
    })
}
