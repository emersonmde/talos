//! Entropy diagnostic classification.
//!
//! This module deliberately classifies caller-supplied observations. It does
//! not sample hardware, generate random bytes, derive keys, persist seed
//! material, print seed bytes, or assert SSH readiness.

use crate::{
    initramfs::{ReadOnlyInitramfs, VfsNodeKind},
    posix::PosixError,
};

pub(crate) const OPERATOR_SEED_PATH: &[u8] = b"/etc/talos/operator-seed.bin";
pub(crate) const OPERATOR_SEED_MIN_SUFFICIENT_BYTES: usize = 32;
pub(crate) const OPERATOR_SEED_MAX_DIAGNOSTIC_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EntropyObservation {
    value: u64,
    sequence: u64,
}

impl EntropyObservation {
    pub(crate) const fn new(value: u64, sequence: u64) -> Self {
        Self { value, sequence }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OperatorSeedObservation {
    byte_len: usize,
}

impl OperatorSeedObservation {
    pub(crate) const fn new(byte_len: usize) -> Self {
        Self { byte_len }
    }

    pub(crate) const fn byte_len(self) -> usize {
        self.byte_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OperatorSeedMaterialState {
    Missing,
    Invalid,
    Insufficient,
    Sufficient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OperatorSeedMaterialMetadata {
    state: OperatorSeedMaterialState,
    byte_len: Option<usize>,
}

impl OperatorSeedMaterialMetadata {
    pub(crate) const fn missing() -> Self {
        Self {
            state: OperatorSeedMaterialState::Missing,
            byte_len: None,
        }
    }

    pub(crate) const fn invalid(byte_len: Option<usize>) -> Self {
        Self {
            state: OperatorSeedMaterialState::Invalid,
            byte_len,
        }
    }

    pub(crate) const fn insufficient(byte_len: usize) -> Self {
        Self {
            state: OperatorSeedMaterialState::Insufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn sufficient(byte_len: usize) -> Self {
        Self {
            state: OperatorSeedMaterialState::Sufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn state(self) -> OperatorSeedMaterialState {
        self.state
    }

    pub(crate) const fn byte_len(self) -> Option<usize> {
        self.byte_len
    }

    pub(crate) const fn entropy_observation(self) -> Option<OperatorSeedObservation> {
        match (self.state, self.byte_len) {
            (OperatorSeedMaterialState::Insufficient, Some(byte_len))
            | (OperatorSeedMaterialState::Sufficient, Some(byte_len)) => {
                Some(OperatorSeedObservation::new(byte_len))
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EntropyDiagnosticSnapshot {
    timer: Option<EntropyObservation>,
    scheduler_event: Option<EntropyObservation>,
    console_timing: Option<EntropyObservation>,
    operator_seed: Option<OperatorSeedObservation>,
    deterministic_control: bool,
    hardware_rng_observed: bool,
    csprng_ready: bool,
}

impl EntropyDiagnosticSnapshot {
    pub(crate) const fn empty() -> Self {
        Self {
            timer: None,
            scheduler_event: None,
            console_timing: None,
            operator_seed: None,
            deterministic_control: false,
            hardware_rng_observed: false,
            csprng_ready: false,
        }
    }

    pub(crate) const fn with_timer(mut self, observation: EntropyObservation) -> Self {
        self.timer = Some(observation);
        self
    }

    pub(crate) const fn with_scheduler_event(mut self, observation: EntropyObservation) -> Self {
        self.scheduler_event = Some(observation);
        self
    }

    pub(crate) const fn with_console_timing(mut self, observation: EntropyObservation) -> Self {
        self.console_timing = Some(observation);
        self
    }

    pub(crate) const fn with_operator_seed(mut self, observation: OperatorSeedObservation) -> Self {
        self.operator_seed = Some(observation);
        self
    }

    pub(crate) const fn as_deterministic_control(mut self) -> Self {
        self.deterministic_control = true;
        self
    }

    pub(crate) const fn with_hardware_rng_observed(mut self) -> Self {
        self.hardware_rng_observed = true;
        self
    }

    pub(crate) const fn with_csprng_ready(mut self) -> Self {
        self.csprng_ready = true;
        self
    }

    const fn has_timer(self) -> bool {
        self.timer.is_some()
    }

    const fn has_scheduler_event(self) -> bool {
        self.scheduler_event.is_some()
    }

    const fn has_console_timing(self) -> bool {
        self.console_timing.is_some()
    }

    const fn has_operator_seed(self) -> bool {
        self.operator_seed.is_some()
    }

    const fn has_any_input(self) -> bool {
        self.has_timer()
            || self.has_scheduler_event()
            || self.has_console_timing()
            || self.has_operator_seed()
            || self.hardware_rng_observed
    }

    const fn local_observation_count(self) -> usize {
        self.has_timer() as usize
            + self.has_scheduler_event() as usize
            + self.has_console_timing() as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EntropyDiagnosticLabel {
    FailClosedNoInput,
    DeterministicControl,
    UntrustedTimerOnly,
    UntrustedLocalMix,
    OperatorSeedRequired,
    HardwareRngUnaccepted,
}

impl EntropyDiagnosticLabel {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::FailClosedNoInput => "entropydiag-fail-closed-no-input",
            Self::DeterministicControl => "entropydiag-deterministic-control",
            Self::UntrustedTimerOnly => "entropydiag-untrusted-timer-only",
            Self::UntrustedLocalMix => "entropydiag-untrusted-local-mix",
            Self::OperatorSeedRequired => "entropydiag-operator-seed-required",
            Self::HardwareRngUnaccepted => "entropydiag-hardware-rng-unaccepted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EntropyDiagnosticReport {
    input_label: EntropyDiagnosticLabel,
    hardware_rng_label: EntropyDiagnosticLabel,
    operator_seed_required: bool,
    cryptographic_strength: bool,
    ssh_ready: bool,
}

impl EntropyDiagnosticReport {
    pub(crate) const fn input_label(self) -> EntropyDiagnosticLabel {
        self.input_label
    }

    pub(crate) const fn hardware_rng_label(self) -> EntropyDiagnosticLabel {
        self.hardware_rng_label
    }

    pub(crate) const fn operator_seed_label(self) -> Option<EntropyDiagnosticLabel> {
        if self.operator_seed_required {
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        } else {
            None
        }
    }

    pub(crate) const fn cryptographic_strength(self) -> bool {
        self.cryptographic_strength
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }
}

pub(crate) fn classify_operator_seed_material(
    initramfs: ReadOnlyInitramfs,
) -> OperatorSeedMaterialMetadata {
    let handle = match initramfs.lookup_default(OPERATOR_SEED_PATH) {
        Ok(handle) => handle,
        Err(PosixError::NoEntry) => return OperatorSeedMaterialMetadata::missing(),
        Err(_) => return OperatorSeedMaterialMetadata::invalid(None),
    };

    let metadata = handle.metadata();
    if metadata.kind() != VfsNodeKind::RegularFile {
        return OperatorSeedMaterialMetadata::invalid(Some(metadata.len()));
    }

    let byte_len = metadata.len();
    if byte_len == 0 || byte_len > OPERATOR_SEED_MAX_DIAGNOSTIC_BYTES {
        OperatorSeedMaterialMetadata::invalid(Some(byte_len))
    } else if byte_len < OPERATOR_SEED_MIN_SUFFICIENT_BYTES {
        OperatorSeedMaterialMetadata::insufficient(byte_len)
    } else {
        OperatorSeedMaterialMetadata::sufficient(byte_len)
    }
}

pub(crate) fn entropy_snapshot_with_operator_seed_material(
    initramfs: ReadOnlyInitramfs,
) -> EntropyDiagnosticSnapshot {
    let metadata = classify_operator_seed_material(initramfs);
    match metadata.entropy_observation() {
        Some(observation) => EntropyDiagnosticSnapshot::empty().with_operator_seed(observation),
        None => EntropyDiagnosticSnapshot::empty(),
    }
}

pub(crate) const fn classify_entropy_snapshot(
    snapshot: EntropyDiagnosticSnapshot,
) -> EntropyDiagnosticReport {
    let input_label = if snapshot.deterministic_control {
        EntropyDiagnosticLabel::DeterministicControl
    } else if !snapshot.has_any_input() {
        EntropyDiagnosticLabel::FailClosedNoInput
    } else if snapshot.hardware_rng_observed
        && snapshot.local_observation_count() == 0
        && !snapshot.has_operator_seed()
    {
        EntropyDiagnosticLabel::HardwareRngUnaccepted
    } else if snapshot.has_timer()
        && snapshot.local_observation_count() == 1
        && !snapshot.has_operator_seed()
        && !snapshot.hardware_rng_observed
    {
        EntropyDiagnosticLabel::UntrustedTimerOnly
    } else {
        EntropyDiagnosticLabel::UntrustedLocalMix
    };

    EntropyDiagnosticReport {
        input_label,
        hardware_rng_label: EntropyDiagnosticLabel::HardwareRngUnaccepted,
        operator_seed_required: !snapshot.has_operator_seed(),
        cryptographic_strength: snapshot.has_operator_seed() && snapshot.csprng_ready,
        ssh_ready: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initramfs::{DirectoryEntry, InitramfsNode, phase8_readonly_initramfs_fixture};

    #[test_case]
    fn no_input_fails_closed_without_crypto_or_ssh_readiness() {
        let report = classify_entropy_snapshot(EntropyDiagnosticSnapshot::empty());

        assert_eq!(
            report.input_label(),
            EntropyDiagnosticLabel::FailClosedNoInput
        );
        assert_eq!(
            report.operator_seed_label(),
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        );
        assert_eq!(
            report.hardware_rng_label(),
            EntropyDiagnosticLabel::HardwareRngUnaccepted
        );
        assert_eq!(
            report.hardware_rng_label(),
            EntropyDiagnosticLabel::HardwareRngUnaccepted
        );
        assert!(!report.cryptographic_strength());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn fixed_timer_only_input_is_untrusted_timer_only() {
        let snapshot =
            EntropyDiagnosticSnapshot::empty().with_timer(EntropyObservation::new(1234, 1));

        let first = classify_entropy_snapshot(snapshot);
        let second = classify_entropy_snapshot(snapshot);

        assert_eq!(first, second);
        assert_eq!(
            first.input_label(),
            EntropyDiagnosticLabel::UntrustedTimerOnly
        );
        assert_eq!(
            first.operator_seed_label(),
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        );
    }

    #[test_case]
    fn fixed_local_event_mix_is_untrusted_local_mix() {
        let snapshot = EntropyDiagnosticSnapshot::empty()
            .with_timer(EntropyObservation::new(100, 1))
            .with_scheduler_event(EntropyObservation::new(7, 2))
            .with_console_timing(EntropyObservation::new(14, 3));

        let report = classify_entropy_snapshot(snapshot);

        assert_eq!(
            report.input_label(),
            EntropyDiagnosticLabel::UntrustedLocalMix
        );
        assert_eq!(
            report.operator_seed_label(),
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        );
        assert!(!report.cryptographic_strength());
    }

    #[test_case]
    fn fixed_deterministic_test_seed_uses_deterministic_control_label() {
        let snapshot = EntropyDiagnosticSnapshot::empty()
            .with_operator_seed(OperatorSeedObservation::new(32))
            .as_deterministic_control();

        let report = classify_entropy_snapshot(snapshot);

        assert_eq!(
            report.input_label(),
            EntropyDiagnosticLabel::DeterministicControl
        );
        assert_eq!(report.operator_seed_label(), None);
        assert!(!report.cryptographic_strength());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn ready_csprng_metadata_sets_crypto_strength_without_ssh_readiness() {
        let snapshot = EntropyDiagnosticSnapshot::empty()
            .with_operator_seed(OperatorSeedObservation::new(32))
            .with_csprng_ready();

        let report = classify_entropy_snapshot(snapshot);

        assert_eq!(report.operator_seed_label(), None);
        assert!(report.cryptographic_strength());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn hardware_rng_observation_remains_unaccepted() {
        let report = classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty().with_hardware_rng_observed(),
        );

        assert_eq!(
            report.hardware_rng_label(),
            EntropyDiagnosticLabel::HardwareRngUnaccepted
        );
        assert_eq!(
            report.operator_seed_label(),
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        );
        assert!(!report.cryptographic_strength());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn default_initramfs_reports_operator_seed_missing_without_reading_secret_bytes() {
        let metadata = classify_operator_seed_material(phase8_readonly_initramfs_fixture());
        let report = classify_entropy_snapshot(entropy_snapshot_with_operator_seed_material(
            phase8_readonly_initramfs_fixture(),
        ));

        assert_eq!(metadata, OperatorSeedMaterialMetadata::missing());
        assert_eq!(
            report.operator_seed_label(),
            Some(EntropyDiagnosticLabel::OperatorSeedRequired)
        );
        assert!(!report.cryptographic_strength());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn vfs_operator_seed_metadata_distinguishes_insufficient_and_sufficient_lengths() {
        let insufficient = classify_operator_seed_material(insufficient_seed_initramfs());
        let sufficient = classify_operator_seed_material(sufficient_seed_initramfs());

        assert_eq!(
            insufficient,
            OperatorSeedMaterialMetadata::insufficient(OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1)
        );
        assert_eq!(
            sufficient,
            OperatorSeedMaterialMetadata::sufficient(OPERATOR_SEED_MIN_SUFFICIENT_BYTES)
        );
        assert_eq!(
            insufficient.byte_len(),
            Some(OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1)
        );
        assert_eq!(
            sufficient.byte_len(),
            Some(OPERATOR_SEED_MIN_SUFFICIENT_BYTES)
        );
        assert_eq!(
            insufficient
                .entropy_observation()
                .map(|observation| observation.byte_len()),
            Some(OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1)
        );
        assert_eq!(
            classify_entropy_snapshot(entropy_snapshot_with_operator_seed_material(
                sufficient_seed_initramfs()
            ))
            .operator_seed_label(),
            None
        );
    }

    #[test_case]
    fn invalid_operator_seed_paths_do_not_clear_required_seed_diagnostic() {
        let directory = classify_operator_seed_material(directory_seed_initramfs());
        let oversized = classify_operator_seed_material(oversized_seed_initramfs());

        assert_eq!(directory, OperatorSeedMaterialMetadata::invalid(Some(0)));
        assert_eq!(
            oversized,
            OperatorSeedMaterialMetadata::invalid(Some(OPERATOR_SEED_MAX_DIAGNOSTIC_BYTES + 1))
        );
        assert_eq!(directory.entropy_observation(), None);
        assert_eq!(oversized.entropy_observation(), None);
    }

    const ROOT_INDEX: usize = 0;
    const ETC_INDEX: usize = 1;
    const TALOS_INDEX: usize = 2;
    const SEED_INDEX: usize = 3;

    static ROOT_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"etc", ETC_INDEX)];
    static ETC_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"talos", TALOS_INDEX)];
    static TALOS_ENTRIES: [DirectoryEntry; 1] =
        [DirectoryEntry::new(b"operator-seed.bin", SEED_INDEX)];
    static EMPTY_ENTRIES: [DirectoryEntry; 0] = [];
    static SUFFICIENT_BYTES: [u8; OPERATOR_SEED_MIN_SUFFICIENT_BYTES] =
        [0; OPERATOR_SEED_MIN_SUFFICIENT_BYTES];
    static INSUFFICIENT_BYTES: [u8; OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1] =
        [0; OPERATOR_SEED_MIN_SUFFICIENT_BYTES - 1];
    static OVERSIZED_BYTES: [u8; OPERATOR_SEED_MAX_DIAGNOSTIC_BYTES + 1] =
        [0; OPERATOR_SEED_MAX_DIAGNOSTIC_BYTES + 1];

    static INSUFFICIENT_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &INSUFFICIENT_BYTES),
    ];
    static SUFFICIENT_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &SUFFICIENT_BYTES),
    ];
    static DIRECTORY_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SEED_INDEX, &EMPTY_ENTRIES),
    ];
    static OVERSIZED_SEED_NODES: [InitramfsNode; 4] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::regular_file(SEED_INDEX, &OVERSIZED_BYTES),
    ];

    const fn insufficient_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_SEED_NODES, ROOT_INDEX)
    }

    const fn sufficient_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&SUFFICIENT_SEED_NODES, ROOT_INDEX)
    }

    const fn directory_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&DIRECTORY_SEED_NODES, ROOT_INDEX)
    }

    const fn oversized_seed_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_SEED_NODES, ROOT_INDEX)
    }
}
