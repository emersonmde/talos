# v54 Runtime-Ready Staging Reconciliation Report

Task id: phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-staging-reconciliation-v54-20260701

## Accepted Inputs

- v51 selected archive:
  target/tmp/selected-normal-runtime-runtime-ready-v51.tar.gz.
- v51 archive SHA-256:
  44afdb8b849bd2fb1878a1b280e8e46b66cbcb5b48fc40dd2822fe06091c84e9.
- v51 selected fetch path: da591740/kernel_2712.img.
- v51 selected kernel bytes: 152,144.
- v51 selected kernel SHA-256:
  b3d4ff79d0790980f68ef446a7c41dcbe2858824bd29ce7f150f86fa053c7982.
- v51 required marker:
  TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-ready-static.
- v52 selected publication tree:
  c49997afe4dd2136706ad4f0dc05326d93abf60593c8a01104472984d5481bbc.
- v52 restore target:
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.

## v52 Reconciliation

v52 proved the archive could be published and observed through /boot/files
before the power action: the preflight identity matched the selected tree,
effective kernel, expected fetch path, and 152,144-byte kernel size.

The decisive hardware window did not retain that selected identity:

- stable same-cursor TFTP observed two da591740/kernel_2712.img serves, both
  at 104,136 bytes instead of the expected 152,144 bytes;
- final pre-restore identity was baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10;
- the final pre-restore da591740/kernel_2712.img entry was 104,136 bytes;
- the serial window retained zero occurrences of the required runtime-ready
  marker and zero occurrences of the runtime-ready nonce.

This evidence does not distinguish whether the selected tree is lost
immediately by the power-cycle/lab path, whether dnsmasq serves an older tree
despite selected /boot/files identity, or whether selected runtime execution
itself fails before the marker. v52 therefore cannot safely promote packet-I/O
or OpenSSH work.

## Accepted v54 Change

scripts/rpi5-capture-invariant-proof-bundle.sh now captures an immediate
post-power, pre-serial-observe identity checkpoint:

- post-power-pre-observe-status.json;
- post-power-pre-observe-root-endpoint.json and body;
- post-power-pre-observe-root.json;
- post-power-pre-observe-boot-files.json;
- capture-window-order stage post_power_pre_observe_identity;
- capture-invariant-summary.json fields post_power_pre_observe_identity and
  proof_run_identity.post_power_pre_observe.

The checkpoint is taken after POST /power/cycle and before the long serial
observe/TFTP wait. On the next Pi 5 preflight it can distinguish:

- selected identity already lost immediately after power cycle;
- selected identity still staged after power but dnsmasq serves baseline bytes;
- selected identity and selected TFTP bytes retained but runtime-ready marker
  absent.

This is a control discriminator, not runtime-ready proof. It changes the
evidence timing and source without accepting packet-I/O, OpenSSH/generated-root
retry, service readiness, ssh-ready=true, fake command expansion, broad shell
work, or a phase transition.

## Findings

- fixed: added the immediate post-power/pre-observe identity checkpoint to the
  capture helper and dry-run contract.
- fixed: retained the checkpoint in the annotated capture summary and
  proof-run identity map for future v55 classification.
- not-an-issue: v51 archive metadata remains authoritative; no source/runtime
  archive repair was required by this no-hardware reconciliation.
- deferred: the next serialized Pi 5 preflight must run the revised helper and
  classify the new post-power identity against TFTP byte counts, final
  pre-restore identity, serial marker retention, and restore proof.
- removed: packet-I/O, OpenSSH/generated-root retry, service readiness,
  ssh-ready=true, fake command expansion, broad shell work, and phase
  transition as successors before v55 hardware evidence.

## Redaction Review

No raw serial text, raw TFTP peer/log-line fields, packet payloads,
SSH/session/key material, boot artifact bytes, private user data, stable
secret-derived identifiers, public-key blobs, signatures, fingerprints, or
operator identifiers are retained by this report or the v54 dry-run artifact.
