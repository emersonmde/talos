//! SSH service readiness diagnostic shape.
//!
//! This module models fixed fail-closed service lifecycle diagnostics only. It
//! does not adopt an SSH dependency, perform SSH crypto, authenticate users,
//! attach a shell, inspect hardware, or expose secrets.

use zeroize::Zeroize;

use crate::{
    csprng::OperatorSeededCsprng,
    ssh_key_readiness::{HostKeyPrivateMaterial, SshKeyReadinessLabel, SshKeyReadinessReport},
    ssh_runtime_crypto::{
        SshRuntimeKexInput, SshRuntimeKexLabel, SshRuntimeKexResultKind, perform_runtime_kex,
    },
};

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
    KexinitModeled,
    KexinitCookieGeneratedRedacted,
    KexinitClientPacketValid,
    KexinitAlgorithmNegotiated,
    KexinitAlgorithmUnsupported,
    KexinitPacketMalformed,
    KexinitPacketOverLimit,
    KexinitListOverLimit,
    KexinitFirstPacketFollowsIgnored,
    KexinitSelectedKexCurve25519Sha256,
    KexinitSelectedHostKeySshEd25519,
    KexinitSelectedCipherChacha20Poly1305OpenSsh,
    KexinitSelectedMacHmacSha2_256,
    KexinitSelectedCompressionNone,
    CryptoBackendReady,
    KexPeerPublicKeyInvalid,
    KexCsprngNotReady,
    KexHostKeyNotReady,
    KexTranscriptInvalid,
    KexKeyDerivationFailed,
    EncryptedPacketStateNotReady,
    EncryptedPacketStateReady,
    NewkeysNotReady,
    NewkeysSendActive,
    NewkeysReceiveActive,
    EncryptedPacketStateActive,
    EncryptedPacketSequenceAdvanced,
    EncryptedPacketSequenceOverflow,
    EncryptedPacketCryptoFailed,
    EncryptedPacketDiagnosticReady,
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
            Self::KexinitModeled => "sshservicediag-kexinit-modeled",
            Self::KexinitCookieGeneratedRedacted => {
                "sshservicediag-kexinit-cookie-generated-redacted"
            }
            Self::KexinitClientPacketValid => "sshservicediag-kexinit-client-packet-valid",
            Self::KexinitAlgorithmNegotiated => "sshservicediag-kexinit-algorithm-negotiated",
            Self::KexinitAlgorithmUnsupported => "sshservicediag-kexinit-algorithm-unsupported",
            Self::KexinitPacketMalformed => "sshservicediag-kexinit-packet-malformed",
            Self::KexinitPacketOverLimit => "sshservicediag-kexinit-packet-over-limit",
            Self::KexinitListOverLimit => "sshservicediag-kexinit-list-over-limit",
            Self::KexinitFirstPacketFollowsIgnored => {
                "sshservicediag-kexinit-first-packet-follows-ignored"
            }
            Self::KexinitSelectedKexCurve25519Sha256 => {
                "sshservicediag-kexinit-selected-kex-curve25519-sha256"
            }
            Self::KexinitSelectedHostKeySshEd25519 => {
                "sshservicediag-kexinit-selected-hostkey-ssh-ed25519"
            }
            Self::KexinitSelectedCipherChacha20Poly1305OpenSsh => {
                "sshservicediag-kexinit-selected-cipher-chacha20-poly1305-openssh"
            }
            Self::KexinitSelectedMacHmacSha2_256 => {
                "sshservicediag-kexinit-selected-mac-hmac-sha2-256"
            }
            Self::KexinitSelectedCompressionNone => {
                "sshservicediag-kexinit-selected-compression-none"
            }
            Self::CryptoBackendReady => "sshservicediag-crypto-backend-ready",
            Self::KexPeerPublicKeyInvalid => "sshservicediag-kex-peer-public-key-invalid",
            Self::KexCsprngNotReady => "sshservicediag-kex-csprng-not-ready",
            Self::KexHostKeyNotReady => "sshservicediag-kex-host-key-not-ready",
            Self::KexTranscriptInvalid => "sshservicediag-kex-transcript-invalid",
            Self::KexKeyDerivationFailed => "sshservicediag-kex-key-derivation-failed",
            Self::EncryptedPacketStateNotReady => "sshservicediag-encrypted-packet-state-not-ready",
            Self::EncryptedPacketStateReady => "sshservicediag-encrypted-packet-state-ready",
            Self::NewkeysNotReady => "sshservicediag-newkeys-not-ready",
            Self::NewkeysSendActive => "sshservicediag-newkeys-send-active",
            Self::NewkeysReceiveActive => "sshservicediag-newkeys-receive-active",
            Self::EncryptedPacketStateActive => "sshservicediag-encrypted-packet-state-active",
            Self::EncryptedPacketSequenceAdvanced => {
                "sshservicediag-encrypted-packet-sequence-advanced"
            }
            Self::EncryptedPacketSequenceOverflow => {
                "sshservicediag-encrypted-packet-sequence-overflow"
            }
            Self::EncryptedPacketCryptoFailed => "sshservicediag-encrypted-packet-crypto-failed",
            Self::EncryptedPacketDiagnosticReady => {
                "sshservicediag-encrypted-packet-diagnostic-ready"
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

pub(crate) const MAX_SSH_SERVICE_READINESS_LABELS: usize = 40;
pub(crate) const SSH_LOCAL_IDENTIFICATION: &str = "SSH-2.0-Talos_0.1\r\n";
pub(crate) const SSH_REMOTE_IDENTIFICATION_MAX_BYTES: usize = 255;
pub(crate) const SSH_KEXINIT_PACKET_MAX_BYTES: usize = 1024;
pub(crate) const SSH_KEXINIT_PAYLOAD_MAX_BYTES: usize = 768;
pub(crate) const SSH_KEXINIT_NAME_LIST_MAX_BYTES: usize = 256;
pub(crate) const SSH_KEXINIT_NAME_LIST_MAX_NAMES: usize = 16;
const SSH_REMOTE_IDENTIFICATION_PREFIX: &[u8] = b"SSH-2.0-";
const SSH_LOCAL_MODELED_ENDPOINT_PORT: u16 = 22;
const SSH_LOCAL_TRANSPORT_SOCKET_CAPACITY: usize = 4;
const SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION: &[u8] = b"SSH-2.0-local-model\r\n";
const SSH_LOCAL_TRANSPORT_OWNER_RAW: u64 = 0x5353_4801;
const SSH_LOCAL_TRANSPORT_CLIENT_OWNER_RAW: u64 = 0x5353_4802;
const SSH_MSG_KEXINIT: u8 = 20;
const SSH_KEXINIT_COOKIE_BYTES: usize = 16;
const SSH_KEXINIT_LIST_COUNT: usize = 10;
const SSH_KEXINIT_REQUIRED_LIST_COUNT: usize = 8;
const SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES: usize = SSH_KEXINIT_PACKET_MAX_BYTES + 4;
const SSH_KEXINIT_MODELED_COOKIE_SEED: [u8; crate::csprng::CSPRNG_SEED_BYTES] =
    *b"Talos-kexinit-cookie-redacted!!!";

const SSH_KEXINIT_POLICY_KEX: &[u8] = b"curve25519-sha256";
const SSH_KEXINIT_POLICY_HOST_KEY: &[u8] = b"ssh-ed25519";
const SSH_KEXINIT_POLICY_CIPHER: &[u8] = b"chacha20-poly1305@openssh.com";
const SSH_KEXINIT_POLICY_MAC: &[u8] = b"hmac-sha2-256";
const SSH_KEXINIT_POLICY_COMPRESSION: &[u8] = b"none";

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
    kexinit_result: Option<SshKexinitNegotiationResult>,
    runtime_kex_result: Option<SshRuntimeKexResultKind>,
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

    pub(crate) const fn kexinit_result(self) -> Option<SshKexinitNegotiationResult> {
        self.kexinit_result
    }

    pub(crate) const fn runtime_kex_result(self) -> Option<SshRuntimeKexResultKind> {
        self.runtime_kex_result
    }

    pub(crate) const fn kexinit_modeled(self) -> bool {
        matches!(
            self.kexinit_result,
            Some(SshKexinitNegotiationResult::Negotiated { .. })
                | Some(SshKexinitNegotiationResult::UnsupportedAlgorithm)
                | Some(SshKexinitNegotiationResult::MalformedPacket)
                | Some(SshKexinitNegotiationResult::PacketOverLimit)
                | Some(SshKexinitNegotiationResult::ListOverLimit)
        )
    }

    pub(crate) const fn kexinit_cookie_generated_redacted(self) -> bool {
        matches!(
            self.kexinit_result,
            Some(SshKexinitNegotiationResult::Negotiated { .. })
        )
    }
}

pub(crate) fn classify_ssh_service_readiness(
    key_report: SshKeyReadinessReport,
) -> SshServiceReadinessReport {
    let mut packet = [0u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES];
    let packet_len = build_modeled_client_kexinit_packet(&mut packet, false);
    classify_ssh_service_readiness_with_remote_identification_and_kexinit(
        key_report,
        SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
        SshRemoteIdentificationInputState::Complete,
        &packet[..packet_len],
    )
}

pub(crate) fn classify_ssh_service_readiness_with_remote_identification(
    key_report: SshKeyReadinessReport,
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
) -> SshServiceReadinessReport {
    let mut packet = [0u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES];
    let packet_len = build_modeled_client_kexinit_packet(&mut packet, false);
    classify_ssh_service_readiness_with_remote_identification_and_kexinit(
        key_report,
        remote_input,
        input_state,
        &packet[..packet_len],
    )
}

pub(crate) fn classify_ssh_service_readiness_with_remote_identification_and_kexinit(
    key_report: SshKeyReadinessReport,
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
    client_kexinit_packet: &[u8],
) -> SshServiceReadinessReport {
    classify_ssh_service_readiness_inner(
        key_report,
        remote_input,
        input_state,
        client_kexinit_packet,
        None,
    )
}

pub(crate) fn classify_ssh_service_readiness_with_runtime_kex(
    key_report: SshKeyReadinessReport,
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
    client_kexinit_packet: &[u8],
    host_key: Option<&HostKeyPrivateMaterial>,
    csprng: &mut OperatorSeededCsprng,
    peer_public_key: &[u8],
) -> SshServiceReadinessReport {
    classify_ssh_service_readiness_inner(
        key_report,
        remote_input,
        input_state,
        client_kexinit_packet,
        Some(RuntimeKexAttempt {
            host_key,
            csprng,
            peer_public_key,
        }),
    )
}

struct RuntimeKexAttempt<'a> {
    host_key: Option<&'a HostKeyPrivateMaterial>,
    csprng: &'a mut OperatorSeededCsprng,
    peer_public_key: &'a [u8],
}

fn classify_ssh_service_readiness_inner(
    key_report: SshKeyReadinessReport,
    remote_input: &[u8],
    input_state: SshRemoteIdentificationInputState,
    client_kexinit_packet: &[u8],
    mut runtime_kex_attempt: Option<RuntimeKexAttempt<'_>>,
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
        kexinit_result: None,
        runtime_kex_result: None,
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
                if banner.remote_identification_valid() {
                    let kexinit = model_ssh_kexinit_negotiation(client_kexinit_packet);
                    report.kexinit_result = Some(kexinit);
                    report.push(SshServiceReadinessLabel::KexinitModeled);
                    match kexinit {
                        SshKexinitNegotiationResult::Negotiated {
                            first_packet_follows,
                        } => {
                            report.push(SshServiceReadinessLabel::KexinitCookieGeneratedRedacted);
                            report.push(SshServiceReadinessLabel::KexinitClientPacketValid);
                            report.push(SshServiceReadinessLabel::KexinitAlgorithmNegotiated);
                            if first_packet_follows {
                                report.push(
                                    SshServiceReadinessLabel::KexinitFirstPacketFollowsIgnored,
                                );
                            }
                            report
                                .push(SshServiceReadinessLabel::KexinitSelectedKexCurve25519Sha256);
                            report.push(SshServiceReadinessLabel::KexinitSelectedHostKeySshEd25519);
                            report.push(
                                SshServiceReadinessLabel::KexinitSelectedCipherChacha20Poly1305OpenSsh,
                            );
                            report.push(SshServiceReadinessLabel::KexinitSelectedMacHmacSha2_256);
                            report.push(SshServiceReadinessLabel::KexinitSelectedCompressionNone);
                            if let Some(attempt) = runtime_kex_attempt.as_mut() {
                                let mut server_kexinit_packet =
                                    [0u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES];
                                let server_kexinit_len = build_modeled_client_kexinit_packet(
                                    &mut server_kexinit_packet,
                                    false,
                                );
                                let runtime_kex = perform_runtime_kex(SshRuntimeKexInput {
                                    client_identification: remote_input,
                                    server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
                                    client_kexinit_packet,
                                    server_kexinit_packet: &server_kexinit_packet
                                        [..server_kexinit_len],
                                    peer_public_key: attempt.peer_public_key,
                                    host_key: attempt.host_key,
                                    csprng: attempt.csprng,
                                });
                                report.runtime_kex_result = Some(runtime_kex.kind());
                                report.push(runtime_kex_label(runtime_kex.label()));
                                if runtime_kex.encrypted_packet_state_ready() {
                                    report.push(runtime_kex_label(
                                        SshRuntimeKexLabel::EncryptedPacketStateReady,
                                    ));
                                }
                            }
                        }
                        SshKexinitNegotiationResult::UnsupportedAlgorithm => {
                            report.push(SshServiceReadinessLabel::KexinitClientPacketValid);
                            report.push(SshServiceReadinessLabel::KexinitAlgorithmUnsupported);
                        }
                        SshKexinitNegotiationResult::MalformedPacket => {
                            report.push(SshServiceReadinessLabel::KexinitPacketMalformed);
                        }
                        SshKexinitNegotiationResult::PacketOverLimit => {
                            report.push(SshServiceReadinessLabel::KexinitPacketOverLimit);
                        }
                        SshKexinitNegotiationResult::ListOverLimit => {
                            report.push(SshServiceReadinessLabel::KexinitListOverLimit);
                        }
                    }
                }
                report.push(SshServiceReadinessLabel::TransportClosedBeforeKex);
            }
            Err(()) => {
                report.push(SshServiceReadinessLabel::TransportUnaccepted);
            }
        }
    } else {
        report.push(SshServiceReadinessLabel::TransportUnaccepted);
    }

    if report.runtime_kex_result.is_none() {
        report.push(SshServiceReadinessLabel::DependencyUnaccepted);
        report.push(SshServiceReadinessLabel::CryptoBackendUnaccepted);
    }
    report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
    report.push(SshServiceReadinessLabel::SessionUnimplemented);
    report.push(SshServiceReadinessLabel::NotReady);
    report
}

const fn runtime_kex_label(label: SshRuntimeKexLabel) -> SshServiceReadinessLabel {
    match label {
        SshRuntimeKexLabel::CryptoBackendReady => SshServiceReadinessLabel::CryptoBackendReady,
        SshRuntimeKexLabel::KexPeerPublicKeyInvalid => {
            SshServiceReadinessLabel::KexPeerPublicKeyInvalid
        }
        SshRuntimeKexLabel::KexCsprngNotReady => SshServiceReadinessLabel::KexCsprngNotReady,
        SshRuntimeKexLabel::KexHostKeyNotReady => SshServiceReadinessLabel::KexHostKeyNotReady,
        SshRuntimeKexLabel::KexTranscriptInvalid => SshServiceReadinessLabel::KexTranscriptInvalid,
        SshRuntimeKexLabel::KexKeyDerivationFailed => {
            SshServiceReadinessLabel::KexKeyDerivationFailed
        }
        SshRuntimeKexLabel::EncryptedPacketStateNotReady => {
            SshServiceReadinessLabel::EncryptedPacketStateNotReady
        }
        SshRuntimeKexLabel::EncryptedPacketStateReady => {
            SshServiceReadinessLabel::EncryptedPacketStateReady
        }
        SshRuntimeKexLabel::NewkeysNotReady => SshServiceReadinessLabel::NewkeysNotReady,
        SshRuntimeKexLabel::NewkeysSendActive => SshServiceReadinessLabel::NewkeysSendActive,
        SshRuntimeKexLabel::NewkeysReceiveActive => SshServiceReadinessLabel::NewkeysReceiveActive,
        SshRuntimeKexLabel::EncryptedPacketStateActive => {
            SshServiceReadinessLabel::EncryptedPacketStateActive
        }
        SshRuntimeKexLabel::EncryptedPacketSequenceAdvanced => {
            SshServiceReadinessLabel::EncryptedPacketSequenceAdvanced
        }
        SshRuntimeKexLabel::EncryptedPacketSequenceOverflow => {
            SshServiceReadinessLabel::EncryptedPacketSequenceOverflow
        }
        SshRuntimeKexLabel::EncryptedPacketCryptoFailed => {
            SshServiceReadinessLabel::EncryptedPacketCryptoFailed
        }
        SshRuntimeKexLabel::EncryptedPacketDiagnosticReady => {
            SshServiceReadinessLabel::EncryptedPacketDiagnosticReady
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshKexinitNegotiationResult {
    Negotiated { first_packet_follows: bool },
    UnsupportedAlgorithm,
    MalformedPacket,
    PacketOverLimit,
    ListOverLimit,
}

fn model_ssh_kexinit_negotiation(packet: &[u8]) -> SshKexinitNegotiationResult {
    let parse = parse_ssh_kexinit_packet(packet);
    let kexinit = match parse {
        Ok(kexinit) => kexinit,
        Err(error) => return error,
    };

    let mut server_cookie = [0u8; SSH_KEXINIT_COOKIE_BYTES];
    let mut csprng =
        crate::csprng::OperatorSeededCsprng::from_seed_bytes(&SSH_KEXINIT_MODELED_COOKIE_SEED);
    if csprng.fill_bytes(&mut server_cookie).is_err() {
        server_cookie.zeroize();
        return SshKexinitNegotiationResult::MalformedPacket;
    }
    server_cookie.zeroize();

    if kexinit.negotiates_policy() {
        SshKexinitNegotiationResult::Negotiated {
            first_packet_follows: kexinit.first_packet_follows,
        }
    } else {
        SshKexinitNegotiationResult::UnsupportedAlgorithm
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParsedKexinit<'a> {
    lists: [&'a [u8]; SSH_KEXINIT_LIST_COUNT],
    first_packet_follows: bool,
}

impl ParsedKexinit<'_> {
    fn negotiates_policy(self) -> bool {
        name_list_contains(self.lists[0], SSH_KEXINIT_POLICY_KEX)
            && name_list_contains(self.lists[1], SSH_KEXINIT_POLICY_HOST_KEY)
            && name_list_contains(self.lists[2], SSH_KEXINIT_POLICY_CIPHER)
            && name_list_contains(self.lists[3], SSH_KEXINIT_POLICY_CIPHER)
            && name_list_contains(self.lists[4], SSH_KEXINIT_POLICY_MAC)
            && name_list_contains(self.lists[5], SSH_KEXINIT_POLICY_MAC)
            && name_list_contains(self.lists[6], SSH_KEXINIT_POLICY_COMPRESSION)
            && name_list_contains(self.lists[7], SSH_KEXINIT_POLICY_COMPRESSION)
            && self.lists[8].is_empty()
            && self.lists[9].is_empty()
    }
}

fn parse_ssh_kexinit_packet(
    packet: &[u8],
) -> Result<ParsedKexinit<'_>, SshKexinitNegotiationResult> {
    if packet.len() < 6 {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }
    let packet_length =
        read_be_u32(packet, 0).ok_or(SshKexinitNegotiationResult::MalformedPacket)? as usize;
    if packet_length > SSH_KEXINIT_PACKET_MAX_BYTES {
        return Err(SshKexinitNegotiationResult::PacketOverLimit);
    }
    if packet.len() != packet_length + 4 {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }

    let padding_length = packet[4] as usize;
    if padding_length < 4 || packet_length <= padding_length + 1 {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }
    let payload_len = packet_length - padding_length - 1;
    if payload_len > SSH_KEXINIT_PAYLOAD_MAX_BYTES {
        return Err(SshKexinitNegotiationResult::PacketOverLimit);
    }

    let payload_start = 5usize;
    let payload_end = payload_start + payload_len;
    let payload = &packet[payload_start..payload_end];
    if payload.len() < 1 + SSH_KEXINIT_COOKIE_BYTES + (SSH_KEXINIT_LIST_COUNT * 4) + 5 {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }
    if payload[0] != SSH_MSG_KEXINIT {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }

    let mut cursor = 1 + SSH_KEXINIT_COOKIE_BYTES;
    let mut lists = [&[][..]; SSH_KEXINIT_LIST_COUNT];
    let mut index = 0usize;
    while index < SSH_KEXINIT_LIST_COUNT {
        if cursor + 4 > payload.len() {
            return Err(SshKexinitNegotiationResult::MalformedPacket);
        }
        let list_len = read_be_u32(payload, cursor)
            .ok_or(SshKexinitNegotiationResult::MalformedPacket)? as usize;
        cursor += 4;
        if list_len > SSH_KEXINIT_NAME_LIST_MAX_BYTES {
            return Err(SshKexinitNegotiationResult::ListOverLimit);
        }
        if cursor + list_len > payload.len() {
            return Err(SshKexinitNegotiationResult::MalformedPacket);
        }
        let list = &payload[cursor..cursor + list_len];
        cursor += list_len;
        if index < SSH_KEXINIT_REQUIRED_LIST_COUNT && list.is_empty() {
            return Err(SshKexinitNegotiationResult::UnsupportedAlgorithm);
        }
        if !name_list_shape_valid(list) {
            return Err(SshKexinitNegotiationResult::ListOverLimit);
        }
        lists[index] = list;
        index += 1;
    }

    if cursor + 5 != payload.len() {
        return Err(SshKexinitNegotiationResult::MalformedPacket);
    }
    let first_packet_follows = match payload[cursor] {
        0 => false,
        1 => true,
        _ => return Err(SshKexinitNegotiationResult::MalformedPacket),
    };
    cursor += 1;
    let _reserved =
        read_be_u32(payload, cursor).ok_or(SshKexinitNegotiationResult::MalformedPacket)?;

    Ok(ParsedKexinit {
        lists,
        first_packet_follows,
    })
}

fn read_be_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    if offset + 4 > bytes.len() {
        return None;
    }
    Some(
        ((bytes[offset] as u32) << 24)
            | ((bytes[offset + 1] as u32) << 16)
            | ((bytes[offset + 2] as u32) << 8)
            | (bytes[offset + 3] as u32),
    )
}

fn name_list_shape_valid(list: &[u8]) -> bool {
    if list.is_empty() {
        return true;
    }
    let mut names = 1usize;
    let mut previous_comma = false;
    for byte in list {
        if *byte == b',' {
            if previous_comma {
                return false;
            }
            names += 1;
            if names > SSH_KEXINIT_NAME_LIST_MAX_NAMES {
                return false;
            }
            previous_comma = true;
        } else if matches!(*byte, 0x21..=0x7e) {
            previous_comma = false;
        } else {
            return false;
        }
    }
    !previous_comma
}

fn name_list_contains(list: &[u8], target: &[u8]) -> bool {
    let mut start = 0usize;
    let mut index = 0usize;
    while index <= list.len() {
        if index == list.len() || list[index] == b',' {
            if &list[start..index] == target {
                return true;
            }
            start = index + 1;
        }
        index += 1;
    }
    false
}

fn build_modeled_client_kexinit_packet(
    output: &mut [u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES],
    first_packet_follows: bool,
) -> usize {
    let mut cursor = 5usize;
    output[cursor] = SSH_MSG_KEXINIT;
    cursor += 1;
    cursor += SSH_KEXINIT_COOKIE_BYTES;
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_KEX);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_HOST_KEY);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_CIPHER);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_CIPHER);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_MAC);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_MAC);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_COMPRESSION);
    cursor = write_name_list(output, cursor, SSH_KEXINIT_POLICY_COMPRESSION);
    cursor = write_name_list(output, cursor, b"");
    cursor = write_name_list(output, cursor, b"");
    output[cursor] = if first_packet_follows { 1 } else { 0 };
    cursor += 5;

    let payload_len = cursor - 5;
    let padding_length = 8usize;
    let packet_length = 1 + payload_len + padding_length;
    write_be_u32(output, 0, packet_length as u32);
    output[4] = padding_length as u8;
    cursor += padding_length;
    cursor
}

fn write_name_list(
    output: &mut [u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES],
    cursor: usize,
    name: &[u8],
) -> usize {
    write_be_u32(output, cursor, name.len() as u32);
    let start = cursor + 4;
    let end = start + name.len();
    output[start..end].copy_from_slice(name);
    end
}

fn write_be_u32(
    output: &mut [u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES],
    offset: usize,
    value: u32,
) {
    output[offset] = (value >> 24) as u8;
    output[offset + 1] = (value >> 16) as u8;
    output[offset + 2] = (value >> 8) as u8;
    output[offset + 3] = value as u8;
}

#[cfg(test)]
pub(crate) fn build_modeled_client_kexinit_packet_for_runtime_test(
    output: &mut [u8; SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES],
    first_packet_follows: bool,
) -> usize {
    build_modeled_client_kexinit_packet(output, first_packet_follows)
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
        csprng::OperatorSeededCsprng,
        entropy::{self, EntropyDiagnosticSnapshot, OperatorSeedObservation},
        ssh_key_readiness::{self, HostKeyMaterialMetadata, SshKeyReadinessSnapshot},
        ssh_runtime_crypto::SshRuntimeKexResultKind,
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

    fn shape_modeled_key_report() -> SshKeyReadinessReport {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty()
                .with_operator_seed(OperatorSeedObservation::new(32))
                .with_csprng_ready(),
        );
        ssh_key_readiness::classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_metadata()
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_persistence_metadata()
                .with_exposure_enabled()
                .with_entropy_report(entropy),
        )
    }

    const PUBLIC_FIXTURE_SEED: [u8; crate::csprng::CSPRNG_SEED_BYTES + 16] = [
        0x70, 0x68, 0x61, 0x73, 0x65, 0x31, 0x32, 0x2d, 0x63, 0x73, 0x70, 0x72, 0x6e, 0x67, 0x2d,
        0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x2d, 0x66, 0x69, 0x78, 0x74, 0x75, 0x72, 0x65, 0x2d,
        0x76, 0x31, 0x2d, 0x6e, 0x6f, 0x74, 0x2d, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x21, 0x21,
        0x21, 0x21, 0x21,
    ];

    fn modeled_kexinit_packet(
        first_packet_follows: bool,
    ) -> [u8; SSH_KEXINIT_PACKET_MAX_BYTES + 4] {
        let mut packet = [0u8; SSH_KEXINIT_PACKET_MAX_BYTES + 4];
        let packet_len = build_modeled_client_kexinit_packet(&mut packet, first_packet_follows);
        packet[0..4].copy_from_slice(&((packet_len - 4) as u32).to_be_bytes());
        packet
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
            &label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-exposure-disabled",
                "sshservicediag-prerequisites-missing",
                "sshservicediag-transport-unaccepted",
                "sshservicediag-dependency-unaccepted",
                "sshservicediag-crypto-backend-unaccepted",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
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
        let key_report = shape_modeled_key_report();
        let report = classify_ssh_service_readiness(key_report);

        assert_eq!(report.lifecycle(), SshServiceLifecycleState::ShapeModeled);
        assert_eq!(
            &label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-shape-modeled",
                "sshservicediag-local-listener-modeled",
                "sshservicediag-local-transport-modeled",
                "sshservicediag-identification-banner-modeled",
                "sshservicediag-local-identification-literal",
                "sshservicediag-remote-identification-valid",
                "sshservicediag-kexinit-modeled",
                "sshservicediag-kexinit-cookie-generated-redacted",
                "sshservicediag-kexinit-client-packet-valid",
                "sshservicediag-kexinit-algorithm-negotiated",
                "sshservicediag-kexinit-selected-kex-curve25519-sha256",
                "sshservicediag-kexinit-selected-hostkey-ssh-ed25519",
                "sshservicediag-kexinit-selected-cipher-chacha20-poly1305-openssh",
                "sshservicediag-kexinit-selected-mac-hmac-sha2-256",
                "sshservicediag-kexinit-selected-compression-none",
                "sshservicediag-transport-closed-before-kex",
                "sshservicediag-dependency-unaccepted",
                "sshservicediag-crypto-backend-unaccepted",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
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
        assert_eq!(
            report.kexinit_result(),
            Some(SshKexinitNegotiationResult::Negotiated {
                first_packet_follows: false
            })
        );
        assert!(report.kexinit_modeled());
        assert!(report.kexinit_cookie_generated_redacted());
        assert!(!report.authentication_success());
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn local_transport_model_classifies_invalid_remote_identification_fail_closed() {
        let key_report = shape_modeled_key_report();
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
        assert_eq!(report.kexinit_result(), None);
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn local_transport_model_classifies_unterminated_remote_identification_as_over_limit() {
        let key_report = shape_modeled_key_report();
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
        assert_eq!(report.kexinit_result(), None);
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn kexinit_negotiates_policy_with_redacted_cookie_and_ignored_followup() {
        let key_report = shape_modeled_key_report();
        let packet = modeled_kexinit_packet(true);
        let packet_len = (read_be_u32(&packet, 0).unwrap() as usize) + 4;
        let report = classify_ssh_service_readiness_with_remote_identification_and_kexinit(
            key_report,
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
        );

        assert_eq!(
            report.kexinit_result(),
            Some(SshKexinitNegotiationResult::Negotiated {
                first_packet_follows: true
            })
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitCookieGeneratedRedacted)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitFirstPacketFollowsIgnored)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn runtime_kex_integration_marks_crypto_ready_without_ssh_readiness() {
        let key_report = shape_modeled_key_report();
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let packet = modeled_kexinit_packet(false);
        let packet_len = (read_be_u32(&packet, 0).unwrap() as usize) + 4;
        let peer_public_key = [
            9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let report = classify_ssh_service_readiness_with_runtime_kex(
            key_report,
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
            Some(&host_key),
            &mut csprng,
            &peer_public_key,
        );

        assert_eq!(
            report.runtime_kex_result(),
            Some(SshRuntimeKexResultKind::Ready)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::CryptoBackendReady)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::EncryptedPacketStateReady)
        );
        assert!(
            !report
                .labels()
                .contains(&SshServiceReadinessLabel::CryptoBackendUnaccepted)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::AuthenticationUnimplemented)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::SessionUnimplemented)
        );
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn runtime_kex_integration_reports_fail_closed_labels_without_secret_evidence() {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let packet = modeled_kexinit_packet(false);
        let packet_len = (read_be_u32(&packet, 0).unwrap() as usize) + 4;
        let peer_public_key = [
            9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let mut not_ready_csprng = OperatorSeededCsprng::from_seed_bytes(b"short");
        let csprng_report = classify_ssh_service_readiness_with_runtime_kex(
            shape_modeled_key_report(),
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
            Some(&host_key),
            &mut not_ready_csprng,
            &peer_public_key,
        );
        assert_eq!(
            csprng_report.runtime_kex_result(),
            Some(SshRuntimeKexResultKind::Failed(
                crate::ssh_runtime_crypto::SshRuntimeKexFailure::CsprngNotReady(
                    not_ready_csprng.readiness().state()
                )
            ))
        );
        assert!(
            csprng_report
                .labels()
                .contains(&SshServiceReadinessLabel::KexCsprngNotReady)
        );
        assert!(!csprng_report.ssh_ready());

        let mut ready_csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let missing_host_report = classify_ssh_service_readiness_with_runtime_kex(
            shape_modeled_key_report(),
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
            None,
            &mut ready_csprng,
            &peer_public_key,
        );
        assert_eq!(
            missing_host_report.runtime_kex_result(),
            Some(SshRuntimeKexResultKind::Failed(
                crate::ssh_runtime_crypto::SshRuntimeKexFailure::HostKeyNotReady
            ))
        );
        assert!(
            missing_host_report
                .labels()
                .contains(&SshServiceReadinessLabel::KexHostKeyNotReady)
        );
        assert!(!missing_host_report.ssh_ready());

        let mut ready_csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let invalid_peer_public_key = [0u8; 32];
        let peer_report = classify_ssh_service_readiness_with_runtime_kex(
            shape_modeled_key_report(),
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
            Some(&host_key),
            &mut ready_csprng,
            &invalid_peer_public_key,
        );
        assert_eq!(
            peer_report.runtime_kex_result(),
            Some(SshRuntimeKexResultKind::Failed(
                crate::ssh_runtime_crypto::SshRuntimeKexFailure::InvalidPeerPublicKey
            ))
        );
        assert!(
            peer_report
                .labels()
                .contains(&SshServiceReadinessLabel::KexPeerPublicKeyInvalid)
        );
        assert!(!peer_report.ssh_ready());

        let invalid_host_key_report = ssh_key_readiness::classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_material(HostKeyMaterialMetadata::invalid(Some(0)))
                .with_authorized_key_metadata()
                .with_seed_material_metadata()
                .with_persistence_metadata()
                .with_exposure_enabled(),
        );
        let invalid_host_report = classify_ssh_service_readiness(invalid_host_key_report);
        assert_eq!(
            invalid_host_report.lifecycle(),
            SshServiceLifecycleState::PrerequisitesMissing
        );
        assert!(
            invalid_host_report
                .labels()
                .contains(&SshServiceReadinessLabel::PrerequisitesMissing)
        );
        assert!(
            invalid_host_report
                .labels()
                .contains(&SshServiceReadinessLabel::CryptoBackendUnaccepted)
        );
        assert_eq!(invalid_host_report.runtime_kex_result(), None);
        assert!(!invalid_host_report.ssh_ready());
    }

    #[test_case]
    fn kexinit_rejects_unsupported_algorithm_without_retaining_client_text() {
        let key_report = shape_modeled_key_report();
        let mut packet = modeled_kexinit_packet(false);
        let packet_len = (read_be_u32(&packet, 0).unwrap() as usize) + 4;
        let kex_name_offset = 5 + 1 + SSH_KEXINIT_COOKIE_BYTES + 4;
        packet[kex_name_offset] = b'x';

        let report = classify_ssh_service_readiness_with_remote_identification_and_kexinit(
            key_report,
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
        );

        assert_eq!(
            report.kexinit_result(),
            Some(SshKexinitNegotiationResult::UnsupportedAlgorithm)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitAlgorithmUnsupported)
        );
        assert!(!report.kexinit_cookie_generated_redacted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn kexinit_rejects_malformed_packet_and_size_limits() {
        let key_report = shape_modeled_key_report();
        let malformed = [0u8; 5];
        let malformed_report =
            classify_ssh_service_readiness_with_remote_identification_and_kexinit(
                key_report,
                SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
                SshRemoteIdentificationInputState::Complete,
                &malformed,
            );

        assert_eq!(
            malformed_report.kexinit_result(),
            Some(SshKexinitNegotiationResult::MalformedPacket)
        );
        assert!(
            malformed_report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitPacketMalformed)
        );

        let mut oversized = modeled_kexinit_packet(false);
        oversized[0..4].copy_from_slice(&((SSH_KEXINIT_PACKET_MAX_BYTES + 1) as u32).to_be_bytes());
        let oversized_report =
            classify_ssh_service_readiness_with_remote_identification_and_kexinit(
                key_report,
                SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
                SshRemoteIdentificationInputState::Complete,
                &oversized,
            );

        assert_eq!(
            oversized_report.kexinit_result(),
            Some(SshKexinitNegotiationResult::PacketOverLimit)
        );
        assert!(
            oversized_report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitPacketOverLimit)
        );
    }

    #[test_case]
    fn kexinit_rejects_list_over_limits() {
        let key_report = shape_modeled_key_report();
        let mut packet = modeled_kexinit_packet(false);
        let packet_len = (read_be_u32(&packet, 0).unwrap() as usize) + 4;
        let first_list_len_offset = 5 + 1 + SSH_KEXINIT_COOKIE_BYTES;
        packet[first_list_len_offset..first_list_len_offset + 4]
            .copy_from_slice(&((SSH_KEXINIT_NAME_LIST_MAX_BYTES + 1) as u32).to_be_bytes());

        let report = classify_ssh_service_readiness_with_remote_identification_and_kexinit(
            key_report,
            SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            SshRemoteIdentificationInputState::Complete,
            &packet[..packet_len],
        );

        assert_eq!(
            report.kexinit_result(),
            Some(SshKexinitNegotiationResult::ListOverLimit)
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::KexinitListOverLimit)
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
