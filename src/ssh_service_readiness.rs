//! SSH service readiness diagnostic shape.
//!
//! This module models fixed fail-closed service lifecycle diagnostics only. It
//! does not adopt an SSH dependency, perform SSH crypto, authenticate users,
//! attach a shell, inspect hardware, or expose secrets.

use zeroize::Zeroize;

use alloc::vec::Vec;
use signature::Verifier;
use ssh_key::{Algorithm, PublicKey, Signature, encoding::Decode};

use crate::{
    csprng::OperatorSeededCsprng,
    ssh_key_readiness::{
        AuthorizedKeyMatchLabel, AuthorizedKeyMatchReport, HostKeyPrivateMaterial,
        SshKeyReadinessLabel, SshKeyReadinessReport,
    },
    ssh_runtime_crypto::{
        SSH_USERAUTH_SESSION_IDENTIFIER_BYTES, SshRuntimeKexInput, SshRuntimeKexLabel,
        SshRuntimeKexResultKind, SshUserauthSessionIdentifier, perform_runtime_kex,
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
    EncryptedTransportDispatchModeled,
    EncryptedTransportPreauthState,
    EncryptedTransportServiceRequest,
    EncryptedTransportUserauthRequest,
    EncryptedTransportMessageUnsupported,
    EncryptedTransportPacketMalformed,
    EncryptedTransportPlaintextRejected,
    PreauthServiceRequestModeled,
    PreauthServiceUserauthRecognized,
    PreauthServiceUnsupported,
    PreauthServiceMalformed,
    PreauthUserauthRequestModeled,
    PreauthUserauthServiceRecognized,
    PreauthUserauthServiceUnsupported,
    PreauthUserauthMethodPublickeyModeled,
    PreauthUserauthMethodUnsupported,
    PreauthUserauthBeforeService,
    PreauthUserauthMalformed,
    UserauthSessionIdentifierAvailable,
    UserauthSessionIdentifierUnavailable,
    UserauthSessionIdentifierMalformed,
    UserauthSessionIdentifierOverLimit,
    PublickeyVerificationPrerequisiteOnly,
    PublickeyVerificationSignatureNotPresent,
    PublickeyVerificationSessionIdentifierMissing,
    PublickeyVerificationAuthorizedKeyMissingOrNoMatch,
    PublickeyVerificationAlgorithmUnsupported,
    PublickeyVerificationKeyBlobMalformed,
    PublickeyVerificationSignatureMalformed,
    PublickeyVerificationSignedDataMalformed,
    PublickeyVerificationSignatureRejected,
    PublickeyAuthResponsePkOkPrerequisiteOnly,
    PublickeyAuthResponseFailureSignatureValidSuccessDeferred,
    PublickeyAuthResponseFailureSignatureRejected,
    PublickeyAuthResponseFailureSignatureMalformed,
    PublickeyAuthResponseFailureAuthorizedKeyMissing,
    PublickeyAuthResponseFailureAuthorizedKeyNoMatch,
    PublickeyAuthResponseFailureRequestMalformed,
    PublickeyAuthResponseFailureAlgorithmUnsupported,
    PublickeyAuthResponseFailurePrerequisiteMissing,
    PublickeyAuthResponseFailurePolicyDisabled,
    PublickeyAuthResponseFailureRedactionSensitive,
    PublickeyAuthSuccessPrerequisiteOnly,
    PublickeyAuthSuccessAccountMatch,
    PublickeyAuthFailureAccountMismatch,
    PublickeyAuthFailureAccountPolicyDisabled,
    PublickeyAuthFailureAccountPrerequisiteMissing,
    PublickeyAuthFailureResponsePrerequisiteMissing,
    PublickeyAuthFailureSignatureInvalid,
    PublickeyAuthFailureAuthorizedKeyNoMatch,
    PublickeyAuthFailureRequestMalformed,
    PublickeyAuthFailureRedactionSensitive,
    AuthenticationSuccessLocalOnly,
    SessionChannelOpenPrerequisiteOnly,
    SessionChannelOpenSessionAccepted,
    SessionOpenLocalOnly,
    ChannelOpenLocalOnly,
    ShellUnattached,
    ShellAttached,
    SessionChannelOpenFailurePrerequisiteMissing,
    SessionChannelOpenFailurePolicyDisabled,
    SessionChannelOpenFailureWrongMessage,
    SessionChannelOpenFailureUnsupportedType,
    SessionChannelOpenFailureMalformed,
    SessionChannelOpenFailureDuplicate,
    SessionChannelOpenFailureRedactionSensitive,
    SessionShellRequestPrerequisiteOnly,
    SessionShellRequestShellType,
    SessionShellRequestWantReply,
    SessionShellRequestNoReply,
    SessionShellRequestFailureShellUnattached,
    SessionShellRequestFailureAuthenticationMissing,
    SessionShellRequestFailureChannelMissing,
    SessionShellRequestFailurePolicyDisabled,
    SessionShellRequestFailureDuplicate,
    SessionShellRequestFailureUnsupportedMessage,
    SessionShellRequestFailureUnsupportedRequestType,
    SessionShellRequestFailureRequestMalformed,
    SessionShellRequestFailureRedactionSensitive,
    SessionShellAttachmentPrerequisiteOnly,
    SessionShellAttachmentLocalExecutionOwned,
    SessionShellAttachmentLocalStdioOwned,
    SessionShellAttachmentWantReply,
    SessionShellAttachmentNoReply,
    SessionShellAttachmentChannelSuccess,
    SessionShellAttachmentFailurePolicyDisabled,
    SessionShellAttachmentFailureDuplicate,
    SessionShellAttachmentFailureLocalExecutionMissing,
    SessionShellAttachmentFailureLifecycleViolation,
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
            Self::EncryptedTransportDispatchModeled => {
                "sshservicediag-encrypted-transport-dispatch-modeled"
            }
            Self::EncryptedTransportPreauthState => {
                "sshservicediag-encrypted-transport-preauth-state"
            }
            Self::EncryptedTransportServiceRequest => {
                "sshservicediag-encrypted-transport-service-request"
            }
            Self::EncryptedTransportUserauthRequest => {
                "sshservicediag-encrypted-transport-userauth-request"
            }
            Self::EncryptedTransportMessageUnsupported => {
                "sshservicediag-encrypted-transport-message-unsupported"
            }
            Self::EncryptedTransportPacketMalformed => {
                "sshservicediag-encrypted-transport-packet-malformed"
            }
            Self::EncryptedTransportPlaintextRejected => {
                "sshservicediag-encrypted-transport-plaintext-rejected"
            }
            Self::PreauthServiceRequestModeled => "sshservicediag-preauth-service-request-modeled",
            Self::PreauthServiceUserauthRecognized => {
                "sshservicediag-preauth-service-userauth-recognized"
            }
            Self::PreauthServiceUnsupported => "sshservicediag-preauth-service-unsupported",
            Self::PreauthServiceMalformed => "sshservicediag-preauth-service-malformed",
            Self::PreauthUserauthRequestModeled => {
                "sshservicediag-preauth-userauth-request-modeled"
            }
            Self::PreauthUserauthServiceRecognized => {
                "sshservicediag-preauth-userauth-service-recognized"
            }
            Self::PreauthUserauthServiceUnsupported => {
                "sshservicediag-preauth-userauth-service-unsupported"
            }
            Self::PreauthUserauthMethodPublickeyModeled => {
                "sshservicediag-preauth-userauth-method-publickey-modeled"
            }
            Self::PreauthUserauthMethodUnsupported => {
                "sshservicediag-preauth-userauth-method-unsupported"
            }
            Self::PreauthUserauthBeforeService => "sshservicediag-preauth-userauth-before-service",
            Self::PreauthUserauthMalformed => "sshservicediag-preauth-userauth-malformed",
            Self::UserauthSessionIdentifierAvailable => {
                "sshservicediag-userauth-session-identifier-available"
            }
            Self::UserauthSessionIdentifierUnavailable => {
                "sshservicediag-userauth-session-identifier-unavailable"
            }
            Self::UserauthSessionIdentifierMalformed => {
                "sshservicediag-userauth-session-identifier-malformed"
            }
            Self::UserauthSessionIdentifierOverLimit => {
                "sshservicediag-userauth-session-identifier-over-limit"
            }
            Self::PublickeyVerificationPrerequisiteOnly => {
                "sshservicediag-publickey-verification-prerequisite-only"
            }
            Self::PublickeyVerificationSignatureNotPresent => {
                "sshservicediag-publickey-verification-signature-not-present"
            }
            Self::PublickeyVerificationSessionIdentifierMissing => {
                "sshservicediag-publickey-verification-session-id-missing"
            }
            Self::PublickeyVerificationAuthorizedKeyMissingOrNoMatch => {
                "sshservicediag-publickey-verification-authorized-key-missing-or-no-match"
            }
            Self::PublickeyVerificationAlgorithmUnsupported => {
                "sshservicediag-publickey-verification-algorithm-unsupported"
            }
            Self::PublickeyVerificationKeyBlobMalformed => {
                "sshservicediag-publickey-verification-key-blob-malformed"
            }
            Self::PublickeyVerificationSignatureMalformed => {
                "sshservicediag-publickey-verification-signature-malformed"
            }
            Self::PublickeyVerificationSignedDataMalformed => {
                "sshservicediag-publickey-verification-signed-data-malformed"
            }
            Self::PublickeyVerificationSignatureRejected => {
                "sshservicediag-publickey-verification-signature-rejected"
            }
            Self::PublickeyAuthResponsePkOkPrerequisiteOnly => {
                "sshservicediag-publickey-auth-response-pk-ok-prerequisite-only"
            }
            Self::PublickeyAuthResponseFailureSignatureValidSuccessDeferred => {
                "sshservicediag-publickey-auth-response-failure-signature-valid-success-deferred"
            }
            Self::PublickeyAuthResponseFailureSignatureRejected => {
                "sshservicediag-publickey-auth-response-failure-signature-rejected"
            }
            Self::PublickeyAuthResponseFailureSignatureMalformed => {
                "sshservicediag-publickey-auth-response-failure-signature-malformed"
            }
            Self::PublickeyAuthResponseFailureAuthorizedKeyMissing => {
                "sshservicediag-publickey-auth-response-failure-authorized-key-missing"
            }
            Self::PublickeyAuthResponseFailureAuthorizedKeyNoMatch => {
                "sshservicediag-publickey-auth-response-failure-authorized-key-no-match"
            }
            Self::PublickeyAuthResponseFailureRequestMalformed => {
                "sshservicediag-publickey-auth-response-failure-request-malformed"
            }
            Self::PublickeyAuthResponseFailureAlgorithmUnsupported => {
                "sshservicediag-publickey-auth-response-failure-algorithm-unsupported"
            }
            Self::PublickeyAuthResponseFailurePrerequisiteMissing => {
                "sshservicediag-publickey-auth-response-failure-prerequisite-missing"
            }
            Self::PublickeyAuthResponseFailurePolicyDisabled => {
                "sshservicediag-publickey-auth-response-failure-policy-disabled"
            }
            Self::PublickeyAuthResponseFailureRedactionSensitive => {
                "sshservicediag-publickey-auth-response-failure-redaction-sensitive"
            }
            Self::PublickeyAuthSuccessPrerequisiteOnly => {
                "sshservicediag-publickey-auth-success-prerequisite-only"
            }
            Self::PublickeyAuthSuccessAccountMatch => {
                "sshservicediag-publickey-auth-success-account-match"
            }
            Self::PublickeyAuthFailureAccountMismatch => {
                "sshservicediag-publickey-auth-failure-account-mismatch"
            }
            Self::PublickeyAuthFailureAccountPolicyDisabled => {
                "sshservicediag-publickey-auth-failure-account-policy-disabled"
            }
            Self::PublickeyAuthFailureAccountPrerequisiteMissing => {
                "sshservicediag-publickey-auth-failure-account-prerequisite-missing"
            }
            Self::PublickeyAuthFailureResponsePrerequisiteMissing => {
                "sshservicediag-publickey-auth-failure-response-prerequisite-missing"
            }
            Self::PublickeyAuthFailureSignatureInvalid => {
                "sshservicediag-publickey-auth-failure-signature-invalid"
            }
            Self::PublickeyAuthFailureAuthorizedKeyNoMatch => {
                "sshservicediag-publickey-auth-failure-authorized-key-no-match"
            }
            Self::PublickeyAuthFailureRequestMalformed => {
                "sshservicediag-publickey-auth-failure-request-malformed"
            }
            Self::PublickeyAuthFailureRedactionSensitive => {
                "sshservicediag-publickey-auth-failure-redaction-sensitive"
            }
            Self::AuthenticationSuccessLocalOnly => {
                "sshservicediag-authentication-success-local-only"
            }
            Self::SessionChannelOpenPrerequisiteOnly => {
                "sshservicediag-session-channel-open-prerequisite-only"
            }
            Self::SessionChannelOpenSessionAccepted => {
                "sshservicediag-session-channel-open-session-type"
            }
            Self::SessionOpenLocalOnly => "sshservicediag-session-open-local-only",
            Self::ChannelOpenLocalOnly => "sshservicediag-channel-open-local-only",
            Self::ShellUnattached => "sshservicediag-shell-unattached",
            Self::ShellAttached => "sshservicediag-shell-attached",
            Self::SessionChannelOpenFailurePrerequisiteMissing => {
                "sshservicediag-session-channel-open-failure-authentication-missing"
            }
            Self::SessionChannelOpenFailurePolicyDisabled => {
                "sshservicediag-session-channel-open-failure-policy-disabled"
            }
            Self::SessionChannelOpenFailureWrongMessage => {
                "sshservicediag-session-channel-open-failure-unsupported-message"
            }
            Self::SessionChannelOpenFailureUnsupportedType => {
                "sshservicediag-session-channel-open-failure-unsupported-type"
            }
            Self::SessionChannelOpenFailureMalformed => {
                "sshservicediag-session-channel-open-failure-request-malformed"
            }
            Self::SessionChannelOpenFailureDuplicate => {
                "sshservicediag-session-channel-open-failure-existing-channel"
            }
            Self::SessionChannelOpenFailureRedactionSensitive => {
                "sshservicediag-session-channel-open-failure-redaction-sensitive"
            }
            Self::SessionShellRequestPrerequisiteOnly => {
                "sshservicediag-session-shell-request-prerequisite-only"
            }
            Self::SessionShellRequestShellType => "sshservicediag-session-shell-request-shell-type",
            Self::SessionShellRequestWantReply => "sshservicediag-session-shell-request-want-reply",
            Self::SessionShellRequestNoReply => "sshservicediag-session-shell-request-no-reply",
            Self::SessionShellRequestFailureShellUnattached => {
                "sshservicediag-session-shell-request-failure-shell-unattached"
            }
            Self::SessionShellRequestFailureAuthenticationMissing => {
                "sshservicediag-session-shell-request-failure-authentication-missing"
            }
            Self::SessionShellRequestFailureChannelMissing => {
                "sshservicediag-session-shell-request-failure-channel-missing"
            }
            Self::SessionShellRequestFailurePolicyDisabled => {
                "sshservicediag-session-shell-request-failure-policy-disabled"
            }
            Self::SessionShellRequestFailureDuplicate => {
                "sshservicediag-session-shell-request-failure-duplicate"
            }
            Self::SessionShellRequestFailureUnsupportedMessage => {
                "sshservicediag-session-shell-request-failure-unsupported-message"
            }
            Self::SessionShellRequestFailureUnsupportedRequestType => {
                "sshservicediag-session-shell-request-failure-unsupported-request-type"
            }
            Self::SessionShellRequestFailureRequestMalformed => {
                "sshservicediag-session-shell-request-failure-request-malformed"
            }
            Self::SessionShellRequestFailureRedactionSensitive => {
                "sshservicediag-session-shell-request-failure-redaction-sensitive"
            }
            Self::SessionShellAttachmentPrerequisiteOnly => {
                "sshservicediag-session-shell-attachment-prerequisite-only"
            }
            Self::SessionShellAttachmentLocalExecutionOwned => {
                "sshservicediag-session-shell-attachment-local-execution-owned"
            }
            Self::SessionShellAttachmentLocalStdioOwned => {
                "sshservicediag-session-shell-attachment-local-stdio-owned"
            }
            Self::SessionShellAttachmentWantReply => {
                "sshservicediag-session-shell-attachment-want-reply"
            }
            Self::SessionShellAttachmentNoReply => {
                "sshservicediag-session-shell-attachment-no-reply"
            }
            Self::SessionShellAttachmentChannelSuccess => {
                "sshservicediag-session-shell-attachment-channel-success"
            }
            Self::SessionShellAttachmentFailurePolicyDisabled => {
                "sshservicediag-session-shell-attachment-failure-policy-disabled"
            }
            Self::SessionShellAttachmentFailureDuplicate => {
                "sshservicediag-session-shell-attachment-failure-duplicate"
            }
            Self::SessionShellAttachmentFailureLocalExecutionMissing => {
                "sshservicediag-session-shell-attachment-failure-local-execution-missing"
            }
            Self::SessionShellAttachmentFailureLifecycleViolation => {
                "sshservicediag-session-shell-attachment-failure-lifecycle-violation"
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
const SSH_MSG_SERVICE_REQUEST: u8 = 5;
const SSH_MSG_KEXINIT: u8 = 20;
const SSH_MSG_USERAUTH_REQUEST: u8 = 50;
const SSH_MSG_USERAUTH_FAILURE: u8 = 51;
const SSH_MSG_USERAUTH_SUCCESS: u8 = 52;
const SSH_MSG_USERAUTH_PK_OK: u8 = 60;
const SSH_MSG_CHANNEL_OPEN: u8 = 90;
const SSH_MSG_CHANNEL_OPEN_CONFIRMATION: u8 = 91;
const SSH_MSG_CHANNEL_OPEN_FAILURE: u8 = 92;
const SSH_MSG_CHANNEL_REQUEST: u8 = 98;
const SSH_MSG_CHANNEL_SUCCESS: u8 = 99;
const SSH_MSG_CHANNEL_FAILURE: u8 = 100;
const SSH_KEXINIT_COOKIE_BYTES: usize = 16;
const SSH_KEXINIT_LIST_COUNT: usize = 10;
const SSH_KEXINIT_REQUIRED_LIST_COUNT: usize = 8;
const SSH_KEXINIT_CLIENT_PACKET_BUFFER_BYTES: usize = SSH_KEXINIT_PACKET_MAX_BYTES + 4;
const SSH_ENCRYPTED_TRANSPORT_DISPATCH_MIN_PAYLOAD_BYTES: usize = 1;
const MAX_SSH_ENCRYPTED_TRANSPORT_DISPATCH_LABELS: usize = 6;
const MAX_SSH_PREAUTH_SERVICE_USERAUTH_LABELS: usize = 10;
const MAX_SSH_USERAUTH_SESSION_IDENTIFIER_LABELS: usize = 4;
const MAX_SSH_PUBLICKEY_VERIFICATION_LABELS: usize = 4;
const MAX_SSH_PUBLICKEY_AUTH_RESPONSE_LABELS: usize = 4;
const MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS: usize = 5;
const MAX_SSH_SESSION_CHANNEL_OPEN_LABELS: usize = 8;
const MAX_SSH_SESSION_SHELL_REQUEST_LABELS: usize = 10;
const MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS: usize = 14;
const SSH_PREAUTH_STRING_MAX_BYTES: usize = 256;
const SSH_PREAUTH_PUBLIC_KEY_BLOB_MAX_BYTES: usize = 512;
const SSH_PREAUTH_SIGNATURE_MAX_BYTES: usize = 512;
const SSH_CHANNEL_OPEN_PAYLOAD_MAX_BYTES: usize = 256;
const SSH_CHANNEL_OPEN_TYPE_MAX_BYTES: usize = 64;
const SSH_CHANNEL_REQUEST_PAYLOAD_MAX_BYTES: usize = 256;
const SSH_CHANNEL_REQUEST_TYPE_MAX_BYTES: usize = 64;
const SSH_KEXINIT_MODELED_COOKIE_SEED: [u8; crate::csprng::CSPRNG_SEED_BYTES] =
    *b"Talos-kexinit-cookie-redacted!!!";

const SSH_KEXINIT_POLICY_KEX: &[u8] = b"curve25519-sha256";
const SSH_KEXINIT_POLICY_HOST_KEY: &[u8] = b"ssh-ed25519";
const SSH_KEXINIT_POLICY_CIPHER: &[u8] = b"chacha20-poly1305@openssh.com";
const SSH_KEXINIT_POLICY_MAC: &[u8] = b"hmac-sha2-256";
const SSH_KEXINIT_POLICY_COMPRESSION: &[u8] = b"none";
const SSH_SERVICE_USERAUTH: &[u8] = b"ssh-userauth";
const SSH_SERVICE_CONNECTION: &[u8] = b"ssh-connection";
const SSH_AUTH_METHOD_PUBLICKEY: &[u8] = b"publickey";
const SSH_RESERVED_ACCOUNT_TALOS: &[u8] = b"talos";
const SSH_CHANNEL_TYPE_SESSION: &[u8] = b"session";
const SSH_CHANNEL_REQUEST_TYPE_SHELL: &[u8] = b"shell";

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshEncryptedTransportDispatchResult {
    ServiceRequest,
    UserauthRequest,
    UnsupportedMessage,
    MalformedPacket,
    InactiveEncryptedPacketState,
    PlaintextRejected,
    PacketCryptoFailed,
}

pub(crate) struct SshEncryptedTransportDispatchInput<'a> {
    pub(crate) encrypted_packet_state_active: bool,
    pub(crate) post_newkeys_plaintext_attempted: bool,
    pub(crate) packet_crypto_failed: bool,
    pub(crate) decrypted_payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshEncryptedTransportDispatchReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_ENCRYPTED_TRANSPORT_DISPATCH_LABELS],
    label_count: usize,
    result: SshEncryptedTransportDispatchResult,
    message_number: Option<u8>,
    encrypted_packet_state_active: bool,
}

impl SshEncryptedTransportDispatchReport {
    fn new(
        result: SshEncryptedTransportDispatchResult,
        primary_label: SshServiceReadinessLabel,
        message_number: Option<u8>,
        encrypted_packet_state_active: bool,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady;
                MAX_SSH_ENCRYPTED_TRANSPORT_DISPATCH_LABELS],
            label_count: 0,
            result,
            message_number,
            encrypted_packet_state_active,
        };
        match result {
            SshEncryptedTransportDispatchResult::ServiceRequest
            | SshEncryptedTransportDispatchResult::UserauthRequest
            | SshEncryptedTransportDispatchResult::UnsupportedMessage => {
                report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
                report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
                report.push(primary_label);
            }
            SshEncryptedTransportDispatchResult::PlaintextRejected => {
                report.push(primary_label);
                report.push(SshServiceReadinessLabel::EncryptedPacketCryptoFailed);
            }
            SshEncryptedTransportDispatchResult::PacketCryptoFailed
            | SshEncryptedTransportDispatchResult::MalformedPacket
            | SshEncryptedTransportDispatchResult::InactiveEncryptedPacketState => {
                report.push(primary_label);
            }
        }
        report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshEncryptedTransportDispatchResult {
        self.result
    }

    pub(crate) const fn message_number(self) -> Option<u8> {
        self.message_number
    }

    pub(crate) const fn encrypted_packet_state_active(self) -> bool {
        self.encrypted_packet_state_active
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
    }

    pub(crate) const fn shell_attached(self) -> bool {
        false
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        false
    }
}

pub(crate) fn classify_ssh_encrypted_transport_dispatch(
    input: SshEncryptedTransportDispatchInput<'_>,
) -> SshEncryptedTransportDispatchReport {
    if input.packet_crypto_failed {
        return SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::PacketCryptoFailed,
            SshServiceReadinessLabel::EncryptedPacketCryptoFailed,
            None,
            input.encrypted_packet_state_active,
        );
    }
    if input.post_newkeys_plaintext_attempted {
        return SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::PlaintextRejected,
            SshServiceReadinessLabel::EncryptedTransportPlaintextRejected,
            None,
            input.encrypted_packet_state_active,
        );
    }
    if !input.encrypted_packet_state_active {
        return SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::InactiveEncryptedPacketState,
            SshServiceReadinessLabel::NewkeysNotReady,
            None,
            false,
        );
    }
    if input.decrypted_payload.len() < SSH_ENCRYPTED_TRANSPORT_DISPATCH_MIN_PAYLOAD_BYTES {
        return SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::MalformedPacket,
            SshServiceReadinessLabel::EncryptedTransportPacketMalformed,
            None,
            true,
        );
    }

    let message_number = input.decrypted_payload[0];
    match message_number {
        SSH_MSG_SERVICE_REQUEST => SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::ServiceRequest,
            SshServiceReadinessLabel::EncryptedTransportServiceRequest,
            Some(message_number),
            true,
        ),
        SSH_MSG_USERAUTH_REQUEST => SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::UserauthRequest,
            SshServiceReadinessLabel::EncryptedTransportUserauthRequest,
            Some(message_number),
            true,
        ),
        _ => SshEncryptedTransportDispatchReport::new(
            SshEncryptedTransportDispatchResult::UnsupportedMessage,
            SshServiceReadinessLabel::EncryptedTransportMessageUnsupported,
            Some(message_number),
            true,
        ),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshPreauthServiceUserauthResult {
    ServiceUserauthRecognized,
    ServiceUnsupported,
    ServiceMalformed,
    UserauthPublickeyModeled,
    UserauthBeforeService,
    UserauthServiceUnsupported,
    UserauthMethodUnsupported,
    UserauthMalformed,
    DispatchUnsupportedMessage,
    DispatchMalformedPacket,
    InactiveEncryptedPacketState,
    PlaintextRejected,
    PacketCryptoFailed,
}

pub(crate) struct SshPreauthServiceUserauthInput<'a> {
    pub(crate) encrypted_packet_state_active: bool,
    pub(crate) post_newkeys_plaintext_attempted: bool,
    pub(crate) packet_crypto_failed: bool,
    pub(crate) service_userauth_requested: bool,
    pub(crate) decrypted_payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshPreauthServiceUserauthReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_PREAUTH_SERVICE_USERAUTH_LABELS],
    label_count: usize,
    result: SshPreauthServiceUserauthResult,
    message_number: Option<u8>,
    service_userauth_requested: bool,
    parsed_field_count: usize,
}

impl SshPreauthServiceUserauthReport {
    fn new(
        result: SshPreauthServiceUserauthResult,
        message_number: Option<u8>,
        service_userauth_requested: bool,
        parsed_field_count: usize,
    ) -> Self {
        Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_PREAUTH_SERVICE_USERAUTH_LABELS],
            label_count: 0,
            result,
            message_number,
            service_userauth_requested,
            parsed_field_count,
        }
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    fn finish(mut self) -> Self {
        self.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        self.push(SshServiceReadinessLabel::SessionUnimplemented);
        self.push(SshServiceReadinessLabel::NotReady);
        self
    }

    fn from_dispatch(dispatch: SshEncryptedTransportDispatchReport) -> Self {
        let result = match dispatch.result() {
            SshEncryptedTransportDispatchResult::ServiceRequest
            | SshEncryptedTransportDispatchResult::UserauthRequest => {
                SshPreauthServiceUserauthResult::DispatchMalformedPacket
            }
            SshEncryptedTransportDispatchResult::UnsupportedMessage => {
                SshPreauthServiceUserauthResult::DispatchUnsupportedMessage
            }
            SshEncryptedTransportDispatchResult::MalformedPacket => {
                SshPreauthServiceUserauthResult::DispatchMalformedPacket
            }
            SshEncryptedTransportDispatchResult::InactiveEncryptedPacketState => {
                SshPreauthServiceUserauthResult::InactiveEncryptedPacketState
            }
            SshEncryptedTransportDispatchResult::PlaintextRejected => {
                SshPreauthServiceUserauthResult::PlaintextRejected
            }
            SshEncryptedTransportDispatchResult::PacketCryptoFailed => {
                SshPreauthServiceUserauthResult::PacketCryptoFailed
            }
        };
        let mut report = Self::new(
            result,
            dispatch.message_number(),
            false,
            usize::from(dispatch.message_number().is_some()),
        );
        for label in dispatch.labels() {
            report.push(*label);
        }
        report
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshPreauthServiceUserauthResult {
        self.result
    }

    pub(crate) const fn message_number(self) -> Option<u8> {
        self.message_number
    }

    pub(crate) const fn service_userauth_requested(self) -> bool {
        self.service_userauth_requested
    }

    pub(crate) const fn parsed_field_count(self) -> usize {
        self.parsed_field_count
    }

    pub(crate) const fn service_success(self) -> bool {
        false
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
    }

    pub(crate) const fn shell_attached(self) -> bool {
        false
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        false
    }
}

pub(crate) fn classify_ssh_preauth_service_userauth(
    input: SshPreauthServiceUserauthInput<'_>,
) -> SshPreauthServiceUserauthReport {
    let dispatch = classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
        encrypted_packet_state_active: input.encrypted_packet_state_active,
        post_newkeys_plaintext_attempted: input.post_newkeys_plaintext_attempted,
        packet_crypto_failed: input.packet_crypto_failed,
        decrypted_payload: input.decrypted_payload,
    });
    match dispatch.result() {
        SshEncryptedTransportDispatchResult::ServiceRequest => {
            classify_ssh_service_request_payload(input.decrypted_payload)
        }
        SshEncryptedTransportDispatchResult::UserauthRequest => {
            classify_ssh_userauth_request_payload(
                input.decrypted_payload,
                input.service_userauth_requested,
            )
        }
        _ => SshPreauthServiceUserauthReport::from_dispatch(dispatch),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshUserauthSessionIdentifierResult {
    Available,
    Unavailable,
    Malformed,
    OverLimit,
}

#[derive(Clone, Copy)]
pub(crate) enum SshUserauthSessionIdentifierInput<'a> {
    Available(SshUserauthSessionIdentifier<'a>),
    Unavailable,
    Malformed { byte_len: usize },
    OverLimit { byte_len: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshUserauthSessionIdentifierReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_USERAUTH_SESSION_IDENTIFIER_LABELS],
    label_count: usize,
    result: SshUserauthSessionIdentifierResult,
    byte_len: Option<usize>,
}

impl SshUserauthSessionIdentifierReport {
    fn new(
        result: SshUserauthSessionIdentifierResult,
        label: SshServiceReadinessLabel,
        byte_len: Option<usize>,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady;
                MAX_SSH_USERAUTH_SESSION_IDENTIFIER_LABELS],
            label_count: 0,
            result,
            byte_len,
        };
        report.push(label);
        report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshUserauthSessionIdentifierResult {
        self.result
    }

    pub(crate) const fn byte_len(self) -> Option<usize> {
        self.byte_len
    }

    pub(crate) const fn session_identifier_available(self) -> bool {
        matches!(self.result, SshUserauthSessionIdentifierResult::Available)
    }

    pub(crate) const fn service_success(self) -> bool {
        false
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
    }

    pub(crate) const fn shell_attached(self) -> bool {
        false
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        false
    }
}

pub(crate) fn classify_ssh_userauth_session_identifier(
    input: SshUserauthSessionIdentifierInput<'_>,
) -> SshUserauthSessionIdentifierReport {
    match input {
        SshUserauthSessionIdentifierInput::Available(identifier) => {
            let byte_len = identifier.byte_len();
            if byte_len == SSH_USERAUTH_SESSION_IDENTIFIER_BYTES {
                SshUserauthSessionIdentifierReport::new(
                    SshUserauthSessionIdentifierResult::Available,
                    SshServiceReadinessLabel::UserauthSessionIdentifierAvailable,
                    Some(byte_len),
                )
            } else {
                SshUserauthSessionIdentifierReport::new(
                    SshUserauthSessionIdentifierResult::Malformed,
                    SshServiceReadinessLabel::UserauthSessionIdentifierMalformed,
                    Some(byte_len),
                )
            }
        }
        SshUserauthSessionIdentifierInput::Unavailable => SshUserauthSessionIdentifierReport::new(
            SshUserauthSessionIdentifierResult::Unavailable,
            SshServiceReadinessLabel::UserauthSessionIdentifierUnavailable,
            None,
        ),
        SshUserauthSessionIdentifierInput::Malformed { byte_len } => {
            SshUserauthSessionIdentifierReport::new(
                SshUserauthSessionIdentifierResult::Malformed,
                SshServiceReadinessLabel::UserauthSessionIdentifierMalformed,
                Some(byte_len),
            )
        }
        SshUserauthSessionIdentifierInput::OverLimit { byte_len } => {
            SshUserauthSessionIdentifierReport::new(
                SshUserauthSessionIdentifierResult::OverLimit,
                SshServiceReadinessLabel::UserauthSessionIdentifierOverLimit,
                Some(byte_len),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshPublickeyVerificationResult {
    VerifiedPrerequisiteOnly,
    SignatureNotPresent,
    SessionIdentifierMissing,
    AuthorizedKeyMissingOrNoMatch,
    AlgorithmUnsupported,
    KeyBlobMalformed,
    SignatureMalformed,
    SignedDataMalformed,
    SignatureRejected,
}

#[derive(Clone, Copy)]
pub(crate) enum SshPublickeyVerificationSessionInput<'a> {
    Available(SshUserauthSessionIdentifier<'a>),
    Unavailable,
}

pub(crate) struct SshPublickeyVerificationInput<'a> {
    pub(crate) decrypted_payload: &'a [u8],
    pub(crate) service_userauth_requested: bool,
    pub(crate) session_identifier: SshPublickeyVerificationSessionInput<'a>,
    pub(crate) authorized_key_match: &'a AuthorizedKeyMatchReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshPublickeyVerificationReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_PUBLICKEY_VERIFICATION_LABELS],
    label_count: usize,
    result: SshPublickeyVerificationResult,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
}

impl SshPublickeyVerificationReport {
    fn new(
        result: SshPublickeyVerificationResult,
        label: SshServiceReadinessLabel,
        request_public_key_blob_len: Option<usize>,
        signature_blob_len: Option<usize>,
        signed_data_len: Option<usize>,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_PUBLICKEY_VERIFICATION_LABELS],
            label_count: 0,
            result,
            request_public_key_blob_len,
            signature_blob_len,
            signed_data_len,
        };
        report.push(label);
        report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshPublickeyVerificationResult {
        self.result
    }

    pub(crate) const fn request_public_key_blob_len(self) -> Option<usize> {
        self.request_public_key_blob_len
    }

    pub(crate) const fn signature_blob_len(self) -> Option<usize> {
        self.signature_blob_len
    }

    pub(crate) const fn signed_data_len(self) -> Option<usize> {
        self.signed_data_len
    }

    pub(crate) const fn verified_prerequisite_only(self) -> bool {
        matches!(
            self.result,
            SshPublickeyVerificationResult::VerifiedPrerequisiteOnly
        )
    }

    pub(crate) const fn service_success(self) -> bool {
        false
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
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
}

pub(crate) fn classify_ssh_publickey_verification(
    input: SshPublickeyVerificationInput<'_>,
) -> SshPublickeyVerificationReport {
    let Some(request) = parse_ssh_publickey_verification_request(
        input.decrypted_payload,
        input.service_userauth_requested,
    ) else {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignedDataMalformed,
            SshServiceReadinessLabel::PublickeyVerificationSignedDataMalformed,
            None,
            None,
            None,
        );
    };

    if !request.signature_present {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignatureNotPresent,
            SshServiceReadinessLabel::PublickeyVerificationSignatureNotPresent,
            Some(request.public_key_blob.len()),
            None,
            None,
        );
    }
    if request.algorithm != SSH_KEXINIT_POLICY_HOST_KEY {
        return publickey_verification_report(
            SshPublickeyVerificationResult::AlgorithmUnsupported,
            SshServiceReadinessLabel::PublickeyVerificationAlgorithmUnsupported,
            Some(request.public_key_blob.len()),
            request.signature.map(<[u8]>::len),
            None,
        );
    }

    let Ok(public_key) = PublicKey::from_bytes(request.public_key_blob) else {
        return publickey_verification_report(
            SshPublickeyVerificationResult::KeyBlobMalformed,
            SshServiceReadinessLabel::PublickeyVerificationKeyBlobMalformed,
            Some(request.public_key_blob.len()),
            request.signature.map(<[u8]>::len),
            None,
        );
    };
    if public_key.algorithm() != Algorithm::Ed25519 || public_key.key_data().ed25519().is_none() {
        return publickey_verification_report(
            SshPublickeyVerificationResult::AlgorithmUnsupported,
            SshServiceReadinessLabel::PublickeyVerificationAlgorithmUnsupported,
            Some(request.public_key_blob.len()),
            request.signature.map(<[u8]>::len),
            None,
        );
    }

    if !input.authorized_key_match.match_prerequisite_only()
        || input.authorized_key_match.request_public_key_blob_len() != request.public_key_blob.len()
        || input.authorized_key_match.matched_public_key_blob_len()
            != Some(request.public_key_blob.len())
    {
        return publickey_verification_report(
            SshPublickeyVerificationResult::AuthorizedKeyMissingOrNoMatch,
            SshServiceReadinessLabel::PublickeyVerificationAuthorizedKeyMissingOrNoMatch,
            Some(request.public_key_blob.len()),
            request.signature.map(<[u8]>::len),
            None,
        );
    }

    let session_identifier = match input.session_identifier {
        SshPublickeyVerificationSessionInput::Available(identifier) => identifier,
        SshPublickeyVerificationSessionInput::Unavailable => {
            return publickey_verification_report(
                SshPublickeyVerificationResult::SessionIdentifierMissing,
                SshServiceReadinessLabel::PublickeyVerificationSessionIdentifierMissing,
                Some(request.public_key_blob.len()),
                request.signature.map(<[u8]>::len),
                None,
            );
        }
    };

    let Some(signature_blob) = request.signature else {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignatureMalformed,
            SshServiceReadinessLabel::PublickeyVerificationSignatureMalformed,
            Some(request.public_key_blob.len()),
            None,
            None,
        );
    };
    let mut signature_reader = signature_blob;
    let Ok(signature) = Signature::decode(&mut signature_reader) else {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignatureMalformed,
            SshServiceReadinessLabel::PublickeyVerificationSignatureMalformed,
            Some(request.public_key_blob.len()),
            Some(signature_blob.len()),
            None,
        );
    };
    if !signature_reader.is_empty() || signature.algorithm() != Algorithm::Ed25519 {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignatureMalformed,
            SshServiceReadinessLabel::PublickeyVerificationSignatureMalformed,
            Some(request.public_key_blob.len()),
            Some(signature_blob.len()),
            None,
        );
    }

    let signed_data = build_publickey_verification_signed_data(session_identifier, &request);
    let Ok(mut signed_data) = signed_data else {
        return publickey_verification_report(
            SshPublickeyVerificationResult::SignedDataMalformed,
            SshServiceReadinessLabel::PublickeyVerificationSignedDataMalformed,
            Some(request.public_key_blob.len()),
            Some(signature_blob.len()),
            None,
        );
    };
    let signed_data_len = signed_data.len();
    let verified = Verifier::<Signature>::verify(&public_key, &signed_data, &signature).is_ok();
    signed_data.zeroize();

    if verified {
        publickey_verification_report(
            SshPublickeyVerificationResult::VerifiedPrerequisiteOnly,
            SshServiceReadinessLabel::PublickeyVerificationPrerequisiteOnly,
            Some(request.public_key_blob.len()),
            Some(signature_blob.len()),
            Some(signed_data_len),
        )
    } else {
        publickey_verification_report(
            SshPublickeyVerificationResult::SignatureRejected,
            SshServiceReadinessLabel::PublickeyVerificationSignatureRejected,
            Some(request.public_key_blob.len()),
            Some(signature_blob.len()),
            Some(signed_data_len),
        )
    }
}

fn publickey_verification_report(
    result: SshPublickeyVerificationResult,
    label: SshServiceReadinessLabel,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
) -> SshPublickeyVerificationReport {
    SshPublickeyVerificationReport::new(
        result,
        label,
        request_public_key_blob_len,
        signature_blob_len,
        signed_data_len,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshPublickeyAuthResponseResult {
    UserauthPkOkPrerequisiteOnly,
    UserauthFailureSignatureValidSuccessDeferred,
    UserauthFailureSignatureRejected,
    UserauthFailureSignatureMalformed,
    UserauthFailureAuthorizedKeyMissing,
    UserauthFailureAuthorizedKeyNoMatch,
    UserauthFailureRequestMalformed,
    UserauthFailureAlgorithmUnsupported,
    UserauthFailurePrerequisiteMissing,
    UserauthFailurePolicyDisabled,
    UserauthFailureRedactionSensitive,
}

pub(crate) struct SshPublickeyAuthResponsePolicyInput<'a> {
    pub(crate) response_policy_enabled: bool,
    pub(crate) redaction_sensitive: bool,
    pub(crate) decrypted_payload: &'a [u8],
    pub(crate) service_userauth_requested: bool,
    pub(crate) session_identifier: SshPublickeyVerificationSessionInput<'a>,
    pub(crate) authorized_key_match: &'a AuthorizedKeyMatchReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshPublickeyAuthResponseReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_PUBLICKEY_AUTH_RESPONSE_LABELS],
    label_count: usize,
    result: SshPublickeyAuthResponseResult,
    response_message_number: u8,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
}

impl SshPublickeyAuthResponseReport {
    fn new(
        result: SshPublickeyAuthResponseResult,
        label: SshServiceReadinessLabel,
        response_message_number: u8,
        request_public_key_blob_len: Option<usize>,
        signature_blob_len: Option<usize>,
        signed_data_len: Option<usize>,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_PUBLICKEY_AUTH_RESPONSE_LABELS],
            label_count: 0,
            result,
            response_message_number,
            request_public_key_blob_len,
            signature_blob_len,
            signed_data_len,
        };
        report.push(label);
        report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshPublickeyAuthResponseResult {
        self.result
    }

    pub(crate) const fn response_message_number(self) -> u8 {
        self.response_message_number
    }

    pub(crate) const fn userauth_pk_ok(self) -> bool {
        matches!(
            self.result,
            SshPublickeyAuthResponseResult::UserauthPkOkPrerequisiteOnly
        )
    }

    pub(crate) const fn userauth_failure(self) -> bool {
        !self.userauth_pk_ok()
    }

    pub(crate) const fn request_public_key_blob_len(self) -> Option<usize> {
        self.request_public_key_blob_len
    }

    pub(crate) const fn signature_blob_len(self) -> Option<usize> {
        self.signature_blob_len
    }

    pub(crate) const fn signed_data_len(self) -> Option<usize> {
        self.signed_data_len
    }

    pub(crate) const fn service_success(self) -> bool {
        false
    }

    pub(crate) const fn authentication_success(self) -> bool {
        false
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
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
}

pub(crate) fn classify_ssh_publickey_auth_response_policy(
    input: SshPublickeyAuthResponsePolicyInput<'_>,
) -> SshPublickeyAuthResponseReport {
    if !input.response_policy_enabled {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailurePolicyDisabled,
            SshServiceReadinessLabel::PublickeyAuthResponseFailurePolicyDisabled,
            None,
            None,
            None,
        );
    }
    if input.redaction_sensitive {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureRedactionSensitive,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureRedactionSensitive,
            None,
            None,
            None,
        );
    }
    if !input.service_userauth_requested {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailurePrerequisiteMissing,
            SshServiceReadinessLabel::PublickeyAuthResponseFailurePrerequisiteMissing,
            None,
            None,
            None,
        );
    }

    let Some(request) = parse_ssh_publickey_verification_request(
        input.decrypted_payload,
        input.service_userauth_requested,
    ) else {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureRequestMalformed,
            None,
            None,
            None,
        );
    };
    let request_public_key_blob_len = Some(request.public_key_blob.len());
    let signature_blob_len = request.signature.map(<[u8]>::len);

    if request.algorithm != SSH_KEXINIT_POLICY_HOST_KEY {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureAlgorithmUnsupported,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureAlgorithmUnsupported,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    let Ok(public_key) = PublicKey::from_bytes(request.public_key_blob) else {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureRequestMalformed,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    };
    if public_key.algorithm() != Algorithm::Ed25519 || public_key.key_data().ed25519().is_none() {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureAlgorithmUnsupported,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureAlgorithmUnsupported,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    if let Some((result, label)) =
        publickey_auth_authorized_key_failure(input.authorized_key_match, request.public_key_blob)
    {
        return publickey_auth_response_failure(
            result,
            label,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    if matches!(
        input.session_identifier,
        SshPublickeyVerificationSessionInput::Unavailable
    ) {
        return publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailurePrerequisiteMissing,
            SshServiceReadinessLabel::PublickeyAuthResponseFailurePrerequisiteMissing,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    if !request.signature_present {
        return SshPublickeyAuthResponseReport::new(
            SshPublickeyAuthResponseResult::UserauthPkOkPrerequisiteOnly,
            SshServiceReadinessLabel::PublickeyAuthResponsePkOkPrerequisiteOnly,
            SSH_MSG_USERAUTH_PK_OK,
            request_public_key_blob_len,
            None,
            None,
        );
    }

    let verification = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
        decrypted_payload: input.decrypted_payload,
        service_userauth_requested: input.service_userauth_requested,
        session_identifier: input.session_identifier,
        authorized_key_match: input.authorized_key_match,
    });

    match verification.result() {
        SshPublickeyVerificationResult::VerifiedPrerequisiteOnly => {
            publickey_auth_response_failure(
                SshPublickeyAuthResponseResult::UserauthFailureSignatureValidSuccessDeferred,
                SshServiceReadinessLabel::PublickeyAuthResponseFailureSignatureValidSuccessDeferred,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::SignatureRejected => publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureSignatureRejected,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureSignatureRejected,
            verification.request_public_key_blob_len(),
            verification.signature_blob_len(),
            verification.signed_data_len(),
        ),
        SshPublickeyVerificationResult::SignatureMalformed
        | SshPublickeyVerificationResult::SignedDataMalformed => publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureSignatureMalformed,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureSignatureMalformed,
            verification.request_public_key_blob_len(),
            verification.signature_blob_len(),
            verification.signed_data_len(),
        ),
        SshPublickeyVerificationResult::AuthorizedKeyMissingOrNoMatch => {
            publickey_auth_response_failure(
                SshPublickeyAuthResponseResult::UserauthFailureAuthorizedKeyNoMatch,
                SshServiceReadinessLabel::PublickeyAuthResponseFailureAuthorizedKeyNoMatch,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::SessionIdentifierMissing => {
            publickey_auth_response_failure(
                SshPublickeyAuthResponseResult::UserauthFailurePrerequisiteMissing,
                SshServiceReadinessLabel::PublickeyAuthResponseFailurePrerequisiteMissing,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::AlgorithmUnsupported => publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureAlgorithmUnsupported,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureAlgorithmUnsupported,
            verification.request_public_key_blob_len(),
            verification.signature_blob_len(),
            verification.signed_data_len(),
        ),
        SshPublickeyVerificationResult::KeyBlobMalformed
        | SshPublickeyVerificationResult::SignatureNotPresent => publickey_auth_response_failure(
            SshPublickeyAuthResponseResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthResponseFailureRequestMalformed,
            verification.request_public_key_blob_len(),
            verification.signature_blob_len(),
            verification.signed_data_len(),
        ),
    }
}

fn publickey_auth_authorized_key_failure(
    authorized_key_match: &AuthorizedKeyMatchReport,
    public_key_blob: &[u8],
) -> Option<(SshPublickeyAuthResponseResult, SshServiceReadinessLabel)> {
    if authorized_key_match.match_prerequisite_only()
        && authorized_key_match.request_public_key_blob_len() == public_key_blob.len()
        && authorized_key_match.matched_public_key_blob_len() == Some(public_key_blob.len())
    {
        return None;
    }

    let label = match authorized_key_match.primary_label() {
        AuthorizedKeyMatchLabel::MissingOrMetadataInvalid
        | AuthorizedKeyMatchLabel::EmptyOrCommentOnly
        | AuthorizedKeyMatchLabel::LineMalformed
        | AuthorizedKeyMatchLabel::LineUnsupported
        | AuthorizedKeyMatchLabel::AlgorithmUnsupported
        | AuthorizedKeyMatchLabel::BlobMalformed => {
            SshServiceReadinessLabel::PublickeyAuthResponseFailureAuthorizedKeyMissing
        }
        AuthorizedKeyMatchLabel::NoMatch
        | AuthorizedKeyMatchLabel::MatchPrerequisiteOnly
        | AuthorizedKeyMatchLabel::AuthenticationUnimplemented
        | AuthorizedKeyMatchLabel::NotReady => {
            SshServiceReadinessLabel::PublickeyAuthResponseFailureAuthorizedKeyNoMatch
        }
    };
    let result = match label {
        SshServiceReadinessLabel::PublickeyAuthResponseFailureAuthorizedKeyMissing => {
            SshPublickeyAuthResponseResult::UserauthFailureAuthorizedKeyMissing
        }
        _ => SshPublickeyAuthResponseResult::UserauthFailureAuthorizedKeyNoMatch,
    };
    Some((result, label))
}

fn publickey_auth_response_failure(
    result: SshPublickeyAuthResponseResult,
    label: SshServiceReadinessLabel,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
) -> SshPublickeyAuthResponseReport {
    SshPublickeyAuthResponseReport::new(
        result,
        label,
        SSH_MSG_USERAUTH_FAILURE,
        request_public_key_blob_len,
        signature_blob_len,
        signed_data_len,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshPublickeyAuthSuccessAccountResult {
    UserauthSuccessPrerequisiteOnly,
    UserauthFailureAccountMismatch,
    UserauthFailureAccountPolicyDisabled,
    UserauthFailureAccountPrerequisiteMissing,
    UserauthFailureResponsePrerequisiteMissing,
    UserauthFailureSignatureInvalid,
    UserauthFailureAuthorizedKeyNoMatch,
    UserauthFailureRequestMalformed,
    UserauthFailureRedactionSensitive,
}

pub(crate) struct SshPublickeyAuthSuccessAccountInput<'a> {
    pub(crate) account_policy_enabled: bool,
    pub(crate) account_prerequisite_available: bool,
    pub(crate) redaction_sensitive: bool,
    pub(crate) decrypted_payload: &'a [u8],
    pub(crate) service_userauth_requested: bool,
    pub(crate) session_identifier: SshPublickeyVerificationSessionInput<'a>,
    pub(crate) authorized_key_match: &'a AuthorizedKeyMatchReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshPublickeyAuthSuccessAccountReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS],
    label_count: usize,
    result: SshPublickeyAuthSuccessAccountResult,
    response_message_number: u8,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
}

impl SshPublickeyAuthSuccessAccountReport {
    fn success(
        request_public_key_blob_len: Option<usize>,
        signature_blob_len: Option<usize>,
        signed_data_len: Option<usize>,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady;
                MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS],
            label_count: 0,
            result: SshPublickeyAuthSuccessAccountResult::UserauthSuccessPrerequisiteOnly,
            response_message_number: SSH_MSG_USERAUTH_SUCCESS,
            request_public_key_blob_len,
            signature_blob_len,
            signed_data_len,
        };
        report.push(SshServiceReadinessLabel::PublickeyAuthSuccessPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::PublickeyAuthSuccessAccountMatch);
        report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn failure(
        result: SshPublickeyAuthSuccessAccountResult,
        label: SshServiceReadinessLabel,
        request_public_key_blob_len: Option<usize>,
        signature_blob_len: Option<usize>,
        signed_data_len: Option<usize>,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady;
                MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS],
            label_count: 0,
            result,
            response_message_number: SSH_MSG_USERAUTH_FAILURE,
            request_public_key_blob_len,
            signature_blob_len,
            signed_data_len,
        };
        report.push(label);
        report.push(SshServiceReadinessLabel::SessionUnimplemented);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshPublickeyAuthSuccessAccountResult {
        self.result
    }

    pub(crate) const fn response_message_number(self) -> u8 {
        self.response_message_number
    }

    pub(crate) const fn userauth_success(self) -> bool {
        matches!(
            self.result,
            SshPublickeyAuthSuccessAccountResult::UserauthSuccessPrerequisiteOnly
        )
    }

    pub(crate) const fn userauth_failure(self) -> bool {
        !self.userauth_success()
    }

    pub(crate) const fn request_public_key_blob_len(self) -> Option<usize> {
        self.request_public_key_blob_len
    }

    pub(crate) const fn signature_blob_len(self) -> Option<usize> {
        self.signature_blob_len
    }

    pub(crate) const fn signed_data_len(self) -> Option<usize> {
        self.signed_data_len
    }

    pub(crate) const fn service_success(self) -> bool {
        false
    }

    pub(crate) const fn authentication_success(self) -> bool {
        self.userauth_success()
    }

    pub(crate) const fn session_count(self) -> usize {
        0
    }

    pub(crate) const fn channel_count(self) -> usize {
        0
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
}

pub(crate) fn classify_ssh_publickey_auth_success_account_policy(
    input: SshPublickeyAuthSuccessAccountInput<'_>,
) -> SshPublickeyAuthSuccessAccountReport {
    if input.redaction_sensitive {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRedactionSensitive,
            SshServiceReadinessLabel::PublickeyAuthFailureRedactionSensitive,
            None,
            None,
            None,
        );
    }
    if !input.account_policy_enabled {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountPolicyDisabled,
            SshServiceReadinessLabel::PublickeyAuthFailureAccountPolicyDisabled,
            None,
            None,
            None,
        );
    }
    if !input.account_prerequisite_available {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountPrerequisiteMissing,
            SshServiceReadinessLabel::PublickeyAuthFailureAccountPrerequisiteMissing,
            None,
            None,
            None,
        );
    }
    if !input.service_userauth_requested {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureResponsePrerequisiteMissing,
            SshServiceReadinessLabel::PublickeyAuthFailureResponsePrerequisiteMissing,
            None,
            None,
            None,
        );
    }

    let Some(request) = parse_ssh_publickey_verification_request(
        input.decrypted_payload,
        input.service_userauth_requested,
    ) else {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthFailureRequestMalformed,
            None,
            None,
            None,
        );
    };
    let request_public_key_blob_len = Some(request.public_key_blob.len());
    let signature_blob_len = request.signature.map(<[u8]>::len);

    if request.user_name != SSH_RESERVED_ACCOUNT_TALOS {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountMismatch,
            SshServiceReadinessLabel::PublickeyAuthFailureAccountMismatch,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }
    if !request.signature_present {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureSignatureInvalid,
            SshServiceReadinessLabel::PublickeyAuthFailureSignatureInvalid,
            request_public_key_blob_len,
            None,
            None,
        );
    }
    if request.algorithm != SSH_KEXINIT_POLICY_HOST_KEY {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthFailureRequestMalformed,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    let Ok(public_key) = PublicKey::from_bytes(request.public_key_blob) else {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthFailureRequestMalformed,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    };
    if public_key.algorithm() != Algorithm::Ed25519 || public_key.key_data().ed25519().is_none() {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed,
            SshServiceReadinessLabel::PublickeyAuthFailureRequestMalformed,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    if publickey_auth_authorized_key_failure(input.authorized_key_match, request.public_key_blob)
        .is_some()
    {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAuthorizedKeyNoMatch,
            SshServiceReadinessLabel::PublickeyAuthFailureAuthorizedKeyNoMatch,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }
    if matches!(
        input.session_identifier,
        SshPublickeyVerificationSessionInput::Unavailable
    ) {
        return publickey_auth_success_account_failure(
            SshPublickeyAuthSuccessAccountResult::UserauthFailureResponsePrerequisiteMissing,
            SshServiceReadinessLabel::PublickeyAuthFailureResponsePrerequisiteMissing,
            request_public_key_blob_len,
            signature_blob_len,
            None,
        );
    }

    let verification = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
        decrypted_payload: input.decrypted_payload,
        service_userauth_requested: input.service_userauth_requested,
        session_identifier: input.session_identifier,
        authorized_key_match: input.authorized_key_match,
    });

    match verification.result() {
        SshPublickeyVerificationResult::VerifiedPrerequisiteOnly => {
            SshPublickeyAuthSuccessAccountReport::success(
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::SignatureRejected
        | SshPublickeyVerificationResult::SignatureMalformed
        | SshPublickeyVerificationResult::SignedDataMalformed
        | SshPublickeyVerificationResult::SignatureNotPresent => {
            publickey_auth_success_account_failure(
                SshPublickeyAuthSuccessAccountResult::UserauthFailureSignatureInvalid,
                SshServiceReadinessLabel::PublickeyAuthFailureSignatureInvalid,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::AuthorizedKeyMissingOrNoMatch => {
            publickey_auth_success_account_failure(
                SshPublickeyAuthSuccessAccountResult::UserauthFailureAuthorizedKeyNoMatch,
                SshServiceReadinessLabel::PublickeyAuthFailureAuthorizedKeyNoMatch,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::SessionIdentifierMissing => {
            publickey_auth_success_account_failure(
                SshPublickeyAuthSuccessAccountResult::UserauthFailureResponsePrerequisiteMissing,
                SshServiceReadinessLabel::PublickeyAuthFailureResponsePrerequisiteMissing,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
        SshPublickeyVerificationResult::AlgorithmUnsupported
        | SshPublickeyVerificationResult::KeyBlobMalformed => {
            publickey_auth_success_account_failure(
                SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed,
                SshServiceReadinessLabel::PublickeyAuthFailureRequestMalformed,
                verification.request_public_key_blob_len(),
                verification.signature_blob_len(),
                verification.signed_data_len(),
            )
        }
    }
}

fn publickey_auth_success_account_failure(
    result: SshPublickeyAuthSuccessAccountResult,
    label: SshServiceReadinessLabel,
    request_public_key_blob_len: Option<usize>,
    signature_blob_len: Option<usize>,
    signed_data_len: Option<usize>,
) -> SshPublickeyAuthSuccessAccountReport {
    SshPublickeyAuthSuccessAccountReport::failure(
        result,
        label,
        request_public_key_blob_len,
        signature_blob_len,
        signed_data_len,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshSessionChannelOpenResult {
    ChannelOpenConfirmationPrerequisiteOnly,
    ChannelOpenFailurePrerequisiteMissing,
    ChannelOpenFailurePolicyDisabled,
    ChannelOpenFailureWrongMessage,
    ChannelOpenFailureUnsupportedType,
    ChannelOpenFailureMalformed,
    ChannelOpenFailureDuplicate,
    ChannelOpenFailureRedactionSensitive,
}

pub(crate) struct SshSessionChannelOpenInput<'a> {
    pub(crate) authentication_success: bool,
    pub(crate) channel_open_policy_enabled: bool,
    pub(crate) existing_session_channel: bool,
    pub(crate) redaction_sensitive: bool,
    pub(crate) decrypted_payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshSessionChannelOpenReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_SESSION_CHANNEL_OPEN_LABELS],
    label_count: usize,
    result: SshSessionChannelOpenResult,
    request_message_number: Option<u8>,
    response_message_number: u8,
    parsed_field_count: usize,
    channel_type_len: Option<usize>,
    authentication_success: bool,
}

impl SshSessionChannelOpenReport {
    fn success(channel_type_len: usize) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_CHANNEL_OPEN_LABELS],
            label_count: 0,
            result: SshSessionChannelOpenResult::ChannelOpenConfirmationPrerequisiteOnly,
            request_message_number: Some(SSH_MSG_CHANNEL_OPEN),
            response_message_number: SSH_MSG_CHANNEL_OPEN_CONFIRMATION,
            parsed_field_count: 5,
            channel_type_len: Some(channel_type_len),
            authentication_success: true,
        };
        report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        report.push(SshServiceReadinessLabel::SessionChannelOpenPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::SessionChannelOpenSessionAccepted);
        report.push(SshServiceReadinessLabel::SessionOpenLocalOnly);
        report.push(SshServiceReadinessLabel::ChannelOpenLocalOnly);
        report.push(SshServiceReadinessLabel::ShellUnattached);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn failure(
        result: SshSessionChannelOpenResult,
        label: SshServiceReadinessLabel,
        request_message_number: Option<u8>,
        parsed_field_count: usize,
        channel_type_len: Option<usize>,
        authentication_success: bool,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_CHANNEL_OPEN_LABELS],
            label_count: 0,
            result,
            request_message_number,
            response_message_number: SSH_MSG_CHANNEL_OPEN_FAILURE,
            parsed_field_count,
            channel_type_len,
            authentication_success,
        };
        if authentication_success {
            report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        } else {
            report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        }
        report.push(label);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshSessionChannelOpenResult {
        self.result
    }

    pub(crate) const fn request_message_number(self) -> Option<u8> {
        self.request_message_number
    }

    pub(crate) const fn response_message_number(self) -> u8 {
        self.response_message_number
    }

    pub(crate) const fn channel_open_confirmation(self) -> bool {
        matches!(
            self.result,
            SshSessionChannelOpenResult::ChannelOpenConfirmationPrerequisiteOnly
        )
    }

    pub(crate) const fn channel_open_failure(self) -> bool {
        !self.channel_open_confirmation()
    }

    pub(crate) const fn parsed_field_count(self) -> usize {
        self.parsed_field_count
    }

    pub(crate) const fn channel_type_len(self) -> Option<usize> {
        self.channel_type_len
    }

    pub(crate) const fn authentication_success(self) -> bool {
        self.authentication_success
    }

    pub(crate) const fn session_count(self) -> usize {
        if self.channel_open_confirmation() {
            1
        } else {
            0
        }
    }

    pub(crate) const fn channel_count(self) -> usize {
        if self.channel_open_confirmation() {
            1
        } else {
            0
        }
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
}

pub(crate) fn classify_ssh_session_channel_open(
    input: SshSessionChannelOpenInput<'_>,
) -> SshSessionChannelOpenReport {
    if input.redaction_sensitive {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureRedactionSensitive,
            SshServiceReadinessLabel::SessionChannelOpenFailureRedactionSensitive,
            None,
            0,
            None,
            input.authentication_success,
        );
    }
    if !input.authentication_success {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailurePrerequisiteMissing,
            SshServiceReadinessLabel::SessionChannelOpenFailurePrerequisiteMissing,
            None,
            0,
            None,
            false,
        );
    }
    if !input.channel_open_policy_enabled {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailurePolicyDisabled,
            SshServiceReadinessLabel::SessionChannelOpenFailurePolicyDisabled,
            None,
            0,
            None,
            true,
        );
    }
    if input.existing_session_channel {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureDuplicate,
            SshServiceReadinessLabel::SessionChannelOpenFailureDuplicate,
            None,
            0,
            None,
            true,
        );
    }
    if input.decrypted_payload.len() > SSH_CHANNEL_OPEN_PAYLOAD_MAX_BYTES {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureMalformed,
            SshServiceReadinessLabel::SessionChannelOpenFailureMalformed,
            input.decrypted_payload.first().copied(),
            usize::from(!input.decrypted_payload.is_empty()),
            None,
            true,
        );
    }
    let Some(message_number) = input.decrypted_payload.first().copied() else {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureMalformed,
            SshServiceReadinessLabel::SessionChannelOpenFailureMalformed,
            None,
            0,
            None,
            true,
        );
    };
    if message_number != SSH_MSG_CHANNEL_OPEN {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureWrongMessage,
            SshServiceReadinessLabel::SessionChannelOpenFailureWrongMessage,
            Some(message_number),
            1,
            None,
            true,
        );
    }

    let Some((channel_type, cursor)) = parse_ssh_binary_string_bounded(
        input.decrypted_payload,
        1,
        SSH_CHANNEL_OPEN_TYPE_MAX_BYTES,
    ) else {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureMalformed,
            SshServiceReadinessLabel::SessionChannelOpenFailureMalformed,
            Some(message_number),
            1,
            None,
            true,
        );
    };
    let channel_type_len = Some(channel_type.len());
    if channel_type != SSH_CHANNEL_TYPE_SESSION {
        return session_channel_open_failure(
            SshSessionChannelOpenResult::ChannelOpenFailureUnsupportedType,
            SshServiceReadinessLabel::SessionChannelOpenFailureUnsupportedType,
            Some(message_number),
            2,
            channel_type_len,
            true,
        );
    }
    let Some(cursor) = skip_ssh_u32(input.decrypted_payload, cursor) else {
        return malformed_session_channel_open(Some(message_number), 2, channel_type_len);
    };
    let Some(cursor) = skip_ssh_u32(input.decrypted_payload, cursor) else {
        return malformed_session_channel_open(Some(message_number), 3, channel_type_len);
    };
    let Some(cursor) = skip_ssh_u32(input.decrypted_payload, cursor) else {
        return malformed_session_channel_open(Some(message_number), 4, channel_type_len);
    };
    if cursor != input.decrypted_payload.len() {
        return malformed_session_channel_open(Some(message_number), 5, channel_type_len);
    }

    SshSessionChannelOpenReport::success(channel_type.len())
}

fn session_channel_open_failure(
    result: SshSessionChannelOpenResult,
    label: SshServiceReadinessLabel,
    request_message_number: Option<u8>,
    parsed_field_count: usize,
    channel_type_len: Option<usize>,
    authentication_success: bool,
) -> SshSessionChannelOpenReport {
    SshSessionChannelOpenReport::failure(
        result,
        label,
        request_message_number,
        parsed_field_count,
        channel_type_len,
        authentication_success,
    )
}

fn malformed_session_channel_open(
    request_message_number: Option<u8>,
    parsed_field_count: usize,
    channel_type_len: Option<usize>,
) -> SshSessionChannelOpenReport {
    session_channel_open_failure(
        SshSessionChannelOpenResult::ChannelOpenFailureMalformed,
        SshServiceReadinessLabel::SessionChannelOpenFailureMalformed,
        request_message_number,
        parsed_field_count,
        channel_type_len,
        true,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshSessionShellRequestResult {
    ChannelFailureShellUnattachedWantReply,
    NoReplyShellUnattached,
    ShellRequestFailureAuthenticationMissing,
    ShellRequestFailureChannelMissing,
    ShellRequestFailurePolicyDisabled,
    ShellRequestFailureDuplicate,
    ShellRequestFailureUnsupportedMessage,
    ShellRequestFailureUnsupportedRequestType,
    ShellRequestFailureMalformed,
    ShellRequestFailureRedactionSensitive,
}

pub(crate) struct SshSessionShellRequestInput<'a> {
    pub(crate) authentication_success: bool,
    pub(crate) open_session_channel: bool,
    pub(crate) shell_request_policy_enabled: bool,
    pub(crate) existing_shell_request_or_attachment: bool,
    pub(crate) redaction_sensitive: bool,
    pub(crate) decrypted_payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshSessionShellRequestReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_SESSION_SHELL_REQUEST_LABELS],
    label_count: usize,
    result: SshSessionShellRequestResult,
    request_message_number: Option<u8>,
    response_message_number: Option<u8>,
    parsed_field_count: usize,
    request_type_len: Option<usize>,
    want_reply: Option<bool>,
    authentication_success: bool,
    open_session_channel: bool,
}

impl SshSessionShellRequestReport {
    fn recognized(want_reply: bool, request_type_len: usize) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_SHELL_REQUEST_LABELS],
            label_count: 0,
            result: if want_reply {
                SshSessionShellRequestResult::ChannelFailureShellUnattachedWantReply
            } else {
                SshSessionShellRequestResult::NoReplyShellUnattached
            },
            request_message_number: Some(SSH_MSG_CHANNEL_REQUEST),
            response_message_number: if want_reply {
                Some(SSH_MSG_CHANNEL_FAILURE)
            } else {
                None
            },
            parsed_field_count: 4,
            request_type_len: Some(request_type_len),
            want_reply: Some(want_reply),
            authentication_success: true,
            open_session_channel: true,
        };
        report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        report.push(SshServiceReadinessLabel::SessionOpenLocalOnly);
        report.push(SshServiceReadinessLabel::ChannelOpenLocalOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestShellType);
        if want_reply {
            report.push(SshServiceReadinessLabel::SessionShellRequestWantReply);
        } else {
            report.push(SshServiceReadinessLabel::SessionShellRequestNoReply);
        }
        report.push(SshServiceReadinessLabel::SessionShellRequestFailureShellUnattached);
        report.push(SshServiceReadinessLabel::ShellUnattached);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn failure(
        result: SshSessionShellRequestResult,
        label: SshServiceReadinessLabel,
        request_message_number: Option<u8>,
        parsed_field_count: usize,
        request_type_len: Option<usize>,
        want_reply: Option<bool>,
        authentication_success: bool,
        open_session_channel: bool,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_SHELL_REQUEST_LABELS],
            label_count: 0,
            result,
            request_message_number,
            response_message_number: if want_reply == Some(true) {
                Some(SSH_MSG_CHANNEL_FAILURE)
            } else {
                None
            },
            parsed_field_count,
            request_type_len,
            want_reply,
            authentication_success,
            open_session_channel,
        };
        if authentication_success {
            report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        } else {
            report.push(SshServiceReadinessLabel::AuthenticationUnimplemented);
        }
        if open_session_channel {
            report.push(SshServiceReadinessLabel::SessionOpenLocalOnly);
            report.push(SshServiceReadinessLabel::ChannelOpenLocalOnly);
        }
        report.push(label);
        report.push(SshServiceReadinessLabel::ShellUnattached);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshSessionShellRequestResult {
        self.result
    }

    pub(crate) const fn request_message_number(self) -> Option<u8> {
        self.request_message_number
    }

    pub(crate) const fn response_message_number(self) -> Option<u8> {
        self.response_message_number
    }

    pub(crate) const fn channel_failure_response(self) -> bool {
        matches!(self.response_message_number, Some(SSH_MSG_CHANNEL_FAILURE))
    }

    pub(crate) const fn parsed_field_count(self) -> usize {
        self.parsed_field_count
    }

    pub(crate) const fn request_type_len(self) -> Option<usize> {
        self.request_type_len
    }

    pub(crate) const fn want_reply(self) -> Option<bool> {
        self.want_reply
    }

    pub(crate) const fn authentication_success(self) -> bool {
        self.authentication_success
    }

    pub(crate) const fn session_count(self) -> usize {
        if self.open_session_channel { 1 } else { 0 }
    }

    pub(crate) const fn channel_count(self) -> usize {
        if self.open_session_channel { 1 } else { 0 }
    }

    pub(crate) const fn shell_request_count(self) -> usize {
        match self.result {
            SshSessionShellRequestResult::ChannelFailureShellUnattachedWantReply
            | SshSessionShellRequestResult::NoReplyShellUnattached => 1,
            _ => 0,
        }
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
}

pub(crate) fn classify_ssh_session_shell_request(
    input: SshSessionShellRequestInput<'_>,
) -> SshSessionShellRequestReport {
    if input.redaction_sensitive {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureRedactionSensitive,
            SshServiceReadinessLabel::SessionShellRequestFailureRedactionSensitive,
            None,
            0,
            None,
            None,
            input.authentication_success,
            input.open_session_channel,
        );
    }
    if !input.authentication_success {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureAuthenticationMissing,
            SshServiceReadinessLabel::SessionShellRequestFailureAuthenticationMissing,
            None,
            0,
            None,
            None,
            false,
            input.open_session_channel,
        );
    }
    if !input.open_session_channel {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureChannelMissing,
            SshServiceReadinessLabel::SessionShellRequestFailureChannelMissing,
            None,
            0,
            None,
            None,
            true,
            false,
        );
    }
    if !input.shell_request_policy_enabled {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailurePolicyDisabled,
            SshServiceReadinessLabel::SessionShellRequestFailurePolicyDisabled,
            None,
            0,
            None,
            None,
            true,
            true,
        );
    }
    if input.existing_shell_request_or_attachment {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureDuplicate,
            SshServiceReadinessLabel::SessionShellRequestFailureDuplicate,
            None,
            0,
            None,
            None,
            true,
            true,
        );
    }
    if input.decrypted_payload.len() > SSH_CHANNEL_REQUEST_PAYLOAD_MAX_BYTES {
        return malformed_session_shell_request(
            input.decrypted_payload.first().copied(),
            usize::from(!input.decrypted_payload.is_empty()),
            None,
            None,
        );
    }
    let Some(message_number) = input.decrypted_payload.first().copied() else {
        return malformed_session_shell_request(None, 0, None, None);
    };
    if message_number != SSH_MSG_CHANNEL_REQUEST {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureUnsupportedMessage,
            SshServiceReadinessLabel::SessionShellRequestFailureUnsupportedMessage,
            Some(message_number),
            1,
            None,
            None,
            true,
            true,
        );
    }

    let Some(cursor) = skip_ssh_u32(input.decrypted_payload, 1) else {
        return malformed_session_shell_request(Some(message_number), 1, None, None);
    };
    let Some((request_type, cursor)) = parse_ssh_binary_string_bounded(
        input.decrypted_payload,
        cursor,
        SSH_CHANNEL_REQUEST_TYPE_MAX_BYTES,
    ) else {
        return malformed_session_shell_request(Some(message_number), 2, None, None);
    };
    let request_type_len = Some(request_type.len());
    if request_type != SSH_CHANNEL_REQUEST_TYPE_SHELL {
        return session_shell_request_failure(
            SshSessionShellRequestResult::ShellRequestFailureUnsupportedRequestType,
            SshServiceReadinessLabel::SessionShellRequestFailureUnsupportedRequestType,
            Some(message_number),
            3,
            request_type_len,
            None,
            true,
            true,
        );
    }
    let Some(want_reply_byte) = input.decrypted_payload.get(cursor).copied() else {
        return malformed_session_shell_request(Some(message_number), 3, request_type_len, None);
    };
    if !matches!(want_reply_byte, 0 | 1) {
        return malformed_session_shell_request(Some(message_number), 3, request_type_len, None);
    }
    let cursor = cursor + 1;
    if cursor != input.decrypted_payload.len() {
        return malformed_session_shell_request(
            Some(message_number),
            4,
            request_type_len,
            Some(want_reply_byte == 1),
        );
    }

    SshSessionShellRequestReport::recognized(
        want_reply_byte == 1,
        SSH_CHANNEL_REQUEST_TYPE_SHELL.len(),
    )
}

fn session_shell_request_failure(
    result: SshSessionShellRequestResult,
    label: SshServiceReadinessLabel,
    request_message_number: Option<u8>,
    parsed_field_count: usize,
    request_type_len: Option<usize>,
    want_reply: Option<bool>,
    authentication_success: bool,
    open_session_channel: bool,
) -> SshSessionShellRequestReport {
    SshSessionShellRequestReport::failure(
        result,
        label,
        request_message_number,
        parsed_field_count,
        request_type_len,
        want_reply,
        authentication_success,
        open_session_channel,
    )
}

fn malformed_session_shell_request(
    request_message_number: Option<u8>,
    parsed_field_count: usize,
    request_type_len: Option<usize>,
    want_reply: Option<bool>,
) -> SshSessionShellRequestReport {
    session_shell_request_failure(
        SshSessionShellRequestResult::ShellRequestFailureMalformed,
        SshServiceReadinessLabel::SessionShellRequestFailureRequestMalformed,
        request_message_number,
        parsed_field_count,
        request_type_len,
        want_reply,
        true,
        true,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshSessionShellAttachmentResult {
    ChannelSuccessShellAttachedWantReply,
    NoReplyShellAttached,
    ShellAttachmentFailureAuthenticationMissing,
    ShellAttachmentFailureChannelMissing,
    ShellAttachmentFailureShellRequestPolicyDisabled,
    ShellAttachmentFailureShellAttachmentPolicyDisabled,
    ShellAttachmentFailureDuplicateShellRequest,
    ShellAttachmentFailureDuplicateAttachment,
    ShellAttachmentFailureUnsupportedMessage,
    ShellAttachmentFailureUnsupportedRequestType,
    ShellAttachmentFailureMalformed,
    ShellAttachmentFailureRedactionSensitive,
    ShellAttachmentFailureLocalExecutionMissing,
    ShellAttachmentFailureLifecycleViolation,
}

pub(crate) struct SshSessionShellAttachmentInput<'a> {
    pub(crate) authentication_success: bool,
    pub(crate) open_session_channel: bool,
    pub(crate) shell_request_policy_enabled: bool,
    pub(crate) shell_attachment_policy_enabled: bool,
    pub(crate) existing_shell_request: bool,
    pub(crate) existing_shell_attachment: bool,
    pub(crate) redaction_sensitive: bool,
    pub(crate) local_process_session_owned: bool,
    pub(crate) local_stdio_descriptors_owned: bool,
    pub(crate) channel_lifecycle_open: bool,
    pub(crate) decrypted_payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshSessionShellAttachmentReport {
    labels: [SshServiceReadinessLabel; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS],
    label_count: usize,
    result: SshSessionShellAttachmentResult,
    request_message_number: Option<u8>,
    response_message_number: Option<u8>,
    parsed_field_count: usize,
    request_type_len: Option<usize>,
    want_reply: Option<bool>,
    authentication_success: bool,
    open_session_channel: bool,
    shell_request_recognized: bool,
    shell_attached: bool,
    local_process_session_owned: bool,
    local_stdio_descriptors_owned: bool,
    channel_lifecycle_open: bool,
}

impl SshSessionShellAttachmentReport {
    fn success(want_reply: bool, request_type_len: usize) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS],
            label_count: 0,
            result: if want_reply {
                SshSessionShellAttachmentResult::ChannelSuccessShellAttachedWantReply
            } else {
                SshSessionShellAttachmentResult::NoReplyShellAttached
            },
            request_message_number: Some(SSH_MSG_CHANNEL_REQUEST),
            response_message_number: if want_reply {
                Some(SSH_MSG_CHANNEL_SUCCESS)
            } else {
                None
            },
            parsed_field_count: 4,
            request_type_len: Some(request_type_len),
            want_reply: Some(want_reply),
            authentication_success: true,
            open_session_channel: true,
            shell_request_recognized: true,
            shell_attached: true,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
        };
        report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        report.push(SshServiceReadinessLabel::SessionOpenLocalOnly);
        report.push(SshServiceReadinessLabel::ChannelOpenLocalOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestShellType);
        report.push(if want_reply {
            SshServiceReadinessLabel::SessionShellRequestWantReply
        } else {
            SshServiceReadinessLabel::SessionShellRequestNoReply
        });
        report.push(SshServiceReadinessLabel::SessionShellAttachmentPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::SessionShellAttachmentLocalExecutionOwned);
        report.push(SshServiceReadinessLabel::SessionShellAttachmentLocalStdioOwned);
        report.push(if want_reply {
            SshServiceReadinessLabel::SessionShellAttachmentWantReply
        } else {
            SshServiceReadinessLabel::SessionShellAttachmentNoReply
        });
        if want_reply {
            report.push(SshServiceReadinessLabel::SessionShellAttachmentChannelSuccess);
        }
        report.push(SshServiceReadinessLabel::ShellAttached);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn recognized_failure(
        result: SshSessionShellAttachmentResult,
        label: SshServiceReadinessLabel,
        want_reply: bool,
        request_type_len: usize,
        local_process_session_owned: bool,
        local_stdio_descriptors_owned: bool,
        channel_lifecycle_open: bool,
    ) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS],
            label_count: 0,
            result,
            request_message_number: Some(SSH_MSG_CHANNEL_REQUEST),
            response_message_number: if want_reply {
                Some(SSH_MSG_CHANNEL_FAILURE)
            } else {
                None
            },
            parsed_field_count: 4,
            request_type_len: Some(request_type_len),
            want_reply: Some(want_reply),
            authentication_success: true,
            open_session_channel: true,
            shell_request_recognized: true,
            shell_attached: false,
            local_process_session_owned,
            local_stdio_descriptors_owned,
            channel_lifecycle_open,
        };
        report.push(SshServiceReadinessLabel::AuthenticationSuccessLocalOnly);
        report.push(SshServiceReadinessLabel::SessionOpenLocalOnly);
        report.push(SshServiceReadinessLabel::ChannelOpenLocalOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestPrerequisiteOnly);
        report.push(SshServiceReadinessLabel::SessionShellRequestShellType);
        report.push(if want_reply {
            SshServiceReadinessLabel::SessionShellRequestWantReply
        } else {
            SshServiceReadinessLabel::SessionShellRequestNoReply
        });
        report.push(SshServiceReadinessLabel::SessionShellAttachmentPrerequisiteOnly);
        report.push(label);
        report.push(SshServiceReadinessLabel::ShellUnattached);
        report.push(SshServiceReadinessLabel::NotReady);
        report
    }

    fn from_shell_request_failure(shell_request: SshSessionShellRequestReport) -> Self {
        let mut report = Self {
            labels: [SshServiceReadinessLabel::NotReady; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS],
            label_count: 0,
            result: shell_attachment_result_from_shell_request(shell_request.result()),
            request_message_number: shell_request.request_message_number(),
            response_message_number: shell_request.response_message_number(),
            parsed_field_count: shell_request.parsed_field_count(),
            request_type_len: shell_request.request_type_len(),
            want_reply: shell_request.want_reply(),
            authentication_success: shell_request.authentication_success(),
            open_session_channel: shell_request.channel_count() == 1,
            shell_request_recognized: false,
            shell_attached: false,
            local_process_session_owned: false,
            local_stdio_descriptors_owned: false,
            channel_lifecycle_open: false,
        };
        for label in shell_request.labels() {
            report.push(*label);
        }
        report
    }

    fn push(&mut self, label: SshServiceReadinessLabel) {
        self.labels[self.label_count] = label;
        self.label_count += 1;
    }

    pub(crate) fn labels(&self) -> &[SshServiceReadinessLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn result(self) -> SshSessionShellAttachmentResult {
        self.result
    }

    pub(crate) const fn request_message_number(self) -> Option<u8> {
        self.request_message_number
    }

    pub(crate) const fn response_message_number(self) -> Option<u8> {
        self.response_message_number
    }

    pub(crate) const fn channel_success_response(self) -> bool {
        matches!(self.response_message_number, Some(SSH_MSG_CHANNEL_SUCCESS))
    }

    pub(crate) const fn channel_failure_response(self) -> bool {
        matches!(self.response_message_number, Some(SSH_MSG_CHANNEL_FAILURE))
    }

    pub(crate) const fn parsed_field_count(self) -> usize {
        self.parsed_field_count
    }

    pub(crate) const fn request_type_len(self) -> Option<usize> {
        self.request_type_len
    }

    pub(crate) const fn want_reply(self) -> Option<bool> {
        self.want_reply
    }

    pub(crate) const fn authentication_success(self) -> bool {
        self.authentication_success
    }

    pub(crate) const fn session_count(self) -> usize {
        if self.open_session_channel { 1 } else { 0 }
    }

    pub(crate) const fn channel_count(self) -> usize {
        if self.open_session_channel { 1 } else { 0 }
    }

    pub(crate) const fn shell_request_count(self) -> usize {
        if self.shell_request_recognized { 1 } else { 0 }
    }

    pub(crate) const fn shell_attached(self) -> bool {
        self.shell_attached
    }

    pub(crate) const fn local_process_session_owned(self) -> bool {
        self.local_process_session_owned
    }

    pub(crate) const fn local_stdio_descriptors_owned(self) -> bool {
        self.local_stdio_descriptors_owned
    }

    pub(crate) const fn channel_lifecycle_open(self) -> bool {
        self.channel_lifecycle_open
    }

    pub(crate) const fn reachability_accepted(self) -> bool {
        false
    }

    pub(crate) const fn ssh_ready(self) -> bool {
        false
    }
}

pub(crate) fn classify_ssh_session_shell_attachment(
    input: SshSessionShellAttachmentInput<'_>,
) -> SshSessionShellAttachmentReport {
    let shell_request = classify_ssh_session_shell_request(SshSessionShellRequestInput {
        authentication_success: input.authentication_success,
        open_session_channel: input.open_session_channel,
        shell_request_policy_enabled: input.shell_request_policy_enabled,
        existing_shell_request_or_attachment: input.existing_shell_request,
        redaction_sensitive: input.redaction_sensitive,
        decrypted_payload: input.decrypted_payload,
    });

    match shell_request.result() {
        SshSessionShellRequestResult::ChannelFailureShellUnattachedWantReply
        | SshSessionShellRequestResult::NoReplyShellUnattached => {}
        _ => return SshSessionShellAttachmentReport::from_shell_request_failure(shell_request),
    }

    let want_reply = shell_request.want_reply().unwrap_or(false);
    let request_type_len = shell_request
        .request_type_len()
        .unwrap_or(SSH_CHANNEL_REQUEST_TYPE_SHELL.len());

    if !input.shell_attachment_policy_enabled {
        return SshSessionShellAttachmentReport::recognized_failure(
            SshSessionShellAttachmentResult::ShellAttachmentFailureShellAttachmentPolicyDisabled,
            SshServiceReadinessLabel::SessionShellAttachmentFailurePolicyDisabled,
            want_reply,
            request_type_len,
            false,
            false,
            input.channel_lifecycle_open,
        );
    }
    if input.existing_shell_attachment {
        return SshSessionShellAttachmentReport::recognized_failure(
            SshSessionShellAttachmentResult::ShellAttachmentFailureDuplicateAttachment,
            SshServiceReadinessLabel::SessionShellAttachmentFailureDuplicate,
            want_reply,
            request_type_len,
            false,
            false,
            input.channel_lifecycle_open,
        );
    }
    if !input.channel_lifecycle_open {
        return SshSessionShellAttachmentReport::recognized_failure(
            SshSessionShellAttachmentResult::ShellAttachmentFailureLifecycleViolation,
            SshServiceReadinessLabel::SessionShellAttachmentFailureLifecycleViolation,
            want_reply,
            request_type_len,
            false,
            false,
            false,
        );
    }
    if !input.local_process_session_owned || !input.local_stdio_descriptors_owned {
        return SshSessionShellAttachmentReport::recognized_failure(
            SshSessionShellAttachmentResult::ShellAttachmentFailureLocalExecutionMissing,
            SshServiceReadinessLabel::SessionShellAttachmentFailureLocalExecutionMissing,
            want_reply,
            request_type_len,
            input.local_process_session_owned,
            input.local_stdio_descriptors_owned,
            true,
        );
    }

    SshSessionShellAttachmentReport::success(want_reply, request_type_len)
}

const fn shell_attachment_result_from_shell_request(
    result: SshSessionShellRequestResult,
) -> SshSessionShellAttachmentResult {
    match result {
        SshSessionShellRequestResult::ChannelFailureShellUnattachedWantReply
        | SshSessionShellRequestResult::NoReplyShellUnattached => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureLocalExecutionMissing
        }
        SshSessionShellRequestResult::ShellRequestFailureAuthenticationMissing => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureAuthenticationMissing
        }
        SshSessionShellRequestResult::ShellRequestFailureChannelMissing => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureChannelMissing
        }
        SshSessionShellRequestResult::ShellRequestFailurePolicyDisabled => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureShellRequestPolicyDisabled
        }
        SshSessionShellRequestResult::ShellRequestFailureDuplicate => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureDuplicateShellRequest
        }
        SshSessionShellRequestResult::ShellRequestFailureUnsupportedMessage => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureUnsupportedMessage
        }
        SshSessionShellRequestResult::ShellRequestFailureUnsupportedRequestType => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureUnsupportedRequestType
        }
        SshSessionShellRequestResult::ShellRequestFailureMalformed => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureMalformed
        }
        SshSessionShellRequestResult::ShellRequestFailureRedactionSensitive => {
            SshSessionShellAttachmentResult::ShellAttachmentFailureRedactionSensitive
        }
    }
}

struct ParsedSshPublickeyVerificationRequest<'a> {
    user_name: &'a [u8],
    service: &'a [u8],
    method: &'a [u8],
    signature_present: bool,
    algorithm: &'a [u8],
    public_key_blob: &'a [u8],
    signature: Option<&'a [u8]>,
}

fn parse_ssh_publickey_verification_request<'a>(
    payload: &'a [u8],
    service_userauth_requested: bool,
) -> Option<ParsedSshPublickeyVerificationRequest<'a>> {
    if !service_userauth_requested || payload.first().copied()? != SSH_MSG_USERAUTH_REQUEST {
        return None;
    }

    let (user_name, cursor) =
        parse_ssh_binary_string_bounded(payload, 1, SSH_PREAUTH_STRING_MAX_BYTES)?;
    let (service, cursor) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)?;
    let (method, cursor) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)?;
    if service != SSH_SERVICE_CONNECTION || method != SSH_AUTH_METHOD_PUBLICKEY {
        return None;
    }
    let signature_present = payload.get(cursor).copied()?;
    if !matches!(signature_present, 0 | 1) {
        return None;
    }
    let cursor = cursor + 1;
    let (algorithm, cursor) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)?;
    let (public_key_blob, cursor) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_PUBLIC_KEY_BLOB_MAX_BYTES)?;
    if algorithm.is_empty() || public_key_blob.is_empty() {
        return None;
    }
    let (signature, cursor) = if signature_present == 1 {
        let (signature, cursor) =
            parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_SIGNATURE_MAX_BYTES)?;
        if signature.is_empty() {
            return None;
        }
        (Some(signature), cursor)
    } else {
        (None, cursor)
    };
    if cursor != payload.len() {
        return None;
    }

    Some(ParsedSshPublickeyVerificationRequest {
        user_name,
        service,
        method,
        signature_present: signature_present == 1,
        algorithm,
        public_key_blob,
        signature,
    })
}

fn build_publickey_verification_signed_data(
    session_identifier: SshUserauthSessionIdentifier<'_>,
    request: &ParsedSshPublickeyVerificationRequest<'_>,
) -> Result<Vec<u8>, ()> {
    let mut signed_data = Vec::new();
    push_ssh_string_to_vec(&mut signed_data, session_identifier.as_bytes())?;
    signed_data.push(SSH_MSG_USERAUTH_REQUEST);
    push_ssh_string_to_vec(&mut signed_data, request.user_name)?;
    push_ssh_string_to_vec(&mut signed_data, request.service)?;
    push_ssh_string_to_vec(&mut signed_data, request.method)?;
    signed_data.push(1);
    push_ssh_string_to_vec(&mut signed_data, request.algorithm)?;
    push_ssh_string_to_vec(&mut signed_data, request.public_key_blob)?;
    Ok(signed_data)
}

fn push_ssh_string_to_vec(output: &mut Vec<u8>, value: &[u8]) -> Result<(), ()> {
    let len = u32::try_from(value.len()).map_err(|_| ())?;
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn skip_ssh_u32(bytes: &[u8], cursor: usize) -> Option<usize> {
    let next = cursor.checked_add(4)?;
    if next > bytes.len() {
        return None;
    }
    Some(next)
}

fn classify_ssh_service_request_payload(payload: &[u8]) -> SshPreauthServiceUserauthReport {
    let result = parse_ssh_binary_string_bounded(payload, 1, SSH_PREAUTH_STRING_MAX_BYTES);
    let Some((service, cursor)) = result else {
        let mut report = SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::ServiceMalformed,
            Some(SSH_MSG_SERVICE_REQUEST),
            false,
            1,
        );
        report.push(SshServiceReadinessLabel::PreauthServiceMalformed);
        return report.finish();
    };
    if cursor != payload.len() {
        let mut report = SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::ServiceMalformed,
            Some(SSH_MSG_SERVICE_REQUEST),
            false,
            2,
        );
        report.push(SshServiceReadinessLabel::PreauthServiceMalformed);
        return report.finish();
    }

    let mut report = if service == SSH_SERVICE_USERAUTH {
        SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::ServiceUserauthRecognized,
            Some(SSH_MSG_SERVICE_REQUEST),
            true,
            2,
        )
    } else {
        SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::ServiceUnsupported,
            Some(SSH_MSG_SERVICE_REQUEST),
            false,
            2,
        )
    };
    report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
    report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
    report.push(SshServiceReadinessLabel::PreauthServiceRequestModeled);
    if service == SSH_SERVICE_USERAUTH {
        report.push(SshServiceReadinessLabel::PreauthServiceUserauthRecognized);
    } else {
        report.push(SshServiceReadinessLabel::PreauthServiceUnsupported);
    }
    report.finish()
}

fn classify_ssh_userauth_request_payload(
    payload: &[u8],
    service_userauth_requested: bool,
) -> SshPreauthServiceUserauthReport {
    if !service_userauth_requested {
        let mut report = SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::UserauthBeforeService,
            Some(SSH_MSG_USERAUTH_REQUEST),
            false,
            1,
        );
        report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
        report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
        report.push(SshServiceReadinessLabel::PreauthUserauthRequestModeled);
        report.push(SshServiceReadinessLabel::PreauthUserauthBeforeService);
        return report.finish();
    }

    let Some((_user_name, cursor)) =
        parse_ssh_binary_string_bounded(payload, 1, SSH_PREAUTH_STRING_MAX_BYTES)
    else {
        return malformed_userauth_report(service_userauth_requested, 1);
    };
    let Some((service, cursor)) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)
    else {
        return malformed_userauth_report(service_userauth_requested, 2);
    };
    let Some((method, cursor)) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)
    else {
        return malformed_userauth_report(service_userauth_requested, 3);
    };

    if service != SSH_SERVICE_CONNECTION {
        let mut report = SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::UserauthServiceUnsupported,
            Some(SSH_MSG_USERAUTH_REQUEST),
            service_userauth_requested,
            4,
        );
        report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
        report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
        report.push(SshServiceReadinessLabel::PreauthUserauthRequestModeled);
        report.push(SshServiceReadinessLabel::PreauthUserauthServiceUnsupported);
        return report.finish();
    }

    if method != SSH_AUTH_METHOD_PUBLICKEY {
        let mut report = SshPreauthServiceUserauthReport::new(
            SshPreauthServiceUserauthResult::UserauthMethodUnsupported,
            Some(SSH_MSG_USERAUTH_REQUEST),
            service_userauth_requested,
            4,
        );
        report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
        report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
        report.push(SshServiceReadinessLabel::PreauthUserauthRequestModeled);
        report.push(SshServiceReadinessLabel::PreauthUserauthServiceRecognized);
        report.push(SshServiceReadinessLabel::PreauthUserauthMethodUnsupported);
        return report.finish();
    }

    let Some(signature_present) = payload.get(cursor).copied() else {
        return malformed_userauth_report(service_userauth_requested, 4);
    };
    if !matches!(signature_present, 0 | 1) {
        return malformed_userauth_report(service_userauth_requested, 4);
    }
    let cursor = cursor + 1;
    let Some((algorithm, cursor)) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_STRING_MAX_BYTES)
    else {
        return malformed_userauth_report(service_userauth_requested, 5);
    };
    let Some((public_key_blob, cursor)) =
        parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_PUBLIC_KEY_BLOB_MAX_BYTES)
    else {
        return malformed_userauth_report(service_userauth_requested, 6);
    };
    if algorithm.is_empty() || public_key_blob.is_empty() {
        return malformed_userauth_report(service_userauth_requested, 6);
    }
    let cursor = if signature_present == 1 {
        let Some((signature, cursor)) =
            parse_ssh_binary_string_bounded(payload, cursor, SSH_PREAUTH_SIGNATURE_MAX_BYTES)
        else {
            return malformed_userauth_report(service_userauth_requested, 7);
        };
        if signature.is_empty() {
            return malformed_userauth_report(service_userauth_requested, 7);
        }
        cursor
    } else {
        cursor
    };
    if cursor != payload.len() {
        return malformed_userauth_report(service_userauth_requested, 7);
    }

    let mut report = SshPreauthServiceUserauthReport::new(
        SshPreauthServiceUserauthResult::UserauthPublickeyModeled,
        Some(SSH_MSG_USERAUTH_REQUEST),
        service_userauth_requested,
        usize::from(signature_present == 1) + 7,
    );
    report.push(SshServiceReadinessLabel::EncryptedTransportDispatchModeled);
    report.push(SshServiceReadinessLabel::EncryptedTransportPreauthState);
    report.push(SshServiceReadinessLabel::PreauthUserauthRequestModeled);
    report.push(SshServiceReadinessLabel::PreauthUserauthServiceRecognized);
    report.push(SshServiceReadinessLabel::PreauthUserauthMethodPublickeyModeled);
    report.finish()
}

fn malformed_userauth_report(
    service_userauth_requested: bool,
    parsed_field_count: usize,
) -> SshPreauthServiceUserauthReport {
    let mut report = SshPreauthServiceUserauthReport::new(
        SshPreauthServiceUserauthResult::UserauthMalformed,
        Some(SSH_MSG_USERAUTH_REQUEST),
        service_userauth_requested,
        parsed_field_count,
    );
    report.push(SshServiceReadinessLabel::PreauthUserauthMalformed);
    report.finish()
}

fn parse_ssh_binary_string_bounded(
    bytes: &[u8],
    cursor: usize,
    max_len: usize,
) -> Option<(&[u8], usize)> {
    let len = read_be_u32(bytes, cursor)? as usize;
    if len > max_len {
        return None;
    }
    let start = cursor + 4;
    let end = start.checked_add(len)?;
    if end > bytes.len() {
        return None;
    }
    Some((&bytes[start..end], end))
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
        ssh_key_readiness::{
            self, AuthorizedKeyMatchReport, HostKeyMaterialMetadata, SshKeyReadinessSnapshot,
        },
        ssh_runtime_crypto::{SshRuntimeKexReady, SshRuntimeKexResult, SshRuntimeKexResultKind},
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

    fn dispatch_label_names(
        report: &SshEncryptedTransportDispatchReport,
    ) -> [&'static str; MAX_SSH_ENCRYPTED_TRANSPORT_DISPATCH_LABELS] {
        let mut labels = [""; MAX_SSH_ENCRYPTED_TRANSPORT_DISPATCH_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn preauth_label_names(
        report: &SshPreauthServiceUserauthReport,
    ) -> [&'static str; MAX_SSH_PREAUTH_SERVICE_USERAUTH_LABELS] {
        let mut labels = [""; MAX_SSH_PREAUTH_SERVICE_USERAUTH_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn userauth_session_identifier_label_names(
        report: &SshUserauthSessionIdentifierReport,
    ) -> [&'static str; MAX_SSH_USERAUTH_SESSION_IDENTIFIER_LABELS] {
        let mut labels = [""; MAX_SSH_USERAUTH_SESSION_IDENTIFIER_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn publickey_verification_label_names(
        report: &SshPublickeyVerificationReport,
    ) -> [&'static str; MAX_SSH_PUBLICKEY_VERIFICATION_LABELS] {
        let mut labels = [""; MAX_SSH_PUBLICKEY_VERIFICATION_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn publickey_auth_response_label_names(
        report: &SshPublickeyAuthResponseReport,
    ) -> [&'static str; MAX_SSH_PUBLICKEY_AUTH_RESPONSE_LABELS] {
        let mut labels = [""; MAX_SSH_PUBLICKEY_AUTH_RESPONSE_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn publickey_auth_success_account_label_names(
        report: &SshPublickeyAuthSuccessAccountReport,
    ) -> [&'static str; MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS] {
        let mut labels = [""; MAX_SSH_PUBLICKEY_AUTH_SUCCESS_ACCOUNT_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn session_channel_open_label_names(
        report: &SshSessionChannelOpenReport,
    ) -> [&'static str; MAX_SSH_SESSION_CHANNEL_OPEN_LABELS] {
        let mut labels = [""; MAX_SSH_SESSION_CHANNEL_OPEN_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn session_shell_request_label_names(
        report: &SshSessionShellRequestReport,
    ) -> [&'static str; MAX_SSH_SESSION_SHELL_REQUEST_LABELS] {
        let mut labels = [""; MAX_SSH_SESSION_SHELL_REQUEST_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn session_shell_attachment_label_names(
        report: &SshSessionShellAttachmentReport,
    ) -> [&'static str; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS] {
        let mut labels = [""; MAX_SSH_SESSION_SHELL_ATTACHMENT_LABELS];
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

    fn write_ssh_string(output: &mut [u8], cursor: usize, value: &[u8]) -> usize {
        output[cursor..cursor + 4].copy_from_slice(&(value.len() as u32).to_be_bytes());
        let start = cursor + 4;
        output[start..start + value.len()].copy_from_slice(value);
        start + value.len()
    }

    fn service_request_payload(service: &[u8]) -> ([u8; 64], usize) {
        let mut payload = [0u8; 64];
        payload[0] = SSH_MSG_SERVICE_REQUEST;
        let cursor = write_ssh_string(&mut payload, 1, service);
        (payload, cursor)
    }

    fn userauth_publickey_payload(signature_present: bool) -> ([u8; 192], usize) {
        let mut payload = [0u8; 192];
        payload[0] = SSH_MSG_USERAUTH_REQUEST;
        let mut cursor = 1usize;
        cursor = write_ssh_string(&mut payload, cursor, b"fixture-user");
        cursor = write_ssh_string(&mut payload, cursor, SSH_SERVICE_CONNECTION);
        cursor = write_ssh_string(&mut payload, cursor, SSH_AUTH_METHOD_PUBLICKEY);
        payload[cursor] = u8::from(signature_present);
        cursor += 1;
        cursor = write_ssh_string(&mut payload, cursor, b"ssh-ed25519");
        cursor = write_ssh_string(&mut payload, cursor, b"public-fixture-key");
        if signature_present {
            cursor = write_ssh_string(&mut payload, cursor, b"public-fixture-signature");
        }
        (payload, cursor)
    }

    fn userauth_publickey_verification_payload(
        signature_present: bool,
        algorithm: &[u8],
        public_key_blob: &[u8],
        signature_blob: Option<&[u8]>,
    ) -> Vec<u8> {
        userauth_publickey_verification_payload_for_user(
            b"fixture-user",
            signature_present,
            algorithm,
            public_key_blob,
            signature_blob,
        )
    }

    fn userauth_publickey_verification_payload_for_user(
        user_name: &[u8],
        signature_present: bool,
        algorithm: &[u8],
        public_key_blob: &[u8],
        signature_blob: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(SSH_MSG_USERAUTH_REQUEST);
        push_ssh_string_to_vec(&mut payload, user_name).unwrap();
        push_ssh_string_to_vec(&mut payload, SSH_SERVICE_CONNECTION).unwrap();
        push_ssh_string_to_vec(&mut payload, SSH_AUTH_METHOD_PUBLICKEY).unwrap();
        payload.push(u8::from(signature_present));
        push_ssh_string_to_vec(&mut payload, algorithm).unwrap();
        push_ssh_string_to_vec(&mut payload, public_key_blob).unwrap();
        if let Some(signature_blob) = signature_blob {
            push_ssh_string_to_vec(&mut payload, signature_blob).unwrap();
        }
        payload
    }

    fn signed_publickey_data_for_user_test(
        user_name: &[u8],
        session_identifier: SshUserauthSessionIdentifier<'_>,
        algorithm: &[u8],
        public_key_blob: &[u8],
    ) -> Vec<u8> {
        let mut signed_data = Vec::new();
        push_ssh_string_to_vec(&mut signed_data, session_identifier.as_bytes()).unwrap();
        signed_data.push(SSH_MSG_USERAUTH_REQUEST);
        push_ssh_string_to_vec(&mut signed_data, user_name).unwrap();
        push_ssh_string_to_vec(&mut signed_data, SSH_SERVICE_CONNECTION).unwrap();
        push_ssh_string_to_vec(&mut signed_data, SSH_AUTH_METHOD_PUBLICKEY).unwrap();
        signed_data.push(1);
        push_ssh_string_to_vec(&mut signed_data, algorithm).unwrap();
        push_ssh_string_to_vec(&mut signed_data, public_key_blob).unwrap();
        signed_data
    }

    fn public_fixture_public_key_blob() -> Vec<u8> {
        ssh_key_readiness::public_fixture_host_key_private_material()
            .public_key_blob()
            .expect("public fixture public key blob encodes")
    }

    fn public_fixture_signature_blob(
        session_identifier: SshUserauthSessionIdentifier<'_>,
        public_key_blob: &[u8],
    ) -> Vec<u8> {
        public_fixture_signature_blob_for_user(b"fixture-user", session_identifier, public_key_blob)
    }

    fn public_fixture_signature_blob_for_user(
        user_name: &[u8],
        session_identifier: SshUserauthSessionIdentifier<'_>,
        public_key_blob: &[u8],
    ) -> Vec<u8> {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut signed_data = signed_publickey_data_for_user_test(
            user_name,
            session_identifier,
            SSH_KEXINIT_POLICY_HOST_KEY,
            public_key_blob,
        );
        let signature = host_key
            .sign_exchange_hash(&signed_data)
            .expect("public fixture signs publickey request data");
        signed_data.zeroize();
        signature
            .encoded_blob()
            .expect("public fixture signature blob encodes")
    }

    fn userauth_unsupported_payload(service: &[u8], method: &[u8]) -> ([u8; 128], usize) {
        let mut payload = [0u8; 128];
        payload[0] = SSH_MSG_USERAUTH_REQUEST;
        let mut cursor = 1usize;
        cursor = write_ssh_string(&mut payload, cursor, b"fixture-user");
        cursor = write_ssh_string(&mut payload, cursor, service);
        cursor = write_ssh_string(&mut payload, cursor, method);
        (payload, cursor)
    }

    fn session_channel_open_payload(channel_type: &[u8]) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(SSH_MSG_CHANNEL_OPEN);
        push_ssh_string_to_vec(&mut payload, channel_type).unwrap();
        payload.extend_from_slice(&0u32.to_be_bytes());
        payload.extend_from_slice(&4096u32.to_be_bytes());
        payload.extend_from_slice(&1024u32.to_be_bytes());
        payload
    }

    fn session_shell_request_payload(request_type: &[u8], want_reply: bool) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(SSH_MSG_CHANNEL_REQUEST);
        payload.extend_from_slice(&0u32.to_be_bytes());
        push_ssh_string_to_vec(&mut payload, request_type).unwrap();
        payload.push(u8::from(want_reply));
        payload
    }

    fn runtime_kex_ready_for_userauth() -> SshRuntimeKexReady {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let client_packet = modeled_kexinit_packet(false);
        let server_packet = modeled_kexinit_packet(false);
        let client_len = (read_be_u32(&client_packet, 0).unwrap() as usize) + 4;
        let server_len = (read_be_u32(&server_packet, 0).unwrap() as usize) + 4;
        let peer_public_key = [
            9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: SSH_LOCAL_TRANSPORT_REMOTE_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_packet[..client_len],
            server_kexinit_packet: &server_packet[..server_len],
            peer_public_key: &peer_public_key,
            host_key: Some(&host_key),
            csprng: &mut csprng,
        });
        let SshRuntimeKexResult::Ready(ready) = result else {
            panic!("runtime KEX should expose a userauth session-id handle");
        };
        ready
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
    fn encrypted_transport_dispatch_routes_preauth_message_numbers_only_when_active() {
        let service =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                decrypted_payload: &[SSH_MSG_SERVICE_REQUEST, 0, 0, 0],
            });
        assert_eq!(
            service.result(),
            SshEncryptedTransportDispatchResult::ServiceRequest
        );
        assert_eq!(service.message_number(), Some(SSH_MSG_SERVICE_REQUEST));
        assert_eq!(
            &dispatch_label_names(&service)[..service.labels().len()],
            &[
                "sshservicediag-encrypted-transport-dispatch-modeled",
                "sshservicediag-encrypted-transport-preauth-state",
                "sshservicediag-encrypted-transport-service-request",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(service.encrypted_packet_state_active());
        assert!(!service.authentication_success());
        assert_eq!(service.session_count(), 0);
        assert_eq!(service.channel_count(), 0);
        assert!(!service.shell_attached());
        assert!(!service.ssh_ready());

        let userauth =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                decrypted_payload: &[SSH_MSG_USERAUTH_REQUEST, 0, 0, 0],
            });
        assert_eq!(
            userauth.result(),
            SshEncryptedTransportDispatchResult::UserauthRequest
        );
        assert_eq!(userauth.message_number(), Some(SSH_MSG_USERAUTH_REQUEST));
        assert_eq!(
            &dispatch_label_names(&userauth)[..userauth.labels().len()],
            &[
                "sshservicediag-encrypted-transport-dispatch-modeled",
                "sshservicediag-encrypted-transport-preauth-state",
                "sshservicediag-encrypted-transport-userauth-request",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!userauth.authentication_success());
        assert_eq!(userauth.session_count(), 0);
        assert_eq!(userauth.channel_count(), 0);
        assert!(!userauth.shell_attached());
        assert!(!userauth.ssh_ready());
    }

    #[test_case]
    fn encrypted_transport_dispatch_fails_closed_without_retaining_payload_material() {
        let empty = classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
            encrypted_packet_state_active: true,
            post_newkeys_plaintext_attempted: false,
            packet_crypto_failed: false,
            decrypted_payload: &[],
        });
        assert_eq!(
            empty.result(),
            SshEncryptedTransportDispatchResult::MalformedPacket
        );
        assert_eq!(empty.message_number(), None);
        assert_eq!(
            &dispatch_label_names(&empty)[..empty.labels().len()],
            &[
                "sshservicediag-encrypted-transport-packet-malformed",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );

        let unsupported =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                decrypted_payload: &[99, 1, 2, 3],
            });
        assert_eq!(
            unsupported.result(),
            SshEncryptedTransportDispatchResult::UnsupportedMessage
        );
        assert_eq!(unsupported.message_number(), Some(99));
        assert_eq!(
            &dispatch_label_names(&unsupported)[..unsupported.labels().len()],
            &[
                "sshservicediag-encrypted-transport-dispatch-modeled",
                "sshservicediag-encrypted-transport-preauth-state",
                "sshservicediag-encrypted-transport-message-unsupported",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );

        let inactive =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: false,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                decrypted_payload: &[SSH_MSG_SERVICE_REQUEST],
            });
        assert_eq!(
            inactive.result(),
            SshEncryptedTransportDispatchResult::InactiveEncryptedPacketState
        );
        assert_eq!(inactive.message_number(), None);
        assert_eq!(
            &dispatch_label_names(&inactive)[..inactive.labels().len()],
            &[
                "sshservicediag-newkeys-not-ready",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );

        let plaintext =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: true,
                packet_crypto_failed: false,
                decrypted_payload: &[SSH_MSG_SERVICE_REQUEST],
            });
        assert_eq!(
            plaintext.result(),
            SshEncryptedTransportDispatchResult::PlaintextRejected
        );
        assert_eq!(
            &dispatch_label_names(&plaintext)[..plaintext.labels().len()],
            &[
                "sshservicediag-encrypted-transport-plaintext-rejected",
                "sshservicediag-encrypted-packet-crypto-failed",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );

        let crypto_failed =
            classify_ssh_encrypted_transport_dispatch(SshEncryptedTransportDispatchInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: true,
                decrypted_payload: &[SSH_MSG_USERAUTH_REQUEST],
            });
        assert_eq!(
            crypto_failed.result(),
            SshEncryptedTransportDispatchResult::PacketCryptoFailed
        );
        assert_eq!(crypto_failed.message_number(), None);
        assert_eq!(
            &dispatch_label_names(&crypto_failed)[..crypto_failed.labels().len()],
            &[
                "sshservicediag-encrypted-packet-crypto-failed",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!crypto_failed.ssh_ready());
    }

    #[test_case]
    fn preauth_service_userauth_recognizes_service_request_without_service_success() {
        let (payload, len) = service_request_payload(SSH_SERVICE_USERAUTH);
        let report = classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
            encrypted_packet_state_active: true,
            post_newkeys_plaintext_attempted: false,
            packet_crypto_failed: false,
            service_userauth_requested: false,
            decrypted_payload: &payload[..len],
        });

        assert_eq!(
            report.result(),
            SshPreauthServiceUserauthResult::ServiceUserauthRecognized
        );
        assert_eq!(report.message_number(), Some(SSH_MSG_SERVICE_REQUEST));
        assert!(report.service_userauth_requested());
        assert_eq!(report.parsed_field_count(), 2);
        assert_eq!(
            &preauth_label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-encrypted-transport-dispatch-modeled",
                "sshservicediag-encrypted-transport-preauth-state",
                "sshservicediag-preauth-service-request-modeled",
                "sshservicediag-preauth-service-userauth-recognized",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!report.service_success());
        assert!(!report.authentication_success());
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.shell_attached());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn preauth_service_userauth_models_publickey_after_service_prerequisite() {
        let (payload, len) = userauth_publickey_payload(false);
        let report = classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
            encrypted_packet_state_active: true,
            post_newkeys_plaintext_attempted: false,
            packet_crypto_failed: false,
            service_userauth_requested: true,
            decrypted_payload: &payload[..len],
        });

        assert_eq!(
            report.result(),
            SshPreauthServiceUserauthResult::UserauthPublickeyModeled
        );
        assert_eq!(report.message_number(), Some(SSH_MSG_USERAUTH_REQUEST));
        assert!(report.service_userauth_requested());
        assert_eq!(report.parsed_field_count(), 7);
        assert_eq!(
            &preauth_label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-encrypted-transport-dispatch-modeled",
                "sshservicediag-encrypted-transport-preauth-state",
                "sshservicediag-preauth-userauth-request-modeled",
                "sshservicediag-preauth-userauth-service-recognized",
                "sshservicediag-preauth-userauth-method-publickey-modeled",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!report.service_success());
        assert!(!report.authentication_success());
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.shell_attached());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn userauth_session_identifier_is_available_only_after_runtime_kex() {
        let ready = runtime_kex_ready_for_userauth();
        let first = classify_ssh_userauth_session_identifier(
            SshUserauthSessionIdentifierInput::Available(ready.userauth_session_identifier()),
        );
        let second = classify_ssh_userauth_session_identifier(
            SshUserauthSessionIdentifierInput::Available(ready.userauth_session_identifier()),
        );

        assert_eq!(
            first.result(),
            SshUserauthSessionIdentifierResult::Available
        );
        assert!(first.session_identifier_available());
        assert_eq!(
            first.byte_len(),
            Some(SSH_USERAUTH_SESSION_IDENTIFIER_BYTES)
        );
        assert_eq!(first.byte_len(), second.byte_len());
        assert_eq!(
            &userauth_session_identifier_label_names(&first)[..first.labels().len()],
            &[
                "sshservicediag-userauth-session-identifier-available",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!first.service_success());
        assert!(!first.authentication_success());
        assert_eq!(first.session_count(), 0);
        assert_eq!(first.channel_count(), 0);
        assert!(!first.shell_attached());
        assert!(!first.ssh_ready());
    }

    #[test_case]
    fn userauth_session_identifier_fails_closed_when_unavailable_or_malformed() {
        let unavailable = classify_ssh_userauth_session_identifier(
            SshUserauthSessionIdentifierInput::Unavailable,
        );
        assert_eq!(
            unavailable.result(),
            SshUserauthSessionIdentifierResult::Unavailable
        );
        assert!(!unavailable.session_identifier_available());
        assert_eq!(unavailable.byte_len(), None);
        assert_eq!(
            &userauth_session_identifier_label_names(&unavailable)[..unavailable.labels().len()],
            &[
                "sshservicediag-userauth-session-identifier-unavailable",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(!unavailable.authentication_success());
        assert!(!unavailable.ssh_ready());

        let malformed = classify_ssh_userauth_session_identifier(
            SshUserauthSessionIdentifierInput::Malformed { byte_len: 0 },
        );
        assert_eq!(
            malformed.result(),
            SshUserauthSessionIdentifierResult::Malformed
        );
        assert_eq!(malformed.byte_len(), Some(0));
        assert!(
            malformed
                .labels()
                .contains(&SshServiceReadinessLabel::UserauthSessionIdentifierMalformed)
        );

        let over_limit = classify_ssh_userauth_session_identifier(
            SshUserauthSessionIdentifierInput::OverLimit {
                byte_len: SSH_USERAUTH_SESSION_IDENTIFIER_BYTES + 1,
            },
        );
        assert_eq!(
            over_limit.result(),
            SshUserauthSessionIdentifierResult::OverLimit
        );
        assert_eq!(
            over_limit.byte_len(),
            Some(SSH_USERAUTH_SESSION_IDENTIFIER_BYTES + 1)
        );
        assert!(
            over_limit
                .labels()
                .contains(&SshServiceReadinessLabel::UserauthSessionIdentifierOverLimit)
        );
        assert!(!over_limit.authentication_success());
        assert!(!over_limit.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_accepts_valid_signature_as_prerequisite_only() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        let payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let report = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });

        assert_eq!(
            report.result(),
            SshPublickeyVerificationResult::VerifiedPrerequisiteOnly
        );
        assert_eq!(
            publickey_verification_label_names(&report),
            [
                "sshservicediag-publickey-verification-prerequisite-only",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert!(report.verified_prerequisite_only());
        assert_eq!(
            report.request_public_key_blob_len(),
            Some(public_key_blob.len())
        );
        assert_eq!(report.signature_blob_len(), Some(signature_blob.len()));
        assert!(report.signed_data_len().is_some());
        assert!(!report.service_success());
        assert!(!report.authentication_success());
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_rejects_bad_signature_without_authentication_success() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let mut signature_blob =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        let last = signature_blob
            .last_mut()
            .expect("public fixture signature is non-empty");
        *last ^= 1;
        let payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let report = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });

        assert_eq!(
            report.result(),
            SshPublickeyVerificationResult::SignatureRejected
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::PublickeyVerificationSignatureRejected)
        );
        assert!(!report.authentication_success());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_fails_closed_for_malformed_signature() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(b"not-a-signature-blob"),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let report = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });

        assert_eq!(
            report.result(),
            SshPublickeyVerificationResult::SignatureMalformed
        );
        assert!(
            report
                .labels()
                .contains(&SshServiceReadinessLabel::PublickeyVerificationSignatureMalformed)
        );
        assert!(!report.authentication_success());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_fails_closed_for_unsupported_algorithm_and_key_blob() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());
        let unsupported_payload = userauth_publickey_verification_payload(
            true,
            b"ssh-rsa",
            &public_key_blob,
            Some(b"not-checked"),
        );
        let malformed_key_payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            b"not-a-public-key-blob",
            Some(b"not-checked"),
        );

        let unsupported = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &unsupported_payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });
        let malformed_key = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &malformed_key_payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });

        assert_eq!(
            unsupported.result(),
            SshPublickeyVerificationResult::AlgorithmUnsupported
        );
        assert_eq!(
            malformed_key.result(),
            SshPublickeyVerificationResult::KeyBlobMalformed
        );
        assert!(!unsupported.authentication_success());
        assert!(!malformed_key.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_requires_authorized_key_match_and_session_identifier() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        let payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let no_match = AuthorizedKeyMatchReport::no_match_for_test(public_key_blob.len());
        let matched = AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let missing_authorized_key =
            classify_ssh_publickey_verification(SshPublickeyVerificationInput {
                decrypted_payload: &payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &no_match,
            });
        let missing_session = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Unavailable,
            authorized_key_match: &matched,
        });

        assert_eq!(
            missing_authorized_key.result(),
            SshPublickeyVerificationResult::AuthorizedKeyMissingOrNoMatch
        );
        assert_eq!(
            missing_session.result(),
            SshPublickeyVerificationResult::SessionIdentifierMissing
        );
        assert!(!missing_authorized_key.authentication_success());
        assert!(!missing_session.ssh_ready());
    }

    #[test_case]
    fn publickey_verification_fails_closed_without_signature_or_well_formed_signed_data() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());
        let unsigned_payload = userauth_publickey_verification_payload(
            false,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            None,
        );
        let malformed_payload = &unsigned_payload[..unsigned_payload.len() - 1];

        let unsigned = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: &unsigned_payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });
        let malformed = classify_ssh_publickey_verification(SshPublickeyVerificationInput {
            decrypted_payload: malformed_payload,
            service_userauth_requested: true,
            session_identifier: SshPublickeyVerificationSessionInput::Available(
                ready.userauth_session_identifier(),
            ),
            authorized_key_match: &authorized_key_match,
        });

        assert_eq!(
            unsigned.result(),
            SshPublickeyVerificationResult::SignatureNotPresent
        );
        assert_eq!(
            malformed.result(),
            SshPublickeyVerificationResult::SignedDataMalformed
        );
        assert!(!unsigned.authentication_success());
        assert!(!malformed.ssh_ready());
    }

    #[test_case]
    fn publickey_auth_response_pk_ok_for_unsigned_authorized_probe_only() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());
        let payload = userauth_publickey_verification_payload(
            false,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            None,
        );

        let report =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            });

        assert_eq!(
            report.result(),
            SshPublickeyAuthResponseResult::UserauthPkOkPrerequisiteOnly
        );
        assert_eq!(report.response_message_number(), SSH_MSG_USERAUTH_PK_OK);
        assert!(report.userauth_pk_ok());
        assert!(!report.userauth_failure());
        assert_eq!(
            &publickey_auth_response_label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-publickey-auth-response-pk-ok-prerequisite-only",
                "sshservicediag-authentication-unimplemented",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert_eq!(
            report.request_public_key_blob_len(),
            Some(public_key_blob.len())
        );
        assert_eq!(report.signature_blob_len(), None);
        assert_eq!(report.signed_data_len(), None);
        assert!(!report.service_success());
        assert!(!report.authentication_success());
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_auth_response_defers_valid_signature_to_failure_until_success_policy() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        let payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let report =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            });

        assert_eq!(
            report.result(),
            SshPublickeyAuthResponseResult::UserauthFailureSignatureValidSuccessDeferred
        );
        assert_eq!(report.response_message_number(), SSH_MSG_USERAUTH_FAILURE);
        assert!(report.userauth_failure());
        assert!(report.labels().contains(
            &SshServiceReadinessLabel::PublickeyAuthResponseFailureSignatureValidSuccessDeferred
        ));
        assert_eq!(report.signature_blob_len(), Some(signature_blob.len()));
        assert!(report.signed_data_len().is_some());
        assert!(!report.authentication_success());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_auth_response_fails_closed_for_invalid_and_malformed_signatures() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let mut signature_blob =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        *signature_blob.last_mut().unwrap() ^= 1;
        let rejected_payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let malformed_payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(b"not-a-signature-blob"),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let rejected =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &rejected_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            });
        let malformed =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &malformed_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            });

        assert_eq!(
            rejected.result(),
            SshPublickeyAuthResponseResult::UserauthFailureSignatureRejected
        );
        assert_eq!(
            malformed.result(),
            SshPublickeyAuthResponseResult::UserauthFailureSignatureMalformed
        );
        assert!(!rejected.authentication_success());
        assert!(!malformed.ssh_ready());
    }

    #[test_case]
    fn publickey_auth_response_fails_closed_for_policy_and_prerequisite_cases() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let unsigned_payload = userauth_publickey_verification_payload(
            false,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            None,
        );
        let unsupported_payload =
            userauth_publickey_verification_payload(false, b"ssh-rsa", &public_key_blob, None);
        let malformed_payload = &unsigned_payload[..unsigned_payload.len() - 1];
        let matched = AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());
        let no_match = AuthorizedKeyMatchReport::no_match_for_test(public_key_blob.len());

        let disabled =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: false,
                redaction_sensitive: false,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &matched,
            });
        let redaction =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: true,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &matched,
            });
        let missing_service =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: false,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &matched,
            });
        let missing_session =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Unavailable,
                authorized_key_match: &matched,
            });
        let unauthorized =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &no_match,
            });
        let unsupported =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: &unsupported_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &matched,
            });
        let malformed =
            classify_ssh_publickey_auth_response_policy(SshPublickeyAuthResponsePolicyInput {
                response_policy_enabled: true,
                redaction_sensitive: false,
                decrypted_payload: malformed_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &matched,
            });

        assert_eq!(
            disabled.result(),
            SshPublickeyAuthResponseResult::UserauthFailurePolicyDisabled
        );
        assert_eq!(
            redaction.result(),
            SshPublickeyAuthResponseResult::UserauthFailureRedactionSensitive
        );
        assert_eq!(
            missing_service.result(),
            SshPublickeyAuthResponseResult::UserauthFailurePrerequisiteMissing
        );
        assert_eq!(
            missing_session.result(),
            SshPublickeyAuthResponseResult::UserauthFailurePrerequisiteMissing
        );
        assert_eq!(
            unauthorized.result(),
            SshPublickeyAuthResponseResult::UserauthFailureAuthorizedKeyNoMatch
        );
        assert_eq!(
            unsupported.result(),
            SshPublickeyAuthResponseResult::UserauthFailureAlgorithmUnsupported
        );
        assert_eq!(
            malformed.result(),
            SshPublickeyAuthResponseResult::UserauthFailureRequestMalformed
        );
        assert!(!disabled.authentication_success());
        assert!(!redaction.ssh_ready());
        assert_eq!(
            unauthorized.response_message_number(),
            SSH_MSG_USERAUTH_FAILURE
        );
    }

    #[test_case]
    fn publickey_auth_success_account_accepts_reserved_account_only() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob = public_fixture_signature_blob_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            ready.userauth_session_identifier(),
            &public_key_blob,
        );
        let payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let report = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );

        assert_eq!(
            report.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthSuccessPrerequisiteOnly
        );
        assert_eq!(report.response_message_number(), SSH_MSG_USERAUTH_SUCCESS);
        assert!(report.userauth_success());
        assert!(!report.userauth_failure());
        assert_eq!(
            &publickey_auth_success_account_label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-publickey-auth-success-prerequisite-only",
                "sshservicediag-publickey-auth-success-account-match",
                "sshservicediag-authentication-success-local-only",
                "sshservicediag-session-unimplemented",
                "sshservicediag-not-ready",
            ]
        );
        assert_eq!(
            report.request_public_key_blob_len(),
            Some(public_key_blob.len())
        );
        assert_eq!(report.signature_blob_len(), Some(signature_blob.len()));
        assert!(report.signed_data_len().is_some());
        assert!(!report.service_success());
        assert!(report.authentication_success());
        assert_eq!(report.session_count(), 0);
        assert_eq!(report.channel_count(), 0);
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn publickey_auth_success_account_fails_closed_for_account_policy_cases() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob = public_fixture_signature_blob_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            ready.userauth_session_identifier(),
            &public_key_blob,
        );
        let matched_payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let mismatch_signature =
            public_fixture_signature_blob(ready.userauth_session_identifier(), &public_key_blob);
        let mismatch_payload = userauth_publickey_verification_payload(
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&mismatch_signature),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());

        let mismatch = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &mismatch_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let disabled = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: false,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &matched_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let missing = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: false,
                redaction_sensitive: false,
                decrypted_payload: &matched_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let redaction = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: true,
                decrypted_payload: &matched_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );

        assert_eq!(
            mismatch.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountMismatch
        );
        assert_eq!(
            disabled.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountPolicyDisabled
        );
        assert_eq!(
            missing.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAccountPrerequisiteMissing
        );
        assert_eq!(
            redaction.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRedactionSensitive
        );
        assert!(mismatch.userauth_failure());
        assert_eq!(mismatch.response_message_number(), SSH_MSG_USERAUTH_FAILURE);
        assert!(!mismatch.authentication_success());
        assert!(!disabled.ssh_ready());
        assert!(
            redaction
                .labels()
                .contains(&SshServiceReadinessLabel::PublickeyAuthFailureRedactionSensitive)
        );
    }

    #[test_case]
    fn publickey_auth_success_account_fails_closed_for_signature_and_prerequisites() {
        let ready = runtime_kex_ready_for_userauth();
        let public_key_blob = public_fixture_public_key_blob();
        let signature_blob = public_fixture_signature_blob_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            ready.userauth_session_identifier(),
            &public_key_blob,
        );
        let mut rejected_signature = signature_blob.clone();
        *rejected_signature.last_mut().unwrap() ^= 1;
        let signed_payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&signature_blob),
        );
        let unsigned_payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            false,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            None,
        );
        let rejected_payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            true,
            SSH_KEXINIT_POLICY_HOST_KEY,
            &public_key_blob,
            Some(&rejected_signature),
        );
        let unsupported_payload = userauth_publickey_verification_payload_for_user(
            SSH_RESERVED_ACCOUNT_TALOS,
            true,
            b"ssh-rsa",
            &public_key_blob,
            Some(&signature_blob),
        );
        let authorized_key_match =
            AuthorizedKeyMatchReport::prerequisite_only_for_test(public_key_blob.len());
        let no_match = AuthorizedKeyMatchReport::no_match_for_test(public_key_blob.len());

        let unsigned = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &unsigned_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let rejected = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &rejected_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let unauthorized = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &signed_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &no_match,
            },
        );
        let missing_service = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &signed_payload,
                service_userauth_requested: false,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let missing_session = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &signed_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Unavailable,
                authorized_key_match: &authorized_key_match,
            },
        );
        let unsupported = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &unsupported_payload,
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );
        let malformed = classify_ssh_publickey_auth_success_account_policy(
            SshPublickeyAuthSuccessAccountInput {
                account_policy_enabled: true,
                account_prerequisite_available: true,
                redaction_sensitive: false,
                decrypted_payload: &signed_payload[..signed_payload.len() - 1],
                service_userauth_requested: true,
                session_identifier: SshPublickeyVerificationSessionInput::Available(
                    ready.userauth_session_identifier(),
                ),
                authorized_key_match: &authorized_key_match,
            },
        );

        assert_eq!(
            unsigned.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureSignatureInvalid
        );
        assert_eq!(
            rejected.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureSignatureInvalid
        );
        assert_eq!(
            unauthorized.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureAuthorizedKeyNoMatch
        );
        assert_eq!(
            missing_service.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureResponsePrerequisiteMissing
        );
        assert_eq!(
            missing_session.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureResponsePrerequisiteMissing
        );
        assert_eq!(
            unsupported.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed
        );
        assert_eq!(
            malformed.result(),
            SshPublickeyAuthSuccessAccountResult::UserauthFailureRequestMalformed
        );
        assert!(!unsigned.authentication_success());
        assert!(!rejected.ssh_ready());
        assert_eq!(unauthorized.session_count(), 0);
        assert_eq!(unsupported.channel_count(), 0);
    }

    #[test_case]
    fn session_channel_open_accepts_one_modeled_authenticated_session_only() {
        let payload = session_channel_open_payload(SSH_CHANNEL_TYPE_SESSION);

        let report = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });

        assert_eq!(
            report.result(),
            SshSessionChannelOpenResult::ChannelOpenConfirmationPrerequisiteOnly
        );
        assert_eq!(report.request_message_number(), Some(SSH_MSG_CHANNEL_OPEN));
        assert_eq!(
            report.response_message_number(),
            SSH_MSG_CHANNEL_OPEN_CONFIRMATION
        );
        assert!(report.channel_open_confirmation());
        assert!(!report.channel_open_failure());
        assert_eq!(
            &session_channel_open_label_names(&report)[..report.labels().len()],
            &[
                "sshservicediag-authentication-success-local-only",
                "sshservicediag-session-channel-open-prerequisite-only",
                "sshservicediag-session-channel-open-session-type",
                "sshservicediag-session-open-local-only",
                "sshservicediag-channel-open-local-only",
                "sshservicediag-shell-unattached",
                "sshservicediag-not-ready",
            ]
        );
        assert_eq!(report.parsed_field_count(), 5);
        assert_eq!(
            report.channel_type_len(),
            Some(SSH_CHANNEL_TYPE_SESSION.len())
        );
        assert!(report.authentication_success());
        assert_eq!(report.session_count(), 1);
        assert_eq!(report.channel_count(), 1);
        assert!(!report.shell_attached());
        assert!(!report.reachability_accepted());
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn session_channel_open_fails_closed_for_prerequisites_and_policy() {
        let payload = session_channel_open_payload(SSH_CHANNEL_TYPE_SESSION);

        let missing_auth = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: false,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let disabled = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: false,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let duplicate = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: true,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let redaction = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: true,
            decrypted_payload: &payload,
        });

        assert_eq!(
            missing_auth.result(),
            SshSessionChannelOpenResult::ChannelOpenFailurePrerequisiteMissing
        );
        assert_eq!(
            disabled.result(),
            SshSessionChannelOpenResult::ChannelOpenFailurePolicyDisabled
        );
        assert_eq!(
            duplicate.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureDuplicate
        );
        assert_eq!(
            redaction.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureRedactionSensitive
        );
        assert_eq!(
            missing_auth.response_message_number(),
            SSH_MSG_CHANNEL_OPEN_FAILURE
        );
        assert!(!missing_auth.authentication_success());
        assert!(disabled.authentication_success());
        assert_eq!(duplicate.session_count(), 0);
        assert_eq!(redaction.channel_count(), 0);
        assert!(!redaction.ssh_ready());
    }

    #[test_case]
    fn session_channel_open_fails_closed_for_message_type_and_shape() {
        let wrong_message = [SSH_MSG_USERAUTH_SUCCESS];
        let unsupported_type = session_channel_open_payload(b"direct-tcpip");
        let malformed_missing_fields = {
            let mut payload = Vec::new();
            payload.push(SSH_MSG_CHANNEL_OPEN);
            push_ssh_string_to_vec(&mut payload, SSH_CHANNEL_TYPE_SESSION).unwrap();
            payload
        };
        let mut over_limit = Vec::new();
        over_limit.push(SSH_MSG_CHANNEL_OPEN);
        over_limit.extend_from_slice(&((SSH_CHANNEL_OPEN_TYPE_MAX_BYTES + 1) as u32).to_be_bytes());
        over_limit.extend_from_slice(&[b'a'; SSH_CHANNEL_OPEN_TYPE_MAX_BYTES + 1]);

        let wrong = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &wrong_message,
        });
        let unsupported = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &unsupported_type,
        });
        let malformed = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &malformed_missing_fields,
        });
        let over_limit = classify_ssh_session_channel_open(SshSessionChannelOpenInput {
            authentication_success: true,
            channel_open_policy_enabled: true,
            existing_session_channel: false,
            redaction_sensitive: false,
            decrypted_payload: &over_limit,
        });

        assert_eq!(
            wrong.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureWrongMessage
        );
        assert_eq!(
            unsupported.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureUnsupportedType
        );
        assert_eq!(
            malformed.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureMalformed
        );
        assert_eq!(
            over_limit.result(),
            SshSessionChannelOpenResult::ChannelOpenFailureMalformed
        );
        assert_eq!(
            wrong.request_message_number(),
            Some(SSH_MSG_USERAUTH_SUCCESS)
        );
        assert_eq!(unsupported.channel_type_len(), Some("direct-tcpip".len()));
        assert_eq!(malformed.parsed_field_count(), 2);
        assert_eq!(over_limit.parsed_field_count(), 1);
        assert_eq!(wrong.session_count(), 0);
        assert_eq!(unsupported.channel_count(), 0);
        assert!(!malformed.shell_attached());
        assert!(!over_limit.ssh_ready());
    }

    #[test_case]
    fn session_shell_request_recognizes_shell_but_returns_failure_or_no_reply() {
        let want_reply = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true),
        });
        let no_reply = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &session_shell_request_payload(
                SSH_CHANNEL_REQUEST_TYPE_SHELL,
                false,
            ),
        });

        assert_eq!(
            want_reply.result(),
            SshSessionShellRequestResult::ChannelFailureShellUnattachedWantReply
        );
        assert_eq!(
            no_reply.result(),
            SshSessionShellRequestResult::NoReplyShellUnattached
        );
        assert_eq!(
            &session_shell_request_label_names(&want_reply)[..want_reply.labels().len()],
            &[
                "sshservicediag-authentication-success-local-only",
                "sshservicediag-session-open-local-only",
                "sshservicediag-channel-open-local-only",
                "sshservicediag-session-shell-request-prerequisite-only",
                "sshservicediag-session-shell-request-shell-type",
                "sshservicediag-session-shell-request-want-reply",
                "sshservicediag-session-shell-request-failure-shell-unattached",
                "sshservicediag-shell-unattached",
                "sshservicediag-not-ready",
            ]
        );
        assert_eq!(
            want_reply.request_message_number(),
            Some(SSH_MSG_CHANNEL_REQUEST)
        );
        assert_eq!(
            want_reply.response_message_number(),
            Some(SSH_MSG_CHANNEL_FAILURE)
        );
        assert!(want_reply.channel_failure_response());
        assert_eq!(no_reply.response_message_number(), None);
        assert!(!no_reply.channel_failure_response());
        assert_eq!(want_reply.parsed_field_count(), 4);
        assert_eq!(
            want_reply.request_type_len(),
            Some(SSH_CHANNEL_REQUEST_TYPE_SHELL.len())
        );
        assert_eq!(want_reply.want_reply(), Some(true));
        assert_eq!(no_reply.want_reply(), Some(false));
        assert!(want_reply.authentication_success());
        assert_eq!(want_reply.session_count(), 1);
        assert_eq!(want_reply.channel_count(), 1);
        assert_eq!(want_reply.shell_request_count(), 1);
        assert!(!want_reply.shell_attached());
        assert!(!want_reply.reachability_accepted());
        assert!(!want_reply.ssh_ready());
        assert_eq!(no_reply.shell_request_count(), 1);
        assert!(!no_reply.shell_attached());
        assert!(!no_reply.ssh_ready());
    }

    #[test_case]
    fn session_shell_request_fails_closed_for_prerequisites_and_policy() {
        let payload = session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true);

        let missing_auth = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: false,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let missing_channel = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: false,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let disabled = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: false,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let duplicate = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: true,
            redaction_sensitive: false,
            decrypted_payload: &payload,
        });
        let redaction = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: true,
            decrypted_payload: &payload,
        });

        assert_eq!(
            missing_auth.result(),
            SshSessionShellRequestResult::ShellRequestFailureAuthenticationMissing
        );
        assert_eq!(
            missing_channel.result(),
            SshSessionShellRequestResult::ShellRequestFailureChannelMissing
        );
        assert_eq!(
            disabled.result(),
            SshSessionShellRequestResult::ShellRequestFailurePolicyDisabled
        );
        assert_eq!(
            duplicate.result(),
            SshSessionShellRequestResult::ShellRequestFailureDuplicate
        );
        assert_eq!(
            redaction.result(),
            SshSessionShellRequestResult::ShellRequestFailureRedactionSensitive
        );
        assert!(!missing_auth.authentication_success());
        assert_eq!(missing_channel.channel_count(), 0);
        assert_eq!(disabled.session_count(), 1);
        assert_eq!(duplicate.shell_request_count(), 0);
        assert!(!redaction.ssh_ready());
    }

    #[test_case]
    fn session_shell_request_fails_closed_for_message_type_and_shape() {
        let wrong_message = [SSH_MSG_CHANNEL_OPEN];
        let unsupported_type = session_shell_request_payload(b"exec", true);
        let malformed_missing_bool = {
            let mut payload = Vec::new();
            payload.push(SSH_MSG_CHANNEL_REQUEST);
            payload.extend_from_slice(&0u32.to_be_bytes());
            push_ssh_string_to_vec(&mut payload, SSH_CHANNEL_REQUEST_TYPE_SHELL).unwrap();
            payload
        };
        let mut trailing = session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true);
        trailing.push(0);
        let mut over_limit = Vec::new();
        over_limit.push(SSH_MSG_CHANNEL_REQUEST);
        over_limit.extend_from_slice(&0u32.to_be_bytes());
        over_limit
            .extend_from_slice(&((SSH_CHANNEL_REQUEST_TYPE_MAX_BYTES + 1) as u32).to_be_bytes());
        over_limit.extend_from_slice(&[b'a'; SSH_CHANNEL_REQUEST_TYPE_MAX_BYTES + 1]);
        over_limit.push(1);

        let wrong = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &wrong_message,
        });
        let unsupported = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &unsupported_type,
        });
        let malformed = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &malformed_missing_bool,
        });
        let trailing = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &trailing,
        });
        let over_limit = classify_ssh_session_shell_request(SshSessionShellRequestInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            existing_shell_request_or_attachment: false,
            redaction_sensitive: false,
            decrypted_payload: &over_limit,
        });

        assert_eq!(
            wrong.result(),
            SshSessionShellRequestResult::ShellRequestFailureUnsupportedMessage
        );
        assert_eq!(
            unsupported.result(),
            SshSessionShellRequestResult::ShellRequestFailureUnsupportedRequestType
        );
        assert_eq!(
            malformed.result(),
            SshSessionShellRequestResult::ShellRequestFailureMalformed
        );
        assert_eq!(
            trailing.result(),
            SshSessionShellRequestResult::ShellRequestFailureMalformed
        );
        assert_eq!(
            over_limit.result(),
            SshSessionShellRequestResult::ShellRequestFailureMalformed
        );
        assert_eq!(wrong.request_message_number(), Some(SSH_MSG_CHANNEL_OPEN));
        assert_eq!(unsupported.request_type_len(), Some("exec".len()));
        assert_eq!(malformed.parsed_field_count(), 3);
        assert_eq!(trailing.parsed_field_count(), 4);
        assert_eq!(trailing.want_reply(), Some(true));
        assert_eq!(over_limit.parsed_field_count(), 2);
        assert_eq!(wrong.shell_request_count(), 0);
        assert_eq!(unsupported.channel_count(), 1);
        assert!(!malformed.shell_attached());
        assert!(!over_limit.ssh_ready());
    }

    #[test_case]
    fn session_shell_attachment_accepts_local_modeled_shell_with_channel_success() {
        let want_reply = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true),
        });
        let no_reply = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &session_shell_request_payload(
                SSH_CHANNEL_REQUEST_TYPE_SHELL,
                false,
            ),
        });

        assert_eq!(
            want_reply.result(),
            SshSessionShellAttachmentResult::ChannelSuccessShellAttachedWantReply
        );
        assert_eq!(
            no_reply.result(),
            SshSessionShellAttachmentResult::NoReplyShellAttached
        );
        assert_eq!(
            &session_shell_attachment_label_names(&want_reply)[..want_reply.labels().len()],
            &[
                "sshservicediag-authentication-success-local-only",
                "sshservicediag-session-open-local-only",
                "sshservicediag-channel-open-local-only",
                "sshservicediag-session-shell-request-prerequisite-only",
                "sshservicediag-session-shell-request-shell-type",
                "sshservicediag-session-shell-request-want-reply",
                "sshservicediag-session-shell-attachment-prerequisite-only",
                "sshservicediag-session-shell-attachment-local-execution-owned",
                "sshservicediag-session-shell-attachment-local-stdio-owned",
                "sshservicediag-session-shell-attachment-want-reply",
                "sshservicediag-session-shell-attachment-channel-success",
                "sshservicediag-shell-attached",
                "sshservicediag-not-ready",
            ]
        );
        assert_eq!(
            want_reply.request_message_number(),
            Some(SSH_MSG_CHANNEL_REQUEST)
        );
        assert_eq!(
            want_reply.response_message_number(),
            Some(SSH_MSG_CHANNEL_SUCCESS)
        );
        assert!(want_reply.channel_success_response());
        assert!(!want_reply.channel_failure_response());
        assert_eq!(no_reply.response_message_number(), None);
        assert!(!no_reply.channel_success_response());
        assert_eq!(want_reply.parsed_field_count(), 4);
        assert_eq!(
            want_reply.request_type_len(),
            Some(SSH_CHANNEL_REQUEST_TYPE_SHELL.len())
        );
        assert_eq!(want_reply.want_reply(), Some(true));
        assert!(want_reply.authentication_success());
        assert_eq!(want_reply.session_count(), 1);
        assert_eq!(want_reply.channel_count(), 1);
        assert_eq!(want_reply.shell_request_count(), 1);
        assert!(want_reply.shell_attached());
        assert!(want_reply.local_process_session_owned());
        assert!(want_reply.local_stdio_descriptors_owned());
        assert!(want_reply.channel_lifecycle_open());
        assert!(!want_reply.reachability_accepted());
        assert!(!want_reply.ssh_ready());
        assert_eq!(no_reply.shell_request_count(), 1);
        assert!(no_reply.shell_attached());
        assert!(!no_reply.ssh_ready());
    }

    #[test_case]
    fn session_shell_attachment_fails_closed_for_attachment_ownership_and_lifecycle() {
        let payload = session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true);

        let disabled = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: false,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &payload,
        });
        let duplicate_attachment =
            classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
                authentication_success: true,
                open_session_channel: true,
                shell_request_policy_enabled: true,
                shell_attachment_policy_enabled: true,
                existing_shell_request: false,
                existing_shell_attachment: true,
                redaction_sensitive: false,
                local_process_session_owned: true,
                local_stdio_descriptors_owned: true,
                channel_lifecycle_open: true,
                decrypted_payload: &payload,
            });
        let missing_execution =
            classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
                authentication_success: true,
                open_session_channel: true,
                shell_request_policy_enabled: true,
                shell_attachment_policy_enabled: true,
                existing_shell_request: false,
                existing_shell_attachment: false,
                redaction_sensitive: false,
                local_process_session_owned: false,
                local_stdio_descriptors_owned: true,
                channel_lifecycle_open: true,
                decrypted_payload: &payload,
            });
        let missing_stdio = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: false,
            channel_lifecycle_open: true,
            decrypted_payload: &payload,
        });
        let lifecycle_closed =
            classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
                authentication_success: true,
                open_session_channel: true,
                shell_request_policy_enabled: true,
                shell_attachment_policy_enabled: true,
                existing_shell_request: false,
                existing_shell_attachment: false,
                redaction_sensitive: false,
                local_process_session_owned: true,
                local_stdio_descriptors_owned: true,
                channel_lifecycle_open: false,
                decrypted_payload: &payload,
            });

        assert_eq!(
            disabled.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureShellAttachmentPolicyDisabled
        );
        assert_eq!(
            duplicate_attachment.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureDuplicateAttachment
        );
        assert_eq!(
            missing_execution.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureLocalExecutionMissing
        );
        assert_eq!(
            missing_stdio.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureLocalExecutionMissing
        );
        assert_eq!(
            lifecycle_closed.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureLifecycleViolation
        );
        assert!(disabled.channel_failure_response());
        assert!(duplicate_attachment.channel_failure_response());
        assert!(missing_execution.channel_failure_response());
        assert!(missing_stdio.channel_failure_response());
        assert!(lifecycle_closed.channel_failure_response());
        assert_eq!(missing_execution.shell_request_count(), 1);
        assert_eq!(missing_stdio.shell_request_count(), 1);
        assert!(!missing_execution.local_process_session_owned());
        assert!(!missing_stdio.local_stdio_descriptors_owned());
        assert!(!lifecycle_closed.channel_lifecycle_open());
        assert!(!disabled.shell_attached());
        assert!(!duplicate_attachment.shell_attached());
        assert!(!missing_execution.ssh_ready());
    }

    #[test_case]
    fn session_shell_attachment_preserves_shell_request_fail_closed_controls() {
        let payload = session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true);
        let unsupported_type = session_shell_request_payload(b"exec", true);
        let mut trailing = session_shell_request_payload(SSH_CHANNEL_REQUEST_TYPE_SHELL, true);
        trailing.push(0);

        let missing_auth = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: false,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &payload,
        });
        let missing_channel =
            classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
                authentication_success: true,
                open_session_channel: false,
                shell_request_policy_enabled: true,
                shell_attachment_policy_enabled: true,
                existing_shell_request: false,
                existing_shell_attachment: false,
                redaction_sensitive: false,
                local_process_session_owned: true,
                local_stdio_descriptors_owned: true,
                channel_lifecycle_open: true,
                decrypted_payload: &payload,
            });
        let duplicate_request =
            classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
                authentication_success: true,
                open_session_channel: true,
                shell_request_policy_enabled: true,
                shell_attachment_policy_enabled: true,
                existing_shell_request: true,
                existing_shell_attachment: false,
                redaction_sensitive: false,
                local_process_session_owned: true,
                local_stdio_descriptors_owned: true,
                channel_lifecycle_open: true,
                decrypted_payload: &payload,
            });
        let unsupported = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &unsupported_type,
        });
        let malformed = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: false,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &trailing,
        });
        let redaction = classify_ssh_session_shell_attachment(SshSessionShellAttachmentInput {
            authentication_success: true,
            open_session_channel: true,
            shell_request_policy_enabled: true,
            shell_attachment_policy_enabled: true,
            existing_shell_request: false,
            existing_shell_attachment: false,
            redaction_sensitive: true,
            local_process_session_owned: true,
            local_stdio_descriptors_owned: true,
            channel_lifecycle_open: true,
            decrypted_payload: &payload,
        });

        assert_eq!(
            missing_auth.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureAuthenticationMissing
        );
        assert_eq!(
            missing_channel.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureChannelMissing
        );
        assert_eq!(
            duplicate_request.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureDuplicateShellRequest
        );
        assert_eq!(
            unsupported.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureUnsupportedRequestType
        );
        assert_eq!(
            malformed.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureMalformed
        );
        assert_eq!(
            redaction.result(),
            SshSessionShellAttachmentResult::ShellAttachmentFailureRedactionSensitive
        );
        assert!(!missing_auth.authentication_success());
        assert_eq!(missing_channel.channel_count(), 0);
        assert_eq!(duplicate_request.shell_request_count(), 0);
        assert_eq!(unsupported.request_type_len(), Some("exec".len()));
        assert_eq!(malformed.parsed_field_count(), 4);
        assert!(!redaction.shell_attached());
        assert!(!malformed.ssh_ready());
    }

    #[test_case]
    fn preauth_service_userauth_fails_closed_for_before_service_and_unsupported_shapes() {
        let (payload, len) = userauth_publickey_payload(true);
        let before_service =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: false,
                decrypted_payload: &payload[..len],
            });
        assert_eq!(
            before_service.result(),
            SshPreauthServiceUserauthResult::UserauthBeforeService
        );
        assert!(
            before_service
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthUserauthBeforeService)
        );
        assert!(!before_service.authentication_success());
        assert!(!before_service.ssh_ready());

        let (payload, len) = service_request_payload(b"ssh-connection");
        let unsupported_service_request =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: false,
                decrypted_payload: &payload[..len],
            });
        assert_eq!(
            unsupported_service_request.result(),
            SshPreauthServiceUserauthResult::ServiceUnsupported
        );
        assert!(
            unsupported_service_request
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthServiceUnsupported)
        );

        let (payload, len) = userauth_unsupported_payload(b"ssh-userauth", b"publickey");
        let unsupported_userauth_service =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: true,
                decrypted_payload: &payload[..len],
            });
        assert_eq!(
            unsupported_userauth_service.result(),
            SshPreauthServiceUserauthResult::UserauthServiceUnsupported
        );
        assert!(
            unsupported_userauth_service
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthUserauthServiceUnsupported)
        );

        let (payload, len) = userauth_unsupported_payload(SSH_SERVICE_CONNECTION, b"password");
        let unsupported_method =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: true,
                decrypted_payload: &payload[..len],
            });
        assert_eq!(
            unsupported_method.result(),
            SshPreauthServiceUserauthResult::UserauthMethodUnsupported
        );
        assert!(
            unsupported_method
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthUserauthMethodUnsupported)
        );
        assert!(!unsupported_method.ssh_ready());
    }

    #[test_case]
    fn preauth_service_userauth_rejects_malformed_and_inherited_dispatch_failures() {
        let (mut payload, len) = service_request_payload(SSH_SERVICE_USERAUTH);
        payload[len] = 0xff;
        let malformed_service =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: false,
                decrypted_payload: &payload[..len + 1],
            });
        assert_eq!(
            malformed_service.result(),
            SshPreauthServiceUserauthResult::ServiceMalformed
        );
        assert!(
            malformed_service
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthServiceMalformed)
        );

        let (payload, len) = userauth_publickey_payload(false);
        let malformed_userauth =
            classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
                encrypted_packet_state_active: true,
                post_newkeys_plaintext_attempted: false,
                packet_crypto_failed: false,
                service_userauth_requested: true,
                decrypted_payload: &payload[..len - 1],
            });
        assert_eq!(
            malformed_userauth.result(),
            SshPreauthServiceUserauthResult::UserauthMalformed
        );
        assert!(
            malformed_userauth
                .labels()
                .contains(&SshServiceReadinessLabel::PreauthUserauthMalformed)
        );

        let inactive = classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
            encrypted_packet_state_active: false,
            post_newkeys_plaintext_attempted: false,
            packet_crypto_failed: false,
            service_userauth_requested: true,
            decrypted_payload: &[SSH_MSG_USERAUTH_REQUEST],
        });
        assert_eq!(
            inactive.result(),
            SshPreauthServiceUserauthResult::InactiveEncryptedPacketState
        );
        assert!(
            inactive
                .labels()
                .contains(&SshServiceReadinessLabel::NewkeysNotReady)
        );

        let crypto_failed = classify_ssh_preauth_service_userauth(SshPreauthServiceUserauthInput {
            encrypted_packet_state_active: true,
            post_newkeys_plaintext_attempted: false,
            packet_crypto_failed: true,
            service_userauth_requested: true,
            decrypted_payload: &[SSH_MSG_USERAUTH_REQUEST],
        });
        assert_eq!(
            crypto_failed.result(),
            SshPreauthServiceUserauthResult::PacketCryptoFailed
        );
        assert!(
            crypto_failed
                .labels()
                .contains(&SshServiceReadinessLabel::EncryptedPacketCryptoFailed)
        );
        assert!(!crypto_failed.authentication_success());
        assert!(!crypto_failed.ssh_ready());
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
