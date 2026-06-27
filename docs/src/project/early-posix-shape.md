# Early POSIX Shape

This is not a full POSIX design. It is an early guardrail so kernel threads, processes, descriptors, and the scheduler do not grow in a direction that makes Unix-like behavior painful later.

## Naming

Talos should distinguish:

- Task: schedulable execution context known to the scheduler.
- Kernel thread: task running only in kernel address space.
- Process: resource-owning container with address space, descriptor table, current working directory, credentials later, and one or more tasks.
- User thread: task executing in a process address space at EL0.

Early milestones may implement only kernel threads, but scheduler structures should not assume that every task owns global resources directly.

## Process Shape

A process should eventually own:

- process ID
- parent process ID
- address space
- descriptor table
- current working directory
- root directory or namespace handle
- exit status
- child state for wait
- signal state later

The scheduler should schedule tasks, not processes. A task should point at its owning process when user processes exist.

## Descriptor Shape

File descriptors should be process-local integer handles. Descriptor operations should be designed around:

- open
- read
- write
- close
- dup
- pipe
- socket later
- inheritance across spawn or exec

The target abstraction is one descriptor table per process, with descriptor entries referencing open file descriptions or kernel objects. Console, pipes, files, devices, and sockets should use the same descriptor-facing operations even if their internals differ.

## Path Shape

Talos should define path behavior before implementing VFS:

- absolute paths start at the process root
- relative paths start at the current working directory
- dot and dot-dot normalization must be explicit
- path lookup should return structured errors, not strings
- current working directory belongs to the process, not the shell

## Error Shape

Internal Rust errors can be rich enums. The syscall boundary should translate them into stable numeric errors. The mapping does not need to be complete initially, but it should reserve a path toward familiar errno-style behavior.

The architecture-quality review for
talos-review-posix-syscall-descriptors-20260603 tightened that rule: the
syscall boundary now maps every current PosixError variant to an explicit
errno number. Future PosixError additions should add syscall encoding tests
with the new variant instead of relying on a fallback such as ENOSYS.

Phase 12.4 now has a private socket ABI contract for the accepted
descriptor-backed AF_INET/SOCK_STREAM surface. It uses the stable Talos syscall
trap shape (svc #0, x8 selector, x0 through x5 scalar arguments, x0 return,
negative x0 errno) and records the current private socket selector vocabulary
before adding no_std/userspace wrappers. That contract is not POSIX/Linux
compatibility and does not accept libc/std sockets, UDP/raw sockets, live packet
I/O, hardware reachability, SSH, or a public stable socket ABI.

## Spawn, Exec, Exit, and Wait

The first implementation can choose a small subset, but it should keep these concepts separate:

- spawn creates a process or task from a kernel-selected program image
- exec replaces the current process image
- exit records status and terminates the process
- wait observes child process completion

A shell-only command dispatcher is acceptable as a diagnostic tool, but it should not become the only way programs run.

Phase 12's local POSIX continuation adds the first bounded process-table
substrate for accepted local VFS exec forms. Direct foreground fixtures
/bin/init, /bin/zero, and /bin/status42 record an internal kernel-owned
process-table lifecycle/status entry with stable pid, shell parent, owner,
path, exited state, status, observed-status, and reaped state. The same
bounded substrate now also records exact two-stage pipeline producer/consumer
entries with distinct stable pids and accepted background /bin/status42 and
/bin/zero job entries while preserving the existing shell-visible waitpid,
laststatus, and jobs accounting surfaces. That is still not a public process
enumeration API, Linux procfs compatibility, fork, process groups, waitpid option model, PID
reuse policy, or scheduler-concurrent process model.

The first process-status VFS view is `/proc/talos/processes`. It is
Talos-private, read-only, versioned as `talos-processes-v1`, descriptor-backed
through the same `cat`/open/read path used by accepted VFS files, and reports
only bounded process-table records for direct VFS exec, exact two-stage
pipelines, and accepted background jobs. It is not Linux procfs compatibility,
`/proc/self`, `/proc/<pid>`, a public process enumeration ABI, or a
scheduler-concurrent process model. The zero-argument `ps` shell view is a
thin presentation of that same Talos-private VFS file; unsupported arguments
or options are outside the accepted boundary.

The 2026-06-26 local POSIX frontier checkpoint keeps that whole chain at
static/unit/QEMU-substitute evidence level: descriptor-backed VFS file I/O,
direct fixture loading, local userspace launch/status, bounded process-table
records, waitpid/laststatus/jobs accounting, `/proc/talos/processes`, and
zero-argument VFS-backed `ps` are accepted; live networking/SSH, Pi 5 hardware
proof, scheduler concurrency, fork/signals, process groups/sessions, broad
procfs, PID policy expansion, persistent storage, arbitrary pipelines,
pipefail, and phase transition remain deferred pending supervisor planning.

The first accepted multistage pipeline remains deliberately narrow:
`exec stdout | exec stdin | exec stdin`. It is shell-visible and VFS-backed,
but still serialized and bounded. The three fixtures exchange bytes through
two pipe descriptor handoffs and install three process-table records that
`waitpid`, `/proc/talos/processes`, and zero-argument `ps` can observe.
The bounded `pipestatus` shell surface now reads those same process-table
records for exact two-stage and accepted three-stage pipelines, preserving
default final-stage status while exposing a labeled `pipefail-status`
observation for a nonzero producer case. This is not POSIX shell
compatibility and does not accept arbitrary pipeline grammar, unbounded
pipeline length, pipeline concurrency, or a shell option framework.

The 2026-06-26 local pipeline frontier checkpoint keeps that pipeline layer at
static/unit/QEMU-substitute evidence level. Accepted pipeline behavior is still
limited to exact two-stage local forms, the accepted
`exec stdout | exec stdin | exec stdin` three-stage form, bounded
process-table records, `/proc/talos/processes`, zero-argument VFS-backed
`ps`, and `pipestatus` with `pipefail-status` labeled
`bounded-observation-not-posix-shell`. Live networking/SSH, Pi 5 hardware
proof, scheduler concurrency, fork/signals, process groups/sessions, broad
procfs/Linux `ps`, persistent storage, arbitrary shell grammar, unbounded
pipelines, POSIX pipefail compatibility, and phase transition remain deferred
pending supervisor planning.

The first direct path-form command is also deliberately narrow. A shell input
of /bin/status42 now reaches the same accepted VFS open/read, program-loader,
initial stack, and userspace launch path previously exercised by
exec /bin/status42. It records the same bounded lifecycle/process-table state
and remains observable through waitpid, laststatus, /proc/talos/processes, and
zero-argument ps. Unsupported direct paths fail closed, while bin/status42
remains unknown-command; this is absolute-path VFS command execution only, not
PATH lookup, POSIX shell compatibility, direct path redirection/argument
support, or broad shell support.

The first accepted path-form pipeline is equally bounded:
\`/bin/stdout | /bin/stdin\`. Both stages are absolute paths and load through the
accepted VFS exec/loading path; the existing serialized two-stage pipe byte
flow, lifecycle/status records, waitpid, laststatus, \`/proc/talos/processes\`,
zero-argument \`ps\`, and \`pipestatus\` observations remain intact. Mixed
diagnostic/path forms, bare command names, unsupported paths, and path-form
multistage pipelines fail closed. PATH lookup, bare-name lookup, arbitrary
shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, live networking/SSH, Pi 5 hardware proof, and phase
transition remain deferred.

The 2026-06-26 local path-command frontier checkpoint keeps direct
absolute-path commands and the bounded path-form pipeline at static/unit/QEMU
substitute evidence level. The accepted boundary is only '/bin/status42' as a
direct VFS command and '/bin/stdout | /bin/stdin' as a two-stage path-form
pipeline, both backed by VFS open/read, loader, userspace launch,
process-table, waitpid, '/proc/talos/processes', zero-argument 'ps', and
'pipestatus' observations. Live networking/SSH, Pi 5 hardware proof, PATH or
bare-name lookup, path-form arguments/redirections, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, persistent storage, generated-root
command-input retry, and phase transition remain deferred pending supervisor
planning.

The first direct bare-name command is narrower than POSIX PATH. A shell input
of status42 is resolved through the fixed bounded /bin lookup to /bin/status42
and then follows the same VFS open/read, loader, userspace launch,
lifecycle/status, waitpid, laststatus, /proc/talos/processes, and
zero-argument ps path accepted for /bin/status42. The first bare-name pipeline
is equally bounded: `stdout | stdin` resolves each stage through the fixed
/bin lookup to /bin/stdout and /bin/stdin before using the accepted VFS
open/read, loader, userspace launch/status, descriptor-backed pipe handoff,
and bounded process-table path. Relative names with slashes, unsupported bare
arguments/redirections, mixed bare/path/exec pipeline forms, unsupported
stage names, and bare-name multistage pipelines fail closed without
successful process records. This does not accept environment-backed PATH
compatibility, command lookup beyond the bounded /bin surface, arbitrary shell
grammar, unbounded pipelines, generated-root command-input retry, live
networking/SSH, Pi 5 hardware proof, or a phase transition.

The 2026-06-26 local bare-name path frontier checkpoint keeps that bare-name
surface at static/unit/QEMU substitute evidence level. The accepted boundary is
only status42 as a direct bare-name command and stdout | stdin as a two-stage
bare-name pipeline, both backed by the fixed /bin VFS lookup, VFS open/read,
loader, userspace launch/status, descriptor-backed pipe handoff, bounded
process-table state, waitpid, /proc/talos/processes, zero-argument ps, and
pipestatus observations. Live networking/SSH, Pi 5 hardware proof,
environment-backed POSIX PATH compatibility, command lookup beyond the bounded
/bin surface, path-form arguments/redirections, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, persistent storage, generated-root
command-input retry, and phase transition remain deferred pending supervisor
planning.

The first direct command argv slice extends only the direct absolute-path
command form. A shell input of '/bin/status42 alpha beta' still opens and
reads '/bin/status42' from VFS, then launches through the accepted loader,
userspace startup/status, and bounded process-table path. The startup ABI
records argc=3, argv0=/bin/status42, argv1=alpha, argv2=beta, an empty envp,
inherited standard descriptors, and a closed loader temporary descriptor.
No-argument direct path commands, bare-name commands, bare-name pipelines,
exec-prefixed literal argv, process-status VFS, zero-argument 'ps', and
'pipestatus' remain regression surfaces. The direct argv closeout reconciles
that surface and selects bounded bare-name argv as the next objective local
step through the accepted fixed /bin lookup policy. Pipeline stage argv,
redirections, environment-backed PATH, arbitrary shell grammar, unbounded
pipelines, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred pending supervisor planning.

The first bare-name command argv slice extends only the accepted direct
bare-name command form. A shell input of 'status42 alpha beta' resolves through
the fixed /bin lookup to /bin/status42, then uses VFS open/read, the accepted
loader, userspace startup/status, and bounded process-table path. The startup
ABI records argc=3, canonical argv0=/bin/status42, argv1=alpha, argv2=beta,
deterministic empty envp, inherited standard descriptors, and a closed loader
temporary descriptor. No-argument bare-name commands, bare-name pipelines,
direct absolute-path argv, exec-prefixed literal argv, process-status VFS,
zero-argument 'ps', and 'pipestatus' remain regression surfaces. Too many
bare-name arguments, unsupported literal characters, and unsupported bare
commands fail closed. Pipeline stage argv, redirections, environment-backed
PATH, current-directory search, arbitrary shell grammar, unbounded pipelines,
live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred pending the command argv frontier checkpoint.

The 2026-06-26 local command argv frontier checkpoint keeps command argv at
static/unit/QEMU substitute evidence level. The accepted boundary is only the
direct command forms '/bin/status42 alpha beta' and 'status42 alpha beta',
with bare names resolving through the fixed /bin VFS lookup and both forms
using VFS open/read, the loader, userspace startup/status, inherited standard
descriptors, a closed loader temporary descriptor, bounded process-table
records, waitpid, /proc/talos/processes, zero-argument ps, and pipestatus.
Pipeline stage argv, redirections, environment-backed PATH, command lookup
beyond the bounded /bin surface, arbitrary shell grammar, unbounded pipelines,
live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred pending supervisor planning.

The first direct path-form pipeline argv slice extends only the accepted
two-stage direct pipeline. A shell input of '/bin/stdout alpha | /bin/stdin
beta' opens and reads both executable files from VFS, launches each stage
through the accepted userspace startup/status path, and preserves the existing
serialized pipe descriptor handoff. The producer startup ABI records argc=2,
argv0=/bin/stdout, argv1=alpha, empty envp, inherited standard descriptors with
fd1 as the pipe endpoint, and a closed loader temporary descriptor. The
consumer records argc=2, argv0=/bin/stdin, argv1=beta, empty envp, inherited
standard descriptors with fd0 as the pipe endpoint, and a closed loader
temporary descriptor. No-argument path and bare-name pipelines, direct and
bare-name command argv, multistage pipeline, process-status VFS,
zero-argument ps, and pipestatus remain regression surfaces. Bare-name
pipeline argv, multistage pipeline argv, redirections, environment-backed
PATH, arbitrary shell grammar, unbounded pipelines, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred
pending the direct pipeline argv closeout.

The direct pipeline stage argv closeout reconciles that accepted surface and
selects bounded bare-name pipeline argv as the next objective local step
through the accepted fixed /bin lookup policy. The accepted frontier remains
only the direct path-form two-stage pipeline '/bin/stdout alpha | /bin/stdin
beta', backed by VFS open/read, loader, userspace startup/status, pipe
descriptors, process-table records, waitpid, /proc/talos/processes,
zero-argument ps, and pipestatus. Bare-name pipeline argv, multistage pipeline
argv, redirections, environment-backed PATH, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred until the selected follow-up task.

The first bare-name pipeline argv slice extends only the accepted two-stage
bare-name pipeline. A shell input of 'stdout alpha | stdin beta' resolves
each stage through the fixed bounded /bin lookup to /bin/stdout and /bin/stdin,
then uses descriptor-backed VFS open/read, the accepted loader, userspace
startup/status, and the existing serialized pipe descriptor handoff. The
producer startup ABI records argc=2, canonical argv0=/bin/stdout, argv1=alpha,
empty envp, inherited standard descriptors with fd1 as the pipe endpoint, and
a closed loader temporary descriptor. The consumer records argc=2, canonical
argv0=/bin/stdin, argv1=beta, empty envp, inherited standard descriptors with
fd0 as the pipe endpoint, and a closed loader temporary descriptor. Direct
path-form pipeline argv, direct/bare-name command argv, no-argument pipelines,
multistage pipeline, process-status VFS, zero-argument ps, and pipestatus
remain regression surfaces. Unsupported bare-name pipeline argument shapes,
unsupported literal characters, and unsupported bare commands fail closed
without successful process records. Multistage pipeline argv, redirections,
environment-backed PATH, command lookup beyond bounded /bin, arbitrary shell
grammar, unbounded pipelines, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred pending the
bare-name pipeline argv closeout.

The bare-name pipeline argv closeout reconciles that accepted surface without
adding runtime behavior. The accepted frontier remains exactly 'stdout alpha |
stdin beta' through fixed bounded /bin lookup, VFS open/read, loader,
userspace startup/status, pipe descriptors, process-table records, waitpid,
/proc/talos/processes, zero-argument ps, and pipestatus. Direct path-form
pipeline stage argv, command argv, no-argument pipelines, multistage pipeline,
process-status VFS, ps, and pipestatus remain retained controls. Multistage
pipeline argv, redirections, environment-backed PATH, command lookup beyond
bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5 hardware
proof, generated-root retry, and phase transition remain deferred until the
selected pipeline stage argv frontier checkpoint.

The pipeline stage argv frontier checkpoint reconciles the accepted direct
path-form and bare-name two-stage pipeline argv surfaces without adding runtime
behavior. The accepted frontier remains exactly '/bin/stdout alpha |
/bin/stdin beta' and 'stdout alpha | stdin beta' through VFS open/read, loader,
userspace startup/status, pipe descriptors, process-table records, waitpid,
/proc/talos/processes, zero-argument ps, and pipestatus; the bare-name form
still resolves only through fixed bounded /bin lookup. Multistage pipeline
argv, redirections, environment-backed PATH, command lookup beyond bounded
/bin, arbitrary shell grammar, unbounded pipelines, pipeline concurrency,
scheduler concurrency, fork/signals, process groups/sessions, persistent
storage, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred pending supervisor planning.

The first direct stdin redirection slice extends only the accepted direct
absolute-path command form. A shell input of `/bin/stdin </etc/banner.txt`
opens and reads `/bin/stdin` from VFS, launches through the accepted
loader/userspace startup/status path, and replaces only child fd0 with the
read-only initramfs regular file `initramfs:/etc/banner.txt`. The redirection
evidence records op=source, source-path=/etc/banner.txt,
source-route=initramfs:/etc/banner.txt, child-only shell restoration, inherited
fd1/fd2, a closed loader temporary descriptor, and a userspace stdin read that
ends with regular-file EOF after the read. waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain regression surfaces. Unsupported direct path redirection
variants fail closed without additional successful process records. Bare-name
stdin redirection, pipeline-stage redirection, output redirection expansion,
append/truncate, writable filesystem behavior, environment-backed PATH,
arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred pending the direct
stdin redirection closeout.

The direct stdin redirection closeout reconciles that boundary against retained
task records, task-owned classification/evidence JSON, the QEMU/substitute
transcript, docs, and regression evidence. The accepted surface remains only
'/bin/stdin </etc/banner.txt': the executable comes through descriptor-backed
VFS open/read and the accepted loader/userspace launch/status path, while fd0
is replaced only for the child by 'initramfs:/etc/banner.txt' and restored for
the shell afterward. The fixed bounded /bin lookup and direct redirection
evidence make bare-name stdin redirection a mechanically objective next task,
but it remains separate implementation work. Pipeline-stage redirection,
output redirection expansion, append/truncate, writable filesystem behavior,
environment-backed PATH, command lookup beyond bounded /bin, arbitrary shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred.

The bare-name stdin redirection slice accepts the corresponding bounded
fixed-/bin form only. A shell input of `stdin </etc/banner.txt` resolves
`stdin` through the accepted fixed `/bin` lookup to `/bin/stdin`, loads the
executable through descriptor-backed VFS open/read, and launches through the
same loader/userspace startup/status path. The child sees fd0 rebound to
`initramfs:/etc/banner.txt`, inherited fd1/fd2 remain stdio output, the loader
temporary descriptor is closed, and the shell restores fd0 afterward. waitpid,
laststatus, `/proc/talos/processes`, zero-argument `ps`, pipestatus, direct
path-form stdin redirection, exec-prefixed stdin redirection, command argv,
pipeline argv, and cat-banner controls remain regression surfaces. Unsupported
bare-name redirection variants such as `stdout </etc/banner.txt`,
`stdin </dev/null`, and `stdin < /etc/banner.txt` fail closed without
additional successful process records. Environment-backed PATH, command lookup
beyond bounded `/bin`, pipeline-stage redirection, output redirection,
append/truncate, writable filesystem behavior, generated-root retry, live
network/SSH, Pi 5 hardware proof, and phase transition remain deferred.

The bare-name stdin redirection closeout reconciles that boundary against
retained task records, task-owned classification/evidence JSON, the
QEMU/substitute transcript, docs, and regression evidence. The accepted
surface remains only 'stdin </etc/banner.txt': the command resolves through
fixed bounded /bin lookup to /bin/stdin, the executable comes through
descriptor-backed VFS open/read and the accepted loader/userspace launch/status
path, and fd0 is replaced only for the child by
initramfs:/etc/banner.txt before being restored for the shell. Direct
path-form stdin redirection, exec-prefixed stdin redirection, command argv,
pipeline argv, process-status VFS, zero-argument ps, pipestatus, and
cat-banner controls remain coherent. Pipeline-stage redirection, output
redirection expansion, append/truncate, writable filesystem behavior,
environment-backed PATH, command lookup beyond bounded /bin, arbitrary shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred pending the stdin redirection frontier
checkpoint.

The stdin redirection frontier checkpoint reconciles the accepted local-only
read-only stdin redirection pair. The accepted forms remain
`/bin/stdin </etc/banner.txt` and `stdin </etc/banner.txt`; both load
`/bin/stdin` through descriptor-backed VFS open/read and the accepted
loader/userspace launch/status path, then replace only child fd0 with
`initramfs:/etc/banner.txt` before shell restoration. The bare-name form
still canonicalizes only through fixed bounded `/bin` lookup. Direct
path-form stdin redirection, bare-name stdin redirection, exec-prefixed stdin
redirection, command argv, pipeline argv, process-status VFS, zero-argument
`ps`, pipestatus, and cat-banner controls remain coherent. Output
regular-file redirection, append/truncate, writable filesystem behavior,
pipeline-stage redirection, combined redirections beyond accepted exact forms,
environment-backed PATH, current-directory search, command lookup beyond
bounded `/bin`, arbitrary shell grammar, unbounded pipelines, pipeline
concurrency, scheduler concurrency, fork/signals, process groups/sessions,
persistent storage, live networking/SSH, Pi 5 hardware proof, generated-root
retry, and phase transition remain deferred pending supervisor planning.

The direct pipeline stdin redirection slice extends read-only stdin
redirection into only the producer stage of the accepted direct path-form
two-stage pipeline. A shell input of
`/bin/stdin </etc/banner.txt | /bin/stdin` loads both stages through
descriptor-backed VFS open/read and the accepted loader/userspace
launch/status path. The producer sees fd0 rebound to
`initramfs:/etc/banner.txt`, keeps fd1 as the pipe endpoint, inherits fd2, and
writes the userspace stdin fixture output into the pipe. The consumer sees fd0
as that pipe endpoint, inherits fd1/fd2, reads to pipe EOF, and exits
successfully. Explicit waitpid for both participants, laststatus,
`/proc/talos/processes`, zero-argument `ps`, and pipestatus remain coherent.
Bare-name pipeline-stage stdin redirection, consumer-stage redirection,
multistage pipeline redirection, output redirection, append/truncate, writable
filesystem behavior, environment-backed PATH, command lookup beyond bounded
surfaces, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred. The accepted
direct path-form closeout selects bare-name pipeline-stage stdin redirection
as the next local POSIX/VFS shell surface through the existing fixed bounded
`/bin` lookup policy.

The bare-name pipeline stdin redirection slice accepts the corresponding
fixed-/bin lookup form: 'stdin </etc/banner.txt | stdin'. Both stages resolve
to /bin/stdin before descriptor-backed VFS open/read and the accepted
loader/userspace launch/status path. The producer sees fd0 rebound to
initramfs:/etc/banner.txt, keeps fd1 as the pipe endpoint, inherits fd2, and
writes the userspace stdin fixture output into the pipe. The consumer sees fd0
as that pipe endpoint, inherits fd1/fd2, reads to pipe EOF, and exits
successfully. Explicit waitpid for both participants, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus remain coherent.
Consumer-stage redirection, redirection on multiple pipeline stages, output
redirection, append/truncate, writable filesystem behavior, environment-backed
PATH, command lookup beyond bounded surfaces, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred. The accepted bare-name pipeline stdin redirection
closeout reconciles the direct path-form and fixed-/bin bare-name
producer-stage pipeline redirection evidence, then selects a local pipeline
stdin redirection frontier checkpoint before any consumer-stage redirection,
output redirection, writable filesystem, PATH, hardware, generated-root, or
live network/SSH work.

The pipeline stdin redirection frontier checkpoint reconciles the accepted
direct path-form and fixed-/bin bare-name producer-stage redirection pair:
'/bin/stdin </etc/banner.txt | /bin/stdin' and
'stdin </etc/banner.txt | stdin'. Both surfaces remain local-only and
static/unit/QEMU-substitute backed. Producer fd0 is sourced from
initramfs:/etc/banner.txt, producer fd1 stays the pipe endpoint, consumer fd0
reads that pipe endpoint to EOF, loader temporary descriptors close, shell
fd0 restoration stays coherent, and waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus remain intact.
Consumer-stage redirection, redirection on multiple pipeline stages,
multistage pipeline redirection, output regular-file redirection,
append/truncate, writable filesystem behavior, combined redirections beyond
accepted exact forms, environment-backed PATH, current-directory search,
command lookup beyond bounded /bin, arbitrary shell grammar, unbounded
pipelines, pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, persistent storage, live networking/SSH, Pi 5 hardware
proof, generated-root retry, and phase transition remain deferred pending
supervisor planning.

The direct consumer-stage pipeline stdin redirection slice accepts exactly
`/bin/stdin | /bin/stdin </etc/banner.txt`. Both stages still load through
descriptor-backed VFS open/read and the accepted loader/userspace
launch/status path. The producer keeps inherited fd0 and fd2 while fd1 is the
pipe endpoint; under QEMU/substitute evidence it records a readiness/no-data
stdin observation when no console byte is available. The consumer starts from
the accepted pipeline fd0 handoff, then replaces only child fd0 with
initramfs:/etc/banner.txt before launch, reads the banner file to EOF, and
restores shell fd0 afterward. Explicit waitpid for both participants,
laststatus, /proc/talos/processes, zero-argument ps, and pipestatus remain
coherent. Bare-name consumer-stage stdin redirection, redirection on multiple
pipeline stages, multistage pipeline redirection, output redirection,
append/truncate, writable filesystem behavior, environment-backed PATH,
command lookup beyond bounded surfaces, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred. The accepted core selects a direct consumer-stage
closeout before any bare-name consumer-stage, output redirection, writable
filesystem, PATH, hardware, generated-root, or live network/SSH work.

The direct consumer-stage pipeline stdin redirection closeout reconciles that
frontier against the retained core task record, classification/evidence JSON,
QEMU/substitute transcript, regression transcripts, and project docs. The
accepted witness remains exactly
`/bin/stdin | /bin/stdin </etc/banner.txt`: the producer keeps fd1 as the pipe
endpoint while the consumer replaces only child fd0 with
initramfs:/etc/banner.txt, both stages load /bin/stdin through
descriptor-backed VFS, loader temporary descriptors close, shell fd0
restoration remains coherent, and waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus remain intact.
Bare-name consumer-stage stdin redirection, redirection on multiple pipeline
stages, multistage pipeline redirection, combined input/output redirection,
output regular-file redirection, append/truncate, writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.
The fixed bounded /bin lookup and retained bare-name pipeline evidence make
bare-name consumer-stage stdin redirection the mechanically objective next
task.

The bare-name consumer-stage pipeline stdin redirection slice accepts exactly
`stdin | stdin </etc/banner.txt`. Both stage names canonicalize through fixed
bounded /bin lookup to /bin/stdin, then load through descriptor-backed VFS
open/read and the accepted loader/userspace launch/status path. The producer
keeps inherited fd0/fd2 and fd1 as the pipe endpoint; under QEMU/substitute
evidence it records readiness/no-data when no console byte is available. The
consumer replaces only child fd0 with initramfs:/etc/banner.txt, reads the
banner file to EOF, and restores shell fd0 afterward. Direct consumer-stage,
direct/bare producer-stage, command/stdin/process/ps/pipestatus/cat-banner
regression surfaces remain coherent. Redirection on multiple pipeline stages,
multistage pipeline redirection, output redirection, writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.
The accepted core selects a bare-name consumer-stage closeout before any
output redirection, writable filesystem, PATH, hardware, generated-root, or
live network/SSH work.

The bare-name consumer-stage pipeline stdin redirection closeout reconciles
that accepted surface against retained task records, task-owned JSON,
QEMU/substitute evidence, regression transcripts, and project docs. The
accepted witness remains exactly 'stdin | stdin </etc/banner.txt': both stages
resolve only through the fixed bounded /bin lookup, the producer keeps fd1 as
the pipe endpoint, the consumer replaces only child fd0 with
initramfs:/etc/banner.txt, loader temporary descriptors close, shell fd0 is
restored, and waitpid, laststatus, /proc/talos/processes, zero-argument ps,
and pipestatus remain coherent. Redirection on multiple pipeline stages,
multistage pipeline redirection, output regular-file redirection,
append/truncate, writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, persistent storage, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred. The closeout selects a consumer-stage stdin
redirection frontier checkpoint before any output redirection, writable
filesystem, PATH, hardware, generated-root, or live network/SSH work.

The consumer-stage pipeline stdin redirection frontier checkpoint reconciles
the accepted direct path-form and fixed-/bin bare-name consumer-stage
redirection pair:
`/bin/stdin | /bin/stdin </etc/banner.txt` and
`stdin | stdin </etc/banner.txt`. Both surfaces remain local-only and
static/unit/QEMU-substitute backed. The producer keeps inherited fd0/fd2 and
fd1 as the pipe endpoint. The consumer replaces only child fd0 with
initramfs:/etc/banner.txt, inherits fd1/fd2, closes loader temporary
descriptors, reads the redirected file to EOF, and exits successfully.
Pipeline lifecycle/status, explicit waitpid for both participants,
laststatus, /proc/talos/processes, zero-argument ps, and pipestatus remain
coherent. Redirection on multiple pipeline stages, multistage pipeline
redirection, output regular-file redirection, append/truncate, writable
filesystem behavior, combined redirections beyond accepted exact forms,
environment-backed PATH, current-directory search, command lookup beyond
bounded /bin, arbitrary shell grammar, unbounded pipelines, pipeline
concurrency, scheduler concurrency, fork/signals, process groups/sessions,
persistent storage, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred pending supervisor
planning.

The dual-stage pipeline stdin redirection core accepts the smallest
two-stage form where both participants independently receive read-only
stdin redirection from initramfs:
\`/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt\` and
\`stdin </etc/banner.txt | stdin </etc/banner.txt\`. Both stages still load
through descriptor-backed VFS open/read and the accepted loader/userspace
launch/status path. Each child fd0 is replaced with
initramfs:/etc/banner.txt; the producer keeps fd1 as the pipe endpoint and
writes the redirected banner bytes to the pipe surface, while the consumer
reads its own regular-file fd0 to EOF. QEMU/substitute evidence records
\`source=shell-pipe-dual-stdin-redirection-from-file\`, \`bytes-written=0x3d\`,
\`bytes-read=0x0\`, \`reader-eof=false\`, restored shell descriptors, closed
loader temporary descriptors, and coherent waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus observations.
Consumer-stage-only and producer-stage-only stdin redirection, command argv,
pipeline argv, process-status VFS, ps, pipestatus, and cat-banner surfaces
remain retained regressions. Mixed direct/bare dual-stage forms fail closed
without additional successful process records. Separated \`<\` syntax,
unsupported paths, output redirection, append/truncate, multistage
redirection, arbitrary stage redirection, writable filesystem behavior,
environment-backed PATH, current-directory search, broader shell grammar,
unbounded or concurrent pipelines, scheduler concurrency, fork/signals,
process groups/sessions, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.

The dual-stage pipeline stdin redirection closeout reconciles that accepted
frontier against retained task records, classification/evidence JSON,
QEMU/substitute transcripts, regression transcripts, and project docs. The
accepted witnesses remain exactly
`/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt` and
`stdin </etc/banner.txt | stdin </etc/banner.txt`. Both forms keep
independent initramfs:/etc/banner.txt fd0 replacement for each child, producer
fd1 pipe-endpoint coherence, closed loader temporary descriptors, restored
shell descriptors, waitpid/laststatus/process-table/procfs/ps/pipestatus
observability, and fail-closed mixed direct/bare variants. Live networking/SSH,
Pi 5 hardware proof, generated-root retry, writable filesystem behavior,
output redirection, append/truncate, multistage or concurrent pipelines,
arbitrary shell grammar, and phase transition remain deferred. The closeout
selects the dual-stage pipeline stdin redirection frontier checkpoint before
any output redirection, writable filesystem, multistage pipeline, hardware,
network, or phase-transition work.

The dual-stage pipeline stdin redirection frontier checkpoint freezes that
local-only static/unit/QEMU-substitute boundary. The accepted frontier remains
exactly `/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt` and
`stdin </etc/banner.txt | stdin </etc/banner.txt`, with independent
initramfs:/etc/banner.txt fd0 replacement for both children, coherent producer
fd1 pipe setup, restored shell descriptors, closed loader temporary
descriptors, and retained waitpid/laststatus/process-table/procfs/ps/pipestatus
observability. Multistage pipeline redirection, output regular-file
redirection, append/truncate, writable filesystem behavior, combined
redirections beyond the accepted exact forms, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, unbounded or concurrent pipelines, scheduler concurrency,
fork/signals, process groups/sessions, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root retry, and phase transition remain
deferred. No later queued same-lane local POSIX task is mechanically objective,
so supervisor planning is required before further worker promotion.

The direct stdout regular-file redirection core accepts the first narrow
output file surface after that checkpoint. The accepted witness is exactly
'/bin/stdout >/tmp/stdout.txt': the executable loads through descriptor-backed
VFS open/read and the accepted loader/userspace launch/status path; child fd1
is rebound only for that process to a volatile regular-file descriptor for
'/tmp/stdout.txt'; and 'cat /tmp/stdout.txt' reads back the userspace stdout
fixture bytes through descriptor-backed VFS readback. A subsequent
'/bin/stdout' normal-output command proves shell fd1 restoration, while
waitpid, laststatus, process-table/procfs/ps, and pipestatus-compatible
observations remain coherent. Bare-name output redirection, stderr file
redirection, append/truncate, unsupported output paths, pipeline output
redirection, combined input/output redirection, kernel-backed command
redirection, persistent writable filesystem behavior, PATH/current-directory
search, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred.

The direct stdout regular-file redirection closeout freezes that accepted
local-only boundary. The retained evidence remains the exact
'/bin/stdout >/tmp/stdout.txt' witness, descriptor-backed
'cat /tmp/stdout.txt' readback, and a later normal '/bin/stdout' restoration
control. The next mechanically objective same-lane task is the already queued
bare-name stdout regular-file redirection core; stderr redirection, append,
arbitrary output paths, pipeline-output redirection, combined input/output
redirection, persistent writable filesystem behavior, live networking/SSH,
Pi 5 hardware proof, generated-root retry, and phase transition remain
deferred.

The bare-name stdout regular-file redirection core extends only that accepted
surface through the fixed bounded /bin lookup policy. The accepted witness is
exactly 'stdout >/tmp/stdout.txt': the command name resolves to '/bin/stdout',
the child fd1 is rebound to 'volatile-vfs:/tmp/stdout.txt', and
'cat /tmp/stdout.txt' reads the userspace stdout fixture back through
descriptor-backed VFS. A later normal 'stdout' command proves shell fd1
restoration. Environment-backed PATH lookup, current-directory search, command
lookup beyond bounded /bin, stderr redirection, append/truncate, arbitrary
output paths, pipeline output redirection, combined input/output redirection,
writable persistent filesystem behavior, live networking/SSH, Pi 5 hardware
proof, generated-root retry, and phase transition remain deferred.

The bare-name stdout regular-file redirection closeout freezes that accepted
local-only boundary. The retained evidence remains the exact
'stdout >/tmp/stdout.txt' witness, bounded /bin lookup to '/bin/stdout',
descriptor-backed 'cat /tmp/stdout.txt' readback, and a later normal 'stdout'
restoration control. The next mechanically objective same-lane task is the
already queued stdout regular-file redirection frontier checkpoint; stderr
redirection, append, arbitrary output paths, pipeline-output redirection,
combined input/output redirection, persistent writable filesystem behavior,
live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred.

The stdout regular-file redirection frontier checkpoint reconciles the accepted
direct path-form and fixed-/bin bare-name output redirection pair. The accepted
witnesses remain exactly '/bin/stdout >/tmp/stdout.txt' and
'stdout >/tmp/stdout.txt', with child-only fd1 rebinding to
volatile-vfs:/tmp/stdout.txt, userspace TalosWrite provenance,
descriptor-backed 'cat /tmp/stdout.txt' readback, restored shell stdout, closed
loader temporary descriptors, and retained waitpid/laststatus/process-table
observability. Stderr redirection, append/truncate, arbitrary output paths,
pipeline-output redirection, combined input/output redirection, persistent
writable filesystem behavior, environment-backed PATH, current-directory
search, command lookup beyond bounded /bin, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition
remain deferred. No later queued same-lane local POSIX/VFS task is
mechanically objective, so supervisor planning is required before further
worker promotion.

The direct stderr regular-file redirection core extends the same local-only
output-file model to fd2. The accepted witness is exactly
'/bin/stderr 2>/tmp/stderr.txt': the executable loads through
descriptor-backed VFS open/read and the accepted loader/userspace
launch/status path; child fd2 is rebound only for that process to
volatile-vfs:/tmp/stderr.txt; and 'cat /tmp/stderr.txt' reads back the
userspace stderr fixture bytes through descriptor-backed VFS readback. A
subsequent '/bin/stderr' normal-output command proves shell fd2 restoration,
while fd0/fd1 inheritance, loader temporary descriptor closure,
waitpid/laststatus/process-table observations, and retained stdout
regular-file redirection surfaces remain coherent. Bare-name stderr output
redirection, append/truncate, arbitrary output paths, pipeline output
redirection, combined input/output redirection, kernel-backed command
redirection, persistent writable filesystem behavior, PATH/current-directory
search, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred.

The direct stderr regular-file redirection closeout reconciles that accepted
boundary against the retained core task, classification, evidence map,
QEMU/substitute transcript, regression summary, and project docs. The accepted
surface remains exactly '/bin/stderr 2>/tmp/stderr.txt',
'cat /tmp/stderr.txt' descriptor-backed readback, and later normal
'/bin/stderr' shell fd2 restoration. The fixed bounded /bin lookup and direct
stderr descriptor evidence make bare-name stderr regular-file redirection a
mechanically objective next task, but it remains separate implementation work.
Append/truncate, arbitrary output paths, pipeline-output redirection, combined
input/output redirection, persistent writable filesystem behavior,
environment-backed PATH, current-directory search, command lookup beyond
bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5 hardware
proof, generated-root retry, and phase transition remain deferred.

The bare-name stderr regular-file redirection core accepts exactly
'stderr 2>/tmp/stderr.txt'. The command name resolves through the accepted
fixed /bin lookup to '/bin/stderr'; the child fd2 is rebound only for that
process to volatile-vfs:/tmp/stderr.txt; and 'cat /tmp/stderr.txt' reads the
userspace stderr fixture bytes back through descriptor-backed VFS. A later
normal 'stderr' command proves shell fd2 restoration to runtime-console0/stderr,
while fd0/fd1 inheritance, loader temporary descriptor closure,
waitpid/laststatus/process-table observations, direct stderr, stdout
regular-file redirection, stdin redirection, pipeline stdin redirection,
command argv, pipeline argv, process-status VFS/ps, and pipestatus regressions
remain coherent. Append/truncate, arbitrary output paths, pipeline-output
redirection, combined input/output redirection, persistent writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.

The bare-name stderr regular-file redirection closeout reconciles that accepted
boundary against the retained core task, classification, evidence map,
QEMU/substitute transcript, regression summary, and project docs. The accepted
surface remains exactly 'stderr 2>/tmp/stderr.txt',
'cat /tmp/stderr.txt' descriptor-backed readback, and later normal 'stderr'
shell fd2 restoration. Direct and bare-name stderr regular-file evidence make
the stderr regular-file redirection frontier checkpoint mechanically objective,
but append/truncate, arbitrary paths, pipeline-output redirection, persistent
writable filesystem behavior, PATH/current-directory search, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition
remain deferred.

The stderr regular-file redirection frontier checkpoint reconciles the accepted
direct path-form and fixed-/bin bare-name fd2 output redirection pair. The
accepted witnesses remain exactly '/bin/stderr 2>/tmp/stderr.txt' and
'stderr 2>/tmp/stderr.txt', with child-only fd2 rebinding to
volatile-vfs:/tmp/stderr.txt, descriptor-backed 'cat /tmp/stderr.txt'
readback, normal stderr restoration, closed loader temporary descriptors, and
coherent waitpid/laststatus/process observations. Append/truncate, arbitrary
output paths, pipeline-output redirection, combined input/output redirection,
persistent writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred. Supervisor planning later selected the direct
stdout append regular-file redirection core as the next same-lane local
POSIX/VFS task.

The direct stdout append regular-file redirection core accepts exactly
'/bin/stdout >/tmp/stdout.txt' followed by
'/bin/stdout >>/tmp/stdout.txt'. Each launch still uses descriptor-backed VFS
open/read and the accepted userspace launch/status path. The first command
reuses the accepted child-only fd1 truncate/sink redirection to
volatile-vfs:/tmp/stdout.txt; the second command records op=append and writes at
regular-file EOF; 'cat /tmp/stdout.txt' reads both stdout fixture writes in
order with bytes=0x3e; and a later normal '/bin/stdout' proves shell fd1
restoration. Unsupported append forms such as
'/bin/stdout >>/var/other.txt' and '/bin/stderr 2>>/tmp/stderr.txt' fail
closed. Bare-name stdout append, stderr append, arbitrary paths,
pipeline-output append, combined input/output redirection, persistent writable
filesystem behavior, live networking/SSH, Pi 5 hardware proof, generated-root
retry, and phase transition remain deferred.

The direct stdout append regular-file redirection closeout freezes that
accepted local-only boundary. The retained surface remains exactly
'/bin/stdout >/tmp/stdout.txt' followed by
'/bin/stdout >>/tmp/stdout.txt', with the first child fd1 write using
truncate/sink semantics, the second using append-at-EOF semantics, and
descriptor-backed 'cat /tmp/stdout.txt' reading both stdout fixtures in order
from volatile-vfs:/tmp/stdout.txt. The fixed bounded /bin lookup and direct
append evidence make bare-name stdout append regular-file redirection a
mechanically objective next task, but it remains separate implementation work.
Stderr append, arbitrary output paths, pipeline-output append, combined
input/output redirection, persistent writable filesystem behavior,
PATH/current-directory search, command lookup beyond bounded /bin, arbitrary
shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry,
and phase transition remain deferred.

The bare-name stdout append regular-file redirection core accepts exactly
'stdout >/tmp/stdout.txt' followed by 'stdout >>/tmp/stdout.txt'. Each command
name resolves only through the fixed bounded /bin lookup to '/bin/stdout';
the child fd1 is rebound only for each process to
volatile-vfs:/tmp/stdout.txt; the first command uses truncate/sink semantics;
the second records op=append at regular-file EOF; and descriptor-backed
'cat /tmp/stdout.txt' reads both stdout fixture writes in order with
bytes=0x3e. A later normal 'stdout' proves shell fd1 restoration. Direct
path-form stdout append remains accepted, while unsupported bare-name append
paths, stderr append, pipeline-output append, combined input/output
redirection, persistent writable filesystem behavior, PATH/current-directory
search, command lookup beyond bounded /bin, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred.

The bare-name stdout append regular-file redirection closeout freezes the
accepted local-only direct and fixed-/bin bare-name stdout append boundary. The
retained surface remains exactly '/bin/stdout >/tmp/stdout.txt' then
'/bin/stdout >>/tmp/stdout.txt' and 'stdout >/tmp/stdout.txt' then
'stdout >>/tmp/stdout.txt', with child-only fd1 rebinding to
volatile-vfs:/tmp/stdout.txt, truncate/sink then append-at-EOF operations, and
descriptor-backed 'cat /tmp/stdout.txt' readback of both stdout fixture writes
in order. Unsupported append forms remain fail-closed. The direct and
bare-name append evidence make the stdout append regular-file redirection
frontier checkpoint mechanically objective, but stderr append, arbitrary paths,
pipeline-output append, combined input/output redirection, persistent writable
filesystem behavior, PATH/current-directory search, command lookup beyond
bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5 hardware
proof, generated-root retry, and phase transition remain deferred.

The stdout append regular-file redirection frontier checkpoint freezes the
accepted local-only append boundary after the direct path-form and fixed-/bin
bare-name closeouts. The accepted witnesses remain exactly the direct sequence
'/bin/stdout >/tmp/stdout.txt' then '/bin/stdout >>/tmp/stdout.txt' and the
bare-name sequence 'stdout >/tmp/stdout.txt' then 'stdout >>/tmp/stdout.txt'.
Both forms keep child-only fd1 rebinding to volatile-vfs:/tmp/stdout.txt,
truncate/sink then append-at-EOF operations, descriptor-backed
'cat /tmp/stdout.txt' readback of both stdout fixture writes in order, closed
loader temporary descriptors, and coherent waitpid/laststatus/process
observations. Stderr append, arbitrary output paths, pipeline-output
redirection, combined input/output redirection, persistent writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.
No later queued same-lane local POSIX/VFS task is mechanically objective;
supervisor planning is required before the next worker promotion.

The direct stderr append regular-file redirection core accepts exactly
'/bin/stderr 2>/tmp/stderr.txt' followed by
'/bin/stderr 2>>/tmp/stderr.txt'. Both commands load '/bin/stderr' through the
descriptor-backed VFS open/read and userspace launch path; child fd2 is rebound
to volatile-vfs:/tmp/stderr.txt only for each launched process; the first write
uses truncate/sink semantics; the second write records op=append at regular-file
EOF; and descriptor-backed 'cat /tmp/stderr.txt' reads both stderr fixture
writes in order. A later normal '/bin/stderr' proves shell fd2 restoration.
Direct stdout append remains accepted, while arbitrary stderr append paths,
bare-name stderr append, pipeline-output append, combined input/output
redirection, persistent writable filesystem behavior, PATH/current-directory
search, command lookup beyond bounded /bin, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition
remain deferred.

The direct stderr append regular-file redirection closeout freezes that
accepted local-only direct stderr append boundary. The accepted sequence remains
'/bin/stderr 2>/tmp/stderr.txt' then '/bin/stderr 2>>/tmp/stderr.txt'; both
commands keep child-only fd2 rebinding to volatile-vfs:/tmp/stderr.txt,
truncate/sink then append-at-EOF operations, descriptor-backed
'cat /tmp/stderr.txt' readback of both stderr fixture writes, and later normal
'/bin/stderr' restoration. Unsupported direct and bare-name append forms remain
fail-closed. The direct append evidence makes bare-name stderr append
regular-file redirection mechanically objective, but arbitrary paths,
pipeline-output
append, combined input/output redirection, persistent writable filesystem
behavior, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred.

The bare-name stderr append regular-file redirection core accepts exactly
'stderr 2>/tmp/stderr.txt' followed by 'stderr 2>>/tmp/stderr.txt'. Each command
name resolves only through the fixed bounded /bin lookup to '/bin/stderr'; the
child fd2 is rebound only for each process to
volatile-vfs:/tmp/stderr.txt; the first command uses truncate/sink semantics;
the second records op=append at regular-file EOF; and descriptor-backed
'cat /tmp/stderr.txt' reads both stderr fixture writes in order with
bytes=0x3e. A later normal 'stderr' proves shell fd2 restoration to
runtime-console0/stderr. Direct path-form stderr append remains accepted, while
unsupported bare-name append paths, arbitrary output paths, pipeline-output
append, combined input/output redirection, persistent writable filesystem
behavior, PATH/current-directory search, command lookup beyond bounded /bin,
arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred.

The bare-name stderr append regular-file redirection closeout freezes that
accepted fixed-/bin bare-name stderr append boundary. The accepted sequence
remains 'stderr 2>/tmp/stderr.txt' then 'stderr 2>>/tmp/stderr.txt'; both
commands canonicalize only through bounded /bin to '/bin/stderr', keep
child-only fd2 rebinding to volatile-vfs:/tmp/stderr.txt, use truncate/sink
then append-at-EOF operations, descriptor-backed 'cat /tmp/stderr.txt' readback
of both stderr fixture writes, and later normal 'stderr' restoration.
Unsupported bare-name append forms, arbitrary output paths, pipeline-output
append, combined input/output redirection, persistent writable filesystem
behavior, PATH/current-directory search, command lookup beyond bounded /bin,
arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred. The next
mechanically objective local POSIX/VFS task is the queued stderr append
frontier checkpoint.

The stderr append regular-file redirection frontier checkpoint freezes the
accepted local-only append boundary after the direct path-form and fixed-/bin
bare-name closeouts. The accepted witnesses remain exactly the direct sequence
'/bin/stderr 2>/tmp/stderr.txt' then '/bin/stderr 2>>/tmp/stderr.txt' and the
bare-name sequence 'stderr 2>/tmp/stderr.txt' then
'stderr 2>>/tmp/stderr.txt'. Both forms keep child-only fd2 rebinding to
volatile-vfs:/tmp/stderr.txt, truncate/sink then append-at-EOF operations,
descriptor-backed 'cat /tmp/stderr.txt' readback of both stderr fixture writes
in order, closed loader temporary descriptors, and coherent
waitpid/laststatus/process observations. Arbitrary output paths,
pipeline-output redirection and append, combined input/output redirection,
persistent writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
phase transition remain deferred. No later queued same-lane local POSIX/VFS
task is mechanically objective; supervisor planning is required before the
next worker promotion.

The direct combined stdin/stdout regular-file redirection core accepts exactly
'/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt'. The command still loads
'/bin/stdin' through descriptor-backed VFS. For the launched child only, fd0 is
sourced from initramfs:/etc/banner.txt and fd1 is rebound to
volatile-vfs:/tmp/stdin-report.txt while fd2 remains stdio output. The
userspace stdin fixture reads the banner and writes its report through
redirected fd1; descriptor-backed 'cat /tmp/stdin-report.txt' reads the report
back; and a later normal '/bin/stdin' unit control proves shell fd0/fd1
restoration and closed loader temporary descriptors. Unsupported direct
combined forms fail closed for output-first ordering, spaced input grammar,
/dev/null input, explicit 1> output, append output, stderr output, and
arbitrary output path forms. Bare-name combined redirection, arbitrary
input/output paths, append/stderr combined variants, pipeline-output
redirection, persistent writable filesystem behavior, PATH/current-directory
search, command lookup beyond bounded /bin, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition
remain deferred.

The direct combined stdin/stdout closeout freezes that direct path-form
frontier without adding runtime behavior. The accepted surface remains exactly
'/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt', with fd0 sourced from
initramfs:/etc/banner.txt, fd1 targeting volatile-vfs:/tmp/stdin-report.txt,
descriptor-backed readback through 'cat /tmp/stdin-report.txt', and
fail-closed unsupported direct forms. The next local POSIX/VFS task is the
separate fixed-/bin bare-name combined core; live networking/SSH, Pi 5 hardware
proof, generated-root retry, arbitrary paths, append/stderr variants, and phase
transition remain paused/deferred.

The bare-name combined stdin/stdout regular-file redirection core accepts
exactly 'stdin </etc/banner.txt >/tmp/stdin-report.txt'. The command resolves
only through bounded fixed /bin lookup to '/bin/stdin', then uses the same
descriptor-backed launch path as the direct witness. For the launched child
only, fd0 is sourced from initramfs:/etc/banner.txt, fd1 targets
volatile-vfs:/tmp/stdin-report.txt, and fd2 remains stdio output. The userspace
stdin fixture writes its report through redirected fd1, descriptor-backed
'cat /tmp/stdin-report.txt' reads it back, and retained direct/bare stdin,
stdout regular-file, append, process-status, ps, pipestatus, and cat-banner
controls remain passing. Unsupported bare-name combined forms fail closed for
output-first ordering, spaced input grammar, /dev/null input, explicit 1>
output, append output, stderr output, unsupported command names, and arbitrary
output paths. Arbitrary paths, append/stderr combined variants,
pipeline-output redirection, persistent writable filesystem behavior,
PATH/current-directory search, command lookup beyond bounded /bin, arbitrary
shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry,
and phase transition remain deferred.

The bare-name combined stdin/stdout closeout freezes that fixed-/bin boundary
without runtime feature changes. The accepted command remains exactly
'stdin </etc/banner.txt >/tmp/stdin-report.txt', resolved only to '/bin/stdin',
with child fd0 from initramfs:/etc/banner.txt, child fd1 to
volatile-vfs:/tmp/stdin-report.txt, fd2 on stdio output, descriptor-backed
'cat /tmp/stdin-report.txt' readback, and retained fail-closed unsupported
bare-name combined forms. Direct combined evidence remains the comparison
baseline. The next local POSIX/VFS task is the combined stdin/stdout frontier
checkpoint; arbitrary paths, append/stderr combined variants, pipeline-output
redirection, persistence, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred.

The combined stdin/stdout regular-file redirection frontier checkpoint freezes
the accepted local-only static/unit/QEMU-substitute boundary after the direct
path-form and fixed-/bin bare-name closeouts. The accepted witnesses remain
exactly '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt' and
'stdin </etc/banner.txt >/tmp/stdin-report.txt'. Both forms keep child-only fd0
from initramfs:/etc/banner.txt, child-only fd1 to
volatile-vfs:/tmp/stdin-report.txt, fd2 on stdio output, descriptor-backed
'cat /tmp/stdin-report.txt' readback of the userspace stdin report, closed
loader temporary descriptors, coherent waitpid/laststatus observations, and
shell descriptor restoration. Arbitrary input/output paths, append in combined
forms, stderr combined forms, pipeline-output redirection and append,
persistent writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, unbounded/concurrent pipelines, scheduler concurrency, fork/signals,
process groups/sessions, live networking/SSH, Pi 5 hardware proof,
generated-root retry, and phase transition remain deferred. The next
same-lane local POSIX/VFS task is the direct path-form pipeline-output
regular-file redirection core.

The direct pipeline-output regular-file redirection core accepts exactly
`/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt`. Both stages remain
absolute paths loaded through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status path, and the bounded foreground process table.
The producer writes only to the accepted pipe endpoint; the consumer reads from
that pipe through fd0, writes the userspace stdin report through child-only fd1
to `volatile-vfs:/tmp/pipeline-report.txt`, restores shell descriptors, and is
read back with `cat /tmp/pipeline-report.txt`. `waitpid`, `laststatus`,
`/proc/talos/processes`, zero-argument `ps`, and `pipestatus` remain coherent.
Bare-name pipeline-output redirection, append pipeline-output forms,
input/stderr/combined pipeline redirections, arbitrary output paths,
persistent writable filesystem behavior, generated-root retry, live
networking/SSH, Pi 5 hardware proof, and phase transition remain deferred.

The direct pipeline-output regular-file redirection closeout freezes that
accepted absolute path-form boundary without runtime expansion. The accepted
witness remains exactly `/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt`:
the producer fd1 is the pipe endpoint, the consumer fd0 is the same pipe
endpoint, the consumer fd1 is child-only redirected to
`volatile-vfs:/tmp/pipeline-report.txt`, and descriptor-backed
`cat /tmp/pipeline-report.txt` reads back the userspace stdin report.
Alternate output targets, append syntax, wrong final-stage programs, fixed-/bin
bare-name forms, input/stderr/combined pipeline redirections, arbitrary paths,
persistent storage, live networking/SSH, Pi 5 hardware proof, generated-root
retry, and phase transition remain deferred. The next local POSIX/VFS task is
the queued fixed-/bin bare-name pipeline-output redirection core.

The bare-name pipeline-output regular-file redirection core accepts exactly
'stdout | stdin >/tmp/pipeline-report.txt'. Both stages resolve only through
the bounded fixed-/bin lookup to '/bin/stdout' and '/bin/stdin', then load
through descriptor-backed VFS open/read and the accepted userspace
launch/status path before the pipe handoff. Only the final stage receives
child-only fd1 redirection to 'volatile-vfs:/tmp/pipeline-report.txt';
descriptor-backed 'cat /tmp/pipeline-report.txt' reads back the userspace stdin
report, and the shell fd1 is restored afterward. Direct path-form
pipeline-output redirection and process-status controls remain passing.
Alternate output targets, append syntax, wrong final-stage programs, explicit
'1>', spaced output grammar, and consumer names containing path separators fail
closed. Append pipeline-output forms, stderr forms, input/combined pipeline
redirections, arbitrary paths, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root retry, and phase transition remain deferred.

The bare-name pipeline-output regular-file redirection closeout freezes the
accepted local-only direct path-form and fixed-/bin bare-name pipeline-output
boundary. The accepted witnesses remain exactly
'/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' and
'stdout | stdin >/tmp/pipeline-report.txt'. Both forms keep producer fd1 as the
pipe endpoint, consumer fd0 as that pipe endpoint, and consumer fd1 child-only
redirected to 'volatile-vfs:/tmp/pipeline-report.txt'; descriptor-backed
'cat /tmp/pipeline-report.txt' reads back the userspace stdin report, and the
shell fd1 is restored afterward. Unsupported direct and bare-name variants
remain fail-closed. Append pipeline-output forms, stderr forms,
input/combined pipeline redirections, arbitrary paths, persistent storage,
live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
transition remain deferred. The next local POSIX/VFS task is the
pipeline-output regular-file redirection frontier checkpoint.

## Scheduler Implications

Before implementing scheduler structs, check that:

- task lifetime can detach from process lifetime later
- per-task kernel stack and register state are separate from process resources
- blocking I/O can sleep a task without blocking the whole process model
- wakeups can target tasks
- descriptor and address-space pointers can be added without redesign

This note is now expanded by
[Phase 7 POSIX Contract Baseline](phase7-posix-contract-baseline.md), which
defines the accepted Phase 7.1 errno, path, process lifetime, descriptor,
stdio inheritance, and early loader vocabulary before VFS, syscalls, and user
processes are implemented. The descriptor portion is narrowed by the accepted
[Phase 7 Descriptor Table Contract](phase7-descriptor-table-contract.md),
which fixes process-local descriptor table entries, dup/close behavior,
reserved stdio handles, and deterministic descriptor error cases before the
target-independent table core is implemented. The accepted Phase 7.1
descriptor-table core now implements those target-independent table semantics
without runtime console/TTY I/O, syscall ABI, EL0, VFS, filesystem, pipes,
sockets, shell behavior, networking, SSH, or hardware claims.
