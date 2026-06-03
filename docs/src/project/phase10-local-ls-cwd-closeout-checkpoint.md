# Phase 10 Local Ls Cwd Closeout Checkpoint

Status: accepted

Task: phase10-local-ls-cwd-closeout-checkpoint-20260602

## Scope

This checkpoint closes out the bounded local bare `ls` current-directory
feature as documentation-only work. It reconciles the accepted QEMU/substitute
core, RPi5 candidate archive, serialized Raspberry Pi 5 feature proof,
retained evidence, command-context cwd frontier, deferred filesystem/POSIX and
userspace shell surfaces, and the handoff back to supervisor planning for the
next feature-led local interactivity task.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, power-cycle, or hardwareTestLock
acquisition was performed.

## Reviewed Evidence

- local bare `ls` cwd core implementation, QEMU/substitute evidence, and
  task record commit: 4a3db877499328b10e75bff9f1eb3bc36f7579ae.
- RPi5 bare `ls` cwd candidate archive commit:
  742f0eaba91bd4986d8fa456722de89f94aa7015.
- accepted Pi 5 bare `ls` cwd proof commit:
  b17832a7232d74c3d4f90dde677c4beb86271945.
- retained QEMU/substitute bare `ls` cwd transcript:
  tasks/evidence/2026-06-02-qemu-local-ls-cwd-core/qemu-local-ls-cwd-smoke.log.
- retained RPi5 candidate archive review:
  tasks/evidence/2026-06-02-rpi5-local-ls-cwd-candidate-archive-core/archive-review.txt.
- retained RPi5 candidate static proof-string inspection:
  tasks/evidence/2026-06-02-rpi5-local-ls-cwd-candidate-archive-core/static-proof-strings.txt.
- retained Pi 5 proof summary:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/proof-result.txt.
- retained Pi 5 serial transcript:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/serial-transcript.txt.
- retained Pi 5 settled TFTP proof:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/tftp-delta-settled-before-restore.json.
- retained Pi 5 classification summary:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/classification-summary.json.
- retained Pi 5 restore proof:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/post-restore-status.json.

The retained QEMU/substitute transcript contains `pwd`, bare `ls` at
`/`, `cd /etc`, bare `ls` showing `banner.txt`, `cd /bin`, bare
`ls` showing `init`, `cd /`, bare `ls` showing the root entries,
`bogus` unknown-command regression, next-prompt readiness, final
classification `qemu-local-ls-cwd-complete`, and exact PASS line
`qemu-local-ls-cwd: PASS`.

The accepted RPi5 candidate archive was
`target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz` with archive
sha256 `1f986f73b793b269e5b7aa0cf34cfc4cbf3b58358b0d9b409181e762b986919e`,
kernel sha256 `da6bb65ad8529912e1feca037d6f1e3cfbc46c5ea052ee32a1ab669b000bfd3e`,
and a 110624-byte `kernel_2712.img`. Static inspection retained
`rpi5-local-ls-cwd-proof`, `pi5-local-ls-cwd-complete`,
`ls-cwd-observed`, and `TALOS: command loop proof entered`; quarantined
raw assembly entry markers were absent.

The retained Pi 5 proof held hardwareTestLock, published only the accepted
archive, captured fresh serial and TFTP cursors, collected settled same-cursor
TFTP evidence before restore, and restored the pre-run boot tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
lock release. The TFTP delta includes fresh `da591740/config.txt`,
`da591740/kernel_2712.img`, DTB, overlay, and `cmdline.txt` requests from
`10.42.1.4`; the candidate kernel was served twice at 110624 bytes.

Serial hardware evidence retained the full accepted command sequence: `pwd`
prints `/`; bare `ls` at root prints `bin`, `dir`, `empty`, and
`etc`; `cd /etc` followed by bare `ls` prints `banner.txt`;
`cd /bin` followed by bare `ls` prints `init`; `cd /` followed by
bare `ls` returns to the root entries; `bogus` remains an unknown-command
regression; and the loop returns to a ready prompt with
`pi5-local-ls-cwd-complete` plus exact `rpi5-local-ls-cwd-proof: PASS`.

## Accepted Frontier

The accepted capability is bounded kernel-backed command-context cwd listing
on the descriptor-backed serial command-loop path. Talos can print the serial
prompt, accept `pwd`, fixed-directory `cd`, and bare `ls` through
fd0/runtime-console0 canonical-lite input, resolve bare `ls` against the
accepted cwd values `/`, `/etc`, and `/bin`, print the corresponding
immutable fixture entries through stdout, preserve known unknown-command and
ready-prompt behavior, and return to a ready `talos>` prompt.

This is bounded shell UX and future process-local cwd shaping. It is not an
accepted POSIX `chdir` syscall, process-local cwd inheritance model,
relative-path resolver, broad path traversal mechanism, arbitrary
`ls`/path support, descriptor-backed filesystem syscall, userspace shell, or
filesystem-backed command execution path.

Existing built-ins remain deterministic: `help` reports the accepted command
frontier, `status` reports the kernel-backed built-in frontier, `stdio`
reports fd identity and runtime-console0 backing, `echo hello` and bounded
literal echo print accepted output, exact `ls /` and `ls /bin` remain
accepted, `cat /etc/banner.txt` prints the immutable initramfs banner, empty
input reports `talos: empty-command`, unknown input reports
`talos: unknown-command`, and unexpected arguments to non-argument built-ins
report `talos: unexpected-argument`.

## Deferred Surfaces

Still deferred after this checkpoint:

- broad shell parser/tokenization, quoting, escaping, globbing, environment
  expansion, command substitution, multiline input, and shell variables.
- POSIX `chdir`, process-local cwd inheritance, relative paths, `.` and
  `..` handling, mount namespaces, path normalization, symlink handling, and
  general VFS path traversal.
- descriptor-backed filesystem syscalls, writable filesystem state,
  filesystem-backed command execution, arbitrary file reads, recursive
  listing, and broad `cat` or `ls` behavior.
- argv/envp process ABI, userspace shell execution, process spawning,
  exec/wait/exit, process lifecycle integration, and descriptor inheritance
  across exec.
- terminal sessions, termios, foreground process groups, job control, POSIX
  signal delivery, signal restart behavior, shell history, cursor addressing,
  readline-style editing, broad escape-sequence parsing, pipes, and
  redirection.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- the older blocked `ls /bin` proof strategy and paused Phase 8 proof-only
  work unless a later feature-led task directly needs either one for local
  interactivity.

## Next Planning Handoff

No explicit mechanically unblocked feature task remains after this closeout.
The worker should set planningNeeded=true and hand selection of the next
smallest feature-led local interactivity task back to the supervisor, instead
of inventing a new direction or promoting blocked proof chains.

The next task should stay in the local serial interactivity milestone unless
the supervisor records a deliberate phase or milestone transition. It should
continue to prefer the smallest user-visible behavior over diagnostic-only or
smoke-only work.

## Validation

- static inspection: reconciled retained QEMU/substitute, RPi5 candidate
  archive, and Pi 5 bare `ls` cwd evidence paths from accepted records.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
