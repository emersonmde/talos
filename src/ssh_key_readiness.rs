//! SSH key-management readiness classification.
//!
//! This module classifies caller-supplied prerequisite material only. It does
//! not generate host keys, parse authorized keys, persist secrets, sample
//! hardware, or accept SSH service readiness.

use crate::entropy::{
    self, EntropyDiagnosticReport, EntropyDiagnosticSnapshot, OperatorSeedMaterialMetadata,
    OperatorSeedMaterialState,
};
use crate::{
    initramfs::{ReadOnlyInitramfs, VfsNodeKind},
    posix::PosixError,
};
use alloc::vec::Vec;
use signature::Signer;
use ssh_key::{Algorithm, Cipher, Kdf, PrivateKey, Signature, encoding::Encode};

pub(crate) const HOST_KEY_PATH: &[u8] = b"/etc/talos/ssh/ssh_host_ed25519_key";
pub(crate) const HOST_KEY_MIN_METADATA_BYTES: usize = 64;
pub(crate) const HOST_KEY_MAX_METADATA_BYTES: usize = 4096;
pub(crate) const AUTHORIZED_KEY_PATH: &[u8] = b"/etc/talos/ssh/authorized_keys";
pub(crate) const AUTHORIZED_KEY_MIN_METADATA_BYTES: usize = 64;
pub(crate) const AUTHORIZED_KEY_MAX_METADATA_BYTES: usize = 4096;
pub(crate) const EXPOSURE_MARKER_PATH: &[u8] = b"/etc/talos/ssh/exposure-enabled";
pub(crate) const EXPOSURE_MARKER_MAX_METADATA_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HostKeyState {
    Missing,
    Invalid,
    Insufficient,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizedKeyState {
    Missing,
    Invalid,
    Insufficient,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SeedMaterialState {
    Missing,
    Insufficient,
    MetadataPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HostKeyMaterialState {
    Missing,
    Invalid,
    Insufficient,
    Sufficient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HostKeyMaterialMetadata {
    state: HostKeyMaterialState,
    byte_len: Option<usize>,
}

impl HostKeyMaterialMetadata {
    pub(crate) const fn missing() -> Self {
        Self {
            state: HostKeyMaterialState::Missing,
            byte_len: None,
        }
    }

    pub(crate) const fn invalid(byte_len: Option<usize>) -> Self {
        Self {
            state: HostKeyMaterialState::Invalid,
            byte_len,
        }
    }

    pub(crate) const fn insufficient(byte_len: usize) -> Self {
        Self {
            state: HostKeyMaterialState::Insufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn sufficient(byte_len: usize) -> Self {
        Self {
            state: HostKeyMaterialState::Sufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn state(self) -> HostKeyMaterialState {
        self.state
    }

    pub(crate) const fn byte_len(self) -> Option<usize> {
        self.byte_len
    }
}

pub(crate) struct HostKeyPrivateMaterial {
    private_key: PrivateKey,
    byte_len: usize,
}

impl HostKeyPrivateMaterial {
    pub(crate) const fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub(crate) fn sign_exchange_hash(
        &self,
        exchange_hash: &[u8],
    ) -> Result<HostKeyExchangeSignature, HostKeyMaterialMetadata> {
        let signature = self
            .private_key
            .try_sign(exchange_hash)
            .map_err(|_| HostKeyMaterialMetadata::invalid(Some(self.byte_len)))?;
        if signature.algorithm() != Algorithm::Ed25519 {
            return Err(HostKeyMaterialMetadata::invalid(Some(self.byte_len)));
        }
        Ok(HostKeyExchangeSignature { signature })
    }

    pub(crate) fn public_key_blob(&self) -> Result<Vec<u8>, HostKeyMaterialMetadata> {
        self.private_key
            .public_key()
            .to_bytes()
            .map_err(|_| HostKeyMaterialMetadata::invalid(Some(self.byte_len)))
    }
}

pub(crate) struct HostKeyExchangeSignature {
    signature: Signature,
}

impl HostKeyExchangeSignature {
    pub(crate) const fn algorithm_name(&self) -> &'static str {
        "ssh-ed25519"
    }

    pub(crate) fn byte_len(&self) -> usize {
        self.signature.as_bytes().len()
    }

    pub(crate) const fn as_ssh_signature(&self) -> &Signature {
        &self.signature
    }

    pub(crate) fn encoded_blob(&self) -> Result<Vec<u8>, HostKeyMaterialMetadata> {
        self.signature
            .encode_vec()
            .map_err(|_| HostKeyMaterialMetadata::invalid(None))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizedKeyMaterialState {
    Missing,
    Invalid,
    Insufficient,
    Sufficient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AuthorizedKeyMaterialMetadata {
    state: AuthorizedKeyMaterialState,
    byte_len: Option<usize>,
}

impl AuthorizedKeyMaterialMetadata {
    pub(crate) const fn missing() -> Self {
        Self {
            state: AuthorizedKeyMaterialState::Missing,
            byte_len: None,
        }
    }

    pub(crate) const fn invalid(byte_len: Option<usize>) -> Self {
        Self {
            state: AuthorizedKeyMaterialState::Invalid,
            byte_len,
        }
    }

    pub(crate) const fn insufficient(byte_len: usize) -> Self {
        Self {
            state: AuthorizedKeyMaterialState::Insufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn sufficient(byte_len: usize) -> Self {
        Self {
            state: AuthorizedKeyMaterialState::Sufficient,
            byte_len: Some(byte_len),
        }
    }

    pub(crate) const fn state(self) -> AuthorizedKeyMaterialState {
        self.state
    }

    pub(crate) const fn byte_len(self) -> Option<usize> {
        self.byte_len
    }
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

    pub(crate) const fn with_host_key_material(
        mut self,
        metadata: HostKeyMaterialMetadata,
    ) -> Self {
        self.host_key = match metadata.state() {
            HostKeyMaterialState::Missing => HostKeyState::Missing,
            HostKeyMaterialState::Invalid => HostKeyState::Invalid,
            HostKeyMaterialState::Insufficient => HostKeyState::Insufficient,
            HostKeyMaterialState::Sufficient => HostKeyState::MetadataPresent,
        };
        self
    }

    pub(crate) const fn with_authorized_key_metadata(mut self) -> Self {
        self.authorized_key = AuthorizedKeyState::MetadataPresent;
        self
    }

    pub(crate) const fn with_authorized_key_material(
        mut self,
        metadata: AuthorizedKeyMaterialMetadata,
    ) -> Self {
        self.authorized_key = match metadata.state() {
            AuthorizedKeyMaterialState::Missing => AuthorizedKeyState::Missing,
            AuthorizedKeyMaterialState::Invalid => AuthorizedKeyState::Invalid,
            AuthorizedKeyMaterialState::Insufficient => AuthorizedKeyState::Insufficient,
            AuthorizedKeyMaterialState::Sufficient => AuthorizedKeyState::MetadataPresent,
        };
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

    pub(crate) const fn with_persistence_state(mut self, persistence: PersistenceState) -> Self {
        self.persistence = persistence;
        self
    }

    pub(crate) const fn with_exposure_enabled(mut self) -> Self {
        self.exposure = ExposureState::ExplicitlyEnabled;
        self
    }

    pub(crate) const fn with_exposure_state(mut self, exposure: ExposureState) -> Self {
        self.exposure = exposure;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshKeyReadinessLabel {
    MissingHostKey,
    InvalidHostKey,
    InsufficientHostKey,
    MissingAuthorizedKey,
    InvalidAuthorizedKey,
    InsufficientAuthorizedKey,
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
            Self::InvalidHostKey => "sshkeydiag-host-key-invalid",
            Self::InsufficientHostKey => "sshkeydiag-host-key-insufficient",
            Self::MissingAuthorizedKey => "sshkeydiag-missing-authorized-key",
            Self::InvalidAuthorizedKey => "sshkeydiag-authorized-key-invalid",
            Self::InsufficientAuthorizedKey => "sshkeydiag-authorized-key-insufficient",
            Self::EntropyUnready => "sshkeydiag-entropy-unready",
            Self::SeedMaterialMissing => "sshkeydiag-seed-material-missing",
            Self::SeedMaterialInsufficient => "sshkeydiag-seed-material-insufficient",
            Self::PersistenceUnavailable => "sshkeydiag-persistence-unavailable",
            Self::ExposureDisabled => "sshkeydiag-exposure-disabled",
            Self::NotReady => "sshkeydiag-not-ready",
        }
    }
}

pub(crate) const MAX_SSH_KEY_READINESS_LABELS: usize = 10;

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
    if snapshot.host_key == HostKeyState::Invalid {
        report.push(SshKeyReadinessLabel::InvalidHostKey);
    }
    if snapshot.host_key == HostKeyState::Insufficient {
        report.push(SshKeyReadinessLabel::InsufficientHostKey);
    }
    if snapshot.authorized_key == AuthorizedKeyState::Missing {
        report.push(SshKeyReadinessLabel::MissingAuthorizedKey);
    }
    if snapshot.authorized_key == AuthorizedKeyState::Invalid {
        report.push(SshKeyReadinessLabel::InvalidAuthorizedKey);
    }
    if snapshot.authorized_key == AuthorizedKeyState::Insufficient {
        report.push(SshKeyReadinessLabel::InsufficientAuthorizedKey);
    }
    if !snapshot.entropy.cryptographic_strength() {
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

pub(crate) fn classify_host_key_material(initramfs: ReadOnlyInitramfs) -> HostKeyMaterialMetadata {
    match load_host_key_private_material(initramfs) {
        Ok(material) => HostKeyMaterialMetadata::sufficient(material.byte_len()),
        Err(metadata) => metadata,
    }
}

pub(crate) fn load_host_key_private_material(
    initramfs: ReadOnlyInitramfs,
) -> Result<HostKeyPrivateMaterial, HostKeyMaterialMetadata> {
    let handle = match initramfs.lookup_default(HOST_KEY_PATH) {
        Ok(handle) => handle,
        Err(PosixError::NoEntry) => return Err(HostKeyMaterialMetadata::missing()),
        Err(_) => return Err(HostKeyMaterialMetadata::invalid(None)),
    };

    let metadata = handle.metadata();
    if metadata.kind() != VfsNodeKind::RegularFile {
        return Err(HostKeyMaterialMetadata::invalid(Some(metadata.len())));
    }

    let byte_len = metadata.len();
    if byte_len == 0 || byte_len > HOST_KEY_MAX_METADATA_BYTES {
        return Err(HostKeyMaterialMetadata::invalid(Some(byte_len)));
    } else if byte_len < HOST_KEY_MIN_METADATA_BYTES {
        return Err(HostKeyMaterialMetadata::insufficient(byte_len));
    }

    let bytes = initramfs
        .regular_file_bytes(HOST_KEY_PATH)
        .map_err(|_| HostKeyMaterialMetadata::invalid(Some(byte_len)))?;
    let private_key = parse_host_key_private_material(bytes)
        .map_err(|_| HostKeyMaterialMetadata::invalid(Some(byte_len)))?;
    Ok(HostKeyPrivateMaterial {
        private_key,
        byte_len,
    })
}

fn parse_host_key_private_material(bytes: &[u8]) -> Result<PrivateKey, ()> {
    let private_key = PrivateKey::from_openssh(bytes).map_err(|_| ())?;
    if private_key.cipher() != Cipher::None
        || private_key.kdf() != &Kdf::None
        || private_key.is_encrypted()
        || private_key.algorithm() != Algorithm::Ed25519
        || private_key.key_data().ed25519().is_none()
    {
        return Err(());
    }
    Ok(private_key)
}

#[cfg(test)]
pub(crate) fn public_fixture_host_key_private_material() -> HostKeyPrivateMaterial {
    let private_key =
        parse_host_key_private_material(PUBLIC_FIXTURE_OPENSSH_ED25519_HOST_KEY_BYTES)
            .expect("public fixture host key must parse");
    HostKeyPrivateMaterial {
        private_key,
        byte_len: PUBLIC_FIXTURE_OPENSSH_ED25519_HOST_KEY_BYTES.len(),
    }
}

#[cfg(test)]
static PUBLIC_FIXTURE_OPENSSH_ED25519_HOST_KEY_BYTES: &[u8] = br#"
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYgAAAJgAIAxdACAM
XQAAAAtzc2gtZWQyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYg
AAAEC2BsIi0QwW2uFscKTUUXNHLsYX4FxlaSDSblbAj7WR7bM+rvN+ot98qgEN796jTiQf
ZfG1KaT0PtFDJ/XFSqtiAAAAEHVzZXJAZXhhbXBsZS5jb20BAgMEBQ==
-----END OPENSSH PRIVATE KEY-----
"#;

pub(crate) fn classify_authorized_key_material(
    initramfs: ReadOnlyInitramfs,
) -> AuthorizedKeyMaterialMetadata {
    let handle = match initramfs.lookup_default(AUTHORIZED_KEY_PATH) {
        Ok(handle) => handle,
        Err(PosixError::NoEntry) => return AuthorizedKeyMaterialMetadata::missing(),
        Err(_) => return AuthorizedKeyMaterialMetadata::invalid(None),
    };

    let metadata = handle.metadata();
    if metadata.kind() != VfsNodeKind::RegularFile {
        return AuthorizedKeyMaterialMetadata::invalid(Some(metadata.len()));
    }

    let byte_len = metadata.len();
    if byte_len == 0 || byte_len > AUTHORIZED_KEY_MAX_METADATA_BYTES {
        AuthorizedKeyMaterialMetadata::invalid(Some(byte_len))
    } else if byte_len < AUTHORIZED_KEY_MIN_METADATA_BYTES {
        AuthorizedKeyMaterialMetadata::insufficient(byte_len)
    } else {
        AuthorizedKeyMaterialMetadata::sufficient(byte_len)
    }
}

pub(crate) const fn classify_persistence_metadata(
    seed_metadata: OperatorSeedMaterialMetadata,
    host_key_metadata: HostKeyMaterialMetadata,
    authorized_key_metadata: AuthorizedKeyMaterialMetadata,
) -> PersistenceState {
    if matches!(seed_metadata.state(), OperatorSeedMaterialState::Sufficient)
        && matches!(host_key_metadata.state(), HostKeyMaterialState::Sufficient)
        && matches!(
            authorized_key_metadata.state(),
            AuthorizedKeyMaterialState::Sufficient
        )
    {
        PersistenceState::MetadataPresent
    } else {
        PersistenceState::Unavailable
    }
}

pub(crate) fn classify_exposure_marker(initramfs: ReadOnlyInitramfs) -> ExposureState {
    let handle = match initramfs.lookup_default(EXPOSURE_MARKER_PATH) {
        Ok(handle) => handle,
        Err(_) => return ExposureState::Disabled,
    };

    let metadata = handle.metadata();
    if metadata.kind() == VfsNodeKind::RegularFile
        && metadata.len() <= EXPOSURE_MARKER_MAX_METADATA_BYTES
    {
        ExposureState::ExplicitlyEnabled
    } else {
        ExposureState::Disabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::{EntropyObservation, OperatorSeedObservation};
    use crate::initramfs::{DirectoryEntry, InitramfsNode, phase8_readonly_initramfs_fixture};

    fn label_names(report: &SshKeyReadinessReport) -> [&'static str; MAX_SSH_KEY_READINESS_LABELS] {
        let mut labels = [""; MAX_SSH_KEY_READINESS_LABELS];
        for (index, label) in report.labels().iter().enumerate() {
            labels[index] = label.name();
        }
        labels
    }

    fn load_host_key_private_material_error(
        initramfs: ReadOnlyInitramfs,
    ) -> HostKeyMaterialMetadata {
        match load_host_key_private_material(initramfs) {
            Ok(_) => panic!("public fixture host key unexpectedly loaded"),
            Err(metadata) => metadata,
        }
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
                "",
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
    fn host_key_private_material_maps_to_fail_closed_states() {
        let missing = classify_host_key_material(phase8_readonly_initramfs_fixture());
        let directory = classify_host_key_material(directory_host_key_initramfs());
        let empty = classify_host_key_material(empty_host_key_initramfs());
        let oversized = classify_host_key_material(oversized_host_key_initramfs());
        let insufficient = classify_host_key_material(insufficient_host_key_initramfs());
        let malformed = classify_host_key_material(malformed_host_key_initramfs());
        let encrypted = classify_host_key_material(encrypted_host_key_initramfs());
        let sufficient = classify_host_key_material(sufficient_host_key_initramfs());

        assert_eq!(missing, HostKeyMaterialMetadata::missing());
        assert_eq!(missing.byte_len(), None);
        assert_eq!(directory, HostKeyMaterialMetadata::invalid(Some(0)));
        assert_eq!(empty, HostKeyMaterialMetadata::invalid(Some(0)));
        assert_eq!(
            oversized,
            HostKeyMaterialMetadata::invalid(Some(HOST_KEY_MAX_METADATA_BYTES + 1))
        );
        assert_eq!(
            insufficient,
            HostKeyMaterialMetadata::insufficient(HOST_KEY_MIN_METADATA_BYTES - 1)
        );
        assert_eq!(
            malformed,
            HostKeyMaterialMetadata::invalid(Some(HOST_KEY_MIN_METADATA_BYTES))
        );
        assert_eq!(
            encrypted,
            HostKeyMaterialMetadata::invalid(Some(ENCRYPTED_OPENSSH_HOST_KEY_BYTES.len()))
        );
        assert_eq!(
            sufficient,
            HostKeyMaterialMetadata::sufficient(VALID_OPENSSH_ED25519_HOST_KEY_BYTES.len())
        );
    }

    #[test_case]
    fn host_key_private_material_clears_only_host_key_prerequisite() {
        let invalid = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_material(classify_host_key_material(malformed_host_key_initramfs())),
        );
        let insufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_host_key_material(
                classify_host_key_material(insufficient_host_key_initramfs()),
            ),
        );
        let sufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_host_key_material(
                classify_host_key_material(sufficient_host_key_initramfs()),
            ),
        );

        assert!(
            invalid
                .labels()
                .contains(&SshKeyReadinessLabel::InvalidHostKey)
        );
        assert!(
            insufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InsufficientHostKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::MissingHostKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InvalidHostKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InsufficientHostKey)
        );
        assert!(
            sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::MissingAuthorizedKey)
        );
        assert!(
            sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::EntropyUnready)
        );
        assert!(!sufficient.ssh_ready());
    }

    #[test_case]
    fn host_key_private_material_loads_and_signs_public_fixture() {
        let material = load_host_key_private_material(sufficient_host_key_initramfs())
            .expect("public fixture host key loads");
        let signature = material
            .sign_exchange_hash(b"talos-public-test-exchange-hash")
            .expect("public fixture host key signs");

        assert_eq!(
            material.byte_len(),
            VALID_OPENSSH_ED25519_HOST_KEY_BYTES.len()
        );
        assert_eq!(signature.algorithm_name(), "ssh-ed25519");
        assert_eq!(signature.byte_len(), 64);
        assert_eq!(signature.as_ssh_signature().algorithm(), Algorithm::Ed25519);
    }

    #[test_case]
    fn host_key_private_material_loader_rejects_nonaccepted_inputs() {
        assert_eq!(
            load_host_key_private_material_error(phase8_readonly_initramfs_fixture()),
            HostKeyMaterialMetadata::missing()
        );
        assert_eq!(
            load_host_key_private_material_error(insufficient_host_key_initramfs()),
            HostKeyMaterialMetadata::insufficient(HOST_KEY_MIN_METADATA_BYTES - 1)
        );
        assert_eq!(
            load_host_key_private_material_error(malformed_host_key_initramfs()),
            HostKeyMaterialMetadata::invalid(Some(HOST_KEY_MIN_METADATA_BYTES))
        );
        assert_eq!(
            load_host_key_private_material_error(encrypted_host_key_initramfs()),
            HostKeyMaterialMetadata::invalid(Some(ENCRYPTED_OPENSSH_HOST_KEY_BYTES.len()))
        );
    }

    #[test_case]
    fn authorized_key_vfs_metadata_maps_to_fail_closed_states_without_reading_key_bytes() {
        let missing = classify_authorized_key_material(phase8_readonly_initramfs_fixture());
        let directory = classify_authorized_key_material(directory_authorized_key_initramfs());
        let empty = classify_authorized_key_material(empty_authorized_key_initramfs());
        let oversized = classify_authorized_key_material(oversized_authorized_key_initramfs());
        let insufficient =
            classify_authorized_key_material(insufficient_authorized_key_initramfs());
        let sufficient = classify_authorized_key_material(sufficient_authorized_key_initramfs());

        assert_eq!(missing, AuthorizedKeyMaterialMetadata::missing());
        assert_eq!(missing.byte_len(), None);
        assert_eq!(directory, AuthorizedKeyMaterialMetadata::invalid(Some(0)));
        assert_eq!(empty, AuthorizedKeyMaterialMetadata::invalid(Some(0)));
        assert_eq!(
            oversized,
            AuthorizedKeyMaterialMetadata::invalid(Some(AUTHORIZED_KEY_MAX_METADATA_BYTES + 1))
        );
        assert_eq!(
            insufficient,
            AuthorizedKeyMaterialMetadata::insufficient(AUTHORIZED_KEY_MIN_METADATA_BYTES - 1)
        );
        assert_eq!(
            sufficient,
            AuthorizedKeyMaterialMetadata::sufficient(AUTHORIZED_KEY_MIN_METADATA_BYTES)
        );
    }

    #[test_case]
    fn authorized_key_vfs_metadata_clears_only_authorized_key_prerequisite() {
        let invalid = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_authorized_key_material(
                classify_authorized_key_material(empty_authorized_key_initramfs()),
            ),
        );
        let insufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_authorized_key_material(
                classify_authorized_key_material(insufficient_authorized_key_initramfs()),
            ),
        );
        let sufficient = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default().with_authorized_key_material(
                classify_authorized_key_material(sufficient_authorized_key_initramfs()),
            ),
        );

        assert!(
            invalid
                .labels()
                .contains(&SshKeyReadinessLabel::InvalidAuthorizedKey)
        );
        assert!(
            insufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InsufficientAuthorizedKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::MissingAuthorizedKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InvalidAuthorizedKey)
        );
        assert!(
            !sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::InsufficientAuthorizedKey)
        );
        assert!(
            sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::MissingHostKey)
        );
        assert!(
            sufficient
                .labels()
                .contains(&SshKeyReadinessLabel::EntropyUnready)
        );
        assert!(!sufficient.ssh_ready());
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

    #[test_case]
    fn persistence_metadata_requires_all_accepted_material_sources() {
        let seed = OperatorSeedMaterialMetadata::sufficient(32);
        let host_key = HostKeyMaterialMetadata::sufficient(HOST_KEY_MIN_METADATA_BYTES);
        let authorized_key =
            AuthorizedKeyMaterialMetadata::sufficient(AUTHORIZED_KEY_MIN_METADATA_BYTES);

        assert_eq!(
            classify_persistence_metadata(seed, host_key, authorized_key),
            PersistenceState::MetadataPresent
        );
        assert_eq!(
            classify_persistence_metadata(
                OperatorSeedMaterialMetadata::missing(),
                host_key,
                authorized_key
            ),
            PersistenceState::Unavailable
        );
        assert_eq!(
            classify_persistence_metadata(
                seed,
                HostKeyMaterialMetadata::insufficient(HOST_KEY_MIN_METADATA_BYTES - 1),
                authorized_key
            ),
            PersistenceState::Unavailable
        );
        assert_eq!(
            classify_persistence_metadata(
                seed,
                host_key,
                AuthorizedKeyMaterialMetadata::invalid(Some(0))
            ),
            PersistenceState::Unavailable
        );
    }

    #[test_case]
    fn exposure_marker_vfs_metadata_fails_closed_until_explicit_marker_is_valid() {
        assert_eq!(
            classify_exposure_marker(phase8_readonly_initramfs_fixture()),
            ExposureState::Disabled
        );
        assert_eq!(
            classify_exposure_marker(directory_exposure_marker_initramfs()),
            ExposureState::Disabled
        );
        assert_eq!(
            classify_exposure_marker(oversized_exposure_marker_initramfs()),
            ExposureState::Disabled
        );
        assert_eq!(
            classify_exposure_marker(empty_exposure_marker_initramfs()),
            ExposureState::ExplicitlyEnabled
        );
    }

    #[test_case]
    fn persistence_and_exposure_metadata_clear_only_their_prerequisites() {
        let report = classify_ssh_key_readiness(
            SshKeyReadinessSnapshot::fail_closed_default()
                .with_host_key_material(HostKeyMaterialMetadata::sufficient(
                    HOST_KEY_MIN_METADATA_BYTES,
                ))
                .with_authorized_key_material(AuthorizedKeyMaterialMetadata::sufficient(
                    AUTHORIZED_KEY_MIN_METADATA_BYTES,
                ))
                .with_operator_seed_material(OperatorSeedMaterialMetadata::sufficient(32))
                .with_persistence_state(PersistenceState::MetadataPresent)
                .with_exposure_state(ExposureState::ExplicitlyEnabled),
        );

        assert!(
            !report
                .labels()
                .contains(&SshKeyReadinessLabel::PersistenceUnavailable)
        );
        assert!(
            !report
                .labels()
                .contains(&SshKeyReadinessLabel::ExposureDisabled)
        );
        assert!(
            report
                .labels()
                .contains(&SshKeyReadinessLabel::EntropyUnready)
        );
        assert!(report.labels().contains(&SshKeyReadinessLabel::NotReady));
        assert!(!report.ssh_ready());
    }

    #[test_case]
    fn csprng_ready_entropy_clears_only_entropy_prerequisite() {
        let entropy = entropy::classify_entropy_snapshot(
            EntropyDiagnosticSnapshot::empty()
                .with_operator_seed(OperatorSeedObservation::new(32))
                .with_csprng_ready(),
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

        assert!(
            !report
                .labels()
                .contains(&SshKeyReadinessLabel::EntropyUnready)
        );
        assert_eq!(report.labels(), &[SshKeyReadinessLabel::NotReady]);
        assert!(!report.ssh_ready());
    }

    const ROOT_INDEX: usize = 0;
    const ETC_INDEX: usize = 1;
    const TALOS_INDEX: usize = 2;
    const SSH_INDEX: usize = 3;
    const HOST_KEY_INDEX: usize = 4;
    const AUTHORIZED_KEY_INDEX: usize = 5;
    const EXPOSURE_MARKER_INDEX: usize = 4;

    static ROOT_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"etc", ETC_INDEX)];
    static ETC_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"talos", TALOS_INDEX)];
    static TALOS_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(b"ssh", SSH_INDEX)];
    static SSH_ENTRIES: [DirectoryEntry; 1] =
        [DirectoryEntry::new(b"ssh_host_ed25519_key", HOST_KEY_INDEX)];
    static SSH_AUTHORIZED_KEY_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(
        b"authorized_keys",
        AUTHORIZED_KEY_INDEX,
    )];
    static SSH_EXPOSURE_MARKER_ENTRIES: [DirectoryEntry; 1] = [DirectoryEntry::new(
        b"exposure-enabled",
        EXPOSURE_MARKER_INDEX,
    )];
    static EMPTY_ENTRIES: [DirectoryEntry; 0] = [];
    static INSUFFICIENT_HOST_KEY_BYTES: [u8; HOST_KEY_MIN_METADATA_BYTES - 1] =
        [0; HOST_KEY_MIN_METADATA_BYTES - 1];
    static MALFORMED_HOST_KEY_BYTES: [u8; HOST_KEY_MIN_METADATA_BYTES] =
        [0; HOST_KEY_MIN_METADATA_BYTES];
    static OVERSIZED_HOST_KEY_BYTES: [u8; HOST_KEY_MAX_METADATA_BYTES + 1] =
        [0; HOST_KEY_MAX_METADATA_BYTES + 1];
    static VALID_OPENSSH_ED25519_HOST_KEY_BYTES: &[u8] = br#"
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYgAAAJgAIAxdACAM
XQAAAAtzc2gtZWQyNTUxOQAAACCzPq7zfqLffKoBDe/eo04kH2XxtSmk9D7RQyf1xUqrYg
AAAEC2BsIi0QwW2uFscKTUUXNHLsYX4FxlaSDSblbAj7WR7bM+rvN+ot98qgEN796jTiQf
ZfG1KaT0PtFDJ/XFSqtiAAAAEHVzZXJAZXhhbXBsZS5jb20BAgMEBQ==
-----END OPENSSH PRIVATE KEY-----
"#;
    static ENCRYPTED_OPENSSH_HOST_KEY_BYTES: &[u8] = br#"
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0AAAAGAAAABBKH96ujW
umB6/WnTNPjTeaAAAAEAAAAAEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAILM+rvN+ot98qgEN
796jTiQfZfG1KaT0PtFDJ/XFSqtiAAAAoFzvbvyFMhAiwBOXF0mhUUacPUCMZXivG2up2c
hEnAw1b6BLRPyWbY5cC2n9ggD4ivJ1zSts6sBgjyiXQAReyrP35myYvT/OIB/NpwZM/xIJ
N7MHSUzlkX4adBrga3f7GS4uv4ChOoxC4XsE5HsxtGsq1X8jzqLlZTmOcxkcEneYQexrUc
bQP0o+gL5aKK8cQgiIlXeDbRjqhc4+h4EF6lY=
-----END OPENSSH PRIVATE KEY-----
"#;
    static INSUFFICIENT_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MIN_METADATA_BYTES - 1] =
        [0; AUTHORIZED_KEY_MIN_METADATA_BYTES - 1];
    static SUFFICIENT_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MIN_METADATA_BYTES] =
        [0; AUTHORIZED_KEY_MIN_METADATA_BYTES];
    static OVERSIZED_AUTHORIZED_KEY_BYTES: [u8; AUTHORIZED_KEY_MAX_METADATA_BYTES + 1] =
        [0; AUTHORIZED_KEY_MAX_METADATA_BYTES + 1];
    static OVERSIZED_EXPOSURE_MARKER_BYTES: [u8; EXPOSURE_MARKER_MAX_METADATA_BYTES + 1] =
        [0; EXPOSURE_MARKER_MAX_METADATA_BYTES + 1];

    static DIRECTORY_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::directory(HOST_KEY_INDEX, &EMPTY_ENTRIES),
    ];
    static EMPTY_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b""),
    ];
    static INSUFFICIENT_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, &INSUFFICIENT_HOST_KEY_BYTES),
    ];
    static SUFFICIENT_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, VALID_OPENSSH_ED25519_HOST_KEY_BYTES),
    ];
    static MALFORMED_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, &MALFORMED_HOST_KEY_BYTES),
    ];
    static ENCRYPTED_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, ENCRYPTED_OPENSSH_HOST_KEY_BYTES),
    ];
    static OVERSIZED_HOST_KEY_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, &OVERSIZED_HOST_KEY_BYTES),
    ];
    static DIRECTORY_AUTHORIZED_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::directory(AUTHORIZED_KEY_INDEX, &EMPTY_ENTRIES),
    ];
    static EMPTY_AUTHORIZED_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, b""),
    ];
    static INSUFFICIENT_AUTHORIZED_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &INSUFFICIENT_AUTHORIZED_KEY_BYTES),
    ];
    static SUFFICIENT_AUTHORIZED_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &SUFFICIENT_AUTHORIZED_KEY_BYTES),
    ];
    static OVERSIZED_AUTHORIZED_KEY_NODES: [InitramfsNode; 6] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_AUTHORIZED_KEY_ENTRIES),
        InitramfsNode::regular_file(HOST_KEY_INDEX, b"unused"),
        InitramfsNode::regular_file(AUTHORIZED_KEY_INDEX, &OVERSIZED_AUTHORIZED_KEY_BYTES),
    ];
    static DIRECTORY_EXPOSURE_MARKER_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_EXPOSURE_MARKER_ENTRIES),
        InitramfsNode::directory(EXPOSURE_MARKER_INDEX, &EMPTY_ENTRIES),
    ];
    static EMPTY_EXPOSURE_MARKER_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_EXPOSURE_MARKER_ENTRIES),
        InitramfsNode::regular_file(EXPOSURE_MARKER_INDEX, b""),
    ];
    static OVERSIZED_EXPOSURE_MARKER_NODES: [InitramfsNode; 5] = [
        InitramfsNode::directory(ROOT_INDEX, &ROOT_ENTRIES),
        InitramfsNode::directory(ETC_INDEX, &ETC_ENTRIES),
        InitramfsNode::directory(TALOS_INDEX, &TALOS_ENTRIES),
        InitramfsNode::directory(SSH_INDEX, &SSH_EXPOSURE_MARKER_ENTRIES),
        InitramfsNode::regular_file(EXPOSURE_MARKER_INDEX, &OVERSIZED_EXPOSURE_MARKER_BYTES),
    ];

    const fn directory_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&DIRECTORY_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn empty_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&EMPTY_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn insufficient_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn sufficient_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&SUFFICIENT_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn malformed_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&MALFORMED_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn encrypted_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&ENCRYPTED_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn oversized_host_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_HOST_KEY_NODES, ROOT_INDEX)
    }

    const fn directory_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&DIRECTORY_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn empty_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&EMPTY_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn insufficient_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&INSUFFICIENT_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn sufficient_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&SUFFICIENT_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn oversized_authorized_key_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_AUTHORIZED_KEY_NODES, ROOT_INDEX)
    }

    const fn directory_exposure_marker_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&DIRECTORY_EXPOSURE_MARKER_NODES, ROOT_INDEX)
    }

    const fn empty_exposure_marker_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&EMPTY_EXPOSURE_MARKER_NODES, ROOT_INDEX)
    }

    const fn oversized_exposure_marker_initramfs() -> ReadOnlyInitramfs {
        ReadOnlyInitramfs::new(&OVERSIZED_EXPOSURE_MARKER_NODES, ROOT_INDEX)
    }
}
