//! SSH key-management readiness classification.
//!
//! This module classifies caller-supplied prerequisite metadata only. It does
//! not generate host keys, parse authorized keys, read or persist secrets,
//! sample hardware, import crypto/SSH dependencies, or accept SSH service
//! readiness.

use crate::entropy::{
    self, EntropyDiagnosticReport, EntropyDiagnosticSnapshot, OperatorSeedMaterialMetadata,
    OperatorSeedMaterialState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HostKeyState {
    Missing,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizedKeyState {
    Missing,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SeedMaterialState {
    Missing,
    Insufficient,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PersistenceState {
    Unavailable,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExposureState {
    Disabled,
    ExplicitlyEnabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshKeyReadinessSnapshot {
    host_key: HostKeyState,
    authorized_key: AuthorizedKeyState,
    entropy: EntropyDiagnosticReport,
    seed_material: SeedMaterialState,
    persistence: PersistenceState,
    exposure: ExposureState,
}

impl SshKeyReadinessSnapshot {
    pub(crate) fn fail_closed_default() -> Self {
        Self {
            host_key: HostKeyState::Missing,
            authorized_key: AuthorizedKeyState::Missing,
            entropy: entropy::classify_entropy_snapshot(EntropyDiagnosticSnapshot::empty()),
            seed_material: SeedMaterialState::Missing,
            persistence: PersistenceState::Unavailable,
            exposure: ExposureState::Disabled,
        }
    }

    pub(crate) const fn with_host_key_metadata(mut self) -> Self {
        self.host_key = HostKeyState::MetadataPresent;
        self
    }

    pub(crate) const fn with_authorized_key_metadata(mut self) -> Self {
        self.authorized_key = AuthorizedKeyState::MetadataPresent;
        self
    }

    pub(crate) const fn with_entropy_report(mut self, entropy: EntropyDiagnosticReport) -> Self {
        self.entropy = entropy;
        self
    }

    pub(crate) const fn with_insufficient_seed_material(mut self) -> Self {
        self.seed_material = SeedMaterialState::Insufficient;
        self
    }

    pub(crate) const fn with_seed_material_metadata(mut self) -> Self {
        self.seed_material = SeedMaterialState::MetadataPresent;
        self
    }

    pub(crate) const fn with_operator_seed_material(
        mut self,
        metadata: OperatorSeedMaterialMetadata,
    ) -> Self {
        self.seed_material = match metadata.state() {
            OperatorSeedMaterialState::Missing => SeedMaterialState::Missing,
            OperatorSeedMaterialState::Invalid | OperatorSeedMaterialState::Insufficient => {
                SeedMaterialState::Insufficient
            }
            OperatorSeedMaterialState::Sufficient => SeedMaterialState::MetadataPresent,
        };
        self
    }

    pub(crate) const fn with_persistence_metadata(mut self) -> Self {
        self.persistence = PersistenceState::MetadataPresent;
        self
    }

    pub(crate) const fn with_exposure_enabled(mut self) -> Self {
        self.exposure = ExposureState::ExplicitlyEnabled;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshKeyReadinessLabel {
    MissingHostKey,
    MissingAuthorizedKey,
    EntropyUnready,
    SeedMaterialMissing,
    SeedMaterialInsufficient,
    PersistenceUnavailable,
    ExposureDisabled,
    NotReady,
}

impl SshKeyReadinessLabel {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::MissingHostKey => "sshkeydiag-missing-host-key",
            Self::MissingAuthorizedKey => "sshkeydiag-missing-authorized-key",
            Self::EntropyUnready => "sshkeydiag-entropy-unready",
            Self::SeedMaterialMissing => "sshkeydiag-seed-material-missing",
            Self::SeedMaterialInsufficient => "sshkeydiag-seed-material-insufficient",
            Self::PersistenceUnavailable => "sshkeydiag-persistence-unavailable",
            Self::ExposureDisabled => "sshkeydiag-exposure-disabled",
            Self::NotReady => "sshkeydiag-not-ready",
        }
    }
}

pub(crate) const MAX_SSH_KEY_READINESS_LABELS: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshKeyReadinessReport {
    labels: [SshKeyReadinessLabel; MAX_SSH_KEY_READINESS_LABELS],
    label_count: usize,
    ssh_ready: bool,
}

impl SshKeyReadinessReport {
    pub(crate) fn labels(&self) -> &[SshKeyReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn primary_label(self) -> SshKeyReadinessLabel {
        SshKeyReadinessLabel::NotReady
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        self.ssh_ready
    }

    fn push(&mut self, label: SshKeyReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }
}

pub(crate) fn classify_ssh_key_readiness(
    snapshot: SshKeyReadinessSnapshot,
) -> SshKeyReadinessReport {
    let mut report = SshKeyReadinessReport {
        labels: [SshKeyReadinessLabel::NotReady; MAX_SSH_KEY_READINESS_LABELS],
        label_count: 0,
        ssh_ready: false,
    };

    if snapshot.host_key == HostKeyState::Missing {
        report.push(SshKeyReadinessLabel::MissingHostKey);
    }
    if snapshot.authorized_key == AuthorizedKeyState::Missing {
        report.push(SshKeyReadinessLabel::MissingAuthorizedKey);
    }
    if !snapshot.entropy.cryptographic_strength() || !snapshot.entropy.ssh_ready() {
        report.push(SshKeyReadinessLabel::EntropyUnready);
    }
    match snapshot.seed_material {
        SeedMaterialState::Missing => report.push(SshKeyReadinessLabel::SeedMaterialMissing),
        SeedMaterialState::Insufficient => {
            report.push(SshKeyReadinessLabel::SeedMaterialInsufficient);
        }
        SeedMaterialState::MetadataPresent => {}
    }
    if snapshot.persistence == PersistenceState::Unavailable {
        report.push(SshKeyReadinessLabel::PersistenceUnavailable);
    }
    if snapshot.exposure == ExposureState::Disabled {
        report.push(SshKeyReadinessLabel::ExposureDisabled);
    }

    report.push(SshKeyReadinessLabel::NotReady);
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::{EntropyObservation, OperatorSeedObservation};

    fn label_names(report: &SshKeyReadinessReport) -> [&'static str; MAX_SSH_KEY_READINESS_LABELS] {
        let mut labels = [""; MAX_SSH_KEY_READINESS_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    #[test_case]
    fn all_missing_default_reports_every_fail_closed_label() {
        let report = classify_ssh_key_readiness(SshKeyReadinessSnapshot::fail_closed_default());

        assert!(!report.ssh_ready());
        assert_eq!(report.primary_label(), SshKeyReadinessLabel::NotReady);
        assert_eq!(
            label_names(&report),
            [
                "sshkeydiag-missing-host-key",
                "sshkeydiag-missing-authorized-key",
                "sshkeydiag-entropy-unready",
                "sshkeydiag-seed-material-missing",
                "sshkeydiag-persistence-unavailable",
                "sshkeydiag-exposure-disabled",
                "sshkeydiag-not-ready",
                "",
            ]
        );
    }

    #[test_case]
    fn deterministic_entropy_control_keeps_ssh_key_readiness_false() {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty()
                .with_operator_seed(OperatorSeedObservation::new(32))
                .as_deterministic_control(),
        );
        let report = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_persistence_metadata()
                .with_exposure_enabled()
                .with_entropy_report(entropy),
        );

        assert!(!report.ssh_ready());
        assert_eq!(
            label_names(&report),
            [
                "sshkeydiag-entropy-unready",
                "sshkeydiag-not-ready",
                "",
                "",
                "",
                "",
                "",
                "",
            ]
        );
    }

    #[test_case]
    fn untrusted_local_entropy_without_seed_material_is_not_ready() {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty()
                .with_timer(EntropyObservation::new(1, 1))
                .with_scheduler_event(EntropyObservation::new(2, 2)),
        );
        let report = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_persistence_metadata()
                .with_exposure_enabled()
                .with_entropy_report(entropy),
        );

        assert!(!report.ssh_ready());
        assert_eq!(
            label_names(&report),
            [
                "sshkeydiag-entropy-unready",
                "sshkeydiag-seed-material-missing",
                "sshkeydiag-not-ready",
                "",
                "",
                "",
                "",
                "",
            ]
        );
    }

    #[test_case]
    fn missing_and_insufficient_seed_material_are_distinguished() {
        let missing = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata(),
        );
        let insufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_insufficient_seed_material(),
        );

        assert!(
            missing
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialMissing)
        );
        assert!(
            !missing
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialInsufficient)
        );
        assert!(
            insufficient
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialInsufficient)
        );
        assert!(
            !insufficient
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialMissing)
        );
    }

    #[test_case]
    fn persistence_and_exposure_independently_keep_readiness_false() {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty().with_operator_seed(OperatorSeedObservation::new(32)),
        );
        let persistence_unavailable = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_exposure_enabled()
                .with_entropy_report(entropy),
        );
        let exposure_disabled = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_persistence_metadata()
                .with_entropy_report(entropy),
        );

        assert!(!persistence_unavailable.ssh_ready());
        assert!(
            persistence_unavailable
                .labels()
                .contains(&SshKeyReadinessLabel::PersistenceUnavailable)
        );
        assert!(!exposure_disabled.ssh_ready());
        assert!(
            exposure_disabled
                .labels()
                .contains(&SshKeyReadinessLabel::ExposureDisabled)
        );
    }

    #[test_case]
    fn key_metadata_negative_control_still_requires_entropy_persistence_and_exposure() {
        let report = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata(),
        );

        assert!(
            !report
                .labels()
                .contains(&SshKeyReadinessLabel::MissingHostKey)
        );
        assert!(
            !report
                .labels()
                .contains(&SshKeyReadinessLabel::MissingAuthorizedKey)
        );
        assert!(
            report
                .labels()
                .contains(&SshKeyReadinessLabel::EntropyUnready)
        );
        assert!(
            report
                .labels()
                .contains(&SshKeyReadinessLabel::PersistenceUnavailable)
        );
        assert!(
            report
                .labels()
                .contains(&SshKeyReadinessLabel::ExposureDisabled)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn operator_seed_vfs_metadata_maps_to_missing_insufficient_and_present_states() {
        let missing = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_operator_seed_material(OperatorSeedMaterialMetadata::missing()),
        );
        let invalid = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_operator_seed_material(OperatorSeedMaterialMetadata::invalid(Some(0))),
        );
        let insufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_operator_seed_material(OperatorSeedMaterialMetadata::insufficient(31)),
        );
        let sufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_operator_seed_material(OperatorSeedMaterialMetadata::sufficient(32)),
        );

        assert!(
            missing
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialMissing)
        );
        assert!(
            invalid
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialInsufficient)
        );
        assert!(
            insufficient
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialInsufficient)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialMissing)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::SeedMaterialInsufficient)
        );
        assert!(!sufficient.ssh_ready());
    }
}
