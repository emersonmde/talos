//! SSH service readiness diagnostic shape.
//!
//! This module models fixed fail-closed service lifecycle diagnostics only. It
//! does not adopt an SSH dependency, open a listener, process transport,
//! authenticate users, attach a shell, inspect hardware, or expose secrets.

use crate::ssh_key_readiness::{SshKeyReadinessLabel, SshKeyReadinessReport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshServiceLifecycleState {
    Disabled,
    PrerequisitesMissing,
    ShapeModeled,
}

impl SshServiceLifecycleState {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::PrerequisitesMissing => "prerequisites-missing",
            Self::ShapeModeled => "shape-modeled",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshServiceReadinessLabel {
    DependencyUnaccepted,
    CryptoBackendUnaccepted,
    TransportUnaccepted,
    AuthenticationUnimplemented,
    SessionUnimplemented,
    ExposureDisabled,
    PrerequisitesMissing,
    ShapeModeled,
    NotReady,
}

impl SshServiceReadinessLabel {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::DependencyUnaccepted => "sshservicediag-dependency-unaccepted",
            Self::CryptoBackendUnaccepted => "sshservicediag-crypto-backend-unaccepted",
            Self::TransportUnaccepted => "sshservicediag-transport-unaccepted",
            Self::AuthenticationUnimplemented => "sshservicediag-authentication-unimplemented",
            Self::SessionUnimplemented => "sshservicediag-session-unimplemented",
            Self::ExposureDisabled => "sshservicediag-exposure-disabled",
            Self::PrerequisitesMissing => "sshservicediag-prerequisites-missing",
            Self::ShapeModeled => "sshservicediag-shape-modeled",
            Self::NotReady => "sshservicediag-not-ready",
        }
    }
}

pub(crate) const MAX_SSH_SERVICE_READINESS_LABELS: usize = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshServiceReadinessReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_SERVICE_READINESS_LABELS],
    label_count: usize,
    lifecycle: SshServiceLifecycleState,
}

impl SshServiceReadinessReport {
    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn primary_label(self) -> SshServiceReadinessLabel {
        SshServiceReadinessLabel::NotReady
    }

    pub(crate) const fn lifecycle(self) -> SshServiceLifecycleState {
        self.lifecycle
    }

    pub(crate) const fn listener_count(self) -> usize {
        0
    }

    pub(crate) const fn transport_enabled(self) -> bool {
        false
    }

    pub(crate) const fn accepted_connection_count(self) -> usize {
        0
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn shell_attached(self) -> bool {
        false
    }

    pub(crate) const fn reachability_accepted(self) -> bool {
        false
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        false
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }
}

pub(crate) fn classify_ssh_service_readiness(
    key_report: SshKeyReadinessReport,
) -> SshServiceReadinessReport {
    let exposure_disabled = key_report
        .labels()
        .contains(&SshKeyReadinessLabel::ExposureDisabled);
    let prerequisites_missing = key_report.labels().iter().any(|label| {
        !matches!(
            label,
            SshKeyReadinessLabel::ExposureDisabled | SshKeyReadinessLabel::NotReady
        )
    });
    let lifecycle = if exposure_disabled {
        SshServiceLifecycleState::Disabled
    } else if prerequisites_missing {
        SshServiceLifecycleState::PrerequisitesMissing
    } else {
        SshServiceLifecycleState::ShapeModeled
    };

    let mut report = SshServiceReadinessReport {
        labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SERVICE_READINESS_LABELS],
        label_count: 0,
        lifecycle,
    };

    if exposure_disabled {
        report.push(SshServiceReadinessLabel::ExposureDisabled);
    }
    if prerequisites_missing {
        report.push(SshServiceReadinessLabel::PrerequisitesMissing);
    }
    if lifecycle == SshServiceLifecycleState::ShapeModeled {
        report.push(SshServiceReadinessLabel::ShapeModeled);
    }

    report.push(SshServiceReadinessLabel::DependencyUnaccepted);
    report.push(SshServiceReadinessLabel::CryptoBackendUnaccepted);
    report.push(SshServiceReadinessLabel::TransportUnaccepted);
    report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
    report.push(SshServiceReadinessLabel::SessionUnimplemented);
    report.push(SshServiceReadinessLabel::NotReady);
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entropy::{self, EntropyDiagnosticSnapshot, OperatorSeedObservation},
        ssh_key_readiness::{self, SshKeyReadinessSnapshot},
    };

    fn label_names(
        report: &SshServiceReadinessReport,
    ) -> [&'static str; MAX_SSH_SERVICE_READINESS_LABELS] {
        let mut labels = [""; MAX_SSH_SERVICE_READINESS_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    #[test_case]
    fn exposure_disabled_state_fails_closed_without_service_caps() {
        let key_report = ssh_key_readiness::classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default(),
        );
        let report = classify_ssh_service_readiness(key_report);

        assert_eq!(report.lifecycle(), SshServiceLifecycleState::Disabled);
        assert_eq!(report.primary_label(), SshServiceReadinessLabel::NotReady);
        assert_eq!(
            label_names(&report),
            [
                "sshservicediag-exposure-disabled",
                "sshservicediag-prerequisites-missing",
                "sshservicediag-dependency-unaccepted",
                "sshservicediag-crypto-backend-unaccepted",
                "sshservicediag-transport-unaccepted",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
                "",
            ]
        );
        assert_eq!(report.listener_count(), 0);
        assert_eq!(report.accepted_connection_count(), 0);
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.transport_enabled());
        assert!(!report.authentication_success());
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn exposure_enabled_with_missing_metadata_stays_prerequisites_missing() {
        let key_report = ssh_key_readiness::classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_exposure_enabled(),
        );
        let report = classify_ssh_service_readiness(key_report);

        assert_eq!(
            report.lifecycle(),
            SshServiceLifecycleState::PrerequisitesMissing
        );
        assert!(
            !report
                .labels()
                .contains(&SshServiceReadinessLabel::ExposureDisabled)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::PrerequisitesMissing)
        );
        assert!(
            !report
                .labels()
                .contains(&SshServiceReadinessLabel::ShapeModeled)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn prerequisite_satisfied_shape_remains_not_ready_without_transport_or_session() {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty()
                .with_operator_seed(OperatorSeedObservation::new(32))
                .with_csprng_ready(),
        );
        let key_report = ssh_key_readiness::classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_persistence_metadata()
                .with_exposure_enabled()
                .with_entropy_report(entropy),
        );
        let report = classify_ssh_service_readiness(key_report);

        assert_eq!(report.lifecycle(), SshServiceLifecycleState::ShapeModeled);
        assert_eq!(
            label_names(&report),
            [
                "sshservicediag-shape-modeled",
                "sshservicediag-dependency-unaccepted",
                "sshservicediag-crypto-backend-unaccepted",
                "sshservicediag-transport-unaccepted",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
                "",
                "",
            ]
        );
        assert_eq!(report.listener_count(), 0);
        assert_eq!(report.accepted_connection_count(), 0);
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.transport_enabled());
        assert!(!report.authentication_success());
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }
}
