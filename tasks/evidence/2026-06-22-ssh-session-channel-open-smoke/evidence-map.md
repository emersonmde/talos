# SSH session channel-open smoke evidence map

Task id: phase12-shell-ssh-session-channel-open-smoke-20260622

Classification: host-qemu-substitute-shell-ssh-session-channel-open-smoke-complete.

## Retained transcript

- qemu-shell-ssh-session-channel-open-smoke.log

## Coverage map

- success path:
  session_channel_open_accepts_one_modeled_authenticated_session_only proves the
  accepted authenticated SSH_MSG_CHANNEL_OPEN session request returns
  SSH_MSG_CHANNEL_OPEN_CONFIRMATION and advances session-count/channel-count to
  1 while shell-attached=false, live-reachability=false, and ssh-ready=false.
- prerequisite and policy failures:
  session_channel_open_fails_closed_for_prerequisites_and_policy proves missing
  authentication, disabled policy, duplicate/existing channel, and
  redaction-sensitive paths fail closed with SSH_MSG_CHANNEL_OPEN_FAILURE.
- message and shape failures:
  session_channel_open_fails_closed_for_message_type_and_shape proves wrong
  message, unsupported channel type, malformed shape, and over-limit shape fail
  closed with SSH_MSG_CHANNEL_OPEN_FAILURE.

## Redaction boundary

Retained evidence uses only task ids, paths, fixed labels, public SSH message
names/numbers, public field-count and public channel-type length categories,
readiness counters, validation commands, test names, and classifications.
It retains no request payload bytes, channel identifiers, window sizes, packet
sizes, user/operator identity, key material, session-id bytes, signatures,
hardware data, or boot artifacts.
