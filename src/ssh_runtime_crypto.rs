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
const SSH_ENCRYPTED_PACKET_MIN_BYTES: usize = 6;

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
    NewkeysNotReady,
    NewkeysSendActive,
    NewkeysReceiveActive,
    EncryptedPacketStateActive,
    EncryptedPacketSequenceAdvanced,
    EncryptedPacketSequenceOverflow,
    EncryptedPacketCryptoFailed,
    EncryptedPacketDiagnosticReady,
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

    pub(crate) const fn packet_states_mut(&mut self) -> &mut SshEncryptedPacketStates {
        &mut self.packet_states
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
    send_newkeys_active: bool,
    receive_newkeys_active: bool,
}

impl SshEncryptedPacketStates {
    pub(crate) const fn client_to_server(&self) -> &SshEncryptedPacketState {
        &self.client_to_server
    }

    pub(crate) const fn server_to_client(&self) -> &SshEncryptedPacketState {
        &self.server_to_client
    }

    pub(crate) fn activate_send_newkeys(&mut self) -> SshNewkeysActivationReport {
        self.send_newkeys_active = true;
        SshNewkeysActivationReport {
            label: SshRuntimeKexLabel::NewkeysSendActive,
            send_active: self.send_newkeys_active,
            receive_active: self.receive_newkeys_active,
            encrypted_packet_state_active: self.encrypted_packet_state_active(),
        }
    }

    pub(crate) fn activate_receive_newkeys(&mut self) -> SshNewkeysActivationReport {
        self.receive_newkeys_active = true;
        SshNewkeysActivationReport {
            label: SshRuntimeKexLabel::NewkeysReceiveActive,
            send_active: self.send_newkeys_active,
            receive_active: self.receive_newkeys_active,
            encrypted_packet_state_active: self.encrypted_packet_state_active(),
        }
    }

    pub(crate) const fn send_newkeys_active(&self) -> bool {
        self.send_newkeys_active
    }

    pub(crate) const fn receive_newkeys_active(&self) -> bool {
        self.receive_newkeys_active
    }

    pub(crate) const fn encrypted_packet_state_active(&self) -> bool {
        self.send_newkeys_active && self.receive_newkeys_active
    }

    pub(crate) const fn plaintext_io_label(&self) -> SshRuntimeKexLabel {
        if self.send_newkeys_active || self.receive_newkeys_active {
            SshRuntimeKexLabel::EncryptedPacketCryptoFailed
        } else {
            SshRuntimeKexLabel::NewkeysNotReady
        }
    }

    pub(crate) fn run_diagnostic(
        &mut self,
        direction: SshEncryptedPacketDirection,
        packet: &mut [u8],
    ) -> SshEncryptedPacketDiagnosticReport {
        if !self.encrypted_packet_state_active() {
            return SshEncryptedPacketDiagnosticReport::failed(
                direction,
                SshRuntimeKexLabel::NewkeysNotReady,
                self.direction_state(direction).sequence_number(),
                self,
            );
        }

        let state = self.direction_state_mut(direction);
        let sequence_before = state.sequence_number;
        if sequence_before == u32::MAX {
            return SshEncryptedPacketDiagnosticReport::failed(
                direction,
                SshRuntimeKexLabel::EncryptedPacketSequenceOverflow,
                sequence_before,
                self,
            );
        }
        if !encrypted_packet_shape_valid(packet) {
            return SshEncryptedPacketDiagnosticReport::failed(
                direction,
                SshRuntimeKexLabel::EncryptedPacketCryptoFailed,
                sequence_before,
                self,
            );
        }

        let tag = match state.cipher.encrypt(&state.key, &state.iv, packet) {
            Ok(Some(tag)) => tag,
            Ok(None) => {
                return SshEncryptedPacketDiagnosticReport::failed(
                    direction,
                    SshRuntimeKexLabel::EncryptedPacketCryptoFailed,
                    sequence_before,
                    self,
                );
            }
            Err(_) => {
                packet.zeroize();
                return SshEncryptedPacketDiagnosticReport::failed(
                    direction,
                    SshRuntimeKexLabel::EncryptedPacketCryptoFailed,
                    sequence_before,
                    self,
                );
            }
        };
        let mut tag = tag;
        tag.zeroize();
        packet.zeroize();
        state.sequence_number = sequence_before + 1;
        SshEncryptedPacketDiagnosticReport::advanced(
            direction,
            sequence_before,
            state.sequence_number,
            self,
        )
    }

    fn direction_state(&self, direction: SshEncryptedPacketDirection) -> &SshEncryptedPacketState {
        match direction {
            SshEncryptedPacketDirection::Send => &self.server_to_client,
            SshEncryptedPacketDirection::Receive => &self.client_to_server,
        }
    }

    fn direction_state_mut(
        &mut self,
        direction: SshEncryptedPacketDirection,
    ) -> &mut SshEncryptedPacketState {
        match direction {
            SshEncryptedPacketDirection::Send => &mut self.server_to_client,
            SshEncryptedPacketDirection::Receive => &mut self.client_to_server,
        }
    }

    #[cfg(test)]
    pub(crate) fn force_sequence_number_for_test(
        &mut self,
        direction: SshEncryptedPacketDirection,
        sequence_number: u32,
    ) {
        self.direction_state_mut(direction).sequence_number = sequence_number;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshNewkeysActivationReport {
    label: SshRuntimeKexLabel,
    send_active: bool,
    receive_active: bool,
    encrypted_packet_state_active: bool,
}

impl SshNewkeysActivationReport {
    pub(crate) const fn label(self) -> SshRuntimeKexLabel {
        self.label
    }

    pub(crate) const fn send_active(self) -> bool {
        self.send_active
    }

    pub(crate) const fn receive_active(self) -> bool {
        self.receive_active
    }

    pub(crate) const fn encrypted_packet_state_active(self) -> bool {
        self.encrypted_packet_state_active
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SshEncryptedPacketDirection {
    Send,
    Receive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SshEncryptedPacketDiagnosticReport {
    labels: [SshRuntimeKexLabel; 3],
    label_count: usize,
    direction: SshEncryptedPacketDirection,
    sequence_before: u32,
    sequence_after: u32,
    send_active: bool,
    receive_active: bool,
    encrypted_packet_state_active: bool,
    cipher_name: &'static str,
    key_len: usize,
    iv_len: usize,
}

impl SshEncryptedPacketDiagnosticReport {
    fn failed(
        direction: SshEncryptedPacketDirection,
        label: SshRuntimeKexLabel,
        sequence_number: u32,
        states: &SshEncryptedPacketStates,
    ) -> Self {
        let state = states.direction_state(direction);
        Self {
            labels: [label; 3],
            label_count: 1,
            direction,
            sequence_before: sequence_number,
            sequence_after: sequence_number,
            send_active: states.send_newkeys_active(),
            receive_active: states.receive_newkeys_active(),
            encrypted_packet_state_active: states.encrypted_packet_state_active(),
            cipher_name: state.cipher_name(),
            key_len: state.key_len(),
            iv_len: state.iv_len(),
        }
    }

    fn advanced(
        direction: SshEncryptedPacketDirection,
        sequence_before: u32,
        sequence_after: u32,
        states: &SshEncryptedPacketStates,
    ) -> Self {
        let state = states.direction_state(direction);
        Self {
            labels: [
                SshRuntimeKexLabel::EncryptedPacketStateActive,
                SshRuntimeKexLabel::EncryptedPacketSequenceAdvanced,
                SshRuntimeKexLabel::EncryptedPacketDiagnosticReady,
            ],
            label_count: 3,
            direction,
            sequence_before,
            sequence_after,
            send_active: states.send_newkeys_active(),
            receive_active: states.receive_newkeys_active(),
            encrypted_packet_state_active: states.encrypted_packet_state_active(),
            cipher_name: state.cipher_name(),
            key_len: state.key_len(),
            iv_len: state.iv_len(),
        }
    }

    pub(crate) fn labels(&self) -> &[SshRuntimeKexLabel] {
        &self.labels[..self.label_count]
    }

    pub(crate) const fn direction(self) -> SshEncryptedPacketDirection {
        self.direction
    }

    pub(crate) const fn sequence_before(self) -> u32 {
        self.sequence_before
    }

    pub(crate) const fn sequence_after(self) -> u32 {
        self.sequence_after
    }

    pub(crate) const fn send_active(self) -> bool {
        self.send_active
    }

    pub(crate) const fn receive_active(self) -> bool {
        self.receive_active
    }

    pub(crate) const fn encrypted_packet_state_active(self) -> bool {
        self.encrypted_packet_state_active
    }

    pub(crate) const fn cipher_name(self) -> &'static str {
        self.cipher_name
    }

    pub(crate) const fn key_len(self) -> usize {
        self.key_len
    }

    pub(crate) const fn iv_len(self) -> usize {
        self.iv_len
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
        send_newkeys_active: false,
        receive_newkeys_active: false,
    })
}

fn encrypted_packet_shape_valid(packet: &[u8]) -> bool {
    if packet.len() < SSH_ENCRYPTED_PACKET_MIN_BYTES {
        return false;
    }
    let Some(length_bytes) = packet.get(0..4) else {
        return false;
    };
    let packet_length = u32::from_be_bytes([
        length_bytes[0],
        length_bytes[1],
        length_bytes[2],
        length_bytes[3],
    ]) as usize;
    if packet_length + 4 != packet.len() {
        return false;
    }
    let padding_length = packet[4] as usize;
    packet_length > padding_length + 1
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

    fn runtime_kex_ready() -> SshRuntimeKexReady {
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
        ready
    }

    fn public_fixture_packet() -> [u8; 16] {
        [
            0, 0, 0, 12, 4, 0x15, b'T', b'a', b'l', b'o', b's', b'!', 0, 0, 0, 0,
        ]
    }

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

    #[test_case]
    fn newkeys_activation_is_independent_and_diagnostic_advances_one_sequence() {
        let mut ready = runtime_kex_ready();
        let states = ready.packet_states_mut();
        assert!(!states.send_newkeys_active());
        assert!(!states.receive_newkeys_active());
        assert!(!states.encrypted_packet_state_active());

        let send = states.activate_send_newkeys();
        assert_eq!(send.label(), SshRuntimeKexLabel::NewkeysSendActive);
        assert!(send.send_active());
        assert!(!send.receive_active());
        assert!(!send.encrypted_packet_state_active());
        assert_eq!(
            states.plaintext_io_label(),
            SshRuntimeKexLabel::EncryptedPacketCryptoFailed
        );

        let mut packet = public_fixture_packet();
        let blocked =
            states.run_diagnostic(SshEncryptedPacketDirection::Send, packet.as_mut_slice());
        assert_eq!(blocked.labels(), &[SshRuntimeKexLabel::NewkeysNotReady]);
        assert_eq!(blocked.sequence_before(), 0);
        assert_eq!(blocked.sequence_after(), 0);
        assert!(!blocked.encrypted_packet_state_active());

        let receive = states.activate_receive_newkeys();
        assert_eq!(receive.label(), SshRuntimeKexLabel::NewkeysReceiveActive);
        assert!(receive.send_active());
        assert!(receive.receive_active());
        assert!(receive.encrypted_packet_state_active());

        let mut packet = public_fixture_packet();
        let advanced =
            states.run_diagnostic(SshEncryptedPacketDirection::Send, packet.as_mut_slice());
        assert_eq!(
            advanced.labels(),
            &[
                SshRuntimeKexLabel::EncryptedPacketStateActive,
                SshRuntimeKexLabel::EncryptedPacketSequenceAdvanced,
                SshRuntimeKexLabel::EncryptedPacketDiagnosticReady,
            ]
        );
        assert_eq!(advanced.direction(), SshEncryptedPacketDirection::Send);
        assert_eq!(advanced.sequence_before(), 0);
        assert_eq!(advanced.sequence_after(), 1);
        assert!(advanced.send_active());
        assert!(advanced.receive_active());
        assert!(advanced.encrypted_packet_state_active());
        assert_eq!(
            advanced.cipher_name(),
            SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH
        );
        assert_eq!(advanced.key_len(), CHACHA20_POLY1305_KEY_BYTES);
        assert_eq!(advanced.iv_len(), CHACHA20_POLY1305_IV_BYTES);
        assert_eq!(states.server_to_client().sequence_number(), 1);
        assert_eq!(states.client_to_server().sequence_number(), 0);
    }

    #[test_case]
    fn encrypted_packet_diagnostic_fails_closed_for_overflow_and_malformed_packet() {
        let mut ready = runtime_kex_ready();
        let states = ready.packet_states_mut();
        states.activate_send_newkeys();
        states.activate_receive_newkeys();

        let mut malformed = [0u8; 5];
        let malformed_report = states.run_diagnostic(
            SshEncryptedPacketDirection::Receive,
            malformed.as_mut_slice(),
        );
        assert_eq!(
            malformed_report.labels(),
            &[SshRuntimeKexLabel::EncryptedPacketCryptoFailed]
        );
        assert_eq!(malformed_report.sequence_before(), 0);
        assert_eq!(malformed_report.sequence_after(), 0);
        assert_eq!(states.client_to_server().sequence_number(), 0);

        states.force_sequence_number_for_test(SshEncryptedPacketDirection::Receive, u32::MAX);
        let mut packet = public_fixture_packet();
        let overflow =
            states.run_diagnostic(SshEncryptedPacketDirection::Receive, packet.as_mut_slice());
        assert_eq!(
            overflow.labels(),
            &[SshRuntimeKexLabel::EncryptedPacketSequenceOverflow]
        );
        assert_eq!(overflow.sequence_before(), u32::MAX);
        assert_eq!(overflow.sequence_after(), u32::MAX);
        assert_eq!(states.client_to_server().sequence_number(), u32::MAX);
    }

    #[test_case]
    fn newkeys_packet_crypto_smoke_retains_fixed_label_evidence() {
        let host_key = ssh_key_readiness::public_fixture_host_key_private_material();
        let mut not_ready_csprng = OperatorSeededCsprng::from_seed_bytes(b"short");
        let mut client_kexinit = [0u8; 1028];
        let mut server_kexinit = [0u8; 1028];
        let client_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut client_kexinit, false);
        let server_len =
            build_modeled_client_kexinit_packet_for_runtime_test(&mut server_kexinit, false);
        let missing_kex = perform_runtime_kex(SshRuntimeKexInput {
            client_identification: CLIENT_IDENTIFICATION,
            server_identification: SSH_LOCAL_IDENTIFICATION.as_bytes(),
            client_kexinit_packet: &client_kexinit[..client_len],
            server_kexinit_packet: &server_kexinit[..server_len],
            peer_public_key: &PEER_PUBLIC_KEY,
            host_key: Some(&host_key),
            csprng: &mut not_ready_csprng,
        });
        assert_eq!(
            missing_kex.label().name(),
            "sshservicediag-kex-csprng-not-ready"
        );
        assert!(!missing_kex.encrypted_packet_state_ready());

        let mut ready = runtime_kex_ready();
        let states = ready.packet_states_mut();
        let mut packet = public_fixture_packet();
        let missing_both =
            states.run_diagnostic(SshEncryptedPacketDirection::Send, packet.as_mut_slice());
        assert_label_names(missing_both.labels(), &["sshservicediag-newkeys-not-ready"]);
        assert_eq!(missing_both.sequence_before(), 0);
        assert_eq!(missing_both.sequence_after(), 0);

        let send = states.activate_send_newkeys();
        assert_eq!(send.label().name(), "sshservicediag-newkeys-send-active");
        assert!(send.send_active());
        assert!(!send.receive_active());
        assert!(!send.encrypted_packet_state_active());

        let mut packet = public_fixture_packet();
        let missing_receive =
            states.run_diagnostic(SshEncryptedPacketDirection::Send, packet.as_mut_slice());
        assert_label_names(
            missing_receive.labels(),
            &["sshservicediag-newkeys-not-ready"],
        );
        assert_eq!(missing_receive.sequence_before(), 0);
        assert_eq!(missing_receive.sequence_after(), 0);

        let receive = states.activate_receive_newkeys();
        assert_eq!(
            receive.label().name(),
            "sshservicediag-newkeys-receive-active"
        );
        assert!(receive.send_active());
        assert!(receive.receive_active());
        assert!(receive.encrypted_packet_state_active());

        let mut packet = public_fixture_packet();
        let advanced =
            states.run_diagnostic(SshEncryptedPacketDirection::Send, packet.as_mut_slice());
        assert_label_names(
            advanced.labels(),
            &[
                "sshservicediag-encrypted-packet-state-active",
                "sshservicediag-encrypted-packet-sequence-advanced",
                "sshservicediag-encrypted-packet-diagnostic-ready",
            ],
        );
        assert_eq!(advanced.sequence_before(), 0);
        assert_eq!(advanced.sequence_after(), 1);
        assert_eq!(
            advanced.cipher_name(),
            SSH_CIPHER_NAME_CHACHA20_POLY1305_OPENSSH
        );
        assert_eq!(advanced.key_len(), CHACHA20_POLY1305_KEY_BYTES);
        assert_eq!(advanced.iv_len(), CHACHA20_POLY1305_IV_BYTES);

        let mut malformed = [0u8; 5];
        let malformed_report = states.run_diagnostic(
            SshEncryptedPacketDirection::Receive,
            malformed.as_mut_slice(),
        );
        assert_label_names(
            malformed_report.labels(),
            &["sshservicediag-encrypted-packet-crypto-failed"],
        );
        assert_eq!(malformed_report.sequence_before(), 0);
        assert_eq!(malformed_report.sequence_after(), 0);

        states.force_sequence_number_for_test(SshEncryptedPacketDirection::Receive, u32::MAX);
        let mut packet = public_fixture_packet();
        let overflow =
            states.run_diagnostic(SshEncryptedPacketDirection::Receive, packet.as_mut_slice());
        assert_label_names(
            overflow.labels(),
            &["sshservicediag-encrypted-packet-sequence-overflow"],
        );
        assert_eq!(overflow.sequence_before(), u32::MAX);
        assert_eq!(overflow.sequence_after(), u32::MAX);
    }

    fn assert_label_names(labels: &[SshRuntimeKexLabel], expected: &[&str]) {
        assert_eq!(labels.len(), expected.len());
        let mut index = 0usize;
        while index < labels.len() {
            assert_eq!(labels[index].name(), expected[index]);
            index += 1;
        }
    }
}
