# Pi 5 Production Timer Preemption Proof Evidence Summary

Status: accepted.

The accepted production timer/preemption candidate archive passed local archive
review, was fetched by the Pi 5 at 104,136 bytes, and reached
classification=pi5-production-timer-preemption-complete plus PASS in
local8-multi-observe-rerun. The lab was restored after every attempt.

Key directories:

- local1-candidate: first candidate run, firmware-only serial.
- local1-control-restored: restored control run; retained serial later showed
  the known-good multi-core preemption control still passes.
- local2-candidate-rerun: rerun after known-good control, firmware-only
  candidate evidence.
- local3-candidate-settled: settled candidate observe, still inconclusive.
- local4-candidate-tail: tail-based settled candidate run, still no candidate
  Talos lines.
- local5-padded-control: size-control run; padded multi-core preemption image
  at 104,136 bytes passed.
- local6-static-comparison-rerun: image comparison showed the production image
  shares the accepted early entry path; one-shot serial observe remained too
  short.
- local7-correct-cursor-rerun: corrected TFTP cursor handling but still used a
  one-shot serial observe, so it remained inconclusive.
- local8-multi-observe-rerun: accepted run; repeated serial observe windows
  captured rpi5-production-timer-preemption reports for logical CPUs 1, 2, and
  3, participants=3 expected=3 errors=0, classification complete, and PASS.

Final restored lab status is recorded in local8-multi-observe-rerun/final-status.json.
