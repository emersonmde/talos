//! Private SSH runtime key-exchange crypto boundary.
//!
//! The module performs one bounded curve25519-sha256 exchange for local
//! diagnostic integration. It exposes only caller-owned packet material and
//! fixed labels to the SSH service layer; durable diagnostics must not retain
//! the bytes produced here.

use alloc::vec::Vec;
use core::convert::Infallible;
use hmac::Hmac;
use sha2::{Digest, Sha256};
use ssh_cipher::Cipher;
use x25519_dalek::{
    EphemeralSecret, PublicKey,
    rand_core::{TryCryptoRng, TryRng},
};
use zeroize::Zeroize;

use crate::{
    csprng::{CsprngReadinessState, OperatorSeededCsprng},
    ssh_key_readiness::HostKeyPrivateMaterial,
};

const X25519_PUBLIC_KEY_BYTES: usize = 32;
const SHA256_BYTES: usize = 32;
const CHACHA20_POLY1305_KEY_BYTES: usize = 32;
const CHACHA20_POLY1305_IV_BYTES: usize = 8;
const SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH: &str = "chacha20-poly1305@openssh.com";
const SSH_KDF_IV_CLIENT_TO_SERVER: u8 = b'A';
const SSH_KDF_IV_SERVER_TO_CLIENT: u8 = b'B';
const SSH_KDF_KEY_CLIENT_TO_SERVER: u8 = b'C';
const SSH_KDF_KEY_SERVER_TO_CLIENT: u8 = b'D';

type NegotiatedHmacSha256 = Hmac<Sha256>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshRuntimeKexLabel {
    CryptoBackendReady,
    KexPeerPublicKeyInvalid,
    KexCsprngNotReady,
    KexHostKeyNotReady,
    KexTranscriptInvalid,
    #[allow(dead_code)]
    KexKeyDerivationFailed,
    EncryptedPacketStateNotReady,
    EncryptedPacketStateReady,
}

impl SshRuntimeKexLabel {
    #[allow(dead_code)]
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::CryptoBackendReady => "sshservicediag-crypto-backend-ready",
            Self::KexPeerPublicKeyInvalid => "sshservicediag-kex-peer-public-key-invalid",
            Self::KexCsprngNotReady => "sshservicediag-kex-csprng-not-ready",
            Self::KexHostKeyNotReady => "sshservicediag-kex-host-key-not-ready",
            Self::KexTranscriptInvalid => "sshservicediag-kex-transcript-invalid",
            Self::KexKeyDerivationFailed => "sshservicediag-kex-key-derivation-failed",
            Self::EncryptedPacketStateNotReady => "sshservicediag-encrypted-packet-state-not-ready",
            Self::EncryptedPacketStateReady => "sshservicediag-encrypted-packet-state-ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshRuntimeKexFailure {
    CsprngNotReady(CsprngReadinessState),
    HostKeyNotReady,
    InvalidPeerPublicKey,
    TranscriptInvalid,
    KeyDerivationFailed,
    EncryptedPacketStateNotReady,
}

impl SshRuntimeKexFailure {
    pub(crate) const fn label(self) -> SshRuntimeKexLabel {
        match self {
            Self::CsprngNotReady(_) => SshRuntimeKexLabel::KexCsprngNotReady,
            Self::HostKeyNotReady => SshRuntimeKexLabel::KexHostKeyNotReady,
            Self::InvalidPeerPublicKey => SshRuntimeKexLabel::KexPeerPublicKeyInvalid,
            Self::TranscriptInvalid => SshRuntimeKexLabel::KexTranscriptInvalid,
            Self::KeyDerivationFailed => SshRuntimeKexLabel::KexKeyDerivationFailed,
            Self::EncryptedPacketStateNotReady => SshRuntimeKexLabel::EncryptedPacketStateNotReady,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshRuntimeKexResultKind {
    Ready,
    Failed(SshRuntimeKexFailure),
}

pub(crate) struct SshRuntimeKexInput<'a> {
    pub(crate) client_identification: &'a [u8],
    pub(crate) server_identification: &'a [u8],
    pub(crate) client_kexinit_packet: &'a [u8],
    pub(crate) server_kexinit_packet: &'a [u8],
    pub(crate) peer_public_key: &'a [u8],
    pub(crate) host_key: Option<&'a HostKeyPrivateMaterial>,
    pub(crate) csprng: &'a mut OperatorSeededCsprng,
}

pub(crate) enum SshRuntimeKexResult {
    Ready(SshRuntimeKexReady),
    Failed(SshRuntimeKexFailure),
}

impl SshRuntimeKexResult {
    pub(crate) const fn kind(&self) -> SshRuntimeKexResultKind {
        match self {
            Self::Ready(_) => SshRuntimeKexResultKind::Ready,
            Self::Failed(failure) => SshRuntimeKexResultKind::Failed(*failure),
        }
    }

    pub(crate) const fn label(&self) -> SshRuntimeKexLabel {
        match self {
            Self::Ready(_) => SshRuntimeKexLabel::CryptoBackendReady,
            Self::Failed(failure) => failure.label(),
        }
    }

    pub(crate) const fn encrypted_packet_state_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }
}

pub(crate) struct SshRuntimeKexReady {
    local_public_key: [u8; X25519_PUBLIC_KEY_BYTES],
    host_key_public_blob: Vec<u8>,
    host_key_signature_blob: Vec<u8>,
    packet_states: SshEncryptedPacketStates,
}

impl SshRuntimeKexReady {
    pub(crate) const fn local_public_key(&self) -> &[u8; X25519_PUBLIC_KEY_BYTES] {
        &self.local_public_key
    }

    pub(crate) fn host_key_public_blob(&self) -> &[u8] {
        &self.host_key_public_blob
    }

    pub(crate) fn host_key_signature_blob(&self) -> &[u8] {
        &self.host_key_signature_blob
    }

    pub(crate) const fn packet_states(&self) -> &SshEncryptedPacketStates {
        &self.packet_states
    }
}

impl Drop for SshRuntimeKexReady {
    fn drop(&mut self) {
        self.host_key_signature_blob.zeroize();
    }
}

pub(crate) struct SshEncryptedPacketStates {
    client_to_server: SshEncryptedPacketState,
    server_to_client: SshEncryptedPacketState,
}

impl SshEncryptedPacketStates {
    pub(crate) const fn client_to_server(&self) -> &SshEncryptedPacketState {
        &self.client_to_server
    }

    pub(crate) const fn server_to_client(&self) -> &SshEncryptedPacketState {
        &self.server_to_client
    }
}

pub(crate) struct SshEncryptedPacketState {
    cipher: Cipher,
    key: [u8; CHACHA20_POLY1305_KEY_BYTES],
    iv: [u8; CHACHA20_POLY1305_IV_BYTES],
    sequence_number: u32,
}

impl SshEncryptedPacketState {
    pub(crate) fn cipher_name(&self) -> &'static str {
        self.cipher.as_str()
    }

    pub(crate) const fn key_len(&self) -> usize {
        self.key.len()
    }

    pub(crate) const fn iv_len(&self) -> usize {
        self.iv.len()
    }

    pub(crate) const fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
}

impl Drop for SshEncryptedPacketState {
    fn drop(&mut self) {
        self.key.zeroize();
        self.iv.zeroize();
    }
}

pub(crate) fn perform_runtime_kex(input: SshRuntimeKexInput<'_>) -> SshRuntimeKexResult {
    let _accepted_mac_surface = core::mem::size_of::<NegotiatedHmacSha256>();

    if !input.csprng.readiness().cryptographic_strength() {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::CsprngNotReady(
            input.csprng.readiness().state(),
        ));
    }
    let Some(host_key) = input.host_key else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::HostKeyNotReady);
    };
    if input.peer_public_key.len() != X25519_PUBLIC_KEY_BYTES {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::InvalidPeerPublicKey);
    };

    let Ok(client_kexinit_payload) = kexinit_payload(input.client_kexinit_packet) else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::TranscriptInvalid);
    };
    let Ok(server_kexinit_payload) = kexinit_payload(input.server_kexinit_packet) else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::TranscriptInvalid);
    };
    let Some(client_identification) = identification_without_crlf(input.client_identification)
    else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::TranscriptInvalid);
    };
    let Some(server_identification) = identification_without_crlf(input.server_identification)
    else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::TranscriptInvalid);
    };

    let Ok(host_key_public_blob) = host_key.public_key_blob() else {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::HostKeyNotReady);
    };
    let mut peer_public_key = [0u8; X25519_PUBLIC_KEY_BYTES];
    peer_public_key.copy_from_slice(input.peer_public_key);
    let peer_public_key = PublicKey::from(peer_public_key);

    let mut adapter = OperatorSeededCryptoRng::new(input.csprng);
    let local_secret = EphemeralSecret::random_from_rng(&mut adapter);
    let local_public_key = PublicKey::from(&local_secret).to_bytes();
    let shared_secret = local_secret.diffie_hellman(&peer_public_key);
    if !shared_secret.was_contributory() {
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::InvalidPeerPublicKey);
    }

    let mut shared_secret_bytes = shared_secret.to_bytes();
    let Some(mut exchange_hash) = compute_exchange_hash(
        client_identification,
        server_identification,
        client_kexinit_payload,
        server_kexinit_payload,
        &host_key_public_blob,
        input.peer_public_key,
        &local_public_key,
        &shared_secret_bytes,
    ) else {
        shared_secret_bytes.zeroize();
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::TranscriptInvalid);
    };

    let Ok(signature) = host_key.sign_exchange_hash(&exchange_hash) else {
        shared_secret_bytes.zeroize();
        exchange_hash.zeroize();
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::HostKeyNotReady);
    };
    let Ok(host_key_signature_blob) = signature.encoded_blob() else {
        shared_secret_bytes.zeroize();
        exchange_hash.zeroize();
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::HostKeyNotReady);
    };
    let Some(packet_states) = derive_packet_states(&shared_secret_bytes, &exchange_hash) else {
        shared_secret_bytes.zeroize();
        exchange_hash.zeroize();
        return SshRuntimeKexResult::Failed(SshRuntimeKexFailure::EncryptedPacketStateNotReady);
    };
    shared_secret_bytes.zeroize();
    exchange_hash.zeroize();

    SshRuntimeKexResult::Ready(SshRuntimeKexReady {
        local_public_key,
        host_key_public_blob,
        host_key_signature_blob,
        packet_states,
    })
}

fn compute_exchange_hash(
    client_identification: &[u8],
    server_identification: &[u8],
    client_kexinit_payload: &[u8],
    server_kexinit_payload: &[u8],
    host_key_public_blob: &[u8],
    client_public_key: &[u8],
    server_public_key: &[u8],
    shared_secret: &[u8; X25519_PUBLIC_KEY_BYTES],
) -> Option<[u8; SHA256_BYTES]> {
    let mut hash = Sha256::new();
    update_string(&mut hash, client_identification)?;
    update_string(&mut hash, server_identification)?;
    update_string(&mut hash, client_kexinit_payload)?;
    update_string(&mut hash, server_kexinit_payload)?;
    update_string(&mut hash, host_key_public_blob)?;
    update_string(&mut hash, client_public_key)?;
    update_string(&mut hash, server_public_key)?;
    update_mpint(&mut hash, shared_secret)?;
    let digest = hash.finalize();
    let mut output = [0u8; SHA256_BYTES];
    output.copy_from_slice(&digest);
    Some(output)
}

fn derive_packet_states(
    shared_secret: &[u8; X25519_PUBLIC_KEY_BYTES],
    exchange_hash: &[u8; SHA256_BYTES],
) -> Option<SshEncryptedPacketStates> {
    let cipher = Cipher::new(SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH).ok()?;
    let (key_len, iv_len) = cipher.key_and_iv_size()?;
    if key_len != CHACHA20_POLY1305_KEY_BYTES || iv_len != CHACHA20_POLY1305_IV_BYTES {
        return None;
    }

    let mut client_to_server_iv = [0u8; CHACHA20_POLY1305_IV_BYTES];
    let mut server_to_client_iv = [0u8; CHACHA20_POLY1305_IV_BYTES];
    let mut client_to_server_key = [0u8; CHACHA20_POLY1305_KEY_BYTES];
    let mut server_to_client_key = [0u8; CHACHA20_POLY1305_KEY_BYTES];

    derive_key_material(
        shared_secret,
        exchange_hash,
        SSH_KDF_IV_CLIENT_TO_SERVER,
        &mut client_to_server_iv,
    )?;
    derive_key_material(
        shared_secret,
        exchange_hash,
        SSH_KDF_IV_SERVER_TO_CLIENT,
        &mut server_to_client_iv,
    )?;
    derive_key_material(
        shared_secret,
        exchange_hash,
        SSH_KDF_KEY_CLIENT_TO_SERVER,
        &mut client_to_server_key,
    )?;
    derive_key_material(
        shared_secret,
        exchange_hash,
        SSH_KDF_KEY_SERVER_TO_CLIENT,
        &mut server_to_client_key,
    )?;

    Some(SshEncryptedPacketStates {
        client_to_server: SshEncryptedPacketState {
            cipher,
            key: client_to_server_key,
            iv: client_to_server_iv,
            sequence_number: 0,
        },
        server_to_client: SshEncryptedPacketState {
            cipher,
            key: server_to_client_key,
            iv: server_to_client_iv,
            sequence_number: 0,
        },
    })
}

fn derive_key_material(
    shared_secret: &[u8; X25519_PUBLIC_KEY_BYTES],
    exchange_hash: &[u8; SHA256_BYTES],
    letter: u8,
    output: &mut [u8],
) -> Option<()> {
    let mut offset = 0usize;
    let mut previous = [0u8; SHA256_BYTES];
    let mut previous_len = 0usize;
    while offset < output.len() {
        let mut hash = Sha256::new();
        update_mpint(&mut hash, shared_secret)?;
        hash.update(exchange_hash);
        if previous_len == 0 {
            hash.update([letter]);
            hash.update(exchange_hash);
        } else {
            hash.update(&previous[..previous_len]);
        }
        let digest = hash.finalize();
        previous.copy_from_slice(&digest);
        previous_len = previous.len();
        let take = core::cmp::min(output.len() - offset, previous_len);
        output[offset..offset + take].copy_from_slice(&previous[..take]);
        offset += take;
    }
    previous.zeroize();
    Some(())
}

fn update_string(hash: &mut Sha256, bytes: &[u8]) -> Option<()> {
    let len = u32::try_from(bytes.len()).ok()?;
    hash.update(len.to_be_bytes());
    hash.update(bytes);
    Some(())
}

fn update_mpint(hash: &mut Sha256, bytes: &[u8; X25519_PUBLIC_KEY_BYTES]) -> Option<()> {
    let mut start = 0usize;
    while start < bytes.len() && bytes[start] == 0 {
        start += 1;
    }
    if start == bytes.len() {
        hash.update(0u32.to_be_bytes());
        return Some(());
    }
    let needs_positive_prefix = bytes[start] & 0x80 != 0;
    let mpint_len = bytes.len() - start + usize::from(needs_positive_prefix);
    let mpint_len = u32::try_from(mpint_len).ok()?;
    hash.update(mpint_len.to_be_bytes());
    if needs_positive_prefix {
        hash.update([0u8]);
    }
    hash.update(&bytes[start..]);
    Some(())
}

fn kexinit_payload(packet: &[u8]) -> Result<&[u8], ()> {
    if packet.len() < 6 {
        return Err(());
    }
    let packet_length = u32::from_be_bytes(packet[0..4].try_into().map_err(|_| ())?) as usize;
    if packet.len() != packet_length + 4 {
        return Err(());
    }
    let padding_length = packet[4] as usize;
    if packet_length <= padding_length + 1 {
        return Err(());
    }
    let payload_len = packet_length - padding_length - 1;
    let payload_start = 5usize;
    let payload_end = payload_start.checked_add(payload_len).ok_or(())?;
    packet.get(payload_start..payload_end).ok_or(())
}

fn identification_without_crlf(bytes: &[u8]) -> Option<&[u8]> {
    if bytes.is_empty() {
        return None;
    }
    if bytes.ends_with(b"\r\n") {
        Some(&bytes[..bytes.len() - 2])
    } else if bytes.ends_with(b"\n") {
        Some(&bytes[..bytes.len() - 1])
    } else {
        Some(bytes)
    }
}

struct OperatorSeededCryptoRng<'a> {
    csprng: &'a mut OperatorSeededCsprng,
}

impl<'a> OperatorSeededCryptoRng<'a> {
    const fn new(csprng: &'a mut OperatorSeededCsprng) -> Self {
        Self { csprng }
    }
}

impl TryRng for OperatorSeededCryptoRng<'_> {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut bytes = [0u8; 4];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut bytes = [0u8; 8];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        match self.csprng.fill_bytes(dst) {
            Ok(()) => Ok(()),
            Err(_) => {
                dst.zeroize();
                Ok(())
            }
        }
    }
}

impl TryCryptoRng for OperatorSeededCryptoRng<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        csprng::OperatorSeededCsprng,
        ssh_key_readiness,
        ssh_service_readiness::{
            SSH_LOCAL_IDENTIFICATION, build_modeled_client_kexinit_packet_for_runtime_test,
        },
    };

    const PUBLIC_FIXTURE_SEED: [u8; 48] = [
        0x70, 0x68, 0x61, 0x73, 0x65, 0x31, 0x32, 0x2d, 0x63, 0x73, 0x70, 0x72, 0x6e, 0x67, 0x2d,
        0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x2d, 0x66, 0x69, 0x78, 0x74, 0x75, 0x72, 0x65, 0x2d,
        0x76, 0x31, 0x2d, 0x6e, 0x6f, 0x74, 0x2d, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x21, 0x21,
        0x21, 0x21, 0x21,
    ];
    const CLIENT_IDENTIFICATION: &[u8] = b"SSH-2.0-local-model\r\n";
    const PEER_PUBLIC_KEY: [u8; X25519_PUBLIC_KEY_BYTES] = [
        9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    #[test_case]
    fn runtime_kex_success_uses_real_crypto_and_private_packet_state_handles() {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let mut client_kexinit = [0u8; 1028];
        let mut server_kexinit = [0u8; 1028];
        let client_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut client_kexinit, false);
        let server_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut server_kexinit, false);

        let result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: CLIENT_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &PEER_PUBLIC_KEY,
            host_key: Some(&host_key),
            csprng: &mut csprng,
        });

        let SshRuntimeKexResult::Ready(ready) = result else {
            panic!("runtime KEX should produce caller-owned packet material");
        };
        assert_eq!(ready.local_public_key().len(), X25519_PUBLIC_KEY_BYTES);
        assert_ne!(ready.local_public_key(), &[0u8; X25519_PUBLIC_KEY_BYTES]);
        assert!(!ready.host_key_public_blob().is_empty());
        assert!(!ready.host_key_signature_blob().is_empty());
        assert_eq!(
            ready.packet_states().client_to_server().cipher_name(),
            SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH
        );
        assert_eq!(
            ready.packet_states().server_to_client().cipher_name(),
            SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH
        );
        assert_eq!(
            ready.packet_states().client_to_server().key_len(),
            CHACHA20_POLY1305_KEY_BYTES
        );
        assert_eq!(
            ready.packet_states().client_to_server().iv_len(),
            CHACHA20_POLY1305_IV_BYTES
        );
        assert_eq!(
            ready.packet_states().client_to_server().sequence_number(),
            0
        );
    }

    #[test_case]
    fn runtime_kex_fail_closed_labels_cover_missing_prerequisites() {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut not_ready_csprng = OperatorSeededCsprng::from_seed_bytes(b"short");
        let mut ready_csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let mut client_kexinit = [0u8; 1028];
        let mut server_kexinit = [0u8; 1028];
        let client_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut client_kexinit, false);
        let server_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut server_kexinit, false);

        let csprng_result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: CLIENT_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &PEER_PUBLIC_KEY,
            host_key: Some(&host_key),
            csprng: &mut not_ready_csprng,
        });
        assert_eq!(csprng_result.label(), SshRuntimeKexLabel::KexCsprngNotReady);

        let host_key_result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: CLIENT_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &PEER_PUBLIC_KEY,
            host_key: None,
            csprng: &mut ready_csprng,
        });
        assert_eq!(
            host_key_result.label(),
            SshRuntimeKexLabel::KexHostKeyNotReady
        );

        let mut ready_csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let invalid_peer = [0u8; X25519_PUBLIC_KEY_BYTES];
        let peer_result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: CLIENT_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &invalid_peer,
            host_key: Some(&host_key),
            csprng: &mut ready_csprng,
        });
        assert_eq!(
            peer_result.label(),
            SshRuntimeKexLabel::KexPeerPublicKeyInvalid
        );

        let mut ready_csprng = OperatorSeededCsprng::from_seed_bytes(&PUBLIC_FIXTURE_SEED);
        let transcript_result = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: b"",
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &PEER_PUBLIC_KEY,
            host_key: Some(&host_key),
            csprng: &mut ready_csprng,
        });
        assert_eq!(
            transcript_result.label(),
            SshRuntimeKexLabel::KexTranscriptInvalid
        );
        assert_eq!(
            SshRuntimeKexFailure::KeyDerivationFailed.label(),
            SshRuntimeKexLabel::KexKeyDerivationFailed
        );
    }
}
