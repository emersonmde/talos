//! SSH service readiness diagnostic shape.
//!
//! This module models fixed fail-closed service lifecycle diagnostics only. It
//! does not adopt an SSH dependency, perform SSH crypto, authenticate users,
//! attach a shell, inspect hardware, or expose secrets.

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
    LocalListenerModeled,
    LocalTransportModeled,
    IdentificationBannerModeled,
    LocalIdentificationLiteral,
    RemoteIdentificationValid,
    RemoteIdentificationInvalid,
    RemoteIdentificationOverLimit,
    TransportClosedBeforeKex,
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
            Self::LocalListenerModeled => "sshservicediag-local-listener-modeled",
            Self::LocalTransportModeled => "sshservicediag-local-transport-modeled",
            Self::IdentificationBannerModeled => "sshservicediag-identification-banner-modeled",
            Self::LocalIdentificationLiteral => "sshservicediag-local-identification-literal",
            Self::RemoteIdentificationValid => "sshservicediag-remote-identification-valid",
            Self::RemoteIdentificationInvalid => "sshservicediag-remote-identification-invalid",
            Self::RemoteIdentificationOverLimit => {
                "sshservicediag-remote-identification-over-limit"
            }
            Self::TransportClosedBeforeKex => "sshservicediag-transport-closed-before-kex",
            Self::AuthenticationUnimplemented => "sshservicediag-authentication-unimplemented",
            Self::SessionUnimplemented => "sshservicediag-session-unimplemented",
            Self::ExposureDisabled => "sshservicediag-exposure-disabled",
            Self::PrerequisitesMissing => "sshservicediag-prerequisites-missing",
            Self::ShapeModeled => "sshservicediag-shape-modeled",
            Self::NotReady => "sshservicediag-not-ready",
        }
    }
}

pub(crate) const MAX_SSH_SERVICE_READINESS_LABELS: usize = 16;
pub(crate) const SSH_LOCAL_IDENTIFICATION: &str = "SSH-2.0-Talos_0.1\r\n";
pub(crate) const SSH_REMOTE_IDENTIFICATION_MAX_BYTES: usize = 255;
const SSH_REMOTE_IDENTIFICATION_PREFIX: &[u8] = b"SSH-2.0-";
const SSH_LOCAL_MODELED_ENDPOINT_PORT: u16 = 22;
const SSH_LOCAL_TRANSPORT_SOCKET_CAPACITY: usize = 4;
const SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION: &[u8] = b"SSH-2.0-local-model\r\n";
const SSH_LOCAL_TRANSPORT_OWNER_RAW: u64 = 0x5353_4801;
const SSH_LOCAL_TRANSPORT_CLIENT_OWNER_RAW: u64 = 0x5353_4802;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshRemoteIdentificationInputState {
    Complete,
    EofBeforeCompleteLine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshRemoteIdentificationResult {
    Valid,
    Invalid,
    OverLimit,
}

impl SshRemoteIdentificationResult {
    pub(crate) const fn label(self) -> SshServiceReadinessLabel {
        match self {
            Self::Valid => SshServiceReadinessLabel::RemoteIdentificationValid,
            Self::Invalid => SshServiceReadinessLabel::RemoteIdentificationInvalid,
            Self::OverLimit => SshServiceReadinessLabel::RemoteIdentificationOverLimit,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshIdentificationBannerReport {
    remote_identification: SshRemoteIdentificationResult,
}

impl SshIdentificationBannerReport {
    pub(crate) const fn local_identification(&self) -> &'static str {
        SSH_LOCAL_IDENTIFICATION
    }

    pub(crate) const fn remote_identification(self) -> SshRemoteIdentificationResult {
        self.remote_identification
    }

    pub(crate) const fn remote_identification_present(self) -> bool {
        matches!(
            self.remote_identification,
            SshRemoteIdentificationResult::Valid
        )
    }

    pub(crate) const fn remote_identification_valid(self) -> bool {
        matches!(
            self.remote_identification,
            SshRemoteIdentificationResult::Valid
        )
    }

    pub(crate) const fn remote_identification_limited(self) -> bool {
        true
    }

    pub(crate) const fn transport_closed_before_kex(self) -> bool {
        true
    }
}

pub(crate) fn classify_ssh_identification_banner(
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
) -> SshIdentificationBannerReport {
    SshIdentificationBannerReport {
        remote_identification: classify_remote_identification(remote_input, input_state),
    }
}

fn classify_remote_identification(
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
) -> SshRemoteIdentificationResult {
    let Some(line_end) = first_lf_index(remote_input) else {
        return match input_state {
            SshRemoteIdentificationInputState::Complete => SshRemoteIdentificationResult::OverLimit,
            SshRemoteIdentificationInputState::EofBeforeCompleteLine => {
                SshRemoteIdentificationResult::Invalid
            }
        };
    };
    if line_end + 1 > SSH_REMOTE_IDENTIFICATION_MAX_BYTES {
        return SshRemoteIdentificationResult::OverLimit;
    }

    let semantic_end = if line_end > 0 && remote_input[line_end - 1] == b'\r' {
        line_end - 1
    } else {
        line_end
    };
    let semantic = &remote_input[..semantic_end];
    if !semantic.starts_with(SSH_REMOTE_IDENTIFICATION_PREFIX)
        || semantic.len() == SSH_REMOTE_IDENTIFICATION_PREFIX.len()
    {
        return SshRemoteIdentificationResult::Invalid;
    }

    let version = &semantic[SSH_REMOTE_IDENTIFICATION_PREFIX.len()..];
    if version.iter().all(|byte| matches!(byte, 0x20..=0x7e)) {
        SshRemoteIdentificationResult::Valid
    } else {
        SshRemoteIdentificationResult::Invalid
    }
}

fn first_lf_index(bytes: &[u8]) -> Option<usize> {
    let mut index = 0usize;
    let scan_len = if bytes.len() > SSH_REMOTE_IDENTIFICATION_MAX_BYTES {
        SSH_REMOTE_IDENTIFICATION_MAX_BYTES
    } else {
        bytes.len()
    };
    while index < scan_len {
        if bytes[index] == b'\n' {
            return Some(index);
        }
        index += 1;
    }
    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshServiceReadinessReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_SERVICE_READINESS_LABELS],
    label_count: usize,
    lifecycle: SshServiceLifecycleState,
    listener_count: usize,
    transport_enabled: bool,
    accepted_connection_count: usize,
    remote_identification: Option<SshRemoteIdentificationResult>,
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
        self.listener_count
    }

    pub(crate) const fn transport_enabled(self) -> bool {
        self.transport_enabled
    }

    pub(crate) const fn local_identification(self) -> &'static str {
        SSH_LOCAL_IDENTIFICATION
    }

    pub(crate) const fn accepted_connection_count(self) -> usize {
        self.accepted_connection_count
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

    pub(crate) const fn remote_identification(self) -> Option<SshRemoteIdentificationResult> {
        self.remote_identification
    }
}

pub(crate) fn classify_ssh_service_readiness(
    key_report: SshKeyReadinessReport,
) -> SshServiceReadinessReport {
    classify_ssh_service_readiness_with_remote_identification(
        key_report,
        SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
        SshRemoteIdentificationInputState::Complete,
    )
}

pub(crate) fn classify_ssh_service_readiness_with_remote_identification(
    key_report: SshKeyReadinessReport,
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
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
        listener_count: 0,
        transport_enabled: false,
        accepted_connection_count: 0,
        remote_identification: None,
    };

    if exposure_disabled {
        report.push(SshServiceReadinessLabel::ExposureDisabled);
    }
    if prerequisites_missing {
        report.push(SshServiceReadinessLabel::PrerequisitesMissing);
    }
    if lifecycle == SshServiceLifecycleState::ShapeModeled {
        let transport = model_local_ssh_listener_transport(remote_input, input_state);
        report.push(SshServiceReadinessLabel::ShapeModeled);
        match transport {
            Ok(banner) => {
                report.listener_count = 1;
                report.transport_enabled = true;
                report.accepted_connection_count = 1;
                report.remote_identification = Some(banner.remote_identification());
                report.push(SshServiceReadinessLabel::LocalListenerModeled);
                report.push(SshServiceReadinessLabel::LocalTransportModeled);
                report.push(SshServiceReadinessLabel::IdentificationBannerModeled);
                report.push(SshServiceReadinessLabel::LocalIdentificationLiteral);
                report.push(banner.remote_identification().label());
                report.push(SshServiceReadinessLabel::TransportClosedBeforeKex);
            }
            Err(()) => {
                report.push(SshServiceReadinessLabel::TransportUnaccepted);
            }
        }
    } else {
        report.push(SshServiceReadinessLabel::TransportUnaccepted);
    }

    report.push(SshServiceReadinessLabel::DependencyUnaccepted);
    report.push(SshServiceReadinessLabel::CryptoBackendUnaccepted);
    report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
    report.push(SshServiceReadinessLabel::SessionUnimplemented);
    report.push(SshServiceReadinessLabel::NotReady);
    report
}

fn model_local_ssh_listener_transport(
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
) -> Result<SshIdentificationBannerReport, ()> {
    let server_owner =
        crate::scheduler::ProcessOwnerId::new(SSH_LOCAL_TRANSPORT_OWNER_RAW).ok_or(())?;
    let client_owner =
        crate::scheduler::ProcessOwnerId::new(SSH_LOCAL_TRANSPORT_CLIENT_OWNER_RAW).ok_or(())?;
    let endpoint = crate::network::Ipv4Endpoint::new(
        crate::network::SOCKET_SYNTHETIC_LOCAL_IPV4_BE,
        SSH_LOCAL_MODELED_ENDPOINT_PORT,
    );
    let mut sockets =
        crate::network::NetworkSocketDescriptorTable::<SSH_LOCAL_TRANSPORT_SOCKET_CAPACITY>::new();

    let listener = sockets
        .open(
            server_owner,
            crate::network::SOCKET_DOMAIN_AF_INET,
            crate::network::SOCKET_TYPE_STREAM,
            crate::network::SOCKET_PROTOCOL_DEFAULT,
        )
        .map_err(|_| ())?;
    sockets
        .bind(server_owner, listener, endpoint)
        .map_err(|_| ())?;
    sockets
        .listen(
            server_owner,
            listener,
            crate::network::SOCKET_LISTEN_BACKLOG_MIN as u8,
        )
        .map_err(|_| ())?;

    let client = sockets
        .open(
            client_owner,
            crate::network::SOCKET_DOMAIN_AF_INET,
            crate::network::SOCKET_TYPE_STREAM,
            crate::network::SOCKET_PROTOCOL_DEFAULT,
        )
        .map_err(|_| ())?;
    sockets
        .connect(client_owner, client, endpoint)
        .map_err(|_| ())?;
    let listener_readiness = sockets
        .readiness(
            server_owner,
            listener,
            crate::network::NetworkSocketReadiness::READ,
        )
        .map_err(|_| ())?;
    if !listener_readiness.contains(crate::network::NetworkSocketReadiness::READ) {
        return Err(());
    }

    let accepted = sockets.accept(server_owner, listener).map_err(|_| ())?;
    sockets
        .send(server_owner, accepted, SSH_LOCAL_IDENTIFICATION.as_bytes())
        .map_err(|_| ())?;
    sockets
        .send(client_owner, client, remote_input)
        .map_err(|_| ())?;

    let mut received = [0u8; SSH_REMOTE_IDENTIFICATION_MAX_BYTES];
    let received_len = sockets
        .recv_peek(server_owner, accepted, &mut received)
        .map_err(|_| ())?;
    sockets
        .recv_commit(server_owner, accepted, received_len)
        .map_err(|_| ())?;
    let banner = classify_ssh_identification_banner(&received[..received_len], input_state);

    sockets.close(server_owner, accepted).map_err(|_| ())?;
    sockets.close(client_owner, client).map_err(|_| ())?;
    sockets.close(server_owner, listener).map_err(|_| ())?;
    Ok(banner)
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
                "",
                "",
                "",
                "",
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
    fn prerequisite_satisfied_shape_models_local_transport_but_remains_not_ready() {
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
                "sshservicediag-local-listener-modeled",
                "sshservicediag-local-transport-modeled",
                "sshservicediag-identification-banner-modeled",
                "sshservicediag-local-identification-literal",
                "sshservicediag-remote-identification-valid",
                "sshservicediag-transport-closed-before-kex",
                "sshservicediag-dependency-unaccepted",
                "sshservicediag-crypto-backend-unaccepted",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
                "",
                "",
                "",
                "",
                "",
            ]
        );
        assert_eq!(report.listener_count(), 1);
        assert_eq!(report.local_identification(), "SSH-2.0-Talos_0.1\r\n");
        assert_eq!(report.accepted_connection_count(), 1);
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(report.transport_enabled());
        assert_eq!(
            report.remote_identification(),
            Some(SshRemoteIdentificationResult::Valid)
        );
        assert!(!report.authentication_success());
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn local_transport_model_classifies_invalid_remote_identification_fail_closed() {
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
        let report = classify_ssh_service_readiness_with_remote_identification(
            key_report,
            b"invalid\r\n",
            SshRemoteIdentificationInputState::Complete,
        );

        assert_eq!(report.listener_count(), 1);
        assert_eq!(report.accepted_connection_count(), 1);
        assert!(report.transport_enabled());
        assert_eq!(
            report.remote_identification(),
            Some(SshRemoteIdentificationResult::Invalid)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::RemoteIdentificationInvalid)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::TransportClosedBeforeKex)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn local_transport_model_classifies_unterminated_remote_identification_as_over_limit() {
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
        let report = classify_ssh_service_readiness_with_remote_identification(
            key_report,
            &[b'a'; crate::network::SOCKET_PAYLOAD_QUEUE_CAPACITY],
            SshRemoteIdentificationInputState::Complete,
        );

        assert_eq!(report.listener_count(), 1);
        assert_eq!(report.accepted_connection_count(), 1);
        assert!(report.transport_enabled());
        assert_eq!(
            report.remote_identification(),
            Some(SshRemoteIdentificationResult::OverLimit)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::RemoteIdentificationOverLimit)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::TransportClosedBeforeKex)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn banner_accepts_single_valid_remote_identification_and_closes_before_kex() {
        let report = classify_ssh_identification_banner(
            b"SSH-2.0-test-client\r\n",
            SshRemoteIdentificationInputState::Complete,
        );

        assert_eq!(report.local_identification(), "SSH-2.0-Talos_0.1\r\n");
        assert_eq!(
            report.remote_identification(),
            SshRemoteIdentificationResult::Valid
        );
        assert_eq!(
            report.remote_identification().label(),
            SshServiceReadinessLabel::RemoteIdentificationValid
        );
        assert!(report.remote_identification_present());
        assert!(report.remote_identification_valid());
        assert!(report.remote_identification_limited());
        assert!(report.transport_closed_before_kex());
    }

    #[test_case]
    fn banner_rejects_comments_prefixes_empty_versions_and_control_bytes() {
        for input in [
            &b"comment\r\nSSH-2.0-client\r\n"[..],
            &b"SSH-1.99-client\r\n"[..],
            &b"SSH-2.0-\r\n"[..],
            &b"SSH-2.0-client\x00\r\n"[..],
            &b"SSH-2.0-client\x7f\r\n"[..],
            &b"SSH-2.0-client\xff\r\n"[..],
        ] {
            let report = classify_ssh_identification_banner(
                input,
                SshRemoteIdentificationInputState::Complete,
            );

            assert_eq!(
                report.remote_identification(),
                SshRemoteIdentificationResult::Invalid
            );
            assert_eq!(
                report.remote_identification().label(),
                SshServiceReadinessLabel::RemoteIdentificationInvalid
            );
            assert!(!report.remote_identification_present());
            assert!(!report.remote_identification_valid());
            assert!(report.transport_closed_before_kex());
        }
    }

    #[test_case]
    fn banner_distinguishes_eof_before_line_from_missing_terminator_limit() {
        let eof_report = classify_ssh_identification_banner(
            b"SSH-2.0-client",
            SshRemoteIdentificationInputState::EofBeforeCompleteLine,
        );
        let over_limit_report = classify_ssh_identification_banner(
            &[b'a'; SSH_REMOTE_IDENTIFICATION_MAX_BYTES],
            SshRemoteIdentificationInputState::Complete,
        );

        assert_eq!(
            eof_report.remote_identification(),
            SshRemoteIdentificationResult::Invalid
        );
        assert_eq!(
            over_limit_report.remote_identification(),
            SshRemoteIdentificationResult::OverLimit
        );
        assert_eq!(
            over_limit_report.remote_identification().label(),
            SshServiceReadinessLabel::RemoteIdentificationOverLimit
        );
        assert!(eof_report.transport_closed_before_kex());
        assert!(over_limit_report.transport_closed_before_kex());
    }

    #[test_case]
    fn banner_accepts_maximum_line_with_lf_inside_limit() {
        let mut input = [b'a'; SSH_REMOTE_IDENTIFICATION_MAX_BYTES];
        input[..SSH_REMOTE_IDENTIFICATION_PREFIX.len()]
            .copy_from_slice(SSH_REMOTE_IDENTIFICATION_PREFIX);
        input[SSH_REMOTE_IDENTIFICATION_MAX_BYTES - 1] = b'\n';

        let report =
            classify_ssh_identification_banner(&input, SshRemoteIdentificationInputState::Complete);

        assert_eq!(
            report.remote_identification(),
            SshRemoteIdentificationResult::Valid
        );
        assert!(report.remote_identification_limited());
        assert!(report.transport_closed_before_kex());
    }
}
