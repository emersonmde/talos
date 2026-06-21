//! Entropy diagnostic classification.
//!
//! This module deliberately classifies caller-supplied observations. It does
//! not sample hardware, generate random bytes, derive keys, persist seed
//! material, or assert SSH readiness.

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
    fingerprint: u64,
}

impl OperatorSeedObservation {
    pub(crate) const fn new(byte_len: usize, fingerprint: u64) -> Self {
        Self {
            byte_len,
            fingerprint,
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
        cryptographic_strength: false,
        ssh_ready: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            report.input_label(),
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
            .with_operator_seed(OperatorSeedObservation::new(32, 0xfeed_cafe))
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
}
