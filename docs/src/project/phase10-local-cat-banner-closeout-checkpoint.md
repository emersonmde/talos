# Phase 10 Local Cat Banner Closeout Checkpoint

Status: accepted

Task: phase10-local-cat-banner-closeout-checkpoint-20260602

## Scope

This checkpoint closes out the bounded local `cat /etc/banner.txt` feature
as documentation-only work. It reconciles the accepted QEMU/substitute core,
serialized Raspberry Pi 5 feature proof, retained evidence, descriptor-backed
stdin/stdout frontier, the accepted read-only initramfs banner read behavior,
deferred parser/filesystem/userspace surfaces, and the handoff back to
supervisor planning for the next feature-led local interactivity task.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, power-cycle, or hardwareTestLock
acquisition was performed.

## Reviewed Evidence

- local `cat /etc/banner.txt` core implementation, QEMU/substitute evidence,
  and task record commit: a4980a8cd1c225771f7531718bee8edffa711e03.
- original Pi 5 cat-banner candidate/control blocker commit:
  9301bed6b955c61f5c0bae5ce8b145498538d595.
- settled accepted prompt-control discriminator commit:
  aa2e2b342f73ff1790bd5a20a403d857d10eb24f.
- unchanged Pi 5 cat-banner rerun evidence commit:
  582fc2784dcb5022daaf00720b175791cadbe88a.
- retained QEMU/substitute `cat /etc/banner.txt` transcript:
  tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.
- retained Pi 5 unchanged-rerun proof summary:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/proof-result.txt.
- retained Pi 5 serial transcript:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/serial-transcript.txt.
- retained Pi 5 post-run serial tail:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/serial-peek-post-run-65536.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/tftp-delta-before-restore.json.
- retained Pi 5 restore proof:
  tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/post-restore-status.json.

The retained QEMU/substitute transcript contains input
`cat /etc/banner.txt`, visible `Talos initramfs fixture` output,
descriptor-backed fd0/runtime-console0 input and descriptor-backed stdout
markers, next-prompt readiness, final classification
`qemu-local-cat-banner-complete`, and exact PASS line
`qemu-local-cat-banner: PASS`. The same scenario reruns `ls /bin` and
retains visible `init` output, keeping the previous bounded directory-listing
frontier covered.

The retained Pi 5 unchanged rerun records candidate archive digest
35937283006c1079df2d95836343c4cd81e54655989e238fea70aa746778feb0 and
candidate kernel digest
5300184ebc40ac3b5bb44c9c96828f0d4b1c71b2a8f4431593fff8e5394abce3. The
candidate boot tree was staged with effective `kernel_2712.img`; settled TFTP
evidence from cursor 4043654 includes fresh `da591740/config.txt` requests
and two fresh 107520-byte `da591740/kernel_2712.img` fetches from the Pi 5.

Serial hardware evidence retained a fresh `talos>` prompt, the delayed write
of `cat /etc/banner.txt`, visible `Talos initramfs fixture` output,
`cat-banner-observed`, ready-for-next prompt evidence, final classification
`pi5-local-cat-banner-complete`, and exact PASS line
`rpi5-local-cat-banner-proof: PASS`. Post-run restore returned the boot tree
to hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Descriptor Marker Policy

The original Pi 5 cat-banner proof gate required explicit descriptor-backed
fd0/stdout markers in retained serial evidence. The unchanged
`rpi5_local_cat_banner` scenario did not emit that startup marker, even though
the feature path reached a prompt, accepted the command, printed the immutable
initramfs fixture bytes, returned to a ready prompt, and produced the exact PASS
line.

Supervisor review at 2026-06-02T08:14:00Z accepted this proof on feature-led
evidence. Descriptor-backed command-loop behavior remains covered by the
accepted QEMU/substitute cat-banner core and the previous descriptor-backed
local command-loop proof lineage. Future Pi 5 proof-harness marker work is
optional and must be justified by a real feature or evidence gap; the missing
marker in this unchanged scenario is not a blocker for the accepted bounded
`cat /etc/banner.txt` capability.

## Accepted Frontier

The accepted capability is a bounded kernel-backed `cat /etc/banner.txt`
command on the descriptor-backed serial command-loop path. Talos can print the
serial prompt, accept the exact command through fd0/runtime-console0
canonical-lite input, read the accepted read-only initramfs fixture file, print
`Talos initramfs fixture` through stdout, and return to a ready `talos>`
prompt.

This is deliberately a single fixed file read. It validates the immutable
initramfs fixture bytes and command-loop stdout path, but it does not accept a
general `cat` implementation, arbitrary paths, path traversal, streaming
large files, descriptor-backed filesystem syscalls, writable filesystem state,
or filesystem-backed external command execution.

Existing built-ins remain deterministic: `help` reports the accepted command
frontier, `status` reports the kernel-backed built-in frontier, `stdio`
reports fd identity and runtime-console0 backing, `pwd` prints `/`,
`echo hello` and bounded literal echo print accepted output, `ls /` and
`ls /bin` print the accepted fixture listings, empty input reports
`talos: empty-command`, unknown input reports `talos: unknown-command`, and
unexpected arguments to non-argument built-ins report
`talos: unexpected-argument`.

This frontier is still a kernel-backed local command loop. It is not accepted
userspace shell execution, process spawning, external command lookup, a broad
shell parser, or broad POSIX read/stdio behavior.

## Deferred Surfaces

Still deferred after this checkpoint:

- broad shell parser/tokenization, quoting, escaping, globbing, environment
  expansion, command substitution, multiline input, and shell variables.
- general `cat`, arbitrary file reads, recursive listing, general path
  traversal, VFS lookup beyond the accepted fixture paths, descriptor-backed
  filesystem syscalls, writable filesystem state, and filesystem-backed command
  execution.
- large-file streaming, pager behavior, binary output policy, EOF handling over
  user-visible descriptors, and POSIX read/write/stdio completeness.
- argv/envp process ABI, userspace shell execution, process spawning,
  exec/wait/exit, process lifecycle integration, and descriptor inheritance
  across exec.
- terminal sessions, termios, foreground process groups, job control, POSIX
  signal delivery, signal restart behavior, shell history, cursor addressing,
  readline-style editing, broad escape-sequence parsing, pipes, and
  redirection.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- the blocked `ls /bin` Pi 5 proof strategy and paused Phase 8 proof-only
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

- static inspection: reconciled retained QEMU/substitute and Pi 5
  `cat /etc/banner.txt` evidence paths from the accepted core and hardware
  proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
