# Cursor Caveat Disposition

The previous discriminator's caveat was not that `/tftp/logs` lacked a durable
cursor. The retained evidence includes authoritative `.tftp.cursor_end=4095602`
in `tftp-tail-before.json`, and replay from that cursor recovered the expected
known-good boot-file fetches.

The mistake was allowing a wrapper path to look for a top-level `cursor_end`
and continue with a blank cursor when the deployed API exposed the cursor at
`.tftp.cursor_end`. This task fixes/quarantines that class of mistake:

- `scripts/rpi5-tftp-cursor.sh` now requires a numeric `.tftp.cursor_end` or
  fallback `.cursor_end` and exits non-zero otherwise.
- `scripts/rpi5-wait-tftp-delta.sh` now rejects blank and non-numeric cursors
  before it queries `/tftp/logs`.

The next hardware task must still record the raw cursor source file, direct
stable pre-restore TFTP delta from that cursor, and restore evidence. These
helper checks do not accept known-good runtime readiness; they only prevent
empty-cursor evidence from being recorded as a meaningful direct-cursor sample.
