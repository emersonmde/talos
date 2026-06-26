# Phase 12 Local Absolute-Path VFS Command Closeout

Task id: phase12-local-absolute-path-vfs-command-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Close out the accepted direct absolute-path VFS command surface:

~~~text
/bin/status42
~~~

This is static closeout work. It maps the retained core task evidence to the
accepted/deferred frontier and selects the next mechanically objective local
POSIX/VFS follow-up. It does not change source behavior, run Pi 5 hardware,
resume live networking/SSH, retry generated-root command input, or advance a
phase transition.

## Findings

- fixed: Mapped the accepted direct /bin/status42 command behavior to the core
  task record, core classification/evidence map, retained QEMU/substitute
  transcript, roadmap entry, Phase 12 project note, and early POSIX note.
- fixed: Recorded that direct absolute-path command execution uses the
  accepted VFS open/read, program-loader, startup ABI, userspace launch,
  lifecycle/status, process-table, waitpid, laststatus, /proc/talos/processes,
  and zero-argument ps path rather than diagnostic exec-prefix dispatch.
- fixed: Preserved the fail-closed negative controls: /missing is not found,
  /bin and /etc/banner.txt are not executable, and bin/status42 plus status42
  remain unknown-command with no PATH or bare-name lookup.
- fixed: Added closeout classification and evidence-map records that preserve
  the accepted/deferred frontier and select the path-form pipeline follow-up.
- fixed: Updated roadmap and Phase 12 project notes with the closeout boundary
  and dependency-gated path-form pipeline core follow-up.
- not-an-issue: The accepted surface remains direct absolute-path VFS command
  execution only; it does not claim POSIX shell compatibility or PATH lookup.
- deferred: PATH lookup, bare command lookup, direct path arguments,
  redirections, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  waitpid options, broad procfs/Linux ps compatibility, public process
  enumeration ABI, PID policy expansion, persistent storage, live networking,
  SSH, Pi 5 hardware proof, generated-root command-input retry, and phase
  transition.

## Evidence Map

- Core task record:
  `tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md`.
- Core classification:
  `tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/classification.json`.
- Core evidence map:
  `tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/evidence-map.json`.
- QEMU/substitute direct absolute-path transcript:
  `tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/qemu-local-shell-absolute-path-vfs-command-smoke.log`.
- Roadmap entry:
  `docs/src/roadmap.md`.
- Phase 12 project note:
  `docs/src/project/phase12-networking-ssh.md`.
- Early POSIX note:
  `docs/src/project/early-posix-shape.md`.
- Closeout classification:
  `tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-closeout/evidence-map.json`.

## Accepted Frontier

The accepted command frontier is direct absolute-path VFS command execution for
the supported executable fixture paths already covered by the existing VFS exec
path. The retained proof case is /bin/status42.

The retained transcript proves that /bin/status42:

- opens and reads executable bytes through the accepted VFS path;
- reaches the program loader, initial user stack, and userspace launch path;
- records the accepted lifecycle/status and process-table entry with status
  0x2a;
- remains observable through waitpid, laststatus, /proc/talos/processes, and
  zero-argument ps.

Unsupported direct paths fail closed, relative path forms do not trigger PATH
lookup, and bare command names do not trigger bare-name lookup.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain PATH lookup, bare command lookup, direct path
arguments, direct path redirections, arbitrary shell grammar, path-form
pipelines beyond the explicitly selected follow-up, unbounded pipelines,
pipeline-concurrent execution, scheduler concurrency, fork/signals, process
groups/sessions, waitpid options, broad procfs/Linux ps compatibility,
/proc/self, /proc/<pid>, public process enumeration ABI, PID policy expansion,
persistent storage, live networking, SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Selected Follow-Up

selected_next_task is
`phase12-local-absolute-path-vfs-pipeline-core-20260626` because the direct
absolute-path command core was accepted and committed, this closeout reconciles
its evidence, and the queued path-form pipeline core is mechanically objective
only after this accepted closeout.

The follow-up remains local feature work over the existing VFS exec/loading,
pipeline, process-table/status, waitpid, /proc/talos/processes, ps, and
pipestatus regression surfaces. It does not authorize PATH lookup, POSIX shell
compatibility, unbounded pipeline support, live networking, SSH, Pi 5 hardware
proof, or a phase transition.

## Validation

- passed: static inspection of retained core task record, classification,
  evidence map, QEMU/substitute transcript, roadmap, Phase 12 project notes,
  and early POSIX notes.
- passed: `jq empty` on closeout classification/evidence-map JSON.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build`.
- passed: `git diff --cached --check`.

## Result

The direct absolute-path VFS command frontier is closed out as a local
static/unit/QEMU-substitute surface. /bin/status42 is the accepted proof case
for command dispatch through VFS open/read, loader, userspace launch,
lifecycle/status, process-table, waitpid, laststatus, /proc/talos/processes,
and zero-argument ps. The closeout does not authorize fake command expansion,
PATH lookup, POSIX shell compatibility, hardware work, live networking, SSH, or
a phase transition.
