# Roadmap

Talos is planned as a long-running Raspberry Pi 5 operating-system program, not as a single bring-up spike. The roadmap is organized around user-visible capabilities and validation gates. Each phase should leave the repository in a buildable, documented state.

The near-term strategy is dual-target:

- talos-aarch64-virt: a fast QEMU virt target for architecture work, tests, and CI.
- talos-rpi5-bcm2712: the physical Raspberry Pi 5 target, validated through the lab controller and serial console.

QEMU does not currently model the Raspberry Pi 5, BCM2712, or RP1. The physical Pi 5 lab is therefore the authority for board behavior. QEMU is still valuable for generic AArch64 boot, exceptions, MMU, scheduler, and pure subsystem tests.

The Pi 5 boot path should follow the normal firmware contract first. The EEPROM bootloader loads the kernel image directly, prefers kernel_2712.img, falls back to kernel8.img, and passes the physical device-tree address in x0 according to the arm64 boot ABI. Talos should implement that handoff before considering any custom boot path.

## Current Status

Architecture quality campaign: the scheduled senior-engineer subsystem review
and refactor campaign is complete through closeout. The campaign reviewed and
fixed entry/boot/targets, memory/MMU/allocator, device tree/boot reports,
console/TTY/command/stdio, scheduler/SMP/synchronization,
POSIX/syscall/descriptors, VFS/loader/userspace, and docs/scripts/evidence
hygiene, then ran two full-system review cycles. The accepted closeout
frontier allows the descriptor-backed VFS/open/read/userspace feature chain to
resume, with retained review risks tracked as non-blocking follow-up context.

Priority correction after the Phase 10 shell-command review: Talos should not
plan additional fake/kernel-backed shell command expansion as operating-system
progress. The accepted `ls`, `cd`, `pwd`, and `cat` command-loop slices may
remain useful regression/control surfaces, but future command-visible behavior
must be backed by real VFS, descriptor, syscall, or userspace-program
capability. The immediate feature path is descriptor-backed read-only
initramfs/VFS file I/O, then program loading and userspace process launch,
then shell behavior that consumes those layers.

The post-review correction chain is:

1. descriptor-backed read-only initramfs/VFS file I/O: accepted in
   `phase8-open-read-initramfs-descriptor-integration-20260603` with
   descriptor-backed QEMU/substitute reads for `/etc/banner.txt` and
   `/bin/init`;
2. POSIX-shaped open/read syscall-substitute surface: accepted in
   `phase8-open-read-syscall-surface-20260603` with path-taking read-only
   `open`, filesystem-backed descriptor `read`, deterministic negative
   cases, and QEMU/substitute evidence for `/etc/banner.txt` and
   `/bin/init`;
3. program loader input from the VFS-backed `/bin/init` file: accepted in
   `phase8-program-loader-from-vfs-file-20260603` with the loader sourcing
   bytes through the read-only initramfs file-object boundary before returning
   its image plan;
4. smallest real initial userspace `/bin/init` launch boundary: accepted in
   `phase8-initial-userspace-process-launch-20260603` with QEMU/substitute
   evidence that the VFS-backed `/bin/init` ELF text is mapped at EL0, entered
   through the accepted launch/stack prerequisites, and reports a lower-AArch64
   SVC signal from the init entry path;
5. Phase 8 to Phase 10 shell transition checkpoint: accepted in
   `phase8-to-phase10-shell-transition-checkpoint-20260603`, recording the
   accepted chain, retained evidence, and rule that future shell-visible file
   behavior must consume VFS, descriptors, syscalls, and userspace rather than
   expanding fake command fixtures;
6. first shell-visible file operation backed by the accepted layers: accepted
   in `phase10-shell-backed-by-userspace-and-vfs-20260603` with
   `cat /etc/banner.txt` reading through `TalosOpen`/`TalosRead`, a
   descriptor-backed initramfs regular-file description, userspace-memory copy,
   cleanup of the opened descriptor/file-description pair, and QEMU/substitute
   evidence ending in `qemu-local-cat-banner-complete` PASS;
7. first shell-visible execution boundary backed by the accepted VFS/userspace
   layers: accepted in `phase10-shell-vfs-exec-boundary-20260603` with
   `exec /bin/init` reading `/bin/init` through the descriptor-backed
   `TalosOpen`/`TalosRead` path, parsing those bytes with the program loader,
   building process-install/address-space/materialization/initial-stack/launch
   records, and retaining QEMU/substitute evidence ending in
   `qemu-local-shell-vfs-exec-complete` PASS;
8. first shell-visible userspace completion/status observation: accepted in
   `phase10-shell-userspace-exit-status-20260603` with the VFS-backed
   `/bin/init` fixture setting `x0=0` before the accepted `svc #0x7a10`,
   QEMU lower-EL launch evidence observing that status in the exception frame,
   and shell-visible `exec /bin/init` reporting the matching
   `lower-aarch64-svc-status-equivalent` zero status. General process
   lifecycle, wait, external command lookup, argv/envp, PATH, pipes,
   redirection, writable filesystem, and Pi 5 proof remain deferred.
9. first explicit shell process lifecycle/status record: accepted in
   `phase10-process-lifecycle-status-record-core-20260603` with
   `exec /bin/init` preserving the VFS/open/read, loader, launch, and
   lower-AArch64 SVC status-equivalent lineage while routing the zero status
   through a stable kernel-owned lifecycle record. The retained transcript
   reports process identity, shell parent ownership, exited state, observed
   status, and reaped state. Shell observation of the most recent lifecycle
   record, general wait/waitpid, asynchronous execution, PATH lookup,
   argv/envp, pipes, redirection, writable filesystem, hardware proof,
   networking, and SSH remain deferred.
10. first shell-visible observation of the most recent lifecycle/status
    record: accepted in
    `phase10-shell-last-process-status-observation-20260603` with
    `laststatus` reporting the same `/bin/init` lifecycle identity, shell
    parent ownership, exited state, zero status, observed status, and reaped
    state from the accepted kernel-owned lifecycle record. No-prior-process
    behavior is deterministic with `talos: last-process none`. General
    wait/waitpid, asynchronous execution, multiple children, zombie policy,
    PATH lookup, argv/envp, pipes, redirection, writable filesystem, hardware
    proof, networking, and SSH remain deferred.
11. minimal startup argc/argv for the explicit VFS-backed `/bin/init` exec
    path: accepted in
    `phase10-minimal-argv-argc-exec-init-20260603` with the initial stack
    record carrying `argc=1`, `argv[0]=/bin/init`, non-null argv state, an
    argv0 user pointer, and copied startup bytes while preserving the same
    VFS/open/read, loader, launch, lifecycle, `laststatus`, and VFS cat
    regression surfaces. envp/auxv/TLS, environment variables, PATH lookup,
    arbitrary executable dispatch, descriptor inheritance, wait/waitpid,
    asynchronous execution, pipes, redirection, writable filesystem, hardware
    proof, networking, and SSH remain deferred.
12. deterministic empty envp for the same explicit VFS-backed `/bin/init`
    exec path: accepted in `phase10-empty-envp-exec-init-20260603` with
    the initial stack record preserving `argc=1`, `argv[0]=/bin/init`,
    and non-null argv state while adding `envp-state=empty-envp0`,
    `envp-entries=0`, an envp NULL-slot user pointer, and
    `copied-startup-bytes=0x2a`. Environment variables, auxv/TLS, libc
    startup, PATH lookup, arbitrary executable dispatch, descriptor
    inheritance, wait/waitpid, asynchronous execution, pipes, redirection,
    writable filesystem, hardware proof, networking, and SSH remain deferred.
13. minimal startup ABI closeout: accepted in
    `phase10-startup-abi-closeout-20260603`. The accepted frontier is limited
    to the explicit VFS-backed `/bin/init` startup record with `argc=1`,
    `argv[0]=/bin/init`, non-null argv state, `envp-state=empty-envp0`,
    `envp-entries=0`, and `copied-startup-bytes=0x2a`, all tied to
    `source=initial-user-stack-record` in the same VFS/open/read, loader,
    launch, lifecycle, and status lineage. The recommended next feature-led
    step is supervisor planning for absolute VFS executable dispatch before
    PATH lookup, wait/waitpid, descriptor inheritance, pipes, redirection,
    writable filesystem, hardware proof, networking, or SSH.
14. absolute VFS executable dispatch core: accepted in
    `phase10-absolute-vfs-exec-dispatch-core-20260603` with
    shell-visible `exec /bin/zero` and `exec /bin/init` both reading
    executable bytes through descriptor-backed VFS/open/read before the
    accepted loader/process/launch/startup/lifecycle/status chain.
    `/bin/zero` reports `argv[0]=/bin/zero`,
    `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty
    envp, zero status, and path-aware `laststatus`. Missing paths,
    relative/PATH-style names, directories, non-ELF files, and empty files
    fail deterministically. PATH lookup, broad argv/envp, nonzero status
    variation, wait/waitpid, descriptor inheritance, pipes, redirection,
    writable filesystem, hardware proof, networking, and SSH remain deferred.
15. absolute VFS executable dispatch closeout: accepted in
    `phase10-absolute-vfs-exec-dispatch-closeout-20260603`. It reconciles the
    accepted `/bin/zero` non-init dispatch, `/bin/init` regression,
    path-aware `laststatus`, negative controls, and descriptor-backed VFS cat
    evidence. The next queued feature-led slice remains nonzero status
    variation through a VFS-backed executable before PATH lookup,
    wait/waitpid, descriptor inheritance, pipes, redirection, writable
    filesystem, hardware proof, networking, or SSH.
16. nonzero VFS exec status core: accepted in
    `phase10-vfs-exec-nonzero-status-core-20260603` with shell-visible
    `exec /bin/status42` reading executable bytes through descriptor-backed
    VFS/open/read before the accepted loader/process/launch/startup/
    lifecycle/status chain. `/bin/status42` reports
    `argv[0]=/bin/status42`,
    `state=minimal-argc1-argv0-absolute-empty-envp`, deterministic empty
    envp, copied startup bytes for the longer path, lifecycle/status
    `0x2a`, and matching `laststatus`. `/bin/init` and `/bin/zero`
    remain zero-status controls. Missing paths, relative/PATH-style names,
    directories, non-ELF files, and empty files fail deterministically. PATH
    lookup, broad argv/envp, wait/waitpid, descriptor inheritance, pipes,
    redirection, writable filesystem, hardware proof, networking, and SSH
    remain deferred.
17. nonzero VFS exec status closeout: accepted in
    `phase10-vfs-exec-nonzero-status-closeout-20260603`. It reconciles
    the accepted `/bin/status42` nonzero status transcript, `/bin/init`
    and `/bin/zero` zero-status controls, matching `laststatus`,
    deterministic negative exec controls, and descriptor-backed VFS cat
    evidence. The next recommended feature-led local execution primitive is
    minimal wait/waitpid-style lifecycle observation backed by the accepted
    kernel-owned lifecycle/status record, before descriptor inheritance, PATH
    lookup, broad argv/envp, pipes, redirection, writable filesystem,
    hardware proof, networking, or SSH.
18. minimal waitpid lifecycle observation core: accepted in
    `phase10-minimal-waitpid-lifecycle-observation-core-20260603` with a
    shell-visible `waitpid` observation command backed by the accepted
    VFS-exec lifecycle/status record. `waitpid` reports no-child before any
    successful exec, reports and consumes the completed `/bin/status42`
    lifecycle/status record with status `0x2a`, reports no-child on a
    second wait, and leaves `laststatus` as the non-consuming latest
    lifecycle regression view. `/bin/init` and `/bin/zero` remain zero-status
    wait controls; deterministic negative exec controls and descriptor-backed
    `cat /etc/banner.txt` remain covered. This does not accept asynchronous
    execution, multiple children, broad zombie-table policy, fork, signals,
    descriptor inheritance expansion, PATH lookup, pipes, redirection,
    writable filesystem, hardware proof, networking, or SSH.
19. waitpid lifecycle observation closeout: accepted in
    `phase10-waitpid-lifecycle-observation-closeout-20260603`. It reconciles
    the accepted single-child wait observation evidence, status `0x2a`
    variation for `/bin/status42`, zero-status `/bin/init` and `/bin/zero`
    controls, deterministic no-child and already-consumed behavior,
    non-consuming `laststatus`, negative exec controls, and descriptor-backed
    `cat /etc/banner.txt`. The next recommended feature-led local execution
    primitive is standard descriptor inheritance across VFS-backed exec before
    PATH lookup, broad argv/envp, pipes, redirection, writable filesystem,
    hardware proof, networking, or SSH.
20. standard descriptor inheritance across VFS-backed exec: accepted in
    `phase10-standard-descriptor-inheritance-exec-core-20260603` with
    shell-visible `exec /bin/status42`, `exec /bin/init`, and
    `exec /bin/zero` each reporting inherited standard descriptors
    `fd0=stdio-input`, `fd1=stdio-output`, and `fd2=stdio-output`
    from the shell process descriptor table. The same transcript proves the
    loader/VFS temporary executable-read descriptor is absent from the
    inherited set with `loader-temp-open=false`. Nonzero/zero status
    controls, `laststatus`, `waitpid`, deterministic negative exec
    controls, and descriptor-backed `cat /etc/banner.txt` remain covered.
    Userspace stdio I/O through inherited descriptors, broad descriptor
    inheritance policy, close-on-exec behavior, fork, asynchronous execution,
    multiple children, PATH lookup, broad argv/envp, pipes, redirection,
    writable filesystem, hardware proof, networking, and SSH remain deferred.
21. standard descriptor inheritance closeout: accepted in
    `phase10-standard-descriptor-inheritance-closeout-20260603`. It
    reconciles the accepted standard `fd0`/`fd1`/`fd2` inheritance
    records, loader/VFS temporary descriptor non-leak evidence,
    `/bin/status42` nonzero status, `/bin/init` and `/bin/zero`
    zero-status controls, `laststatus`, `waitpid`, deterministic negative
    exec controls, and descriptor-backed `cat /etc/banner.txt`. The next
    recommended feature-led local execution primitive is minimal literal argv
    expansion for absolute VFS exec, before PATH lookup, pipes, redirection,
    userspace stdio I/O through inherited descriptors, broad descriptor
    policy, writable filesystem, hardware proof, networking, or SSH.
22. literal argv propagation for absolute VFS exec: accepted in
    `phase10-literal-argv-exec-core-20260603` with shell-visible
    `exec /bin/status42 alpha beta` still reading executable bytes through
    descriptor-backed VFS/open/read before the accepted loader/process/launch/
    startup/lifecycle/status chain. The startup ABI transcript now records
    `state=literal-argv-absolute-empty-envp`, `argc=3`,
    `argv[0]=/bin/status42`, `argv[1]=alpha`, `argv[2]=beta`,
    deterministic empty envp, adjusted startup pointers, and
    `copied-startup-bytes=0x49`. The same QEMU/substitute evidence preserves
    inherited `fd0`/`fd1`/`fd2`, loader temporary descriptor non-leak,
    `waitpid`, non-consuming `laststatus`, `/bin/init` and `/bin/zero`
    controls, unsupported glob/escape-style grammar rejection, missing and
    relative exec negatives, and descriptor-backed `cat /etc/banner.txt`.
    Quoting, escaping, globbing, variables, PATH lookup, broad envp/auxv/TLS,
    userspace stdio I/O through inherited descriptors, pipes, redirection,
    writable filesystem, hardware proof, networking, and SSH remain deferred.
23. literal argv exec closeout: accepted in
    `phase10-literal-argv-exec-closeout-20260603`. It reconciles the
    accepted `exec /bin/status42 alpha beta` startup ABI evidence,
    descriptor-backed VFS/open/read and loader/lifecycle/status lineage,
    inherited standard descriptor records, loader temporary descriptor
    non-leak, `waitpid`, non-consuming `laststatus`, zero-status
    controls, unsupported grammar rejection, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt`. The next recommended
    feature-led local execution primitive is minimal fixed `/bin`
    PATH-style lookup for bare executable names, with no environment-backed
    PATH, current-directory search, quoting/globbing/variables, pipes,
    redirection, userspace stdio I/O, writable filesystem, hardware proof,
    networking, or SSH.
24. minimal fixed `/bin` PATH-style exec lookup core: accepted in
    `phase10-minimal-path-lookup-exec-core-20260603`. Shell-visible
    `exec status42 alpha beta` resolves the bare first exec token only to
    `/bin/status42`, reads the resolved executable through descriptor-backed
    VFS/open/read, and runs it through the accepted loader/startup/launch/
    lifecycle/status chain. The accepted argv0 policy is canonical resolved
    path argv0: the startup ABI records `argv0=/bin/status42`,
    `argv1=alpha`, and `argv2=beta` with deterministic empty envp, inherited
    standard descriptors, loader temporary descriptor non-leak, nonzero
    status `0x2a`, and matching `waitpid`/`laststatus`. The retained
    QEMU/substitute evidence also covers bare `exec init` and `exec zero`
    zero-status controls, absolute `exec /bin/status42 gamma`, missing bare
    name, path-like relative name, directory and non-executable negatives,
    unsupported glob grammar rejection, descriptor-backed
    `cat /etc/banner.txt`, and `qemu-local-shell-path-lookup-complete` PASS.
    Environment-backed PATH, current-directory search, command hashing, shell
    builtin conversion, quoting/escaping/globbing, variables, pipes,
    redirection, userspace stdio I/O, writable filesystem, hardware proof,
    networking, and SSH remain deferred.
25. minimal fixed `/bin` PATH-style exec lookup closeout: accepted in
    `phase10-minimal-path-lookup-exec-closeout-20260603`. It reconciles the
    accepted bare `exec status42 alpha beta` lookup transcript, canonical
    resolved-path argv0 policy, descriptor-backed VFS/open/read and
    loader/lifecycle/status lineage, inherited standard descriptor records,
    loader temporary descriptor non-leak, `waitpid`, non-consuming
    `laststatus`, zero-status controls, absolute exec regression,
    deterministic negative controls, unsupported grammar rejection, and
    descriptor-backed `cat /etc/banner.txt`. The next recommended
    feature-led local shell primitive is minimal userspace stdout I/O through
    inherited standard descriptors before pipes, redirection, writable
    filesystem behavior, broader shell grammar, hardware proof, networking, or
    SSH.
26. userspace stdout through inherited fd core: accepted in
    `phase10-userspace-stdout-inherited-fd-core-20260603`. Shell-visible
    `exec stdout` resolves through the accepted fixed `/bin` lookup to
    `/bin/stdout`, reads the executable through descriptor-backed VFS/open/read,
    and runs it through the accepted loader/startup/launch/descriptor
    inheritance/lifecycle/status chain. The launched fixture emits the
    deterministic line `Talos userspace stdout fixture` through inherited
    `fd1=stdio-output` using the process descriptor `TalosWrite`
    syscall-substitute path and records the matching write byte count and
    return value. The retained evidence also covers `waitpid`, non-consuming
    `laststatus`, `/bin/status42` nonzero status, `/bin/init` and `/bin/zero`
    zero-status controls, absolute exec regression, fixed `/bin` lookup,
    negative exec controls, loader temporary descriptor non-leak, inherited
    standard descriptors, descriptor-backed `cat /etc/banner.txt`,
    `qemu-local-shell-userspace-stdout-complete`, and PASS. Stdin reads,
    stderr-specific output, pipes, redirection, writable filesystem behavior,
    fork/async execution, libc stdio, broader shell grammar, hardware proof,
    networking, and SSH remain deferred.
27. userspace stdout through inherited fd closeout: accepted in
    `phase10-userspace-stdout-inherited-fd-closeout-20260603`. It reconciles
    the accepted process-originated stdout transcript, descriptor-backed
    VFS/open/read and loader/lifecycle/status lineage, inherited descriptor
    records, `exec-stdout fd=1 bytes=0x1f return=0x1f
    source=userspace-talos-write`, `waitpid`, non-consuming `laststatus`,
    retained fixed `/bin` lookup and absolute exec controls, `/bin/status42`
    nonzero status, `/bin/init` and `/bin/zero` zero-status controls,
    deterministic negative exec controls, loader temporary descriptor non-leak,
    and descriptor-backed `cat /etc/banner.txt`. The next accepted
    feature-led local I/O primitive is a bounded userspace read through
    inherited `fd0=stdio-input`, reported through the accepted userspace stdout
    path.
28. userspace stdin through inherited fd core: accepted in
    `phase10-userspace-stdin-inherited-fd-core-20260603`. Shell-visible
    `exec stdin` resolves through the accepted fixed `/bin` lookup to
    `/bin/stdin`, reads the executable through descriptor-backed VFS/open/read,
    and runs it through the accepted loader/startup/launch/descriptor
    inheritance/lifecycle/status chain. The launched fixture performs a bounded
    `TalosRead` through inherited `fd0=stdio-input` from a deterministic
    QEMU/substitute proof input buffer `talos-fd0\n`, then reports
    `Talos userspace stdin fixture read: talos-fd0` through inherited
    `fd1=stdio-output` using the accepted process descriptor `TalosWrite`
    path. The transcript records `exec-stdin fd=0 bytes=0xa return=0xa
    stdout-fd=1 stdout-bytes=0x2f stdout-return=0x2f
    source=userspace-talos-read+userspace-talos-write`, zero lifecycle status
    for `/bin/stdin`, matching `waitpid` and non-consuming `laststatus`,
    retained userspace stdout fixture evidence, `/bin/init` and `/bin/zero`
    controls, literal argv control, deterministic negative exec controls,
    descriptor-backed `cat /etc/banner.txt`,
    `qemu-local-shell-userspace-stdin-complete`, and PASS. The first
    implementation also fixed an overlap between the stdout prefix buffer and
    the fd0 read destination; the accepted evidence asserts the non-corrupted
    `talos-fd0` bytes. Runtime-console0/TTY-backed stdin, EOF/no-data/error
    variants, stderr-specific output, blocking I/O, pipes, redirection,
    writable filesystem behavior, broader shell grammar, hardware proof,
    networking, and SSH remain deferred.
29. userspace stdin through inherited fd closeout: accepted in
    `phase10-userspace-stdin-inherited-fd-closeout-20260603`. It reconciles
    the accepted `exec stdin` transcript, descriptor-backed VFS/open/read and
    loader/lifecycle/status lineage, inherited descriptor records,
    `exec-stdin fd=0 bytes=0xa return=0xa stdout-fd=1 stdout-bytes=0x2f
    stdout-return=0x2f source=userspace-talos-read+userspace-talos-write`,
    the visible stdout report `Talos userspace stdin fixture read: talos-fd0`,
    `waitpid`, non-consuming `laststatus`, retained userspace stdout
    regression evidence, fixed `/bin` lookup and absolute exec controls,
    `/bin/init` and `/bin/zero` zero-status controls, deterministic negative
    exec controls, loader temporary descriptor non-leak, and descriptor-backed
    `cat /etc/banner.txt`. The next queued stderr-through-inherited-fd task
    remains mechanically unblocked by accepted fd2 inheritance plus accepted
    stdout/stdin syscall-substitute evidence. Runtime-console0/TTY-backed
    process stdin, EOF/no-data/error stdin variants, stderr behavior, blocking
    I/O, pipes, redirection, writable filesystem behavior, hardware proof,
    networking, and SSH remain deferred.
30. userspace stderr through inherited fd core: accepted in
    `phase10-userspace-stderr-inherited-fd-core-20260603`. Shell-visible
    `exec stderr` resolves through the accepted fixed `/bin` lookup to
    `/bin/stderr`, reads the executable through descriptor-backed VFS/open/read,
    and runs it through the accepted loader/startup/launch/descriptor
    inheritance/lifecycle/status chain. The launched fixture emits
    `Talos userspace stderr fixture` through inherited `fd2=stdio-output` using
    the process descriptor `TalosWrite` syscall-substitute path and records
    `exec-stderr fd=2 bytes=0x1f return=0x1f source=userspace-talos-write`.
    The retained evidence also covers `waitpid`, non-consuming `laststatus`,
    `/bin/status42` nonzero status, `/bin/init` and `/bin/zero` zero-status
    controls, accepted stdout/stdin regressions, loader temporary descriptor
    non-leak, deterministic negative exec controls, descriptor-backed
    `cat /etc/banner.txt`, `qemu-local-shell-userspace-stderr-complete`, and
    PASS. `fd2` currently shares the accepted `stdio-output` backend with `fd1`;
    distinct stderr stream separation, pipes, redirection, terminal policy,
    blocking I/O, writable filesystem behavior, hardware proof, networking, and
    SSH remain deferred.
31. userspace stderr through inherited fd closeout: accepted in
    `phase10-userspace-stderr-inherited-fd-closeout-20260603`. It reconciles
    the accepted `exec stderr` transcript, descriptor-backed VFS/open/read and
    loader/lifecycle/status lineage, inherited descriptor records,
    `exec-stderr fd=2 bytes=0x1f return=0x1f
    source=userspace-talos-write`, the visible process-originated stderr
    fixture line, `waitpid`, non-consuming `laststatus`, retained
    userspace stdout and stdin regression evidence, fixed `/bin` lookup and
    absolute exec controls, `/bin/status42` nonzero status, `/bin/init` and
    `/bin/zero` zero-status controls, deterministic negative exec controls,
    loader temporary descriptor non-leak, and descriptor-backed
    `cat /etc/banner.txt`. The next queued stdio triad closeout remains
    mechanically unblocked by accepted fd0/fd1/fd2 process descriptor evidence.
    Distinct stderr stream routing, runtime-console0/TTY-backed process stdin,
    blocking I/O, pipes, redirection, writable filesystem behavior, hardware
    proof, networking, and SSH remain deferred.
32. userspace standard stdio triad closeout: accepted in
    `phase10-userspace-stdio-triad-closeout-20260603`. It reconciles the
    accepted process-originated standard descriptor operations: `/bin/stdout`
    writes through inherited fd1 with `source=userspace-talos-write`,
    `/bin/stdin` reads deterministic proof input through inherited fd0 and
    reports through the accepted fd1 path with
    `source=userspace-talos-read+userspace-talos-write`, and `/bin/stderr`
    writes through inherited fd2 with `source=userspace-talos-write`.
    The checkpoint keeps kernel-backed shell built-ins as regression/control
    surfaces only; accepted userspace stdio capability is limited to launched
    VFS-backed fixtures using inherited process descriptors. Retained evidence
    also covers descriptor-backed VFS/open/read exec, loader temporary
    descriptor non-leak, lifecycle/status, `waitpid`, non-consuming
    `laststatus`, fixed `/bin` lookup, literal argv, zero/nonzero status
    controls, negative exec controls, and descriptor-backed
    `cat /etc/banner.txt`. Runtime-console0/TTY-backed process stdin,
    EOF/no-data/error stdin variants beyond the accepted proof input, blocking
    scheduler I/O, close/dup/fork policy, pipes, redirection, distinct stderr
    stream routing, writable filesystem behavior, broad shell grammar, libc
    stdio, Pi 5 proof, networking, and SSH remain deferred. The next accepted
    feature-led local I/O primitive is a bounded runtime-console0/local-input
    read through inherited fd0, reported through the accepted userspace fd1
    stdout path.
33. runtime-console0 userspace stdin core: accepted in
    `phase10-runtime-console0-stdin-core-20260603`. Shell-visible
    `exec stdin` continues to resolve through the accepted fixed `/bin`
    lookup to `/bin/stdin`, read the executable through descriptor-backed
    VFS/open/read, and run it through the accepted loader/startup/descriptor
    inheritance/lifecycle/status/`waitpid`/`laststatus` chain. The launched
    fixture now performs one bounded `TalosRead` through inherited
    `fd0=stdio-input` from runtime-console0/local input plumbing rather than
    the older deterministic `FixedStdin` proof buffer. The QEMU/substitute
    transcript feeds `talos-console0` after the `exec stdin` command
    terminator, reports `Talos userspace stdin fixture read: talos-console0`
    through inherited fd1, and records
    `exec-stdin fd=0 bytes=0xe return=0xe
    read-source=runtime-console0/local-input stdout-fd=1 stdout-bytes=0x33
    stdout-return=0x33 source=userspace-talos-read+userspace-talos-write`.
    Retained evidence also covers stdout, proof-buffer stdin as historical
    regression evidence only, stderr, fixed `/bin` lookup, lifecycle/status,
    `waitpid`, non-consuming `laststatus`, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt`. EOF/no-data/error stdin
    variants, blocking scheduler I/O, terminal policy expansion, async
    execution, fork, signals, pipes, redirection, distinct stderr routing,
    writable filesystem behavior, libc stdio, Pi 5 proof, networking, and SSH
    remain deferred. The queued runtime-console0 stdin closeout is
    mechanically unblocked by this accepted evidence.
34. runtime-console0 userspace stdin closeout: accepted in
    `phase10-runtime-console0-stdin-closeout-20260603`. It reconciles the
    accepted `exec stdin` transcript where `/bin/stdin` reads
    `talos-console0` through inherited `fd0=stdio-input` from
    runtime-console0/local-input plumbing, then reports the bytes through
    inherited fd1 using the accepted userspace TalosWrite path. The closeout
    distinguishes this accepted frontier from the older proof-buffer stdin
    evidence and from unaccepted EOF/no-data/error, blocking scheduler I/O,
    terminal policy, pipes, redirection, writable filesystem, Pi 5 proof,
    networking, and SSH claims. The queued EOF/no-data stdin variant remains
    mechanically unblocked as the next narrow local I/O primitive, with the
    successful runtime-console0 stdin read retained as a regression.
35. stdin EOF/no-data core: accepted in
    `phase10-stdin-eof-no-data-core-20260603`. Shell-visible `exec stdin`
    continues to resolve through the accepted fixed `/bin` lookup to
    `/bin/stdin`, read the executable through descriptor-backed VFS/open/read,
    and run it through the accepted loader/startup/descriptor inheritance/
    lifecycle/status/`waitpid`/`laststatus` chain. When runtime-console0/local
    input has no bytes immediately available after the command terminator, the
    launched fixture observes `TalosRead` return `0` through inherited
    `fd0=stdio-input`, reports
    `Talos userspace stdin fixture no-data: eof` through inherited fd1, and
    records
    `exec-stdin fd=0 bytes=0 return=0
    read-source=runtime-console0/local-input stdout-fd=1 stdout-bytes=0x2b
    stdout-return=0x2b source=userspace-talos-read+userspace-talos-write
    read-result=eof/no-data`. The successful runtime-console0
    `talos-console0` stdin read remains retained as the happy-path regression
    alongside stdout/stderr, VFS exec, lifecycle/status, `waitpid`,
    non-consuming `laststatus`, fixed `/bin` lookup, zero/nonzero status
    controls, negative exec controls, and descriptor-backed
    `cat /etc/banner.txt`. Blocking scheduler I/O, readiness/polling APIs,
    canonical terminal policy expansion, async execution, fork, signals, pipes,
    redirection, distinct stderr stream routing, writable filesystem behavior,
    libc stdio, Pi 5 proof, networking, and SSH remain deferred. The queued
    EOF/no-data closeout is mechanically unblocked by this accepted evidence.
36. stdin EOF/no-data closeout: accepted in
    phase10-stdin-eof-no-data-closeout-20260603. It reconciles the accepted
    successful runtime-console0/local-input fd0 read and deterministic
    EOF/no-data branch for the same inherited fd0 path. The accepted stdin
    frontier now includes exec stdin resolving through fixed /bin lookup,
    descriptor-backed VFS/open/read loading of /bin/stdin, inherited
    fd0=stdio-input reads from runtime-console0/local-input, visible reports
    through inherited fd1, lifecycle/status, waitpid, non-consuming
    laststatus, zero/nonzero controls, negative exec controls, and
    descriptor-backed cat /etc/banner.txt regressions. The closeout prevents
    acceptance drift back to proof-buffer-only stdin and does not accept
    blocking terminal reads, readiness/polling APIs, canonical terminal policy
    expansion, async execution, fork, signals, pipes, redirection, distinct
    stderr stream routing, writable filesystem behavior, libc stdio, Pi 5
    proof, networking, or SSH. No broader shell I/O task is mechanically
    unblocked by the closeout alone; supervisor planning is required for the
    next feature-led local I/O primitive.
37. runtime-console0 stdin readiness distinction core: accepted in
    `phase10-runtime-stdin-readiness-distinction-core-20260604`. This task
    corrects the earlier EOF/no-data wording for runtime-console0/local-input:
    ordinary no bytes immediately available now returns `-EAGAIN` from the
    inherited `fd0=stdio-input` `TalosRead` path, reports
    `Talos userspace stdin fixture no-data: readiness` through inherited fd1,
    and records `read-result=readiness/no-data`. True terminal EOF remains
    unimplemented for runtime-console0/local-input; fixed proof-buffer stdin
    retains its bounded EOF behavior as historical Phase 7 evidence. The
    successful `talos-console0` stdin read, stdout/stderr, descriptor-backed
    VFS exec, lifecycle/status, consuming `waitpid`, non-consuming
    `laststatus`, fixed `/bin` lookup, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt` remain retained regressions.
    Scheduler-backed blocking reads, wait queues, select/poll, nonblocking
    flags, Ctrl-D EOF policy, async execution, fork, signals, pipes,
    redirection, distinct stderr stream routing, writable filesystem behavior,
    libc stdio, Pi 5 proof, networking, and SSH remain deferred.
38. runtime-console0 stdin readiness distinction closeout: accepted in
    `phase10-runtime-stdin-readiness-distinction-closeout-20260604`. This
    checkpoint reconciles the accepted readiness/no-data implementation and
    retained evidence before bounded wait work. The evidence map keeps ordinary
    no-data as `-EAGAIN`/`read-result=readiness/no-data`, keeps true terminal
    EOF unimplemented for runtime-console0/local-input, and keeps the
    successful `talos-console0` stdin read as a separate regression. Retained
    stdout/stderr, descriptor-backed VFS exec, lifecycle/status, consuming
    `waitpid`, non-consuming `laststatus`, fixed `/bin` lookup, negative
    exec controls, and descriptor-backed `cat /etc/banner.txt` evidence remain
    mapped. The bounded runtime-console0 stdin wait core is mechanically
    unblocked as the next local feature step, but scheduler-backed blocking
    reads, wait queues, select/poll, nonblocking flags, Ctrl-D EOF policy,
    async execution, fork, signals, pipes, redirection, distinct stderr stream
    routing, writable filesystem behavior, libc stdio, Pi 5 proof, networking,
    and SSH remain deferred.
39. bounded runtime-console0 stdin wait core: accepted in
    `phase10-bounded-runtime-stdin-wait-core-20260604`. Shell-visible
    VFS-backed `exec stdin` now has a task-local bounded retry step for
    runtime-console0/local-input: the delayed-byte evidence records
    `Talos userspace stdin fixture no-data: readiness` first, then consumes
    delayed `talos-console0` bytes through inherited `fd0=stdio-input` and
    reports them through inherited fd1 with
    `read-result=bounded-wait/delayed-input` and
    `readiness-observations=...`. A separate no-bytes-within-budget
    regression still reports `-EAGAIN` and
    `read-result=readiness/no-data` without claiming true EOF or hanging.
    Immediate runtime-console0 stdin, stdout/stderr, descriptor-backed VFS
    exec, loader temporary descriptor non-leak, lifecycle/status, consuming
    `waitpid`, non-consuming `laststatus`, fixed `/bin` lookup, negative
    exec controls, and descriptor-backed `cat /etc/banner.txt` remain
    retained regressions. This is not full POSIX blocking read semantics:
    scheduler-backed blocking reads, wait queues, select/poll, nonblocking
    flags, Ctrl-D EOF policy, async execution, fork, signals, pipes,
    redirection, distinct stderr stream routing, writable filesystem behavior,
    libc stdio, Pi 5 proof, networking, and SSH remain deferred.
40. bounded runtime-console0 stdin wait closeout: accepted in
    `phase10-bounded-runtime-stdin-wait-closeout-20260604`. This checkpoint
    reconciles the delayed-byte bounded wait and no-bytes-within-budget
    readiness/no-data regression before any broader I/O work. The evidence map
    keeps delayed `talos-console0` bytes as
    `read-result=bounded-wait/delayed-input`, keeps no bytes within the
    bounded budget as `-EAGAIN`/`read-result=readiness/no-data`, retains the
    successful immediate runtime-console0 stdin read, and preserves
    stdout/stderr, descriptor-backed VFS exec, loader temporary descriptor
    non-leak, lifecycle/status, consuming `waitpid`, non-consuming
    `laststatus`, fixed `/bin` lookup, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt` regressions. The bounded retry is
    not full POSIX blocking read semantics. No explicit queued follow-up task
    remains; supervisor planning is required before scheduler-backed
    blocking/readiness, Ctrl-D EOF, distinct stderr routing, pipes,
    redirection, or other local I/O expansion.
41. scheduler-backed runtime-console0 stdin wait core: accepted in
    `phase10-scheduler-backed-stdin-wait-core-20260604`. Shell-visible
    VFS-backed `exec stdin` now records an explicit scheduler-owned stdin
    wait/readiness state after the first inherited fd0 `TalosRead` returns
    `-EAGAIN`. The delayed-byte evidence records `talos: stdin-wait` sleep and
    wake/resume markers tied to task `0x100001`, fd0, blocked/runnable task
    states, wait-cycle count, and
    `source=scheduler-runtime-console-readiness`, then consumes delayed
    `talos-console0` bytes through inherited fd0 and reports them through
    inherited fd1 with `read-result=scheduler-wait/delayed-input`. The
    no-delayed-input control remains `-EAGAIN` with
    `result=timeout/no-false-eof` and does not claim terminal EOF. The prior
    bounded retry loop is no longer the accepted delayed-stdin mechanism; the
    old bounded smoke path is retained only as compatibility scaffolding for
    scheduler-backed markers. Immediate runtime-console0 stdin, stdout/stderr,
    descriptor-backed VFS exec, loader temporary descriptor non-leak,
    lifecycle/status, consuming `waitpid`, non-consuming `laststatus`, fixed
    `/bin` lookup, negative exec controls, and descriptor-backed
    `cat /etc/banner.txt` remain retained regressions. Ctrl-D EOF, select/poll,
    nonblocking flags, async execution, fork, signals, termios, pipes,
    redirection, distinct stderr routing, writable filesystem behavior, libc
    stdio, Pi 5 proof, networking, and SSH remain deferred.
42. scheduler-backed runtime-console0 stdin wait closeout: accepted in
    `phase10-scheduler-backed-stdin-wait-closeout-20260604`. This checkpoint
    reconciles the accepted scheduler-owned stdin wait/readiness boundary:
    ordinary runtime-console0/local-input no-data remains `-EAGAIN`,
    delayed `talos-console0` bytes wake/resume the waiting VFS-backed
    `/bin/stdin` path through inherited fd0, and no delayed input within the
    finite QEMU/substitute harness records `timeout/no-false-eof` rather than
    true terminal EOF. The prior bounded retry evidence is superseded by
    `read-result=scheduler-wait/delayed-input` and retained only as
    compatibility scaffolding for scheduler-backed markers. Immediate stdin,
    stdout/stderr, descriptor-backed VFS exec, loader temporary descriptor
    non-leak, lifecycle/status, consuming `waitpid`, non-consuming
    `laststatus`, fixed `/bin` lookup, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt` remain mapped as regressions.
    Ctrl-D EOF remains the next smallest feature-led local I/O task before
    select/poll, nonblocking flags, async execution, fork, signals, termios,
    pipes, redirection, distinct stderr routing, writable filesystem behavior,
    libc stdio, Pi 5 proof, networking, or SSH.
43. terminal Ctrl-D EOF core: accepted in
    `phase10-terminal-ctrl-d-eof-core-20260604`. Shell-visible VFS-backed
    `exec stdin` now treats a first runtime-console0/local-input Ctrl-D
    0x04 byte on inherited `fd0=stdio-input` as true terminal EOF. The
    launched `/bin/stdin` fixture reports
    `Talos userspace stdin fixture read-result: terminal-eof` through
    inherited fd1 and records
    `exec-stdin ... return=0x0000000000000000 ... read-result=terminal-eof`.
    Ordinary no-data/readiness remains `-EAGAIN` with
    `read-result=readiness/no-data`, and delayed input remains
    `read-result=scheduler-wait/delayed-input` with scheduler sleep and
    wake/resume markers. Retained QEMU/substitute evidence also covers
    stdout/stderr, descriptor-backed VFS exec, loader temporary descriptor
    non-leak, lifecycle/status, consuming `waitpid`, non-consuming
    `laststatus`, fixed `/bin` lookup, negative exec controls, and
    descriptor-backed `cat /etc/banner.txt`. Full termios, POSIX signals,
    sessions, job control, select/poll, nonblocking flags, pipes, redirection,
    async execution, fork, writable filesystem behavior, distinct stderr
    routing, Pi 5 proof, networking, and SSH remain deferred.
44. terminal Ctrl-D EOF closeout: accepted in
    `phase10-terminal-ctrl-d-eof-closeout-20260604`. This checkpoint records
    the accepted stdin policy split for shell-visible VFS-backed
    `exec stdin`: first-byte Ctrl-D 0x04 on inherited fd0 is true terminal
    EOF with `return=0` and `read-result=terminal-eof`; ordinary no-data
    remains `-EAGAIN` with `read-result=readiness/no-data` and
    `timeout/no-false-eof`; delayed `talos-console0` input remains
    scheduler-owned `read-result=scheduler-wait/delayed-input` with sleep
    and wake/resume markers. Retained evidence still maps immediate
    runtime-console0 stdin, stdout/stderr, descriptor-backed VFS exec, loader
    temporary descriptor non-leak, lifecycle/status, consuming `waitpid`,
    non-consuming `laststatus`, fixed `/bin` lookup, negative exec controls,
    and descriptor-backed `cat /etc/banner.txt`. Full termios, POSIX signals,
    sessions, job control, select/poll, nonblocking flags, pipes, redirection,
    async execution, fork, writable filesystem behavior, libc stdio, Pi 5
    proof, networking, and SSH remain deferred. Distinct stderr routing is the
    next smallest queued local I/O task because fd2 still shares the accepted
    `stdio-output` backend with fd1.
45. distinct stderr routing core: accepted in
    `phase10-distinct-stderr-routing-core-20260604`. Shell-visible
    VFS-backed `exec stderr` now records inherited fd2 writes with explicit
    stream identity and route metadata:
    `stream=stderr route=runtime-console0/stderr`. The retained stdout
    control records inherited fd1 writes as
    `stream=stdout route=runtime-console0/stdout`, proving fd1 is not
    mislabeled as stderr. The accepted physical sink remains shared
    runtime-console0; this task does not add pipes, redirection, file-backed
    stderr, separate physical sinks, terminal colors/policy, libc stdio,
    async jobs, fork, signals, writable filesystem behavior, Pi 5 proof,
    networking, or SSH. Retained evidence still maps scheduler-backed stdin
    wait/readiness, Ctrl-D EOF, descriptor-backed VFS exec, loader temporary
    descriptor non-leak, lifecycle/status, consuming `waitpid`,
    non-consuming `laststatus`, fixed `/bin` lookup, negative exec controls,
    and descriptor-backed `cat /etc/banner.txt`. The queued distinct stderr
    routing closeout is mechanically unblocked.
46. distinct stderr routing closeout: accepted in
    `phase10-distinct-stderr-routing-closeout-20260604`. This checkpoint
    reconciles the accepted fd2 stream-origin split for shell-visible
    VFS-backed userspace execution: inherited fd2 writes now carry
    `stream=stderr route=runtime-console0/stderr`, while the retained fd1
    stdout control carries `stream=stdout route=runtime-console0/stdout`.
    The accepted physical sink remains shared runtime-console0. Retained
    evidence still maps scheduler-backed stdin wait/readiness, Ctrl-D EOF,
    descriptor-backed VFS exec, loader temporary descriptor non-leak,
    lifecycle/status, consuming `waitpid`, non-consuming `laststatus`,
    fixed `/bin` lookup, negative exec controls, and descriptor-backed
    `cat /etc/banner.txt`. Pipes, redirection, file-backed stderr/stdout,
    writable filesystem behavior, separate physical stdout/stderr sinks,
    terminal policy, async jobs, fork, signals, Pi 5 proof, networking, and SSH
    remain deferred. Supervisor planning is required before the next feature;
    if choosing between pipes and redirection, a narrow descriptor-routing
    redirection slice is likely smaller than pipe-backed producer/consumer
    lifecycle behavior, provided it does not claim writable filesystem support
    without an explicit writable target.
47. stdout-to-stderr descriptor-dup redirection core: accepted in
    `phase10-stdout-to-stderr-fd-dup-redirection-core-20260604`.
    Shell-visible `exec stdout 1>&2` now parses as the first exact
    descriptor-dup redirection form. The launched VFS-backed `/bin/stdout`
    child still loads through the accepted descriptor-backed `/bin` lookup and
    VFS/open/read path, but child fd1 is temporarily rebound to the inherited
    fd2 descriptor target. The task-owned QEMU/substitute evidence records
    `exec-redirection op=dup source-fd=1 target-fd=2
    target-stream=stderr target-route=runtime-console0/stderr
    child-only=true shell-restored=true`, followed by a userspace fd1 write
    with `stream=stderr route=runtime-console0/stderr`. A following normal
    `exec stdout` control records `stream=stdout
    route=runtime-console0/stdout`, proving the shell descriptor table is
    restored after the child launch. At that frontier, inverse `2>&1`, file,
    and pipe redirection forms were deterministic negatives. Inverse `2>&1`, regular
    file redirection, append/truncate, descriptor close/move syntax, pipes,
    writable filesystem behavior, separate physical sinks, terminal policy,
    async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH
    remain deferred. The queued stdout-to-stderr redirection closeout is
    mechanically unblocked.
48. stdout-to-stderr descriptor-dup redirection closeout: accepted in
    `phase10-stdout-to-stderr-fd-dup-redirection-closeout-20260604`. This
    checkpoint accepts only the narrow `exec stdout 1>&2` boundary:
    child fd1 is rebound to the inherited fd2 stderr route for the launched
    VFS-backed `/bin/stdout` executable, and the shell descriptor table is
    restored afterward. Retained QEMU/substitute evidence maps redirected fd1
    writes as `stream=stderr route=runtime-console0/stderr`, normal fd1
    stdout writes as `stream=stdout route=runtime-console0/stdout`, and
    normal fd2 stderr writes as
    `stream=stderr route=runtime-console0/stderr`. The evidence map also
    retains scheduler-backed stdin wait/readiness, Ctrl-D EOF,
    descriptor-backed VFS exec, loader temporary descriptor non-leak,
    lifecycle/status, consuming `waitpid`, non-consuming `laststatus`,
    fixed `/bin` lookup, deterministic negative exec/redirection controls,
    and descriptor-backed `cat /etc/banner.txt`. Inverse `2>&1`,
    descriptor close/move syntax, regular-file redirection, append/truncate,
    pipes, writable filesystem behavior, separate physical sinks, terminal
    policy, async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and
    SSH remain deferred. The queued inverse descriptor-duplication slice is
    mechanically unblocked, provided it stays bounded to child-only `2>&1`
    descriptor duplication.
49. stderr-to-stdout descriptor-dup redirection core: accepted in
    `phase10-stderr-to-stdout-fd-dup-redirection-core-20260604`.
    Shell-visible `exec stderr 2>&1` now parses as the inverse exact
    descriptor-dup redirection form. The launched VFS-backed `/bin/stderr`
    child still loads through the accepted descriptor-backed `/bin` lookup and
    VFS/open/read path, but child fd2 is temporarily rebound to the inherited
    fd1 descriptor target. The task-owned QEMU/substitute evidence records
    `exec-redirection op=dup source-fd=2 target-fd=1
    target-stream=stdout target-route=runtime-console0/stdout
    child-only=true shell-restored=true`, followed by a userspace fd2 write
    with `stream=stdout route=runtime-console0/stdout`. A following normal
    `exec stderr` control records `stream=stderr
    route=runtime-console0/stderr`, proving the shell descriptor table is
    restored after the child launch. Accepted `exec stdout 1>&2` behavior,
    normal stdout/stderr route controls, terminal EOF/readiness, lifecycle,
    waitpid/laststatus, negative redirection, and descriptor-backed cat
    controls remain covered. Descriptor close/move syntax, arbitrary `N>&M`,
    regular-file redirection, append/truncate, pipes, writable filesystem
    behavior, separate physical sinks, terminal policy, async jobs, fork,
    signals, libc stdio, Pi 5 proof, networking, and SSH remain deferred. The
    queued inverse descriptor-dup closeout is mechanically unblocked.
50. stderr-to-stdout descriptor-dup redirection closeout: accepted in
    `phase10-stderr-to-stdout-fd-dup-redirection-closeout-20260604`. This
    checkpoint accepts only the narrow `exec stderr 2>&1` boundary:
    child fd2 is rebound to the inherited fd1 stdout route for the launched
    VFS-backed `/bin/stderr` executable, and the shell descriptor table is
    restored afterward. Retained QEMU/substitute evidence maps redirected fd2
    writes as `stream=stdout route=runtime-console0/stdout`, retained
    `exec stdout 1>&2` fd1 writes as
    `stream=stderr route=runtime-console0/stderr`, normal fd1 stdout writes
    as `stream=stdout route=runtime-console0/stdout`, and normal fd2 stderr
    writes as `stream=stderr route=runtime-console0/stderr`. The evidence
    map also retains scheduler-backed stdin wait/readiness, Ctrl-D EOF,
    descriptor-backed VFS exec, loader temporary descriptor non-leak,
    lifecycle/status, consuming `waitpid`, non-consuming `laststatus`,
    fixed `/bin` lookup, deterministic negative exec/redirection controls,
    and descriptor-backed `cat /etc/banner.txt`. Arbitrary `N>&M`,
    descriptor close/move syntax, regular-file redirection, append/truncate,
    pipes, writable filesystem behavior, separate physical sinks, terminal
    policy, async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and
    SSH remain deferred. The queued descriptor-dup redirection frontier
    closeout is mechanically unblocked, provided it stays bounded to the
    accepted exact descriptor-dup forms and does not claim file redirection,
    pipes, writable filesystem behavior, or a phase transition.
51. descriptor-dup redirection frontier closeout: accepted in
    `phase10-fd-dup-redirection-frontier-closeout-20260604`. This checkpoint
    accepts exactly the two shell-visible descriptor-duplication forms
    `exec stdout 1>&2` and `exec stderr 2>&1` for launched VFS-backed
    children. Both forms are child-only descriptor-table mutations: fd1 can be
    rebound to the inherited fd2 stderr route for `/bin/stdout`, and fd2 can
    be rebound to the inherited fd1 stdout route for `/bin/stderr`, while
    the shell descriptor table is restored afterward. Retained evidence maps
    both direction smokes, normal stdout/stderr route controls,
    scheduler-backed stdin wait/readiness, Ctrl-D EOF, descriptor-backed VFS
    exec, loader temporary descriptor non-leak, lifecycle/status, consuming
    `waitpid`, non-consuming `laststatus`, deterministic negative
    exec/redirection controls, and descriptor-backed `cat /etc/banner.txt`.
    The accepted physical sink remains shared runtime-console0; this frontier
    accepts stream/route metadata and descriptor-table behavior, not separate
    physical sinks. Arbitrary `N>&M`, descriptor close/move syntax,
    regular-file redirection, append/truncate, pipes, writable filesystem
    behavior, terminal policy, async jobs, fork, signals, libc stdio, Pi 5
    proof, networking, and SSH remain deferred. Supervisor planning is
    required before the next feature-led shell I/O task because descriptor
    close/restore syntax, minimal pipe producer-consumer lifecycle, and
    file/device redirection have different scope and dependency risks. If the
    supervisor continues inside descriptor policy, descriptor close/restore
    syntax is the smallest plausible next slice; file/device redirection needs
    an explicit writable target or device-sink plan.
52. stdout descriptor-close redirection core: accepted in
    `phase10-stdout-close-redirection-core-20260604`. Shell-visible
    `exec stdout 1>&-` now parses as the first exact descriptor-close
    redirection form. The launched VFS-backed `/bin/stdout` child still loads
    through the accepted descriptor-backed `/bin` lookup and VFS/open/read
    path, but child fd1 is closed before the userspace fixture attempts its
    `TalosWrite`. The task-owned QEMU/substitute evidence records
    `exec-descriptors ... inherited-count=2 fd1=closed`,
    `exec-redirection op=close source-fd=1 result=closed-descriptor
    child-only=true shell-restored=true`, and fd1 write return `-EBADF`
    with `stream=closed route=closed-descriptor`. A following normal
    `exec stdout` control records `stream=stdout
    route=runtime-console0/stdout`, proving the shell descriptor table is
    restored after the child launch. Accepted descriptor-dup directions,
    normal stdout/stderr route controls, terminal EOF/readiness, lifecycle,
    waitpid/laststatus, negative redirection, and descriptor-backed cat
    controls remain covered. Stderr `2>&-`, arbitrary `N>&-`, descriptor
    moves, regular-file redirection, append/truncate, pipes, writable
    filesystem behavior, separate physical sinks, terminal policy, async
    jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH remain
    deferred. The queued stdout descriptor-close closeout is mechanically
    unblocked.
53. stdout descriptor-close redirection closeout: accepted in
    'phase10-stdout-close-redirection-closeout-20260604'. This closeout
    checkpoints the accepted 'exec stdout 1>&-' frontier as one exact
    child-only descriptor-close form. Retained QEMU/substitute evidence maps
    closed child fd1, shell fd1 restoration, both descriptor-dup direction
    controls, normal stdout/stderr route controls, scheduler-backed stdin
    wait/readiness, Ctrl-D EOF, descriptor-backed VFS exec, lifecycle/status,
    waitpid/laststatus, deterministic negative exec/redirection controls, and
    descriptor-backed 'cat /etc/banner.txt'. The accepted observable close
    result remains the userspace stdout fixture's fd1 write '-EBADF' with
    'stream=closed route=closed-descriptor' after descriptor-backed VFS loading
    succeeds. Stderr '2>&-', arbitrary 'N>&-', descriptor moves, regular-file
    redirection, append/truncate, pipes, writable filesystem behavior,
    separate physical sinks, terminal policy, async jobs, fork, signals, libc
    stdio, Pi 5 proof, networking, and SSH remain deferred. The queued stderr
    descriptor-close core is mechanically unblocked as the inverse
    standard-stream close slice and must stay bounded to child-only
    'exec stderr 2>&-'.
54. stderr descriptor-close redirection core: accepted in
    `phase10-stderr-close-redirection-core-20260604`. Shell-visible
    `exec stderr 2>&-` now parses as the inverse exact descriptor-close
    redirection form. The launched VFS-backed `/bin/stderr` child still loads
    through the accepted descriptor-backed `/bin` lookup and VFS/open/read
    path, but child fd2 is closed before the userspace fixture attempts its
    `TalosWrite`. The task-owned QEMU/substitute evidence records
    `exec-descriptors ... inherited-count=2 fd2=closed`,
    `exec-redirection op=close source-fd=2 result=closed-descriptor
    child-only=true shell-restored=true`, and fd2 write return `-EBADF`
    with `stream=closed route=closed-descriptor`. A following normal
    `exec stderr` control records `stream=stderr
    route=runtime-console0/stderr`, proving shell fd2 restoration. The
    accepted `exec stdout 1>&-` close form, both descriptor-dup directions,
    normal stdout/stderr route controls, terminal EOF/readiness, lifecycle,
    waitpid/laststatus, negative redirection, and descriptor-backed cat
    controls remain covered. Arbitrary `N>&-`, descriptor moves,
    regular-file redirection, append/truncate, pipes, writable filesystem
    behavior, separate physical sinks, terminal policy, async jobs, fork,
    signals, libc stdio, Pi 5 proof, networking, and SSH remain deferred. The
    queued stderr descriptor-close closeout is mechanically unblocked.
55. stderr descriptor-close redirection closeout: accepted in
    'phase10-stderr-close-redirection-closeout-20260604'. This closeout
    checkpoints the accepted standard-stream descriptor-close frontier as two
    exact child-only forms: 'exec stdout 1>&-' and 'exec stderr 2>&-'.
    Retained QEMU/substitute evidence maps closed child fd1 and fd2 behavior,
    shell descriptor restoration, both descriptor-dup direction controls,
    normal stdout/stderr route controls, scheduler-backed stdin
    wait/readiness, Ctrl-D EOF, descriptor-backed VFS exec, lifecycle/status,
    waitpid/laststatus, deterministic negative exec/redirection controls, and
    descriptor-backed 'cat /etc/banner.txt'. The accepted observable close
    result remains the relevant userspace fixture's fd write '-EBADF' with
    'stream=closed route=closed-descriptor' after descriptor-backed VFS
    loading succeeds. Arbitrary 'N>&-' descriptor close syntax, descriptor
    moves, regular-file redirection, append/truncate, pipes, writable
    filesystem behavior, separate physical sinks, terminal policy, async jobs,
    fork, signals, libc stdio, Pi 5 proof, networking, and SSH remain
    deferred. The queued descriptor-close frontier closeout is mechanically
    unblocked and must remain docs/evidence reconciliation only.
56. descriptor-close redirection frontier closeout: accepted in
    'phase10-fd-close-redirection-frontier-closeout-20260604'. This closeout
    accepts exactly the two shell-visible descriptor-close forms
    'exec stdout 1>&-' and 'exec stderr 2>&-' for VFS-backed exec children.
    Both are child-only descriptor-table mutations: the launched child sees
    fd1 or fd2 closed, the relevant userspace fixture's fd write reports
    '-EBADF' with 'stream=closed route=closed-descriptor', and the shell
    descriptor table is restored after the launch. Retained QEMU/substitute
    evidence maps both descriptor-close directions, both descriptor-dup
    directions, normal stdout/stderr route controls, scheduler-backed stdin
    wait/readiness, Ctrl-D EOF, descriptor-backed VFS exec, lifecycle/status,
    waitpid/laststatus, deterministic negative exec/redirection controls, and
    descriptor-backed 'cat /etc/banner.txt'. Arbitrary descriptor close
    beyond the two exact forms, descriptor moves, broad descriptor
    close/restore syntax, regular-file redirection, append/truncate, pipes,
    writable filesystem behavior, separate physical sinks, terminal policy,
    async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH
    remain deferred. Supervisor planning is required before the next
    feature-led shell I/O task; do not infer arbitrary descriptor syntax,
    pipes, file/device redirection, writable filesystem behavior, networking,
    SSH, or a phase transition from this frontier closeout.
57. minimal stdout-to-stdin pipeline core: accepted in
    `phase10-minimal-stdout-to-stdin-pipe-core-20260604`.
    Shell-visible `exec stdout | exec stdin` now parses as the first exact
    two-stage pipeline form. The producer `/bin/stdout` and consumer
    `/bin/stdin` both launch through the accepted fixed `/bin`
    descriptor-backed VFS exec path. During the command, producer fd1 is a
    pipe writer and consumer fd0 is the matching pipe reader; the retained
    QEMU/substitute evidence records 31 bytes written and read, the consumer
    visible line `Talos userspace stdin fixture read: Talos userspace stdout
    fixture`, `writer-closed=true reader-eof=true shell-restored=true`,
    and `read-result=pipe-eof-after-writer-close`. The accepted lifecycle
    observation for this bounded form is the consumer record through
    `waitpid` and `laststatus`; the producer and consumer exec summaries
    remain printed in the pipeline transcript. Retained controls cover both
    descriptor-dup directions, both descriptor-close directions, normal
    userspace stdout/stdin, scheduler-backed stdin wait/readiness, unsupported
    pipe forms, deterministic bad-command behavior, and descriptor-backed
    `cat /etc/banner.txt`. Multi-stage pipelines, concurrent pipe
    scheduling, stdout-only stderr-not-piped proof, pipefail, background jobs,
    async execution, fork, signals, job control, file redirection, arbitrary
    descriptor syntax, writable filesystem behavior, Pi 5 proof, networking,
    and SSH remain deferred. The queued minimal pipeline closeout is
    mechanically unblocked and must remain docs/evidence reconciliation only.
58. minimal stdout-to-stdin pipeline closeout: accepted in
    'phase10-minimal-stdout-to-stdin-pipe-closeout-20260604'. This closeout
    checkpoints the accepted first pipeline frontier as exactly one
    shell-visible form: 'exec stdout | exec stdin'. Retained QEMU/substitute
    evidence maps producer fd1 as the pipe writer, consumer fd0 as the
    matching pipe reader, matching 31-byte write/read counts,
    writer-close EOF, shell descriptor restoration, producer and consumer
    lifecycle/status records, consumer 'waitpid' and 'laststatus',
    deterministic unsupported pipe/bad-command controls, descriptor
    redirection controls, normal stdio controls, stdin wait/readiness and EOF
    controls, and descriptor-backed 'cat /etc/banner.txt'. This frontier
    accepts descriptor ownership, deterministic byte transfer, close/EOF
    behavior, and consumer status observation for one exact two-stage form.
    It does not accept concurrent POSIX pipeline scheduling, multi-stage
    pipelines, pipefail, background jobs, async execution, fork, signals, job
    control, file redirection, arbitrary descriptor syntax, writable
    filesystem behavior, Pi 5 proof, networking, or SSH. The queued
    stdout-only stderr-not-piped core is mechanically unblocked as the next
    bounded pipe semantic and must keep the accepted minimal pipeline as its
    positive control.

59. pipeline stderr-not-piped core: accepted in
    `phase10-pipeline-stderr-not-piped-core-20260604`. Shell-visible
    `exec stderr | exec stdin` now proves stdout-only pipe semantics for the
    accepted exact two-stage pipeline grammar: producer fd1 is the pipe writer
    endpoint, while producer fd2 remains the inherited stderr descriptor and
    writes `stream=stderr route=runtime-console0/stderr`. The consumer
    `/bin/stdin` sees zero bytes from fd0 and records deterministic
    `read-result=pipe-eof/no-data`, distinct from terminal Ctrl-D EOF and
    runtime-console0 readiness/no-data. The task-owned QEMU/substitute smoke
    also retains the positive `exec stdout | exec stdin` control with 31 bytes
    written/read through the pipe and a descriptor-backed
    `cat /etc/banner.txt` control after both pipeline forms. Refreshed
    controls cover distinct stderr routing and both descriptor-dup directions.
    Unsupported descriptor-mixing forms such as `exec stderr 2>&1 | exec
    stdin` still fail deterministically. This does not accept `2>&1` inside
    pipelines, stderr piping by default, pipefail, multi-stage pipelines,
    concurrent POSIX pipeline scheduling, async execution, fork, signals, job
    control, file redirection, writable filesystem behavior, Pi 5 proof,
    networking, or SSH. The queued stderr-not-piped closeout is mechanically
    unblocked and must remain docs/evidence reconciliation only.
60. pipeline stderr-not-piped closeout: accepted in
    'phase10-pipeline-stderr-not-piped-closeout-20260604'. This closeout
    checkpoints stdout-only pipeline semantics as a durable local shell I/O
    boundary for the accepted exact two-stage pipeline grammar. The pipeline
    operator connects producer fd1/stdout only: 'exec stdout | exec stdin'
    transfers 31 bytes through the pipe, while 'exec stderr | exec stdin'
    leaves producer fd2 on 'stream=stderr route=runtime-console0/stderr' and
    the consumer observes zero pipe bytes with
    'read-result=pipe-eof/no-data'. Retained QEMU/substitute evidence maps
    normal stderr routing, shell descriptor restoration, both descriptor-dup
    and descriptor-close controls, stdin wait/readiness, Ctrl-D EOF,
    descriptor-backed VFS exec, lifecycle/status, consuming 'waitpid',
    'laststatus', deterministic unsupported descriptor-mixing pipeline forms,
    and descriptor-backed 'cat /etc/banner.txt'. This frontier accepts the
    stdout-only pipe rule, empty-pipe no-data behavior, shell descriptor
    restoration, and consumer lifecycle observation for the accepted exact
    two-stage forms. It does not accept '2>&1' inside pipelines, explicit
    stderr piping, multi-stage pipelines, concurrent POSIX pipeline
    scheduling, pipefail, async execution, fork, signals, job control, file
    redirection, arbitrary descriptor syntax, writable filesystem behavior,
    Pi 5 proof, networking, or SSH. The queued minimal pipeline frontier
    closeout is mechanically unblocked and must remain docs/evidence
    reconciliation only.
61. minimal pipeline frontier closeout: accepted in
    'phase10-minimal-pipeline-frontier-closeout-20260604'. This closeout
    reconciles the accepted exact two-stage pipeline frontier before broader
    pipe syntax, descriptor-mixing syntax, file/device redirection,
    process-control expansion, networking, or SSH. The accepted forms remain
    exactly 'exec stdout | exec stdin' and 'exec stderr | exec stdin'.
    The first transfers 31 bytes from producer fd1/stdout to consumer
    fd0/stdin; the second proves stdout-only pipe semantics by leaving
    producer fd2/stderr on 'stream=stderr route=runtime-console0/stderr'
    while the consumer sees zero pipe bytes with
    'read-result=pipe-eof/no-data'. Retained QEMU/substitute evidence maps
    shell descriptor restoration, descriptor-dup and descriptor-close
    controls, normal stdout/stderr route controls, stdin wait/readiness and
    Ctrl-D EOF controls, descriptor-backed VFS exec, lifecycle/status,
    consuming 'waitpid', non-consuming 'laststatus', deterministic negative
    controls, and descriptor-backed 'cat /etc/banner.txt'. This frontier does
    not accept '2>&1' inside pipelines, explicit stderr piping, multi-stage
    pipelines, concurrent POSIX pipeline scheduling, pipefail, async
    execution, fork, signals, job control, file/device redirection, arbitrary
    descriptor syntax, writable filesystem behavior, Pi 5 proof, networking,
    or SSH. Supervisor planning is required before the next feature-led shell
    I/O task because no explicit queued task remains. The strongest local
    recommendation is an explicit descriptor-mixing pipeline slice such as
    'exec stderr 2>&1 | exec stdin'; file/device redirection needs an explicit
    target/sink contract, and broader multi-stage pipelines need a separate
    scheduling/status plan.
62. pipeline stderr dup-to-stdout core: accepted in
    'phase10-pipeline-stderr-dup-to-stdout-core-20260604'. This checkpoint
    accepts exactly one descriptor-mixing pipeline form:
    'exec stderr 2>&1 | exec stdin'. The producer pipe endpoint is installed on
    fd1 first, then child-only '2>&1' duplicates that pipe-backed fd1 endpoint
    into producer fd2. The VFS-backed '/bin/stderr' fixture writes 31 bytes
    through fd2 as 'stream=pipe-writer route=pipe:stdout-to-stdin', the
    VFS-backed '/bin/stdin' consumer reads those stderr fixture bytes from
    inherited fd0, reports them through inherited fd1, observes
    writer-close EOF, and leaves shell fd0/fd1/fd2 restored after the command.
    Task-owned QEMU/substitute evidence records the mixed positive case,
    consumer 'waitpid', non-consuming 'laststatus', no leaked pipe endpoints,
    final classification
    'qemu-local-shell-pipeline-stderr-dup-to-stdout-complete', and PASS.
    Refreshed controls preserve plain 'exec stderr | exec stdin' as
    stdout-only with zero pipe bytes and 'pipe-eof/no-data', plain
    'exec stdout | exec stdin' as the 31-byte stdout transfer, both
    descriptor-dup directions, both descriptor-close directions, deterministic
    negative redirection controls, and descriptor-backed
    'cat /etc/banner.txt'. This does not accept the inverse
    'exec stdout 1>&2 | exec stdin', arbitrary 'N>&M', explicit stderr pipe
    syntax, file/device redirection, writable filesystem behavior,
    multi-stage/concurrent pipelines, pipefail, jobs, fork/signals, Pi 5
    proof, networking, or SSH. The queued closeout is mechanically unblocked
    and must remain docs/evidence reconciliation only.
63. pipeline stderr dup-to-stdout closeout: accepted in
    'phase10-pipeline-stderr-dup-to-stdout-closeout-20260604'. This closeout
    records the accepted descriptor-ordering boundary for exactly
    'exec stderr 2>&1 | exec stdin': producer fd1 becomes the pipe writer,
    then child-only '2>&1' duplicates that pipe-backed endpoint into producer
    fd2 before '/bin/stderr' writes. Retained QEMU/substitute evidence maps
    the 31-byte stderr fixture transfer into consumer fd0, stdout reporting
    through inherited fd1, writer-close EOF, shell fd0/fd1/fd2 restoration,
    consumer 'waitpid', consumer 'laststatus', no leaked pipe endpoints,
    final classification
    'qemu-local-shell-pipeline-stderr-dup-to-stdout-complete', errors=0, and
    PASS. The evidence map also retains plain 'exec stdout | exec stdin',
    plain 'exec stderr | exec stdin', descriptor-dup and descriptor-close
    controls, normal stdio/stderr routing, stdin readiness and Ctrl-D EOF,
    VFS exec, lifecycle/status, deterministic negative controls, and
    descriptor-backed 'cat /etc/banner.txt'. This checkpoint does not accept
    inverse 'exec stdout 1>&2 | exec stdin', arbitrary descriptor syntax,
    explicit stderr pipe syntax, file/device redirection, writable filesystem
    behavior, multi-stage/concurrent pipelines, pipefail, jobs, fork/signals,
    Pi 5 proof, networking, SSH, or a phase transition. The queued inverse
    redirection-away core is mechanically unblocked and must remain bounded to
    that exact command form.
64. pipeline stdout redirect-away core: accepted in
    `phase10-pipeline-stdout-redirect-away-core-20260604`. This checkpoint
    accepts exactly one inverse descriptor-mixing pipeline form:
    `exec stdout 1>&2 | exec stdin`. The producer pipe endpoint is installed
    on fd1 first, then child-only `1>&2` rebinds producer fd1 to inherited
    fd2/stderr before `/bin/stdout` writes. The stdout fixture writes 31 bytes
    through `stream=stderr route=runtime-console0/stderr`, no fixture bytes
    enter the pipe, and the VFS-backed `/bin/stdin` consumer reports
    `pipe-eof/no-data` through inherited fd1. Task-owned QEMU/substitute
    evidence records zero pipe bytes, shell fd0/fd1/fd2 restoration, consumer
    `waitpid`, consumer `laststatus`, final classification
    `qemu-local-shell-pipeline-stdout-redirect-away-complete`, errors=0, and
    PASS. Refreshed controls preserve `exec stderr 2>&1 | exec stdin`, plain
    `exec stdout | exec stdin`, plain `exec stderr | exec stdin`, both
    descriptor-dup directions, both descriptor-close directions, deterministic
    negative controls, and descriptor-backed `cat /etc/banner.txt`. This does
    not accept arbitrary `N>&M`, explicit stderr pipe syntax, file/device
    redirection, writable filesystem behavior, multi-stage/concurrent
    pipelines, pipefail, jobs, fork/signals, Pi 5 proof, networking, SSH, or a
    phase transition. The queued inverse closeout is mechanically unblocked
    and must remain docs/evidence reconciliation only.
65. pipeline stdout redirect-away closeout: accepted in
    `phase10-pipeline-stdout-redirect-away-closeout-20260604`. This closeout
    checkpoints both exact descriptor-mixing pipeline directions. For
    `exec stderr 2>&1 | exec stdin`, producer fd1 is installed as the pipe
    writer first and child-only `2>&1` duplicates that pipe endpoint onto
    producer fd2, so the stderr fixture enters consumer fd0. For
    `exec stdout 1>&2 | exec stdin`, producer fd1 is also installed as the
    pipe writer first, but child-only `1>&2` rebinds fd1 to inherited
    fd2/stderr before `/bin/stdout` writes, so no fixture bytes enter the
    pipe and the consumer reports `pipe-eof/no-data`. The evidence map
    retains both mixed forms, plain stdout pipeline transfer, stdout-only
    stderr-not-piped semantics, descriptor-dup/close controls, normal stdio,
    stdin wait/readiness/EOF controls, VFS exec, lifecycle/status, waitpid,
    laststatus, negative controls, and descriptor-backed `cat /etc/banner.txt`.
    This checkpoint does not accept arbitrary `N>&M`, explicit stderr pipe
    syntax, file/device redirection, writable filesystem behavior,
    multi-stage/concurrent pipelines, pipefail, jobs, fork/signals, Pi 5 proof,
    networking, SSH, or a phase transition. The queued descriptor-mixing
    frontier closeout is mechanically unblocked and must remain
    docs/evidence reconciliation only.
66. pipeline descriptor-mixing frontier closeout: accepted in
    'phase10-pipeline-descriptor-mixing-frontier-closeout-20260604'. This
    closeout checkpoints the accepted descriptor-mixing pipeline frontier as
    exactly four two-stage forms: 'exec stdout | exec stdin',
    'exec stderr | exec stdin', 'exec stderr 2>&1 | exec stdin', and
    'exec stdout 1>&2 | exec stdin'. Producer fd1 remains the default pipe
    writer; mixed forms install that pipe endpoint before applying the
    producer child-only descriptor operation. The accepted evidence map
    retains the positive stdout transfer, stdout-only stderr-not-piped
    no-data behavior, stderr-to-pipe '2>&1' behavior, stdout-redirection-away
    '1>&2' behavior, descriptor-dup and descriptor-close controls, normal
    stdio/stderr routing, stdin wait/readiness/EOF controls, VFS exec,
    lifecycle/status, waitpid, laststatus, negative controls, and
    descriptor-backed 'cat /etc/banner.txt'. This checkpoint does not accept
    arbitrary descriptor syntax, arbitrary 'N>&M', explicit stderr pipe
    syntax, file/device redirection, writable filesystem behavior,
    multi-stage/concurrent pipelines, pipefail, jobs, fork/signals, Pi 5
    proof, networking, SSH, or a phase transition. Supervisor planning is
    required before the next feature-led shell I/O task. The bounded
    recommendation is file/device redirection only with an explicit
    target/sink contract; multi-stage pipeline status/scheduling needs a
    separate process-accounting plan first, and descriptor syntax cleanup
    should wait until it directly supports an explicit feature slice.
67. /dev/null stdout redirection contract core: accepted in
    `phase10-dev-null-stdout-redirection-contract-core-20260604`. This
    checkpoint accepts exactly `exec stdout >/dev/null` as a child-only fd1
    sink redirection for the VFS-backed `/bin/stdout` fixture. The child
    descriptor table reports `fd1=device`; the redirection record reports
    `op=sink`, `target-path=/dev/null`, `target-stream=null-sink`, and
    `target-route=device:/dev/null`; and the userspace write reports the
    accepted byte count while routing to `stream=null-sink`. The
    task-owned QEMU/substitute smoke confirms that the redirected stdout
    fixture payload is absent from runtime-console0/stdout for the redirected
    command and that a following normal `exec stdout` prints through restored
    shell fd1. `/dev/null` is accepted only as an explicit sink device, not as
    writable filesystem behavior. `1>/dev/null`, regular-file redirection,
    append/truncate, input redirection, stderr-to-/dev/null, arbitrary
    descriptor syntax, broader file/device semantics, multi-stage pipelines,
    writable filesystem behavior, Pi 5 proof, networking, SSH, and a phase
    transition remain deferred. The queued stdout-to-/dev/null closeout is
    mechanically unblocked for docs/evidence reconciliation only.
68. /dev/null stdout redirection closeout: accepted in
    `phase10-dev-null-stdout-redirection-closeout-20260604`. This checkpoint
    reconciles the first explicit file/device redirection sink contract before
    extending it to stderr. The accepted behavior remains exactly
    `exec stdout >/dev/null`: the VFS-backed `/bin/stdout` child has fd1
    rebound to the `/dev/null` device sink, reports `fd1=device`,
    `op=sink`, `target-path=/dev/null`,
    `target-stream=null-sink`, `target-route=device:/dev/null`, and
    `exec-stdout ... stream=null-sink route=device:/dev/null`, while
    `TalosWrite` validates/copies and discards 31 bytes. The evidence map
    keeps the redirected stdout payload absent from runtime-console0/stdout
    for the redirected command, keeps the following normal `exec stdout`
    visible payload as shell fd1 restoration proof, and retains lifecycle,
    `waitpid`, `laststatus`, deterministic negative redirection forms,
    descriptor-dup/close controls, descriptor-mixing pipeline controls, stdin
    readiness/EOF controls, and descriptor-backed `cat /etc/banner.txt`.
    This closeout does not accept writable filesystem behavior, regular-file
    redirection, append/truncate, input redirection, stderr-to-/dev/null,
    arbitrary descriptor syntax, broader file/device semantics,
    multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, or a phase
    transition. The queued stderr-to-/dev/null core is mechanically unblocked
    and must remain bounded to the same explicit `/dev/null` sink contract.

69. /dev/null stderr redirection core: accepted in
    `phase10-dev-null-stderr-redirection-core-20260604`. This checkpoint
    accepts exactly `exec stderr 2>/dev/null` as the fd2 sibling of the
    accepted stdout sink contract. The VFS-backed `/bin/stderr` child has
    fd2 rebound to the `/dev/null` device sink, reports `fd2=device`,
    `op=sink`, `target-path=/dev/null`, `target-stream=null-sink`,
    `target-route=device:/dev/null`, and `exec-stderr ... stream=null-sink
    route=device:/dev/null`, while `TalosWrite` validates/copies and
    discards 31 bytes. The task-owned QEMU/substitute smoke confirms the
    redirected stderr fixture payload is absent from runtime-console0/stderr
    for the redirected command, then a following normal `exec stderr` proves
    shell fd2 restoration. The evidence map retains stdout-to-/dev/null,
    normal stderr, descriptor redirection, descriptor-mixing pipeline, stdin
    readiness/EOF, and descriptor-backed `cat /etc/banner.txt` controls.
    `exec stderr 2>file`, `exec stderr 2>>/dev/null`,
    `exec stderr </dev/null`, regular-file redirection, append/truncate, input
    redirection, arbitrary descriptor syntax, writable filesystem behavior,
    multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, and a
    phase transition remain deferred. The queued stderr-to-/dev/null closeout
    is mechanically unblocked for docs/evidence reconciliation only.

70. /dev/null stderr redirection closeout: accepted in
    `phase10-dev-null-stderr-redirection-closeout-20260604`. This checkpoint
    reconciles the accepted explicit `/dev/null` sink behavior for stdout
    and stderr before the broader file/device redirection
    frontier closeout. The accepted forms are exactly
    `exec stdout >/dev/null` and `exec stderr 2>/dev/null`: each launched
    VFS-backed child has only the target child descriptor rebound to the
    `/dev/null` device sink, reports `op=sink`,
    `target-path=/dev/null`, `target-stream=null-sink`,
    `target-route=device:/dev/null`, and returns the accepted 31-byte
    `TalosWrite` count while discarding the userspace fixture payload. The
    evidence map keeps redirected stdout/stderr payloads absent for the
    redirected commands, keeps following normal `exec stdout` and
    `exec stderr` controls as shell descriptor restoration proof, and
    retains lifecycle, `waitpid`, `laststatus`, deterministic negative
    redirection forms, descriptor redirection/pipeline controls, stdin
    readiness/EOF controls, and descriptor-backed `cat /etc/banner.txt`.
    This closeout does not accept regular-file redirection, append/truncate,
    input redirection, arbitrary descriptor syntax, writable filesystem
    behavior, broader file/device semantics, multi-stage/concurrent
    pipelines, Pi 5 proof, networking, SSH, or a phase transition. The queued
    /dev/null redirection frontier closeout is mechanically unblocked and
    must remain docs/evidence reconciliation only.
71. /dev/null redirection frontier closeout: accepted in
    'phase10-dev-null-redirection-frontier-closeout-20260604'. This checkpoint
    reconciles the accepted explicit '/dev/null' file/device redirection
    frontier. The accepted forms are exactly 'exec stdout >/dev/null' and
    'exec stderr 2>/dev/null'; '/dev/null' is accepted only as an output sink
    device, not writable filesystem behavior. Each form launches the
    VFS-backed child with only the target child descriptor rebound to the
    '/dev/null' device sink, records 'op=sink',
    'target-path=/dev/null', 'target-stream=null-sink',
    'target-route=device:/dev/null', routes the userspace write to
    'stream=null-sink route=device:/dev/null', and returns the accepted
    31-byte 'TalosWrite' count while discarding the fixture payload. The
    evidence map keeps redirected stdout/stderr payloads absent for the
    redirected commands, keeps following normal 'exec stdout' and
    'exec stderr' controls as shell descriptor restoration proof, and retains
    descriptor redirection controls, descriptor-mixing pipeline controls,
    normal stdio/stderr routing, stdin readiness/EOF controls, VFS exec,
    lifecycle/status, 'waitpid', 'laststatus', deterministic negatives, and
    descriptor-backed 'cat /etc/banner.txt'. This frontier does not accept
    shorthand '1>/dev/null', arbitrary descriptor syntax, regular-file
    redirection, append/truncate, input redirection, writable filesystem
    behavior, broader file/device semantics, multi-stage/concurrent
    pipelines, Pi 5 proof, networking, SSH, or a phase transition. Supervisor
    planning is required before the next feature-led shell I/O task; likely
    smaller next candidates are an explicit '/dev/null' input-redirection
    contract or a read-only VFS-backed regular-file redirection target, while
    append/truncate and writable regular-file output require a separate
    filesystem mutation plan.
72. /dev/null stdin redirection core: accepted in
    `phase10-dev-null-stdin-redirection-core-20260604`. This checkpoint
    accepts exactly `exec stdin </dev/null` as a child-only fd0 source
    redirection for the VFS-backed `/bin/stdin` fixture. The child descriptor
    table reports `fd0=device`; the redirection record reports
    `op=source`, `source-path=/dev/null`,
    `source-stream=null-source`, and
    `source-route=device:/dev/null`; and `TalosRead` from that descriptor
    returns zero bytes as true device-source EOF without polling
    runtime-console0 input. The stdin fixture reports the result through
    accepted stdout/status paths as
    `read-source=device:/dev/null` and
    `read-result=null-source-eof/no-data`. A following normal
    `exec stdin` control consumes `talos-console0` through the restored
    shell fd0, proving child-only restoration. Deterministic negative controls
    keep `exec stdout </dev/null`, `exec stdin </etc/banner.txt`, and
    `exec stdin < /dev/null` outside the accepted surface. This does not
    accept regular-file input redirection, output regular-file redirection,
    append/truncate, shorthand/broader descriptor syntax, writable filesystem
    behavior, broader file/device semantics, Pi 5 proof, networking, SSH, or a
    phase transition. The queued `/dev/null` stdin closeout is mechanically
    unblocked and must remain docs/evidence reconciliation only.
73. /dev/null stdin redirection closeout: accepted in
    'phase10-dev-null-stdin-redirection-closeout-20260604'. This checkpoint
    reconciles exact 'exec stdin </dev/null' as a child-only fd0 source
    redirection for the VFS-backed '/bin/stdin' fixture. The accepted record
    reports 'fd0=device', 'op=source', 'source-path=/dev/null',
    'source-stream=null-source', 'source-route=device:/dev/null', and
    'read-source=device:/dev/null'; 'TalosRead' returns zero bytes as true
    device-source EOF/no-data without polling runtime-console0. A following
    normal 'exec stdin' control consumes 'talos-console0' through the restored
    shell fd0, and deterministic negatives keep 'exec stdout </dev/null',
    'exec stdin </etc/banner.txt', and 'exec stdin < /dev/null' outside the
    accepted surface. The evidence map retains stdout/stderr '/dev/null'
    sinks, runtime-console0 stdin, stdin readiness/EOF, descriptor
    redirection and pipeline controls, VFS exec, lifecycle/status, waitpid,
    laststatus, and descriptor-backed cat. This does not accept regular-file
    input redirection, output regular-file redirection, append/truncate,
    shorthand/broader descriptor syntax, writable filesystem behavior,
    broader file/device semantics, Pi 5 proof, networking, SSH, or a phase
    transition. The queued /dev/null stdio frontier closeout is mechanically
    unblocked and must remain docs/evidence reconciliation only.
74. /dev/null stdio redirection frontier closeout: accepted in
    'phase10-dev-null-stdio-redirection-frontier-closeout-20260604'. This
    checkpoint reconciles the accepted explicit /dev/null standard-I/O
    frontier across fd0/fd1/fd2. The accepted forms are exactly
    'exec stdout >/dev/null', 'exec stderr 2>/dev/null', and
    'exec stdin </dev/null'. Stdout and stderr forms bind only the launched
    child fd1/fd2 descriptor to the /dev/null sink device, record 'op=sink',
    'target-path=/dev/null', 'target-stream=null-sink',
    'target-route=device:/dev/null', return the accepted 31-byte TalosWrite
    count, and discard the userspace fixture payload. The stdin form binds
    only the launched child fd0 descriptor to the /dev/null source device,
    records 'op=source', 'source-path=/dev/null',
    'source-stream=null-source', 'source-route=device:/dev/null', and returns
    a zero-byte true EOF/no-data TalosRead result without polling
    runtime-console0. Following normal 'exec stdout', 'exec stderr', and
    'exec stdin' controls prove shell descriptor restoration. The evidence
    map retains normal stdio/stderr routing, descriptor dup/close
    redirection, descriptor-mixing pipelines, stdin readiness and terminal
    EOF, VFS exec, lifecycle/status, waitpid, laststatus, deterministic
    negatives, and descriptor-backed cat. This does not accept regular-file
    input or output redirection, append/truncate, writable filesystem
    behavior, shorthand/broader descriptor syntax, broader file/device
    semantics, multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH,
    or a phase transition. The queued read-only regular-file stdin
    redirection core is mechanically unblocked and must stay bounded to a
    read-only input source through the accepted descriptor-backed initramfs/VFS
    path.
75. read-only regular-file stdin redirection core: accepted in
    `phase10-readonly-regular-file-stdin-redirection-core-20260604`. This
    checkpoint accepts exactly `exec stdin </etc/banner.txt` as a child-only
    fd0 source redirection for the VFS-backed `/bin/stdin` fixture. The shell
    closes fd0 for the child, opens `/etc/banner.txt` through the existing
    `TalosOpen` initramfs path, requires the resulting read-only regular-file
    descriptor to occupy fd0, and restores the original shell fd0 after the
    child exits. The local shell read-only file-description capacity is now two
    slots so redirected fd0 and the loader temporary descriptor can coexist.
    The accepted transcript reports `fd0=regular-file`, `op=source`,
    `source-path=/etc/banner.txt`, `source-stream=regular-file`,
    `source-route=initramfs:/etc/banner.txt`,
    `read-source=initramfs:/etc/banner.txt`, `bytes=0x18`, and
    `read-result=regular-file-eof-after-read`; the visible userspace output is
    the banner payload read through `TalosRead`, not a kernel-backed command
    shim. Following normal `exec stdin`, `/dev/null` stdin, unsupported stdout
    input-redirection, shorthand-negative, waitpid/laststatus, and
    descriptor-backed `cat /etc/banner.txt` controls are retained. Output
    regular-file redirection, append/truncate, writable filesystem mutation,
    arbitrary descriptor syntax, arbitrary path expansion, here-docs, broader
    pipes, Pi 5 proof, networking, SSH, and a phase transition remain deferred.
    The queued read-only regular-file stdin closeout is mechanically unblocked
    and must remain docs/evidence reconciliation only.
76. read-only regular-file stdin redirection closeout: accepted in
    'phase10-readonly-regular-file-stdin-redirection-closeout-20260604'. This
    checkpoint reconciles exact 'exec stdin </etc/banner.txt' as the first
    accepted regular-file fd0 source redirection for the VFS-backed
    '/bin/stdin' fixture. The accepted record reports 'fd0=regular-file',
    'op=source', 'source-path=/etc/banner.txt',
    'source-stream=regular-file',
    'source-route=initramfs:/etc/banner.txt',
    'read-source=initramfs:/etc/banner.txt', 'bytes=0x18', and
    'read-result=regular-file-eof-after-read'; the visible userspace output is
    the banner payload read through 'TalosRead'. A following normal
    'exec stdin' control proves restored shell fd0 by reading
    'talos-console0' from runtime-console0/local-input. The evidence map
    retains '/dev/null' stdin, runtime-console0 stdin readiness/EOF,
    userspace stdout/stderr, descriptor redirection and pipeline controls,
    VFS exec/open/read, lifecycle/status, waitpid, laststatus, deterministic
    negatives, and descriptor-backed 'cat /etc/banner.txt'. This does not
    accept output regular-file redirection, append/truncate, writable
    filesystem mutation, arbitrary descriptor syntax, arbitrary path
    expansion, here-docs, broader pipes, Pi 5 proof, networking, SSH, or a
    phase transition. The queued read-only input redirection frontier closeout
    is mechanically unblocked and must remain docs/evidence reconciliation
    only.
77. read-only input redirection frontier closeout: accepted in
    'phase10-readonly-input-redirection-frontier-closeout-20260604'. This
    checkpoint reconciles the accepted read-only input redirection forms:
    exactly 'exec stdin </dev/null' and 'exec stdin </etc/banner.txt' for the
    VFS-backed '/bin/stdin' fixture. The '/dev/null' form binds only the
    launched child fd0 descriptor to the device source, reports 'fd0=device',
    'op=source', 'source-path=/dev/null',
    'source-stream=null-source', 'source-route=device:/dev/null', and a
    zero-byte true EOF/no-data read result. The regular-file form binds only
    child fd0 to a read-only initramfs regular-file descriptor for
    '/etc/banner.txt', reports 'fd0=regular-file', 'op=source',
    'source-path=/etc/banner.txt', 'source-stream=regular-file',
    'source-route=initramfs:/etc/banner.txt',
    'read-source=initramfs:/etc/banner.txt', 'bytes=0x18', and
    'read-result=regular-file-eof-after-read'. Following normal 'exec stdin'
    controls prove restored shell fd0 through runtime-console0/local-input.
    The evidence map retains accepted stdout/stderr '/dev/null' sink
    redirection, normal stdio/stderr routing, descriptor dup/close
    redirection, descriptor-mixing pipelines, stdin readiness and terminal
    EOF, VFS exec/open/read, lifecycle/status, waitpid, laststatus,
    deterministic negatives, and descriptor-backed 'cat /etc/banner.txt'.
    This does not accept output regular-file redirection, append/truncate,
    writable filesystem mutation, arbitrary descriptor syntax, arbitrary path
    expansion, here-docs, broader pipes, Pi 5 proof, networking, SSH, or a
    phase transition. Supervisor planning is required before the next
    feature-led shell I/O task because likely follow-ups cross distinct risks:
    writable output targets, broader descriptor grammar, or lifecycle/process
    accounting.
78. stdout regular-file redirection core: accepted in
    'phase10-stdout-regular-file-redirection-core-20260604'. This accepts
    exactly 'exec stdout >/tmp/stdout.txt' for the VFS-backed '/bin/stdout'
    fixture. The launched child fd1 is rebound to a minimal volatile
    regular-file descriptor, reports 'fd1=regular-file', 'op=sink',
    'target-path=/tmp/stdout.txt', 'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stdout.txt', and the userspace stdout
    fixture writes 'bytes=0x1f' through 'source=userspace-talos-write'. A
    following descriptor-backed 'cat /tmp/stdout.txt' observes the captured
    'Talos userspace stdout fixture' bytes and records
    'cat path=/tmp/stdout.txt bytes=0x1f
    source=volatile-vfs-descriptor-read'; a later normal 'exec stdout' proves
    shell fd1 restoration through 'runtime-console0/stdout'. The evidence map
    retains /dev/null output sinks, read-only stdin redirection, normal stdio,
    descriptor dup/close redirection, descriptor-mixing pipelines, VFS
    exec/open/read, lifecycle/status, waitpid, laststatus, deterministic
    append/arbitrary-path/stderr-file negatives, and
    descriptor-backed 'cat /etc/banner.txt'. This does not accept stderr
    regular-file redirection, append/truncate variants beyond the exact
    create/truncate behavior, arbitrary paths, persistent storage, wider
    writable filesystem mutation, broader descriptor syntax, Pi 5 proof,
    networking, SSH, or a phase transition. The queued stdout regular-file
    redirection closeout is mechanically unblocked and must remain
    docs/evidence reconciliation only.
79. stdout regular-file redirection closeout: accepted in
    'phase10-stdout-regular-file-redirection-closeout-20260604'. This
    checkpoint reconciles the exact accepted output form:
    'exec stdout >/tmp/stdout.txt' for the VFS-backed '/bin/stdout' fixture.
    The child-only fd1 sink is a volatile VFS regular-file descriptor for
    '/tmp/stdout.txt', reports 'fd1=regular-file', 'op=sink',
    'target-path=/tmp/stdout.txt', 'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stdout.txt', and writes 0x1f
    userspace stdout bytes through 'source=userspace-talos-write'. A following
    descriptor-backed 'cat /tmp/stdout.txt' reads back
    'Talos userspace stdout fixture' with 'bytes=0x1f' and
    'source=volatile-vfs-descriptor-read', while a later normal
    'exec stdout' proves shell fd1 restoration through
    'runtime-console0/stdout'. The evidence map retains task-owned
    append/arbitrary-path/stderr-file negatives, /dev/null output sinks,
    read-only input redirection, normal stdio, descriptor dup/close
    redirection, descriptor-mixing pipelines, VFS exec/open/read,
    lifecycle/status, waitpid, laststatus, and descriptor-backed cat controls.
    This accepts only minimal volatile create/truncate/write/read behavior for
    '/tmp/stdout.txt'; stderr file redirection, append, partial overwrite,
    arbitrary paths, persistence, wider filesystem mutation, broader
    descriptor syntax, Pi 5 proof, networking, SSH, and phase transition
    remain deferred. The queued stderr regular-file redirection core is
    mechanically unblocked only as the exact already-planned stderr slice.
80. stderr regular-file redirection core: accepted in
    'phase10-stderr-regular-file-redirection-core-20260604'. This accepts
    exactly 'exec stderr 2>/tmp/stderr.txt' for the VFS-backed '/bin/stderr'
    fixture. The launched child fd2 is rebound to a minimal volatile
    regular-file descriptor, reports 'fd2=regular-file', 'op=sink',
    'target-path=/tmp/stderr.txt', 'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stderr.txt', and the userspace stderr
    fixture writes 'bytes=0x1f' through 'source=userspace-talos-write'. A
    following descriptor-backed 'cat /tmp/stderr.txt' observes the captured
    'Talos userspace stderr fixture' bytes and records
    'cat path=/tmp/stderr.txt bytes=0x1f
    source=volatile-vfs-descriptor-read'; a later normal 'exec stderr' proves
    shell fd2 restoration through 'runtime-console0/stderr', and a normal
    'exec stdout' remains visible through 'runtime-console0/stdout'. The
    evidence map retains stdout regular-file redirection, /dev/null output
    sinks, normal stdio, descriptor dup redirection, descriptor-mixing
    pipelines, VFS exec/open/read, lifecycle/status, waitpid, laststatus,
    deterministic append/arbitrary-path/stdout-file negatives, and
    descriptor-backed 'cat /etc/banner.txt'. This does not accept append,
    arbitrary paths, persistent storage, wider writable filesystem mutation,
    broader descriptor syntax, Pi 5 proof, networking, SSH, or a phase
    transition. The queued stderr regular-file redirection closeout is
    mechanically unblocked and must remain docs/evidence reconciliation only.
81. stderr regular-file redirection closeout: accepted in
    'phase10-stderr-regular-file-redirection-closeout-20260604'. This
    checkpoint reconciles the exact accepted output form:
    'exec stderr 2>/tmp/stderr.txt' for the VFS-backed '/bin/stderr' fixture.
    The child-only fd2 sink is a volatile VFS regular-file descriptor for
    '/tmp/stderr.txt', reports 'fd2=regular-file', 'op=sink',
    'target-path=/tmp/stderr.txt', 'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stderr.txt', and writes 0x1f
    userspace stderr bytes through 'source=userspace-talos-write'. A following
    descriptor-backed 'cat /tmp/stderr.txt' reads back
    'Talos userspace stderr fixture' with 'bytes=0x1f' and
    'source=volatile-vfs-descriptor-read', while a later normal
    'exec stderr' proves shell fd2 restoration through
    'runtime-console0/stderr'. A normal 'exec stdout' remains visible through
    'runtime-console0/stdout', proving the stderr file sink does not capture
    stdout. The evidence map retains stdout regular-file redirection,
    task-owned append/arbitrary-path/stdout-file negatives, /dev/null output
    sinks, read-only input redirection, normal stdio, descriptor dup/close
    redirection, descriptor-mixing pipelines, VFS exec/open/read/write,
    lifecycle/status, waitpid, laststatus, and descriptor-backed cat controls.
    This accepts only minimal volatile create/truncate/write/read behavior for
    '/tmp/stderr.txt'; append, partial overwrite, arbitrary paths,
    persistence, wider filesystem mutation, broader descriptor syntax, Pi 5
    proof, networking, SSH, and phase transition remain deferred. The queued
    regular-file output redirection frontier closeout is mechanically
    unblocked only as docs/evidence reconciliation across the accepted stdout
    and stderr regular-file slices.
82. regular-file output redirection frontier closeout: accepted in
    'phase10-regular-file-output-redirection-frontier-closeout-20260604'.
    This checkpoint reconciles the exact accepted output regular-file forms:
    'exec stdout >/tmp/stdout.txt' for '/bin/stdout' and
    'exec stderr 2>/tmp/stderr.txt' for '/bin/stderr'. The accepted scratch
    paths are only '/tmp/stdout.txt' and '/tmp/stderr.txt'. Each launched child
    has only the target descriptor rebound to a volatile VFS regular-file
    descriptor, writes 0x1f fixture bytes through
    'source=userspace-talos-write', and is read back through descriptor-backed
    'cat' with 'source=volatile-vfs-descriptor-read'. Later normal
    'exec stdout' and 'exec stderr' controls prove shell fd1/fd2 restoration,
    and the stderr evidence retains a normal stdout control proving distinct
    stream behavior. The evidence map retains read-only input redirection,
    /dev/null output sinks, normal stdio, descriptor dup/close redirection,
    descriptor-mixing pipelines, VFS exec/open/read/write, lifecycle/status,
    waitpid, laststatus, deterministic append/arbitrary-path/cross-file
    negatives, and descriptor-backed cat controls. This accepts only minimal
    volatile create/truncate/write/read behavior for the two exact scratch
    files; append, partial overwrite, arbitrary output paths, persistence,
    broad writable filesystem mutation, arbitrary descriptor syntax, Pi 5
    proof, networking, SSH, and phase transition remain deferred. No further
    queued task is mechanically unblocked by this closeout; supervisor
    planning is required before choosing between append behavior, broader
    descriptor grammar, or process accounting/concurrency.
83. stdout regular-file append redirection core: accepted in
    'phase10-stdout-regular-file-append-redirection-core-20260604'. This
    accepts exactly the bounded sequence 'exec stdout >/tmp/stdout.txt'
    followed by 'exec stdout >>/tmp/stdout.txt' for the VFS-backed
    '/bin/stdout' fixture. The first command preserves the accepted volatile
    create/truncate behavior; the append command rebinds child fd1 to the same
    volatile regular-file descriptor without truncating existing contents and
    records 'op=append', 'target-path=/tmp/stdout.txt',
    'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stdout.txt',
    'source=shell-redirection-stdout-tmp-stdout-append', and userspace
    'TalosWrite' provenance. A following descriptor-backed
    'cat /tmp/stdout.txt' reads two stdout fixture payloads in order with
    'bytes=0x3e source=volatile-vfs-descriptor-read'; later normal
    'exec stdout' proves shell fd1 restoration through
    'runtime-console0/stdout'. Append to missing files, arbitrary append
    paths, stderr append, persistence, partial overwrite, broad writable
    filesystem mutation, broader descriptor grammar, Pi 5 proof, networking,
    SSH, and phase transition remain deferred. The queued stdout append
    closeout is mechanically unblocked and must remain docs/evidence
    reconciliation only.
84. stdout regular-file append redirection closeout: accepted in
    'phase10-stdout-regular-file-append-redirection-closeout-20260604'. It
    reconciles the bounded append sequence 'exec stdout >/tmp/stdout.txt'
    followed by 'exec stdout >>/tmp/stdout.txt' for the VFS-backed
    '/bin/stdout' fixture, with required setup/truncate-create before append,
    scratch path limited to '/tmp/stdout.txt', child-only fd1 regular-file
    rebinding, 'op=append',
    'target-route=volatile-vfs:/tmp/stdout.txt', userspace TalosWrite
    provenance, descriptor-backed 'cat /tmp/stdout.txt' readback of two
    stdout fixture payloads with 'bytes=0x3e
    source=volatile-vfs-descriptor-read', shell fd1 restoration through a
    later normal 'exec stdout', lifecycle/status, waitpid, laststatus, and
    deterministic negative controls. The closeout retains stdout
    truncate/create, stderr regular-file, read-only input redirection,
    /dev/null redirection, normal stdio, descriptor redirection/pipeline,
    VFS exec/open/read/write, and descriptor-backed cat controls. Stderr
    append, append-create for missing files, arbitrary append paths,
    persistence, broad writable filesystem mutation, broader descriptor
    grammar, process accounting/concurrency, Pi 5 proof, networking, SSH, and
    phase transition remain deferred. The queued stderr append core is
    mechanically unblocked and must stay limited to the explicit
    '/tmp/stderr.txt' mirror.
85. stderr regular-file append redirection core: accepted in
    'phase10-stderr-regular-file-append-redirection-core-20260604'. This
    accepts exactly the bounded sequence 'exec stderr 2>/tmp/stderr.txt'
    followed by 'exec stderr 2>>/tmp/stderr.txt' for the VFS-backed
    '/bin/stderr' fixture. The first command preserves the accepted volatile
    create/truncate behavior; the append command rebinds child fd2 to the same
    volatile regular-file descriptor without truncating existing contents and
    records 'op=append', 'target-path=/tmp/stderr.txt',
    'target-stream=regular-file',
    'target-route=volatile-vfs:/tmp/stderr.txt',
    'source=shell-redirection-stderr-tmp-stderr-append', and userspace
    'TalosWrite' provenance. A following descriptor-backed
    'cat /tmp/stderr.txt' reads two stderr fixture payloads in order with
    'bytes=0x3e source=volatile-vfs-descriptor-read'; later normal
    'exec stderr' proves shell fd2 restoration through
    'runtime-console0/stderr' and normal 'exec stdout' proves stdout remains
    distinct through 'runtime-console0/stdout'. Append-create for missing
    files, arbitrary append paths, stdout-to-stderr path mixups, persistence,
    partial overwrite, broad writable filesystem mutation, broader descriptor
    grammar, process accounting/concurrency, Pi 5 proof, networking, SSH, and
    phase transition remain deferred. The queued stderr append closeout is
    mechanically unblocked and must remain docs/evidence reconciliation only.
86. stderr regular-file append redirection closeout: accepted in
    'phase10-stderr-regular-file-append-redirection-closeout-20260604'. It
    reconciles the bounded append sequence 'exec stderr 2>/tmp/stderr.txt'
    followed by 'exec stderr 2>>/tmp/stderr.txt' for the VFS-backed
    '/bin/stderr' fixture, with required setup/truncate-create before append,
    scratch path limited to '/tmp/stderr.txt', child-only fd2 regular-file
    rebinding, 'op=append',
    'target-route=volatile-vfs:/tmp/stderr.txt', userspace TalosWrite
    provenance, descriptor-backed 'cat /tmp/stderr.txt' readback of two
    stderr fixture payloads with 'bytes=0x3e
    source=volatile-vfs-descriptor-read', shell fd2 restoration through a
    later normal 'exec stderr', distinct stdout behavior through a later
    normal 'exec stdout', lifecycle/status, waitpid, laststatus, and
    deterministic negative controls. The closeout retains stdout
    append/truncate, stderr truncate/create, read-only input redirection,
    /dev/null redirection, normal stdio, descriptor redirection/pipeline,
    VFS exec/open/read/write, and descriptor-backed cat controls. Append-create
    for missing files, arbitrary append paths, stdout-to-stderr and
    stderr-to-stdout append mixups, persistence, broad writable filesystem
    mutation, broader descriptor grammar, process accounting/concurrency,
    Pi 5 proof, networking, SSH, and phase transition remain deferred. The
    queued regular-file append frontier closeout is mechanically unblocked and
    must stay docs/evidence reconciliation only before supervisor planning
    chooses any broader local execution primitive.
87. regular-file append redirection frontier closeout: accepted in
    'phase10-regular-file-append-redirection-frontier-closeout-20260604'. It
    reconciles the accepted append sequences 'exec stdout >/tmp/stdout.txt'
    followed by 'exec stdout >>/tmp/stdout.txt' and
    'exec stderr 2>/tmp/stderr.txt' followed by
    'exec stderr 2>>/tmp/stderr.txt'. Append is accepted only after the prior
    setup/truncate-create command in retained evidence, only for scratch paths
    '/tmp/stdout.txt' and '/tmp/stderr.txt', and only for the VFS-backed
    '/bin/stdout' and '/bin/stderr' fixtures. The accepted evidence records
    child-only fd1/fd2 regular-file rebinding, 'op=append',
    'target-route=volatile-vfs:/tmp/stdout.txt' or
    'target-route=volatile-vfs:/tmp/stderr.txt', userspace TalosWrite
    provenance, descriptor-backed 'cat' readback of two fixture payloads per
    stream with 'bytes=0x3e source=volatile-vfs-descriptor-read', shell
    fd1/fd2 restoration, distinct stdout behavior after stderr append,
    lifecycle/status, waitpid, laststatus, and deterministic negative
    controls. The closeout retains stdout/stderr truncate-create, read-only
    input redirection, /dev/null redirection, normal stdio, descriptor
    redirection/pipeline controls, VFS exec/open/read/write lineage, and
    descriptor-backed cat evidence. Append-create for missing files, arbitrary
    append paths, stdout-to-stderr and stderr-to-stdout append mixups beyond
    accepted negatives, persistence, broad writable filesystem mutation,
    arbitrary descriptor syntax, descriptor moves, process
    accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition
    remain deferred. No explicit queued follow-up remains mechanically
    unblocked; supervisor planning is required before any broader descriptor
    grammar, append-create, arbitrary-path, process-management, or
    phase-transition work.
88. regular-file append-create redirection core: accepted in
    'phase10-regular-file-append-create-redirection-core-20260604'. This
    accepts exactly 'exec stdout >>/tmp/stdout.txt' and exactly
    'exec stderr 2>>/tmp/stderr.txt' as missing-file append-create forms with
    no prior setup/truncate command in the task-owned transcripts. The stdout
    form binds child fd1 to the volatile '/tmp/stdout.txt' regular-file
    descriptor, records 'op=append',
    'target-route=volatile-vfs:/tmp/stdout.txt', writes one VFS-backed
    '/bin/stdout' fixture payload through userspace TalosWrite, reads it back
    through descriptor-backed 'cat /tmp/stdout.txt' with 'bytes=0x1f
    source=volatile-vfs-descriptor-read', and restores normal fd1 stdout. The
    stderr form mirrors this for child fd2, '/tmp/stderr.txt', the
    VFS-backed '/bin/stderr' fixture, descriptor-backed
    'cat /tmp/stderr.txt', normal fd2 stderr restoration, and a normal stdout
    distinct-stream control. Existing setup-then-append still appends without
    truncating existing scratch-file contents. Arbitrary append paths,
    stdout/stderr scratch path mixups, persistent storage, broad writable
    filesystem mutation, arbitrary descriptor syntax, descriptor moves,
    process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
    transition remain deferred. The queued append-create closeout is
    mechanically unblocked and must remain docs/evidence reconciliation only
    before explicit fd1 grammar can be promoted.
89. regular-file append-create redirection closeout: accepted in
    'phase10-regular-file-append-create-redirection-closeout-20260604'. It
    reconciles the accepted missing-file append-create forms
    'exec stdout >>/tmp/stdout.txt' and
    'exec stderr 2>>/tmp/stderr.txt' with no prior setup/truncate command in
    the task-owned transcripts. The accepted scratch paths remain only
    '/tmp/stdout.txt' and '/tmp/stderr.txt', with VFS-backed '/bin/stdout' and
    '/bin/stderr' fixtures, child-only fd1/fd2 regular-file rebinding,
    create-if-missing behavior for missing volatile scratch files,
    append-without-truncate behavior for existing scratch files, userspace
    TalosWrite provenance, descriptor-backed 'cat' readback of one fixture
    payload per transcript with 'bytes=0x1f
    source=volatile-vfs-descriptor-read', shell fd1/fd2 restoration,
    lifecycle/status, waitpid, laststatus, and deterministic negatives. The
    closeout retains setup-then-append, truncate/create output redirection,
    read-only input redirection, /dev/null redirection, normal stdio,
    descriptor redirection/pipeline controls, VFS exec/open/read/write
    lineage, and descriptor-backed cat evidence. Arbitrary output paths,
    arbitrary descriptor syntax beyond accepted exact fd1/fd2 forms,
    descriptor moves, broad writable filesystem mutation, process
    accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition
    remain deferred. The queued explicit fd1 regular-file redirection core is
    mechanically unblocked and must stay limited to exact '1>/tmp/stdout.txt'
    and '1>>/tmp/stdout.txt' aliases for the VFS-backed '/bin/stdout'
    fixture.
90. explicit fd1 regular-file redirection core: accepted in
    'phase10-explicit-fd1-regular-file-redirection-core-20260604'. This
    accepts exactly 'exec stdout 1>/tmp/stdout.txt' and exactly
    'exec stdout 1>>/tmp/stdout.txt' as fd1 aliases for the accepted volatile
    '/tmp/stdout.txt' VFS descriptor path. The truncate alias records
    'op=sink', 'source-fd=0x1', 'target-path=/tmp/stdout.txt',
    'target-route=volatile-vfs:/tmp/stdout.txt', userspace TalosWrite
    provenance, descriptor-backed readback with 'bytes=0x1f
    source=volatile-vfs-descriptor-read', waitpid, laststatus, and shell fd1
    restoration. The append alias records 'op=append' through the same fd1
    descriptor path and reads back two fixture payloads with 'bytes=0x3e
    source=volatile-vfs-descriptor-read'. Unsupported explicit fd numbers and
    arbitrary output paths remain deterministic negatives. These forms are
    grammar aliases for existing fd1 regular-file sink/append behavior, not a
    new descriptor class. Arbitrary descriptors, arbitrary output paths, fd2
    alias expansion beyond accepted exact stderr forms, descriptor moves,
    persistent storage, broad writable filesystem mutation, process
    accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition
    remain deferred. The queued explicit fd1 closeout is mechanically
    unblocked and must stay docs/evidence reconciliation only.
91. explicit fd1 regular-file redirection closeout: accepted in
    'phase10-explicit-fd1-regular-file-redirection-closeout-20260604'. It
    checkpoints the exact explicit fd1 aliases
    'exec stdout 1>/tmp/stdout.txt' and
    'exec stdout 1>>/tmp/stdout.txt' as grammar spellings for the accepted
    volatile fd1 regular-file descriptor path. The truncate alias records
    'op=sink', 'source-fd=0x1', 'target-path=/tmp/stdout.txt',
    'target-route=volatile-vfs:/tmp/stdout.txt', userspace TalosWrite
    provenance, descriptor-backed readback with 'bytes=0x1f
    source=volatile-vfs-descriptor-read', waitpid, laststatus, and shell fd1
    restoration. The append alias records 'op=append' through the same fd1
    descriptor path and reads back two fixture payloads with 'bytes=0x3e
    source=volatile-vfs-descriptor-read'. The closeout retains implicit
    stdout/stderr truncate and append/create redirection, read-only input
    redirection, /dev/null stdio redirection, normal stdio, descriptor
    redirection/pipeline controls, VFS exec/open/read/write lineage,
    lifecycle/status, waitpid, laststatus, deterministic negatives, and
    descriptor-backed cat evidence. Arbitrary descriptor syntax, descriptor
    moves, arbitrary output paths, fd2 shorthand aliases beyond accepted exact
    stderr forms, persistent storage, broad writable filesystem mutation,
    process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
    transition remain deferred. No explicit queued follow-up remains
    mechanically unblocked; supervisor planning is required before any broader
    descriptor grammar, arbitrary-path, process-management, persistent
    filesystem, hardware-proof, networking/SSH, or phase-transition work.
92. stdout arbitrary /tmp output redirection core: accepted in
    'phase10-stdout-arbitrary-tmp-output-redirection-core-20260604'. This
    extends stdout regular-file redirection from exact '/tmp/stdout.txt' to
    conservative volatile `/tmp/<basename>` output paths for the VFS-backed
    '/bin/stdout' fixture. Accepted forms are
    `exec stdout >/tmp/<basename>`,
    `exec stdout >>/tmp/<basename>`,
    `exec stdout 1>/tmp/<basename>`, and
    `exec stdout 1>>/tmp/<basename>` where the basename is non-empty ASCII
    letters, digits, '.', '_', or '-'. The retained QEMU/substitute transcript
    proves truncate/create through '/tmp/alpha.log', append/create through
    '/tmp/beta.out', explicit fd1 truncate through '/tmp/gamma.log',
    explicit fd1 append through '/tmp/delta.out', userspace TalosWrite
    provenance, `target-route=volatile-vfs:/tmp/<basename>`,
    descriptor-backed cat readbacks, waitpid, laststatus, and shell fd1
    restoration. Deterministic negatives reject outside-/tmp paths,
    nested/traversal paths, empty basenames, unsupported fd numbers, and the
    reserved stderr scratch name '/tmp/stderr.txt'. Stderr arbitrary paths,
    input arbitrary paths, persistent storage, broad writable filesystem
    mutation, recursive directories, descriptor moves, process
    accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition
    remain deferred. The queued stdout arbitrary-/tmp closeout is mechanically
    unblocked and must stay docs/evidence reconciliation only.
93. stdout arbitrary /tmp output redirection closeout: accepted in
    'phase10-stdout-arbitrary-tmp-output-redirection-closeout-20260604'. This
    reconciles the accepted stdout volatile '/tmp/BASENAME' output path
    frontier without adding code. The accepted forms remain
    'exec stdout >/tmp/BASENAME',
    'exec stdout >>/tmp/BASENAME',
    'exec stdout 1>/tmp/BASENAME', and
    'exec stdout 1>>/tmp/BASENAME' where the basename is non-empty ASCII
    letters, digits, '.', '_', or '-'. The evidence map retains the
    task-owned QEMU/substitute stdout arbitrary-/tmp transcript for
    truncate/create, append/create, explicit fd1 aliases, userspace TalosWrite
    provenance, volatile VFS target routes, descriptor-backed readbacks,
    waitpid, laststatus, shell fd1 restoration, deterministic path/fd
    negatives, errors=0, and PASS. The closeout also records the existing ADR
    as the no-new-policy rationale for reusing the conservative output-path
    grammar. Stderr arbitrary paths, input arbitrary paths, persistence,
    recursive directories, path traversal, broad writable filesystem mutation,
    arbitrary descriptor syntax beyond accepted forms, descriptor moves,
    process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
    transition remain deferred. The queued stderr arbitrary-/tmp core is
    mechanically unblocked and must stay bounded to stderr '2>' and '2>>'
    output redirection using this same policy.
94. stderr arbitrary /tmp output redirection core: accepted in
    'phase10-stderr-arbitrary-tmp-output-redirection-core-20260604'. This
    extends stderr regular-file redirection from exact '/tmp/stderr.txt' to
    conservative volatile '/tmp/BASENAME' output paths for the VFS-backed
    '/bin/stderr' fixture. Accepted forms are
    'exec stderr 2>/tmp/BASENAME' and
    'exec stderr 2>>/tmp/BASENAME' where the basename is non-empty ASCII
    letters, digits, '.', '_', or '-'. The retained QEMU/substitute transcript
    proves truncate/create through '/tmp/omega.err', append/create through
    '/tmp/theta.log', userspace TalosWrite provenance,
    'target-route=volatile-vfs:/tmp/BASENAME', descriptor-backed cat readbacks,
    waitpid, laststatus, and shell fd2 restoration. A normal 'exec stdout'
    control remains routed to 'runtime-console0/stdout'. Deterministic negatives
    reject outside-/tmp paths, nested/traversal paths, empty basenames,
    unsupported fd numbers, the reserved stdout scratch name '/tmp/stdout.txt',
    and fd2 shorthand output redirection without an explicit '2>'. Input
    arbitrary paths, persistence, recursive directories, broad writable
    filesystem mutation, arbitrary descriptor syntax beyond accepted forms,
    descriptor moves, process accounting/concurrency, Pi 5 proof, networking,
    SSH, and phase transition remain deferred. The queued stderr arbitrary-/tmp
    closeout is mechanically unblocked and must stay docs/evidence
    reconciliation only.
95. stderr arbitrary /tmp output redirection closeout: accepted in
    'phase10-stderr-arbitrary-tmp-output-redirection-closeout-20260604'. This
    reconciles the accepted stderr volatile '/tmp/BASENAME' output path
    frontier without adding code. The accepted forms remain
    'exec stderr 2>/tmp/BASENAME' and
    'exec stderr 2>>/tmp/BASENAME' where the basename is non-empty ASCII
    letters, digits, '.', '_', or '-'. The evidence map retains the
    task-owned QEMU/substitute stderr arbitrary-/tmp transcript for
    truncate/create, append/create, userspace TalosWrite provenance, volatile
    VFS target routes, descriptor-backed readbacks, waitpid, laststatus, shell
    fd2 restoration, distinct stdout behavior, deterministic path/fd/shorthand
    negatives, errors=0, and PASS. This closeout reuses the accepted
    conservative output-path grammar and records no new policy change. Input
    arbitrary paths, persistence, recursive directories, path traversal, broad
    writable filesystem mutation, arbitrary descriptor syntax beyond accepted
    fd1/fd2 forms, descriptor moves, process accounting/concurrency, Pi 5
    proof, networking, SSH, and phase transition remain deferred. The queued
    arbitrary '/tmp' output redirection frontier closeout is mechanically
    unblocked and must stay docs/evidence reconciliation only before requiring
    supervisor planning for any broader capability.
96. arbitrary /tmp output redirection frontier closeout: accepted in
    'phase10-arbitrary-tmp-output-redirection-frontier-closeout-20260604'.
    This closeout reconciles the accepted stdout and stderr volatile
    '/tmp/BASENAME' output path frontier without adding code. The accepted
    stdout forms remain 'exec stdout >/tmp/BASENAME',
    'exec stdout >>/tmp/BASENAME', 'exec stdout 1>/tmp/BASENAME', and
    'exec stdout 1>>/tmp/BASENAME'; the accepted stderr forms remain
    'exec stderr 2>/tmp/BASENAME' and
    'exec stderr 2>>/tmp/BASENAME'. The basename grammar remains non-empty
    ASCII letters, digits, '.', '_', or '-' under '/tmp/', and the accepted
    route remains volatile VFS only. The consolidated evidence map retains the
    task-owned stdout and stderr arbitrary-/tmp QEMU/substitute transcripts for
    truncate/create, append/create, explicit fd1 aliases, userspace TalosWrite
    provenance, volatile VFS target routes, descriptor-backed readbacks,
    fd1/fd2 restoration, distinct stdout/stderr behavior, waitpid,
    laststatus, path-policy negatives, errors=0, and PASS. Retained controls
    cover exact stdout/stderr output redirection, append/create, explicit fd1,
    /dev/null, read-only stdin redirection, normal stdio, descriptor
    redirection/pipeline behavior, VFS exec/open/read/write, lifecycle/status,
    and descriptor-backed cat. Arbitrary input paths, persistence, recursive
    directories, path traversal, broad writable filesystem mutation,
    arbitrary descriptor syntax beyond accepted fd1/fd2 output forms,
    descriptor moves, process accounting/concurrency, Pi 5 proof, networking,
    SSH, and phase transition remain deferred. No explicit queued follow-up is
    mechanically unblocked; supervisor planning is required before any broader
    capability is promoted.
97. combined stdin/stdout redirection core: accepted in
    'phase10-combined-stdin-stdout-redirection-core-20260605'.
    Shell-visible 'exec stdin </etc/banner.txt >/tmp/stdin-report.txt' now
    composes two child-only descriptor mutations in one VFS-backed userspace
    launch: fd0 is rebound to the accepted read-only initramfs
    '/etc/banner.txt' source and fd1 is rebound to the accepted volatile VFS
    '/tmp/stdin-report.txt' sink. The task-owned QEMU/substitute evidence
    records 'fd0=regular-file fd1=regular-file fd2=stdio-output', separate
    exec-redirection records for fd0 and fd1, the userspace stdin fixture
    reading 'Talos initramfs fixture', writing its report to redirected fd1,
    descriptor-backed 'cat /tmp/stdin-report.txt' readback, consuming
    'waitpid', non-consuming 'laststatus', errors=0, and PASS. The accepted
    serial command required raising the canonical line capacity from 32 to 64
    bytes; parser policy remains conservative and the retained negatives
    reject output-first ordering, combined '/dev/null' input, explicit fd1
    aliasing in the combined form, and spaced input grammar. Arbitrary input
    paths, '/dev/null' combined input, append/stderr combined forms, broader
    descriptor grammar, descriptor moves, multi-command redirection,
    persistence, recursive directories, process accounting/concurrency, Pi 5
    proof, networking, SSH, and phase transition remain deferred. The queued
    combined-redirection closeout is mechanically unblocked and must stay
    docs/evidence reconciliation only.
98. combined stdin/stdout redirection closeout: accepted in
    'phase10-combined-stdin-stdout-redirection-closeout-20260605'. This
    closeout checkpoints the exact single-command combined-redirection
    frontier without adding code. The accepted form remains
    'exec stdin </etc/banner.txt >/tmp/stdin-report.txt': fd0 is rebound to
    the accepted read-only initramfs '/etc/banner.txt' source, fd1 is rebound
    to the accepted volatile VFS '/tmp/stdin-report.txt' sink, and one
    VFS-backed userspace '/bin/stdin' process observes both child-only
    descriptor mutations. The consolidated evidence map retains the primary
    QEMU/substitute transcript with separate fd0/fd1 exec-redirection records,
    'fd0=regular-file fd1=regular-file fd2=stdio-output',
    descriptor-backed 'cat /tmp/stdin-report.txt' readback, waitpid,
    laststatus, deterministic negatives for output-first ordering,
    '/dev/null' combined input, explicit fd1 aliasing, spaced input grammar,
    errors=0, and PASS. Retained controls cover read-only stdin,
    '/dev/null' stdin, arbitrary '/tmp' stdout output, descriptor-routing and
    pipeline behavior, VFS exec/open/read/write, lifecycle/status, waitpid,
    laststatus, and descriptor-backed cat. No ordering or path-policy decision
    changed; the core task's 64-byte canonical line capacity is treated as an
    implementation fix for carrying the accepted exact command, not a parser
    grammar expansion. Arbitrary input paths, '/dev/null' combined input,
    append/stderr combined forms, broader descriptor grammar, descriptor
    moves, multi-command redirection, persistence, recursive directories,
    process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
    transition remain deferred. The next queued pipeline consumer-output
    redirection core is mechanically unblocked and must remain bounded to
    exact consumer stdout file redirection on the accepted two-stage pipeline.
99. pipeline consumer-output redirection core: accepted in
    'phase10-pipeline-consumer-output-redirection-core-20260605'.
    Shell-visible 'exec stdout | exec stdin >/tmp/pipe-consumer.txt' now
    composes the accepted two-stage VFS-backed stdout-to-stdin pipeline with a
    child-only volatile VFS stdout sink on the consumer. The producer still
    inherits fd1 as the pipe writer and writes 0x1f fixture bytes to
    'stream=pipe-writer route=pipe:stdout-to-stdin'. The consumer inherits
    fd0 as the pipe endpoint and fd1 as a regular file, reads the pipe bytes,
    writes the 0x44-byte stdin report to '/tmp/pipe-consumer.txt', and the
    report is read back through descriptor-backed 'cat
    /tmp/pipe-consumer.txt'. The retained QEMU/substitute evidence records
    waitpid and laststatus for the consumer lifecycle, pipe closure and
    descriptor restoration, the new
    'source=shell-pipe-consumer-stdout-redirection' marker, the retained plain
    pipeline control, deterministic negatives for consumer append redirection,
    stderr-producer consumer-output redirection, and producer-file plus
    consumer-file redirection, errors=0, and PASS. A first smoke iteration
    exposed an overlong negative command that hit the 64-byte line boundary;
    the retained evidence uses a shorter negative so the failure is a parser
    rejection rather than line truncation. Append consumer output, stderr
    producer output redirection, producer file redirection combined with
    consumer output redirection, multi-stage/concurrent pipelines, pipefail,
    jobs, fork/signals, arbitrary descriptor syntax, persistence, recursive
    directories, process accounting/concurrency, Pi 5 proof, networking, SSH,
    and phase transition remain deferred. The queued consumer-output closeout
    is mechanically unblocked and must remain docs/evidence reconciliation
    only.
100. pipeline consumer-output redirection closeout: accepted in
    'phase10-pipeline-consumer-output-redirection-closeout-20260605'. This
    closeout checkpoints the exact pipeline consumer-output frontier without
    adding code. The accepted form remains
    'exec stdout | exec stdin >/tmp/pipe-consumer.txt': producer fd1 remains
    the accepted pipe writer, consumer fd0 remains the accepted pipe reader,
    and consumer fd1 is rebound child-only to the accepted volatile VFS
    '/tmp/pipe-consumer.txt' sink. The consolidated evidence map retains the
    primary QEMU/substitute transcript with producer
    'stream=pipe-writer route=pipe:stdout-to-stdin', consumer
    'fd0=pipe-endpoint fd1=regular-file', the child-only stdout sink route
    'volatile-vfs:/tmp/pipe-consumer.txt', pipe bytes written/read at 0x1f,
    writer closure, reader EOF, shell restoration, descriptor-backed
    'cat /tmp/pipe-consumer.txt' readback of the 0x44-byte consumer report,
    waitpid, laststatus, deterministic negatives, errors=0, and PASS.
    Retained controls cover the plain stdout-to-stdin pipeline,
    stderr-not-piped and descriptor-mixing pipelines, arbitrary '/tmp' output
    redirection, descriptor-backed cat, waitpid, laststatus, VFS
    exec/open/read/write, lifecycle/status, and descriptor restoration. No
    ordering, descriptor, path, persistence, or lifecycle policy changed in
    this closeout. Producer output file redirection away from the pipe,
    consumer append/stderr redirection, arbitrary descriptor syntax,
    multi-stage/concurrent pipelines, pipefail, jobs, fork/signals,
    persistence, recursive directories, process accounting/concurrency,
    Pi 5 proof, networking, SSH, and phase transition remain deferred. The
    next queued producer file-redirection-away core is mechanically unblocked
    and must remain bounded to the exact inverse pipeline/file-redirection
    form.
101. pipeline producer file-redirection-away core: accepted in
    'phase10-pipeline-producer-file-redirection-away-core-20260605'.
    Shell-visible 'exec stdout >/tmp/pipe-source.txt | exec stdin' now
    composes the accepted two-stage VFS-backed stdout-to-stdin pipeline with a
    child-only volatile VFS stdout sink on the producer. The producer pipe
    endpoint is installed on fd1 first, then the producer's child-only stdout
    file redirection replaces fd1 with the accepted regular-file sink
    'volatile-vfs:/tmp/pipe-source.txt', so the stdout fixture writes 0x1f
    bytes to the volatile VFS file instead of the pipe. The consumer inherits
    fd0 as the pipe reader, observes pipe EOF/no-data with bytes written/read at
    zero, writes the accepted no-data stdin report to inherited stdout, and
    waitpid/laststatus report the consumer lifecycle. Descriptor-backed 'cat
    /tmp/pipe-source.txt' reads back the redirected producer payload. The
    retained QEMU/substitute evidence records the new
    'source=shell-pipe-producer-file-redirection-away' marker, producer
    'fd1=regular-file', the stdout sink redirection record, shell descriptor
    restoration, the positive plain pipeline control, deterministic negatives
    for producer append redirection, stderr producer file redirection, and
    producer+consumer file redirection, errors=0, and PASS. The first smoke
    iteration exposed missing kernel-side label/classification wiring for the
    new scenario; the feature transcript was correct but the harness classified
    it as the generic serial loop, so the harness wiring was completed and
    rerun. Producer append pipeline redirection, stderr producer file
    redirection, producer and consumer file redirection in the same pipeline,
    multi-stage/concurrent pipelines, process accounting/concurrency, arbitrary
    descriptor syntax, persistence, recursive directories, Pi 5 proof,
    networking, SSH, and phase transition remain deferred. The queued pipeline
    file-redirection frontier closeout is mechanically unblocked and must
    remain docs/evidence reconciliation only.
102. pipeline file-redirection frontier closeout: accepted in
    'phase10-pipeline-file-redirection-frontier-closeout-20260605'. This
    closeout checkpoints the accepted pipeline plus volatile-file redirection
    frontier without adding code. The accepted consumer-output form remains
    'exec stdout | exec stdin >/tmp/pipe-consumer.txt': producer fd1 feeds the
    pipe, consumer fd0 reads the pipe, and consumer fd1 is rebound child-only
    to 'volatile-vfs:/tmp/pipe-consumer.txt'. The accepted producer-output-away
    form remains 'exec stdout >/tmp/pipe-source.txt | exec stdin': producer fd1
    is rebound child-only to 'volatile-vfs:/tmp/pipe-source.txt', so the
    consumer observes deterministic pipe EOF/no-data while descriptor-backed
    'cat /tmp/pipe-source.txt' reads the producer payload. The consolidated
    evidence map retains both QEMU/substitute transcripts, descriptor-backed
    readbacks, pipe endpoint closure/restoration, waitpid, laststatus,
    deterministic negatives, errors=0, final classifications, and PASS.
    Retained controls cover plain stdout-to-stdin pipeline transfer,
    stderr-not-piped and descriptor-mixing pipelines, arbitrary '/tmp' output
    redirection, descriptor-backed cat, VFS exec/open/read/write,
    lifecycle/status, and descriptor restoration. No ordering, descriptor,
    path, persistence, lifecycle, or process-concurrency policy changed in
    this closeout. Multi-stage/concurrent pipelines, process
    accounting/concurrency, pipefail, jobs, fork/signals, arbitrary descriptor
    syntax, descriptor moves, producer plus consumer file redirection in the
    same pipeline, persistence, recursive directories, Pi 5 proof, networking,
    SSH, and phase transition remain deferred. No explicit queued follow-up is
    mechanically unblocked; supervisor planning is required before promoting
    minimal process accounting/concurrency or another justified shell feature.
103. background VFS exec lifecycle core: accepted in
    'phase10-background-vfs-exec-lifecycle-core-20260605'. Shell-visible
    'exec /bin/status42 &' now launches through the accepted fixed '/bin'
    descriptor-backed VFS exec path while recording minimal shell-owned
    background job accounting. The launch transcript reports
    'source=vfs-open-read mode=background', preserves the accepted
    exec-source/loader/launch/descriptors/startup ABI lines, records a running
    background job with 'shell-responsive=true', and observes completion on the
    next command with status '0x2a', 'observed-status=0x2a', and
    'reaped=true'. The task-owned QEMU/substitute transcript proves the next
    command remains responsive by running descriptor-backed 'cat
    /etc/banner.txt', then records foreground 'waitpid no-child' and
    'last-process none' so the background completion does not corrupt
    foreground lifecycle state. A following foreground 'exec /bin/zero'
    preserves normal consuming 'waitpid' and non-consuming 'laststatus'. The
    evidence map retains plain pipeline transfer, pipeline/file-redirection
    controls, stdio descriptor controls, descriptor-backed cat, deterministic
    async syntax negatives, errors=0, final classification, and PASS. This
    slice accepts only one exact background VFS exec/accounting boundary; true
    scheduler-concurrent user processes, multiple background jobs, jobs/fg/bg,
    process groups, signals, background pipelines/redirections, arbitrary async
    syntax, broad process table policy, Pi 5 proof, networking, SSH, and phase
    transition remain deferred. The queued background lifecycle closeout is
    mechanically unblocked and must remain docs/evidence reconciliation only.
104. background VFS exec lifecycle closeout: accepted in
    'phase10-background-vfs-exec-lifecycle-closeout-20260605'. Static
    inspection reconciles the accepted 'exec /bin/status42 &' QEMU/substitute
    transcript, retained control evidence, roadmap language, and commit
    history without adding runtime behavior. The accepted boundary remains one
    exact shell-owned background accounting record layered on the existing
    VFS exec/open/read, loader, descriptor inheritance, startup ABI, and
    lifecycle/status records. Evidence confirms command-loop responsiveness
    through a following descriptor-backed 'cat /etc/banner.txt', completion
    observation with status '0x2a' and 'reaped=true', foreground 'waitpid
    no-child' and 'last-process none' isolation, and a later normal
    foreground 'exec /bin/zero' waitpid/laststatus control. Retained controls
    cover pipeline/file redirection, stdio, descriptor-backed cat, VFS
    exec/open/read/write, waitpid, laststatus, deterministic negatives,
    errors=0, classifications, and PASS. The next mechanically unblocked task
    is the minimal jobs/accounting list core. Multiple jobs, POSIX job-control
    commands, process groups, sessions, terminal ownership, signals, fork,
    true scheduler-concurrent userspace execution, background pipelines or
    redirections, pipefail, Pi 5 proof, networking, SSH, and phase transition
    remain deferred.
105. jobs/accounting list core: accepted in
    'phase10-jobs-accounting-list-core-20260605'. Shell-visible 'jobs' now
    reports the existing single background VFS exec accounting record without
    adding POSIX job control. Before any background launch it reports
    'jobs none'. After 'exec /bin/status42 &' the first 'jobs' inspection
    reports the stable job id, pid, command label, 'state=running',
    'status=pending', and 'reaped=false'. A following 'jobs' inspection reports
    the same stable id/pid/command with 'state=completed', status '0x2a',
    matching observed status, and 'reaped=true'. The task-owned QEMU/substitute
    transcript proves that the background job table does not create foreground
    waitable lifecycle records: foreground 'waitpid' still reports no-child and
    'laststatus' still reports none after the background accounting observation.
    A later foreground 'exec /bin/zero' preserves normal consuming 'waitpid' and
    non-consuming 'laststatus', and retained controls cover plain pipeline
    transfer, descriptor-backed 'cat /etc/banner.txt', accepted background exec
    evidence, waitpid, pipeline/file redirection, errors=0, final
    classification, and PASS. Unsupported 'fg', 'bg', and 'kill %1' remain
    deterministic unknown-command negatives. This slice accepts only a minimal
    one-record accounting inspection surface; multiple jobs, kill/fg/bg/disown,
    signals, process groups, sessions, terminal ownership, process-tree
    inspection, true scheduler-concurrent userspace execution, Pi 5 proof,
    networking, SSH, and phase transition remain deferred. The queued
    jobs/accounting closeout is mechanically unblocked and must remain
    docs/evidence reconciliation only.
106. jobs/accounting list closeout: accepted in
    'phase10-jobs-accounting-list-closeout-20260605'. Static inspection
    reconciles the accepted 'jobs' QEMU/substitute transcript, retained control
    evidence, roadmap language, and commit history without adding runtime
    behavior. The accepted boundary remains one minimal shell-owned accounting
    inspection command: 'jobs none' before launch, then stable id, pid, command
    label, running/completed state, pending/completed status, observed status,
    and reaped flag for the single accepted 'exec /bin/status42 &' background
    record. Evidence confirms background accounting does not create foreground
    waitable lifecycle records: 'waitpid' reports no-child and 'laststatus'
    reports none after background jobs observations, while a later foreground
    'exec /bin/zero' preserves the normal waitpid/laststatus controls.
    Retained controls cover accepted background exec, pipeline/file
    redirection, descriptor inheritance, descriptor-backed cat, deterministic
    job-control negatives, errors=0, classifications, and PASS. Multiple jobs,
    kill/fg/bg/disown, process groups, sessions, terminal ownership, signals,
    process-tree/procfs inspection, true scheduler-concurrent userspace
    execution, scheduling fairness, background pipelines/redirections, Pi 5
    proof, networking, SSH, and phase transition remain deferred. The queued
    async process-control frontier closeout is mechanically unblocked and must
    remain docs/evidence reconciliation only.
107. async process-control frontier closeout: accepted in
    'phase10-async-process-control-frontier-closeout-20260605'. Static
    inspection reconciles the accepted background VFS exec lifecycle and
    jobs/accounting list slices with retained VFS exec, descriptor inheritance,
    waitpid, laststatus, pipeline, redirection, and descriptor-backed file
    controls without adding runtime behavior. The accepted frontier is one exact
    trailing-ampersand background launch, 'exec /bin/status42 &', through the
    accepted fixed '/bin' VFS exec path, plus one shell-owned accounting record
    visible through 'jobs'. Evidence confirms a stable job id, pid, command
    label, running/completed state, pending/completed status, observed status,
    and reaped flag; shell responsiveness at the command-loop boundary; and
    foreground 'waitpid no-child' plus 'last-process none' isolation until a
    normal foreground exec updates lifecycle state. Retained controls cover
    VFS exec/open/read/write, descriptor inheritance/restoration,
    descriptor-backed cat, plain pipeline transfer, pipeline/file redirection
    composition, deterministic async/job-control negatives, errors=0, final
    classifications, and PASS. Multiple jobs, stale-entry policy beyond the
    accepted single record, kill/fg/bg/disown, process groups, sessions,
    terminal ownership, signals, fork, true scheduler-concurrent userspace
    execution, background pipelines/redirections, pipefail,
    process-tree/procfs inspection, scheduling fairness proof, Pi 5 proof,
    networking, SSH, and phase transition remain deferred. Supervisor planning
    is required before any further process-control, local storage, Pi 5 proof,
    networking, SSH, or phase-transition work.
108. multiple background VFS exec records core: accepted in
    'phase10-multiple-background-vfs-exec-records-core-20260605'. The
    command loop now retains a bounded two-record background accounting table
    for accepted fixed-/bin background VFS exec commands. The task-owned
    QEMU/substitute transcript launches 'exec /bin/status42 &' and
    'exec /bin/zero &' through 'source=vfs-open-read mode=background', then
    reports both records through 'jobs' with distinct stable job ids and pids:
    '/bin/status42' as job 0x1 pid 0x100001 with status 0x2a, and '/bin/zero'
    as job 0x2 pid 0x100002 with status 0x0. Foreground 'waitpid' still
    reports no-child and 'laststatus' still reports none after background-only
    completions; a later foreground 'exec /bin/zero' preserves the normal
    waitpid/laststatus lifecycle controls. Malformed 'exec /bin/status42&' and
    unsupported 'exec stdout &' remain deterministic negatives. Retained
    controls cover accepted single-background and jobs evidence,
    pipeline/file-redirection composition, descriptor-backed cat,
    waitpid/laststatus, errors=0, final classifications, and PASS. This slice
    accepts bounded local/QEMU multiple-background accounting only; stale-entry
    policy beyond retained records, fg/bg/kill/disown, process groups,
    sessions, terminal ownership, signals, fork, background pipelines or
    redirections, scheduler fairness proof, Pi 5 proof, persistent storage,
    networking, SSH, and phase transition remain deferred. The queued
    multiple-background closeout is mechanically unblocked and must remain
    docs/evidence reconciliation only.
109. multiple background VFS exec records closeout: accepted in
    'phase10-multiple-background-vfs-exec-records-closeout-20260605'. Static
    inspection reconciles the accepted two-record background accounting
    transcript, retained controls, roadmap language, and commit history without
    adding runtime behavior. The accepted boundary is two exact fixed-/bin
    background VFS exec forms, 'exec /bin/status42 &' and 'exec /bin/zero &',
    plus 'jobs' inspection of both retained shell-owned records. Evidence
    confirms distinct stable job ids and pids, '/bin/status42' status 0x2a,
    '/bin/zero' status 0x0, running/completed state transitions,
    observed-status fields, reaped flags, foreground 'waitpid no-child' and
    'last-process none' isolation after background-only completions, and normal
    foreground '/bin/zero' waitpid/laststatus controls afterward. Retained
    controls cover accepted single-background exec, jobs/accounting list,
    pipeline/file-redirection composition, descriptor-backed cat,
    waitpid/laststatus, deterministic async negatives, errors=0,
    classifications, and PASS. Stale-entry clearing/retention beyond retained
    records, fg/bg/kill/disown, process groups, sessions, terminal ownership,
    signals, fork, true scheduler-concurrent userspace execution, background
    pipelines/redirections, Pi 5 proof, persistent storage, networking, SSH,
    and phase transition remain deferred. The next mechanically unblocked task
    is the stale-entry policy core; keep it bounded to documented completed-job
    retention for the accepted two-record table.
110. background jobs stale-entry policy core: accepted in
    'phase10-background-jobs-stale-entry-policy-core-20260605'. The
    shell-owned background accounting table now has a minimal bounded
    stale-entry policy: 'jobs' reports all retained records, clears
    completed/reaped records after the report that exposes them, then observes
    one running background job completion for the next inspection. The
    task-owned QEMU/substitute transcript launches 'exec /bin/status42 &' and
    'exec /bin/zero &', shows the first post-launch jobs inspection reporting
    both records, the second inspection reporting only the completed '/bin/zero'
    record, and a later inspection reporting 'jobs none'. Foreground
    'waitpid' still reports no-child and 'laststatus' still reports none after
    background-only completions; a later foreground 'exec /bin/zero' preserves
    normal waitpid/laststatus lifecycle controls. Retained QEMU controls for
    multiple-background records and prior jobs/accounting still pass, including
    descriptor-backed cat, pipeline/file-redirection controls, deterministic
    async/job-control negatives, errors=0, final classifications, and PASS.
    This slice accepts only a minimal shell-owned completed-job retention rule;
    arbitrary process tree/procfs inspection, fg/bg/kill/disown, process
    groups, sessions, terminal ownership, signals, fork, background
    pipelines/redirections, scheduler fairness proof, Pi 5 proof, persistent
    storage, networking, SSH, and phase transition remain deferred. The queued
    stale-entry closeout is mechanically unblocked and must remain
    docs/evidence reconciliation only.
111. background jobs stale-entry policy closeout: accepted in
    'phase10-background-jobs-stale-entry-policy-closeout-20260605'. Static
    inspection reconciles the accepted completed-job retention policy, retained
    multiple-background records, prior jobs/accounting controls, foreground
    lifecycle isolation, descriptor-backed file controls, pipeline/file
    redirection controls, deterministic negatives, roadmap language, and commit
    history without adding runtime behavior. The accepted boundary is two exact
    fixed-/bin background VFS exec forms, 'exec /bin/status42 &' and
    'exec /bin/zero &', plus a minimal shell-owned jobs retention rule:
    completed/reaped records are visible for one jobs report, cleared
    afterward, and a later jobs inspection reports 'jobs none' once all
    completed records have been exposed and removed. Evidence confirms stable
    job ids and pids, '/bin/status42' status 0x2a, '/bin/zero' status 0x0,
    foreground 'waitpid no-child' and 'last-process none' isolation after
    background-only completions, normal foreground '/bin/zero'
    waitpid/laststatus controls afterward, retained descriptor-backed cat and
    pipeline/file-redirection controls, errors=0, classifications, and PASS.
    fg/bg/kill/disown, process groups, sessions, terminal ownership, signals,
    fork, process-tree/procfs inspection, true scheduler-concurrent userspace
    execution, background pipelines/redirections, persistent storage, Pi 5
    proof, networking, SSH, and phase transition remain deferred. The queued
    process-control frontier checkpoint is mechanically unblocked and must
    remain static evidence reconciliation before any milestone closeout,
    local-storage, hardware-proof, networking, SSH, or phase-transition plan.
112. process-control frontier checkpoint: accepted in
    'phase10-process-control-frontier-checkpoint-20260605'. Static inspection
    reconciles the accepted Milestone 10.2 local/QEMU frontier across simple
    pipelines, pipeline/file-redirection composition, descriptor inheritance and
    restoration, VFS exec/open/read/write, waitpid, laststatus,
    descriptor-backed cat controls, fixed-/bin background VFS exec, jobs
    accounting, two retained background records, and deterministic stale-entry
    clearing. The accepted multiple-program progress claim is intentionally
    narrow: multiple fixed-/bin VFS-backed user program records can be launched
    through the background path, inspected through shell-owned jobs accounting,
    completed with stable status records, and cleared by the bounded
    stale-entry policy while the command loop remains responsive for later
    inspections and commands. This checkpoint does not claim preemptive or
    otherwise proven scheduler-concurrent userspace execution, POSIX job
    control, process groups, signals, fork, terminal ownership, background
    pipelines/redirections, multi-stage/concurrent pipelines, pipefail,
    persistent or larger local storage, Pi 5 proof, networking, SSH, Milestone
    10.3, Phase 11, or any phase transition. Supervisor planning should decide
    whether Milestone 10.2 closeout can explicitly preserve that local/QEMU
    frontier wording, or whether one more bounded local process-control proof is
    required first.
113. pipelines and process-control milestone closeout: accepted in
    'phase10-pipelines-process-control-milestone-closeout-20260605'. This
    closeout records Milestone 10.2 as accepted only at the current local/QEMU
    frontier. Retained evidence covers simple two-stage pipelines, pipeline and
    volatile-file redirection composition, exit-status observation, waitpid and
    laststatus, descriptor inheritance/restoration, loader temporary descriptor
    non-leak, descriptor-backed cat, fixed-/bin background VFS exec, jobs
    accounting, multiple background records, and deterministic stale-entry
    clearing. The accepted multiple-program progress claim remains intentionally
    narrow: multiple fixed-/bin VFS-backed user program records can be launched
    through the background path, completed, inspected through shell-owned jobs
    accounting, and cleared while the command loop remains responsive for later
    inspections and commands. This does not claim preemptive or otherwise
    proven scheduler-concurrent userspace execution, full POSIX job control,
    process groups, signals, fork, terminal ownership, background
    pipelines/redirections, multi-stage/concurrent pipelines, pipefail,
    persistent or larger local storage, Pi 5 proof, networking, SSH, Milestone
    10.3 implementation, Phase 11, or any phase transition. The next queued
    Milestone 10.3 storage path evaluation remains a separate explicit
    checkpoint.
114. local storage path evaluation checkpoint: accepted in
    'phase10-local-storage-path-evaluation-checkpoint-20260605'. Static
    source/doc/script review selects generated userland/initramfs manifest
    ingestion into the existing read-only VFS model as the primary Milestone
    10.3 implementation slice. The first proof should add or change userland
    content through a manifest/root input and consume it through the accepted
    descriptor-backed VFS/open/read and, if contracted, VFS-backed exec path
    without adding a hardcoded file-content constant to 'src/initramfs.rs'.
    This selection proves source-code edit avoidance only; kernel binary
    rebuild avoidance, boot archive update, true persistence, SD/USB/block
    storage, Pi 5 proof, networking, SSH, and phase transition remain deferred.
    TFTP-loaded generated initramfs transport is the fallback after the local
    manifest/root contract and implementation are accepted.
115. Pi 5 generated-root boot transport contract: accepted in
    'phase10-pi5-generated-root-boot-transport-contract-20260605'. Static
    source/doc/script review selects a concrete Pi 5 candidate transport:
    carry the existing talos-generated-root-v1 artifact as firmware-loaded
    'initramfs_2712' with root and 'da591740/' archive copies, configure both
    config files with 'initramfs initramfs_2712 followkernel', and derive the
    runtime artifact range from FDT /chosen 'linux,initrd-start' and
    'linux,initrd-end' rather than QEMU's fixed 0x47000000 loader-device
    address. The next candidate task is mechanically unblocked only for a
    non-published archive/static review using
    'target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz'
    and the matching boot tree directory. Hardware publication, fresh serial and
    TFTP evidence, candidate restore, writable persistence, SD/USB/block
    drivers, networking, SSH, and phase transition remain deferred to explicit
    later tasks.
116. Pi 5 generated-root boot archive candidate core: accepted in
    'phase10-pi5-generated-root-boot-archive-candidate-core-20260605'. The
    non-published candidate archive exists at
    'target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz'
    with archive SHA-256
    '8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e'. The
    candidate boot tree carries root and 'da591740/' copies of
    'initramfs_2712', configures both config files with
    'initramfs initramfs_2712 followkernel', and includes a Pi 5 runtime source
    path that reads FDT /chosen 'linux,initrd-start'/'linux,initrd-end' before
    installing source 'firmware-initramfs'. Static evidence records kernel and
    artifact digests, boot-tree listing, config placement, and proof strings.
    The archive remains non-published; Pi 5 power-cycle, TFTP/serial proof,
    restore, writable persistence, SD/USB/block drivers, networking, SSH, and
    phase transition remain deferred to the serialized proof and closeout tasks.
117. Pi 5 generated-root boot transport proof: completed with a source-backed
    blocker in 'phase10-pi5-generated-root-boot-transport-proof-20260605', not
    accepted. Serialized lab evidence shows the Pi fetched the accepted
    candidate kernel and 'da591740/initramfs_2712' artifact, restored the prior
    accepted boot tree hash after the run, and reached the command-loop proof
    harness. The blocker is runtime placement: firmware supplied '/chosen'
    initramfs bounds at '0x2efff000..0x2efff296', but that range overlaps
    Talos' early page-frame seed/bootstrap reservation/translation tables, so
    the generated-root installer later observed 'source=compiled-fallback
    reason=missing-artifact'. The next work should be a planned implementation
    to reserve or copy the firmware initramfs range before early memory setup
    can overwrite it, followed by a fresh serialized Pi 5 proof.
118. Pi 5 generated-root boot transport closeout: accepted in
    'phase10-pi5-generated-root-boot-transport-closeout-20260605' as static
    evidence/docs reconciliation only. It freezes the current Milestone 10.3
    hardware boundary: local/QEMU no-kernel-rebuild generated-root transport is
    accepted, the Pi 5 firmware-initramfs candidate archive shape is accepted as
    a non-published static candidate, and the serialized Pi 5 run is retained as
    source-backed blocker evidence. Pi 5 consumption of the external
    generated-root artifact remains not accepted because the firmware initramfs
    range overlapped early memory setup. Milestone 10.3 remains open until a
    planned implementation reserves or copies that range and a fresh serialized
    Pi 5 proof passes.

The process lifecycle/status closeout checkpoint is accepted in
`phase10-process-lifecycle-status-closeout-20260603`. It records the accepted
frontier as QEMU/substitute-proven shell-visible cat and exec behavior backed
by real VFS/userspace/lifecycle layers: `cat /etc/banner.txt` uses
descriptor-backed open/read, `exec /bin/init` reads the VFS-backed executable
and derives the accepted loader/process/launch records, the zero userspace
status flows through a kernel-owned lifecycle/status record, and `laststatus`
reports that latest lifecycle record with matching identity and status.
Kernel built-ins remain regression/control surfaces only. The recommended next
local execution slice is a supervisor-planned minimal argv/argc ABI for the
existing `exec /bin/init` path, before PATH lookup, arbitrary executable
dispatch, pipes, redirection, writable filesystem, networking, SSH, or Pi 5
proof.

The minimal argc/argv exec-init task is accepted in
`phase10-minimal-argv-argc-exec-init-20260603`. It advances only the explicit
`/bin/init` startup ABI record: the shell-visible exec transcript now reports
`state=minimal-argc1-argv0-init`, `argc=1`, `argv0=/bin/init`,
`argv-null=false`, `envp-null=true`, and
`source=initial-user-stack-record` from the same VFS/program-loader/launch/
lifecycle lineage. The next queued checkpoint should reconcile this narrow ABI
claim before any empty-envp slice or broader command lookup/process-management
planning.

The argc/argv exec-init closeout checkpoint is accepted in
`phase10-argv-argc-exec-init-closeout-20260603`. It keeps the accepted
frontier limited to explicit `/bin/init` startup with `argc=1`,
`argv[0]=/bin/init`, non-null argv state, `envp-null=true` as a current
record field, and the same descriptor-backed VFS/open/read, loader, launch,
lifecycle, `laststatus`, and VFS cat evidence chain. The next queued startup
ABI task remains a deterministic empty-envp record for that same explicit path;
environment variables, auxv/TLS, PATH lookup, arbitrary executable dispatch,
wait/waitpid, descriptor inheritance, pipes, redirection, writable filesystem,
hardware proof, networking, and SSH remain deferred.

The empty-envp exec-init task is accepted in
`phase10-empty-envp-exec-init-20260603`. It advances only the same explicit
`/bin/init` startup ABI record: the shell-visible exec transcript now reports
`state=minimal-argc1-argv0-init-empty-envp`, `argc=1`,
`argv0=/bin/init`, `argv-null=false`, `envp-null=true`,
`envp-state=empty-envp0`, `envp-entries=0`, and
`source=initial-user-stack-record` from the same VFS/program-loader/launch/
lifecycle lineage. The next queued checkpoint should reconcile this narrow
startup ABI frontier before any broader command lookup, process-management,
environment-variable, auxv/TLS, or libc-startup planning.

The startup ABI closeout checkpoint is accepted in
`phase10-startup-abi-closeout-20260603`. It keeps the accepted frontier
limited to explicit `/bin/init` startup with `argc=1`, `argv[0]=/bin/init`,
non-null argv state, `envp-state=empty-envp0`, `envp-entries=0`, an envp
NULL-slot user pointer, `copied-startup-bytes=0x2a`, and the same
descriptor-backed VFS/open/read, loader, launch, lifecycle, `laststatus`, and
VFS cat evidence chain. The closeout recommends supervisor planning for
absolute VFS executable dispatch as the next local execution feature before
PATH lookup, wait/waitpid, descriptor inheritance, pipes, redirection,
writable filesystem, hardware proof, networking, or SSH.

The absolute VFS executable dispatch core is accepted in
`phase10-absolute-vfs-exec-dispatch-core-20260603`. It advances the
shell-visible exec frontier from the special `/bin/init` fixture to bounded
absolute VFS dispatch for `/bin/init` and `/bin/zero`. Both successful
paths read executable bytes through descriptor-backed `TalosOpen`/
`TalosRead`, then preserve the accepted loader, process-install,
address-space, materialization, launch, initial-stack, lifecycle, and status
chain. `laststatus` now includes the latest lifecycle path. `exec /missing`,
`exec init`, `exec /bin`, `exec /etc/banner.txt`, and `exec /empty`
remain deterministic negative controls. Nonzero status variation, PATH lookup,
broad argv/envp, wait/waitpid, descriptor inheritance, pipes, redirection,
writable filesystem, hardware proof, networking, and SSH remain deferred.

The absolute VFS executable dispatch closeout checkpoint is accepted in
`phase10-absolute-vfs-exec-dispatch-closeout-20260603`. It keeps the accepted
frontier limited to absolute VFS dispatch for `/bin/init` and `/bin/zero`
through descriptor-backed `TalosOpen`/`TalosRead`, the accepted loader,
startup ABI, launch, lifecycle/status, and path-aware `laststatus` chain.
Retained evidence covers the non-init `/bin/zero` transcript, `/bin/init`
regression, deterministic failures for `/missing`, `init`, `/bin`,
`/etc/banner.txt`, and `/empty`, plus descriptor-backed `cat /etc/banner.txt`.
The next queued task remains mechanically justified as nonzero status
variation from a VFS-backed executable before PATH lookup, broader argv/envp,
wait/waitpid, descriptor inheritance, pipes, redirection, writable filesystem,
hardware proof, networking, or SSH.

The nonzero VFS exec status task is accepted in
`phase10-vfs-exec-nonzero-status-core-20260603`. It advances the
shell-visible exec/status frontier from only zero-status fixtures to a
deterministic VFS-backed nonzero executable, `/bin/status42`. The transcript
shows `/bin/status42` reaching the accepted descriptor-backed `TalosOpen`/
`TalosRead`, loader, startup ABI, launch, lifecycle/status, and
`laststatus` chain with status `0x2a`. `/bin/init` and `/bin/zero`
remain zero-status controls, deterministic negative exec controls remain
intact, and descriptor-backed `cat /etc/banner.txt` still passes. PATH
lookup, broad argv/envp, wait/waitpid, descriptor inheritance, pipes,
redirection, writable filesystem, hardware proof, networking, and SSH remain
deferred.

The nonzero VFS exec status closeout checkpoint is accepted in
`phase10-vfs-exec-nonzero-status-closeout-20260603`. It keeps the accepted
frontier limited to status variation from VFS-backed executable bytes:
`/bin/status42` reaches the descriptor-backed `TalosOpen`/`TalosRead`,
loader, startup ABI, launch, lifecycle/status, and `laststatus` chain with
status `0x2a`; `/bin/init` and `/bin/zero` remain zero-status controls;
negative exec controls and descriptor-backed `cat /etc/banner.txt` remain
covered by retained evidence. The next recommended feature-led local execution
primitive is minimal wait/waitpid-style lifecycle observation backed by the
accepted kernel-owned lifecycle/status record, before descriptor inheritance,
PATH lookup, broad argv/envp, pipes, redirection, writable filesystem,
hardware proof, networking, or SSH.

The minimal waitpid lifecycle observation core is accepted in
`phase10-minimal-waitpid-lifecycle-observation-core-20260603`. It adds the
narrowest shell-visible wait surface for the current local execution frontier:
`waitpid` consumes one completed child lifecycle/status record produced by the
accepted descriptor-backed VFS exec path. The retained QEMU/substitute
transcript proves no-child before exec, `/bin/status42` wait observation with
status `0x2a`, deterministic already-consumed no-child behavior, `laststatus`
as a non-consuming latest lifecycle view, zero-status waits for `/bin/init`
and `/bin/zero`, negative exec controls, and descriptor-backed
`cat /etc/banner.txt`. Multiple children, asynchronous execution, fork,
signals, descriptor inheritance expansion, PATH lookup, pipes, redirection,
writable filesystem, hardware proof, networking, and SSH remain deferred.

The waitpid lifecycle observation closeout checkpoint is accepted in
`phase10-waitpid-lifecycle-observation-closeout-20260603`. It keeps the
accepted process-management frontier limited to a minimal consuming
wait/waitpid-style observation of one completed child lifecycle/status record
from the descriptor-backed VFS exec path. Retained evidence covers no-child
before exec, `/bin/status42` status `0x2a`, already-consumed no-child,
non-consuming `laststatus`, zero-status `/bin/init` and `/bin/zero`
wait controls, negative exec controls, and descriptor-backed
`cat /etc/banner.txt`. The next recommended feature-led local execution
primitive is standard descriptor inheritance across VFS-backed exec, while
multiple children, asynchronous execution, fork, signals, PATH lookup, pipes,
redirection, writable filesystem, hardware proof, networking, and SSH remain
deferred.

The standard descriptor inheritance closeout checkpoint is accepted in
`phase10-standard-descriptor-inheritance-closeout-20260603`. It keeps the
accepted descriptor frontier limited to standard `fd0`/`fd1`/`fd2`
inheritance records from the shell process descriptor table across
VFS-backed exec, plus proof that loader/VFS temporary executable-read
descriptors are absent from the launched process descriptor set. Retained
evidence covers `/bin/status42`, `/bin/init`, and `/bin/zero` exec
controls, `laststatus`, `waitpid`, deterministic negative exec controls,
and descriptor-backed `cat /etc/banner.txt`. The next recommended
feature-led local execution primitive is minimal literal argv expansion for
absolute VFS exec, while PATH lookup, pipes, redirection, userspace stdio I/O
through inherited descriptors, broad descriptor policy, writable filesystem,
hardware proof, networking, and SSH remain deferred.

Talos is in Phase 8 Milestone 8.3 after the accepted Phase 7 final closeout
checkpoint recommended the first bounded filesystem/program-loading planning
task, the Phase 8 source inventory was accepted, and the read-only
initramfs/VFS contract, smoke plan, target-independent core, and
QEMU/substitute smoke were accepted. The read-only initramfs/VFS closeout
checkpoint is accepted and recommends a documentation-only program-loader
source inventory before any loader implementation or shell work. That
program-loader source inventory is now accepted and recommends a
documentation-only loader format contract before any parser, mapper, process
install, or shell task. That loader format contract is now accepted and
chooses a narrow static ELF64/AArch64 subset, deterministic rejection matrix,
segment permission/zero-fill/entry validation policy, and process-install
boundary before any implementation. The
QEMU/substitute program-loader smoke plan is accepted, naming fixture identity
phase8-program-loader-elf64-aarch64-v1, the retained future smoke evidence
path, exact PASS/classification vocabulary, and deterministic negative cases.
The target-independent program-loader core is now accepted: /bin/init is the
immutable static ELF64/AArch64 fixture, the loader returns an image plan only
with digest, UserText/UserData segment classification, file-copy ranges,
explicit BSS zero-fill, entry validation, and deterministic errors for bad
identity, unsupported dynamic/interpreter headers, malformed ranges, W+X,
out-of-range/overlap, bad entry, and file-range overflow. QEMU/substitute
program-loader smoke evidence is now accepted from the retained
qemu_program_loader_smoke log, which proves the image-plan-only success and
negative cases without process launch or hardware claims. Process address-space
installation remains blocked until later explicit tasks. The program-loader
closeout checkpoint is now accepted and recommends a documentation-only
process-install source inventory before any address-space installation,
lower-EL launch, or shell implementation.
The process-install source inventory is now followed by an accepted
documentation-only process-install contract. The first process-install
boundary is target-independent and metadata-only: a ProcessImageInstallPlan
derived from a validated ProgramImagePlan, preserving exact UserText/UserData
permissions, ordered file-copy and zero-fill page records, deterministic
errors, and all-or-nothing semantics. It accepts no frame allocation,
page-table mutation, scheduler handoff, lower-EL launch, argv/envp, descriptor
inheritance, shell, hardware, or filesystem syscall behavior. The next bounded
task is a QEMU/substitute process-install smoke plan for this metadata-only
boundary.
That QEMU/substitute process-install smoke plan is now accepted. It defines
qemu_process_install_smoke, loader fixture identity
phase8-program-loader-elf64-aarch64-v1, install boundary identity
phase8-process-install-plan-v1, retained evidence path
tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
exact PASS/classification vocabulary, metadata-only success observations,
deterministic no-partial-install rejection cases, and conditional regression
gates. Process-install implementation remains the next queued bounded task;
hardware, physical page allocation, page-table mutation, lower-EL launch,
argv/envp, scheduler handoff, shell, and filesystem syscall behavior remain
blocked.
The metadata-only process-install core and QEMU/substitute smoke are now
accepted. The retained qemu_process_install_smoke evidence proves that the
accepted /bin/init ProgramImagePlan derives a ProcessImageInstallPlan with
preserved entry, footprint, ordered UserText/UserData page records, exact
R-X/RW- permissions, explicit copy/zero-fill ranges, zero side effects, and
deterministic no-partial-install rejections for bad plan invariants, overlap,
permission widening, bad entry, and budget overflow. This still accepts no
physical frame allocation, page-table mutation, process creation, lower-EL
launch, argv/envp, exec/spawn/wait, shell, filesystem syscall behavior,
hardware proof, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.
The process-install closeout checkpoint is now accepted. It reconciles the
accepted source inventory, contract, QEMU/substitute smoke plan, metadata-only
core, retained QEMU/substitute smoke evidence, deferred surfaces, and residual
risks. The accepted frontier is still only target-independent
ProcessImageInstallPlan derivation plus QEMU/substitute no-partial-install
evidence for the immutable /bin/init ProgramImagePlan. No process-owned
address-space installation, frame allocation, page-table mutation, lower-EL
launch, argv/envp, exec/spawn/wait, shell, filesystem syscall behavior,
hardware proof, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy is accepted. No explicit queued follow-up task remains;
supervisor planning is required before the worker may promote the next
Phase 8.3 task.
The process address-space source inventory and contract are now accepted. The
contract selects a target-independent ProcessAddressSpace boundary with
explicit identity, owner label, root/table lease records, user-frame leases,
ordered mappings, copy/zero accounting, all-or-nothing rollback, idempotent
teardown, and deterministic POSIX-shaped errors. It is not a hardware
page-table installer and accepts no AArch64 descriptor construction,
TTBR/TCR/ASID/TLB policy, lower-EL launch, argv/envp, scheduler handoff,
process table/PID/wait/exit, descriptor inheritance, filesystem syscalls,
hardware proof, networking, or SSH. The next bounded task is a
QEMU/substitute process address-space smoke plan for this selected boundary.
That QEMU/substitute process address-space smoke plan, target-independent
core, retained QEMU/substitute smoke evidence, and closeout checkpoint are now
accepted. The accepted frontier is a target-independent ProcessAddressSpace
model for immutable /bin/init derived from the accepted ProgramImagePlan and
ProcessImageInstallPlan. It proves one model root token, table lease,
user-frame leases, ordered UserText/UserData mappings, copy/zero accounting,
all-or-nothing rollback, idempotent teardown, and deterministic no-partial
install/no-leak rejections through the retained
qemu-process-address-space-smoke log. It still accepts no physical page-table
mutation, TTBR/TCR switching, ASID/TLB policy, lower-EL launch, argv/envp,
exec/spawn/wait, shell, descriptor-backed filesystem syscalls, writable
filesystem, hardware proof, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy. No explicit queued follow-up task
remains; supervisor planning is required before the worker may promote the
next Phase 8.3 task.
The process page-table materialization source inventory, contract, smoke plan,
implementation, QEMU/substitute smoke evidence, and closeout checkpoint are
now accepted. The accepted frontier is a non-activating AArch64
descriptor-image/user-frame materialization record for immutable /bin/init,
below TTBR activation and lower-EL launch, with retained
qemu-process-page-table-materialization-smoke evidence. The initial process
launch source inventory is now accepted and recommends a documentation-only
initial-process-launch contract before any TTBR activation, lower-EL ERET,
initial user stack implementation, scheduler runnable publication, argv/envp,
process lifecycle, shell, filesystem syscall behavior, hardware proof,
networking, or SSH. That initial-process-launch contract is now accepted. It
selects a target-independent InitialProcessLaunchPlan boundary with identity
phase8-initial-process-launch-plan-v1, validates entry provenance across the
accepted image/install/address-space/materialization records, records
blocked-missing-initial-user-stack and blocked-no-ttbr-activation state, and
keeps saved-frame intent and scheduler publication below any register write,
ERET, TTBR activation, runnable process, or hardware claim. The next bounded
task is a QEMU/substitute smoke plan for this launch-preparation boundary.
That QEMU/substitute smoke plan and the target-independent initial process
launch core are now accepted. The accepted frontier is an inspectable
InitialProcessLaunchPlan for immutable /bin/init with copied fixture/install,
address-space, and materialization identities, entry provenance through
UserText mapping and EL0-executable descriptor coverage,
blocked-missing-initial-user-stack and blocked-no-ttbr-activation state,
saved-frame intent without architectural register writes, explicit zero
TTBR/TLB/scheduler/process-table/descriptor-table/lower-EL side effects, and
ENOSYS runnable-commit rejection with no-partial-launch and
no-runnable-publication evidence. QEMU/substitute smoke evidence for this
boundary is now accepted at
tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log
with qemu-initial-process-launch-smoke-complete and PASS lines. Pi 5 hardware
proof and runnable lower-EL process claims remain blocked. The initial process
launch closeout checkpoint is now accepted. It reconciles the source
inventory, contract, smoke plan, core, retained QEMU/substitute evidence, and
deferred surfaces for this launch-preparation boundary. The accepted frontier
is still only target-independent InitialProcessLaunchPlan construction plus
QEMU/substitute no-partial-launch/no-runnable-publication evidence for
immutable /bin/init. No executable user process, initial stack, TTBR
activation, lower-EL ERET, process lifecycle, shell, filesystem syscall,
hardware, networking, or SSH capability is accepted. No explicit queued
follow-up task remains; supervisor planning is required before the worker may
promote the next Phase 8.3 task. Supervisor planning has selected the next
bounded Phase 8.3 slice: initial user stack construction below live launch.
The initial user stack source inventory is now accepted. It maps POSIX
user-range/copy vocabulary, loader/install/address-space/materialization
lease owners, InitialProcessLaunchPlan blocked stack state, lower-EL
saved-frame vocabulary, scheduler placeholders, and QEMU/Pi 5 proof-local
stack fixtures. It recommends
phase8-initial-user-stack-contract-20260530 as the next bounded
documentation-only task. Stack implementation, argv/envp/auxv/TLS setup,
TTBR activation, lower-EL ERET, scheduler publication, process lifecycle,
filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
The initial user stack contract is now accepted. It selects a model-only
InitialUserStackPlan boundary with stack top 0x0000_8000_0000_0000, usable
range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000), one unmapped guard page
below it, 16-byte initial SP alignment, USER_DATA stack pages,
copied_bytes=0, zeroed_bytes=0x4000, idempotent stack teardown,
deterministic no-partial-stack/no-partial-launch errors, and launch-plan
integration that may change only model stack state while activation,
lower-EL ERET, scheduler publication, process lifecycle, filesystem syscalls,
Pi 5 proof, networking, and SSH remain blocked. It names
phase8-qemu-initial-user-stack-smoke-plan-20260530 as the next bounded
documentation-only task. That QEMU/substitute smoke plan is now accepted. It
selects scenario qemu_initial_user_stack_smoke, retained evidence path
tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log,
classification qemu-initial-user-stack-smoke-complete, and PASS vocabulary.
It requires success observations for stack range, guard, alignment, top SP,
frame/page ownership, zero/copy accounting, teardown, minimal-empty-argc0
startup metadata, model-only launch-plan stack-state integration, and zero
TTBR/TLB/lower-EL/scheduler/process-table/descriptor-table side effects. It
also requires deterministic no-partial-stack/no-partial-launch rejection cases
and names phase8-initial-user-stack-core-20260530 as the next bounded
implementation task. Live TTBR activation, lower-EL ERET, scheduler
publication, process lifecycle, broad argv/envp/auxv/TLS ABI, filesystem
syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
The initial user stack core is now accepted. It adds the target-independent
InitialUserStackPlan model boundary, fixed top-of-user-space stack layout,
stack-owned zeroed USER_DATA page leases, unmapped guard accounting,
minimal-empty-argc0 startup placeholder, launch-plan stack-ready binding,
idempotent stack teardown, deterministic no-partial-stack/no-partial-launch
rejections, and unit evidence that live TTBR/TLB/lower-EL/scheduler/process
and descriptor side effects remain zero. No QEMU smoke, Pi 5 hardware proof,
live activation, runnable publication, process lifecycle, filesystem syscall,
networking, or SSH capability is accepted by this core slice.
QEMU/substitute initial user stack smoke evidence is now accepted from the
retained qemu_initial_user_stack_smoke log. It proves the accepted
InitialUserStackPlan success, fixed stack layout, guard and page ownership,
minimal-empty-argc0 startup placeholder, model-only launch-plan stack-ready
binding, idempotent teardown, deterministic no-partial-stack/no-partial-launch
rejections, zero live-launch side effects, and final
qemu-initial-user-stack-smoke-complete/PASS lines. No Pi 5 hardware proof,
live TTBR activation, lower-EL ERET, scheduler runnable publication, process
lifecycle, filesystem syscall, networking, or SSH capability is accepted by
this smoke slice. The initial user stack closeout checkpoint is accepted, and
supervisor planning selected live address-space activation as the next Phase
8.3 frontier. The live address-space activation source inventory is now
accepted. It maps accepted loader/install/address-space/materialization/
launch/stack artifacts to the still-missing live activation boundary:
TTBR0_EL1/TTBR1_EL1 root provenance, TCR_EL1/MAIR_EL1/SCTLR_EL1 compatibility,
ASID/TLB policy, barrier ordering, kernel reachability, exception/fault
reporting, rollback/teardown, and the separation from lower-EL ERET and
scheduler runnable publication. It recommends
phase8-live-address-space-activation-contract-20260530 as the next bounded
documentation-only task. Live register mutation, lower-EL launch, process
lifecycle, filesystem syscalls, Pi 5 hardware proof, networking, and SSH
remain blocked.
The live address-space activation contract is accepted. It selects
phase8-live-address-space-activation-plan-v1 as a target-independent
activation-preflight record with policy
preflight-split-user-ttbr0-kernel-reachability-blocked-v1. The contract
defines the before/after activation invariant, TTBR0 root provenance,
TTBR1/kernel-half blocker, TCR/MAIR/SCTLR compatibility checks, ASID/TLB/
barrier blocked states, kernel reachability and fault-reporting prerequisites,
deterministic error/blocker vocabulary, and no-partial-activation/
no-runnable-publication behavior. It recommends
phase8-qemu-live-address-space-activation-smoke-plan-20260530 as the next
bounded documentation-only task. Live translation-register mutation,
lower-EL ERET, scheduler publication, process lifecycle, filesystem syscalls,
Pi 5 hardware proof, networking, and SSH remain blocked.
The QEMU/substitute live address-space activation smoke plan is accepted. It
defines qemu_live_address_space_activation_smoke, retained evidence path
tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log,
exact qemu-live-address-space-activation-smoke-complete/PASS vocabulary,
success observations for copied input identities, TTBR0 root provenance,
blocked TTBR1/kernel-half policy, TCR/MAIR/SCTLR compatibility records,
ASID/TLB/barrier blocked states, kernel reachability prerequisites,
model-only activation binding, teardown, deterministic no-partial-activation
rejections, and zero live side effects. The next bounded implementation task
is phase8-live-address-space-activation-core-20260530 if dependencies remain
satisfied. Live translation-register mutation, lower-EL ERET, scheduler
publication, process lifecycle, filesystem syscalls, Pi 5 hardware proof,
networking, and SSH remain blocked.
The live address-space activation core is now accepted. It adds the
target-independent LiveAddressSpaceActivationPlan preflight boundary with
identity phase8-live-address-space-activation-plan-v1, copied accepted
image/install/address-space/materialization/launch/stack lineage, TTBR0 root
provenance from the materialized root lease without writing TTBR0_EL1, blocked
TTBR1/kernel-half policy, TCR/MAIR compatibility records, blocked SCTLR/ASID/
TLB/barrier/live-register states, required kernel reachability checklist,
model-only activation-preflight-ready launch binding, idempotent plan-local
teardown, deterministic no-partial-activation/no-runnable-publication
rejections, and unit evidence that all live TTBR/TCR/MAIR/SCTLR/TLB/
lower-EL/scheduler/process/descriptor side effects remain zero. QEMU smoke
evidence, live register mutation, lower-EL ERET, scheduler publication,
process lifecycle, filesystem syscalls, Pi 5 hardware proof, networking, and
SSH remain blocked.
The QEMU/substitute live address-space activation smoke core is accepted. It
adds qemu_live_address_space_activation_smoke and retained evidence proving
the accepted activation preflight identity, policy, copied loader/install/
address-space/materialization/launch/stack lineage, TTBR0 root provenance,
blocked TTBR1/kernel-half policy, TCR/MAIR/SCTLR compatibility/blocker
states, ASID/TLB/barrier/live-register blockers, kernel reachability
prerequisites, model-only activation binding, idempotent plan-local teardown,
deterministic no-partial-activation rejection cases, zero live side effects,
and final qemu-live-address-space-activation-smoke-complete/PASS lines. Live
register mutation, lower-EL ERET, scheduler publication, process lifecycle,
filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
The live address-space activation closeout checkpoint is now accepted. It
reconciles the source inventory, contract, smoke plan, target-independent
preflight core, retained QEMU/substitute smoke evidence, deferred surfaces,
and residual risks. The accepted frontier remains a model-only
LiveAddressSpaceActivationPlan below live register mutation, lower-EL ERET,
and runnable publication. No explicit queued follow-up task remains;
supervisor planning is required before the worker may promote another Phase
8.3 task.
The kernel-half reachability source inventory is now accepted. It maps the
accepted activation frontier to the remaining blocked-no-accepted-kernel-half-map
gap: kernel text/rodata/data/bss, active kernel stack, heap/page-frame
allocator, VBAR_EL1, exception vectors, UART/MMIO diagnostics, scheduler state,
panic/fault reporting, early translation helpers, process descriptor images,
and AArch64 TTBR/TCR/MAIR vocabulary. It identifies TTBR1_EL1 shared kernel
root, replicated kernel-half descriptors, and an explicitly blocked preflight
record as candidate first-slice policies, and recommends
phase8-kernel-half-reachability-contract-20260531 as the next bounded
documentation-only task. Live register mutation, lower-EL ERET, scheduler
publication, process lifecycle, filesystem syscalls, Pi 5 hardware proof,
networking, and SSH remain blocked.
The kernel-half reachability contract is now accepted. It selects a
preflight-only KernelHalfReachabilityPlan with identity
phase8-kernel-half-reachability-plan-v1 and policy
preflight-ttbr1-shared-kernel-root-reachability-v1. The policy chooses the
future split direction: process user mappings remain TTBR0_EL1-owned, while a
shared privileged kernel root is reserved for TTBR1_EL1. This contract records
required kernel text/data/bss/vector/stack/heap/UART/MMIO/scheduler/fault
reachability, TCR/MAIR/TTBR/ASID/TLB/barrier compatibility vocabulary,
deterministic blocker/error behavior, no-partial construction, and zero live
side effects. Kernel-half descriptor-image construction, live register
mutation, lower-EL ERET, scheduler publication, process lifecycle, filesystem
syscalls, Pi 5 hardware proof, networking, and SSH remain blocked. The next
bounded task is phase8-qemu-kernel-half-reachability-smoke-plan-20260531.
The QEMU/substitute kernel-half reachability smoke plan is now accepted. It
selects qemu_kernel_half_reachability_smoke as the retained-evidence scenario
for the accepted preflight-only KernelHalfReachabilityPlan boundary and fixes
the evidence path under
tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/. The plan
requires copied loader/install/address-space/materialization/launch/stack/
activation lineage, TTBR0 root provenance, selected TTBR1 shared-kernel-root
policy with descriptor-image construction blocked, kernel text/data/bss/
vector/stack/heap/UART/MMIO/scheduler/fault reachability entries,
compatibility-only TCR/MAIR records, blocked SCTLR/ASID/TLB/barrier/live
register states, deterministic no-partial rejection cases, idempotent
plan-local teardown, zero live side effects, and final
qemu-kernel-half-reachability-smoke-complete/PASS lines. Kernel-half
descriptor-image construction, live register mutation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscalls, Pi 5 hardware
proof, networking, and SSH remain blocked. The next bounded task is
phase8-kernel-half-reachability-core-20260531.
The kernel-half reachability core is now accepted. It adds the
target-independent KernelHalfReachabilityPlan preflight boundary with identity
phase8-kernel-half-reachability-plan-v1 and policy
preflight-ttbr1-shared-kernel-root-reachability-v1. The plan consumes accepted
loader/install/address-space/materialization/launch/stack/live-activation
lineage, copies TTBR0 root provenance, selects a TTBR1 shared privileged
kernel-root policy, keeps descriptor-image construction blocked, records
required kernel text/rodata/data/bss/vector/stack/heap/page-frame/UART/MMIO/
scheduler/fault reachability, records split TCR and normal/device MAIR
compatibility only, and proves deterministic no-partial rejection plus
idempotent plan-local teardown in unit tests. No QEMU/substitute retained log,
live register mutation, lower-EL ERET, scheduler publication, process
lifecycle, filesystem syscall behavior, Pi 5 hardware proof, networking, or
SSH is accepted by this core. The next bounded task is
phase8-qemu-kernel-half-reachability-smoke-core-20260531.
The QEMU/substitute kernel-half reachability smoke core is now accepted. It
adds qemu_kernel_half_reachability_smoke and retains evidence at
tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.
The log proves copied loader/install/address-space/materialization/launch/
stack/live-activation lineage, the phase8-kernel-half-reachability-plan-v1
boundary, TTBR0 materialized-root provenance with no write, TTBR1 shared
privileged kernel-root policy with descriptor-image construction blocked,
required kernel reachability entries, split TCR and normal/device MAIR
compatibility-only records, blocked SCTLR/ASID/TLB/barrier/live-register
states, deterministic no-partial rejection cases, idempotent plan-local
teardown, zero live side effects, and final
qemu-kernel-half-reachability-smoke-complete/PASS lines. Kernel-half
descriptor-image construction, live register mutation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscalls, Pi 5 hardware
proof, networking, and SSH remain blocked. The next bounded task is
phase8-kernel-half-reachability-closeout-checkpoint-20260531.
The kernel-half reachability closeout checkpoint is now accepted. It
reconciles the source inventory, contract, smoke plan, target-independent
preflight core, retained QEMU/substitute smoke evidence, deferred surfaces,
and residual risks. The accepted frontier remains a model-only
KernelHalfReachabilityPlan below kernel-half descriptor-image construction,
live translation-register mutation, lower-EL ERET, and runnable publication.
No explicit queued follow-up task remains; supervisor planning is required
before the worker may promote another Phase 8.3 task.
The kernel-half descriptor-image source inventory is now accepted. It maps the
accepted KernelHalfReachabilityPlan frontier to the remaining
blocked-no-kernel-half-descriptor-image boundary and distinguishes
descriptor-image construction from live TTBR/TCR/MAIR/SCTLR mutation,
ASID/TLB/barrier activation, lower-EL ERET, and scheduler publication. It
identifies src/kernel_half_reachability.rs, src/process_page_table_materialization.rs,
src/memory_map/translation.rs, linker-owned kernel ranges, memory/page-frame
owners, exception/vector owners, UART/MMIO diagnostics, scheduler state, and
live activation records as source material for a non-installed TTBR1 shared
privileged kernel-root descriptor image. Live register mutation, lower-EL
launch, scheduler publication, process lifecycle, filesystem syscall
expansion, Pi 5 hardware proof, networking, and SSH remain blocked. The next
bounded task is phase8-kernel-half-descriptor-image-contract-20260531.
The kernel-half descriptor-image contract is now accepted. It selects a
non-installed KernelHalfDescriptorImage boundary with identity
phase8-kernel-half-descriptor-image-v1 and policy
ttbr1-shared-privileged-kernel-root-descriptor-image-v1. The contract consumes
the accepted KernelHalfReachabilityPlan, TTBR0 materialized-root provenance,
linker-owned kernel ranges, AArch64 descriptor vocabulary, and source-owner
coverage for kernel text/rodata/data/bss, vectors, stack, heap, page-frame
metadata, UART/MMIO diagnostics, scheduler state, runtime console, and
panic/fault reporting. It requires privileged-only descriptor attributes,
deterministic no-partial-image errors, explicit model-owned root/table lease
ownership, idempotent teardown, and zero live TTBR/TCR/MAIR/SCTLR/TLB/barrier,
lower-EL, scheduler, process-table, and descriptor-table side effects. Live
register mutation, ASID/TLB/barrier activation, lower-EL ERET, scheduler
publication, process lifecycle, filesystem syscall expansion, Pi 5 hardware
proof, shell behavior, networking, and SSH remain blocked. The next bounded
task is phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531.
The QEMU/substitute kernel-half descriptor-image smoke plan is now accepted.
It defines the retained qemu_kernel_half_descriptor_image_smoke evidence
boundary for the selected non-installed KernelHalfDescriptorImage model:
copied accepted input lineage, TTBR0 provenance-only root intent,
model-owned TTBR1 kernel-root image intent, required kernel coverage,
privileged-only normal/device descriptor attributes, root/table lease
ownership, deterministic rejection, no-partial rollback, idempotent teardown,
zero live activation side effects, and final
qemu-kernel-half-descriptor-image-smoke-complete/PASS lines. Live register
mutation, ASID/TLB/barrier activation, lower-EL ERET, scheduler publication,
process lifecycle, filesystem syscall expansion, Pi 5 hardware proof, shell
behavior, networking, and SSH remain blocked. The next bounded task is
phase8-kernel-half-descriptor-image-core-20260531.
The kernel-half descriptor-image core is now accepted. It adds the
target-independent KernelHalfDescriptorImage construction boundary selected by
the accepted contract and smoke plan, with boundary identity
phase8-kernel-half-descriptor-image-v1 and policy
ttbr1-shared-privileged-kernel-root-descriptor-image-v1. The model consumes
the accepted KernelHalfReachabilityPlan and ProcessPageTableMaterialization
provenance, publishes model-owned root/table leases and descriptor records
for kernel text/rodata/data/bss, vectors, active stack, heap, page-frame
metadata, UART/MMIO diagnostics, scheduler state, runtime console, and
panic/fault reporting, preserves privileged-only normal/device attributes,
rolls back resource failures without a partial image, supports idempotent
teardown, and records zero live TTBR/TCR/MAIR/SCTLR/TLB/barrier, lower-EL,
scheduler, process-table, or descriptor-table side effects. Retained
QEMU/substitute smoke evidence, live register mutation, ASID/TLB/barrier
activation, lower-EL ERET, scheduler publication, process lifecycle,
filesystem syscall expansion, Pi 5 hardware proof, shell behavior,
networking, and SSH remain blocked. The next bounded task is
phase8-qemu-kernel-half-descriptor-image-smoke-core-20260531.
The QEMU/substitute kernel-half descriptor-image smoke core is now accepted.
It wires qemu_kernel_half_descriptor_image_smoke, retains the planned log at
tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log,
and proves copied input lineage, TTBR0 materialized-root provenance without a
write, TTBR1 model-owned kernel-root image intent without installation,
required kernel coverage, privileged-only normal/device descriptor
attributes, model-owned root/table leases, deterministic no-partial-image
rejections, idempotent teardown, and zero live activation/lower-EL/scheduler/
process/descriptor-table side effects. Pi 5 hardware was not used and the
hardware lock remained untouched. Live register mutation, ASID/TLB/barrier
activation, lower-EL ERET, scheduler publication, process lifecycle,
filesystem syscall expansion, Pi 5 proof, shell behavior, networking, and SSH
remain blocked. The next bounded task is
phase8-kernel-half-descriptor-image-closeout-checkpoint-20260531.
The kernel-half descriptor-image closeout checkpoint is now accepted. It
reconciles the accepted source inventory, contract, QEMU/substitute smoke
plan, implementation, retained smoke evidence, and deferred surfaces for the
non-installed KernelHalfDescriptorImage construction boundary. The accepted
frontier is target-independent descriptor-image construction with copied
Phase 8 lineage, TTBR0 provenance, model-owned TTBR1 kernel-root image intent,
required kernel coverage, privileged-only normal/device descriptor attributes,
model-owned root/table leases, deterministic no-partial rollback,
idempotent teardown, and zero live side effects. Descriptor-image
installation, live register mutation, ASID/TLB/barrier activation, lower-EL
ERET, scheduler publication, process lifecycle, filesystem syscall expansion,
Pi 5 proof, shell behavior, networking, and SSH remain blocked. No explicit
queued follow-up task remains; supervisor planning is required before the
worker may promote another Phase 8.3 task.
The live descriptor-image installation source inventory is now accepted. It
maps the accepted non-installed KernelHalfDescriptorImage frontier to the
next installation handoff below TTBR activation. The inventory distinguishes
published=true/installed=false descriptor-image evidence from any future
installation claim, identifies src/kernel_half_descriptor_image.rs,
src/live_address_space_activation.rs, src/kernel_half_reachability.rs,
src/process_page_table_materialization.rs, src/memory_map/translation.rs,
linker-owned ranges, exception/vector owners, UART/MMIO diagnostics, runtime
console, and scheduler owners as source material, and recommends a
target-independent installation-ready activation binding as the next
contract. Live register mutation, ASID/TLB/barrier activation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscall expansion, Pi 5
proof, shell behavior, networking, and SSH remain blocked. The next bounded
task is phase8-live-descriptor-image-installation-contract-20260531.
The live descriptor-image installation contract is now accepted. It selects a
target-independent KernelHalfDescriptorImageInstallation boundary, or
equivalent activation extension record, that binds the accepted non-installed
KernelHalfDescriptorImage to the accepted LiveAddressSpaceActivationPlan as a
model-level installation-ready activation binding below TTBR activation. The
contract preserves copied Phase 8 lineage, TTBR0 materialized-root
provenance, TTBR1 descriptor-image root provenance without a register write,
kernel-half coverage and privileged-only permissions, device MMIO attributes,
fault-reporting prerequisites, rollback/teardown, deterministic no-partial
installation rejection, and zero live side effects. Live register mutation,
active-root descriptor copy, ASID/TLB/barrier activation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscall expansion,
Pi 5 proof, shell behavior, networking, and SSH remain blocked. The next
bounded task is phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531.
The QEMU/substitute live descriptor-image installation smoke plan is now
accepted. It defines qemu_live_descriptor_image_installation_smoke as the
first evidence boundary for the accepted model-level installation-ready
binding, with retained evidence at
tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log,
classification qemu-live-descriptor-image-installation-smoke-complete, and
PASS vocabulary. The smoke must prove copied descriptor-image and activation
lineage, TTBR0/TTBR1 provenance without register writes, preserved
kernel-half coverage/permissions/diagnostics, installation-ready binding,
rollback/teardown, deterministic no-partial-install rejection, and zero live
side effects. The next bounded task is
phase8-live-descriptor-image-installation-core-20260531. Live register
mutation, active-root descriptor copy, ASID/TLB/barrier activation, lower-EL
ERET, scheduler publication, process lifecycle, filesystem syscall expansion,
Pi 5 proof, shell behavior, networking, and SSH remain blocked.
The target-independent live descriptor-image installation core and
QEMU/substitute smoke evidence are now accepted. The accepted frontier is a
model-only KernelHalfDescriptorImageInstallation for immutable /bin/init that
binds the accepted non-installed KernelHalfDescriptorImage to the accepted
LiveAddressSpaceActivationPlan as an installation-ready activation binding.
The retained qemu-live-descriptor-image-installation-smoke log proves copied
loader/install/address-space/materialization/launch/stack/activation/
reachability/descriptor-image lineage, TTBR0/TTBR1 provenance without register
writes, preserved kernel-half coverage and privileged-only normal/device
policy, diagnostic reachability, deterministic no-partial-install rejection,
idempotent teardown, zero live side effects, and final
qemu-live-descriptor-image-installation-smoke-complete/PASS lines. The live
descriptor-image installation closeout checkpoint is now accepted. The next
objective Phase 8.3 frontier is live translation-register activation planning,
but no explicit queued follow-up task remains; supervisor planning is required
before the worker may promote another Phase 8.3 task. Live register mutation,
active-root descriptor copy, ASID/TLB/barrier activation, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscall expansion, Pi 5
proof, shell behavior, networking, and SSH remain blocked.
The live translation-register activation source inventory is now accepted. It
maps the accepted model-only KernelHalfDescriptorImageInstallation frontier to
the next activation handoff and distinguishes accepted TTBR0 materialized-root
provenance, TTBR1 descriptor-image kernel-root provenance, compatibility-only
TCR/MAIR records, blocked SCTLR/ASID/TLB states, planned-only no-live DSB/ISB,
kernel-owned fault-reporting reachability, and zero live side effects from
unaccepted TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation. The
inventory identifies live descriptor-image installation, live activation,
kernel-half descriptor-image, kernel-half reachability, page-table
materialization, translation, linker, exception/vector, UART/MMIO diagnostic,
runtime console, and scheduler source owners, and recommends
phase8-live-translation-register-activation-contract-20260531 as the next
bounded documentation-only task. Active-root descriptor copy, ASID/TLB/barrier
activation, lower-EL ERET, scheduler publication, process lifecycle,
filesystem syscall expansion, Pi 5 proof, shell behavior, networking, and SSH
remain blocked.
The live translation-register activation contract is now accepted. It selects
a target-independent model/substitute-only activation-commit boundary with
identity phase8-live-translation-register-activation-v1 and policy
model-ttbr0-ttbr1-activation-commit-below-live-registers-v1. The contract
consumes the accepted live descriptor-image installation and copied Phase 8
lineage, verifies TTBR0 materialized-root provenance, TTBR1 descriptor-image
kernel-root provenance, TCR/MAIR compatibility records, blocked
SCTLR/ASID/TLB/barrier states, active-root nonmutation, kernel-owned
fault-reporting reachability, deterministic rejection vocabulary, rollback/
teardown, and zero live side effects. It explicitly does not accept live
TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, active-root copy,
ASID/TLB/barrier execution, lower-EL ERET, scheduler publication, process
lifecycle, filesystem syscall expansion, Pi 5 proof, boot archive publication,
shell behavior, networking, or SSH. The mechanically next bounded task is
phase8-qemu-live-translation-register-activation-smoke-plan-20260531 if
dependencies remain satisfied.
The QEMU/substitute live translation-register activation smoke plan is now
accepted. It requires scenario
qemu_live_translation_register_activation_smoke, retained evidence at
tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/qemu-live-translation-register-activation-smoke.log,
classification qemu-live-translation-register-activation-smoke-complete, and
PASS vocabulary. The smoke must prove copied installation and Phase 8 lineage,
TTBR0/TTBR1 provenance without register writes, TCR/MAIR compatibility,
blocked SCTLR/ASID/TLB/barrier states, active-root nonmutation, preserved
kernel diagnostic reachability, model-only activation-commit intent,
rollback/teardown, deterministic no-partial-activation rejection, and zero
live side effects. The next bounded task is
phase8-live-translation-register-activation-core-20260531. Live register
mutation, active-root descriptor copy, ASID/TLB/barrier activation, lower-EL
ERET, scheduler publication, process lifecycle, filesystem syscall expansion,
Pi 5 proof, shell behavior, networking, and SSH remain blocked.
The target-independent live translation-register activation core is now
accepted. It implements a model-level LiveTranslationRegisterActivation record
with boundary phase8-live-translation-register-activation-v1 and policy
model-ttbr0-ttbr1-activation-commit-below-live-registers-v1. The record
consumes the accepted live descriptor-image installation, copies the accepted
Phase 8 lineage, preserves TTBR0 materialized-root and TTBR1 descriptor-image
kernel-root provenance without register writes, records compatibility-only
TCR/MAIR state, blocks SCTLR/ASID/TLB/barrier/live-register state, preserves
kernel diagnostic reachability, supports idempotent teardown, rejects
deterministic no-partial-activation cases, and keeps zero live side effects.
The QEMU/substitute smoke route is wired and a retained log exists for the
queued smoke-core task. Live register mutation, active-root descriptor copy,
ASID/TLB/barrier execution, lower-EL ERET, scheduler publication, process
lifecycle, filesystem syscall expansion, Pi 5 proof, shell behavior,
networking, and SSH remain blocked.
The
accepted Phase 7
frontier includes the Phase
6.3 production scheduler runtime closeout,
the full Phase 7.1 POSIX baseline slice, the Phase 7.2 EL0/address-space source
inventory, the Phase 7.2 EL0 trap/address-space contract, and the first
target-independent user-memory permission core. The accepted Phase 7.2 contract
defines the first user/kernel virtual-address split vocabulary, lower-EL
trap/return invariants, user fault classes, copy-in/copy-out preconditions,
evidence levels, and blocked surfaces. The accepted QEMU EL0 trap smoke plan
defined the first lower-EL proof boundary: one QEMU-only built-in user payload,
a diagnostic SVC marker trap back to the kernel, saved-state output, and
PASS/classification evidence. The accepted QEMU implementation now reports
classification=qemu-el0-trap-smoke-complete and qemu-el0-trap-smoke: PASS
from retained QEMU/substitute serial evidence. The serialized Pi 5 proof is
also accepted: local62 retained physical serial evidence contains the
source-backed translation feature report, regular VBAR_EL1 handoff,
lower-AArch64 synchronous SVC trap state,
classification=pi5-el0-trap-proof-complete, and rpi5-el0-trap-proof: PASS.
This accepts only the bounded lower-EL trap path. General SVC/syscall ABI,
VFS, filesystem, program loading, descriptor I/O, networking, SSH, and shell
work remain blocked until later explicit bounded tasks accept their contracts
and gates. The accepted Phase 7.3 syscall ABI source inventory maps the source
owners and gaps for SVC exception decoding, syscall number and argument
registers, return/error convention, user-copy preconditions, descriptor-table
interaction, and process/task ownership. The accepted Phase 7.3 syscall ABI
contract fixes lower-AArch64 svc #0, x8 syscall numbers, x0 through x5 scalar
arguments, x0 negative errno returns, talos_nop = 0, and unknown syscall =
-ENOSYS. The accepted target-independent syscall dispatch core implements that
bounded vocabulary and unit-tested return/error encoding without production
exception routing, QEMU, or hardware work. The accepted syscall trap-routing
source inventory maps production lower-AArch64 SVC detection, svc immediate
validation, x8 syscall-number extraction, x0-through-x5 argument capture, x0
return mutation, ELR/SPSR handling, diagnostic marker quarantine, and
non-syscall fallback. The accepted syscall trap-routing contract fixes the
production routing preconditions, frame mutation rules, failure classes,
diagnostic marker quarantine, and mandatory QEMU syscall smoke boundary. The
accepted QEMU syscall smoke plan defines the qemu_syscall_smoke invariant,
stable svc #0 talos_nop and unknown-syscall return observations, exact
classification/PASS lines, retained QEMU/substitute evidence, and diagnostic
marker quarantine requirements before implementation. The accepted QEMU syscall
smoke core adds a recoverable lower-AArch64 svc #0 routing boundary, mutates
saved x0 through the target-independent dispatch core, preserves the diagnostic
qemu-el0-trap-smoke proof, and retains QEMU/substitute serial evidence with
classification=qemu-syscall-smoke-complete and qemu-syscall-smoke: PASS. This
does not prove Pi 5 production syscall routing or unblock descriptor I/O,
copy-in/copy-out, process loading, filesystem, shell, networking, or SSH. The
accepted Phase 7.3 syscall routing closeout checkpoint reconciles those commits
and retained logs, closes out only the QEMU/substitute production syscall
routing frontier, and recommends a documentation-only Pi 5 syscall proof plan
before any serialized hardware action or before choosing copy-in/copy-out or
descriptor syscall work. The accepted Pi 5 syscall proof plan defines the
physical invariant for stable svc #0 talos_nop and unknown-syscall return
observations, diagnostic marker 0x7a10 quarantine, hardwareTestLock ownership,
fresh serial/TFTP evidence, candidate identity, inconclusive-run triage,
restoration requirements, and exact PASS/classification lines for the later
hardware proof. It does not acquire hardwareTestLock, publish an archive, run
Pi 5 hardware, or unblock descriptor I/O, copy-in/copy-out, process loading,
filesystem, shell, networking, or SSH.
The serialized Pi 5 syscall proof is now accepted. Retained local3 physical
serial evidence shows stable lower-AArch64 svc #0 reaching the production
syscall dispatch core on Pi 5: talos_nop returns x0 = 0, unknown syscall number
17 returns x0 = 0xffffffffffffffda (-ENOSYS), diagnostic marker 0x7a10 remains
outside production dispatch, and the proof reports
classification=pi5-syscall-proof-complete plus rpi5-syscall-proof: PASS. The
first candidate run was inconclusive, so the accepted evidence includes the
required same-candidate triage: candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, and restore proof for the prior accepted boot tree. This accepts only
physical production routing for the first scalar syscall boundary; descriptor
I/O, copy-in/copy-out, process loading, filesystem, shell, networking, and SSH
remain blocked.
The accepted Pi 5 syscall proof closeout reconciles the accepted syscall ABI,
dispatch core, production trap routing, QEMU routing evidence, Pi 5 hardware
proof, hardware-lock timeline, restore proof, and deferred surfaces. It
accepts no new Rust or assembly behavior and performs no QEMU or Pi 5 rerun.
It recommends the documentation-only copy-in/copy-out helper contract as the
next bounded task before any pointer-taking syscall or descriptor I/O
implementation.
The accepted copy-in/copy-out helper contract defines target-independent
helper inputs, outputs, validation order, EFAULT mapping, all-or-nothing
partial-copy policy, recoverable versus process-fatal fault boundaries, and
unit-testable cases. It names phase7-copyin-copyout-helper-core-20260529 as
the next bounded implementation task and requires supervisor planning before
promotion because the current durable queue names only the contract task.
The accepted copy-in/copy-out helper core adds target-independent
copy_from_user and copy_to_user helpers in src/posix.rs. The helpers validate
the complete user range before byte movement, use UserAccessKind::Read for
copy-in and UserAccessKind::Write for copy-out, return the exact requested
length on success, map user-boundary failures to EFAULT, reserve EINVAL for
malformed kernel-side helper use, and preserve all-or-nothing behavior. Unit
tests cover success, zero-length, null guard, kernel range, wraparound, copy
limit, unmapped gaps, no-access mappings, permission mismatches,
backing-storage gaps, short kernel buffers, and destination preservation. The
copy-in/copy-out helper closeout reconciles this target-independent byte-copy
frontier and recommends phase7-pointer-taking-syscall-source-inventory-20260529
as the next bounded planning task. Pointer-taking syscalls, descriptor I/O,
process loading, filesystem, shell, networking, and SSH remain blocked until
later explicit tasks accept their contracts and gates. The accepted
pointer-taking syscall source inventory maps source owners and gaps for frame
argument extraction, syscall-number allocation, user-memory mapping
provenance, copy helper invocation, return/error encoding, QEMU smoke
ownership, and diagnostic-surface quarantine. It recommends supervisor planning
for phase7-pointer-taking-syscall-contract-20260529 before any implementation
or QEMU pointer-copy smoke plan; phase7-qemu-pointer-copy-smoke-plan-20260529
remains dependency-blocked until that contract is accepted. Descriptor I/O,
process loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy
hardware proof remain blocked.
The accepted pointer-taking syscall contract fixes the first lower-EL
pointer-copy boundary as proof-only and QEMU/substitute scoped:
talos_copy_probe uses stable svc #0 with x8 = 0x7001 only in the later
qemu_pointer_copy_smoke scenario, assigns x0 as user pointer, x1 as length,
x2 as expected byte, x3 as replacement byte, and x4/x5 as reserved zeros, and
defines success, zero-length, -EFAULT, -EINVAL, and -ENOSYS observations. It
uses a fixed QEMU substitute UserData mapping/backing store at
0x0000_0000_0011_0000..0x0000_0000_0011_1000 and keeps diagnostic marker
0x7a10 proof-only. It unblocks only the documentation-only
phase7-qemu-pointer-copy-smoke-plan-20260529 task; descriptor I/O, process
loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy
hardware proof remain blocked.
The accepted QEMU pointer-copy smoke plan defines the
qemu_pointer_copy_smoke QEMU/substitute invariant for proof-only
talos_copy_probe: fixed UserData backing storage, a 16-byte success case that
copies 0x2a bytes in and writes 0xa5 bytes back, a guard-range EFAULT case,
an unknown-syscall -ENOSYS regression, and diagnostic marker quarantine. The
accepted QEMU pointer-copy smoke core implements that boundary with
TALOS_BOOT_SCENARIO=qemu_pointer_copy_smoke, routes x8 = 0x7001 only in that
scenario, invokes the accepted copy_from_user and copy_to_user helpers, keeps
x8 = 0x7001 as -ENOSYS outside the proof scenario, and retains QEMU/substitute
serial evidence at
tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log
with classification=qemu-pointer-copy-smoke-complete and
qemu-pointer-copy-smoke: PASS. This accepts only QEMU/substitute pointer-copy
through lower-EL syscall routing; descriptor I/O, process loading,
VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
remain blocked.
The accepted pointer-copy closeout checkpoint reconciles the contract, smoke
plan, core implementation, retained QEMU evidence, regression gates, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends supervisor planning for a documentation-only
Pi 5 pointer-copy proof plan before any serialized hardware action or before
choosing descriptor syscall work.
The accepted Pi 5 pointer-copy proof plan defines the physical invariant for
proof-only talos_copy_probe on serialized Raspberry Pi 5 hardware: stable
svc #0 with x8 = 0x7001 must prove the 16-byte 0x2a-to-0xa5 success copy,
guard-range -EFAULT, unknown-syscall -ENOSYS, diagnostic marker quarantine,
hardwareTestLock ownership, fresh serial/TFTP evidence, candidate identity,
inconclusive-run triage, restoration proof, and exact
classification=pi5-pointer-copy-proof-complete plus
rpi5-pointer-copy-proof: PASS lines. It does not acquire hardwareTestLock,
publish an archive, run Pi 5 hardware, or unblock descriptor I/O, process
loading, filesystem, shell, networking, or SSH.
The serialized Pi 5 pointer-copy proof is now accepted. Retained local3
physical serial evidence shows stable lower-AArch64 svc #0 reaching the
proof-only talos_copy_probe path on Pi 5: the 16-byte success case returns
x0 = 16 and rewrites UserData from 0x2a to 0xa5, the guard-range request
returns x0 = 0xfffffffffffffff2 (-EFAULT), unknown syscall number 17 returns
x0 = 0xffffffffffffffda (-ENOSYS), diagnostic marker 0x7a10 remains outside
production dispatch, and the proof reports
classification=pi5-pointer-copy-proof-complete plus
rpi5-pointer-copy-proof: PASS. The first candidate run was inconclusive, so
the accepted evidence includes candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, hardwareTestLock release, and restore proof for the prior accepted boot
tree. This accepts only the physical proof-only pointer-copy boundary;
descriptor I/O, process loading, filesystem, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, and stable POSIX descriptor
claims remain blocked.
The accepted Pi 5 pointer-copy proof closeout reconciles the accepted
QEMU/substitute pointer-copy evidence, Pi 5 hardware proof evidence,
hardware-lock timeline, restore proof, proof-only status, residual risks, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends the documentation-only descriptor syscall
source inventory as the next bounded task before any descriptor syscall
contract or implementation.
The accepted descriptor syscall source inventory maps the source owners and
gaps for descriptor table operations, lower-EL syscall argument extraction,
copy helper use, runtime-console/TTY backing, return/error encoding,
task/process ownership, and retained QEMU evidence style. It recommends the
next descriptor syscall contract slice as a stdout/stderr write boundary backed
by runtime-console0, while keeping stdin/read, close, dup, process loading,
VFS/filesystem, shell, networking, SSH, live process-owned address spaces,
blocking/readiness, signals, restart semantics, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and stable POSIX descriptor claims blocked.
The accepted descriptor syscall contract defines the first stable
descriptor-backed syscall slice as talos_write with x8 = 1, fd/user-pointer/
length arguments in x0/x1/x2, reserved zero x3 through x5, copy_from_user
validation, descriptor-table lookup and write-access checks, runtime-console0
as the only backing object, and exact byte-count or negative-errno returns. It
keeps stdin/read, close, dup, process loading, VFS/filesystem, shell,
networking, SSH, live process-owned address spaces, blocking/readiness,
signals, restart semantics, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor claims blocked.
The accepted QEMU descriptor-write smoke plan defines the
qemu_descriptor_write_smoke QEMU/substitute invariant for stable talos_write
x8 = 1: fd 1/fd 2 success through inherited stdio descriptors,
copy_from_user(), and runtime-console0, fd 0 and invalid-fd -EBADF, guard-range
-EFAULT, reserved-register -EINVAL, talos_nop and unknown-syscall regressions,
proof-only talos_copy_probe quarantine, and exact
classification=qemu-descriptor-write-smoke-complete plus
qemu-descriptor-write-smoke: PASS evidence for the later implementation. It
does not add implementation, QEMU, or hardware evidence and keeps stdin/read,
close, dup, process loading, VFS/filesystem, shell, networking, SSH, live
process-owned address spaces, blocking/readiness, signals, restart semantics,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and physical
descriptor-write claims blocked.
The accepted QEMU descriptor-write smoke core adds the
qemu_descriptor_write_smoke lower-AArch64 svc #0 scenario and retained
QEMU/substitute evidence for talos_write x8 = 1. The evidence proves fd 1 and
fd 2 write 18-byte UserData buffers through inherited stdio descriptors,
copy_from_user(), and runtime-console0; fd 0 and fd 99 return -EBADF without
additional console bytes; the guard range returns -EFAULT without console
bytes; a nonzero reserved register returns -EINVAL without console bytes;
talos_nop and unknown-syscall regressions remain intact; x8 = 0x7001 remains
quarantined as -ENOSYS outside proof scenarios; and the diagnostic marker
0x7a10 remains proof-only. This accepts only QEMU/substitute descriptor-backed
stdout/stderr write evidence. Pi 5 descriptor-write hardware proof,
stdin/read, close, dup, process loading, VFS/filesystem, shell, networking,
SSH, live process-owned address spaces, blocking/readiness, signals, restart
semantics, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and
full POSIX descriptor claims remain blocked.
The accepted descriptor-write closeout checkpoint reconciles the source
inventory, talos_write contract, smoke plan, descriptor-write core,
retained QEMU evidence, scalar/pointer-copy regression gates, residual risks,
and deferred surfaces. It recommends a documentation-only
phase7-pi5-descriptor-write-proof-plan-20260529 before any serialized Pi 5
descriptor-write hardware action, and keeps stdin/read, close, dup, process
loading, VFS/filesystem, shell, networking, SSH, live process-owned address
spaces, blocking/readiness, signals, restart semantics, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, physical descriptor-write claims,
and full POSIX descriptor claims blocked.
The accepted Pi 5 descriptor-write proof plan defines the physical invariant
for talos_write fd 1/fd 2 on serialized Raspberry Pi 5 hardware: stable svc #0
with x8 = 1 must write the stdout and stderr buffers through copy_from_user(),
inherited stdio descriptors, and runtime-console0; fd 0 and fd 99 must return
-EBADF; guard-range writes must return -EFAULT; nonzero reserved registers
must return -EINVAL; talos_nop and unknown-syscall regressions must remain
intact; talos_copy_probe x8 = 0x7001 and diagnostic marker 0x7a10 must remain
quarantined; and the proof must report
classification=pi5-descriptor-write-proof-complete plus
rpi5-descriptor-write-proof: PASS. It does not acquire hardwareTestLock,
publish an archive, run Pi 5 hardware, or unblock stdin/read, close, dup,
process loading, filesystem, shell, networking, or SSH.
The serialized Pi 5 descriptor-write proof is now accepted. Retained local3
physical serial evidence shows stable lower-AArch64 svc #0 reaching the
descriptor-write dispatch path on Pi 5: fd 1 stdout and fd 2 stderr write
18-byte UserData buffers through copy_from_user(), inherited stdio
descriptors, and runtime-console0; fd 0 and fd 99 return -EBADF without extra
console bytes; the guard range returns -EFAULT; a nonzero reserved register
returns -EINVAL; talos_nop and unknown-syscall regressions remain intact; x8 =
0x7001 remains quarantined as -ENOSYS; diagnostic marker 0x7a10 remains
proof-only; and the proof reports
classification=pi5-descriptor-write-proof-complete plus
rpi5-descriptor-write-proof: PASS. The first candidate run was inconclusive,
so the accepted evidence includes candidate identity, fresh serial and TFTP
cursors, a passing production-timer known-good control, an unchanged candidate
rerun, hardwareTestLock release, and restore proof for the prior accepted boot
tree. This accepts only the physical descriptor-backed stdout/stderr write
boundary; stdin/read, close, dup, process loading, filesystem, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor claims remain blocked.
The accepted Pi 5 descriptor-write proof closeout reconciles the accepted
QEMU/substitute descriptor-write smoke, physical Pi 5 descriptor-write proof,
hardware-lock timeline, restore proof, residual risks, and blocked surfaces. It
accepts no new Rust or assembly behavior and performs no QEMU or Pi 5 rerun.
It recommends the documentation-only Milestone 7.3 syscall ABI/dispatch
closeout checkpoint before any Milestone 7.4 file-descriptor-table source
inventory or broader descriptor work.
The accepted Milestone 7.3 syscall ABI/dispatch closeout reconciles scalar
syscall routing, QEMU and Pi 5 syscall proof, copy-in/copy-out helpers,
proof-only pointer-copy evidence, descriptor-write QEMU/Pi 5 evidence,
diagnostic-surface quarantine, hardware-lock/restore proof, residual risks, and
blocked surfaces. Milestone 7.3 is closed for the bounded lower-AArch64 svc #0
ABI and dispatch frontier: x8 syscall numbers, x0-through-x5 arguments, x0
return/-errno encoding, talos_nop, unknown-syscall -ENOSYS, copy helper
plumbing, proof-only talos_copy_probe, and talos_write fd 1/fd 2 to
runtime-console0. Process-owned descriptors, stdin/read, close, dup, program
loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.
The accepted Milestone 7.4 file descriptor table source inventory maps the
source owners, accepted contracts, retained evidence, and missing contract
boundaries for moving from proof-owned inherited stdio descriptors to
process-owned descriptor tables. It recommends a documentation-only
phase7-process-descriptor-table-contract-20260529 as the next bounded task and
keeps stdin/read, close/dup syscalls, VFS/filesystem, path copying, process
loading, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor claims blocked.
The accepted process descriptor table contract defines the first
process-owned descriptor table boundary: a ProcessOwnerId-backed owner record,
inherited fd 0/fd 1/fd 2 installation, runtime-console0-backed stdout/stderr
identity, current-process descriptor-table lookup, and deterministic retained
descriptor errors. It recommends phase7-process-descriptor-table-core-20260529
as the next target-independent implementation task and keeps PID allocation,
process loading, close/dup/read syscalls, VFS/filesystem, stdin behavior,
shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, physical proof, and full POSIX descriptor claims
blocked.
The accepted process descriptor table core adds the first target-independent
process-owned descriptor owner/store surface. A ProcessOwnerId can now own one
inherited-stdio DescriptorTable in a bounded ProcessDescriptorStore, current
owner lookup maps missing current task/owner/table state to -EBADF for
descriptor syscalls, and focused unit tests preserve inherited stdio plus
retained descriptor-table errors. It adds no live syscall routing, QEMU or
Pi 5 proof, close/dup/read syscall behavior, process loading,
VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, or full POSIX descriptor claim.
The accepted QEMU process descriptor stdio smoke plan defines the next
QEMU/substitute proof boundary: lower-AArch64 talos_write fd 1/fd 2 must route
through a ProcessOwnerId-backed ProcessDescriptorStore and inherited stdio
DescriptorTable, with current-owner lookup evidence, retained fd/error
regressions, talos_nop and unknown-syscall regressions, proof-only
talos_copy_probe quarantine, diagnostic marker quarantine, exact
classification/PASS lines, and retained QEMU log path. It adds no
implementation, QEMU run, Pi 5 hardware action, or hardware-lock work and
keeps close/dup/read, process loading, VFS/filesystem, shell, networking, SSH,
physical proof, and full POSIX descriptor claims blocked.
The accepted QEMU process descriptor stdio smoke core adds the first
lower-AArch64 QEMU/substitute evidence for process-owned descriptor-table
lookup. The qemu_process_descriptor_stdio_smoke scenario creates
ProcessOwnerId 1, installs inherited stdio in ProcessDescriptorStore, resolves
the current owner through the accepted lookup API, routes talos_write fd 1/fd
2 to runtime-console0 through that table, and retains fd/error, scalar
syscall, copy-probe quarantine, diagnostic-marker quarantine, and PASS
evidence at
tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log.
It remains QEMU/substitute evidence only; Pi 5 physical proof, stdin/read,
close/dup/read, process loading, VFS/filesystem, shell, networking, SSH, and
full POSIX descriptor claims remain blocked.
The accepted process descriptor table closeout reconciles the contract,
target-independent core, QEMU/substitute smoke plan, retained smoke evidence,
validation gates, residual risks, and blocked surfaces for the first
process-owned descriptor-table slice. It accepts ProcessOwnerId-backed
inherited stdio lookup and talos_write fd 1/fd 2 routing to runtime-console0
only at the QEMU/substitute evidence level. Pi 5 physical descriptor-table
proof, stdin/read, close/dup/read, descriptor lifetime and close semantics,
process loading, VFS/filesystem, shell, networking, SSH, and full POSIX
descriptor claims remain blocked. The next bounded Milestone 7.4 task should
be supervisor-planned as a documentation-only descriptor lifetime and
close-semantics source inventory before any close/dup/read syscall contract.
The accepted descriptor lifetime and close source inventory maps
DescriptorTable table-local close/dup primitives, DescriptorEntry/Object/Access
vocabulary, ProcessDescriptorStore owner-table mutation, inherited stdio
lifetime, accepted unit-test evidence, missing close/double-close/reuse/dup
evidence, and owner teardown/reference-count gaps. It recommends the
documentation-only phase7-descriptor-lifetime-close-contract-20260529 as the
next bounded Milestone 7.4 task. Close/dup/read syscalls, process loading,
VFS/filesystem, shell, networking, SSH, Pi 5 physical close/dup/read claims,
and full POSIX descriptor readiness remain blocked.
The accepted descriptor lifetime and close contract defines table-local slot
removal, process-owned mutable lookup through ProcessDescriptorStore, EBADF
error cases, dup/reuse interaction, and open-file-description reference-count
vocabulary. It recommends phase7-descriptor-close-core-20260529 as the next
target-independent Milestone 7.4 implementation task. Close/dup/read syscalls,
process loading, VFS/filesystem, shell, networking, SSH, Pi 5 physical
close/dup/read claims, object finalization, and full POSIX descriptor
readiness remain blocked.
The accepted descriptor close core closeout reconciles the accepted source
inventory, contract, target-independent implementation, focused unit-test
evidence, validation gates, and deferred surfaces for process-owned descriptor
close semantics. It accepts only ProcessDescriptorStore::close_current_descriptor()
applying table-local DescriptorTable::close() to the current owner, with EBADF
for missing/unknown owners and invalid, empty, or already closed descriptors.
Close/dup/read syscalls, lower-EL ABI, QEMU close/dup/read smoke, Pi 5 physical
close/dup/read proof, process loading, VFS/filesystem, shell, networking, SSH,
object finalization, and full POSIX descriptor readiness remain blocked. The
next bounded Milestone 7.4 task should be supervisor-planned as a
documentation-only close/dup/read syscall source inventory.
The accepted close/dup/read syscall source inventory maps the current syscall
dispatch, lower-EL routing, copy helper, ProcessDescriptorStore, DescriptorTable,
descriptor entry/object, runtime-console0, TTY, and stdin/read source owners.
It separates accepted process-owned descriptor-write and target-independent
close/dup/copy-helper evidence from unproven close, dup, and read syscalls.
Close is the smallest next user-visible descriptor operation because the
target-independent close helper is already accepted; dup and read still need
additional policy around fd allocation, read byte sources, EOF,
blocking/readiness, nonblocking behavior, signal/restart policy, and object
lifetime. The next bounded Milestone 7.4 task should be
phase7-close-syscall-contract-20260529. Dup/read, QEMU/Pi 5 close/dup/read
proof, process loading, VFS/filesystem, shell, networking, SSH, object
finalization, and full POSIX descriptor readiness remain blocked.
The accepted close syscall contract defines the first user-visible descriptor
close boundary: stable svc #0 with x8 = 2, descriptor argument in x0,
reserved-zero x1 through x5, x0 = 0 on success, -EBADF for missing/unknown
owners and invalid, empty, or already closed descriptors, and -EINVAL for
nonzero reserved arguments. The contract routes the later implementation
through ProcessDescriptorStore::close_current_descriptor() and preserves
talos_nop, talos_write, unknown-syscall, descriptor-write, and proof-only
pointer-copy quarantine behavior. The next bounded Milestone 7.4 task should
be phase7-close-syscall-core-20260529. Dup/read, QEMU/Pi 5 close/dup/read
proof, process loading, VFS/filesystem, shell, networking, SSH,
open-file-description finalization, and full POSIX descriptor readiness remain
blocked.
The accepted close syscall core adds stable syscall number 2 for talos_close
and a target-independent process descriptor dispatch helper. Close validates
reserved-zero x1 through x5, resolves the current process owner through
ProcessDescriptorStore, clears occupied descriptor slots with x0 = 0, returns
-EBADF for missing/unknown owner or invalid/empty/already-closed descriptor
cases, and returns -EINVAL for reserved-register violations. Focused no_std
tests prove stdout/stderr close, EBADF failures, no-mutation EINVAL, duplicate
slot preservation, and talos_write regression after close. The QEMU syscall
and descriptor-write smokes remain passing regressions. The accepted QEMU close
syscall smoke plan then fixed the lower-AArch64 QEMU/substitute invariant for
closing fd 1 and fd 2 through the current ProcessOwnerId-backed descriptor
table. The accepted QEMU close syscall smoke core retains that evidence:
qemu_close_syscall_smoke closes fd 1/fd 2 through
ProcessDescriptorStore::close_current_descriptor(), proves later talos_write on
closed descriptors returns -EBADF without runtime-console0 side effects, proves
fd 2 remains writable after fd 1 is closed and after a failed reserved close,
and preserves talos_nop, unknown-syscall, copy-probe quarantine, and
diagnostic-marker quarantine. Dup/read, Pi 5 physical close/dup/read proof,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
and full POSIX descriptor readiness remain blocked. The next bounded Milestone
7.4 task should be phase7-close-syscall-closeout-checkpoint-20260529.
The accepted close syscall closeout reconciles the accepted source inventory,
contract, target-independent core, QEMU close smoke plan, retained
QEMU/substitute close evidence, validation gates, and deferred surfaces. It
accepts only the current ProcessOwnerId-backed talos_close QEMU/substitute
frontier and does not add Rust behavior, QEMU rerun, Pi 5 hardware run, or
hardwareTestLock activity. Pi 5 physical close proof, dup/read syscalls,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
and full POSIX descriptor readiness remain blocked. The next bounded Milestone
7.4 task should be a documentation-only Pi 5 close syscall proof plan before
any serialized hardware action.
The accepted Pi 5 close syscall proof plan defines the serialized physical
rpi5_close_syscall_proof boundary for carrying the QEMU/substitute talos_close
invariant to hardware. It requires hardwareTestLock ownership, candidate
archive and kernel identity, fresh serial and TFTP evidence, restoration proof,
and exact observations for close(fd 1), close(fd 2), write-after-close -EBADF,
reserved-argument -EINVAL no-mutation, repeated/invalid close -EBADF,
talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker
quarantine, final classification, and PASS. This plan does not run hardware or
accept a physical close claim. The next bounded Milestone 7.4 task should be
phase7-pi5-close-syscall-proof-20260529. Dup/read, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, and full POSIX
descriptor readiness remain blocked.
The serialized Pi 5 close syscall proof is now accepted. Retained local19
hardware evidence carries the QEMU/substitute talos_close invariant to
Raspberry Pi 5: the focused rpi5_close_syscall_proof payload closes fd 1 and
fd 2 through the current ProcessOwnerId-backed ProcessDescriptorStore,
proves write-after-close returns -EBADF before runtime-console0 side effects,
preserves reserved-argument -EINVAL no-mutation, repeated/invalid close
-EBADF, talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic
marker quarantine, final classification=pi5-close-syscall-proof-complete, and
PASS. The physical fix cleans the initialized ProcessDescriptorStore static to
PoC before the EL2-to-EL1/EL0 proof handoff; local18 showed the pre-fix EL1
handler saw owner-present=false. This accepts only the physical talos_close
proof. Dup/read, process loading, VFS/filesystem, shell, networking, SSH,
object finalization, and full POSIX descriptor readiness remain blocked. The
next bounded Milestone 7.4 task should be
phase7-pi5-close-syscall-proof-closeout-checkpoint-20260529.
The accepted Pi 5 close syscall proof closeout reconciles the close syscall
source inventory, contract, target-independent core, QEMU/substitute close
smoke, serialized Pi 5 physical proof, hardware-lock timeline, restore proof,
and deferred surfaces. It accepts only the physical talos_close proof for the
focused rpi5_close_syscall_proof scenario. Dup/read, process loading,
VFS/filesystem, stdin/read object policy, shell, networking, SSH, object
finalization, broader cache/DMA policy, and full POSIX descriptor readiness
remain blocked. The next bounded Milestone 7.4 task should be the already
queued documentation-only phase7-dup-syscall-contract-20260529.
The accepted dup syscall contract defines talos_dup as stable syscall number 3
on svc #0 with x0 as the source descriptor and x1 through x5 reserved zero. It
duplicates an occupied descriptor in the current ProcessOwnerId-backed
ProcessDescriptorStore into the lowest free slot, returns the new descriptor
number, maps invalid/empty/closed or missing-owner sources to -EBADF, maps a
full table to -EMFILE, and maps reserved arguments to -EINVAL without
mutation. The contract preserves talos_nop, talos_write, talos_close,
unknown-syscall, and proof-only copy-probe behavior, and recommends
phase7-dup-syscall-core-20260529 as the next bounded target-independent
implementation task. Read syscall behavior, stdin/read object policy, QEMU/Pi
5 dup/read proof, process loading, VFS/filesystem, shell, networking, SSH,
dup2/fcntl, object finalization, broader cache/DMA policy, and full POSIX
descriptor readiness remain blocked.
The accepted dup syscall core adds stable syscall number 3 for talos_dup and
routes it through dispatch_process_descriptor() and
ProcessDescriptorStore::dup_current_descriptor(). It validates reserved-zero x1
through x5, duplicates occupied source descriptors into the lowest free slot,
returns -EBADF for invalid, empty, closed, missing-owner, or unknown-owner
sources, returns -EMFILE for full tables, and returns -EINVAL for reserved
argument violations without mutation. Descriptor writes now rely on the copied
DescriptorEntry access and StdioOutput object kind, so duplicated stdout/stderr
descriptors can remain writable after the source is closed while stdin/read
behavior stays blocked. Focused no_std tests prove stdout/stderr/stdin
duplication cases, duplicate/source independence across close, no-mutation
reserved failures, full-table EMFILE, and existing nop/write/close/unknown
regressions. QEMU dup smoke, Pi 5 physical dup proof, read/stdin behavior,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
dup2/fcntl, and full POSIX descriptor readiness remain blocked. The next
bounded Milestone 7.4 task should be
phase7-qemu-dup-syscall-smoke-plan-20260529.
The accepted QEMU dup syscall smoke plan defines the bounded
qemu_dup_syscall_smoke substitute proof before lower-EL runtime evidence is
claimed. It requires a ProcessOwnerId-backed four-slot inherited stdio table,
current-owner lookup through ProcessDescriptorStore, talos_dup(fd 1) returning
fd 3, deterministic -EMFILE and -EINVAL cases, writes through source and
duplicate stdout descriptors, close-one-descriptor preservation, closed
descriptor -EBADF, talos_nop, unknown-syscall -ENOSYS, copy-probe quarantine,
diagnostic-marker quarantine, final
classification=qemu-dup-syscall-smoke-complete, and PASS. It does not run QEMU
or hardware. Pi 5 physical dup proof, read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, and full POSIX
descriptor readiness remain blocked. The next bounded Milestone 7.4 task
should be phase7-qemu-dup-syscall-smoke-core-20260529.
The accepted QEMU dup syscall smoke core adds qemu_dup_syscall_smoke and
retains lower-AArch64 QEMU/substitute evidence at
tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
The smoke creates ProcessOwnerId 1 with a four-slot inherited stdio table,
routes talos_dup through the current ProcessDescriptorStore lookup, proves fd
1 duplicates to fd 3, table-full -EMFILE, reserved-register -EINVAL,
runtime-console0 writes through both source and duplicate descriptors,
close(fd 1) preserving fd 3, closed-descriptor -EBADF, talos_nop,
unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker quarantine,
final classification=qemu-dup-syscall-smoke-complete, and PASS. It is
QEMU/substitute evidence only. Pi 5 physical dup proof, read/stdin behavior,
process loading, VFS/filesystem, shell, networking, SSH, object finalization,
dup2/fcntl, and full POSIX descriptor readiness remain blocked. The next
bounded Milestone 7.4 task should be
phase7-dup-syscall-closeout-checkpoint-20260529.
The accepted QEMU dup syscall closeout checkpoint reconciles the dup contract,
target-independent core, QEMU smoke plan, retained QEMU/substitute dup
evidence, descriptor-write and close regression gates, residual risks, and
deferred surfaces. It accepts no new Rust or assembly behavior and performs no
QEMU or Pi 5 rerun. It recommends the already queued documentation-only
phase7-pi5-dup-syscall-proof-plan-20260529 before any serialized physical dup
proof. Pi 5 physical dup proof, read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl, and
full POSIX descriptor readiness remain blocked.
The accepted Pi 5 dup syscall proof plan defined the serialized physical
rpi5_dup_syscall_proof invariant, including hardwareTestLock ownership,
candidate identity, fresh serial/TFTP evidence, inconclusive-run triage,
restore proof, exact dup/write/close/error/quarantine/PASS lines, and blocked
deferred surfaces. The serialized Pi 5 dup syscall proof is now accepted.
Retained local8 physical evidence proves fd 1 duplicates to fd 3,
full-table -EMFILE, reserved-register -EINVAL, writes through source and
duplicate stdout descriptors, close(fd 1) preserving fd 3, duplicate close,
closed-descriptor -EBADF, scalar and unknown-syscall regressions, copy-probe
quarantine, final
classification=pi5-dup-syscall-proof-complete, and PASS. Retained local7
production-timer control evidence proves lab health after the earlier
inconclusive candidate/control runs. Read/stdin behavior, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl, and
full POSIX descriptor readiness remain blocked.
The accepted Pi 5 dup syscall proof closeout reconciles the QEMU and Pi 5 dup
frontier, retained local7/local8 evidence, archive/TFTP identity, restore
proof, hardware-lock timeline, residual risks, and deferred surfaces. The
accepted read/stdin source inventory maps the current owners for syscall
dispatch, copy_to_user/user-memory validation, ProcessDescriptorStore lookup,
inherited fd 0, runtime-console0, TTY/stdin surfaces, and retained
write/close/dup evidence. It lists the unresolved read/stdin policy gaps for
byte source, EOF, blocking/readiness, partial reads, nonblocking mode,
restart/signals, copy-out failure handling, object lifetime/finalization, and
physical proof. It accepts no read behavior and recommends the documentation-only
phase7-read-stdin-contract-20260529 as the next bounded Milestone 7.4 task.
The accepted read/stdin contract defines talos_read as stable syscall number 4
with x0 fd, x1 destination pointer, x2 requested count, x3 through x5 reserved
zero, and x0 byte-count/0 EOF or negative errno return encoding. The first
bounded stdin source is fixed proof input shared by fd 0 and duplicates of
fd 0, with immediate readiness, proof-buffer short reads, 0 at bounded EOF,
copy_to_user all-or-nothing failure ordering, and no runtime-console0, TTY,
filesystem, pipe, socket, signal, wait-queue, or hardware input claim. It
recommends phase7-read-stdin-core-20260529 as the next bounded
target-independent implementation task.
The accepted read/stdin core adds stable syscall number 4 for talos_read,
`FixedStdin` proof-buffer state, target-independent descriptor dispatch through
`ProcessDescriptorStore`, and focused no_std coverage for fd 0 and duplicated
stdin reads, proof-buffer short reads, 0 EOF, reserved-register -EINVAL,
copy-out -EFAULT without cursor advance, fd/error -EBADF, non-stdin
readable-object -ENOTSUP, missing fixed source -ENOTSUP, and
scalar/write/close/dup/unknown/copy-probe regressions. This is
target-independent implementation
evidence only. QEMU/substitute lower-AArch64 read evidence, Pi 5 physical read
proof, runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem,
shell, networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The accepted QEMU
read/stdin smoke plan defines the qemu_read_stdin_smoke QEMU/substitute
invariant for stable talos_read x8 = 4: fd 0 duplication, fixed proof stdin
length/cursor validation, copy-out -EFAULT, reserved-register -EINVAL,
fd/error -EBADF, fd 0 success copying talos, duplicated-fd short read copying
-stdin-qemu\n, bounded EOF, talos_nop and unknown-syscall regressions,
copy-probe quarantine, diagnostic-marker quarantine, and exact
classification=qemu-read-stdin-smoke-complete plus PASS lines. It does not run
QEMU or hardware. Pi 5 physical read proof, runtime-console0/TTY/hardware
stdin, process loading, VFS/filesystem, shell, networking, SSH, object
finalization, dup2/fcntl, signals, wait queues, nonblocking I/O, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
readiness remain blocked. The next bounded Milestone 7.4 task should be
phase7-qemu-read-stdin-smoke-core-20260529.
The accepted QEMU read/stdin smoke core adds qemu_read_stdin_smoke and
retained QEMU/substitute evidence for lower-AArch64 stable talos_read x8 = 4.
It proves current-owner ProcessDescriptorStore lookup, fd 0 duplication to
fd 3, fixed proof stdin bytes talos-stdin-qemu\n, copy-out -EFAULT without
cursor advance, reserved-register -EINVAL without mutation, fd/error -EBADF,
fd 0 read success copying talos, duplicated-fd short read copying
-stdin-qemu\n, bounded EOF, talos_nop and unknown-syscall regressions,
copy-probe quarantine, diagnostic-marker quarantine, and
classification=qemu-read-stdin-smoke-complete plus PASS. This is
QEMU/substitute evidence only. Pi 5 physical read proof,
runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The next bounded
Milestone 7.4 task should be
phase7-read-stdin-closeout-checkpoint-20260529.
The accepted read/stdin closeout checkpoint reconciles the source inventory,
contract, target-independent core, QEMU/substitute smoke plan, retained
qemu_read_stdin_smoke evidence, regression gates, residual risks, and
deferred surfaces. It keeps accepted behavior bounded to fixed proof stdin on
the target-independent and QEMU/substitute lower-AArch64 paths, with retained
classification=qemu-read-stdin-smoke-complete plus PASS evidence. Pi 5
physical read proof, runtime-console0/TTY/hardware stdin, process loading,
VFS/filesystem, shell, networking, SSH, object finalization, dup2/fcntl,
signals, wait queues, nonblocking I/O, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness remain blocked.
The next mechanically derivable task should be the documentation-only
phase7-pi5-read-stdin-proof-plan-20260530, queued by the supervisor before any
hardware action.
The accepted Pi 5 read/stdin proof plan and serialized proof carry the
QEMU/substitute fixed-stdin talos_read invariant to Raspberry Pi 5 hardware.
Retained local5 evidence ties the unchanged fd2be8e candidate archive to a
114816-byte da591740/kernel_2712.img TFTP fetch, serial output proving fd 0
read, duplicated fd 3 short read, EOF, -EFAULT/-EINVAL/-EBADF error cases,
talos_nop and unknown-syscall regressions, copy-probe quarantine,
diagnostic-marker quarantine, final
classification=pi5-read-stdin-proof-complete, and PASS, followed by restore
proof for the prior accepted 104136-byte boot tree. The accepted claim remains
limited to fixed proof stdin in the focused rpi5_read_stdin_proof scenario.
runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked.
The accepted Pi 5 read/stdin proof closeout reconciles the QEMU/substitute
read/stdin evidence, Pi 5 hardware proof evidence, hardware-lock timeline,
restore proof, fixed-proof-stdin status, residual risks, and deferred surfaces.
It accepts no new Rust or assembly behavior and performs no QEMU or Pi 5
rerun. The next mechanically derivable Milestone 7.4 task is the already queued
phase7-file-descriptor-table-closeout-checkpoint-20260530, and no Phase 8
transition is claimed by this closeout.
The accepted Milestone 7.4 file descriptor table closeout reconciles
process-owned inherited stdio, descriptor-backed stdout/stderr writes,
descriptor lifetime/close semantics, stable talos_close, stable talos_dup,
fixed-proof-stdin talos_read, QEMU/substitute evidence, serialized Pi 5
physical evidence, hardware-lock/restore records, residual risks, and blocked
surfaces. Milestone 7.4 is closed only for the bounded descriptor-table
frontier: ProcessOwnerId-backed inherited stdio, runtime-console0-backed fd 1
and fd 2 writes, close, dup, fixed proof stdin through fd 0/fd 3, scalar
regressions, and diagnostic-surface quarantine. runtime-console0/TTY/hardware
stdin, pipes, sockets, regular files, VFS/filesystem, process loading, shell,
networking, SSH, object finalization, dup2/fcntl, signals, wait queues,
nonblocking I/O, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor readiness remain blocked. The next objective task
should be a supervisor-planned Phase 7 final closeout or frontier checkpoint
before any Phase 8 transition is considered.
The accepted Phase 7 final frontier source inventory reconciles the accepted
Phase 7.1 POSIX baseline, Phase 7.2 lower-EL/address-space proof, Phase 7.3
syscall/copy boundary, and Phase 7.4 descriptor-table work by commit and
evidence level. It identifies no remaining bounded Phase 7 implementation or
evidence task before the final closeout checkpoint, but it does not set a
Phase 8 transition flag or accept filesystem/program-loading behavior. The next
mechanically unblocked queued task is
phase7-final-closeout-checkpoint-20260530.
The accepted Phase 7 final closeout checkpoint closes Phase 7 for that bounded
frontier and records the durable recommendation flag for the first Phase 8
source-inventory task. It does not implement or accept filesystem/program
loading, shell, networking, SSH, runtime-console0/TTY or hardware stdin,
object finalization, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, or full POSIX readiness.

Near-term direction after the accepted Phase 7 closeout:

- Start with the accepted Phase 8 filesystem/program-loading source inventory
  as the source-owner and gap map for Milestone 8.1.
- The next recommended task is the documentation-only
  phase8-readonly-initramfs-vfs-contract-20260530 contract. It should define
  the read-only initial filesystem/VFS boundary before ELF/program loading or
  shell work.
- Keep QEMU, host-side unit tests, and static documentation gates first. Reserve
  serialized Pi 5 runs for the smallest physical claim that cannot be proven on
  the QEMU/substitute path.
- Preserve the deferred-surface boundary: runtime-console0/TTY/hardware stdin,
  process loading, descriptor I/O beyond the accepted write/close/dup/read
  frontiers, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, object finalization, dup2/fcntl, signals, wait queues,
  nonblocking I/O, and DMA/cache-driver policy remain out of scope until
  explicit tasks accept their contracts and gates.
- Treat the roadmap target as a usable local operating system: TTY, shell,
  separate user programs, and interaction/program-based tests that exercise new
  kernel features through the normal kernel/userspace boundary.

The recently accepted Phase 6.3 scheduler frontier includes
evidence-retention, diagnostic-surface, roadmap-refresh,
productionization-boundary, CPU-local scheduler service boundary,
CPU-local scheduler service core, CPU-local scheduler service closeout,
secondary scheduler service-loop source inventory, service-loop core, QEMU
smoke, Pi 5 proof, service-loop closeout, shared run-queue/migration source
inventory, contract, core, QEMU proof, Pi 5 proof, closeout,
load-balancing source inventory, load-balancing policy contract,
load-balancing core, QEMU load-balancing proof, Pi 5 load-balancing proof,
load-balancing closeout, multi-core preemption source inventory, contract,
core, QEMU proof, Pi 5 proof, closeout, production scheduler runtime source
inventory, production timer/preemption contract, production timer/preemption
core, QEMU production timer/preemption proof, Pi 5 production
timer/preemption proof, and production scheduler runtime closeout tasks are
accepted. The accepted
shared
run-queue/migration slice provides a bounded source-owner publish and
destination-owner consume path with explicit migration states and deterministic
errors; QEMU reports
classification=qemu-shared-runqueue-migration-complete, and serialized Pi 5
hardware reports all four physical-core participants completing the named
invariant with
classification=pi5-shared-runqueue-migration-complete. The accepted
load-balancing source inventory identifies policy inputs, freshness checks,
failure modes, and the split between target selection, fairness/affinity,
remote reschedule notification, and the existing migration mechanism. The
accepted load-balancing policy contract keeps the first policy deterministic,
runnable-only, SharedRunQueue-backed, and polling-only unless a later task
adds remote reschedule notification. The accepted load-balancing core adds
target-independent front-runnable selection and publication through
SharedRunQueue with deterministic unit-tested rejection paths. QEMU
load-balancing proof is accepted with
classification=qemu-load-balancing-smoke-complete, proving deterministic
front-runnable selection, source-local removal, shared handoff,
destination-local enqueue, metadata refresh, and PASS. Serialized Pi 5
load-balancing proof is accepted with
classification=pi5-load-balancing-complete for the same named invariant.
The accepted load-balancing closeout preserves those retained gates and
recommends multi-core preemption source inventory as the next bounded
Phase 6.3 task. The accepted multi-core preemption source inventory maps the
timer IRQ, owner-local scheduler service, secondary service-loop, IPI/wake,
metadata, SharedRunQueue, and load-balancing boundaries that the next contract
must preserve. The accepted multi-core preemption contract keeps timer/IPI
paths as bounded recorders and requires owner-local normal control flow to
perform scheduler mutation after interrupt return; it names deterministic
defer/reject outcomes for stale metadata, wrong-owner access,
nested/preemption-disabled sections, pending remote wake, and full queues.
The accepted multi-core preemption core adds target-independent per-owner
pending timer-preemption state, duplicate request coalescing, explicit nested
preemption-disable defer behavior, and an owner-local service entry that
preflights owner/current-task authority before draining wake queues or
mutating scheduler state. The accepted QEMU multi-core preemption proof adds
qemu_multicore_preemption_smoke and
scripts/qemu-multicore-preemption-smoke.sh; logical CPUs 1, 2, and 3 each
record only local pending timer-preemption state, prove the record step does
not mutate scheduler state, then service the request through owner-local normal
control flow with classification=qemu-multicore-preemption-smoke-complete. Pi
5 proof reports classification=pi5-multicore-preemption-complete,
participants=3, expected=3, errors=0, and PASS for the same invariant. The
accepted multi-core preemption closeout preserves the retained gates and
requires a new bounded productionization task before any further scheduler
runtime integration or Phase 7 work. The accepted production scheduler
runtime source inventory maps those retained diagnostic surfaces against the
normal boot, timer, and owner-local runtime paths. The accepted production
timer/preemption contract, core, focused QEMU proof, serialized Pi 5 proof,
and closeout checkpoint establish the first production timer IRQ recording
and owner-local service boundary. Normal QEMU and Pi 5 timer IRQ handlers now
record bounded local production preemption state, and
ProductionSchedulerRuntime services pending preemption only from owner-local
normal control flow. The Pi 5 proof reports
classification=pi5-production-timer-preemption-complete, participants=3,
expected=3, errors=0, and PASS. The accepted Phase 7 POSIX contract source
inventory maps the scheduler task/process boundary, runtime-console and TTY
stdio direction, diagnostic command limitations, lower-EL readiness limits,
and retained validation gates that constrain the POSIX baseline contract. The
accepted Phase 7 POSIX contract baseline defines the first errno-style names,
lexical path normalization semantics, process lifetime vocabulary,
descriptor-operation vocabulary, stdio inheritance shape, early loader
argument/environment vocabulary, and target-independent test seams. The
accepted Phase 7 path/error model core implements the first no_std lexical path
normalizer and PosixError vocabulary. The accepted descriptor-table contract
keeps descriptors process-local, separates descriptor entries from underlying
kernel objects, fixes close/dup and inherited stdio edge cases, and blocks
runtime console/TTY I/O integration until a later explicit task. The accepted
descriptor-table core adds the first fixed-capacity process-local descriptor
table data model with inherited stdio entries, allocation, exact-slot
allocation, lookup, close, dup, access checks, reserved object kinds, and
deterministic PosixError results, all covered by target-independent no_std unit
tests. The accepted Phase 7.1 closeout checkpoint reconciles this evidence and
keeps EL0, syscall, VFS, filesystem, program-loader, descriptor I/O,
networking, SSH, and shell work deferred for supervisor-planned tasks.
The accepted Phase 7.2 EL0/address-space source inventory maps exception
vectors and saved frames, same-EL ERET diagnostics, the broad EL2 identity map,
page-frame ownership, scheduler task/process separation, PosixError/EFAULT
vocabulary, descriptor-table ownership, retained gates, and implementation gaps
before a lower-EL contract. The accepted Phase 7.2 EL0 trap/address-space
contract defines the canonical user range below 0x0000_8000_0000_0000, null
guard, user text/data/heap/stack/guard vocabulary, kernel-only mapping policy,
validated user trap-return frame requirements, user fault classes, and
copy-in/copy-out preconditions. The accepted user-memory permission core adds
target-independent user range, mapping permission, and access validation with
unit coverage for null, wraparound, kernel-range, guard, unmapped, permission,
and length-limit cases. The accepted QEMU EL0 trap smoke plan fixes the first
QEMU-only lower-EL proof invariant and expected output:
classification=qemu-el0-trap-smoke-complete and qemu-el0-trap-smoke: PASS
after a built-in EL0 payload executes diagnostic SVC marker 0x7a10 and the
kernel reports saved user state. The accepted QEMU EL0 trap smoke core
implements that bounded scenario, retaining serial evidence at
tasks/evidence/2026-05-28-qemu-el0-trap-smoke-core/qemu-el0-trap-smoke.txt
with the saved lower-AArch64 synchronous trap state, final classification, and
PASS. The serialized Pi 5 EL0 trap proof is also accepted: retained physical
serial evidence in
tasks/evidence/2026-05-28-pi5-el0-trap-proof/local62-clean-final-lower-el0-trap/
reports the Pi 5 translation feature registers, regular VBAR_EL1 handoff,
lower-AArch64 synchronous SVC trap state, classification=pi5-el0-trap-proof-complete,
and rpi5-el0-trap-proof: PASS. This proves the bounded lower-EL trap path on
hardware only; general syscall ABI, process loading, descriptor I/O,
filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain deferred. The accepted Phase 7.3 syscall ABI
source inventory maps SVC exception decoding, proof-marker boundaries, syscall
number and argument register gaps, return/error convention gaps, user-copy
preconditions, descriptor-table interaction, and process/task ownership before
any syscall implementation. The accepted Phase 7.3 syscall ABI contract fixes
the first stable syscall boundary: lower-AArch64 svc #0, syscall number in x8,
scalar arguments in x0 through x5, x0 as the sole return register, negative
errno returns, talos_nop = 0, and unknown syscall = -ENOSYS. The accepted
Phase 7.3 target-independent syscall dispatch core implements the pure
dispatch vocabulary and return/error encoding, and the accepted trap-routing
source inventory maps the production exception-routing boundary for lower-EL
SVC detection, argument capture, return mutation, ELR/SPSR handling,
diagnostic marker quarantine, and non-syscall fallback. The accepted
trap-routing contract limits production routing to lower-AArch64 svc #0,
requires x8/x0-through-x5 dispatch into the accepted core, preserves ELR/SPSR,
keeps diagnostic 0x7a10 proof-only, and requires QEMU syscall smoke before any
production routing claim.
The obsolete-bloat inventory and removal sweep are accepted before the
multi-core preemption core: historical QEMU secondary-core discriminator paths
and old Pi 5 allocator, exception, panic, and translation-fault proof-only
scripts/cfg/source paths are retired while accepted evidence summaries remain
in task records. Direct IRQ/IPI-context scheduling, running-task migration,
non-diagnostic secondary runtime roles, EL0/syscalls, filesystem, networking,
SSH, and shell work remain deferred until the next explicit bounded task accepts
them.

Accepted status and historical completed facts:

- Talos project directory created separately from Daedalus.
- mdBook documentation skeleton created.
- Lab controller documented and reachable from OpenClaw at http://talos-lab-api:8080.
- TFTP boot archive publishing and PoE control API are documented.
- Minimal Rust no_std AArch64 kernel skeleton created for QEMU virt.
- Pi 5 target definition and target boundary stubs created.
- Early target service descriptors added for boot info, UART kind, timer kind,
  interrupt-controller kind, MMIO map, and device tree pointer.
- Pi 5 kernel image and boot-tree staging scripts added for local archive
  preparation.
- Physical Pi 5 first-light reached Talos code.
- Readable Talos-origin serial output is available through the lab controller.
- Exception and panic diagnostics report useful AArch64 state.
- The Pi 5 boot path parses firmware handoff state and DTB memory metadata.
- Early EL2 stage-1 translation, instruction cache, and data cache have booted
  on hardware while preserving serial output.
- A no-free bootstrap allocator and narrow Rust alloc-crate diagnostics for
  Box, Vec, String, and alloc-backed formatting have hardware evidence.
- Phase 3 has an accepted closeout checkpoint for the current memory, MMU, and
  kernel-runtime boundary. The checkpoint recommends planning Phase 4 next while
  preserving explicit deferrals for high memory, DMA/cache ownership, lower-EL
  userspace, SMP, filesystem/userland, and networking.
- Phase 4 has a source-backed interrupt/timer inventory naming the first QEMU
  virt and Pi 5 GICv2 plus ARM generic-timer targets.
- QEMU virt has a focused EL2 timer-interrupt smoke: with virtualization
  enabled, CNTHP_*_EL2 raises PPI 10 / INTID 26 through GICv2, the current-EL
  IRQ frame path acknowledges and EOIs it, and execution returns to a bounded
  post-IRQ workload.
- Pi 5 has a focused EL2 timer-interrupt smoke using the same CNTHP_*_EL2 /
  PPI 10 / INTID 26 shape through GIC-400. Serialized lab evidence shows the
  candidate image was fetched, the IRQ handler acknowledged and EOI'd INTID 26,
  and execution returned to a bounded post-IRQ workload.
- The Phase 4 timer-smoke checkpoint reconciles the accepted QEMU and Pi 5
  evidence, and monotonic tick accounting now reprograms the EL2 physical
  timer for four periodic ticks on QEMU and Pi 5 before reporting outside the
  IRQ path.
- Phase 4.1/4.2 has a pre-scheduler closeout checkpoint covering the accepted
  interrupt-controller, EL2 physical timer, periodic tick, and single-core
  interrupt-mask/restore boundary. Milestone 4.3 may start with a bounded
  scheduler-shape task that checks task/process terminology against the early
  POSIX note before committing scheduler structs.
- Phase 4.3 scheduler shape is accepted as a single-core, kernel-thread-first
  boundary. The next bounded implementation step is scheduler structs and a
  runnable queue, without context switching, preemption time slicing, SMP,
  userspace, file descriptors, console/TTY, filesystem, networking, or SSH.
- Phase 4.3 now has the first scheduler data structures: scheduler-local task
  IDs, kernel-thread state, per-task kernel stack and context placeholders, an
  optional future process-owner hook, a fixed single-core runnable queue, and
  unit tests for the queue/state invariants. Context switching, sleep queues,
  preemption, SMP, userspace, descriptors, console/TTY, filesystem, networking,
  and SSH remain deferred.
- Phase 4.3 has a documented EL2 cooperative context-switch contract for
  single-core kernel threads. The first QEMU context-switch smoke is accepted:
  two kernel-thread contexts with separate stacks make bounded progress through
  the AArch64 save/restore primitive, and the implementation reports switch,
  current-task, and runnable-task state outside the switch hot path.
- Phase 4.3 voluntary-yield dispatch is accepted in QEMU. The single-core
  scheduler can requeue a running task, select the next runnable task, count
  voluntary yields and dispatch switches, and cross the cooperative switch
  boundary while keeping the short scheduler mutation window IRQ-masked.
  Timer-driven preemption and async exception-frame switching remain deferred.
- The Phase 4.3 preemption-entry policy checkpoint is accepted. The next bounded
  task may attempt a QEMU-only timer-preemption smoke that preserves
  acknowledge/reprogram/EOI ordering, keeps scheduler switching and diagnostics
  out of the IRQ hot path, and remains single-core EL2 kernel-thread only.
- Phase 4.3 QEMU timer-preemption smoke is accepted. EL2 timer ticks now record
  bounded preemption requests in the IRQ hot path, then kernel-thread code
  performs scheduler dispatch and context switching outside IRQ context. Two
  QEMU kernel threads make progress from timer-driven preemption with zero
  voluntary-yield dispatches.
- Phase 4.3 Pi 5 timer-preemption hardware proof is accepted. The physical Pi
  5 fetched the 103,152-byte candidate kernel over TFTP, reached the EL2
  timer-preemption smoke, and reported task1=3, task2=3, ticks=6, requests=6,
  handled=6, timer-preemptions=6, dispatch-switches=6, voluntary-yields=0,
  INTID 26, unexpected=0, and PASS before the pre-run boot snapshot was
  restored.
- Phase 4.3 scheduler/preemption contract consolidation is accepted. The
  production boundary is the single-core scheduler data model, short
  IRQ-masked scheduler mutation windows, and an IRQ hot path limited to
  acknowledge/classify/tick/request/reprogram/EOI. The QEMU and Pi 5
  timer-preemption boot images remain validation surfaces, not supported
  kernel interfaces.
- Phase 4 closeout is accepted. The checkpoint reconciles the accepted QEMU and
  Pi 5 interrupt/timer/preemption evidence, names remaining deferrals and
  risks, and allows Phase 5 planning to start with a bounded local console
  device-model source inventory.
- Phase 5 console device-model source inventory is accepted. The current early
  logging surfaces are inventoried, the early/runtime console ownership
  boundary is documented, and descriptor/TTY compatibility constraints are
  named without implementing descriptor tables, input, userspace, filesystem,
  networking, SSH, or shell behavior.
- Phase 5 runtime console write core and write-result contract are accepted.
  Normal kernel output now routes through the named
  `runtime_console::write_default_console_output` boundary while preserving
  `print!` / `println!` and the existing target-owned polling PL011 backends.
  Pi 5 normal serial output is intended to be preserved through the existing
  firmware-preserved UART10 backend.
- Phase 5 default console identity is accepted. The output-side runtime console
  is named `runtime-console0`; later `stdout` and `stderr` descriptors
  should attach to that console through descriptor-owned handles instead of
  calling target backends directly.
- Phase 5 console input-source inventory is accepted. QEMU PL011 polling RX is
  the recommended first input implementation proof; Pi 5 input should follow
  only with serialized hardware evidence, preferably starting from the accepted
  UART10 console path before revisiting RP1 UART0 risk.
- Phase 5.1 console model checkpoint is accepted. The console model is
  output-capable and input-planned: normal diagnostics route through
  runtime-console0, target modules own QEMU/Pi 5 PL011 backend selection,
  and Milestone 5.2 may start with a documentation-only TTY/stdio shape task.
- Phase 5.2 TTY/stdio shape is accepted as a design boundary. Raw mode,
  canonical-lite line assembly, newline/backspace/echo/control-character
  policy, and descriptor-facing stdin/stdout/stderr shape are documented.
- Phase 5.2 QEMU polling TTY RX, the shared line-discipline core, the internal
  console input result contract, and the Pi 5 UART10 polling RX proof are
  accepted. QEMU and Pi 5 both use the same injected byte sequence through the
  runtime-console/TTY boundary and report deterministic echo, line, truncation,
  and control-event evidence without adding descriptors, syscalls, userspace,
  shell behavior, UART interrupts, networking, SSH, or scheduler blocking I/O.
- The Phase 5.2 TTY/stdio closeout checkpoint is accepted. The next
  supervisor-planned slice should be a Milestone 5.3 local kernel diagnostic
  command-channel source inventory, not an implementation shortcut around the
  accepted runtime-console and TTY boundaries.
- Phase 5.3 local diagnostic command-channel source inventory is accepted. The
  command channel must consume completed TTY lines, write bounded responses
  through runtime-console0, classify existing diagnostics before exposing them,
  and remain separate from descriptor/syscall/POSIX shell semantics.
- Phase 5.3 diagnostic command-channel contract is accepted. The
  target-independent parser/dispatcher consumes complete TTY lines, bounds
  command and argument tokens, exposes deterministic help/list/status responses,
  reports unknown and malformed commands, and keeps the response sink attached
  to runtime-console0 without adding a shell, descriptor table, syscall ABI,
  filesystem command execution, networking, SSH, SMP, UART interrupts, or
  scheduler blocking I/O.
- Phase 5.3 QEMU diagnostic command-channel smoke is accepted. The QEMU serial
  transcript proves `help`, `list`, deterministic unknown-command handling,
  and `status` through the accepted polling TTY line path and
  runtime-console0 response sink without adding Pi 5 hardware behavior,
  descriptors, syscalls, userspace shell behavior, filesystem-backed commands,
  networking, SSH, SMP, UART interrupts, or scheduler blocking I/O.
- Phase 5.3 Pi 5 diagnostic command-channel proof is accepted. The serialized
  UART10 hardware transcript proves the same `help`, `list`, `bogus`, and
  `status` command sequence through canonical-lite TTY input and
  runtime-console0 responses, with TFTP evidence tying the output to the
  staged candidate image.
- The Phase 5.3 diagnostic command-channel closeout checkpoint is accepted.
  Milestone 5.3 now has reconciled source inventory, parser/dispatcher
  contract, QEMU smoke, and Pi 5 UART10 hardware proof evidence. The accepted
  command channel remains kernel-owned and diagnostic-only; descriptor tables,
  syscalls, userspace shell behavior, filesystem-backed commands, networking,
  SSH, SMP, UART interrupts, RP1 UART0, and scheduler blocking I/O remain
  deferred.
- Phase 6.1 secondary-core bring-up source inventory and contract is accepted.
  PSCI with the firmware/DTB SMC conduit is the default bring-up path;
  spin-table and custom mailbox approaches remain fallback research. Before
  scheduler work, each secondary core must prove MPIDR/logical identity,
  exclusive stack ownership, per-core state registration, and controlled
  handoff.
- Phase 6.1 QEMU secondary-core discriminator is accepted. Under QEMU virt with
  EL2 virtualization enabled, PSCI `CPU_ON` through SMC starts secondary CPUs
  1, 2, and 3; each reports distinct MPIDR affinity, runs on its reserved
  stack, reaches `handoff-ready`, and parks without claiming Pi 5 hardware
  behavior.
- Phase 6.1 Pi 5 PSCI secondary-core alive proof is accepted. Serialized
  hardware evidence shows the Pi fetched the 90,784-byte candidate image and
  cores 1, 2, and 3 reported MPIDR affinities `0x100`, `0x200`, and
  `0x300`, distinct owned stack slots, `handoff-ready` state, and
  `pi5-psci-smc-secondary-cores-alive` before the pre-run boot snapshot was
  restored.
- Phase 6.1 controlled secondary-core workload is accepted. QEMU and serialized
  Pi 5 hardware evidence show secondary cores 1, 2, and 3 reach
  `workload-complete` with `progress=64 target=64 ok=true` through the
  accepted PSCI/trampoline/stack boundary while the production scheduler
  remains single-core.
- The Phase 6.1 secondary-core bring-up closeout checkpoint is accepted.
  Milestone 6.1 now has reconciled source inventory, QEMU discriminator,
  per-core state/stacks, Pi 5 PSCI alive proof, and controlled secondary-core
  workload evidence. SMP-safe primitives, scheduler migration, shared run
  queues, cross-core wakeups, userspace, descriptors, filesystem, networking,
  SSH, shell behavior, UART interrupts, RP1/PCIe, and DMA/cache policy remain
  deferred.
- Phase 6.2 SMP-safe primitives source inventory and contract is accepted. It
  separates local IRQ masking, SMP mutual exclusion, memory ordering, and cache
  maintenance; carries forward the accepted Pi 5 cache-maintenance lesson; and
  names `phase6-spinlock-barrier-core-20260524` as the first bounded
  implementation task before scheduler migration or shared run queues.
- Phase 6.2 spinlock/barrier core is accepted. `src/smp_sync.rs` provides a
  narrow `SpinLock<T>`, RAII guard, AArch64 IRQ-save lock composition, and a
  named `dmb ish` full-barrier boundary without wiring scheduler migration,
  shared run queues, cross-core wakeups, or cache maintenance into the lock.
- Phase 6.2 QEMU SMP lock contention smoke is accepted. QEMU virt with
  `-smp 4` starts secondary cores 1, 2, and 3 through the accepted PSCI
  trampoline path; each core contends on the shared `SpinLock<T>` for 64
  iterations, and the transcript reports `counter=192 expected=192`,
  `participants=3`, `errors=0`, and
  `qemu-smp-lock-contention-complete`. This remains QEMU/substitute evidence;
  the separate hardware-locked Pi 5 proof below closes the physical
  cache/coherence claim.
- Phase 6.2 Pi 5 SMP lock cache/coherence proof is accepted. Serialized Pi 5
  hardware evidence shows the boot CPU and logical cores 1, 2, and 3 in the
  accepted cacheable-MMU regime before generic lock access; each secondary
  reports stable identity and `ok=true`; the final invariant reports
  `counter=192 expected=192 participants=3 errors=0`,
  `mixed-cache-mmu=false`,
  `classification=pi5-smp-lock-cache-coherence-complete`, and `PASS`.
- The Phase 6.2 SMP-safe primitives closeout checkpoint is accepted. Milestone
  6.2 now has reconciled source inventory, generic `SpinLock<T>` and barrier
  implementation, QEMU SMP lock contention evidence, serialized Pi 5 physical
  cache/coherence proof, and proof-scaffolding cleanup. Scheduler migration,
  shared run queues, cross-core wakeups, IPIs, userspace, descriptors,
  filesystem, networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and
  DMA/cache-coherent driver policy remain deferred.
- Phase 6.3 scheduler migration readiness, per-core scheduler state, and QEMU
  per-core scheduler ownership evidence are accepted. The scheduler now has a
  CPU-local ownership data boundary and QEMU substitute evidence that logical
  CPUs 0 through 3 can publish distinct local scheduler ownership snapshots
  while secondary production dispatch, shared run queues, task migration,
  cross-core wakeups, and IPIs remain deferred.
- Phase 6.3 cross-core wakeup/IPI source inventory is accepted. The selected
  path was raw SGI delivery first: a QEMU-only SGI/IPI smoke for target-list
  mapping, acknowledgement/EOI, and per-core counters before any scheduler
  wakeup implementation, followed by a serialized Pi 5 proof before SGIs are
  accepted for physical scheduler wakeups.
- Phase 6.3 raw SGI delivery is accepted on both QEMU and Pi 5. The QEMU proof
  shows SGI INTID 1 target-list delivery to logical CPUs 1, 2, and 3; the
  serialized Pi 5 proof shows the physical GIC-400 path delivering and EOI'ing
  SGI INTID 1 on logical CPUs 1, 2, and 3. These are raw interrupt-delivery
  proofs, not scheduler wakeup or remote enqueue implementations.
- Phase 6.3 remote wakeup ownership source inventory is accepted. The selected
  first model is a bounded per-target remote wake-request list: a remote sender
  may publish a bounded request and signal with SGI INTID 1, while the target
  CPU owns request consumption and any later local scheduler effect.
- Phase 6.3 QEMU remote wake-request evidence and the cross-core wakeup
  closeout checkpoint are accepted. QEMU proves request publication, duplicate
  coalescing, SGI signaling, target-owned observation/EOI/consumption, and
  cross-owner mutation rejection for logical CPUs 1, 2, and 3. This is
  scheduler-facing substitute evidence, not a Pi 5 scheduler wakeup claim.
- Phase 6.3 Pi 5 remote wake-request evidence is accepted. The serialized
  hardware proof shows CPU 0 publishing bounded requests for logical CPUs 1, 2,
  and 3, duplicate coalescing for target 1, SGI INTID 1 delivery/EOI,
  target-owned request consumption, drained queues, rejected cross-owner local
  scheduler mutation, and deferred secondary production dispatch. This proves
  request publication/signaling/consumption only; local runnable transitions
  from remote requests remain deferred.
- Phase 6.3 target-owned wake consumption contract is accepted. A remote CPU
  may not mutate another CPU's runnable queue; after a target consumes a
  remote request outside IPI context, only that target may transition one of
  its own blocked local tasks to runnable under local scheduler rules. The
  QEMU target-owned wake-consumption proof is also accepted. It proves
  blocked-to-runnable local transitions for diagnostic tasks on logical CPUs
  1, 2, and 3 after request drain, duplicate coalescing, duplicate-local
  enqueue rejection, cross-owner rejection, drained queues, SGI INTID 1
  observation/EOI, and no production secondary dispatch.
- Phase 6.3 Pi 5 remote-wake-to-local-runnable evidence is accepted. The
  serialized hardware proof carries the QEMU target-owned wake-consumption
  invariant to physical Pi 5: after bounded request drain, logical CPUs 1, 2,
  and 3 each transition only their own diagnostic blocked task to runnable,
  reject duplicate local enqueue, preserve SGI INTID 1 observation/EOI,
  preserve duplicate request coalescing and cross-owner rejection, and leave
  production secondary dispatch deferred.
- The Phase 6.3 remote wakeup scheduler-integration closeout checkpoint is
  accepted. It reconciles raw SGI delivery, bounded remote wake-request
  publication/consumption, target-owned local Blocked -> Runnable transitions,
  retained gates, deferrals, and risks. Talos is ready for a
  supervisor-planned production secondary scheduler dispatch source inventory
  and contract, not implementation, shared run queues, task migration,
  multi-core preemption, Phase 7, filesystem, networking, SSH, or shell work.
- Phase 6.3 production secondary scheduler dispatch source inventory and
  contract is accepted. The first implementation may dispatch only explicitly
  seeded CPU-local diagnostic kernel threads on secondary CPUs, from normal
  secondary control flow, with per-core current-task reporting and local
  runnable transitions. Shared run queues, global task lookup, remote enqueue,
  task migration, load balancing, work stealing, multi-core preemption, Phase
  7, filesystem, networking, SSH, and shell work remain deferred.
- Phase 6.3 QEMU production secondary dispatch evidence is accepted. Under
  QEMU virt, logical CPUs 1, 2, and 3 enter the explicit
  `SecondaryProductionDiagnostic` role, dispatch bounded CPU-local diagnostic
  tasks, publish stable local ownership/current-task/counter snapshots, and
  reject cross-owner local scheduler mutation. This is substitute evidence only;
  Pi 5 production secondary dispatch remains the next hardware proof.
- Phase 6.3 Pi 5 production secondary dispatch evidence is accepted. On
  serialized Pi 5 hardware, logical CPUs 1, 2, and 3 enter the explicit
  `SecondaryProductionDiagnostic` role, dispatch bounded CPU-local diagnostic
  tasks, publish stable local ownership/current-task/counter snapshots, and
  reject cross-owner local scheduler and dispatch attempts. Shared scheduler
  metadata, shared run queues, task migration, load balancing, multi-core
  preemption, Phase 7, filesystem, networking, SSH, and shell work remain
  deferred.
- The Phase 6.3 production secondary dispatch closeout checkpoint is accepted.
  It reconciles the source inventory, implementation, QEMU substitute proof,
  and Pi 5 hardware proof for the CPU-local production secondary diagnostic
  dispatch slice. The next bounded worker task should be a shared scheduler
  metadata source inventory and contract, not shared run queue implementation,
  task migration, multi-core preemption, Phase 7, filesystem, networking, SSH,
  or shell work.
- Phase 6.3 shared scheduler metadata source inventory and contract is
  accepted. The next bounded implementation task should add only local-owner
  metadata types and APIs for scheduler task identity, owning CPU, task state,
  optional process owner, stack bounds, current/runnable membership, and stale
  snapshot rejection. It must preserve CPU-local runnable queue ownership and
  does not authorize shared run queues, remote enqueue, migration, load
  balancing, multi-core preemption, Phase 7, filesystem, networking, SSH, or
  shell work.
- Phase 6.3 shared scheduler metadata core is accepted at static/unit-test and
  retained QEMU-smoke evidence levels. The core adds a bounded
  owner-published metadata table, task snapshots, explicit duplicate/unknown/
  invalid-owner/stale-snapshot outcomes, and a named SpinLock boundary for
  future shared use. It still does not authorize shared run queues, remote
  enqueue, migration, load balancing, multi-core preemption, Phase 7,
  filesystem, networking, SSH, shell work, RP1/PCIe, UART interrupt ownership,
  or DMA/cache-coherent driver policy.
- Phase 6.3 QEMU shared scheduler metadata evidence is accepted. Under QEMU
  SMP, logical CPUs 0 through 3 publish/query the owner-only metadata table,
  prove boot-task and owner-task lookup, reject cross-owner scheduler and
  metadata mutation, preserve target-owned local runnable queues, and report
  classification=qemu-shared-scheduler-metadata-complete. Serialized Pi 5
  proof remains required before treating the invariant as physical hardware
  evidence.
- Phase 6.3 Pi 5 shared scheduler metadata evidence is accepted. On serialized
  Pi 5 hardware, logical CPUs 0 through 3 publish/query the owner-only metadata
  table, prove boot-task and owner-task lookup, reject cross-owner scheduler
  and metadata mutation, preserve local runnable queues, and report
  classification=pi5-shared-scheduler-metadata-complete. This is hardware
  evidence for bounded shared metadata only; shared run queues, remote enqueue,
  migration, load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, shell work, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-coherent driver policy remain deferred.
- The Phase 6.3 shared scheduler metadata closeout checkpoint is accepted. It
  reconciles the source inventory, core implementation, QEMU substitute proof,
  and Pi 5 hardware proof for the bounded owner-published metadata table. The
  next bounded task should audit evidence retention and repository bloat before
  broader scheduler productionization; shared run queues, task migration,
  load balancing, multi-core preemption, Phase 7, filesystem, networking, SSH,
  and shell work remain deferred.
- The evidence-retention policy and bloat audit is accepted. Task records and
  compact evidence summaries are the durable source of truth; large raw lab
  captures should move out of Git only through explicit cleanup with preserved
  classifications, digests, and artifact identity.
- The diagnostic-surface retirement audit is accepted. Current QEMU and Pi 5
  Phase 6.3 proof scripts are named retained gates, while older one-off
  diagnostic paths are queued for bounded retirement only after replacement or
  summary coverage is explicit.
- The senior-review maintainability remediation checkpoint is accepted: stale
  Pi 5 probe/proof surfaces were removed, validation hygiene was restored, the
  Pi 5 boot pipeline is split into named phases, and cross-module tests now
  live in owning modules.
- The Phase 6.3 secondary scheduler service-loop closeout checkpoint is
  accepted. It reconciles the source inventory, core implementation, QEMU
  substitute smoke, and serialized Pi 5 hardware proof for one owner-local
  secondary service cycle. The next bounded task should inventory shared
  run-queue and migration requirements before any shared topology
  implementation; load balancing, multi-core preemption, Phase 7, filesystem,
  networking, SSH, and shell work remain deferred.
- The Phase 6.3 shared run-queue and migration source inventory is accepted.
  It names the owner-local runnable queue assumptions, target-owned remote wake
  boundary, owner-published metadata model, SMP lock boundary, proof-routing
  surfaces, and migration blockers. The next bounded task should be the shared
  run-queue/migration contract before any implementation.
- The Phase 6.3 shared run-queue and migration contract is accepted. It keeps
  task mutation single-owner, separates remote wake from remote enqueue,
  defines local-IRQ-then-SMP-lock ordering, names acquire/release publication
  rules, specifies owner-local/reserved/shared-queued/destination-enqueued/
  rejected migration states, and keeps load balancing and multi-core
  preemption deferred.
- The Phase 6.3 shared run-queue core is accepted. It adds the
  target-independent `SharedRunQueue` owner-transfer surface, source-owner
  publication, destination-owner consumption, metadata owner transfer, local
  runnable queue removal, deterministic failure reporting, and unit-tested
  migration states. QEMU and Pi 5 proof tasks remain separate; the core does
  not add target selection, load balancing, work stealing, multi-core
  preemption, userspace, filesystem, networking, SSH, or shell behavior.
- The Phase 6.3 QEMU shared run-queue/migration smoke is accepted. It adds the
  `qemu_shared_runqueue_migration` diagnostic and proves task 107 moving from
  source owner 0 to destination owner 1 through the implemented
  `SharedRunQueue` publish/consume APIs, with source queue removal,
  destination-local enqueue, shared queue drain, metadata owner transfer, and
  classification=qemu-shared-runqueue-migration-complete.
- The Phase 6.3 Pi 5 shared run-queue/migration proof is accepted. It adds the
  `rpi5_shared_runqueue_migration` diagnostic and focused Pi 5 image/boot-tree
  scripts, records archive/kernel digests, TFTP identity, cursor-valid serial,
  classification, PASS output, and restore evidence, and reports
  participants=4 expected=4 with
  classification=pi5-shared-runqueue-migration-complete.
- The Phase 6.3 load-balancing source inventory is accepted. It records the
  scheduler, metadata, run-queue, wake, timer, SMP, and diagnostic surfaces
  available before policy design; names policy inputs and stale/invalid input
  failure modes; and separates target selection, fairness/affinity, remote
  reschedule notification, and the accepted shared run-queue migration
  mechanism before any implementation.
- The Phase 6.3 load-balancing policy contract is accepted. It defines a
  conservative deterministic policy boundary over accepted inputs, preserves
  SharedRunQueue as the only owner-transfer mechanism, keeps remote reschedule
  polling-only for the first implementation, and defers fairness, affinity,
  work stealing, running-task migration, and multi-core preemption.
- The Phase 6.3 load-balancing core is accepted. It adds
  `LoadBalancingPolicy`, `LoadBalancingPlan`, and deterministic policy
  errors for front-runnable source selection, destination role/capacity
  checks, shared queue backpressure, stale generation rejection through
  `SharedRunQueue::publish_migration`, and single-owner queue membership. It
  does not add QEMU or Pi 5 proof claims, work stealing, running-task
  migration, remote scheduler execution in IPI context, multi-core preemption,
  Phase 7, filesystem, networking, SSH, or shell behavior.
- The Phase 6.3 QEMU load-balancing smoke is accepted. It adds the
  `qemu_load_balancing_smoke` boot scenario and
  `scripts/qemu-load-balancing-smoke.sh`, proves the accepted
  `LoadBalancingPolicy` path over `SharedRunQueue`, and reports
  classification=qemu-load-balancing-smoke-complete.
- The Phase 6.3 Pi 5 load-balancing proof is accepted. It adds the
  `rpi5_load_balancing_proof` boot scenario,
  `scripts/rpi5-load-balancing-image.sh`, and
  `scripts/rpi5-load-balancing-boot-tree.sh`, proves the same deterministic
  `LoadBalancingPolicy` path on serialized Pi 5 hardware, and reports
  classification=pi5-load-balancing-complete.
- The Phase 6.3 load-balancing closeout checkpoint is accepted. It reconciles
  the source inventory, policy contract, target-independent core, QEMU
  substitute proof, serialized Pi 5 proof, retained gates, and remaining
  deferrals. The next bounded recommendation is
  `phase6-multicore-preemption-source-inventory-20260527`, a documentation
  and source-inventory task before any multi-core preemption implementation.
- The Phase 6.3 multi-core preemption source inventory is accepted. It maps
  the accepted timer IRQ, scheduler, CPU-local service, secondary
  service-loop, IPI/wake, metadata, SharedRunQueue, and load-balancing
  boundaries; names CPU-local versus cross-core assumptions; and recommends
  `phase6-multicore-preemption-contract-20260527` before implementation.
- The Phase 6.3 multi-core preemption contract, target-independent core, QEMU
  substitute proof, and serialized Pi 5 hardware proof are accepted. The
  retained QEMU proof reports
  classification=qemu-multicore-preemption-smoke-complete after logical CPUs 1,
  2, and 3 record local pending timer-preemption state without scheduler
  mutation and then service it through owner-local normal scheduler control
  flow. The retained Pi 5 proof reports
  classification=pi5-multicore-preemption-complete with participants=3 and
  PASS for the same invariant.
- The Phase 6.3 multi-core preemption closeout checkpoint is accepted. It
  reconciles the accepted source inventory, contract, target-independent core,
  QEMU substitute proof, serialized Pi 5 proof, retained gates, risks, and
  remaining deferrals before any later scheduler productionization or phase
  transition.

Blocked or pending:

- No next scheduler or phase-transition task is accepted yet. The supervisor
  should plan the next explicit bounded task before any further scheduler
  productionization or Phase 7 work proceeds. Work stealing, running-task
  migration, remote reschedule, userspace, descriptors, filesystem,
  networking, SSH, shell behavior, UART interrupts, RP1/PCIe, and
  DMA/cache-coherent driver policy remain deferred.
- Large raw accepted evidence remains in Git until external artifact storage or
  an explicit no-delete manifest-only cleanup is approved. Do not delete
  tracked accepted evidence during unrelated feature work.
- The roadmap order below now prioritizes a local Unix-like OS before network
  shell access. Ethernet and SSH should reuse the local process, stdio, TTY,
  filesystem, and syscall mechanisms rather than define them.

## Roadmap Principles

- Use Rust for kernel code, with small AArch64 assembly stubs where the hardware requires it.
- Use established Rust kernel development practices where they fit: pinned nightly toolchain, explicit custom targets, no_std, build-std, small unsafe boundaries, narrow target abstractions, and QEMU-backed smoke tests for generic architecture work.
- Keep POSIX direction visible from the start: processes, file descriptors, pipes, paths, sockets, exit/wait, and exec-style program loading should shape interfaces even before compatibility is complete.
- Keep kernel, libraries, and programs as separate product areas. Kernel code
  owns scheduling, memory, drivers, syscalls, VFS, and process isolation;
  libraries own userspace ABI wrappers and reusable runtime support; programs
  own shell, utilities, and interaction-based tests. Use OSDev Wiki examples as
  reference material when shaping these boundaries, but fit the result to the
  Talos Rust/no_std and Pi 5 constraints.
- Prefer local OS capability before remote access: serial/local TTY, stdio,
  user processes, ramfs/initramfs, VFS, libc, and a local shell come before
  Ethernet and SSH on the critical path.
- Reuse proven libraries where they shorten the path without hiding kernel
  responsibilities. smoltcp is preferred for TCP/IP evaluation over
  hand-rolling TCP; Rust uutils is preferred for core utilities once the Rust
  userspace target is viable.
- Treat self-hosting as a long-term north star, not a committed roadmap phase.
  Native compilers such as GCC, LLVM, or rustc require a mature userspace,
  filesystem, process model, libc/Rust std target, linker, storage, memory
  reclamation, and developer tooling.
- Treat Pi 5 hardware facts as evidence, not assumptions. Device tree, Linux drivers, Raspberry Pi firmware docs, Circle/RPi bare-metal examples, serial logs, and lab results should be cited in task notes.
- Keep board-specific code behind clear target boundaries. The QEMU virt target and Pi 5 target should share architecture code where possible, but not pretend to have the same devices.
- Prefer small, inspectable milestones with a boot/test gate over broad subsystem rewrites.
- Every milestone must update docs, ADRs, or task records when it changes architecture or hardware understanding.

## Phase 0: Planning, Sources, and Lab Loop

Goal: make the development system trustworthy before kernel implementation accelerates.

Milestone 0.1: Source Map

- Build a curated source index for Pi 5, BCM2712, RP1, ARMv8-A, QEMU, Linux, Raspberry Pi firmware, Circle/RPi bare-metal, and Daedalus references.
- Record which sources are authoritative and which are advisory.
- Identify missing datasheets or areas that require Linux-source archaeology.

Acceptance criteria:

- project/reference-notes.md lists primary source URLs and known gaps.
- Open hardware questions are tracked as future research tasks.

Milestone 0.2: Lab Controller Readiness

- Verify health, status, boot files, boot archive upload, power cycle, rollback, and serial endpoints.
- Keep network-controller credentials outside OpenClaw; use only the lab API.
- Establish a boot-attempt record format with archive digest, power-cycle time, serial tail, and result classification.

Acceptance criteria:

- A known-good Raspberry Pi OS boot archive can be published, power-cycled, and observed.
- Serial output is available through the API after the physical cable is installed.
- Failed boots can be rolled back without manual SD-card intervention.

Milestone 0.3: Initial ADRs

- Decide target split: QEMU virt plus physical Pi 5.
- Decide Rust toolchain and repository layout.
- Decide boot image contract and lab automation contract.
- Add an early POSIX/process shape note before scheduler task structures harden.

Acceptance criteria:

- ADRs exist for the target strategy, boot/lab loop, and Rust toolchain.
- The early POSIX/process shape note exists and is referenced before implementing scheduler task structs.

## Phase 1: Rust Kernel Skeleton and Fast Test Target

Goal: create a minimal Rust kernel that builds reproducibly and runs under QEMU virt.

Milestone 1.1: Toolchain and Image Build

- Add a custom AArch64 target JSON, Cargo config, linker script, build script, and image conversion step.
- Reuse Daedalus patterns where they still apply: build-std, alloc, panic-strategy abort, redzone disabled, explicit linker memory layout, and assembly build integration.
- Produce artifacts for both talos-aarch64-virt and talos-rpi5-bcm2712 even if the Pi 5 artifact is initially a stub.
- Pin the Rust nightly with rust-toolchain.toml and document the exact build and test commands.
- Decide target-feature policy, relocation model, inline assembly policy, compiler_builtins memory intrinsic handling, and no-unwind guarantees.

Acceptance criteria:

- cargo build produces a kernel artifact.
- The artifact layout documents load address, stack, BSS, heap reservation, and exception-vector alignment.
- Formatting and basic lint gates exist.
- Toolchain drift is detectable through CI or an explicit local check.
- Linker map or equivalent layout output can be inspected when early boot fails.

Milestone 1.2: QEMU Boot Smoke Test

- Boot on QEMU virt with a simple serial console message.
- Add a custom bare-metal test harness modeled after Daedalus, including success/failure exit through QEMU.
- Keep hardware-only behavior out of unit tests; expose it as diagnostics once real hardware exists.

Acceptance criteria:

- cargo test or an equivalent runner boots QEMU and exits with pass/fail status.
- Panic output reaches the QEMU serial console.
- Pure Rust modules can define no_std test cases.

Milestone 1.3: Early Architecture Boundaries

- Define target abstractions for boot info, UART, timer, interrupt controller, MMIO map, and device tree access.
- Keep the interfaces narrow enough to avoid overengineering before hardware facts are known.

Acceptance criteria:

- QEMU virt implements enough target operations for boot and test output. [done: QEMU test gate]
- Pi 5 target has explicit stubs or early implementations with documented unknowns. [done: build gate, pending hardware evidence]

## Phase 2: Raspberry Pi 5 First Light

Goal: boot Talos on physical Pi 5 and get reliable serial output.

Milestone 2.1: Firmware Handoff and Firmware-Preserved Serial

- Build a Pi 5 boot tree that satisfies the lab controller archive contract: config.txt, cmdline.txt, bcm2712-rpi-5-b.dtb, and kernel_2712.img or kernel8.img.
- Prefer kernel_2712.img for the Pi 5 artifact; keep kernel8.img fallback behavior documented only as firmware compatibility.
- Configure AArch64 entry, stack, BSS clearing, panic path, and serial output.
- Implement the arm64 boot ABI: x0 contains the physical DTB address, interrupts are masked, the MMU is off, and non-secure EL2 is preferred while EL1 is allowed.
- Start by using serial state preserved by firmware. Do not assume Talos owns UART clocks, GPIO muxing, or RP1 reset behavior yet.
- Check config.txt serial settings, baud rate, DTB aliases, chosen stdout-path, and whether enable_rp1_uart=1 is required for the attached cable path.

Acceptance criteria:

- The lab can publish the Talos boot archive. [local staging tool exists; publish not yet run]
- Power cycle reaches Talos code on the Pi 5.
- Serial output includes a version string, exception level, core ID, and panic path.
- A failed boot can be rolled back.

Milestone 2.2: Boot-State and UART Ownership Discovery

- Preserve and inspect the firmware-provided device tree from x0 enough to confirm memory and chosen boot arguments.
- Record actual firmware handoff state: exception level, MMU/cache state, DTB address if provided, core startup behavior, and UART clock assumptions.
- Compare observations against Linux device tree and Raspberry Pi documentation.
- Split firmware-preserved serial from Talos-owned UART initialization. The firmware console serial10 maps to BCM2712 uarta; the 40-pin header UART is RP1 UART0 and can be firmware-initialized with enable_rp1_uart=1.
- Verify serial still works after cache and MMU transitions.

Acceptance criteria:

- Architecture docs describe the actual Pi 5 handoff observed in the lab.
- UART path and ownership assumptions are documented before any UART driver is treated as stable.
- Any mismatch with assumptions becomes an ADR or tracked task.

Milestone 2.3: Exception Vectors and Panic Diagnostics

- Install AArch64 exception vectors.
- Dump ESR, FAR, ELR, SPSR, and general registers on synchronous exceptions.
- Add a deliberate exception diagnostic.

Acceptance criteria:

- A deliberate fault produces a readable serial dump.
- The dump includes enough state to debug early MMU and driver faults.

## Phase 3: Memory, MMU, and Kernel Runtime

Goal: build the foundations for safe Rust allocation, virtual memory, and later userspace.

Status: accepted for the current closeout boundary. See
[Phase 3 Closeout Checkpoint](project/phase3-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, and Phase 4
recommendation.

Milestone 3.1: Physical Memory Map

- Determine usable DRAM and reserved regions from device tree and firmware observations.
- Define kernel image, stack, heap, boot info, and early allocator regions.
- Avoid hardcoding a single RAM size.

Acceptance criteria:

- Boot log reports memory regions.
- Early allocator avoids kernel image, stack, DTB, and reserved firmware regions.

Milestone 3.2: Page Tables and MMU

- Implement early identity mappings for kernel memory and required MMIO.
- Map normal memory cacheable and MMIO as device memory.
- Keep translation setup compatible with SMP and future EL0 isolation.

Acceptance criteria:

- Pi 5 boots with MMU enabled.
- Serial still works after MMU enable.
- A page-fault diagnostic is available.

Milestone 3.3: Kernel Heap and Core Runtime

- Add a simple allocator first, then evolve toward a free-capable allocator when needed.
- Enable Rust alloc for Box, Vec, String, and collections.
- Keep allocation failure behavior explicit.

Acceptance criteria:

- Allocation tests pass under QEMU.
- Pi 5 diagnostic confirms heap allocation and panic-on-OOM behavior.

## Phase 4: Interrupts, Timers, and Preemption

Goal: move from cooperative boot code to timer-driven kernel scheduling.

Status: accepted for the current closeout boundary. See
[Phase 4 Closeout Checkpoint](project/phase4-closeout-checkpoint.md) for the
accepted capabilities, commit references, deferred work, risks, and Phase 5
planning recommendation.

Milestone 4.1: Interrupt Controller

- Identify the Pi 5 interrupt controller topology from device tree and Linux references. Current evidence points to GIC-400 / GICv2, with distributor and CPU interfaces in the 0x10_7fff9000 region.
- Bring up enough GIC support for architectural timer and UART interrupts.
- Keep QEMU virt and Pi 5 interrupt-controller setup target-specific.

Acceptance criteria:

- Timer interrupt fires on QEMU virt.
- Timer interrupt fires on Pi 5.
- IRQ entry/exit preserves register state.

Milestone 4.2: Monotonic Time and Preemption Tick

- Implement monotonic time based on the ARM generic timer first. The BCM2835-compatible 1 MHz system timer at 0x10_7c003000 is a secondary board timer path, not the first scheduler clock.
- Add scheduler tick accounting.
- Make interrupt masking and critical sections explicit.

Acceptance criteria:

- Serial diagnostics show periodic ticks without polling.
- Tick handling remains stable under simple workload loops.

Milestone 4.3: Kernel Threads and Scheduler

- Define the scheduler shape against the early POSIX note before committing
  structs. [done: single-core kernel-thread-first boundary]
- Implement kernel task structures and a single-core runnable queue before
  context switch, sleeping, yielding, or preemptive time slicing.
- Start with one core; keep data structures ready for SMP.
- Check task/process terminology and lifetime assumptions against the early POSIX shape note before committing scheduler structs. [done: scheduler shape note]

Acceptance criteria:

- Multiple kernel threads make progress under preemption.
- A diagnostic shows task state and context-switch counts.

## Phase 5: Local Console, TTY, and Kernel Diagnostics

Goal: make Talos locally operable over serial before adding network access.

Milestone 5.1: Console Device Model

- Split early boot logging from a runtime console device.
- Preserve the proven firmware-preserved UART path while defining the ownership
  boundary for later Talos-owned serial drivers.
- Route console reads and writes through interfaces that can become file
  descriptors and TTY devices.

Acceptance criteria:

- Kernel diagnostics can write through a runtime console abstraction.
- The early boot logger and runtime console ownership rules are documented.
- Console paths do not depend on ad hoc shell-only code.

Milestone 5.2: TTY and Stdio Shape

- Define the first TTY line discipline: raw/canonical input policy, newline
  handling, backspace, echo, and control-character behavior.
- Model stdin, stdout, and stderr as descriptor-capable streams even before
  full userspace exists.
- Keep the design compatible with later PTY/SSH sessions.

Acceptance criteria:

- A local serial TTY diagnostic can accept input and echo/process lines.
- Stdio streams can be represented by the same descriptor model planned for
  user processes.
- TTY behavior and known POSIX gaps are documented.

Milestone 5.3: Local Kernel Diagnostic Command Channel

- Add a constrained local diagnostic command channel over the serial TTY.
- Keep commands clearly kernel-owned until EL0 programs and a real shell exist.
- Prefer diagnostics that exercise scheduler, memory, filesystem, and process
  state without becoming permanent shell architecture.

Acceptance criteria:

- A user at the serial console can run bounded kernel diagnostic commands.
- Diagnostic commands are separated from the later user shell design.
- The command channel remains usable while scheduler/timer work is active.

## Phase 6: SMP and Multi-Core Scheduling

Goal: use all Pi 5 CPU cores with correct synchronization and preemptive scheduling.

Status: Milestone 6.1 is accepted through the secondary-core bring-up closeout
checkpoint. Milestone 6.2 has an accepted SMP-safe primitive source inventory,
contract, first spinlock/barrier core, QEMU SMP contention smoke, and physical
Pi 5 lock cache/coherence proof. Milestone 6.3 has accepted the first
scheduler-migration slice, raw QEMU/Pi 5 SGI delivery, remote wake-request
publication/consumption evidence, the target-owned wake-consumption contract,
QEMU and Pi 5 blocked-to-runnable target-owned wake proofs, and the remote
wakeup scheduler-integration closeout, plus the production secondary dispatch
closeout checkpoint and shared scheduler metadata closeout. See
[Phase 6 Secondary-Core Bring-Up Closeout Checkpoint](project/phase6-secondary-core-bringup-closeout-checkpoint.md)
and
[Phase 6 Secondary-Core Bring-Up Source Inventory](project/phase6-secondary-core-bringup-source-inventory.md),
plus
[Phase 6 SMP-Safe Primitives Source Inventory](project/phase6-smp-safe-primitives-source-inventory.md)
and
[Phase 6 Scheduler Migration Slice Checkpoint](project/phase6-scheduler-migration-slice-checkpoint.md),
and
[Phase 6 Remote Wakeup Scheduler Integration Closeout](project/phase6-remote-wakeup-scheduler-integration-closeout.md),
and
[Phase 6 Production Secondary Dispatch Closeout](project/phase6-production-secondary-dispatch-closeout-checkpoint.md),
and
[Phase 6 Shared Scheduler Metadata Source Inventory](project/phase6-shared-scheduler-metadata-source-inventory.md),
and
[Phase 6 Shared Scheduler Metadata Closeout](project/phase6-shared-scheduler-metadata-closeout-checkpoint.md),
and
[Phase 6 CPU-Local Scheduler Service Boundary Source Inventory](project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md),
and
[Phase 6 CPU-Local Scheduler Service Closeout](project/phase6-cpu-local-scheduler-service-closeout-checkpoint.md),
and
[Phase 6 Secondary Scheduler Service Loop Source Inventory](project/phase6-secondary-scheduler-service-loop-source-inventory.md),
and
[Phase 6 Secondary Scheduler Service Loop Closeout](project/phase6-secondary-scheduler-service-loop-closeout-checkpoint.md),
and
[Phase 6 Shared Run-Queue and Migration Source Inventory](project/phase6-shared-runqueue-migration-source-inventory.md),
and
[Phase 6 Shared Run-Queue and Migration Contract](project/phase6-shared-runqueue-migration-contract.md),
and
[Phase 6 Shared Run-Queue Migration Closeout Checkpoint](project/phase6-shared-runqueue-migration-closeout-checkpoint.md).
The target-independent CPU-local scheduler service core is accepted in
`tasks/2026-05-26-phase6-cpu-local-scheduler-service-core.md`.
The target-independent secondary scheduler service-loop core is accepted in
`tasks/2026-05-26-phase6-secondary-scheduler-service-loop-core.md`, with
QEMU substitute and serialized Pi 5 proof records retained as diagnostic
gates.
The target-independent shared run-queue core is accepted in
`tasks/2026-05-26-phase6-shared-runqueue-core.md`, with QEMU substitute proof
accepted in
`tasks/2026-05-26-phase6-qemu-shared-runqueue-migration-smoke.md` and
serialized Pi 5 proof accepted in
`tasks/2026-05-26-phase6-pi5-shared-runqueue-migration-proof.md`.
The shared run-queue/migration closeout reconciles source inventory, contract,
core implementation, QEMU substitute proof, Pi 5 hardware proof, retained
diagnostics, and deferred work. The load-balancing source inventory and policy
contract are accepted, and the target-independent load-balancing core is
accepted in `tasks/2026-05-27-phase6-load-balancing-core.md`. The QEMU
substitute proof is accepted in
`tasks/2026-05-27-phase6-qemu-load-balancing-smoke.md`, and the serialized
Pi 5 proof is accepted in
`tasks/2026-05-27-phase6-pi5-load-balancing-proof.md`. The load-balancing
closeout is accepted in
`docs/src/project/phase6-load-balancing-closeout-checkpoint.md`; its next
bounded Phase 6.3 recommendation is multi-core preemption source inventory.
The multi-core preemption source inventory is accepted in
`docs/src/project/phase6-multicore-preemption-source-inventory.md`; its
contract, target-independent core, QEMU substitute proof, and serialized Pi 5
proof are accepted in the corresponding Phase 6.3 task records. The bounded
multi-core preemption closeout checkpoint is accepted in
docs/src/project/phase6-multicore-preemption-closeout-checkpoint.md. The
production scheduler runtime source inventory is accepted in
`docs/src/project/phase6-production-scheduler-runtime-source-inventory.md`,
and the production timer/preemption contract is accepted in
`docs/src/project/phase6-production-timer-preemption-contract.md`. The first
production timer/preemption core is accepted in
`tasks/2026-05-28-phase6-production-timer-preemption-core.md`. The focused
QEMU production timer/preemption smoke is accepted in
`tasks/2026-05-28-phase6-qemu-production-timer-preemption-smoke.md`, and the
serialized Pi 5 production timer/preemption proof is accepted in
`tasks/2026-05-28-phase6-pi5-production-timer-preemption-proof.md`. The
production scheduler runtime closeout is accepted in
`docs/src/project/phase6-production-scheduler-runtime-closeout-checkpoint.md`.
Phase 7 or later subsystem work remains blocked until the supervisor creates
the next explicit bounded task.
Before broader Phase 6.3 productionization, the accepted
[Evidence Retention Policy](project/evidence-retention-policy.md) and
[Diagnostic Surface Policy](project/diagnostic-surface-policy.md) govern which
raw artifacts and proof-only scripts remain retained gates versus cleanup
candidates.

Milestone 6.1: Secondary Core Bring-Up

- Observe firmware core startup behavior.
- Use PSCI as the primary secondary-core bring-up path; Raspberry Pi Linux device tree advertises PSCI 1.0 with SMC and cpu_on 0xc4000003.
- Treat spin-table or custom mailbox bring-up as fallback research, not the default plan.
- Add per-core stacks, per-core state, and CPU-local data.

Acceptance criteria:

- All four Cortex-A76 cores report alive through serial diagnostics.
- Secondary cores can run a controlled kernel-thread workload.

Milestone 6.2: SMP-Safe Primitives

- Implement spin locks, interrupt-safe locks, atomics policy, memory barriers, and per-core critical-section rules.
- Review any inherited Daedalus synchronization assumptions before reuse.

Acceptance criteria:

- Stress diagnostics run shared counters and queues across cores.
- Lock misuse and interrupt-context constraints are documented.

Milestone 6.3: Multi-Core Preemptive Scheduler

- Support per-core run queues or a global scheduler with clear tradeoffs.
- Add load balancing only after correctness is established.
- Keep task migration visible in diagnostics.

Acceptance criteria:

- Multiple CPU-bound tasks run across all cores.
- Preemption continues to work under cross-core wakeups.

## Phase 7: POSIX Contract, EL0, Syscalls, and File Descriptors

Goal: introduce Unix-like execution boundaries without attempting full POSIX yet.

Milestone 7.1: POSIX Contract Baseline

- Define the first Talos error model and errno mapping.
- Define path normalization, absolute and relative paths, root, current working directory, and namespace assumptions.
- Define initial descriptor operations: open, read, write, close, dup, pipe, and descriptor inheritance.
- Define process lifetime concepts: spawn or exec, exit status, wait, parent/child relationship, and signal deferrals.
- Define the early loader ABI and argument/environment story.

Acceptance criteria:

- A POSIX-baseline design note exists before VFS or process code grows around convenient shortcuts.
- Host-side tests cover path normalization and descriptor-table edge cases.

Accepted progress:

- Phase 7 POSIX contract source inventory is accepted. It maps the accepted
  scheduler task/process separation, runtime-console and TTY stdio direction,
  diagnostic command-channel limits, lower-EL readiness limits, and retained
  Phase 4 through Phase 6 gates that constrain the first POSIX baseline
  contract.
- Phase 7 POSIX contract baseline is accepted. It defines the first
  errno-style names, path normalization semantics, process lifetime vocabulary,
  descriptor operation vocabulary, stdio inheritance shape, and early
  loader/argument/environment vocabulary.
- Phase 7 path/error model core is accepted. It adds the target-independent
  no_std path normalizer and PosixError vocabulary with unit tests.
- Phase 7 descriptor-table contract is accepted. It keeps descriptor entries
  process-local, separates entries from shared object handles, defines
  close/dup/inherited-stdio semantics, and names deterministic descriptor
  table errors for the next core implementation. Runtime console/TTY
  descriptor I/O integration, EL0, SVC/syscall ABI, VFS, filesystem, program
  loading, networking, SSH, and shell work remain blocked.
- Phase 7 descriptor-table core is accepted. It adds the first fixed-capacity
  process-local descriptor table model, inherited stdio entries, allocation,
  lookup, close, dup, access checks, reserved object kind tags, and
  deterministic PosixError results with target-independent no_std unit tests.
- Phase 7.1 POSIX baseline closeout is accepted. It reconciles the accepted
  contract, path/error, and descriptor-table evidence, preserves the retained
  gates, and recommends a supervisor-planned Phase 7.2 source inventory before
  any EL0 trap path, user address-space, syscall, VFS, filesystem, program
  loader, descriptor I/O, networking, SSH, or shell implementation.

Milestone 7.2: EL0 Trap Path and User Address Spaces

- Split kernel and user mappings.
- Add user stacks, trap return, copy-in/copy-out helpers, and fault handling.
- Validate exception return and bad user pointers before stabilizing the syscall ABI.

Acceptance criteria:

- A simple user-mode payload runs and traps back to the kernel.
- Invalid user memory access traps without corrupting the kernel.
- Negative tests cover bad pointers and invalid trap state.

Accepted progress:

- Phase 7 EL0 address-space source inventory is accepted. It names the accepted
  exception-vector and saved-frame surfaces, same-EL ERET diagnostic boundary,
  EL2 translation setup, early page-frame ownership, scheduler task/process
  separation, POSIX error vocabulary, descriptor-table ownership, retained
  gates, diagnostic-only surfaces, and implementation gaps that constrain the
  first EL0 trap-return and user address-space contract.
- Phase 7 EL0 trap and address-space contract is accepted. It defines the
  first canonical user range and null guard, user text/data/heap/stack/guard
  vocabulary, kernel-only mapping policy while a user task runs, validated
  user trap-return frame requirements, user fault classes, copy-in/copy-out
  preconditions, evidence levels, and blocked surfaces. The next implementation
  task remains target-independent user range and permission validation only.

Milestone 7.3: Syscall ABI

- Add an SVC-based syscall path from lower exception level.
- Define stable error handling and numeric syscall IDs.

Acceptance criteria:

- A minimal syscall test exercises return values, invalid calls, and fault handling.

Accepted progress:

- Phase 7 syscall ABI source inventory is accepted. It maps lower-EL
  synchronous exception entry, diagnostic SVC proof surfaces, missing syscall
  number and argument-register contracts, PosixError return/error constraints,
  user-copy preconditions, descriptor-table interaction, and process/task
  ownership. It keeps marker 0x7a10 diagnostic-only and recommends
  phase7-syscall-abi-contract-20260529 before any syscall implementation,
  QEMU rerun, Pi 5 hardware run, descriptor I/O, process loading, VFS,
  filesystem, shell, networking, or SSH work.
- Phase 7 syscall ABI contract is accepted. It defines lower-AArch64 svc #0 as
  the first stable syscall trap, keeps diagnostic SVC marker 0x7a10 out of the
  ABI, assigns x8 as the syscall-number register, x0 through x5 as scalar
  argument registers, x0 as the sole return register, negative x0 as -errno,
  talos_nop = 0, unknown syscall = -ENOSYS, and a first target-independent
  dispatch proof slice. Production exception-handler integration, QEMU syscall
  smoke, Pi 5 hardware proof, descriptor I/O, process loading, VFS, filesystem,
  shell, networking, and SSH remain blocked.
- Phase 7 syscall dispatch core is accepted. It adds a target-independent
  syscall module with stable svc #0 vocabulary, diagnostic marker quarantine,
  talos_nop dispatch, unknown-syscall -ENOSYS handling, scalar x0-through-x5
  argument preservation in the pure dispatch layer, and errno encoding for the
  accepted subset. Production exception-handler integration, QEMU syscall
  smoke, Pi 5 hardware proof, pointer-copy syscalls, descriptor I/O, process
  loading, VFS, filesystem, shell, networking, and SSH remain blocked.
- Phase 7 syscall trap-routing source inventory is accepted. It maps exact
  source owners and gaps for lower-AArch64 SVC detection, svc immediate
  validation, x8 syscall-number extraction, x0-through-x5 argument capture,
  x0 return mutation, ELR/SPSR handling, diagnostic marker 0x7a10 quarantine,
  and non-syscall fallback. It recommends
  phase7-syscall-trap-routing-contract-20260529 before production exception
  routing, QEMU syscall smoke, Pi 5 hardware proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS, filesystem, shell, networking, or
  SSH work.
- Phase 7 QEMU syscall smoke core is accepted. It adds qemu_syscall_smoke,
  routes only lower-AArch64 svc #0 through the target-independent dispatch core,
  returns talos_nop x0 = 0 and unknown syscall x0 = -ENOSYS to the user payload,
  quarantines diagnostic marker 0x7a10 outside production dispatch, and retains
  QEMU/substitute PASS evidence. Pi 5 production syscall proof, descriptor I/O,
  copy-in/copy-out, process loading, VFS, filesystem, shell, networking, and SSH
  remain blocked.
- Phase 7 syscall routing closeout checkpoint is accepted. It reconciles the
  syscall ABI contract, dispatch core, trap-routing contract, QEMU smoke
  implementation, retained QEMU evidence, diagnostic-marker quarantine, and
  deferred surfaces before any physical syscall proof or pointer/descriptor
  syscall work.
- Phase 7 Pi 5 syscall proof plan is accepted. It defines the later serialized
  rpi5_syscall_proof invariant, exact physical PASS/classification and
  syscall-observation lines, hardwareTestLock acquisition/release rules,
  candidate identity, fresh serial/TFTP evidence, inconclusive-run triage,
  restoration requirements, and diagnostic marker 0x7a10 quarantine. No Pi 5
  run, archive publication, descriptor I/O, copy-in/copy-out, process loading,
  filesystem, shell, networking, or SSH behavior is accepted by the plan.
- Phase 7 Pi 5 syscall proof is accepted. It adds rpi5_syscall_proof and the
  focused Pi 5 lower-AArch64 svc #0 recovery path, then retains serialized
  physical evidence that talos_nop returns x0 = 0, unknown syscall number 17
  returns x0 = -ENOSYS, diagnostic marker 0x7a10 is not dispatched as a stable
  syscall, and the final line reports
  classification=pi5-syscall-proof-complete with rpi5-syscall-proof: PASS.
  The proof includes candidate identity, fresh TFTP serves of
  da591740/kernel_2712.img at 101408 bytes, fresh serial evidence, a passing
  production-timer known-good control after the first inconclusive run, an
  unchanged candidate rerun, and post-restore tree-hash proof. Descriptor I/O,
  copy-in/copy-out, process loading, VFS/filesystem, shell, networking, and SSH
  remain blocked.
- Phase 7 Pi 5 syscall proof closeout checkpoint is accepted. It reconciles
  the ABI, dispatch, production trap routing, QEMU syscall smoke evidence, Pi 5
  physical proof evidence, hardware-lock timeline, restore proof, and blocked
  surfaces. It recommends phase7-copyin-copyout-helper-contract-20260529 as the
  next bounded documentation-only task before pointer-taking syscall or
  descriptor I/O implementation.
- Phase 7 copy-in/copy-out helper contract is accepted. It defines the first
  target-independent helper boundary: whole-range validation before byte
  movement, copy-in read access, copy-out write access, deterministic EFAULT
  mapping for null/kernel-range/unmapped/permission/wraparound failures,
  all-or-nothing copy behavior, and a split between recoverable syscall helper
  failures and future process-fatal lower-EL abort classifications. It names
  phase7-copyin-copyout-helper-core-20260529 as the next bounded implementation
  task, pending supervisor planning. Pointer-taking syscalls, descriptor I/O,
  process loading, VFS/filesystem, shell, networking, and SSH remain blocked.
- Phase 7 pointer-taking syscall source inventory is accepted. It maps source
  owners and gaps for lower-AArch64 frame argument extraction, x8 syscall
  number ownership, user-memory mapping provenance, copy_from_user/copy_to_user
  invocation, x0 return/error encoding, QEMU smoke ownership, and proof-only
  diagnostic-surface quarantine. It recommends supervisor planning for
  phase7-pointer-taking-syscall-contract-20260529 before any implementation or
  QEMU pointer-copy smoke plan. Descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
  remain blocked.
- Phase 7 pointer-taking syscall contract is accepted. It defines the first
  pointer-taking syscall as proof-only talos_copy_probe, routed through stable
  svc #0 with x8 = 0x7001 only in the later QEMU/substitute smoke scenario.
  x0 is the user pointer, x1 is a 0-through-32 byte length, x2 is the expected
  byte, x3 is the replacement byte, and x4/x5 are reserved zeros. Success
  copies in, validates the byte pattern, copies out, and returns the copied
  length; user-boundary failures return -EFAULT, malformed proof setup returns
  -EINVAL, and x8 = 0x7001 outside the proof scenario remains -ENOSYS. The
  contract names the fixed QEMU substitute UserData mapping and keeps
  descriptor I/O, process loading, VFS/filesystem, shell, networking, SSH, and
  Pi 5 pointer-copy hardware proof blocked.
- Phase 7 QEMU pointer-copy smoke plan is accepted. It defines the later
  qemu_pointer_copy_smoke QEMU/substitute invariant for proof-only
  talos_copy_probe: UserData at 0x0000_0000_0011_0000, a 16-byte success case
  that copies 0x2a bytes in and writes 0xa5 bytes back, a guard-range EFAULT
  case returning -EFAULT, an unknown-syscall regression returning -ENOSYS, and
  diagnostic marker 0x7a10 quarantine before classification/PASS. The plan
  names phase7-qemu-pointer-copy-smoke-core-20260529 as the next bounded
  implementation task and keeps descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware proof
  blocked.
- Phase 7 QEMU pointer-copy smoke core is accepted. It adds
  qemu_pointer_copy_smoke, a proof-only talos_copy_probe route for x8 = 0x7001
  scoped to that scenario, explicit substitute UserData backing storage,
  copy_from_user/copy_to_user helper invocation, success and guard EFAULT
  observations, an unknown-syscall -ENOSYS regression, diagnostic marker
  quarantine, and retained QEMU/substitute PASS evidence. Descriptor I/O,
  process loading, VFS/filesystem, shell, networking, SSH, and Pi 5
  pointer-copy hardware proof remain blocked.
- Phase 7 pointer-copy closeout checkpoint is accepted. It reconciles the
  pointer-taking syscall contract, QEMU pointer-copy smoke plan, core
  implementation, retained QEMU/substitute evidence, scalar syscall and EL0
  diagnostic regressions, proof-only status, and blocked surfaces. It
  recommends supervisor planning for a documentation-only Pi 5 pointer-copy
  proof plan before any hardware action, and keeps descriptor I/O, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, and stable POSIX descriptor claims
  blocked.
- Phase 7 Pi 5 pointer-copy proof plan is accepted. It translates the accepted
  QEMU/substitute talos_copy_probe boundary into a future serialized physical
  proof with required success-copy, guard-range EFAULT, unknown-syscall,
  diagnostic-quarantine, classification, PASS, candidate-identity,
  fresh-serial/TFTP, hardwareTestLock, restoration, and inconclusive-run
  triage evidence. It does not run hardware and keeps descriptor I/O, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, and stable POSIX descriptor claims
  blocked.
- Phase 7 Pi 5 pointer-copy proof is accepted. It adds the focused
  rpi5_pointer_copy_proof scenario and scripts, then retains serialized Pi 5
  evidence with success-copy, guard-range -EFAULT, unknown-syscall -ENOSYS,
  diagnostic-marker quarantine, classification=pi5-pointer-copy-proof-complete,
  and rpi5-pointer-copy-proof: PASS. The evidence includes the required
  inconclusive-run triage and restore proof. Descriptor I/O, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and stable POSIX descriptor claims remain blocked.
- Phase 7 Pi 5 pointer-copy proof closeout checkpoint is accepted. It
  reconciles the pointer-taking contract, QEMU/substitute pointer-copy smoke,
  Pi 5 physical proof, hardware-lock timeline, restore proof, proof-only
  talos_copy_probe status, retained evidence paths, and blocked surfaces. It
  recommends phase7-descriptor-syscall-source-inventory-20260529 as the next
  bounded documentation-only task before descriptor syscall contracts or
  implementations.
- Phase 7 descriptor syscall source inventory is accepted. It maps
  src/posix.rs descriptor tables and copy helpers, src/syscall.rs stable svc #0
  dispatch, lower-AArch64 saved-frame argument capture, runtime-console0 and
  TTY backing surfaces, scheduler task/process ownership gaps, and retained
  QEMU evidence ownership. It recommends
  phase7-descriptor-syscall-contract-20260529 as a stdout/stderr write
  contract slice before any descriptor implementation. stdin/read, close, dup,
  process loading, VFS/filesystem, shell, networking, SSH, live process-owned
  address spaces, blocking/readiness, signals, restart semantics, RP1/PCIe,
  UART interrupt ownership, DMA/cache-driver policy, and stable POSIX
  descriptor claims remain blocked.
- Phase 7 QEMU descriptor-write smoke plan is accepted. It defines the later
  qemu_descriptor_write_smoke QEMU/substitute invariant for talos_write fd 1
  and fd 2 success through inherited stdio descriptors, copy_from_user(), and
  runtime-console0; fd 0 and invalid-fd -EBADF; guard-range -EFAULT;
  reserved-register -EINVAL; talos_nop and unknown-syscall regressions; and
  proof-only talos_copy_probe quarantine. It names
  phase7-descriptor-write-core-20260529 as the next bounded implementation
  task and keeps stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical descriptor-write claims, and full POSIX
  descriptor claims blocked.
- Phase 7 descriptor-write closeout checkpoint is accepted. It reconciles the
  descriptor source inventory, talos_write contract, QEMU descriptor-write
  smoke plan, descriptor-write core, retained
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log
  evidence, scalar and pointer-copy regression gates, residual risks, and
  deferred surfaces. It recommends
  phase7-pi5-descriptor-write-proof-plan-20260529 as the next bounded
  documentation-only planning task before any Pi 5 descriptor-write hardware
  action. stdin/read, close, dup, process loading, VFS/filesystem, shell,
  networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical descriptor-write claims, and full POSIX
  descriptor claims remain blocked.
- Phase 7 Pi 5 descriptor-write proof plan is accepted. It defines the
  serialized physical proof invariant for talos_write fd 1/fd 2 through
  copy_from_user(), inherited stdio descriptors, and runtime-console0; fd and
  pointer errno cases; talos_nop and unknown-syscall regressions;
  talos_copy_probe and diagnostic-marker quarantine; hardwareTestLock
  ownership; candidate identity; fresh serial/TFTP evidence; restoration; and
  inconclusive-run triage. It names
  phase7-pi5-descriptor-write-proof-20260529 as the next bounded hardware
  task and keeps stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims blocked.
- Phase 7 Pi 5 descriptor-write proof is accepted. It adds the focused
  rpi5_descriptor_write_proof scenario, Pi 5 descriptor-write lower-AArch64
  svc #0 handling, image/boot-tree helpers, retained local evidence, and
  retained lab evidence. The accepted local3 rerun includes fd 1 stdout and fd
  2 stderr runtime-console0 writes, fd0/fd99 -EBADF, guard -EFAULT, reserved
  x3 -EINVAL, talos_nop, unknown syscall -ENOSYS, copy-probe quarantine,
  diagnostic-marker quarantine, classification=pi5-descriptor-write-proof-complete,
  and rpi5-descriptor-write-proof: PASS. The first candidate run was
  inconclusive, so the retained evidence records candidate identity, fresh
  serial/TFTP cursors, a passing production-timer known-good control, an
  unchanged candidate rerun, and restore proof. It recommends
  phase7-pi5-descriptor-write-proof-closeout-checkpoint-20260529 next and
  keeps stdin/read, close, dup, process loading, VFS/filesystem, shell,
  networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims blocked.
- Phase 7 Pi 5 descriptor-write proof closeout checkpoint is accepted. It
  reconciles the descriptor syscall contract, QEMU descriptor-write smoke,
  retained Pi 5 local3 proof evidence, hardware-lock timeline, restore proof,
  residual risks, and blocked surfaces. It recommends
  phase7-syscall-abi-dispatch-closeout-checkpoint-20260529 as the next
  documentation-only Milestone 7.3 closeout task before any Milestone 7.4
  source inventory. stdin/read, close, dup, process loading, VFS/filesystem,
  shell, networking, SSH, live process-owned address spaces, blocking/readiness,
  signals, restart semantics, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor claims remain blocked.
- Phase 7 syscall ABI/dispatch closeout checkpoint is accepted. It reconciles
  all accepted Milestone 7.3 tasks, commits, retained QEMU/substitute and Pi 5
  evidence paths, validation gates, diagnostic-surface quarantine, hardware
  lock/restore proof, and deferred surfaces. Milestone 7.3 is closed for the
  bounded lower-AArch64 svc #0 syscall ABI and dispatch frontier: x8 syscall
  numbers, x0-through-x5 arguments, x0 return/-errno encoding, stable
  talos_nop and unknown-syscall returns, copy_from_user/copy_to_user helper
  plumbing, proof-only talos_copy_probe, and talos_write fd 1/fd 2 writes to
  runtime-console0 through proof-owned inherited stdio descriptors. It
  recommends phase7-file-descriptor-table-source-inventory-20260529 as the
  next documentation-only Milestone 7.4 task. stdin/read, close, dup, process
  loading, VFS/filesystem, shell, networking, SSH, live process-owned address
  spaces, blocking/readiness, signals, restart semantics, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
  claims remain blocked.
- Phase 7 file descriptor table source inventory is accepted. It maps accepted
  descriptor table data-model owners, talos_write syscall dispatch, copy helper
  and user-memory boundaries, runtime-console/TTY stdio backing,
  scheduler/task process-owner vocabulary, deferred VFS/filesystem/device
  surfaces, and retained QEMU/Pi 5 descriptor-write evidence. It recommends
  phase7-process-descriptor-table-contract-20260529 as the next bounded
  documentation-only Milestone 7.4 task and does not add implementation, QEMU,
  Pi 5 hardware, or hardware-lock work.
- Phase 7 process descriptor table contract is accepted. It defines a
  ProcessOwnerId-backed descriptor-table owner, inherited stdio installation,
  runtime-console0 stdout/stderr backing, current-owner descriptor-table
  lookup, retained descriptor error behavior, and the next bounded
  phase7-process-descriptor-table-core-20260529 implementation task. PID
  allocation, process loading, close/dup/read syscalls, VFS/filesystem, stdin
  behavior, shell, networking, SSH, physical proof, and full POSIX descriptor
  claims remain blocked.
- Phase 7 process descriptor table core is accepted. It implements only the
  target-independent ProcessDescriptorOwner and bounded ProcessDescriptorStore
  for ProcessOwnerId-backed inherited stdio tables, current-owner lookup, and
  deterministic -EBADF/-EINVAL/-EMFILE error behavior. Live syscall routing,
  close/dup/read syscalls, process loading, VFS/filesystem, shell, networking,
  SSH, physical proof, and full POSIX descriptor claims remain blocked.
- Phase 7 QEMU process descriptor stdio smoke plan is accepted. It defines the
  QEMU/substitute proof that talos_write fd 1/fd 2 must use a
  ProcessOwnerId-backed process-owned inherited stdio table rather than the
  earlier proof-owned table, while preserving fd/error regressions,
  copy-probe quarantine, diagnostic-marker quarantine, and blocked physical
  claims.
- Phase 7 QEMU process descriptor stdio smoke core is accepted. It proves in
  QEMU/substitute output that lower-AArch64 talos_write fd 1/fd 2 resolves the
  current ProcessOwnerId through ProcessDescriptorStore and writes through the
  process-owned inherited stdio table to runtime-console0. It preserves fd 0
  and fd 99 -EBADF, guard-range -EFAULT, reserved-register -EINVAL, talos_nop,
  unknown-syscall -ENOSYS, copy-probe quarantine, diagnostic-marker
  quarantine, and exact PASS/classification evidence. Pi 5 physical proof,
  stdin/read, close/dup/read, process loading, VFS/filesystem, shell,
  networking, SSH, and full POSIX descriptor claims remain blocked.
- Phase 7 process descriptor table closeout is accepted. It closes the first
  process-owned descriptor-table slice at the QEMU/substitute evidence level,
  records the accepted contract/core/smoke commits and evidence path, and
  preserves blocked Pi 5 physical proof, stdin/read, close/dup/read,
  descriptor lifetime and close semantics, process loading, VFS/filesystem,
  shell, networking, SSH, and full POSIX descriptor claims. The next bounded
  Milestone 7.4 task should be supervisor-planned as a documentation-only
  descriptor lifetime and close-semantics source inventory.
- Phase 7 descriptor lifetime and close source inventory is accepted. It maps
  src/posix.rs table-local close/dup behavior, ProcessDescriptorStore
  owner-table mutation, inherited stdio lifetime, retained descriptor evidence,
  missing close/double-close/reuse/dup unit evidence, and open-file-description
  finalization gaps. It recommends
  phase7-descriptor-lifetime-close-contract-20260529 as the next
  documentation-only Milestone 7.4 task. Close/dup/read syscalls, process
  loading, VFS/filesystem, shell, networking, SSH, physical close/dup/read
  proof, and full POSIX descriptor readiness remain blocked.
- Phase 7 descriptor lifetime and close contract is accepted. It defines the
  supported table-local close rule, process-owned close lookup through
  ProcessDescriptorStore, EBADF cases, dup/reuse interaction, and deferred
  open-file-description finalization. It recommends
  phase7-descriptor-close-core-20260529 as the next target-independent
  Milestone 7.4 task. Close/dup/read syscalls, process loading,
  VFS/filesystem, shell, networking, SSH, physical close/dup/read proof,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 descriptor close core closeout is accepted. It records the accepted
  source inventory, contract, target-independent close helper implementation,
  changed files, focused unit tests, and validation gates. It accepts only
  ProcessDescriptorStore::close_current_descriptor() applying table-local close
  semantics to the current owner. Close/dup/read syscalls, lower-EL ABI, QEMU
  close/dup/read smoke, Pi 5 physical proof, process loading, VFS/filesystem,
  shell, networking, SSH, object finalization, and full POSIX descriptor
  readiness remain blocked. The next bounded Milestone 7.4 task should be a
  documentation-only close/dup/read syscall source inventory.
- Phase 7 close syscall core is accepted. It adds the target-independent
  talos_close syscall number/dispatch path and routes close through
  ProcessDescriptorStore::close_current_descriptor() with focused no_std
  tests. QEMU syscall and descriptor-write regression smokes still pass, but
  QEMU/Pi 5 close proof, dup/read, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, and full POSIX descriptor readiness
  remain blocked. The next bounded Milestone 7.4 task should be
  phase7-qemu-close-syscall-smoke-plan-20260529.
- Phase 7 QEMU close syscall smoke plan is accepted. It defines the later
  qemu_close_syscall_smoke QEMU/substitute invariant for talos_close on fd 1
  and fd 2 through the current ProcessOwnerId-backed descriptor table,
  closed-descriptor talos_write -EBADF behavior, unaffected descriptor
  behavior, scalar syscall regressions, and proof-only diagnostic quarantine.
  The next bounded Milestone 7.4 task should be
  phase7-qemu-close-syscall-smoke-core-20260529. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU close syscall smoke core is accepted. It adds and retains
  qemu_close_syscall_smoke evidence proving current-owner talos_close on fd 1
  and fd 2, closed-descriptor talos_write -EBADF behavior without
  runtime-console0 side effects, unaffected fd 2 writes after closing fd 1 and
  after a failed reserved close, repeated-close/badfd EBADF behavior, and
  talos_nop/unknown/copy-probe/diagnostic quarantine regressions. The next
  bounded Milestone 7.4 task should be
  phase7-close-syscall-closeout-checkpoint-20260529. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 close syscall closeout is accepted. It reconciles the accepted close
  syscall source inventory, contract, target-independent core, QEMU smoke plan,
  retained QEMU/substitute close smoke evidence, validation gates, and deferred
  surfaces. The accepted capability remains stable talos_close x8 = 2 through
  the current ProcessOwnerId-backed ProcessDescriptorStore at QEMU/substitute
  evidence level. The next bounded Milestone 7.4 task should be a
  documentation-only Pi 5 close syscall proof plan. Pi 5 physical close proof,
  dup/read syscalls, process loading, VFS/filesystem, shell, networking, SSH,
  object finalization, and full POSIX descriptor readiness remain blocked.
- Phase 7 Pi 5 close syscall proof plan is accepted. It defines the serialized
  rpi5_close_syscall_proof hardware plan, including lock ownership, candidate
  identity, fresh serial/TFTP requirements, inconclusive-run triage,
  restoration proof, exact close/write/error/quarantine/classification/PASS
  output, and deferred surfaces. No hardware run, archive publication, or
  physical close claim is made by the plan. The next bounded Milestone 7.4
  task should be phase7-pi5-close-syscall-proof-20260529. Dup/read, process
  loading, VFS/filesystem, shell, networking, SSH, object finalization, and
  full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU dup syscall closeout is accepted. Subsequent Milestone 7.4 work
  has accepted the Pi 5 close proof and closeout, dup syscall contract, dup
  syscall core, QEMU dup smoke plan, QEMU dup smoke core, and this
  documentation-only closeout. The retained QEMU/substitute dup evidence proves
  fd 1 duplicates to fd 3, full-table -EMFILE, reserved-register -EINVAL,
  writes through source and duplicate stdout descriptors, close(fd 1)
  preserving fd 3, closed-descriptor -EBADF cases, scalar regressions,
  copy-probe quarantine, diagnostic-marker quarantine, and
  classification=qemu-dup-syscall-smoke-complete plus PASS. The next bounded
  Milestone 7.4 task should be
  phase7-pi5-dup-syscall-proof-plan-20260529. Pi 5 physical dup proof,
  read/stdin behavior, process loading, VFS/filesystem, shell, networking,
  SSH, object finalization, dup2/fcntl, and full POSIX descriptor readiness
  remain blocked.
- Phase 7 Pi 5 dup syscall proof plan is accepted. It defines the serialized
  rpi5_dup_syscall_proof hardware plan, including lock ownership, candidate
  identity, fresh serial/TFTP requirements, inconclusive-run triage,
  restoration proof, exact dup/write/close/error/quarantine/classification/PASS
  output, and deferred surfaces. No hardware run, archive publication, or
  physical dup claim is made by the plan. The next bounded Milestone 7.4 task
  should be phase7-pi5-dup-syscall-proof-20260529. Read/stdin behavior,
  process loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.
- Phase 7 Pi 5 dup syscall proof is accepted. Retained local8 physical serial
  evidence proves current-owner lookup, fd 1 dup to fd 3, full-table -EMFILE,
  reserved-register -EINVAL, source and duplicate stdout writes, close(fd 1)
  preserving fd 3, duplicate close, closed-descriptor -EBADF, talos_nop,
  unknown-syscall -ENOSYS, copy-probe quarantine,
  classification=pi5-dup-syscall-proof-complete, and PASS. local7 is the
  accepted known-good production-timer control after earlier inconclusive
  local4/local5/local6 evidence. The boot tree was restored to the
  pre-pi5-dup-syscall-proof-local1-20260529 snapshot. The next bounded
  Milestone 7.4 task should be
  phase7-pi5-dup-syscall-proof-closeout-checkpoint-20260529. Read/stdin
  behavior, process loading, VFS/filesystem, shell, networking, SSH, object
  finalization, dup2/fcntl, and full POSIX descriptor readiness remain
  blocked.
- Phase 7 Pi 5 dup syscall proof closeout is accepted. It reconciles the dup
  contract/core, retained QEMU/substitute smoke, serialized local8 Pi 5
  hardware proof, local7 known-good control, hardware-lock timeline, restore
  proof, residual risks, and deferred surfaces. The accepted frontier is a
  focused physical talos_dup x8 = 3 proof through the current
  ProcessOwnerId-backed ProcessDescriptorStore, including fd 1 to fd 3,
  -EMFILE, -EINVAL, source/duplicate writes, independent close behavior,
  -EBADF cases, scalar/unknown regressions, copy-probe quarantine, diagnostic
  marker quarantine, classification=pi5-dup-syscall-proof-complete, and PASS.
  The next bounded Milestone 7.4 task should be a supervisor-queued
  documentation-only read/stdin source inventory. Read/stdin behavior, process
  loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.
- Phase 7 QEMU read/stdin smoke core is accepted. Retained QEMU/substitute
  evidence proves qemu_read_stdin_smoke through the lower-AArch64 stable
  talos_read path with fd 0 duplication, fixed proof stdin, errno cases,
  short-read, EOF, scalar regressions, copy-probe quarantine,
  diagnostic-marker quarantine, classification, and PASS. The next bounded
  Milestone 7.4 task should be
  phase7-read-stdin-closeout-checkpoint-20260529. Pi 5 physical read proof,
  runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, dup2/fcntl, and full POSIX descriptor
  readiness remain blocked.
- Phase 7 read/stdin closeout checkpoint is accepted. The checkpoint reconciles
  the accepted inventory, contract, target-independent core, QEMU smoke plan,
  retained QEMU/substitute evidence, residual risks, and deferred surfaces. The
  next mechanically derivable Milestone 7.4 task should be
  phase7-pi5-read-stdin-proof-plan-20260530, queued explicitly by the
  supervisor before any Pi 5 hardware action.
- Phase 7 Pi 5 read/stdin proof plan is accepted. It defines the serialized
  rpi5_read_stdin_proof hardware plan for carrying the accepted fixed-stdin
  talos_read QEMU/substitute invariant to Raspberry Pi 5. The plan requires
  lock ownership, candidate identity, archive/kernel hashes, fresh serial and
  TFTP evidence, inconclusive-run triage, restoration proof, exact fd 0/fd 3
  read, errno, EOF, scalar-regression, copy-probe quarantine,
  diagnostic-marker quarantine, classification, and PASS output. No hardware
  run, archive publication, or physical read claim is made by the plan. The
  next bounded Milestone 7.4 task should be
  phase7-pi5-read-stdin-proof-20260530. runtime-console0/TTY/hardware stdin,
  process loading, VFS/filesystem, shell, networking, SSH, object finalization,
  dup2/fcntl, and full POSIX descriptor readiness remain blocked.

Milestone 7.4: File Descriptor Table

- Implement per-process descriptor tables.
- Model standard input, output, error, pipes, devices, and later sockets through one interface.
- Status: closed for the bounded descriptor-table frontier accepted by
  phase7-file-descriptor-table-closeout-checkpoint-20260530. The accepted
  frontier covers ProcessOwnerId-backed inherited stdio, descriptor-backed
  stdout/stderr writes, talos_close, talos_dup, and fixed-proof-stdin
  talos_read through fd 0/fd 3. Pipes, devices beyond runtime-console0, TTY or
  hardware stdin, filesystems, sockets, process loading, shell, networking,
  SSH, object finalization, dup2/fcntl, signals, wait queues, nonblocking I/O,
  RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
  descriptor readiness remain deferred.

Phase 7 final frontier checkpoint:

- Phase 7 final frontier source inventory is accepted. It maps Phase 7.1
  through Phase 7.4 accepted capabilities, commit/evidence anchors, deferred
  surfaces, and residual risks. No implementation, QEMU run, Pi 5 run, archive
  publication, hardware-lock acquisition, or Phase 8 transition was performed.
  It reports no remaining bounded Phase 7 implementation or evidence blocker
  before the final closeout checkpoint and recommends
  phase7-final-closeout-checkpoint-20260530 as the next mechanically unblocked
  task.
- Phase 7 final closeout checkpoint is accepted. Phase 7 is closed for the
  bounded POSIX/EL0/syscall/copy-helper/descriptor frontier accepted by the
  Phase 7.1 through Phase 7.4 closeouts. The checkpoint records no remaining
  bounded Phase 7 implementation or evidence blocker before Phase 8 source
  inventory planning, and it recommends
  phase8-filesystem-program-loading-source-inventory-20260530 as the next
  mechanically derivable documentation-only task once the durable
  phaseCheckpointStatus recommendation flag is set. This does not accept
  filesystem/program loading, shell, networking, SSH, runtime-console0/TTY or
  hardware stdin, object finalization, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, or full POSIX readiness.

Acceptance criteria:

- A test process can read/write through descriptor-backed console streams.
- Descriptor lifetime and close semantics are documented.
- Phase 7 closes only for the bounded accepted frontier and explicitly leaves
  Phase 8 runtime capability to later tasks.

## Phase 8: Filesystem and Program Loading

Goal: make Talos able to run more than built-in commands.

Milestone 8.1: Initramfs or Ramfs

- Add an embedded or TFTP-loaded initramfs for early files.
- Implement path lookup, file metadata, and read-only file contents.
- Phase 8 filesystem/program-loading source inventory is accepted. It maps
  existing owners and missing contracts for POSIX path copying, VFS/filesystem
  objects, descriptor inheritance, process identity, address-space setup,
  executable images, argv/envp, and boot/test scenarios. It recommends
  phase8-readonly-initramfs-vfs-contract-20260530 as the next
  documentation-only task and keeps ELF/program loading, process creation,
  shell, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy blocked.
- The read-only initramfs/VFS contract is accepted. It defines the immutable
  initial filesystem content model, root/directory/regular-file vocabulary,
  path-copy and lookup rules, descriptor-facing regular-file read semantics,
  errno precedence, deterministic fixture expectations, and deferred surfaces.
  It recommends phase8-readonly-initramfs-vfs-smoke-plan-20260530 next and
  keeps target-independent core implementation, QEMU runtime evidence, Pi 5
  hardware proof, ELF/program loading, process creation, shell, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy blocked
  until later explicit tasks accept their gates.
- The read-only initramfs/VFS smoke plan is accepted. It defines the
  qemu_readonly_initramfs_vfs_smoke scenario, deterministic fixture contents,
  lookup/read/offset/EOF observations, ENOENT/ENOTDIR/EISDIR/ENAMETOOLONG/
  EBADF/EFAULT/EINVAL/ENOTSUP negative cases, exact PASS/classification lines,
  retained QEMU/substitute evidence path, failure classification, and
  regression gates. It recommends
  phase8-readonly-initramfs-vfs-core-20260530 next, followed by
  phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530 after the core is
  accepted. QEMU runtime evidence, Pi 5 hardware proof, ELF/program loading,
  process creation, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- The target-independent read-only initramfs/VFS core is accepted. It adds the
  immutable fixture object model, deterministic root/directory/regular-file
  nodes, normalized absolute and current-directory-relative lookup,
  regular-file open-file descriptions, all-or-nothing copy_to_user-backed
  reads, offset/EOF behavior, and focused no_std unit tests for accepted
  success and failure cases. It does not wire the filesystem to production
  lower-EL syscalls, run QEMU, run Pi 5 hardware, publish a boot archive, parse
  firmware/TFTP initramfs envelopes, or unblock ELF/program loading, process
  creation, shell, networking, SSH, writable filesystems, persistent storage,
  RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy. The next
  bounded task is phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530.
- The QEMU/substitute read-only initramfs/VFS smoke is accepted. It proves the
  planned fixture identity, lookup, regular-file reads, offset/EOF behavior,
  and deterministic ENOENT/ENOTDIR/EISDIR/ENAMETOOLONG/EBADF/EFAULT/EINVAL/
  ENOTSUP cases, and retains
  tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log
  with classification=qemu-readonly-initramfs-vfs-smoke-complete and PASS.
  It does not accept Pi 5 hardware proof, boot archive publication,
  descriptor-backed production filesystem syscalls, open syscall ABI,
  firmware/TFTP initramfs delivery, ELF/program loading, process creation,
  shell, networking, SSH, writable filesystems, persistent storage, RP1/PCIe,
  UART interrupt ownership, or DMA/cache-driver policy. The read-only
  initramfs/VFS closeout checkpoint is accepted and recommends
  phase8-program-loader-source-inventory-20260530 as the next bounded
  documentation-only task before any loader implementation.
- The read-only initramfs/VFS closeout checkpoint is accepted. It reconciles
  the accepted contract, smoke plan, target-independent core, retained
  QEMU/substitute evidence, deferred surfaces, and residual risks, and it
  recommends phase8-program-loader-source-inventory-20260530 as the next
  bounded documentation-only task. It does not accept descriptor-backed
  filesystem syscalls, executable /bin/init, ELF/program loading, process
  creation, shell, Pi 5 hardware proof, networking, SSH, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy.

Acceptance criteria:

- A diagnostic command or test process can list and read files from the initial filesystem.

Milestone 8.2: VFS

- Add VFS nodes for regular files, directories, devices, and pipes.
- Keep interfaces compatible with future persistent filesystems.

Acceptance criteria:

- Common file operations route through the VFS, not ad hoc shell logic.

Milestone 8.3: Program Loader

- Choose an executable format for early user programs.
- Load a program from initramfs, map it into a process, and pass arguments.
- Phase 8 program-loader source inventory is accepted. It maps the accepted
  read-only initramfs/VFS regular-file input, current source owners for
  filesystem bytes, POSIX errors, user-memory permissions, lower-EL proof
  payloads, scheduler/process-owner placeholders, descriptor inheritance, and
  evidence conventions. It also records missing contracts for executable
  format selection, ELF/header validation, segment permissions, zero-fill,
  entry-point validation, user stack and argv/envp layout, loader error
  mapping, process-install ownership, and descriptor inheritance. The next
  bounded task is phase8-program-loader-format-contract-20260530. ELF parsing,
  loader implementation, process creation, exec/spawn/wait, shell, Pi 5
  hardware proof, writable filesystems, persistent storage, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked until later explicit tasks accept their gates.
- Phase 8 program-loader format contract is accepted. It selects the first
  executable format policy: a narrow static ELF64/AArch64 ET_EXEC subset from
  the accepted read-only initramfs/VFS regular-file boundary. It defines
  header and program-header validation, dynamic/interpreter rejection,
  PT_LOAD segment permission mapping, W^X rejection, user-range and overlap
  checks, BSS zero-fill, entry-point validation, deterministic loader errors,
  and the process/address-space/stack/descriptor boundaries that remain later
  responsibilities. The next bounded task is
  phase8-qemu-program-loader-smoke-plan-20260530. Loader Rust implementation,
  process address-space installation, argv/envp stack construction,
  process creation, exec/spawn/wait, shell, descriptor-backed filesystem
  syscalls, Pi 5 hardware proof, writable filesystems, persistent storage,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy remain blocked until later explicit tasks accept their gates.
- Phase 8 QEMU/substitute program-loader smoke plan is accepted. It defines
  the qemu_program_loader_smoke scenario, fixture identity
  phase8-program-loader-elf64-aarch64-v1, image-plan-only success
  observations for a narrow static ELF64/AArch64 ET_EXEC /bin/init fixture,
  deterministic negative cases for bad magic, dynamic interpreter, W+X
  segment, out-of-user-range segment, overlap, bad entry, and file-range
  overflow, retained evidence path
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log,
  PASS/classification lines, and conditional regression gates. The next
  bounded implementation task should be phase8-program-loader-core-20260530
  only after supervisor planning queues it with explicit scope and gates.
  Loader core implementation, process address-space installation, lower-EL
  launch of a loaded image, argv/envp stack construction, process creation,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, Pi 5 hardware
  proof, writable filesystems, persistent storage, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy remain blocked until
  later explicit tasks accept their gates.
- Phase 8 program-loader core is accepted. It adds the target-independent
  ELF64/AArch64 static ET_EXEC image-plan validator for immutable /bin/init
  bytes from the read-only initramfs/VFS fixture, reports digest/source
  identity, ordered UserText/UserData segments, file-copy ranges, explicit BSS
  zero-fill, entry placement, total footprint, and deterministic loader errors
  before any process-owned install surface exists.
- Phase 8 QEMU/substitute program-loader smoke core is accepted. The
  qemu_program_loader_smoke scenario and
  scripts/qemu-program-loader-smoke.sh retain
  tasks/evidence/2026-05-30-qemu-program-loader-smoke-core/qemu-program-loader-smoke.log
  with fixture identity phase8-program-loader-elf64-aarch64-v1, digest
  0x3892eed223900c65, success image-plan lines, all seven required negative
  errno lines with partial-install=false, final
  classification=qemu-program-loader-smoke-complete, and
  qemu-program-loader-smoke: PASS. This evidence is QEMU/substitute only; Pi 5
  hardware proof, process address-space installation, lower-EL launch of the
  loaded image, argv/envp stack construction, process creation, exec/spawn/wait,
  shell, descriptor-backed filesystem syscalls, writable filesystems,
  persistent storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain blocked until later explicit tasks accept
  their gates.
- Phase 8 loader-from-VFS input is accepted. The `plan_phase8_init_image`
  boundary now opens `/bin/init` as a read-only initramfs regular file and
  reads it through the kernel file-object path before running the existing
  ELF64/AArch64 image-plan validator. The retained
  `qemu_program_loader_from_vfs_smoke` evidence proves VFS-sourced `/bin/init`
  bytes, image-plan-only success, and the existing deterministic negative
  loader matrix. Process launch, scheduler publication, argv/envp stack setup,
  shell behavior, Pi 5 hardware claims, writable filesystems, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  deferred.
- Phase 8 program-loader closeout checkpoint is accepted. It reconciles the
  accepted source inventory, format contract, smoke plan, target-independent
  core, retained QEMU/substitute evidence, validation gates, deferred surfaces,
  and residual risks for the image-plan-only frontier. It recommends
  phase8-process-install-source-inventory-20260530 as the next bounded
  documentation-only task and keeps process address-space installation,
  lower-EL launch, argv/envp stack construction, process creation,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, Pi 5 hardware
  proof, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy blocked until later explicit tasks accept their
  gates.
- Phase 8 process-install source inventory is accepted. It maps the source
  owners and gaps between the accepted ProgramImagePlan and any future
  process-owned address-space installation: frame allocation, page-table
  mutation, rollback, initial lower-EL frame, user stack, descriptor
  inheritance, process identity, and scheduler handoff. It recommends
  phase8-process-install-contract-20260530 as the next bounded
  documentation-only task. Rust implementation, QEMU execution, Pi 5 hardware
  proof, lower-EL launch of the loaded image, argv/envp construction,
  exec/spawn/wait, shell, descriptor-backed filesystem syscalls, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- Phase 8 process-install contract is accepted. It selects a
  target-independent metadata-only ProcessImageInstallPlan boundary derived
  from a validated ProgramImagePlan, with exact UserText/UserData permission
  preservation, ordered page records, clipped file-copy and zero-fill ranges,
  deterministic errors, and all-or-nothing semantics. It accepts no frame
  allocation, physical byte copy, page-table mutation, scheduler handoff,
  lower-EL launch, argv/envp, descriptor inheritance, shell, hardware, or
  filesystem syscall behavior.
- Phase 8 QEMU/substitute process-install smoke plan is accepted. It defines
  qemu_process_install_smoke, fixture identity
  phase8-program-loader-elf64-aarch64-v1, install boundary identity
  phase8-process-install-plan-v1, retained evidence path
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
  final classification qemu-process-install-smoke-complete, PASS vocabulary,
  success observations for metadata-only ProcessImageInstallPlan derivation,
  deterministic rejection observations for bad plan invariants, overlap,
  permission widening, bad entry, and budget overflow, plus conditional
  regression gates. The next bounded implementation task should be
  phase8-process-install-core-20260530 only because supervisor planning has
  already queued it with explicit scope and gates. Physical page allocation,
  page-table mutation, lower-EL launch, argv/envp construction, process
  creation, exec/spawn/wait, shell, descriptor-backed filesystem syscalls,
  Pi 5 hardware proof, writable filesystems, persistent storage, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked until later explicit tasks accept their gates.
- Phase 8 process-install core is accepted. It adds the metadata-only
  ProcessImageInstallPlan owner in src/process_install.rs, deriving ordered
  page install records from a validated ProgramImagePlan while preserving
  UserText R-X plus UserData RW-/R-- permissions, exact fixture identity, source
  digest, entry point, total rounded footprint, clipped file-copy ranges,
  explicit zero-fill ranges, and the later action order
  allocate/copy/zero/map. The implementation is target-independent and returns
  deterministic POSIX-shaped errors for malformed plan invariants, overlap,
  permission widening, bad entry, budget overflow, and invalid source ranges.
  No frame allocation, physical byte copy, page-table mutation, process
  creation, descriptor mutation, lower-EL frame, runnable task, QEMU smoke,
  Pi 5 hardware proof, argv/envp construction, exec/spawn/wait, shell,
  writable filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
  DMA/cache-driver policy is accepted by this core.
- Phase 8 QEMU/substitute process-install smoke core is accepted. It adds
  qemu_process_install_smoke routing, retained evidence at
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log,
  and PASS/classification checks for metadata-only success, exact page
  permission preservation, zero physical side effects, and deterministic
  no-partial-install rejections. The next bounded task should be the queued
  phase8-process-install-closeout-checkpoint-20260530 if dependencies remain
  satisfied. Physical page allocation, page-table mutation, lower-EL launch,
  argv/envp construction, process creation, exec/spawn/wait, shell,
  descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- Phase 8 process-install closeout checkpoint is accepted. It reconciles the
  accepted source inventory, contract, QEMU/substitute smoke plan,
  metadata-only core, retained QEMU/substitute process-install smoke evidence,
  and deferred surfaces. The accepted capability remains
  target-independent ProcessImageInstallPlan derivation plus retained
  QEMU/substitute no-partial-install evidence only. No explicit queued
  follow-up task remains; supervisor planning is required before the worker may
  promote another Phase 8.3 task. Physical process address-space mutation,
  frame allocation, physical byte copy, page-table mutation, teardown,
  lower-EL launch, argv/envp construction, process creation, exec/spawn/wait,
  shell, descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- Phase 8 process address-space source inventory is accepted. It maps the
  source owners and accepted inputs for ProcessImageInstallPlan,
  ProgramImagePlan, user permission vocabulary, frame ownership vocabulary,
  page-table/translation helpers, scheduler/process placeholders, and
  QEMU/Pi 5 evidence producers. It separates accepted metadata-only
  process-install behavior from unaccepted physical frame allocation, byte
  copy, page-table mutation, address-space switching, lower-EL launch,
  process creation, descriptor inheritance, and teardown. It recommends
  phase8-process-address-space-contract-20260530 as the next bounded
  documentation-only task to define the process address-space owner/lifetime,
  frame lease/release policy, page-table root ownership, mapping order,
  rollback/teardown rules, deterministic errors, and evidence boundary before
  any implementation. Hardware proof, argv/envp, exec/spawn/wait, shell,
  descriptor-backed filesystem syscalls, writable filesystems, persistent
  storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
  DMA/cache-driver policy remain blocked until later explicit tasks accept
  their gates.
- Phase 8 process address-space contract is accepted. It selects the first
  target-independent ProcessAddressSpace boundary: an installed address-space
  record with explicit identity, owner label, model root/table leases,
  user-frame leases, ordered mappings, copy/zero accounting, publication
  state, all-or-nothing rollback, idempotent teardown, and deterministic
  POSIX-shaped errors. It deliberately does not accept real AArch64
  descriptor construction, TTBR0_EL1/TTBR1_EL1 switching, TCR/MAIR/SCTLR
  policy, ASIDs, TLB invalidation, lower-EL launch, argv/envp construction,
  scheduler handoff, process table/PID/wait/exit, descriptor inheritance,
  filesystem syscalls, Pi 5 hardware proof, networking, or SSH. The next
  bounded task should be
  phase8-qemu-process-address-space-smoke-plan-20260530 to define retained
  QEMU/substitute evidence for success, rejection, rollback, no-leak, and
  teardown observations before implementation.
- Phase 8 QEMU/substitute process address-space smoke plan is accepted. It
  defines qemu_process_address_space_smoke, fixture identity
  phase8-program-loader-elf64-aarch64-v1, install boundary
  phase8-process-install-plan-v1, address-space boundary
  phase8-process-address-space-model-v1, retained evidence path
  tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log,
  classification qemu-process-address-space-smoke-complete, PASS vocabulary,
  target-independent ProcessAddressSpace success observations, deterministic
  rejection/no-partial-install and no-leak observations, teardown idempotence,
  and conditional regression gates. The next bounded task should be
  phase8-process-address-space-core-20260530 because supervisor planning has
  already queued it with explicit scope and gates. Hardware page-table
  mutation, TTBR/TCR switching, lower-EL launch, argv/envp, process creation,
  scheduler handoff, shell, filesystem syscalls, Pi 5 hardware proof,
  networking, and SSH remain blocked.
- Phase 8 process page-table materialization source inventory is accepted. It
  maps the source owners and accepted inputs for ProgramImagePlan,
  ProcessImageInstallPlan, the target-independent ProcessAddressSpace model,
  POSIX user-range/permission vocabulary, early frame ownership vocabulary,
  translation helpers, scheduler owner placeholders, QEMU/substitute evidence,
  and Pi 5 proof-local lower-EL tables. It separates accepted model leases and
  mappings from unaccepted architecture-specific materialization: real user
  frames, page-table roots/table pages, descriptor bits, kernel mapping
  sharing, ASID/TTBR/TCR/MAIR/TLB policy, rollback, teardown, and activation
  evidence. It recommends
  phase8-process-page-table-materialization-contract-20260530 as the next
  bounded documentation-only task. Lower-EL launch, argv/envp, process
  lifecycle, exec/spawn/wait, shell, descriptor-backed filesystem syscalls,
  Pi 5 hardware proof, writable filesystems, persistent storage, networking,
  SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
  blocked until later explicit tasks accept their gates.
- Phase 8 process page-table materialization contract is accepted. It selects
  a non-activating AArch64 descriptor-image and user-frame materialization
  boundary with identity phase8-process-page-table-materialization-v1. The
  contract fixes inputs from ProgramImagePlan, ProcessImageInstallPlan, and
  ProcessAddressSpace; defines user-frame/root/table-page ownership,
  UserText/UserData descriptor policy including R-only data, kernel mapping and activation
  boundaries, ASID/TTBR/TCR/TLB blocked surfaces, rollback, teardown, and
  deterministic errors; and names
  phase8-qemu-process-page-table-materialization-smoke-plan-20260530 as the
  next bounded documentation-only task. TTBR activation, lower-EL launch,
  argv/envp, process lifecycle, exec/spawn/wait, shell,
  descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
  filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
  ownership, and DMA/cache-driver policy remain blocked until later explicit
  tasks accept their gates.
- Phase 8 QEMU/substitute process page-table materialization smoke plan is
  accepted. It defines qemu_process_page_table_materialization_smoke, fixture
  identity phase8-program-loader-elf64-aarch64-v1, install boundary
  phase8-process-install-plan-v1, address-space boundary
  phase8-process-address-space-model-v1, materialization boundary
  phase8-process-page-table-materialization-v1, retained evidence path
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log,
  classification qemu-process-page-table-materialization-smoke-complete, PASS
  vocabulary, non-activating descriptor-image success observations,
  deterministic rejection/no-partial-materialization and no-leak observations,
  teardown idempotence, and conditional regression gates. The next bounded task
  should be phase8-process-page-table-materialization-core-20260530 because
  supervisor planning has already queued it with explicit scope and gates. TTBR
  activation, ASID/TLB policy, lower-EL launch, argv/envp, process lifecycle,
  shell, filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain
  blocked.
- Phase 8 process page-table materialization core is accepted. It adds
  src/process_page_table_materialization.rs and wires it into src/main.rs as a
  non-activating materialization boundary. The implementation consumes the
  accepted ProgramImagePlan, ProcessImageInstallPlan, and ProcessAddressSpace
  records; produces owned root/table-page/user-frame leases, ordered AArch64
  EL0 descriptor records, copied/zeroed byte accounting, rollback accounting,
  idempotent teardown, activation_blocked=true, and
  kernel_mapping_policy=activation-blocked-no-kernel-half. Tests cover success,
  permission preservation including R-only data descriptors, deterministic bad-input and activation-request
  rejection, unsupported topology, resource exhaustion rollback, copy/zero
  failure rollback, and idempotent teardown. The next bounded task should be
  phase8-qemu-process-page-table-materialization-smoke-core-20260530 to retain
  the accepted QEMU/substitute evidence. TTBR activation, ASID/TLB policy,
  lower-EL launch, argv/envp, process lifecycle, shell, filesystem syscalls,
  Pi 5 hardware proof, networking, and SSH remain blocked.
- Phase 8 QEMU/substitute process page-table materialization smoke core is
  accepted. It adds qemu_process_page_table_materialization_smoke and retains
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log
  with classification qemu-process-page-table-materialization-smoke-complete
  and PASS. The smoke proves non-activating descriptor-image success,
  UserText/UserData frame and descriptor permission preservation, no
  TTBR/TLB/scheduler/lower-EL/runnable side effects, teardown idempotence, and
  deterministic no-partial-materialization/no-leak rejection cases. The next
  bounded task should be the queued
  phase8-process-page-table-materialization-closeout-checkpoint-20260530. TTBR
  activation, ASID/TLB policy, lower-EL launch, argv/envp, process lifecycle,
  shell, filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain
  blocked.
- Phase 8 process page-table materialization closeout checkpoint is accepted.
  It reconciles the accepted materialization inventory, contract, smoke plan,
  core implementation, and retained QEMU/substitute smoke evidence. The
  accepted frontier is non-activating AArch64 descriptor-image/user-frame
  materialization for immutable /bin/init below TTBR activation, with
  rollback/no-leak and idempotent teardown evidence. No explicit queued
  follow-up task remains after this checkpoint, so supervisor planning is
  required before the worker may promote another Phase 8.3 task. TTBR
  activation, TCR/MAIR/SCTLR mutation, ASID/TLB policy, lower-EL launch,
  argv/envp, process lifecycle, shell, filesystem syscalls, Pi 5 hardware
  proof, networking, and SSH remain blocked.
- Phase 8 initial process launch source inventory is accepted. It maps the
  accepted loader, process-install, ProcessAddressSpace, and non-activating
  materialization records to the next launch gaps: entry/SP provenance,
  initial user stack prerequisites, lower-EL frame construction,
  TTBR/TCR/MAIR/SCTLR and ASID/TLB activation boundaries, kernel reachability,
  scheduler runnable-publication blockers, trap/return evidence, and
  descriptor/userland state. It recommends
  phase8-initial-process-launch-contract-20260530 as the next bounded
  documentation-only task. Implementation, QEMU execution, TTBR activation,
  lower-EL ERET to /bin/init, argv/envp, process lifecycle, shell, filesystem
  syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
- Phase 8 initial process launch contract is accepted. It selects
  phase8-initial-process-launch-plan-v1 as a target-independent
  launch-preparation boundary, not a live launch. The plan consumes the
  accepted ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace, and
  ProcessPageTableMaterialization records, validates entry provenance and
  UserText descriptor coverage, records user_sp_state as
  blocked-missing-initial-user-stack, records activation_state as
  blocked-no-ttbr-activation, and defines saved-frame intent without writing
  registers. It names
  phase8-qemu-initial-process-launch-smoke-plan-20260530 as the next bounded
  documentation-only task. TTBR activation, lower-EL ERET, initial user stack,
  argv/envp, process lifecycle, scheduler runnable publication, shell,
  filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain
  blocked.
- Phase 8 QEMU/substitute initial process launch smoke plan is accepted. It
  defines qemu_initial_process_launch_smoke as the first evidence boundary for
  the accepted launch-preparation record, with retained evidence at
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log,
  classification qemu-initial-process-launch-smoke-complete, and PASS
  vocabulary. The smoke must prove accepted fixture lineage, entry provenance,
  blocked initial stack state, blocked activation state, saved-frame intent
  without register writes, no launch side effects, and deterministic
  no-partial-launch/no-runnable-publication rejection cases. The next bounded
  task should be the queued phase8-initial-process-launch-core-20260530. TTBR
  activation, lower-EL ERET, initial user stack, argv/envp, process lifecycle,
  scheduler runnable publication, shell, filesystem syscalls, Pi 5 hardware
  proof, networking, and SSH remain blocked.
- Phase 8 initial process launch core is accepted. It adds
  phase8-initial-process-launch-plan-v1 as a target-independent
  launch-preparation record for immutable /bin/init. The core validates
  loader, install, process address-space, and non-activating page-table
  materialization lineage; preserves entry provenance through UserText mapping
  and an EL0-executable descriptor; records blocked initial-stack and
  activation states; exposes saved-frame intent without register writes; and
  rejects runnable commit, activation, stack-required launch, and scheduler
  publication requests with ENOSYS/no-partial-launch/no-runnable-publication
  behavior. QEMU/substitute evidence for the new boundary remains the next
  queued smoke-core task. TTBR activation, lower-EL ERET, initial user stack,
  argv/envp, process lifecycle, scheduler runnable publication, shell,
  filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain
  blocked.
- Phase 8 QEMU/substitute initial process launch smoke core is accepted. The
  retained log at
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log
  records the accepted fixture, install, address-space, materialization, and
  launch-plan identities; success publication of InitialProcessLaunchPlan;
  entry provenance through UserText and EL0-executable descriptor evidence;
  saved-frame intent without register writes; zero TTBR/TLB/scheduler/process
  table/descriptor/lower-EL side effects; ENOSYS runnable commit rejection;
  deterministic negative cases; and final
  qemu-initial-process-launch-smoke-complete/PASS lines. Pi 5 hardware proof,
  runnable lower-EL process launch, initial user stack, argv/envp, process
  lifecycle, scheduler publication, shell, filesystem syscalls, networking,
  and SSH remain blocked.
- Phase 8 initial process launch closeout checkpoint is accepted. It
  reconciles the launch-preparation source inventory, contract, smoke plan,
  core, retained QEMU/substitute evidence, deferred surfaces, and planning
  state. The accepted frontier remains target-independent
  InitialProcessLaunchPlan construction plus QEMU/substitute
  no-partial-launch/no-runnable-publication evidence only. No executable user
  process, initial stack, TTBR activation, lower-EL ERET, process lifecycle,
  filesystem syscalls, hardware proof, networking, or SSH capability is
  accepted.
- Phase 8 initial user stack source inventory is accepted. It maps current
  POSIX user-range/copy owners, loader/install/address-space/materialization
  lease and teardown owners, InitialProcessLaunchPlan blocked stack state,
  lower-EL saved-frame intent surfaces, scheduler placeholders, and proof-local
  diagnostic stack fixtures. It identifies the smallest next boundary as a
  target-independent initial stack record contract covering stack range,
  guard, top-SP alignment/provenance, frame/page ownership, zero/copy
  accounting, teardown, deterministic errors, and launch-plan stack-state
  integration. It recommends
  phase8-initial-user-stack-contract-20260530 as the next documentation-only
  task. Stack implementation, argv/envp/auxv/TLS setup, TTBR activation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem
  syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
- Phase 8 QEMU/substitute initial user stack smoke plan is accepted. It
  selects qemu_initial_user_stack_smoke, retained evidence path
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log,
  classification qemu-initial-user-stack-smoke-complete, and PASS vocabulary.
  The planned smoke must prove stack layout, guard range, top-SP alignment,
  stack-owned USER_DATA pages, zero/copy accounting, teardown, minimal empty
  startup metadata, model-only launch-plan stack-state integration,
  deterministic no-partial-stack/no-partial-launch rejection, and zero
  TTBR/TLB/lower-EL/scheduler/process-table/descriptor-table side effects.
  It names phase8-initial-user-stack-core-20260530 as the next bounded
  implementation task. Live TTBR activation, lower-EL ERET, scheduler
  publication, process lifecycle, broad argv/envp/auxv/TLS ABI, filesystem
  syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
- Phase 8 QEMU/substitute initial user stack smoke evidence is accepted. The
  retained qemu-initial-user-stack-smoke log proves the InitialUserStackPlan
  boundary identity, copied loader/install/address-space/materialization/
  launch lineage, fixed stack top and initial SP 0x0000_8000_0000_0000, usable
  and guard ranges, four zeroed USER_DATA stack-owned pages, one unmapped guard
  page, copied_bytes=0, zeroed_bytes=0x4000, minimal-empty-argc0 startup
  metadata, model-only launch-plan stack-ready binding, teardown, deterministic
  no-partial-stack/no-partial-launch rejection, zero TTBR/TLB/lower-EL/
  scheduler/process-table/descriptor-table side effects, and
  qemu-initial-user-stack-smoke-complete/PASS lines. Live TTBR activation,
  lower-EL ERET, scheduler publication, process lifecycle, broad
  argv/envp/auxv/TLS ABI, filesystem syscalls, Pi 5 hardware proof,
  networking, and SSH remain blocked.
- Phase 8 initial user stack closeout checkpoint is accepted. It reconciles
  the source inventory, contract, smoke plan, core, retained QEMU/substitute
  evidence, deferred surfaces, and planning state. The accepted frontier
  remains target-independent InitialUserStackPlan construction plus
  QEMU/substitute no-partial-stack/no-partial-launch evidence only. No live
  TTBR activation, lower-EL ERET, scheduler runnable publication, process
  lifecycle, broad argv/envp/auxv/TLS ABI, filesystem syscall, hardware proof,
  networking, or SSH capability is accepted. No explicit queued follow-up task
  remains; supervisor planning is required before the worker may promote
  another Phase 8.3 task.
- Phase 8 live address-space activation source inventory is accepted. It maps
  the accepted ProgramImagePlan, ProcessImageInstallPlan,
  ProcessAddressSpace, ProcessPageTableMaterialization,
  InitialProcessLaunchPlan, and InitialUserStackPlan lineage to the next
  missing live activation boundary. The inventory separates TTBR0_EL1/
  TTBR1_EL1 root provenance, TCR_EL1/MAIR_EL1/SCTLR_EL1 compatibility,
  ASID/TLB policy, barrier ordering, kernel reachability, exception/fault
  reporting, rollback/teardown, and activation-state updates from lower-EL
  ERET, scheduler runnable publication, process lifecycle, startup ABI,
  filesystem syscalls, Pi 5 hardware proof, networking, and SSH. It
  recommends phase8-live-address-space-activation-contract-20260530 as the
  next bounded documentation-only task.
- Phase 8 live address-space activation contract is accepted. It selects a
  LiveAddressSpaceActivationPlan preflight boundary with identity
  phase8-live-address-space-activation-plan-v1 and policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1. The contract
  states the invariant before and after any eventual TTBR/TCR/MAIR/SCTLR
  mutation, records TTBR0 root provenance and blocked TTBR1/kernel-half
  policy, requires TCR/MAIR/SCTLR compatibility checks, preserves ASID/TLB/
  barrier blocked states, names kernel reachability and fault-reporting
  prerequisites, defines deterministic errors/blockers, and requires
  no-partial-activation plus no-runnable-publication behavior. It recommends
  phase8-qemu-live-address-space-activation-smoke-plan-20260530 as the next
  bounded documentation-only task. Live register mutation, lower-EL ERET,
  scheduler publication, process lifecycle, filesystem syscalls, Pi 5 hardware
  proof, networking, and SSH remain blocked.
- Phase 8 QEMU/substitute live address-space activation smoke plan is
  accepted. It defines qemu_live_address_space_activation_smoke, retained
  evidence path
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log,
  exact qemu-live-address-space-activation-smoke-complete/PASS vocabulary,
  success observations for copied input identities, TTBR0 root provenance,
  blocked TTBR1/kernel-half policy, TCR/MAIR/SCTLR compatibility records,
  ASID/TLB/barrier blocked states, kernel reachability prerequisites,
  model-only activation binding, teardown, deterministic no-partial-activation
  rejections, and zero live side effects. It names
  phase8-live-address-space-activation-core-20260530 as the next bounded
  implementation task if dependencies remain satisfied. Live register
  mutation, lower-EL ERET, scheduler publication, process lifecycle,
  filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain
  blocked.
- Phase 8 live address-space activation core is accepted. It adds the
  target-independent LiveAddressSpaceActivationPlan preflight boundary with
  identity phase8-live-address-space-activation-plan-v1, policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1, copied accepted
  loader/install/address-space/materialization/launch/stack lineage, TTBR0
  root provenance from the materialized root lease without writing TTBR0_EL1,
  blocked TTBR1/kernel-half policy, TCR/MAIR compatibility records, blocked
  SCTLR/ASID/TLB/barrier/live-register states, required kernel reachability
  checklist, model-only activation-preflight-ready launch binding,
  idempotent plan-local teardown, deterministic no-partial-activation/
  no-runnable-publication rejections, and unit evidence that all live
  TTBR/TCR/MAIR/SCTLR/TLB/lower-EL/scheduler/process/descriptor side effects
  remain zero. QEMU smoke evidence, live register mutation, lower-EL ERET,
  scheduler publication, process lifecycle, filesystem syscalls, Pi 5
  hardware proof, networking, and SSH remain blocked.
- Phase 8 QEMU/substitute live address-space activation smoke core is
  accepted. It adds qemu_live_address_space_activation_smoke and retained
  evidence at
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log
  proving accepted activation identity and policy, copied input lineage,
  TTBR0 root provenance, blocked TTBR1/kernel-half policy, compatibility-only
  TCR/MAIR state, blocked SCTLR/ASID/TLB/barrier/live-register state, kernel
  reachability prerequisites, model-only activation binding, idempotent
  plan-local teardown, deterministic no-partial-activation rejections, zero
  live side effects, and qemu-live-address-space-activation-smoke-complete/
  PASS. Live register mutation, lower-EL ERET, scheduler publication, process
  lifecycle, filesystem syscalls, Pi 5 hardware proof, networking, and SSH
  remain blocked.
- Phase 8 live address-space activation closeout checkpoint is accepted. It
  reconciles the source inventory, contract, smoke plan, core, retained
  QEMU/substitute evidence, deferred surfaces, residual risks, and planning
  state. The accepted frontier remains target-independent
  LiveAddressSpaceActivationPlan construction plus QEMU/substitute
  no-partial-activation evidence below live register mutation, lower-EL ERET,
  scheduler publication, process lifecycle, filesystem syscalls, hardware
  proof, networking, and SSH.
- Phase 8 kernel-half reachability source inventory is accepted. It maps the
  source owners and accepted inputs for the remaining
  blocked-no-accepted-kernel-half-map frontier: linker kernel sections,
  active stack, heap/page-frame allocator, VBAR_EL1, exception vectors,
  UART/MMIO diagnostics, scheduler state, panic/fault reporting, early
  translation helpers, process materialization descriptor images, and AArch64
  TTBR/TCR/MAIR vocabulary. It separates candidate first-slice policies
  (TTBR1_EL1 shared kernel root, replicated kernel-half descriptors, or an
  explicitly blocked preflight record) from live register mutation,
  lower-EL ERET, scheduler publication, process lifecycle, startup ABI,
  filesystem syscalls, Pi 5 proof, networking, and SSH. It recommends
  phase8-kernel-half-reachability-contract-20260531 as the next bounded
  documentation-only task.
- Phase 8 kernel-half reachability contract is accepted. It selects the first
  preflight-only KernelHalfReachabilityPlan boundary with identity
  phase8-kernel-half-reachability-plan-v1 and policy
  preflight-ttbr1-shared-kernel-root-reachability-v1. The selected policy
  chooses TTBR0_EL1 for process-user mappings and reserves TTBR1_EL1 for a
  future shared privileged kernel root, while keeping descriptor-image
  construction and all live register mutation blocked. It requires
  reachability records for kernel text/rodata/data/bss, vectors, active kernel
  stack, heap/page-frame allocator state, UART/MMIO diagnostics, scheduler
  code/data, and panic/fault reporting; compatibility-only TCR/MAIR/TTBR/ASID/
  TLB/barrier vocabulary; deterministic errors/blockers; all-or-nothing
  construction; idempotent plan-local teardown; and zero live side effects.
  It recommends phase8-qemu-kernel-half-reachability-smoke-plan-20260531 as
  the next bounded documentation-only task.
- Phase 8 QEMU/substitute kernel-half reachability smoke plan is accepted. It
  selects qemu_kernel_half_reachability_smoke for retained evidence under
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/ and
  requires copied accepted input lineage, selected TTBR1 shared-kernel-root
  policy, required kernel reachability entries, compatibility-only TCR/MAIR
  records, blocked SCTLR/ASID/TLB/barrier/live-register states,
  deterministic no-partial rejection cases, idempotent teardown, zero live
  side effects, and final qemu-kernel-half-reachability-smoke-complete/PASS
  lines. Kernel-half descriptor-image construction, live register mutation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem
  syscalls, Pi 5 hardware proof, networking, and SSH remain blocked. The next
  bounded task is phase8-kernel-half-reachability-core-20260531.
- Phase 8 kernel-half reachability core is accepted. It implements the
  target-independent KernelHalfReachabilityPlan preflight boundary with
  identity phase8-kernel-half-reachability-plan-v1 and policy
  preflight-ttbr1-shared-kernel-root-reachability-v1. The record copies
  accepted input lineage through LiveAddressSpaceActivationPlan, preserves
  TTBR0 materialized-root provenance, selects a TTBR1 shared privileged
  kernel-root policy while blocking descriptor-image construction, records
  required kernel reachability entries and compatibility-only TCR/MAIR state,
  and unit-tests deterministic no-partial rejection and idempotent plan-local
  teardown. QEMU/substitute retained evidence, live register mutation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem
  syscalls, Pi 5 hardware proof, networking, and SSH remain blocked. The next
  bounded task is phase8-qemu-kernel-half-reachability-smoke-core-20260531.
- Phase 8 QEMU/substitute kernel-half reachability smoke core is accepted. It
  adds qemu_kernel_half_reachability_smoke and retains evidence at
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.
  The smoke proves copied input lineage through LiveAddressSpaceActivationPlan,
  the phase8-kernel-half-reachability-plan-v1 boundary, TTBR0 materialized-root
  provenance, TTBR1 shared privileged kernel-root policy with descriptor-image
  construction blocked, required kernel reachability entries, compatibility-only
  TCR/MAIR records, blocked SCTLR/ASID/TLB/barrier/live-register states,
  deterministic no-partial rejection, idempotent plan-local teardown, zero live
  side effects, and final qemu-kernel-half-reachability-smoke-complete/PASS
  lines. Kernel-half descriptor-image construction, live register mutation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem syscalls,
  Pi 5 hardware proof, networking, and SSH remain blocked. The next bounded
  task is phase8-kernel-half-reachability-closeout-checkpoint-20260531.
- Phase 8 kernel-half reachability closeout checkpoint is accepted. It
  reconciles the source inventory, contract, smoke plan, target-independent
  preflight core, retained QEMU/substitute smoke evidence, deferred surfaces,
  and residual risks. The accepted frontier remains a model-only
  KernelHalfReachabilityPlan below kernel-half descriptor-image construction,
  live translation-register mutation, lower-EL ERET, and runnable
  publication. No explicit queued follow-up task remains; supervisor planning
  is required before the worker may promote another Phase 8.3 task.
- Phase 8 kernel-half descriptor-image source inventory is accepted. It maps
  the accepted KernelHalfReachabilityPlan frontier to the remaining
  blocked-no-kernel-half-descriptor-image boundary and names the source owners
  for a non-installed TTBR1 shared privileged kernel-root descriptor image:
  accepted reachability records, process page-table materialization precedent,
  AArch64 descriptor vocabulary, linker-owned kernel ranges, memory/page-frame
  ownership, exception/vector reachability, UART/MMIO diagnostics, scheduler
  state, and live activation blockers. Live register mutation, lower-EL ERET,
  scheduler publication, process lifecycle, filesystem syscall expansion,
  Pi 5 hardware proof, networking, and SSH remain blocked. The next bounded
  task is phase8-kernel-half-descriptor-image-contract-20260531.
- Phase 8 kernel-half descriptor-image contract is accepted. It selects the
  non-installed KernelHalfDescriptorImage boundary
  phase8-kernel-half-descriptor-image-v1 for the TTBR1 shared privileged
  kernel-root policy
  ttbr1-shared-privileged-kernel-root-descriptor-image-v1. It defines accepted
  inputs, generated image/descriptor/coverage records, privileged-only normal
  and device descriptor attributes, deterministic rejection cases,
  all-or-nothing construction, idempotent teardown, and zero live activation
  side effects. Live register mutation, ASID/TLB/barrier activation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem syscall
  expansion, Pi 5 hardware proof, shell behavior, networking, and SSH remain
  blocked. The next bounded task is
  phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531.
- Phase 8 QEMU/substitute kernel-half descriptor-image smoke plan is accepted.
  It defines qemu_kernel_half_descriptor_image_smoke, retained evidence path
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log,
  classification qemu-kernel-half-descriptor-image-smoke-complete, PASS
  vocabulary, success observations for copied input lineage, kernel coverage,
  privileged-only normal/device descriptor attributes, model-owned root/table
  leases, no-partial-image rollback, idempotent teardown, deterministic
  rejection, and zero live activation side effects. Live register mutation,
  ASID/TLB/barrier activation, lower-EL ERET, scheduler publication, process
  lifecycle, filesystem syscall expansion, Pi 5 hardware proof, shell
  behavior, networking, and SSH remain blocked. The next bounded task is
  phase8-kernel-half-descriptor-image-core-20260531.
- Phase 8 kernel-half descriptor-image core is accepted. It adds
  src/kernel_half_descriptor_image.rs and the non-installed
  KernelHalfDescriptorImage boundary selected by the accepted contract and
  smoke plan. The model records copied KernelHalfReachabilityPlan lineage,
  TTBR0 materialized-root provenance, TTBR1 owned kernel-root image intent,
  required kernel coverage, privileged-only normal/device descriptor records,
  model-owned root/table leases, deterministic no-partial errors, idempotent
  teardown, and zero live activation side effects. Retained QEMU/substitute
  smoke evidence, live register mutation, ASID/TLB/barrier activation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem syscall
  expansion, Pi 5 hardware proof, shell behavior, networking, and SSH remain
  blocked. The next bounded task is
  phase8-qemu-kernel-half-descriptor-image-smoke-core-20260531.
- Phase 8 QEMU/substitute kernel-half descriptor-image smoke core is
  accepted. It wires qemu_kernel_half_descriptor_image_smoke and retains the
  planned log proving descriptor-image construction records, deterministic
  no-partial-image rejection, idempotent teardown, and zero live
  TTBR/TCR/MAIR/SCTLR/TLB/barrier/lower-EL/scheduler/process/descriptor-table
  side effects. Pi 5 hardware was not used and hardwareTestLock remained
  unlocked/restored. Live register mutation, ASID/TLB/barrier activation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem syscall
  expansion, Pi 5 proof, shell behavior, networking, and SSH remain blocked.
  The next bounded task is
  phase8-kernel-half-descriptor-image-closeout-checkpoint-20260531.
- Phase 8 kernel-half descriptor-image closeout checkpoint is accepted. It
  reconciles accepted commits 6cafdd8, a3bc161, ddaebb3, 3e0e836, and
  424c1f3 plus the retained QEMU/substitute smoke log for the
  non-installed KernelHalfDescriptorImage boundary. The accepted frontier
  includes copied Phase 8 lineage, TTBR0 materialized-root provenance,
  model-owned TTBR1 kernel-root image intent, required kernel coverage,
  privileged-only normal/device descriptor attributes, root/table ownership,
  deterministic no-partial-image rejection, idempotent teardown, and zero
  live side effects. Descriptor-image installation, live register mutation,
  ASID/TLB/barrier activation, lower-EL ERET, scheduler publication, process
  lifecycle, filesystem syscall expansion, Pi 5 proof, shell behavior,
  networking, and SSH remain blocked. No explicit queued follow-up task
  remains; supervisor planning is required before the worker may promote
  another Phase 8.3 task.
- Phase 8 live descriptor-image installation source inventory is accepted. It
  maps the accepted non-installed KernelHalfDescriptorImage closeout to the
  next installation-ready handoff below TTBR activation. The inventory
  distinguishes current published=true/installed=false evidence from any
  future installation claim, names descriptor-image, live activation,
  reachability, TTBR0 materialization, translation, linker, exception/vector,
  UART/MMIO, runtime console, and scheduler source owners, and recommends
  phase8-live-descriptor-image-installation-contract-20260531 as the next
  bounded documentation-only task. Live register mutation, ASID/TLB/barrier
  activation, lower-EL ERET, scheduler publication, process lifecycle,
  filesystem syscall expansion, Pi 5 proof, shell behavior, networking, and
  SSH remain blocked.
- Phase 8 live descriptor-image installation contract is accepted. It selects
  phase8-live-descriptor-image-installation-v1 with policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1 as the
  target-independent binding between the accepted non-installed
  KernelHalfDescriptorImage and accepted LiveAddressSpaceActivationPlan. The
  accepted frontier is model-level installation-ready activation binding only:
  copied lineage and TTBR0/TTBR1 provenance are recorded, kernel-half
  coverage/permissions/diagnostics are preserved, rollback/teardown is
  installation-record-local, and zero live side effects remain required.
  Live register mutation, active-root descriptor copy, ASID/TLB/barrier
  activation, lower-EL ERET, scheduler publication, process lifecycle,
  filesystem syscall expansion, Pi 5 proof, shell behavior, networking, and
  SSH remain blocked. The next bounded documentation-only task is
  phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531.
- Phase 8 QEMU/substitute live descriptor-image installation smoke plan is
  accepted. It defines qemu_live_descriptor_image_installation_smoke, fixture
  identity phase8-program-loader-elf64-aarch64-v1, descriptor-image boundary
  phase8-kernel-half-descriptor-image-v1, installation boundary
  phase8-live-descriptor-image-installation-v1, retained evidence path
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log,
  classification qemu-live-descriptor-image-installation-smoke-complete, PASS
  vocabulary, installation-ready binding success observations,
  deterministic no-partial-install/no-live-state-mutation rejection cases,
  teardown idempotence, zero live side effects, and conditional regression
  gates. The next bounded implementation task is
  phase8-live-descriptor-image-installation-core-20260531. Live register
  mutation, active-root descriptor copy, ASID/TLB/barrier activation, lower-EL
  ERET, scheduler publication, process lifecycle, filesystem syscall
  expansion, Pi 5 proof, shell behavior, networking, and SSH remain blocked.
- Phase 8 live descriptor-image installation core is accepted. It implements
  the target-independent KernelHalfDescriptorImageInstallation boundary with
  identity phase8-live-descriptor-image-installation-v1 and policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1. The model
  consumes the accepted non-installed KernelHalfDescriptorImage and accepted
  LiveAddressSpaceActivationPlan, records copied lineage and TTBR0/TTBR1
  provenance without register writes, preserves kernel-half coverage and
  privileged-only normal/device policy, supports installation-local teardown,
  deterministically rejects no-partial-install cases, and keeps all live side
  effects zero. The next bounded task is
  phase8-qemu-live-descriptor-image-installation-smoke-core-20260531.
- Phase 8 QEMU/substitute live descriptor-image installation smoke core is
  accepted. It retains
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log
  with copied descriptor-image/activation lineage, TTBR0/TTBR1 provenance,
  preserved coverage/permissions/diagnostics, installation-ready binding,
  deterministic no-partial-install/no-live-state-mutation rejections,
  idempotent teardown, zero live side effects, final
  qemu-live-descriptor-image-installation-smoke-complete, and PASS. Live
  register mutation, active-root descriptor copy, ASID/TLB/barrier activation,
  lower-EL ERET, scheduler publication, process lifecycle, filesystem syscall
  expansion, Pi 5 proof, shell behavior, networking, and SSH remain blocked.
- Phase 8 live descriptor-image installation closeout checkpoint is accepted.
  It reconciles the accepted source inventory, contract, smoke plan, model
  core, retained QEMU/substitute smoke evidence, deferred surfaces, and
  residual risks. The accepted frontier is model-only installation-ready
  activation binding between KernelHalfDescriptorImage and
  LiveAddressSpaceActivationPlan, below any live translation-register
  mutation. The next objective Phase 8.3 frontier is live
  translation-register activation planning, but no explicit queued follow-up
  task remains; supervisor planning is required before the worker may promote
  another Phase 8.3 task.
- Phase 8 live translation-register activation source inventory is accepted.
  It maps the accepted installation-ready binding to the next activation
  handoff, identifies source owners for TTBR0/TTBR1 provenance, TCR/MAIR
  compatibility, SCTLR/ASID/TLB/barrier vocabulary, kernel reachability,
  exception/vector and UART/MMIO diagnostics, rollback, runtime console, and
  scheduler side-effect boundaries, and recommends
  phase8-live-translation-register-activation-contract-20260531 as the next
  bounded documentation-only task. Live TTBR/TCR/MAIR/SCTLR mutation,
  active-root descriptor copy, ASID/TLB/barrier activation, lower-EL ERET,
  scheduler publication, process lifecycle, filesystem syscall expansion,
  Pi 5 proof, shell behavior, networking, and SSH remain blocked.
- Phase 8 live translation-register activation contract and QEMU/substitute
  smoke plan are accepted. The implementation core is also accepted: it adds
  a model-only LiveTranslationRegisterActivation record that consumes the
  accepted KernelHalfDescriptorImageInstallation, preserves copied Phase 8
  lineage and TTBR0/TTBR1 provenance, records compatibility and blocked live
  register state, exposes deterministic no-partial-activation errors,
  preserves kernel diagnostic reachability, and keeps zero live side effects.
  The QEMU/substitute route is wired for the queued smoke-core task. Live
  TTBR/TCR/MAIR/SCTLR mutation, active-root descriptor copy, ASID/TLB/barrier
  execution, lower-EL ERET, scheduler publication, process lifecycle,
  filesystem syscall expansion, Pi 5 proof, shell behavior, networking, and
  SSH remain blocked.

Acceptance criteria:

- A separate user program can be launched and waited on.

## Phase 9: Libc, Rust Std, and Portable Userland

Goal: make existing user programs portable to Talos instead of hand-writing a
complete command suite.

Milestone 9.1: Libc Strategy

- Define the Talos userspace ABI: startup, crt objects, errno, environment,
  arguments, TLS expectations, allocator hooks, and syscall wrappers.
- Evaluate a small libc path first: Talos-native libc shim, relibc, newlib, or
  musl when the syscall surface is mature enough.
- Treat glibc as a later compatibility target, not the first libc goal. It
  assumes a broad Linux-like environment and is too heavy for the first
  userspace porting layer.

Acceptance criteria:

- An ADR chooses the first libc strategy and records why glibc is deferred or
  rejected for the initial port.
- Simple C programs can call libc wrappers for write, read, open, close, exit,
  malloc/free, and basic path operations.
- Host-side and QEMU tests cover syscall-wrapper error behavior.

Milestone 9.2: Rust Userspace Target and Std Subset

- Define a Talos Rust userspace target distinct from the kernel target.
- Bring up enough Rust runtime support for no_std user programs first, then a
  constrained std subset when libc, allocation, filesystem, time, and descriptor
  behavior are ready.
- Keep proc-macros, build scripts, dynamic loading, and native compilation out
  of scope for this milestone.

Acceptance criteria:

- A cross-compiled Rust user program runs on Talos and uses arguments, stdio,
  heap allocation, and file reads.
- The supported and unsupported Rust std APIs are documented.
- Cargo target configuration for Talos userspace exists.

Milestone 9.3: Core Utilities Port

- Prefer Rust uutils/coreutils once the Rust userspace target is viable.
- Keep toybox, busybox, or GNU coreutils as fallback/reference ports if they
  expose missing POSIX semantics more clearly.
- Start with a small command set: cat, echo, true, false, ls, pwd, cp, mv, rm,
  mkdir, and sh-compatible process launching where practical.

Acceptance criteria:

- A cross-compiled utility suite can be packaged into initramfs/ramfs.
- Basic utilities run as separate user programs through the normal process,
  descriptor, and filesystem paths.
- Porting gaps become tracked syscall/libc/VFS tasks instead of local hacks.

## Phase 10: Local Shell and Developer UX

Goal: make Talos useful from a local console before depending on Ethernet.

Status: the feature-led reset accepted the first local serial interactivity
slice in QEMU/substitute and on serialized Raspberry Pi 5 hardware, then
accepted descriptor-backed local command-loop stdio and `echo hello` slices.
The kernel now has a bounded local command loop that reads a canonical TTY line
from runtime-console0, dispatches kernel-backed built-ins, prints visible
responses for help, empty input, unknown input, `stdio`, and `echo hello`,
and emits a prompt/ready marker for another command. The accepted stdio bridge
reports fd 0/fd 1/fd 2 identities, routes visible output through inherited
stdout in the QEMU/substitute path, records runtime-console0 backing, and
carries the same `stdio` response to serialized Pi 5 hardware. The accepted
stdin-descriptor slice moves the command-loop input side through
fd0/runtime-console0 descriptor plumbing while preserving descriptor-backed
visible output. The accepted echo slice proves a simple command-plus-argument
path over the same descriptor-backed stdin/stdout boundary in QEMU/substitute
and on Pi 5 hardware.

Retained QEMU stdin-descriptor evidence is at
tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log.
Retained Pi 5 stdin-descriptor hardware evidence is at
tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/serial-transcript.txt
and proves the physical `stdio` input, fd0 descriptor-backed-input marker,
visible fd identity response, descriptor-backed-output marker, next prompt, and
pi5-local-command-stdio-bridge-complete PASS path with a fresh 97936-byte
candidate TFTP fetch and restore proof. The stdin-descriptor closeout
checkpoint is accepted and records the current local interactivity frontier:
serial prompt, typed `stdio`, fd0/runtime-console-backed descriptor input,
descriptor-backed visible output, fd identity reporting, runtime-console0
backing, and next-prompt readiness. Retained QEMU echo evidence is at
tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log.
Retained Pi 5 echo hardware evidence is at
tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/serial-transcript.txt
with proof summary
tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/proof-result-local2.txt.
The echo proof records `typed_command=echo hello`, visible `hello` output,
descriptor-backed markers, prompt readiness, two fresh 98664-byte
`da591740/kernel_2712.img` TFTP fetches, and restore to the prior accepted
boot tree hash. The echo closeout checkpoint is accepted and records the
current local interactivity frontier: serial prompt, typed `echo hello`,
fd0/runtime-console-backed descriptor input, simple trailing-argument parsing,
kernel-backed built-in dispatch, descriptor-backed visible `hello` output, and
next-prompt readiness. This is not accepted userspace shell execution,
descriptor-backed filesystem commands, broad POSIX read readiness, networking,
or SSH.

Retained QEMU pwd evidence is at
tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
Retained Pi 5 pwd hardware evidence is at
tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript.txt
with proof summary
tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/proof-result-local2.txt.
The pwd proof records `typed_command=pwd`, visible `/` output from the
root-only current-directory placeholder, descriptor-backed markers, prompt
readiness, two fresh 98816-byte `da591740/kernel_2712.img` TFTP fetches, and
restore to the prior accepted boot tree hash. The pwd closeout checkpoint is
accepted and records the current local interactivity frontier: serial prompt,
typed `pwd`, fd0/runtime-console-backed descriptor input, kernel-backed
built-in dispatch, root-only current-directory placeholder, descriptor-backed
visible `/` output, and next-prompt readiness. This is not accepted `cd`,
VFS lookup, directory listing, userspace shell execution, filesystem-backed
command lookup, networking, or SSH.

Retained QEMU/substitute line-editing evidence is at
tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
The local command-loop core now records Backspace 0x08 and Delete 0x7f erase
telemetry and proves both bytes remove the previous editable byte before
dispatch. The accepted transcript types pwx, erases x, completes pwd, prints
visible / through descriptor-backed stdout, preserves help/status/stdio/echo/
empty/unknown/unexpected-argument behavior, and returns to a ready prompt. This
QEMU/substitute behavior is now carried to serialized Pi 5 hardware. Retained
Pi 5 line-editing evidence is at
tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/serial-transcript-through-pass.txt
with proof summary
tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/proof-result-local2.txt.
The Pi 5 proof records typed `pwx` + Backspace + `d` + Enter input, one
Backspace erase, corrected `pwd` dispatch, visible `/` output,
descriptor-backed input/output markers, ready prompt, two fresh 98944-byte
`da591740/kernel_2712.img` TFTP fetches, and restore to the prior accepted
boot tree hash. This is not accepted termios, cursor addressing, history, broad
escape parsing, userspace shell execution, process spawning, filesystem-backed
command lookup, networking, or SSH. The local line-editing closeout checkpoint
is now accepted and records the current local interactivity frontier: serial
prompt, typed line with Backspace/Delete correction before dispatch,
descriptor-backed input/output markers, corrected kernel-backed 'pwd'
response, and next-prompt readiness. It recommends a bounded Ctrl-C
line-cancel feature as the next smallest Phase 10 local interactivity slice;
old Phase 8 proof-only work remains paused unless it directly unblocks local
interactivity.

The bounded Ctrl-C line-cancel core is now accepted in QEMU/substitute.
Retained evidence is at
tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
The accepted transcript types partial `bogus` input, sends Ctrl-C 0x03,
prints `talos: line-canceled`, records an empty canceled line with control
telemetry, returns to a fresh prompt, dispatches following `pwd`, prints `/`
through descriptor-backed stdout, and ends with
qemu-local-line-cancel-complete plus PASS. This remains a kernel-local prompt
behavior only: no POSIX signal delivery, process interruption, job control,
termios, userspace shell execution, filesystem-backed command lookup, Pi 5
hardware proof, networking, or SSH is accepted by this core task. The next
feature-led step is the already queued serialized Pi 5 line-cancel proof if
its dependencies and hardwareTestLock state remain satisfied.

The serialized Pi 5 Ctrl-C line-cancel proof is now accepted. Retained physical
evidence is at
tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/serial-transcript-through-pass.txt
and summarized in
tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-result-local3.txt.
The accepted transcript shows partial input canceled with Ctrl-C, visible
talos: line-canceled, a fresh prompt, following pwd, visible slash output,
descriptor-backed input/output markers, next-prompt readiness, final
pi5-local-line-cancel-complete classification, and
rpi5-local-line-cancel-proof: PASS. The proof restored the prior accepted boot
tree hash after the run. This still does not accept POSIX signal delivery,
process interruption, job control, termios, userspace shell execution,
filesystem-backed command lookup, networking, or SSH. The next feature-led step
was the closeout checkpoint for the accepted line-cancel frontier.

The local line-cancel closeout checkpoint is now accepted and records the
current local interactivity frontier: serial prompt input through
fd0/runtime-console0, Ctrl-C prompt-local cancellation before dispatch,
visible 'talos: line-canceled', fresh prompt readiness, following 'pwd'
dispatch through descriptor-backed stdout, visible '/' output, and next-prompt
readiness. This remains a kernel-backed local prompt path only: POSIX signal
delivery, process interruption, terminal/session semantics, termios, job
control, history, userspace shell execution, process lifecycle,
filesystem-backed commands, 'cd', path traversal, networking, SSH, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain blocked. The
checkpoint recommends a bounded Ctrl-U prompt-local line-kill feature as the
next smallest Phase 10 local interactivity slice; old Phase 8 proof-only work
remains paused unless it directly unblocks local interactivity.

The bounded Ctrl-U prompt-local line-kill core is now accepted in
QEMU/substitute. Retained evidence is at
tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
The accepted transcript types partial `bogus` input, sends Ctrl-U 0x15,
records the dispatched line as `pwd`, prints `talos: line-killed`, prints `/`
through descriptor-backed stdout, records fd0/stdout markers and one clear-line
control event, returns to a ready prompt, and ends with
qemu-local-line-kill-complete plus PASS. Ctrl-C line-cancel and
Backspace/Delete local editing regressions also passed. This remains a
kernel-local prompt behavior only: no POSIX signal delivery, process
interruption, job control, termios, userspace shell execution,
filesystem-backed command lookup, Pi 5 hardware proof, networking, or SSH is
accepted by this core task. The next feature-led step is the already queued
serialized Pi 5 line-kill proof if dependencies and hardwareTestLock state
remain satisfied.

The serialized Pi 5 Ctrl-U line-kill proof is now accepted. Retained physical
evidence is at
tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/serial-transcript-through-pass.txt
and
tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-result-local6.txt.
The proof types partial bogus, sends Ctrl-U 0x15, records final line pwd,
prints visible talos: line-killed, prints slash output, records
descriptor-backed fd0/stdout markers, reports raw-bytes=10, controls=1,
responses=2, returns to a ready prompt, and ends with
pi5-local-line-kill-complete plus rpi5-local-line-kill-proof: PASS. The task
restored the prior accepted boot tree hash after the proof. This remains
prompt-local kernel behavior only: no POSIX signal/session/terminal semantics,
userspace shell execution, process lifecycle, filesystem-backed shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy
is accepted by this proof.

The local line-kill closeout checkpoint is now accepted and records the current
local interactivity frontier: serial prompt input through fd0/runtime-console0,
Ctrl-U prompt-local whole-line discard before dispatch, visible
'talos: line-killed', following 'pwd' dispatch through descriptor-backed
stdout, visible '/' output, and next-prompt readiness. This remains a
kernel-backed local prompt path only: POSIX signal delivery, process
interruption, terminal/session semantics, termios, job control, history,
userspace shell execution, process lifecycle, filesystem-backed commands, 'cd',
path traversal, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked. The checkpoint recommends a bounded
literal echo tail feature as the next smallest Phase 10 local interactivity
slice; old Phase 8 proof-only work remains paused unless it directly unblocks
local interactivity.

The bounded literal echo core is now accepted in QEMU/substitute evidence. The
local command loop accepts `echo local serial works` through
fd0/runtime-console0 canonical-lite input, dispatches the existing
kernel-backed `echo` built-in with a literal tail, prints visible
`local serial works` through descriptor-backed stdout, preserves
descriptor-backed fd0/stdout markers, returns to a ready prompt, and ends with
qemu-local-literal-echo-complete plus qemu-local-literal-echo: PASS. This
slice raises canonical-lite line capacity from 16 bytes to 32 bytes so the
feature command fits without truncation, while retaining explicit truncation
tests at the new boundary. Existing `echo hello`, `pwd`, `stdio`, empty
input, unknown-command, unexpected-argument, Backspace/Delete editing, Ctrl-C
line cancel, and Ctrl-U line kill behavior remains covered by rerun
QEMU/substitute regressions. This remains kernel-backed prompt-local behavior
only: broad shell parsing, quoting/escaping/globbing, argv/envp process ABI,
userspace shell execution, process lifecycle, filesystem-backed commands,
terminal/session semantics, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain deferred. The next mechanically
queued task is the serialized Pi 5 literal echo proof.

The serialized Pi 5 literal echo proof is now accepted. Retained physical
evidence in
`tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/serial-transcript-through-pass.txt`
and
`tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/proof-result-local3.txt`
shows descriptor-backed input/output markers, the input summary for
`echo local serial works`, visible `local serial works` output, ready prompt
return, pi5-local-literal-echo-complete, and
rpi5-local-literal-echo-proof: PASS. The proof recorded archive/kernel
identity, fresh serial/TFTP evidence, known-good control, unchanged-candidate
rerun for the initial visibility-incomplete transcript, and restore to the
prior accepted boot tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
hardwareTestLock release.

The local literal echo closeout checkpoint is now accepted and records the
current local interactivity frontier: serial prompt input through
fd0/runtime-console0, bounded literal echo tail dispatch for
`echo local serial works`, visible `local serial works` output through
descriptor-backed stdout, and next-prompt readiness. This remains a
kernel-backed prompt path only: broad shell parsing, quoting/escaping/globbing,
argv/envp process ABI, userspace shell execution, process lifecycle,
filesystem-backed commands, terminal/session semantics, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
deferred. No explicit queued task remains after this checkpoint; supervisor
planning is required for the next smallest feature-led local interactivity
task, and paused Phase 8 proof-only work remains paused unless it directly
unblocks that feature.

The local help command core is now accepted in QEMU/substitute evidence. The
serial `help` command prints accurate kernel-backed guidance for the accepted
local command frontier: `help`, `status`, `stdio`, `pwd`, `echo`, the accepted
`echo hello` and `echo local serial works` forms, and prompt-local
Backspace/Delete, Ctrl-C, and Ctrl-U controls. The retained
qemu-local-help-command transcript shows descriptor-backed fd0/stdout markers,
next-prompt readiness, final classification
`qemu-local-help-command-complete`, and exact `qemu-local-help-command: PASS`
vocabulary. Rerun QEMU/substitute regressions cover literal echo, pwd, line
editing, Ctrl-C line cancel, and Ctrl-U line kill. This remains a
kernel-backed prompt-local guide only; broad shell parsing, userspace process
execution, filesystem-backed command lookup, terminal/session behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache policy
remain deferred. The next queued task is the serialized Pi 5 help command
proof.

The serialized Pi 5 help command proof is now accepted. Retained physical
hardware evidence in
tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/serial-transcript-through-pass.txt
and
tasks/evidence/2026-05-31-pi5-local-help-command-proof/local6-observe-before-help-write/proof-result-local6.txt
shows input `help`, visible accurate help output for accepted commands and
editing controls, descriptor-backed dispatch markers, next-prompt readiness,
final classification `pi5-local-help-command-complete`, and exact
`rpi5-local-help-command-proof: PASS` vocabulary. Fresh TFTP evidence served
the 101088-byte `da591740/kernel_2712.img`, and post-proof restore evidence
returned the boot tree to hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. The
accepted frontier remains a kernel-backed prompt-local guide only; userspace
shell execution, filesystem-backed commands, broad parser behavior, terminal
sessions, networking, SSH, RP1/PCIe, UART interrupts, and DMA/cache policy
remain deferred.

The local help command closeout checkpoint is now accepted and records the
current descriptor-backed serial command-loop frontier: 'help' dispatches
through fd0/runtime-console0, prints accurate accepted command guidance through
descriptor-backed stdout, and returns to a ready 'talos>' prompt in both
QEMU/substitute and serialized Pi 5 evidence. The checkpoint keeps userspace
shell execution, filesystem-backed commands, broad parser behavior, terminal
sessions, networking, SSH, RP1/PCIe, UART interrupts, DMA/cache policy, and
paused Phase 8 proof-only work deferred. No explicit queued task remains after
this checkpoint; supervisor planning is required for the next smallest
feature-led local interactivity task.

The local `ls /` core is now accepted in QEMU/substitute evidence. The serial
command loop dispatches the bounded kernel-backed `ls /` command through
fd0/runtime-console0, validates the accepted read-only initramfs root entries,
prints `bin`, `dir`, `empty`, and `etc` through descriptor-backed stdout, and
returns to a ready `talos>` prompt. The retained
qemu-local-ls-root transcript shows descriptor-backed input/output markers,
the exact root entries, next-prompt readiness, final classification
`qemu-local-ls-root-complete`, and exact `qemu-local-ls-root: PASS`
vocabulary. This remains prompt-local and root-only; broad parser behavior,
recursive listing, general path traversal, writable filesystem state,
descriptor-backed filesystem syscalls, userspace shell execution, process
lifecycle, terminal sessions, networking, SSH, RP1/PCIe, UART interrupts, and
DMA/cache policy remain deferred. The next queued task is the serialized Pi 5
`ls /` proof.

The serialized Pi 5 `ls /` proof is now accepted after the local5
capture-window rerun. The unchanged candidate archive
`target/talos-rpi5-local-ls-root-local1.tar.gz` retained descriptor-backed
fd0/stdout markers, input summary `input='ls /'`, visible root entries
`bin`, `dir`, `empty`, and `etc`, complete
`ready-for-next prompt=true`, final classification
`pi5-local-ls-root-complete`, and exact
`rpi5-local-ls-root-proof: PASS` vocabulary. Fresh TFTP evidence recorded
served candidate boot requests, and post-proof restore evidence returned the
boot tree to hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. The
accepted frontier remains bounded to a kernel-backed prompt-local root listing;
broad parser behavior, recursive listing, path traversal, writable filesystem
state, descriptor-backed filesystem syscalls, userspace shell execution,
process lifecycle, terminal sessions, networking, SSH, RP1/PCIe, UART
interrupts, and DMA/cache policy remain deferred. The next queued task is the
local `ls /` closeout checkpoint.

The local `ls /` closeout checkpoint is now accepted and records the current
descriptor-backed serial command-loop frontier: `ls /` dispatches through
fd0/runtime-console0, reads the accepted read-only initramfs root fixture,
prints visible `bin`, `dir`, `empty`, and `etc` entries through
descriptor-backed stdout in both QEMU/substitute and serialized Pi 5 evidence,
and returns to a ready `talos>` prompt. The checkpoint keeps broad parser
behavior, recursive listing, path traversal, writable filesystem state,
descriptor-backed filesystem syscalls, userspace shell execution, process
lifecycle, terminal sessions, networking, SSH, RP1/PCIe, UART interrupts,
DMA/cache policy, and paused Phase 8 proof-only work deferred. No explicit
queued task remains after this checkpoint; supervisor planning is required for
the next smallest feature-led local interactivity task.

The local `ls /bin` core is now accepted in QEMU/substitute evidence. The
serial command loop accepts only the exact bounded `ls /bin` path extension,
uses the accepted read-only initramfs fixture to verify `/bin` and
`/bin/init`, prints visible `init` output through descriptor-backed stdout,
and returns to a ready `talos>` prompt. The retained
qemu-local-ls-bin transcript also reruns `ls /` in the same scenario and
shows the accepted `bin`, `dir`, `empty`, and `etc` root output still
passes. This remains prompt-local and does not accept recursive/general path
listing, relative paths, `cd`, file reads, descriptor-backed filesystem
syscalls, userspace execution, process lifecycle, terminal/session behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache policy. The
next queued task is the serialized Pi 5 `ls /bin` proof.

The local `cat /etc/banner.txt` core is now accepted in QEMU/substitute
evidence while the blocked Pi 5 `ls /bin` proof remains untouched. The serial
command loop accepts only the exact bounded `cat /etc/banner.txt` file read,
uses the accepted read-only initramfs fixture's regular-file bytes, prints
visible `Talos initramfs fixture` output through descriptor-backed stdout, and
returns to a ready `talos>` prompt. Help and status now expose the bounded
`cat` frontier. General `cat`, arbitrary file reads, descriptor-backed
filesystem syscalls, userspace execution, process lifecycle, shell parsing,
terminal/session behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
and DMA/cache policy remain deferred. The next queued task is the serialized
Pi 5 `cat /etc/banner.txt` proof.

The serialized Pi 5 `cat /etc/banner.txt` feature proof is now accepted by
feature-led supervisor review after an unchanged rerun following a settled
accepted prompt-control discriminator. Retained physical evidence in
`tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/serial-transcript.txt`
and
`tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/proof-result.txt`
shows a fresh prompt, delayed write of `cat /etc/banner.txt`, visible
`Talos initramfs fixture` output, `cat-banner-observed`, ready prompt
return, final classification `pi5-local-cat-banner-complete`, and exact
`rpi5-local-cat-banner-proof: PASS` vocabulary. Fresh TFTP evidence served
`da591740/config.txt` and the 107520-byte `da591740/kernel_2712.img`, and
post-proof restore returned the boot tree to hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The local cat-banner closeout checkpoint is now accepted and records the
current descriptor-backed serial command-loop frontier: the exact bounded
`cat /etc/banner.txt` command prints the immutable initramfs fixture banner
and returns to a ready `talos>` prompt in both QEMU/substitute and serialized
Pi 5 feature evidence. The checkpoint reconciles descriptor-marker policy: the
unchanged Pi 5 scenario was accepted on feature PASS without descriptor marker
emission, while descriptor-backed command-loop behavior remains covered by
QEMU/substitute cat-banner evidence and earlier accepted command-loop proof
lineage. Future marker work is optional and must be feature-justified. Broad
`cat`, arbitrary file reads, path traversal, descriptor-backed filesystem
syscalls, userspace execution, process lifecycle, terminal/session behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache policy, blocked
`ls /bin` proof strategy, and paused Phase 8 proof-only work remain deferred.

The local bounded cd fixed-directories core is now accepted in QEMU/substitute
evidence. The descriptor-backed serial command loop tracks command-context
current directory state for the exact directories /, /etc, and /bin; pwd
reflects that state after cd /etc, cd /bin, and cd /; and cd /missing is
rejected with talos: not-directory while leaving the current directory
unchanged. Retained feature evidence is at
tasks/evidence/2026-06-02-qemu-local-cd-fixed-dirs-core/qemu-local-cd-fixed-dirs-smoke.log
with final classification qemu-local-cd-fixed-dirs-complete and exact
qemu-local-cd-fixed-dirs: PASS vocabulary. This is a kernel-backed command-loop
placeholder for future process-local cwd only; it does not accept POSIX chdir,
relative paths, broad path traversal, descriptor-backed filesystem syscalls,
userspace shell execution, process lifecycle, terminal sessions, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache policy. Serialized Pi 5
follow-up evidence has not accepted the cd feature yet. The cd
entry-provenance candidate archive was fetched from TFTP with two
110008-byte `kernel_2712.img` serves, but fresh post-TFTP serial retained no
`TALOS: asm_start`, `TALOS: asm_pre_rust_entry`, Rust entry marker, DTB scan
marker, prompt, cd transcript, classification, or PASS. A prompt-capable
literal-echo control rebuilt with the same earliest entry markers was also
fetched from TFTP with two 108896-byte `kernel_2712.img` serves, but fresh
post-TFTP serial again retained only NUL/space and no entry marker, prompt,
classification, or PASS. The Pi 5 blocker is therefore no longer cd-specific;
the cd proof remains blocked behind
`firmware-fetch-no-entry-marker-control`, supervisor planning is required
before another Pi 5 rerun or code change, and the cd closeout checkpoint must
not be promoted.

The early entry marker quarantine core is now accepted. It removes the
unproven raw pre-Rust assembly entry-provenance marker route from the
prompt-capable rpi5_local_literal_echo control and rpi5_local_cd_fixed_dirs
candidate archives while leaving the marker support available for existing RPi5
SMP diagnostic scenarios. Retained QEMU/substitute literal-echo, cd fixed-dirs,
and command-loop regression logs pass, and rebuilt non-published Pi 5 archives
pass image/archive review. Static string inspection shows both rebuilt images
retain their proof strings and TALOS: command loop proof entered while omitting
TALOS: asm_start and TALOS: asm_pre_rust_entry. The next hardware discriminator
is a serialized non-invasive accepted-control regression proof; only retained
literal-echo prompt/PASS evidence from that proof may unblock cd recovery work.

The serialized non-invasive accepted-control regression proof is now accepted.
Retained Pi 5 evidence in
tasks/evidence/2026-06-02-pi5-accepted-control-noninvasive-regression-proof/local1-noninvasive-literal-echo-control/
shows the rebuilt literal-echo control archive fetched over TFTP with two
108896-byte da591740/kernel_2712.img serves, visible echo local serial works
response output, next talos> prompt readiness,
pi5-local-literal-echo-complete, and rpi5-local-literal-echo-proof: PASS,
followed by restore to the pre-run boot tree hash. The cd fixed-directories
hardware proof remains unaccepted, but the accepted control evidence
mechanically unblocks the planned non-invasive cd recovery core.

The non-invasive cd fixed-directories recovery core is now accepted. It made no
additional runtime behavior change; instead it rebuilt and retained a fresh
quarantined cd candidate archive at
target/talos-rpi5-local-cd-fixed-dirs-noninvasive-recovery-core.tar.gz.
Retained QEMU/substitute cd fixed-dirs, literal-echo, and command-loop
regression logs pass. Archive review passes with a 110008-byte kernel_2712.img,
and static string/route evidence retains rpi5-local-cd-fixed-dirs-proof,
pi5-local-cd-fixed-dirs-complete, and TALOS: command loop proof entered while
omitting TALOS: asm_start and TALOS: asm_pre_rust_entry. The next bounded task
is the serialized Pi 5 non-invasive cd recovery proof; the original cd Pi 5
proof remains unaccepted until hardware evidence retains the full cd transcript,
ready prompt, final classification, and rpi5-local-cd-fixed-dirs-proof: PASS.

The serialized Pi 5 non-invasive cd recovery proof is now
accepted-blocked-firmware-fetch-no-kernel-entry. The recovered cd candidate
archive was fetched from TFTP with two 110008-byte
da591740/kernel_2712.img serves from settled same-cursor evidence, but fresh
serial from the pre-run cursor retained only Raspberry Pi firmware/RP1 reboot
output and no TALOS: rust_entry, command-loop marker, prompt, cd transcript,
classification, or PASS. The lab boot tree was restored to the pre-run hash
before hardwareTestLock release. The cd fixed-directories hardware proof
remains unaccepted.

The RPi5 cd fixed-directories entry-delta fix core is now accepted. It changed
only the Pi 5 cd proof harness command plan from the previous help/status/stdio
prelude plus pwd/cd transcript to the original nine-command pwd/cd feature
transcript. QEMU/substitute cd fixed-dirs, literal-echo, and command-loop
regressions still pass. The fresh candidate archive
target/talos-rpi5-local-cd-fixed-dirs-entry-delta-fix-core.tar.gz passed
archive review with a 109800-byte kernel_2712.img, 208 bytes smaller than the
prior blocked cd candidate, and static string/map evidence retains
rpi5-local-cd-fixed-dirs-proof, pi5-local-cd-fixed-dirs-complete, and
TALOS: command loop proof entered while omitting quarantined raw assembly entry
markers. The next bounded task is the serialized Pi 5 entry-delta cd proof; the
cd feature remains unaccepted until hardware evidence retains the full pwd/cd
transcript, ready prompt, final classification, and
rpi5-local-cd-fixed-dirs-proof: PASS.

The serialized Pi 5 entry-delta cd proof is now accepted. Retained Pi 5
evidence in
tasks/evidence/2026-06-02-pi5-local-cd-fixed-dirs-entry-delta-proof/local1-entry-delta-candidate/
shows the entry-delta cd archive fetched over TFTP with two 109800-byte
da591740/kernel_2712.img serves, command-loop proof entry, the original
nine-command pwd/cd feature sequence, expected cwd outputs for /, /etc, /bin,
and / after returning to root, rejected cd /missing with cwd unchanged,
ready-for-next prompt=true, pi5-local-cd-fixed-dirs-complete, and
rpi5-local-cd-fixed-dirs-proof: PASS. The lab boot tree was restored to the
pre-run hash before hardwareTestLock release. This accepts the bounded
kernel-backed cd fixed-directories Phase 10 feature on Pi 5 hardware; broad
POSIX chdir, relative paths, path traversal, descriptor-backed filesystem
syscalls, userspace shell execution, process lifecycle, terminal/session
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA, and cache
driver policy remain deferred.

The local cd fixed-directories closeout checkpoint is now accepted and records
the current command-context cwd frontier: the exact bounded `cd /`,
`cd /etc`, and `cd /bin` commands update prompt-local cwd state, `pwd`
reflects that state, rejected nonexistent directory input leaves cwd
unchanged, and the serial command loop returns to a ready `talos>` prompt in
both QEMU/substitute and serialized Pi 5 feature evidence. This is deliberately
future process-local cwd shaping, not an accepted POSIX `chdir` syscall,
relative-path resolver, broad path traversal mechanism, descriptor-backed
filesystem syscall, userspace shell, or process cwd inheritance model.
Networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache policy, blocked
`ls /bin` proof strategy, and paused Phase 8 proof-only work remain deferred.

The local cwd-aware bare `ls` core is now accepted in QEMU/substitute
evidence. The descriptor-backed serial command loop resolves bare `ls` against
the accepted command-context cwd state: root cwd lists `bin`, `dir`, `empty`,
and `etc`; `cd /etc` followed by bare `ls` lists `banner.txt`; `cd /bin`
followed by bare `ls` lists `init`; and `cd /` followed by bare `ls` returns
to the root listing. Existing exact `ls /` and `ls /bin` forms remain
preserved, while `ls /etc` is not accepted as a new explicit path form. The
feature remains kernel-backed, prompt-local, and bounded; relative paths, `.`,
`..`, arbitrary path listing, descriptor-backed filesystem syscalls, userspace
shell execution, process cwd inheritance, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache policy remain deferred. The next explicit
queued task is the serialized Pi 5 proof for this bare `ls` cwd feature.

The RPi5 candidate archive path for bare `ls` cwd is now accepted. The fresh
archive `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz` was
built from the `rpi5_local_ls_cwd` scenario with retained archive review,
archive/kernel hashes, kernel size, boot-tree file listing, QEMU/substitute
regressions, and static proof-string inspection under
tasks/evidence/2026-06-02-rpi5-local-ls-cwd-candidate-archive-core/. The image
retains `rpi5-local-ls-cwd-proof`, `pi5-local-ls-cwd-complete`, and
`ls-cwd-observed`, and omits quarantined raw assembly entry markers. No boot
archive was published and no hardware was touched; the queued serialized Pi 5
proof is now unblocked on archive identity, subject to hardwareTestLock and
settled TFTP/serial evidence requirements.

The serialized Pi 5 proof for bare `ls` cwd is now accepted. Published
hardware evidence under
tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/ used the
accepted archive
`target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz` with kernel
sha256 `da6bb65ad8529912e1feca037d6f1e3cfbc46c5ea052ee32a1ab669b000bfd3e`
and 110624-byte `kernel_2712.img`. The run held hardwareTestLock, captured
fresh serial/TFTP cursors, collected settled same-cursor TFTP evidence before
restore, served `da591740/kernel_2712.img` twice at the expected size, and
restored the pre-run boot tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` before
lock release. The retained serial transcript shows `pwd`, bare `ls` at
`/`, `cd /etc` then bare `ls` with `banner.txt`, `cd /bin` then bare
`ls` with `init`, `cd /` then bare `ls` with root entries, the `bogus`
regression, ready-next prompt, `pi5-local-ls-cwd-complete`, and exact
`rpi5-local-ls-cwd-proof: PASS`. This advances the accepted frontier to
Pi 5-proven bounded command-context cwd listings while keeping broad path
traversal, POSIX cwd/syscalls, descriptor-backed filesystem syscalls,
process-local cwd inheritance, userspace shell execution, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache policy deferred.

The local bare `ls` cwd closeout checkpoint is now accepted. It reconciles
the accepted QEMU/substitute core commit
`4a3db877499328b10e75bff9f1eb3bc36f7579ae`, RPi5 candidate archive commit
`742f0eaba91bd4986d8fa456722de89f94aa7015`, and Pi 5 proof commit
`b17832a7232d74c3d4f90dde677c4beb86271945` into the current accepted
frontier: bounded command-context cwd listings for `/`, `/etc`, and
`/bin` over the descriptor-backed serial command loop. No implementation,
QEMU rerun, hardware action, or archive publication was performed by the
checkpoint. No explicit mechanically unblocked feature task remains after this
closeout, so supervisor planning is required for the next smallest
feature-led local interactivity slice.

The local cwd-aware cat banner.txt core is now accepted in QEMU/substitute
evidence with a static RPi5 candidate archive ready for the serialized proof.
The descriptor-backed serial command loop now accepts cat banner.txt only
when command-context cwd is /etc, prints the immutable Talos initramfs fixture
banner, returns to a ready prompt, and preserves a cwd-sensitive negative case
from / with talos: not-found. The exact absolute path cat /etc/banner.txt,
fixed-directory cd, bare ls cwd, line editing, line cancel, line kill, and
command-loop regressions were rerun locally. The fresh archive
target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz was built from
the rpi5_local_cat_cwd scenario with retained archive review, archive/kernel
hashes, kernel size, boot-tree file listing, and static proof-string
inspection under tasks/evidence/2026-06-02-qemu-local-cat-cwd-core/. The
image retains rpi5-local-cat-cwd-proof, pi5-local-cat-cwd-complete,
cat-cwd-observed, and cat-cwd-negative-observed, and omits quarantined raw
assembly entry markers. No boot archive was published and no hardware was
touched; the next explicit queued task is the serialized Pi 5 proof for this
bounded cwd-aware cat banner.txt feature.

Milestone 10.1: Local Shell

- Implement or port a small shell that runs as a user program.
- Use the normal process, descriptor, TTY, filesystem, and program-loader
  mechanisms.
- Support built-ins only where they reflect normal shell behavior, not kernel
  shortcuts.

Acceptance criteria:

- A user can interact through the serial TTY, run commands, inspect files, and
  launch separate user programs.
- Shell I/O uses stdin/stdout/stderr descriptors.
- Shell limitations and POSIX gaps are documented.

Milestone 10.2: Pipelines and Process Control

- Add pipes, redirection, exit status, wait, and basic job/process accounting.
- Keep signals minimal at first but avoid designs that make POSIX signals
  impossible later.

Acceptance criteria:

- The shell can run simple pipelines and report exit statuses.
- Multiple user programs can make progress while the shell remains responsive.
- Descriptor inheritance and close-on-exec behavior are tested.

Accepted closeout boundary:

- 'phase10-pipelines-process-control-milestone-closeout-20260605' accepts these
  criteria only at the local/QEMU frontier: simple exact two-stage pipelines,
  exit-status/waitpid/laststatus controls, descriptor inheritance/restoration
  including loader temporary descriptor non-leak, bounded fixed-/bin background
  VFS exec accounting, and deterministic jobs stale-entry clearing.
- True scheduler-concurrent userspace execution, full POSIX job control,
  multi-stage/concurrent pipelines, pipefail, background pipelines/redirections,
  persistent storage, Pi 5 proof, networking, SSH, and phase transition remain
  deferred.

Milestone 10.3: Persistent or Larger Local Storage

- Evaluate SD, USB mass storage, generated image roots, and TFTP-loaded
  initramfs expansion for a practical development filesystem.
- Add a persistent filesystem path only after VFS and block/storage ownership
  rules are clear.

Selected first slice:

- 'phase10-local-storage-path-evaluation-checkpoint-20260605' selects generated
  userland/initramfs manifest ingestion into the existing read-only VFS model as
  the smallest useful next capability. The first implementation should prove a
  manifest/root-defined file, and optionally a manifest/root-defined
  executable, reaches the accepted descriptor-backed VFS/open/read and loader
  path without adding a new hardcoded file-content constant to
  'src/initramfs.rs'.
- This slice is a source-code edit avoidance step only. Avoiding kernel binary
  rebuilds, publishing boot archives, Pi 5 TFTP transport, writable persistent
  storage, SD/USB/block drivers, networking, SSH, and phase transition remain
  deferred.
- 'phase10-generated-userland-image-contract-20260605' accepts the
  documentation-only contract for that generated-root slice. The next
  implementation must define generated-root identity/digest evidence,
  deterministic path ordering and normalization, directory/regular-file-only
  limits, fixed size limits, descriptor-backed VFS/open/read proof for a
  generated file, and retained controls for accepted VFS exec, loader,
  status/wait, descriptor, pipeline/redirection, and jobs behavior. A generated
  executable is allowed only if it stays within the accepted VFS/loader/
  userspace lifecycle path. No runtime behavior, no-rebuild transport, Pi 5
  proof, boot archive publication, writable persistence, or phase transition is
  accepted by the contract alone.
- 'phase10-generated-userland-image-manifest-core-20260605' accepts the first
  implementation slice: a host-side manifest generates
  '/generated/manifest.txt' into the existing read-only initramfs/VFS fixture,
  cat /generated/manifest.txt reads it through the descriptor-backed shell
  path, and QEMU/substitute evidence records generated-root identity/source/
  digest plus retained cat, VFS exec/open/read, loader, status/wait,
  pipeline/redirection, and jobs/accounting controls. Generated executable
  proof, kernel binary no-rebuild transport, boot archives, Pi 5 hardware proof,
  writable persistence, block drivers, networking, SSH, and phase transition
  remain deferred.
- 'phase10-generated-userland-image-manifest-closeout-20260605' accepts that
  boundary as source-code edit avoidance for one generated read-only userland
  file only. It explicitly does not accept kernel binary no-rebuild transport,
  boot archive update, Pi 5 behavior, writable persistence, generated executable
  support, networking, SSH, or phase transition. A later supervisor-planned
  transport/no-rebuild slice is required before claiming the milestone's
  stronger "without rebuilding the kernel for every user program change"
  criterion.
- 'phase10-generated-userland-executable-core-20260605' accepts the generated
  executable proof: the manifest/root input now defines '/generated/status7',
  build.rs synthesizes its ELF bytes into OUT_DIR generated constants, the
  read-only initramfs/VFS fixture exposes it as a regular generated node, and
  QEMU/substitute evidence shows exec /generated/status7 alpha reaching the
  accepted VFS/open/read, loader, startup argv/envp, lifecycle status,
  waitpid, and laststatus path with deterministic status 0x7. This remains a
  build-time generated-root source-code edit avoidance slice only; no
  no-kernel-rebuild transport, boot archive update, Pi 5 behavior, writable
  persistence, general PATH discovery, networking, SSH, or phase transition is
  accepted.
- 'phase10-generated-userland-executable-closeout-20260605' accepts the
  generated executable frontier closeout. The accepted Milestone 10.3 boundary
  now covers one generated regular file and one generated executable defined by
  the manifest/root input, both consumed through the existing read-only
  initramfs/VFS model. This still does not satisfy the milestone's stronger
  no-kernel-rebuild criterion because the generated root is consumed at build
  time. The next mechanically unblocked task is the local/QEMU
  no-kernel-rebuild generated-root transport contract; implementation,
  boot archive publication, Pi 5 proof, writable persistence, storage drivers,
  networking, SSH, and phase transition remain deferred.
- 'phase10-generated-root-no-rebuild-transport-contract-20260605' accepts the
  documentation-only local/QEMU no-kernel-rebuild transport contract. The
  selected first proof shape is QEMU `-device loader,file=<artifact>,addr=0x47000000`
  on the existing `-M virt -m 256M` local runner, with a 4 MiB artifact
  window, a `__kernel_end` collision guard, a self-describing deterministic
  generated-root v1 artifact, same-kernel image hash evidence for two artifact
  runs, distinct artifact digests, and compiled generated-root fallback for
  missing or malformed artifacts. This contract does not implement the
  transport and does not accept Pi 5 boot archive/TFTP behavior, writable
  persistence, SD/USB/block drivers, networking, SSH, or phase transition.
- 'phase10-generated-root-no-rebuild-transport-core-20260605' accepts the
  local/QEMU no-kernel-rebuild transport core. The accepted proof runs the same
  kernel ELF/image hashes against two different generated-root artifacts loaded
  with QEMU's loader device at `0x47000000`, after proving
  `__kernel_end=0x00000000403bb000 <= 0x47000000`. Artifact A and B have
  distinct SHA-256 digests and produce distinct generated file contents plus
  generated executable statuses through the accepted VFS, loader, waitpid,
  laststatus, pipeline, and jobs controls. Missing and malformed artifacts
  deterministically fall back to the compiled generated-root source. This is
  local/QEMU-substitute evidence only; Pi 5 boot archive/TFTP behavior,
  writable persistence, storage drivers, networking, SSH, and phase transition
  remain deferred.
- 'phase10-generated-root-no-rebuild-transport-closeout-20260605' accepts the
  local/QEMU no-kernel-rebuild generated-root transport frontier. The accepted
  Milestone 10.3 boundary now includes a same-kernel/two-artifact proof where
  external generated-root artifact A and B change /generated/manifest.txt
  contents and /generated/status7 status without rebuilding the kernel binary
  between runs. Missing and malformed artifacts remain deterministic
  compiled-fallback cases. This still does not accept Pi 5 firmware/TFTP
  placement, boot archive publication, writable persistence, SD/USB/block
  storage, networking, SSH, or phase transition.
- 'phase10-pi5-generated-root-boot-archive-candidate-core-20260605' accepts the
  non-published Pi 5 generated-root boot-transport candidate archive. The
  candidate archive carries 'initramfs_2712' at the root and 'da591740/' mirror,
  records kernel and artifact digests, and includes the firmware-initramfs FDT
  source path needed for a later hardware proof. This still does not accept
  archive publication, Pi 5 TFTP/serial consumption proof, writable persistence,
  SD/USB/block storage, networking, SSH, or phase transition.
- 'phase10-pi5-generated-root-boot-transport-proof-20260605' completed with a
  source-backed blocker, not acceptance. Hardware evidence records candidate
  publication, fresh serial/TFTP cursors, da591740/initramfs_2712 fetch at
  662 bytes, command-loop readiness, and restore to the prior boot tree hash.
  The firmware initramfs bounds existed but overlapped Talos' early
  page-frame/bootstrap/translation-table range at 0x2efff000, so the runtime
  fell back to the compiled generated-root image with reason=missing-artifact.
  A later implementation must reserve or copy that firmware range before
  proving Pi 5 generated-root consumption.
- 'phase10-pi5-generated-root-boot-transport-closeout-20260605' accepts a
  static closeout of that blocked proof boundary. It records contract,
  candidate, proof/blocker, restore, and retained local/QEMU control evidence,
  but does not accept Pi 5 generated-root artifact consumption or close
  Milestone 10.3. The next Milestone 10.3 task should be explicitly planned
  around the firmware initramfs range overlap.
- 'phase10-local-storage-milestone-closeout-20260605' accepts the Milestone
  10.3 checkpoint at the local/QEMU generated-root transport frontier and
  defers the Pi 5 hardware boundary on the retained source-backed blocker. The
  accepted frontier covers generated-root file and executable content through
  the read-only VFS/loader/process path plus same-kernel/two-artifact
  local/QEMU no-rebuild transport. Pi 5 generated-root consumption remains
  deferred until Talos reserves or copies the firmware initramfs range before
  early memory setup and passes a fresh serialized hardware proof. Writable
  persistence, SD/USB/block storage, broader filesystem mutation, networking,
  SSH, and Phase 11 transition remain unaccepted.

Acceptance criteria:

- Talos can load a nontrivial userland image without rebuilding the kernel for
  every user program change. Current status: accepted for the local/QEMU
  generated-root transport boundary only. The accepted evidence runs the same
  kernel ELF/image hashes against two different external generated-root
  artifacts and observes distinct generated file and executable behavior.
  Pi 5 candidate boot archive placement is accepted only as a non-published
  static archive. The first Pi 5 boot-transport proof and its closeout retain a
  source-backed blocker: firmware initramfs range overlap with early memory
  setup. Pi 5 consumption acceptance, writable persistence, SD/USB/block
  storage, networking, SSH, and phase transition remain deferred and require
  explicit follow-up tasks.
- Documentation explains the chosen local storage path and remaining risks.
  Current status: accepted by the Milestone 10.3 closeout as a local/QEMU
  generated-root transport checkpoint; Pi 5 generated-root consumption and true
  writable or block-backed storage remain deferred.

## Phase 11: RP1, PCIe, DMA, and Hardware Substrate

Goal: understand the Pi 5 I/O substrate before relying on RP1 devices for
networking, GPIO, storage, or broader hardware support.

Entry status: Phase 10 is closed by
`phase10-to-phase11-transition-checkpoint-20260605`. The accepted Phase 10
frontier covers local shell behavior and local/QEMU generated-root transport.
Pi 5 generated-root consumption remains deferred on the firmware initramfs
range-overlap blocker, but that storage transport blocker does not prevent
Phase 11 RP1/PCIe mapping from starting. The first Phase 11 slice must stay on
RP1/PCIe mapping and a narrow register-read diagnostic before GPIO,
interrupts, DMA, networking, or SSH work.

Milestone 11.1: RP1 and PCIe Mapping

- Determine whether firmware leaves RP1 configured and usable for early
  bare-metal access.
- Map the BCM2712 PCIe2 window, RP1 BAR/peripheral ranges, and address
  translations from device tree.
- Decide how much PCIe enumeration Talos needs for built-in RP1 versus external
  PCIe devices.

Acceptance criteria:

- A hardware note documents CPU physical addresses for initial RP1 access.
- A diagnostic can read a stable RP1 register or otherwise prove RP1 mapping
  assumptions.
- Known limitations around firmware-initialized state are recorded.

Contract status: `phase11-rp1-pcie-map-source-contract-20260605` records
the first source-backed mapping contract. Linux `rpi-6.12.y` device-tree
sources map `pcie2` non-prefetchable PCIe address `00_00000000` to
CPU physical `0x1f_0000_0000`, and map RP1 peripheral bus
`0xc0_4000_0000` to that window. The first diagnostic target is the
read-only RP1 UART0 PL011 flag register at CPU physical
`0x1f_0003_0018`, width 32 bits, classified as `mapped/read-value`,
`bus-fault/trap`, `firmware-state-dependency`, or
`staging/build-blocker`. This contract depends on firmware-preserved RP1
UART0 state and the lab `enable_rp1_uart=1` boot config; it does not claim
GPIO ownership, interrupts, DMA/cache policy, Ethernet, networking, SSH, or
broader PCIe enumeration.

Proof status: `phase11-rp1-register-read-pi5-proof-20260605` completed with
a hardware blocker, not mapping acceptance. The lab published candidate tree
`a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9` and TFTP
served the selected 87,392-byte `da591740/kernel_2712.img` in two candidate
runs, but neither run reached `rpi5-rp1-uart0-fr-read`,
`mapped/read-value`, or `PASS` serial output. A known-good control on the
restored tree `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
retained `TALOS: kernel_main` and accepted command-loop output, so the current
boundary is `blocked-pre-entry-or-handoff-after-candidate-fetch`. The mapping
contract remains unaccepted on hardware until a separately planned
pre-entry/handoff investigation or revised diagnostic reaches decisive output.

Follow-up proof status:
`phase11-rp1-diagnostic-entry-pi5-proof-20260605` reran the revised
pre-MMIO-marker candidate after source-level handoff review. The known-good
control restored
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, fetched
the 104,136-byte control kernel, reached `TALOS: kernel_main`, and retained
PASS output. The candidate tree
`0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511` fetched
the selected 87,480-byte `da591740/kernel_2712.img` twice before restore, but
serial still did not reach Talos entry, the pre-MMIO marker, diagnostic
classification, or PASS. The hardware boundary remains
`blocked-pre-entry-or-handoff-after-candidate-fetch`, not RP1 mapped/unmapped
evidence.

Closeout status: `phase11-rp1-diagnostic-entry-closeout-20260605` supersedes
the earlier `phase11-rp1-pcie-mapping-closeout-20260605` with the revised
pre-MMIO-marker hardware proof. Milestone 11.1 is accepted only at the
source-contract/local-diagnostic/candidate-fetch hardware-blocker boundary. It
does not accept a stable RP1 register read on hardware, RP1
`mapped/read-value`, GPIO ownership, interrupts, DMA/cache policy, Ethernet,
networking, SSH, storage drivers, broader PCIe enumeration, generated-root
blocker work, source-level handoff fixes, or a revised diagnostic shape.
Supervisor planning is required before any next Phase 11 slice.

Entry-control follow-up status:
`phase11-rp1-diagnostic-entry-control-source-core-20260605` accepted a local
`rpi5_rp1_entry_control` candidate that emits a unique no-RP1-MMIO
marker/PASS immediately after the normal Pi 5 `rust_entry` early-phase line
and stops before BootInfo parsing, normal Pi 5 target initialization, RP1
GPIO/pin flushes, boot reports, memory planning, or the RP1 UART0 FR read
path. The serialized
`phase11-rp1-diagnostic-entry-control-pi5-proof-20260605` then published that
candidate, but first candidate, known-good control, and candidate rerun
attempts all had empty fresh TFTP deltas. Serial output reached Raspberry Pi
firmware output through `Boot mode: NETWORK` in all three attempts, but did
not reach `TALOS: kernel_main`, entry-control PASS, known-good PASS, or any
RP1 diagnostic classification. The closeout
`phase11-rp1-diagnostic-entry-control-closeout-20260605` therefore records
`staging-or-capture-blocker`: candidate fetch, Rust entry, entry-control
reachability, RP1 mapped/read-value, RP1 unmapped/trap, firmware-state
behavior, source-level handoff fixes, revised diagnostics, Milestone 11.2,
networking, SSH, GPIO ownership, interrupts, DMA/cache, storage, generated-root
work, and broader PCIe remain unaccepted pending supervisor planning.

Staging/capture repair status:
`phase11-staging-capture-log-stability-core-20260605` repairs the proof
rule that allowed proof-time empty TFTP deltas to stand after late-visible
`/tftp/logs` replay from cursor `4088847` returned 13 events, including a
104,136-byte restored known-good `da591740/kernel_2712.img` fetch. Future
Pi 5 proofs must classify TFTP evidence before restore by re-querying from the
fresh cursor until the cursor/log size/event set is stable or a bounded timeout
is reached. This accepts only evidence-capture semantics; it does not accept
candidate fetch, Rust entry, RP1 mapped/read-value, RP1 unmapped/trap, GPIO,
interrupts, DMA/cache, storage, generated-root work, networking, SSH, broader
PCIe, or Milestone 11.2 behavior.

The serialized
`phase11-staging-capture-known-good-pi5-proof-20260605` applied the repaired
rule to the restored accepted boot tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`. Two
known-good power cycles recorded fresh serial/TFTP cursors, stable pre-restore
TFTP decisions, serial output, and restore proof, but both stable TFTP deltas
had zero events and serial did not reach `TALOS: kernel_main`, command-loop
readiness, or PASS. The classification remains
`staging-capture-still-blocked`, so RP1 candidate reruns remain blocked until
the staging/capture repair closeout and supervisor planning decide the next
bounded lab/capture step.

The closeout `phase11-staging-capture-repair-closeout-20260605` accepts only
the stable TFTP evidence semantics. It does not accept the lab/staging path for
RP1 candidate reruns because the known-good control still produced stable
zero-event TFTP deltas and no Talos serial readiness. The next Phase 11 step
requires supervisor planning for a bounded lab-controller/capture or
staging-publication discriminator before any RP1 diagnostic/source changes,
Milestone 11.2, networking, SSH, GPIO ownership, interrupts, DMA/cache, storage,
generated-root work, or broader PCIe work.

The contract repair
`phase11-lab-evidence-contract-repair-core-20260605` makes `GET /status` the
authoritative boot identity endpoint for the deployed lab API and treats
`GET /` 404s as endpoint-semantics evidence only. The next discriminator must
retain status, boot files, snapshots, fresh serial/TFTP cursors, stable
pre-restore TFTP evidence, and pre-restore inconclusive samples. Its
classification boundary distinguishes staging/publication mismatch, TFTP
capture/logging blindness, serial-only firmware reboot, and valid known-good
Talos readiness without accepting RP1 candidate behavior or Milestone 11.2
progress.

The serialized
`phase11-known-good-capture-staging-pi5-discriminator-20260605` then ran one
power cycle on the restored known-good tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
Final stable pre-restore TFTP evidence from fresh cursor `4094251` contained
13 events, including two served 104,136-byte
`da591740/kernel_2712.img` fetches, while serial reached Raspberry Pi
firmware/RP1 boot output but not `TALOS: kernel_main`, command-loop readiness,
or PASS. The classification is
`known-good-fetch-observed-without-talos-readiness`: capture/staging is now
observed for the known-good tree, but Talos runtime readiness remains a
separate blocker and no RP1 candidate, mapped/unmapped, GPIO, interrupt,
DMA/cache, networking, SSH, storage, broader PCIe, or Milestone 11.2 behavior
is accepted.

The closeout
`phase11-staging-capture-discriminator-closeout-20260605` accepts repaired
proof semantics and known-good capture/staging health only. The initial
zero-event TFTP sample is retained as capture-latency evidence but superseded
for fetch classification by the final stable pre-restore replay. The next
blocker is `boot-runtime-readiness-after-known-good-fetch`: the known-good
tree fetched `kernel_2712.img`, but serial did not reach Talos runtime
readiness. Supervisor planning is required before any RP1 candidate/source
work, candidate rerun, mapped/unmapped claim, GPIO, interrupts, DMA/cache,
networking, SSH, storage, generated-root, broader PCIe, Milestone 11.2 work,
or phase transition.

`phase11-known-good-runtime-readiness-contract-core-20260605` is now accepted
as a no-hardware contract repair for that blocker. It compares prior accepted
known-good runtime evidence against the latest fetch-without-readiness run for
restored tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` and the
104,136-byte `da591740/kernel_2712.img` fetch. The next serialized
known-good discriminator must observe serial from the fresh cursor for the
bounded 75-second/1000 ms/65536-byte window and accepts known-good runtime
readiness only if `TALOS: kernel_main` and
`rpi5-production-timer-preemption: PASS` appear after the stable pre-restore
fetch. RP1 candidate/source work, candidate reruns, mapped/unmapped claims,
GPIO, interrupts, DMA/cache, networking, SSH, storage, generated-root,
broader PCIe, Milestone 11.2, and phase transition remain blocked until that
serialized discriminator and closeout are accepted.

`phase11-known-good-runtime-readiness-pi5-discriminator-20260605` completed
with blocker evidence, and
`phase11-known-good-runtime-readiness-closeout-20260605` accepts the closeout
classification `known-good-fetch-accepted-runtime-readiness-blocked`. Stable
replay from retained fresh TFTP cursor `4095602` showed two 104,136-byte
`da591740/kernel_2712.img` fetches for the restored known-good tree, but the
bounded serial readiness window did not contain `TALOS: kernel_main`, `talos>`,
or `rpi5-production-timer-preemption: PASS`. The queued RP1 entry-control
candidate rerun remains blocked because valid known-good Talos runtime
readiness is not accepted. Supervisor planning is required for the next bounded
boot-runtime-readiness discriminator or repair before RP1 candidate/source
work, candidate rerun, mapped/unmapped claims, GPIO, interrupts, DMA/cache,
networking, SSH, storage, generated-root, broader PCIe, Milestone 11.2, or
phase transition.

phase11-known-good-runtime-lineage-and-cursor-repair-20260605 accepted a
no-hardware repair for the blank-cursor caveat and direct-cursor proof hygiene.
phase11-known-good-runtime-direct-cursor-pi5-recheck-20260605 then completed
with committed blocker evidence: fresh TFTP cursor 4096953 produced stable
pre-restore evidence with two 104,136-byte da591740/kernel_2712.img fetches,
while fresh serial cursor 4096040 did not reach TALOS: kernel_main, talos>, or
rpi5-production-timer-preemption: PASS. The direct-cursor closeout accepts
known-good-direct-cursor-fetch-runtime-readiness-blocked: known-good fetch
visibility and restore hygiene are accepted, but valid known-good Talos runtime
readiness is still blocked after confirmed fetch. The RP1 entry-control
candidate rerun must not be promoted; supervisor planning is required for a
bounded boot/runtime readiness repair or discriminator before RP1
candidate/source work, candidate rerun, mapped/unmapped claims, GPIO,
interrupts, DMA/cache, networking, SSH, storage, generated-root, broader PCIe,
Milestone 11.2, or phase transition.

phase11-known-good-runtime-serial-window-contract-20260606 is now accepted as
a no-hardware serial-observation contract repair. The single-call readiness
helper could classify only the first settled firmware/RP1 burst as the whole
window; it now accumulates serial output across repeated `/serial/observe`
calls until the requested deadline and records
`deadline-loop-accumulated-from-fresh-cursor`. This makes the next queued
known-good Pi 5 discriminator mechanically ready to test only the serial-window
boundary around the restored 104,136-byte control fetch. RP1 entry-control
candidate rerun and source work remain blocked unless that discriminator and
its closeout accept valid known-good Talos readiness.

phase11-known-good-runtime-marker-boundary-closeout-20260606 is now accepted
as `valid-known-good-talos-readiness`. The retained serial-window discriminator
evidence showed two stable 104,136-byte `da591740/kernel_2712.img` fetches and
a fresh 6,746-byte serial window that omitted `TALOS: kernel_main` but reached
`rpi5-production-timer-preemption: PASS`. Static source inspection proves the
PASS line is emitted only after the restored production-timer control has
entered `kernel_main` and completed the production-timer proof predicates, so
the missing earlier marker is treated as a serial-window completeness
limitation rather than a runtime-readiness blocker for that known-good control.
Only the existing queued RP1 entry-control candidate rerun is mechanically
eligible from this closeout, subject to its own hardware lock and validation
gates; no RP1 candidate fetch, Rust entry, mapped/read-value, unmapped/trap,
firmware-state, GPIO, interrupt, DMA/cache, storage, networking, SSH, broader
PCIe, Milestone 11.2, or phase transition is accepted here.

phase11-rp1-entry-control-candidate-rerun-20260605 then published the accepted
entry-control candidate after known-good readiness was accepted. Stable
pre-restore TFTP evidence observed two 51,808-byte
da591740/kernel_2712.img candidate fetches, but fresh serial did not reach
TALOS: kernel_main, entry-control markers, classification, or PASS. The
classification is candidate-fetch-observed-without-entry-control: fetch is
accepted, while Rust entry, pre-BootInfo entry-control reachability,
mapped/read-value, unmapped/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain blocked.

phase11-rp1-entry-control-handoff-discriminator-core-20260606 is accepted as
ready-for-rp1-handoff-pi5-discriminator. It adds the no-RP1-MMIO
rpi5_rp1_handoff_reset candidate and archive
target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz. The candidate
branches from rust_entry directly to PSCI SYSTEM_RESET before BootInfo parsing,
target initialization, boot reports, memory planning, allocator setup, or the
RP1 UART0 FR read path. Static archive/disassembly evidence shows a 45,248-byte
kernel_2712.img, arm64 Image fields text_offset=0, header_image_size=45248,
flags=12, ARMd, and _start -> rust_entry -> smc #0. The next serialized Pi 5
task may classify only candidate fetch and the reset side effect's repeated
TFTP boot/fetch signal; serial visibility and RP1 mapped/unmapped behavior
remain separate and unaccepted.

phase11-rp1-entry-control-handoff-pi5-discriminator-20260606 is accepted as
pre-bootinfo-handoff-reachability-accepted. The Pi 5 run published only
target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz, staged tree
760e7e3c59c3d6d6da4f465c9f67fc53a445bfa18850c6a76f2a3972af680d2d, and stable
same-cursor pre-restore TFTP evidence observed four 45,248-byte
da591740/kernel_2712.img fetches across two boot sequences after one power
cycle. That accepts candidate fetch and pre-BootInfo rust_entry handoff
reachability by PSCI reset side effect only. TALOS: kernel_main serial
visibility, RP1 mapped/read-value, unmapped/trap, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, and Milestone 11.2 remain blocked pending the queued handoff closeout and
later supervisor-planned diagnostic work.

phase11-rp1-entry-control-handoff-closeout-20260606 reconciles the source and
Pi 5 discriminator evidence as pre-bootinfo-handoff-reachability-accepted. The
accepted boundary is candidate fetch plus rust_entry handoff reachability by
the PSCI reset side effect only. Candidate serial visibility and entry-control
UART marker visibility remain unresolved, so the mechanically safe next
direction is supervisor planning for a focused post-handoff observability or
entry-control repair before returning to the RP1 UART0 flag-register read.
Staging/capture is not the active blocker for this boundary, but RP1
mapped/read-value, unmapped/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted.

phase11-rp1-post-handoff-marker-reset-core-20260606 is accepted as
ready-for-post-handoff-marker-reset-pi5-discriminator. It adds the no-RP1-MMIO
rpi5_rp1_post_handoff_marker_reset candidate and archive
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz. The candidate enters
rust_entry, emits the normal TALOS: rust_entry line and a unique
rpi5-rp1-post-handoff-marker-reset marker/classification through the current
UART10 early-serial helper, flushes, then calls PSCI SYSTEM_RESET before
BootInfo parsing, target initialization, boot reports, memory planning,
allocator setup, scheduler work, or the RP1 UART0 FR read path. Static
archive/disassembly evidence shows a 51,736-byte kernel_2712.img, arm64 Image
fields text_offset=0, header_image_size=51736, flags=12, ARMd, and _start ->
rust_entry -> marker writes -> smc #0. The next serialized Pi 5 task may
classify only marker visibility, reset side effect, marker-path hang/fault, or
staging/capture blocker; RP1 mapped/unmapped behavior remains separate and
unaccepted.

phase11-rp1-post-handoff-marker-reset-capture-recheck-closeout-20260606 is
accepted as reset-side-effect-accepted-marker-visibility-blocked. The repaired
Pi 5 recheck published the same 51,736-byte marker/reset archive, selected
tree 37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2, and
retained stable pre-restore TFTP evidence with 10 served candidate
da591740/kernel_2712.img fetches. Fresh serial retained repeated firmware
NETWORK boot/fetch cycles but did not show TALOS: kernel_main, TALOS:
rust_entry, or the unique marker/reset text. This accepts candidate fetch and
the PSCI reset-loop side effect only. Visible post-handoff serial
observability, marker text visibility, RP1 UART0 FR-read readiness,
mapped/read-value, unmapped/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted. The next bounded direction
requires supervisor planning for post-handoff marker visibility before
returning to the RP1 UART0 flag-register read.

phase11-rp1-rust-entry-uart10-marker-loop-core-20260606 is accepted as a
source/static marker-visibility discriminator. It adds the
rpi5_rust_entry_uart10_marker_loop scenario and non-published archive
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz. The selected
45,328-byte kernel branches _start -> rust_entry ->
run_rust_entry_uart10_marker_loop, repeatedly writes TALOS: reu10-loop
through the existing UART10 early-phase helper, and does so before BootInfo
parsing, target initialization, boot reports, memory planning, allocator setup,
scheduler work, PSCI reset, or RP1 UART0 MMIO. Static review confirms the
candidate archive SHA-256
ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b, kernel
SHA-256 6335cc2f229c38258d88000fe968248ca2e47d61e47f874bf246862e0d2b248a,
arm64 Image fields text_offset=0/header_image_size=45328/flags=12, marker
presence, and absence of RP1 UART0 FR-read report strings. This does not
accept visible marker serial output or any RP1 mapped/unmapped behavior; the
queued Pi 5 discriminator must decide marker visibility before the RP1 UART0
flag-register read can resume.

phase11-rp1-rust-entry-uart10-marker-loop-closeout-20260606 is accepted as
post-handoff-rust-entry-uart10-marker-visible after the serialized Pi 5
marker-loop run retained stable 45,328-byte da591740/kernel_2712.img candidate
fetch evidence and observed TALOS: reu10-loop 2,961 times in a fresh serial
window. This accepts only visible UART10 marker output after rust_entry for the
selected marker-loop candidate. It does not accept RP1 mapped/read-value,
unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache,
networking, SSH, storage, generated-root, broader PCIe, Milestone 11.2, or
phase transition. The existing RP1 UART0 FR-read refresh core is mechanically
unblocked next; the hardware proof and closeout remain separately gated.

phase11-rp1-uart0-fr-read-refresh-core-20260606 is accepted as the refreshed
local/static RP1 UART0 flag-register read candidate. The
rpi5_rp1_uart0_fr_read scenario now branches directly from rust_entry, emits
the start and pre-MMIO discriminator lines through the UART10 early-serial
helper, flushes UART10, performs exactly one 32-bit volatile load from
0x1f_0003_0018, reports contract id phase11-rp1-pcie-map-contract-v1, target
rp1-uart0-fr-read, address, width, raw value, mapped/read-value
classification, and PASS if the read returns, then halts. The retained archive
target/talos-rpi5-rp1-uart0-fr-read-refresh-core.tar.gz has SHA-256
da35a26e817fd30b81874a701171de1b9d47c47024d5fc405a7068ca3b2e5d60 and a
45,832-byte kernel_2712.img with text_offset=0/header_image_size=45832/flags=12.
This is source/static and archive-review evidence only; RP1 mapped/read-value,
unmapped/trap, firmware-state behavior, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase
transition remain unaccepted until the separately gated Pi 5 proof runs under
hardwareTestLock.

phase11-rp1-uart0-fr-read-closeout-20260606 is accepted as
serial-capture-saturated-after-candidate-fetch after the serialized Pi 5 proof.
The first candidate run selected tree
25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71 and
retained stable TFTP evidence with two served 45,832-byte
da591740/kernel_2712.img fetches, proving first-run candidate publication and
fetch. The fresh serial cursor was already 4194304, and candidate,
known-good-control, and candidate-rerun observations from that cursor returned
zero bytes. The known-good control still retained two served 104,136-byte
kernel fetches, while the candidate rerun retained stable zero-event TFTP
evidence. This accepts source/static candidate refresh, first-run candidate
publication/fetch, restore hygiene, and serial-capture blocker evidence only.
It does not accept RP1 mapped/read-value, unmapped/trap, firmware-state,
pre-MMIO reachability, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or phase transition. The queued
serial cursor/capture completeness repair must run before any same-shaped RP1
UART0 FR-read hardware rerun.

phase11-rp1-final-preload-marker-hold-core-20260606 is accepted as a
source/static final-preload-marker hold candidate. The
rpi5_rp1_final_preload_marker_hold scenario branches directly from rust_entry,
preserves the delayed-marker FR-read start, pre-MMIO, before-RP1-read,
repeated-preload, and final-preload marker strings, and then loops on the unique
TALOS: fr-final-preload-hold-loop marker without calling read_rp1_reg_u32,
constructing 0x1f_0003_0018, or executing the RP1 UART0 FR volatile load. The
non-published archive
target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz has SHA-256
07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287 and a
45,816-byte kernel_2712.img with text_offset=0/header_image_size=45816/flags=12.
This accepts only source/static and archive-readiness evidence; final marker
visibility, hold marker visibility, RP1 mapped/read-value, unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted until the queued serialized Pi 5 marker-visibility discriminator
runs under hardwareTestLock.

phase11-rp1-final-preload-marker-hold-pi5-discriminator-20260606 is accepted
as final-preload-hold-marker-visible. The hardware-locked run published the
accepted no-RP1-MMIO hold candidate at tree
101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47 with
effective kernel kernel_2712.img and a 45,816-byte
da591740/kernel_2712.img. Stable pre-restore TFTP retained 13 events with two
served candidate kernel fetches. Direct serial read from saturated cursor
4194304 retained 57,040 bytes with 1,628 occurrences of
TALOS: fr-final-preload-hold-loop. This accepts only the selected candidate's
hold-marker visibility; final pre-load marker visibility before the hold loop,
RP1 mapped/read-value, unmapped/trap, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remained unaccepted for closeout
reconciliation.

phase11-rp1-final-preload-marker-hold-closeout-20260606 reconciles that
source/static and hardware evidence as final-preload-hold-marker-visible. The
accepted boundary is limited to the no-RP1-MMIO hold candidate shape, selected
candidate publication/fetch evidence, visible hold-marker output, and restore
hygiene. The direct-read window did not retain the earlier final pre-load
marker, and the candidate intentionally avoided the RP1 UART0 FR volatile load,
so final pre-load marker visibility, RP1 mapped/read-value, unmapped/trap,
firmware-state behavior, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, and phase transition remain
unaccepted. Returning to the actual RP1 UART0 flag-register read now requires
supervisor planning for a non-repetitive bounded task with explicit
source/static and serialized Pi 5 acceptance gates.

phase11-pi5-proof-identity-join-repair-core-20260606, the known-good control,
and phase11-pi5-proof-identity-join-repair-closeout-20260606 repair and accept
the proof-chain gate for the next RP1 UART0 FR-read hold-control candidate
proof. The repaired gate is `pi5-proof-identity-join-v1`: one run label must
tie selected tree hash, effective kernel, expected fetch path and byte count,
serial cursor/window identity, stable TFTP cursor/delta identity, final
pre-restore identity, and restore identity. Replaying the old hold-control run
keeps it capture-staging-blocked because its TFTP/final identity matched the
restored known-good tree instead of the selected candidate. The known-good
control then passed the repaired gate on tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with two
104,136-byte da591740/kernel_2712.img TFTP fetches and a fresh direct-read
serial window containing rpi5-production-timer-preemption: PASS. The accepted
frontier is only proof-chain-ready-for-candidate-rerun. RP1 UART0 FR
mapped/read-value, bus-fault/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted until a separately queued
candidate proof passes that gate.

The separately queued repaired-proof candidate run
phase11-rp1-uart0-fr-read-hold-control-repaired-proof-pi5-20260606 is now
accepted as capture-staging-blocked. It published the accepted 46,320-byte
hold-control RP1 UART0 FR-read candidate, but the repaired
`pi5-proof-identity-join-v1` gate rejected the main serial window because
stable TFTP and final identity matched restored known-good 104,136-byte fetches
instead of selected-candidate fetches. A known-good control passed the repaired
gate, and one candidate rerun stopped same-shaped repetition after recovery
evidence again lacked candidate-byte TFTP identity. RP1 UART0 FR
mapped/read-value, bus-fault/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted. The next bounded step requires
supervisor planning for a different discriminator; another same-shaped hardware
rerun is not progress.

phase11-pi5-capture-transaction-forensics-core-20260606,
phase11-pi5-capture-transaction-no-mmio-sentinel-pi5-20260606, and
phase11-pi5-capture-transaction-v2-closeout-20260606 repair and accept the
capture transaction as proof-chain-ready-for-rp1-fr-read-v2. The v2 proof
contract requires selected candidate identity, effective kernel, expected fetch
path and byte count, an empty pre-power /serial/read drain, fresh serial
evidence, stable same-cursor TFTP evidence before restore, final pre-restore
identity, restore identity, and one shared run label. The no-MMIO sentinel
passed that contract with tree
101a453d873ecec34cf43e0db4129e81167009e8915b25926ce2308d225b1c47, two
45,816-byte da591740/kernel_2712.img fetches, an empty pre-power serial drain,
7,489 occurrences of TALOS: fr-final-preload-hold-loop, final selected-tree
identity, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5-20260606 and
phase11-rp1-uart0-fr-read-hold-control-v2-closeout-20260606 then classify the
selected RP1 UART0 FR-read candidate as candidate-fetch-without-control-marker.
The decisive candidate rerun passed v2 identity join for tree
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0 with two
46,320-byte da591740/kernel_2712.img fetches, empty pre-power serial drain,
final selected-tree identity, restore proof, and 27,177 occurrences of
TALOS: fr-hold-control-post-read-loop. The serial window did not retain the
contracted read-value/classification line, pre-read control marker, post-read
terminal marker, or trap/panic text. RP1 UART0 FR mapped/read-value,
bus-fault/trap, pre-read-control-visible-without-read-result, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, and phase transition remain unaccepted. The
next bounded step requires supervisor planning for a qualitatively different
discriminator; another same-shaped RP1 FR-read hardware rerun is not progress.

phase11-rp1-uart0-fr-tail-stable-result-core-20260606 accepts a local/static
tail-stable discriminator for that next step. The RP1 candidate keeps exactly
one contracted volatile load from RP1 UART0 FR at 0x1f00030018, then, if the
load returns, repeatedly emits TALOS: fr-tail-stable-result with contract id,
target, address, width, raw value, and mapped/read-value classification. The
paired no-RP1-MMIO control candidate constructs no RP1 FR address, performs no
RP1 volatile load, and repeatedly emits TALOS: fr-tail-stable-control with a
simulated/control classification. This accepts only source/static/archive
evidence; the queued no-MMIO Pi 5 control must pass before an RP1
mapped/read-value hardware proof can be attempted.

phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5-20260606 and
phase11-rp1-uart0-fr-tail-stable-control-closeout-20260606 accept that
hardware control as tail-stable-control-visible. The decisive control rerun
passed the v2 identity join for selected tree
b4b780193281538a643aec3c17898ae59204c335f32452b90cf08b0cb8e10161, two
45,728-byte da591740/kernel_2712.img fetches, empty pre-power serial drain,
final selected-tree identity, restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and 1,771
occurrences of TALOS: fr-tail-stable-control. This accepts only the no-MMIO
simulated/control tail-stable output shape and mechanically unblocks the queued
RP1 tail-stable result proof under hardware-lock rules. RP1 UART0 FR
mapped/read-value, bus-fault/trap, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted.

phase11-rp1-uart0-fr-tail-stable-result-pi5-20260606 accepts the RP1
tail-stable result proof as mapped-read-value-tail-stable. The decisive
candidate rerun passed the v2 identity join for selected tree
0e187f9f73118c237337b25d85e57c51dbf18a18bf87ab0d3850c63291b153eb, two
45,800-byte da591740/kernel_2712.img fetches, empty pre-power serial drain,
final selected-tree identity, restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10, and 1,498
occurrences of TALOS: fr-tail-stable-result carrying contract
phase11-rp1-pcie-map-contract-v1, target rp1-uart0-fr-read, address
0x1f00030018, width 32, raw 0xdeaddead, and classification
mapped/read-value. This accepts only the first read-only RP1 UART0 FR
mapped/read-value diagnostic boundary. GPIO/pin-control ownership, RP1
clocks/resets, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, and phase transition remain unaccepted.

phase11-rp1-uart0-fr-tail-stable-result-closeout-20260606 reconciles the
tail-stable source/static core, no-MMIO control, and RP1 Pi 5 proof as
mapped-read-value-tail-stable. The accepted frontier is limited to the
read-only RP1 UART0 FR single-load diagnostic at 0x1f00030018 with v2
candidate identity, stable TFTP, final pre-restore identity, restore proof,
and repeated tail-stable result markers carrying raw 0xdeaddead. No explicit
queued task remains after the closeout; supervisor planning is required for
the next bounded Phase 11 slice. GPIO/pin-control ownership, clocks/resets,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

phase11-rp1-mapping-frontier-checkpoint-20260607 accepts that read-only RP1
UART0 FR diagnostic as the Milestone 11.1 frontier and authorizes only the next
bounded source-contract task, phase11-rp1-irq-clock-gpio-source-contract-20260607.
GPIO/pin-control ownership, RP1 clocks/resets, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.2
implementation, and phase transition remain unaccepted until that source
contract and later proof tasks satisfy their own gates.

phase11-rp1-irq-clock-gpio-source-contract-20260607 is accepted as
phase11-rp1-irq-clock-gpio-contract-v1. It retains Raspberry Pi Linux
`rpi-6.12.y` source references for RP1 GPIO/pads, RP1 interrupt IDs/MSI-X
routing, and RP1 clock/reset assumptions, then selects exactly one next
diagnostic: the read-only `rp1-gpio14-status-read`, a single 32-bit volatile
load from CPU physical `0x1f000d0070` for RP1 IO_BANK0 GPIO14 STATUS. The
paired no-MMIO control must be accepted before any real Pi 5 diagnostic proof.
GPIO/pin-control ownership, pad writes, interrupt enablement or delivery,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, and Pi 5 hardware behavior for this diagnostic remain
unaccepted.

phase11-rp1-gpio-status-no-mmio-control-pi5-20260607 accepted the paired
no-MMIO output shape as visible on Pi 5. The accepted rerun passed the v2
identity join, retained two 46,160-byte selected candidate TFTP fetches,
retained repeated TALOS: gpio14-status-control output, and restored the lab to
the pre-run boot tree. phase11-rp1-gpio-status-diagnostic-pi5-20260607 and
phase11-rp1-irq-clock-gpio-diagnostic-closeout-20260607 then closed the first
real GPIO14 STATUS proof as capture-staging-blocked. A marker-visible run
retained 483 TALOS: gpio14-status-result occurrences, but the v2 identity join
rejected it due to serial-drain, TFTP byte, and final selected-tree mismatches;
the required known-good control and candidate rerun were retained. Real RP1
GPIO14 STATUS behavior, bus-fault/trap behavior, GPIO ownership, pad writes,
interrupt delivery, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
unaccepted. Same-shaped GPIO14 STATUS hardware reruns are blocked pending
supervisor planning for a different discriminator or capture/staging repair.

phase11-pi5-capture-identity-repaired-control-pi5-20260607 then accepted the
repaired capture/identity path on the GPIO14 no-MMIO control, mechanically
unblocking the single queued real diagnostic rerun. The repaired proof
phase11-rp1-gpio-status-repaired-proof-pi5-20260607 is accepted as
gpio14-status-result-identity-joined. The decisive rerun passed the v2 identity
join for tree cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a,
retained two 46,336-byte da591740/kernel_2712.img fetches, final selected-tree
identity, restore proof, and 390 TALOS: gpio14-status-result records carrying
contract phase11-rp1-irq-clock-gpio-contract-v1, target rp1-gpio14-status-read,
address 0x1f000d0070, raw 0xdeaddead, and
classification=diagnostic-result-visible. This accepts only the read-only
GPIO14 STATUS diagnostic boundary. GPIO ownership, pad writes, interrupt
delivery, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
unaccepted.

phase11-rp1-irq-clock-gpio-repaired-proof-closeout-20260607 reconciles the
source contract, local diagnostic core, repaired capture identity replay,
repaired no-MMIO control, and real GPIO14 STATUS repaired proof as
gpio14-status-read-frontier-closed. The accepted frontier is limited to that
one read-only GPIO14 STATUS load and its v2 identity-joined Pi 5 result
evidence. Same-shaped GPIO14 STATUS hardware reruns are blocked unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
At closeout time, no explicit worker-owned task remained; supervisor planning
was required before the next Milestone 11.2 interrupt-routing source contract.

phase11-rp1-interrupt-routing-source-contract-20260607 is accepted as
phase11-rp1-interrupt-routing-source-contract-v1. It retains Raspberry Pi
Linux RP1 interrupt-domain/MSI-X behavior, GPIO bank0 parent interrupt
identity, and BCM2712 pcie2/MIP0/GIC source routing assumptions. The selected
next diagnostic is the read-only/no-enable `rp1-io-bank0-msix-cfg-read`, a
single 32-bit volatile load from RP1 `MSIX_CFG(0)` at CPU physical
`0x1f00108008`; source inspection predicts RP1 hwirq 0 through PCI MSI-X
vector 0 and MIP0 MSI vector 0 to GIC SPI 128 / INTID 160. That route remains
unaccepted hardware behavior. The paired no-MMIO/no-enable control must pass
before any real Pi 5 proof. Interrupt enablement/delivery, GPIO ownership,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-interrupt-routing-no-mmio-control-pi5-20260607 accepted the paired
no-MMIO/no-enable output shape as visible on Pi 5. The subsequent real
diagnostic proof phase11-rp1-interrupt-routing-diagnostic-pi5-20260607 is
accepted as routing-msix-cfg-visible. The decisive rerun passed the v2 identity
join for tree `63800845c9837b3d57153051583b269070b028412bcd57ea9c55a5f9e56a2304`,
retained two 46,648-byte `da591740/kernel_2712.img` fetches, final
selected-tree identity, restore proof, and 970
`TALOS: rp1-interrupt-routing-result` records carrying contract
phase11-rp1-interrupt-routing-source-contract-v1, target
rp1-io-bank0-msix-cfg-read, address `0x1f00108008`, raw `0xdeaddead`, and
classification=routing-msix-cfg-visible. This accepts only the selected
read-only/no-enable MSIX_CFG(0) diagnostic boundary. Interrupt delivery,
handler ownership, GPIO ownership, pin-control behavior, clocks/resets,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-interrupt-routing-diagnostic-closeout-20260607 reconciles the
source contract, local/static core, no-MMIO/no-enable Pi 5 control, and real
Pi 5 diagnostic proof as interrupt-routing-msix-cfg-read-frontier-closed. The
accepted frontier is limited to the source-backed IO_BANK0 interrupt identity,
selected read-only/no-enable MSIX_CFG(0) diagnostic boundary, paired control
proof, and real Pi 5 visibility proof. Same-shaped MSIX_CFG(0) hardware reruns
are blocked unless a future supervisor task supplies a different discriminator
or new acceptance criteria. No explicit worker-owned task remains; supervisor
planning is required for the next Milestone 11.2 feature slice. Interrupt
delivery, handler ownership, GPIO ownership, pin-control behavior, clocks and
resets, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gic-visible-route-source-contract-20260607 is accepted as
phase11-rp1-gic-visible-route-source-contract-v1. It selects the next
read-only/no-ack GIC-visible route status diagnostic for the source-predicted
RP1 IO_BANK0 route to GIC SPI 128 / INTID 160. The allowed status snapshot is
limited to GICD_ISENABLER5 at `0x10_7fff_9114`, GICD_ISPENDR5 at
`0x10_7fff_9214`, GICD_ISACTIVER5 at `0x10_7fff_9314`, and GICC_HPPIR at
`0x10_7fff_a018`. INTID 160 is bank 5 bit 0. The paired control must construct
no GIC, RP1, MSI-X, PCIe, MIP, GPIO, pads, RIO, or clock/reset MMIO path before
any real Pi 5 proof. This accepts only a source contract, not pending state,
interrupt delivery, IAR/EOIR acknowledgement, handler ownership, GPIO
ownership, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, or phase transition.

phase11-rp1-gic-visible-route-no-mmio-control-pi5-20260607 accepted the paired
no-MMIO/no-GIC/no-RP1 output shape as visible on Pi 5. The subsequent real
diagnostic proof phase11-rp1-gic-visible-route-diagnostic-pi5-20260607 is
accepted as gic-route-status-visible. The decisive rerun passed the v2 identity
join for tree `8ef75b3125c21d7025cff539f5004d7f6911af057c5523ce1610be46deecbbe4`,
retained two 47,816-byte `da591740/kernel_2712.img` fetches, final
selected-tree identity, restore proof, and 209
`TALOS: rp1-gic-route-status-result` records carrying contract
phase11-rp1-gic-visible-route-source-contract-v1, target
rp1-io-bank0-gic-route-status-read, predicted GIC SPI 128 / INTID 160,
enable/pending/active bits clear for INTID 160, and HPPIR spurious. This
accepts only the selected read-only/no-ack GIC-visible status boundary.
Interrupt delivery, handler ownership, GPIO ownership, pin-control behavior,
clocks/resets, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gic-visible-route-closeout-20260607 reconciles the source
contract, local/static core, no-MMIO/no-GIC/no-RP1 Pi 5 control, and real
Pi 5 diagnostic proof as gic-visible-route-status-frontier-closed. The
accepted frontier is limited to the source-backed RP1 IO_BANK0 route identity,
selected read-only/no-ack GICv2 status snapshot for INTID 160, paired control
proof, and real Pi 5 visibility proof. Same-shaped GIC-visible route status
hardware reruns are blocked unless a future supervisor task supplies a
different discriminator or new acceptance criteria. No explicit worker-owned
task remains; supervisor planning is required for the next Milestone 11.2
feature slice. Interrupt pending generation, delivery, IAR/EOIR
acknowledgement, handler ownership, GPIO ownership, pin-control behavior,
clocks and resets, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-bank-source-status-contract-20260607 is accepted as
phase11-rp1-gpio-bank-source-status-contract-v1. It selects the next
read-only/non-destructive RP1 IO_BANK0 source-status diagnostic before any
GPIO event generation, interrupt enablement, or delivery work. The allowed
snapshot is limited to IO_BANK0 INTS at `0x1f000d0124` and IO_BANK0 INTE at
`0x1f000d011c`, both 32-bit volatile loads. Bank0 covers GPIO0 through
GPIO27, GPIO14 is bit mask `0x00004000`, and Linux source acknowledges GPIO
events through separate GPIO CTRL IRQRESET writes, which this contract
forbids. The paired control must construct no RP1 GPIO/RIO/pads/clock/reset,
MSI-X/PCIe/MIP, or GIC MMIO path before any real Pi 5 proof. This accepts only
a source contract, not GPIO event generation, pending generation, interrupt
enablement or delivery, IAR/EOIR acknowledgement, handler ownership, GPIO
ownership, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, or phase transition.

phase11-rp1-gpio-bank-source-status-pi5-20260607 accepts the real read-only
GPIO bank source-status proof as gpio-bank-source-status-visible. After an
inconclusive first candidate run, a known-good control retained matching TFTP
and PASS evidence, and the decisive candidate rerun passed the v2 identity
join for tree 84ee89db45d5298e49f44c74e6a18b9c07ce2c146879f677aceace6ad252ea0f.
The accepted rerun retained two 46,904-byte da591740/kernel_2712.img fetches,
final selected-tree identity, restore proof, and 269 TALOS:
rp1-gpio-bank-source-status-result records. The visible result reported
IO_BANK0 INTE at 0x1f000d011c and INTS at 0x1f000d0124, raw values
0xdeaddead/0xdeaddead, GPIO14 mask 0x4000, gpio14-enabled=true, and
gpio14-source-status=true. This accepts only the selected read-only source
snapshot visibility and report decoding. GPIO event generation, interrupt
pending generation beyond that snapshot, interrupt enablement or delivery,
IAR/EOIR acknowledgement, handler ownership, GPIO ownership, pin-control
behavior, clocks and resets, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-bank-source-status-closeout-20260607 closes the GPIO bank
source-status chain as gpio-bank-source-status-frontier-closed. The accepted
frontier is limited to the source-backed RP1 IO_BANK0 INTE/INTS register
identity, selected read-only source-status snapshot, paired control proof, and
real Pi 5 visibility proof. Same-shaped GPIO bank source-status hardware
reruns are blocked unless a future supervisor task supplies a different
discriminator or new acceptance criteria. No explicit worker-owned task
remains; supervisor planning is required for the next Milestone 11.2 feature
slice. GPIO event generation, interrupt pending generation beyond the
read-only snapshot, interrupt enablement or delivery, IAR/EOIR
acknowledgement, handler ownership, GPIO ownership, pin-control behavior,
clocks and resets, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-event-latch-source-contract-20260607 is accepted as
source-contract-blocked. Retained RP1/Linux source identifies the GPIO14 event
configuration path through GPIO14 CTRL event-enable/IRQRESET writes and IO_BANK0
INTE source-enable writes, but it does not justify a safe bounded diagnostic
that deliberately changes GPIO14 event or pending state while GPIO14 may remain
firmware-owned as UART0 TXD. No local/static core, no-write control, or Pi 5
event-latch proof is authorized from this blocker. A future supervisor-planned
task must first define GPIO ownership, parent-route masking, exact restore
semantics, and a deterministic event source. GPIO event generation, interrupt
pending generation beyond the prior read-only snapshot, interrupt enablement or
delivery, IAR/EOIR acknowledgement, handler ownership, GPIO ownership,
pin-control behavior, clocks and resets, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
unaccepted.

phase11-rp1-gpio-ownership-restore-source-contract-20260607 is accepted as
phase11-rp1-gpio-ownership-restore-source-contract-v1. It selects only a
read-only GPIO14 ownership/route preflight target,
rp1-gpio14-ownership-route-preflight-read, before any future event-generation
retry. The allowed reads are GPIO14 STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0
OUT/OE/IN, GPIO14 pad control, and the accepted INTID 160 GIC route status
registers. The contract records GPIO14's source-backed fsel table, including
uart0 at fsel 4, gpio at fsel 5, and proc_rio at fsel 6, so the preflight can
report whether the pin appears compatible with later GPIO event ownership
without switching function or direction. No writes are allowed, and cleanup is
no-op hardware-state cleanup because the preflight is read-only. The paired
control must construct no RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or
GIC MMIO path before any real Pi 5 proof. This accepts only the read-only
preflight source contract, not GPIO ownership, GPIO event generation,
interrupt pending generation, interrupt enablement or delivery, IAR/EOIR
acknowledgement, handler ownership, GPIO CTRL/INTE/RIO/pad writes,
parent-route masking writes, clocks and resets, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, or phase
transition.

phase11-rp1-gpio-ownership-restore-control-pi5-20260607 and
phase11-rp1-gpio-ownership-restore-pi5-20260607 complete the serialized
control/real preflight proof chain. The real Pi 5 proof is accepted as
gpio14-ownership-preflight-blocked-non-gpio-function: the decisive rerun
passed pi5-capture-transaction-v2 with selected tree
91372af6aeecc90b47b57d6d3f1caf46ee5b20f47ec392977fdae2674ac0112f, two
50056-byte candidate TFTP fetches, and 93
TALOS: rp1-gpio14-ownership-route-preflight-result markers. The observed
preflight reported GPIO14 fsel 13 / unknown function, so any event-generation
retry remains blocked pending new supervisor planning around GPIO14 ownership,
function selection, parent-route masking, deterministic event source, and
restore semantics. GPIO ownership, GPIO event generation, interrupt pending
generation beyond the read-only snapshot, interrupt enablement or delivery,
GIC acknowledgement, handler ownership, GPIO CTRL/INTE/RIO/pad writes, clocks
and resets, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-ownership-restore-closeout-20260607 closes that chain as
gpio14-ownership-preflight-blocked-frontier-closed. The accepted frontier is
only the source-backed GPIO14 ownership/route preflight register identity, the
read-only local/static real candidate, the paired no-MMIO/no-RP1/no-GIC
control proof, and the real Pi 5 blocker proof. Same-shaped GPIO
ownership/route preflight, event-latch, or event-generation hardware reruns
are blocked without a different discriminator or supervisor-planned
ownership/function selection, parent-route masking, deterministic event-source,
partial-write recovery, and restore acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 feature slice; this closeout does not
create a worker-owned follow-up task or accept a phase transition.

phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607 is
accepted as accepted-source-contract. It selects GPIO16, not GPIO14, for the
next bounded RP1 GPIO event/source-status discriminator because retained Pi 5
source names GPIO16 as a generic line, no retained fixed board consumer uses
it, the lab debug UART is uart10, and the prior Talos RP1 UART0 path is
GPIO14/GPIO15. The accepted target is
rp1-gpio16-owned-level-high-event-discriminator with source-backed GPIO16
STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad, and read-only
INTID 160 GIC-visible status observations. The only accepted writes are the
bounded GPIO16 pad/CTRL/RIO/event-enable/IRQRESET/IO_BANK0-INTE bit-16 writes
and exact restore writes named by the task, and only after a parent-route
preflight shows INTID 160 disabled, not pending, not active, and not visible in
HPPIR. Hardware behavior, interrupt delivery, GIC acknowledgement, handler
ownership, broad GPIO ownership, GPIO14 event-generation retry, clocks and
resets, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-gpio-owned-event-discriminator-control-pi5-20260607 accepts the
paired no-MMIO/no-RP1/no-GIC control output shape as visible on Pi 5. After an
initial serial-drain-rejected candidate run and a known-good control, the
accepted candidate rerun passed the v2 identity join, retained two stable
49,480-byte candidate fetches, retained 40 control markers, and restored the
lab to the pre-run production-timer boot tree. This accepts only the
simulated/control output path; real GPIO16 event/source-status behavior remains
queued behind its own serialized hardware proof.

phase11-rp1-gpio-owned-event-discriminator-pi5-20260607 accepts the real Pi 5
GPIO16 event discriminator run as
gpio16-owned-event-preflight-blocked-pin-function. After the first candidate
run was rejected by non-empty pre-power serial drain evidence, the worker ran a
known-good control and reran the same candidate without code changes. The
accepted rerun passed the v2 identity join, retained two stable 52,056-byte
candidate fetches, retained 38 real result markers, and restored the lab to the
pre-run production-timer boot tree. The result reported GPIO16 fsel 13 /
unknown function, so the diagnostic skipped all accepted GPIO16 action writes.
This accepts only the pin-function preflight blocker; GPIO16 event generation,
interrupt pending generation, interrupt delivery, GIC acknowledgement, handler
ownership, broad GPIO ownership, clocks and resets, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-gpio-owned-event-discriminator-closeout-20260607 closes the chain
as gpio16-owned-event-preflight-blocked-frontier-closed. The accepted boundary
is only the source-backed GPIO16 discriminator contract, the local
real/control candidate split, the paired no-MMIO/no-RP1/no-GIC control proof,
and the real Pi 5 blocker proof. Same-shaped GPIO16 event-discriminator
hardware reruns are not progress without a different discriminator or new
source-backed ownership/function acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 feature slice; this closeout does not
create a worker-owned follow-up task or accept a phase transition.

phase11-rp1-clock-reset-status-source-contract-20260607 is accepted as
phase11-rp1-clock-reset-status-source-contract-v1. It selects a single
read-only/no-write RP1 clock manager status diagnostic after GPIO14 and
GPIO16 both blocked on fsel 13 / unknown function. The selected target is
`rp1-clock-manager-status-read`, limited to 32-bit volatile loads from
`PLL_SYS_CS` at `0x1f00020000`, `CLK_SYS_CTRL` at `0x1f00018014`,
`CLK_SYS_DIV_INT` at `0x1f00018018`, `CLK_SYS_SEL` at `0x1f00018020`,
`CLK_SLOW_SYS_CTRL` at `0x1f00018024`, `CLK_UART_CTRL` at `0x1f00018054`,
`CLK_UART_DIV_INT` at `0x1f00018058`, and `CLK_UART_SEL` at
`0x1f00018060`. The contract may decode PLL lock, clock enable, source,
and divider fields and report the retained GPIO14/GPIO16 fsel 13 blocker
context. The paired no-MMIO/no-RP1/no-GIC control must pass before any real
Pi 5 proof. Linux RP1 reset behavior is retained only as forbidden source
context. Clock/reset writes or ownership, GPIO ownership retries, event
generation, interrupt delivery, GIC acknowledgement, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-reset-status-core-20260607 is accepted as
accepted-local-static-clock-reset-status-core. It adds the real
rpi5_rp1_clock_manager_status_read diagnostic and paired
rpi5_rp1_clock_manager_status_no_mmio_control candidate. The real candidate
performs only the eight contracted read-only clock manager loads and reports
TALOS: rp1-clock-manager-status-result; the control preserves the output shape
with no RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address
construction. No hardware behavior is accepted by this task.

phase11-rp1-clock-reset-status-control-pi5-20260607 accepted the
no-MMIO/no-RP1/no-GIC clock/reset/status control path as
no-mmio-clock-reset-status-control-visible. The run selected tree
eeb71c0bfc3cbd259a18c5f53403555628a5cf8f3273d764cab80656087dbb66, retained
two 47,120-byte candidate TFTP fetches, passed v2 identity join with no
rejection reasons, retained 49 control markers, and restored to the pre-run
tree. This accepts only the simulated/control output and capture path.

phase11-rp1-clock-reset-status-pi5-20260607 accepted the real read-only clock
manager status diagnostic as rp1-clock-manager-status-visible. After
serial-drain/capture triage, the accepted rerun selected tree
3e64059ed440eaf48f096d8e2e4113609dbfe9f78444955003547515439c3704, retained
two 47,280-byte candidate TFTP fetches, passed v2 identity join with no
rejection reasons, retained 320 result markers, and reported
pll-sys-lock=true, clk-sys-enabled=true, and clk-uart-enabled=true. The lab was
restored to the original pre-run tree after the accepted rerun.

phase11-rp1-clock-reset-status-closeout-20260607 closes the chain as
rp1-clock-manager-status-frontier-closed. The accepted frontier is only the
source-backed clock manager status contract, local real/control candidate
split, no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 read-only status
visibility proof. Clock/reset writes or ownership, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted. Supervisor planning is
required for the next Milestone 11.2 feature slice; this closeout does not
create a worker-owned follow-up task.

phase11-rp1-clock-reset-write-restore-source-contract-20260607 is accepted as
phase11-rp1-clock-reset-write-restore-source-contract-v1. It selects the
bounded idempotent `rp1-clk-adc-ctrl-idempotent-write-restore` target:
pre-read `CLK_ADC_CTRL` at `0x1f00018144`, write the pre-read raw value back to
that register, post-read, restore-write the same pre-read value, and
restore-read. The expected unchanged fields are the full raw value,
`CLK_CTRL_ENABLE` bit 11, `CLK_CTRL_AUXSRC` bits 9:5, and source bits.
`clk_adc` has no GPCLK output-enable mask and the retained ADC device-tree
consumer is disabled, so this does not intentionally disturb boot UART,
critical system clocks, PCIe/RP1 access, GPIO14/GPIO16 state, interrupt
routing, serial capture, or reset-controller paths. The paired
no-MMIO/no-RP1/no-GIC control must pass before any real Pi 5 proof. This
accepts only the narrow idempotent clock-manager write/readback/restore source
contract; non-idempotent clock changes, reset writes, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-reset-write-restore-core-20260607 is accepted as
accepted-local-static-clock-adc-ctrl-write-restore-core. It adds the real
rpi5_rp1_clock_adc_ctrl_write_restore diagnostic and paired
rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control candidate. The real
candidate performs only the contracted CLK_ADC_CTRL pre-read, idempotent
write-back, post-read, restore-write, and restore-read sequence and reports
TALOS: rp1-clock-adc-ctrl-write-restore-result; the control preserves the
output shape with no RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC
MMIO address construction. No hardware behavior is accepted by this task.

phase11-rp1-clock-reset-write-restore-control-pi5-20260607 accepted the
no-MMIO/no-RP1/no-GIC clock ADC control path as
no-mmio-clock-adc-ctrl-write-restore-control-visible. After
serial-drain/capture triage, the accepted rerun selected tree
94775dea793b4493ad2cdbdfd3bd6e8882362d10d440a0fadb1ed9296ab27f8e, retained
two 46,888-byte candidate TFTP fetches, passed v2 identity join with no
rejection reasons, retained 108 control markers, and restored to the pre-run
tree. This accepts only the simulated/control output and capture path.

phase11-rp1-clock-reset-write-restore-pi5-20260607 accepted the real
CLK_ADC_CTRL idempotent write/readback/restore diagnostic as
rp1-clock-adc-ctrl-idempotent-write-restored. After serial-drain/capture
triage, the accepted rerun selected tree
3ea80fee925c554e0e65141bbd18174ab661b3e5ac6a73b82d7c130ca7adb709, retained
two 47,232-byte candidate TFTP fetches, passed v2 identity join with no
rejection reasons, retained 102 result markers, and reported
pre-raw=0xdeaddead, post-raw=0xdeaddead, restore-raw=0xdeaddead,
post-eq-pre=true, and restore-eq-pre=true. The lab was restored to the
original pre-run tree after the accepted rerun.

phase11-rp1-clock-reset-write-restore-closeout-20260607 closes the chain as
rp1-clock-adc-ctrl-write-restore-frontier-closed. The accepted frontier is
only the source-backed CLK_ADC_CTRL idempotent write/readback/restore contract,
local real/control candidate split, no-MMIO/no-RP1/no-GIC control proof, and
real Pi 5 proof that the selected write-back and restore-read preserved the
pre-read raw value for this run. Broad clock/reset ownership, non-idempotent
clock programming, reset-controller writes, GPIO ownership, event generation,
interrupt delivery, GIC acknowledgement, handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted. Supervisor planning is required for the next
Milestone 11.2 feature slice; this closeout does not create a worker-owned
follow-up task.

phase11-rp1-clock-adc-enable-toggle-source-contract-20260607 is accepted as
phase11-rp1-clock-adc-enable-toggle-source-contract-v1. It selects the bounded
non-idempotent rp1-clk-adc-ctrl-enable-bit-toggle-restore target: pre-read and
report CLK_ADC_CTRL at 0x1f00018144, compute
transition_raw = pre_raw ^ 0x00000800, write that transition value, post-read,
restore-write pre_raw, and restore-read. Accepted invariants require a one-bit
enable transition, restore equality, unchanged auxsrc/source fields, and a
paired no-MMIO/no-RP1/no-GIC control before any real Pi 5 proof. The source
evidence is limited to clk_adc, whose retained Linux clock descriptor has no
GPCLK output-enable mask and whose retained ADC device-tree consumer is
disabled. This accepts only the source contract for the selected enable-bit
transition/readback/restore boundary. Hardware behavior, broad clock/reset
ownership, reset-controller writes, GPIO ownership, event generation,
interrupt delivery, GIC acknowledgement, handler ownership, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-clock-adc-enable-toggle-core-20260607 is accepted as
accepted-local-static-clock-adc-ctrl-enable-toggle-core. It adds the real
rpi5_rp1_clock_adc_ctrl_enable_toggle diagnostic and paired
rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control candidate. The real
candidate performs only the contracted CLK_ADC_CTRL pre-read, enable-bit
transition-write, post-read, restore-write, and restore-read sequence and
reports TALOS: rp1-clock-adc-ctrl-enable-toggle-result; the control preserves
the output shape with no RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or
GIC MMIO address construction. No hardware behavior is accepted by this task.

phase11-rp1-clock-adc-enable-toggle-control-pi5-20260607 accepted only the
serialized Pi 5 no-MMIO/no-RP1/no-GIC control output/capture path. After a
first candidate run and known-good control were rejected by serial-drain
freshness evidence, the accepted control rerun selected tree
37d1a4225602da70e0f1aba12047a77f5ab8644a9eba23854d31d05afdd068d1, retained
two served 47,240-byte da591740/kernel_2712.img TFTP fetches, passed the v2
identity join with no rejection reasons, retained 84 TALOS:
rp1-clock-adc-ctrl-enable-toggle-control records, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
proves only the simulated/control output and capture path before the real
diagnostic.

phase11-rp1-clock-adc-enable-toggle-pi5-20260607 accepted the selected real
CLK_ADC_CTRL enable-bit transition attempt as
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored. After a first candidate
run was rejected by capture/staging evidence, a known-good control passed, and
the accepted diagnostic rerun selected tree
7024bb54a9446c681d4a8b9c80372fe52a4d4f93b7939f299a8eb2d7199a697a, retained
two served 47,512-byte da591740/kernel_2712.img TFTP fetches, passed the v2
identity join with no rejection reasons, retained 78 TALOS:
rp1-clock-adc-ctrl-enable-toggle-result records, and restored the lab to the
original pre-run tree. The visible result reported pre-raw=0xdeaddead,
transition-raw=0xdeadd6ad, post-raw=0xdeaddead, restore-raw=0xdeaddead,
one-bit-transition=true, post-enable-flipped=false,
post-delta-is-transition-mask=false, and restore-eq-pre=true. This accepts a
restored mismatch blocker, not successful non-idempotent clock ownership.

phase11-rp1-clock-adc-enable-toggle-closeout-20260607 closes the chain as
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed. The
accepted frontier is only the source-backed CLK_ADC_CTRL enable-bit
transition/readback/restore contract, local real/control candidate split,
no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 proof that the selected
transition attempt ran under identity-joined evidence and restored the
observed pre-read raw value. Same-shaped enable-toggle hardware reruns are
blocked unless a future supervisor task supplies a different discriminator or
new acceptance criteria. Successful non-idempotent clock ownership, broad
clock/reset ownership, reset-controller writes, GPIO ownership, event
generation, interrupt delivery, GIC acknowledgement, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.3, and phase transition remain unaccepted. Supervisor planning is required
for the next Milestone 11.2 feature slice; this closeout does not create a
worker-owned follow-up task.

phase11-rp1-clock-write-effect-discriminator-source-contract-20260607 is
accepted as phase11-rp1-clock-write-effect-discriminator-source-contract-v1.
It selects one read-only rp1-clk-adc-window-coherence-read discriminator
before any further RP1 clock writes. The allowed reads are CLK_SYS_CTRL,
CLK_UART_CTRL, two ordered CLK_ADC_CTRL reads, CLK_ADC_DIV_INT, and
CLK_ADC_SEL; no writes or restore operation are selected. The report must
retain the prior 0xdeaddead/0xdeadd6ad mismatch-restored context and expose
ADC CTRL stability, ADC window repeated-sentinel state, ADC selector shape,
and clk_sys/clk_uart guard fields. The paired no-MMIO/no-RP1/no-GIC control
must pass before any real Pi 5 proof. Successful non-idempotent clock
ownership, broad clock/reset ownership, divider/source/PLL/frequency-counter/
reset-controller writes, GPIO ownership, event generation, interrupt delivery,
handler ownership, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-clock-write-effect-discriminator-core-20260607 is accepted as the
local/static implementation of that discriminator and paired control. The next
hardware step, phase11-rp1-clock-write-effect-discriminator-control-pi5-20260607,
is also accepted as no-mmio-clock-adc-window-coherence-control-visible: after
an inconclusive first candidate run, a production-timer known-good control
passed the v2 identity join, and the unchanged candidate rerun retained two
matching 47,360-byte TFTP fetches, 52 control markers, final selected-tree
identity, and restore proof. This accepts only the no-MMIO/no-RP1/no-GIC
control output/capture path; the real ADC clock-window coherence Pi 5 proof is
the next mechanically gated task.

phase11-rp1-clock-write-effect-discriminator-pi5-20260607 is accepted as
rp1-clock-adc-window-readback-sentinel. After an inconclusive first capture, a
production-timer known-good control passed the v2 identity join, and the
unchanged real candidate rerun retained two matching 48,056-byte TFTP fetches,
52 result markers, final selected-tree identity, and restore proof. The
accepted output reported CLK_SYS_CTRL, CLK_UART_CTRL, two ordered CLK_ADC_CTRL
reads, CLK_ADC_DIV_INT, and CLK_ADC_SEL all returning 0xdeaddead, with
adc-ctrl-stable=true, adc-window-all-equal=true, and
adc-window-all-deaddead=true. This accepts only the selected read-only
sentinel/result boundary; successful non-idempotent clock ownership, broad
clock/reset ownership, any new clock/reset write, GPIO ownership, event
generation, interrupt delivery, handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-clock-write-effect-discriminator-closeout-20260607 is accepted as
rp1-clock-adc-window-readback-sentinel-frontier-closed. The closeout reconciles
the source contract, local/static core, no-MMIO/no-RP1/no-GIC control proof,
real Pi 5 proof, restore evidence, and evidence maps into one accepted
frontier. The accepted boundary is only the read-only ADC clock-window
coherence sentinel/result: the selected clock-manager window returned repeated
0xdeaddead values across CLK_SYS_CTRL, CLK_UART_CTRL, two ordered CLK_ADC_CTRL
reads, CLK_ADC_DIV_INT, and CLK_ADC_SEL. Successful non-idempotent clock
ownership, broad clock/reset ownership, any new clock/reset write,
GPIO ownership, event generation, interrupt delivery, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.3, and phase transition remain unaccepted. Supervisor planning is required
for the next Milestone 11.2 feature slice; this closeout does not create a
worker-owned follow-up task.

phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608 is
accepted as
phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1. It selects
one read-only SYSINFO identity versus retained clock-window sentinel
discriminator before any further RP1 clock writes, GPIO ownership retry, or
interrupt-delivery work. The accepted target is
rp1-sysinfo-vs-clock-sentinel-read, limited to 32-bit loads from
SYSINFO_CHIP_ID at 0x1f00000000, SYSINFO_PLATFORM at 0x1f00000004, and
CLK_ADC_CTRL at 0x1f00018144. A live SYSINFO chip id matching retained Pi 5
firmware identity 0x20001927 while CLK_ADC_CTRL remains 0xdeaddead would
separate RP1 identity/address decode from the clock-window sentinel. SYSINFO
also returning 0xdeaddead would localize the blocker to the broader
SYSINFO/address-decode path. The paired no-MMIO/no-RP1/no-GIC control must
pass before any real Pi 5 proof. This source contract does not accept runtime
or hardware behavior, broad RP1 clock/reset ownership, clock/reset writes,
GPIO ownership, event generation, interrupt delivery, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.3, or a phase transition.

phase11-rp1-clock-sentinel-address-discriminator-pi5-20260608 is accepted as
rp1-sysinfo-and-clock-window-sentinel. The accepted real rerun published only
the committed read-only SYSINFO-vs-clock-sentinel candidate as tree
22c13cf75878b9f1776d9ae00b760457df45a508b915c3032f4ac792693a74a4, retained two
47,776-byte da591740/kernel_2712.img TFTP fetches, retained 62 result markers,
passed the pi5-capture-transaction-v2 identity join with no rejection reasons,
and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The Talos
read-only loads reported SYSINFO_CHIP_ID=0xdeaddead, SYSINFO_PLATFORM=0xdeaddead,
and CLK_ADC_CTRL=0xdeaddead, so the new frontier is a broader
SYSINFO/address-decode sentinel boundary rather than live RP1 SYSINFO identity
or clock/reset ownership. Clock/reset writes, GPIO ownership, event generation,
interrupt delivery, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
unaccepted.

phase11-rp1-clock-sentinel-address-discriminator-closeout-20260608 is
accepted as rp1-sysinfo-and-clock-window-sentinel-frontier-closed. The closeout
reconciles the source contract, local/static core, no-MMIO/no-RP1/no-GIC
control proof, real Pi 5 proof, restore evidence, and evidence maps into one
accepted frontier. The accepted boundary is only the read-only
SYSINFO/address-decode sentinel result: SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and
CLK_ADC_CTRL all returned 0xdeaddead under identity-joined Pi 5 evidence.
Live RP1 SYSINFO identity, broad RP1 clock/reset ownership, any new
clock/reset write, GPIO ownership, event generation, interrupt delivery,
handler ownership, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, and phase transition remain unaccepted.
Supervisor planning is required for the next Milestone 11.2 feature slice;
this closeout does not create a worker-owned follow-up task.

phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608 is
accepted as
phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1. It selects
one read-only BCM2712 PCIe2 host-link status discriminator before any endpoint
config access, PCIe writes, GPIO ownership retry, or interrupt-delivery work.
The accepted target is pcie2-host-link-status-read, limited to a 32-bit load
from PCIE_MISC_PCIE_STATUS at 0x1000124068. Source inspection ties this to
pcie2 controller base 0x10_0012_0000 and Broadcom STB PCIe status offset
0x4068; the retained RP1 SYSINFO/clock-window sentinel remains comparator
context only. A non-sentinel host status with DL_ACTIVE and PHYLINKUP set
would separate visible PCIe2 host/link state from the retained RP1-window
0xdeaddead boundary. A link-down or sentinel host status blocks endpoint/config
claims until a later supervisor-planned discriminator. Direct endpoint config
probing is rejected in this source contract because the retained driver uses
an EXT_CFG_INDEX write and warns that config access without link-up can abort.
The paired no-MMIO/no-RP1/no-GIC control must pass before any real Pi 5 proof.
This source contract does not accept runtime or hardware behavior, endpoint
config access, broad RP1 mapping, endpoint ownership, PCIe writes,
clock/reset ownership, GPIO ownership, event generation, interrupt delivery,
DMA/cache, networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-pcie-endpoint-config-discriminator-pi5-20260608 is accepted as
pcie2-host-link-up-rp1-window-sentinel. After an inconclusive first candidate
capture and known-good control triage, the accepted real rerun published only
the committed read-only PCIE_MISC_PCIE_STATUS candidate as tree
6d1fa1cd754adf38a023909651bcdc40b6ed08a06b559e79859f545886a59393, retained
two 46,880-byte da591740/kernel_2712.img TFTP fetches, retained 120 result
markers, passed the pi5-capture-transaction-v2 identity join with no rejection
reasons, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
read-only host status was raw=0x3e0b0 with pcie_port=true, dl_active=true,
phylinkup=true, link_in_l23=false, and status_is_deaddead=false. This accepts
only visible/link-up PCIe2 host status while the retained RP1
SYSINFO/clock-window sentinel remains comparator context. Endpoint config
access, broad RP1 mapping, endpoint ownership, PCIe writes, interrupt delivery,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-pcie-endpoint-config-discriminator-closeout-20260608 is accepted
as pcie2-host-link-up-rp1-window-sentinel-frontier-closed. The closeout
reconciles the accepted source contract, local/static core, no-MMIO/no-RP1/
no-GIC control proof, real Pi 5 proof, restore evidence, and retained risks.
The frontier is limited to visible/link-up BCM2712 PCIe2 host status while the
retained RP1 SYSINFO/clock-window path remains sentinel-shaped. Same-shaped
PCIe2 host-link status reruns are not progress without a different
discriminator or new acceptance criteria. Endpoint config-space access, broad
RP1 mapping, endpoint ownership, PCIe writes, bridge setup, PERST/link-control
changes, MSI/MIP/GIC operations, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, and phase transition remain
unaccepted. Supervisor planning is required for the next Milestone 11.2
feature slice; this closeout creates no worker-owned follow-up task.

phase11-rp1-endpoint-config-identity-source-contract-20260608 accepts the next
bounded source contract,
phase11-rp1-endpoint-config-identity-source-contract-v1. The selected target is
`rp1-endpoint-config-vendor-device-read`: after checking the accepted PCIe2
host-link status precondition, write controller selector `0x00100000` to
pcie2 `EXT_CFG_INDEX` at `0x1000129000`, then read one 32-bit config dword
from `EXT_CFG_DATA + 0` at `0x1000128000` for BDF 0002:01:00.0 offset 0. The
expected RP1 vendor/device identity is `0x1de4:0x0001`. This accepts only the
source contract and paired no-MMIO/no-RP1/no-GIC control requirement for an
endpoint config identity read; hardware behavior, broad RP1 mapping, endpoint
ownership, endpoint configuration mutation, BAR programming, bridge setup,
interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3, and phase
transition remain unaccepted.

phase11-rp1-endpoint-config-identity-pi5-20260608 is accepted as
rp1-endpoint-config-id-all-ones. The real Pi 5 proof passed the
pi5-capture-transaction-v2 identity join with selected tree
7e66c8cef268d7a94843c0d8e230f89c25161053f0b326a8375c0b6f4ca97d42, two
served 48,456-byte candidate kernel fetches, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Serial
retained 135 result markers showing PCIE_MISC_PCIE_STATUS raw=0x3e0b0 with
link up, the accepted EXT_CFG_INDEX selector write, and EXT_CFG_DATA + 0
returning 0xffffffff for BDF 0002:01:00.0 offset 0. This accepts only the
all-ones endpoint config identity frontier; expected RP1 vendor/device
visibility, endpoint ownership, broad RP1 mapping, endpoint configuration
mutation, bridge setup, interrupt delivery, DMA/cache, networking, SSH,
Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-endpoint-config-identity-closeout-20260608 closes that chain as
rp1-endpoint-config-id-all-ones-frontier-closed. The accepted frontier is
limited to the source-backed endpoint config identity attempt, paired
no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 all-ones config-data result
under identity-joined evidence. Same-shaped endpoint config identity hardware
reruns are not progress without a supervisor-planned different discriminator
or new acceptance criteria. Expected RP1 vendor/device visibility, endpoint
ownership, broad RP1 mapping, endpoint configuration mutation, BAR
programming or discovery, bridge setup, PERST/link-control, interrupt
delivery, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
and phase transition remain unaccepted. Supervisor planning is required for
the next Milestone 11.2 frontier.

phase11-rp1-bridge-config-preflight-source-contract-20260608 accepts
phase11-rp1-bridge-config-preflight-source-contract-v1. It selects one
read-only BCM2712 PCIe2 bridge/config preflight discriminator before any
endpoint config retry, BAR work, bridge setup, or RP1 ownership claim. The
accepted target is pcie2-bridge-misc-ctrl-preflight-read: read the accepted
host-link status register at 0x1000124068, then read PCIE_MISC_MISC_CTRL at
0x1000124008 and decode the source-defined SCB_ACCESS_EN, CFG_READ_UR_MODE,
RCB_MPS_MODE, RCB_64B_MODE, and max-burst fields. This source contract is
qualitatively different from the all-ones endpoint config identity attempt
because it performs no EXT_CFG_INDEX write and no EXT_CFG_DATA access. A
ready-shaped result would justify later supervisor planning only; it does not
accept endpoint ownership, broad RP1 mapping, bridge setup, BAR discovery or
programming, interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3,
or phase transition.

phase11-rp1-bridge-config-preflight-pi5-20260608 accepts the real Pi 5 result
as pcie2-bridge-preflight-ready. The decisive rerun passed
pi5-capture-transaction-v2 with selected tree
e66d21ac433225c19dfa63c09a577c8ab6828ebfdf5a437b57efc5fe0e7f260a, two served
48,000-byte candidate kernel fetches, 123 result markers, final selected-tree
identity, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The report
reached link-up with PCIE_MISC_PCIE_STATUS=0x3e0b0 and read
PCIE_MISC_MISC_CTRL=0xa8003000 with scb-access-en=true,
cfg-read-ur-mode=true, rcb-mps-mode=false, rcb-64b-mode=false,
max-burst-size=0x0, and misc-ctrl-is-sentinel=false. Initial candidate and
known-good runs were retained as capture-staging-blocked evidence because the
pre-power serial drain was non-empty at a saturated cursor; a bounded drain
plus known-good control passed before the accepted unchanged candidate rerun.
This accepts only the bridge/config preflight readiness boundary. Endpoint
ownership, expected RP1 vendor/device visibility, broad RP1 mapping, endpoint
configuration mutation, BAR discovery or programming, bridge setup,
PERST/link-control, interrupt delivery, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-bridge-config-preflight-closeout-20260608 accepts the closeout as
pcie2-bridge-preflight-ready-frontier-closed. The accepted frontier is limited
to the read-only source-backed bridge/config preflight discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5
pcie2-bridge-preflight-ready result under identity-joined evidence. It does
not accept expected RP1 vendor/device visibility, endpoint ownership, broad
RP1 mapping, endpoint configuration mutation, BAR discovery or programming,
bridge setup, PERST/link-control, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition. The
worker creates no follow-up task from this closeout; supervisor planning is
required for the next Milestone 11.2 frontier.

phase11-rp1-bridge-setup-source-contract-20260608 accepts the next bounded
source contract, phase11-rp1-bridge-setup-source-contract-v1. It selects a
read-only BCM2712 PCIe2 setup-state snapshot after the accepted
bridge/config-preflight ready result. The selected target is
pcie2-bridge-setup-state-read: read PCIE_MISC_PCIE_STATUS and
PCIE_MISC_MISC_CTRL as the accepted link/preflight predicates, then read
PCIE_RC_CFG_PRIV1_ID_VAL3 and outbound window 0 LO/HI, BASE_LIMIT, BASE_HI,
and LIMIT_HI. Retained Broadcom STB PCIe source writes class code 0x060400
through PCIE_RC_CFG_PRIV1_ID_VAL3 and programs outbound window 0 from the
host bridge windows; retained BCM2712/RP1 device-tree sources tie pcie2 to
the non-prefetchable PCIe 0 -> CPU 0x1f_0000_0000 window that carries the RP1
bus mapping.

This accepts only the source contract and paired no-MMIO/no-PCIe/no-RP1/
no-GIC control requirement for a read-only setup-state discriminator. It does
not accept runtime or hardware behavior, endpoint config retry, expected RP1
vendor/device visibility, endpoint ownership, broad RP1 mapping, BAR
discovery or programming, bridge setup writes, PERST/link-control, interrupt
delivery, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
or phase transition.

phase11-rp1-bridge-setup-pi5-20260608 accepts the real Pi 5
bridge/setup-state proof as pcie2-bridge-setup-state-incomplete. The accepted
rerun published only target/talos-rpi5-rp1-bridge-setup-state-read-core.tar.gz,
selected boot tree 9fbdcb57cd60519737902b9e3b85799e2479abffd8911a9ca887015a7f0f625a,
retained two 50,736-byte da591740/kernel_2712.img TFTP fetches, passed
capture-transaction-v2-ready identity join, retained 90 occurrences of
TALOS: rp1-bridge-setup-state-result, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Serial
hardware output showed link/preflight predicates true, PCIE_MISC_MISC_CTRL
0xa8003000, root-complex class code 0x060400, and visible outbound-window
registers, but the window did not match the source-expected PCIe 0 -> CPU
0x1f_0000_0000 shape: win0_lo=0x80000000, base_limit=0x3ff00000,
base_hi=0x1c, and limit_hi=0x1c. The first real run and first known-good
control were retained as capture-staging-blocked because the pre-power serial
drain was not empty; after a bounded manual drain, the known-good
production-timer control and unchanged real rerun passed identity join.

This accepts only the identity-joined incomplete bridge/setup-state hardware
classification and capture/restore evidence. It does not accept
pcie2-bridge-setup-state-visible, expected RP1 vendor/device visibility,
endpoint ownership, broad RP1 mapping, BAR discovery or programming, bridge
setup writes, PERST/link-control, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

phase11-rp1-bridge-setup-closeout-20260608 accepts the closeout as
pcie2-bridge-setup-state-incomplete-frontier-closed. The accepted frontier is
limited to the source-backed read-only bridge/setup-state discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5
pcie2-bridge-setup-state-incomplete result under identity-joined evidence. It
accepts visible link/preflight state, root-complex class code 0x060400, and
visible outbound window 0 registers, but the retained window values do not
match the source-expected PCIe 0 -> CPU 0x1f_0000_0000 shape.

This closeout does not accept pcie2-bridge-setup-state-visible, expected RP1
vendor/device visibility, endpoint ownership, broad RP1 mapping, BAR discovery
or programming, bridge setup writes, PERST/link-control, interrupt delivery,
GPIO/clock ownership, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition. Same-shaped endpoint config identity and
same-shaped bridge/setup-state hardware reruns remain closed unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
The worker creates no follow-up task from this closeout; supervisor planning is
required for the next Milestone 11.2 frontier.

phase11-rp1-observed-aperture-source-contract-20260608 accepts the next
bounded source/evidence contract,
phase11-rp1-observed-aperture-source-contract-v1. It answers the accepted
bridge/setup-state blocker by selecting a qualitatively different observed
aperture read instead of a same-shaped endpoint config, 0x1f RP1 peripheral,
or bridge/setup-state rerun. The selected target is
rp1-uart0-fr-observed-aperture-read: one 32-bit volatile read from the RP1
UART0 PL011 flag register at observed CPU physical 0x1c_0003_0018. Retained
rp1.dtsi source backs the UART0 PL011 block and FR offset; retained
first-light/decision evidence records 0x1c_0003_0000 as the
firmware-preserved RP1 UART0 mapping; the accepted bridge/setup proof
observed outbound-window CPU high fields of 0x1c while rejecting the
source-expected 0x1f visible setup-state claim.

The accepted classifications are observed-aperture-rp1-uart0-fr-visible,
observed-aperture-rp1-uart0-fr-sentinel,
observed-aperture-rp1-uart0-fr-all-ones,
observed-aperture-rp1-uart0-fr-zero,
observed-aperture-rp1-uart0-fr-no-return-or-trap,
observed-aperture-rp1-uart0-fr-inconclusive-capture,
no-mmio-observed-aperture-control-visible, and staging/build-blocker. The
paired control must preserve output shape while constructing no BCM2712 PCIe,
RP1, MIP, GIC, GPIO, clock/reset, DMA, or other MMIO address. This accepts
only the contract boundary; live RP1 ownership, endpoint ownership, broad RP1
mapping, UART ownership, interrupt delivery, GPIO/clock ownership, DMA/cache,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-observed-aperture-pi5-20260608 accepts the real Pi 5
observed-aperture proof as observed-aperture-rp1-uart0-fr-visible. After an
initial capture-staging-blocked candidate run and known-good-control triage,
the accepted unchanged rerun published only
target/talos-rpi5-rp1-observed-aperture-read-core.tar.gz, selected boot tree
def82f95b6ee4440de8014a275cbdef3b1baa4d578d9773e30ff7f15cd2d8a87, retained
two 47,664-byte da591740/kernel_2712.img TFTP fetches, passed the
pi5-capture-transaction-v2 identity join with no rejection reasons, retained
69 TALOS: rp1-observed-aperture-result records, and restored the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
visible result read observed CPU physical address 0x1c00030018 and reported
raw=0x187, raw-is-pl011-fr-shaped=true, and
classification=observed-aperture-rp1-uart0-fr-visible. This accepts only the
selected one-read observed aperture and its report shape. Endpoint ownership,
broad RP1 mapping, UART ownership, interrupt delivery, GPIO/clock ownership,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-observed-aperture-closeout-20260608 accepts the closeout as
observed-aperture-rp1-uart0-fr-visible-frontier-closed. The accepted frontier
is limited to the source/evidence-backed observed-aperture discriminator, the
paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof, and the real Pi 5 visible
result under identity-joined evidence. It accepts that one selected read from
observed CPU physical address 0x1c00030018 returned raw=0x187,
raw-is-pl011-fr-shaped=true, and not sentinel/all-ones/zero.

This closeout does not accept endpoint ownership, broad RP1 mapping, UART
ownership, interrupt delivery, GPIO/clock ownership, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.
Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1
peripheral, and 0x1c observed-aperture hardware reruns remain closed unless a
future supervisor task supplies a different discriminator or new acceptance
criteria. The worker creates no follow-up task from this closeout; supervisor
planning is required for the next Milestone 11.2 frontier.

phase11-rp1-observed-gpio-status-source-contract-20260608 accepts the next
bounded source/evidence contract,
phase11-rp1-observed-gpio-status-source-contract-v1. It selects a
qualitatively different observed-aperture GPIO14 preflight rather than a
same-shaped endpoint config, bridge/setup-state, 0x1f RP1 peripheral, 0x1f
GPIO/status, 0x1f GPIO bank source-status, or 0x1c UART0 FR hardware rerun.
The selected target is rp1-gpio14-status-ctrl-observed-aperture-read: two
read-only 32-bit volatile loads from IO_BANK0 GPIO14 STATUS and CTRL at
observed CPU physical addresses 0x1c_000d_0070 and 0x1c_000d_0074.

Retained RP1 Linux source backs the register identity: IO_BANK0 base
0xc0_400d_0000 plus GPIO14 offset 14 * 8 gives STATUS/CTRL at RP1 bus
0xc0_400d_0070/0xc0_400d_0074. The accepted source-expected 0x1f comparators
remain blocked for same-shaped reruns. IO_BANK0 INTE/INTS are not selected in
this contract because the immediate discriminator is per-pin observed-aperture
STATUS/CTRL visibility, not interrupt source-status or event delivery.

The accepted classifications are observed-aperture-gpio14-status-ctrl-visible,
observed-aperture-gpio14-status-ctrl-sentinel,
observed-aperture-gpio14-status-ctrl-all-ones,
observed-aperture-gpio14-status-ctrl-zero,
observed-aperture-gpio14-status-ctrl-no-return-or-trap,
observed-aperture-gpio14-status-ctrl-inconclusive-capture,
no-mmio-observed-gpio-status-control-visible, and staging/build-blocker. This
accepts only the source contract; GPIO ownership, event generation, interrupt
pending generation, interrupt delivery, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, networking, SSH, Milestone 11.3, and
phase transition remain unaccepted.

phase11-rp1-observed-gpio-status-control-pi5-retry-20260608 accepts the paired
no-MMIO/no-RP1/no-GIC control proof as no-mmio-observed-gpio-status-control-visible.
The first repaired-freshness candidate retry was retained as
capture-staging-blocked because TFTP/final identity showed the restored
known-good tree. The required known-good production-timer control passed the
repaired freshness and v2 identity join gate, and the unchanged candidate
rerun then selected tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab, retained an
empty pre-power serial drain, captured two 48,952-byte
da591740/kernel_2712.img TFTP fetches, retained 41 task-owned control markers,
and restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the paired no-MMIO control; the real observed GPIO14 STATUS/CTRL
read, GPIO ownership, event generation, interrupt pending generation,
interrupt delivery, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
ownership, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted.

phase11-rp1-observed-gpio-status-pi5-20260608 completed as
capture-staging-blocked. The real candidate selected tree
52b5f11000b24f6f6d00ab1b9aaa4d62a4d4114486a0302ad593b713a08c2559, retained
two 49,656-byte da591740/kernel_2712.img TFTP fetches, kept final
selected-tree identity, and restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Serial
output contained 42 task-owned result markers with marker-visible
gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
classification=observed-aperture-gpio14-status-ctrl-visible. That result is
not accepted because pi5-capture-transaction-v2 rejected the run for the
repaired serial freshness gate: 96 pre-power drain attempts read 1,095,168
bytes and never reached empty-read-before-power. The required known-good
production-timer control failed the same repaired freshness discriminator, so
the unchanged candidate was not rerun and same-shaped real GPIO14 STATUS/CTRL
proof remains blocked pending supervisor planning.

phase11-rp1-observed-gpio-status-closeout-20260608 accepts the chain as
observed-gpio-status-capture-blocked-frontier-closed. The accepted frontier is
limited to the source/evidence-backed GPIO14 STATUS/CTRL observed-aperture
contract, the local/static real/control core, the serial-drain freshness repair
procedure, the paired no-MMIO/no-RP1/no-GIC control proof, and the committed
real Pi 5 capture-staging blocker. It does not accept observed 0x1c GPIO14
STATUS/CTRL visibility, GPIO ownership, event generation, interrupt
pending/delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or phase transition. Same-shaped endpoint config
identity, bridge/setup-state, 0x1f RP1 peripheral, 0x1c UART0 FR, and real
0x1c GPIO14 STATUS/CTRL hardware reruns remain closed unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
The worker creates no follow-up task from this closeout; supervisor planning
is required for the next Milestone 11.2 frontier.

phase11-pi5-run-unique-capture-marker-core-20260608 accepts
pi5-capture-transaction-run-unique-v1 as a stronger capture freshness
discriminator after the constant-marker V3 retry policy rejected stale
same-shaped evidence. The diagnostic runtime can embed a task-owned
TALOS_CAPTURE_NONCE into observed GPIO status control/result markers, and the
checker requires the exact capture-nonce marker while preserving V3 selected
tree, TFTP, final identity, and restore checks. This is a capture contract
only and does not accept GPIO14 STATUS/CTRL visibility.

phase11-rp1-observed-gpio-status-run-unique-control-pi5-20260608 accepts the
paired no-MMIO/no-RP1/no-GIC run-unique control proof. The control selected
tree 2e0fbbdc8da0ec3066ddc4b74949887c8bcf80c70ac6c4a68edffb5dca6f5173,
retained empty-read-before-power, observed nonce
ru20260608T195401Z-f84941d7 after power, retained two 49,072-byte candidate
TFTP fetches, kept final identity on the selected tree, and restored the lab
to a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the control output/capture path; real GPIO14 STATUS/CTRL
visibility remains unaccepted.

phase11-rp1-observed-gpio-status-run-unique-pi5-20260608 completed as
capture-staging-blocked. The primary real candidate retained nonce-bearing
marker-visible GPIO14 STATUS/CTRL values, but TFTP/final identity matched the
baseline tree: observed fetches were 104,136 bytes while the selected
candidate expected 49,776-byte fetches. A clean same-shaped retry with a fresh
nonce also remained blocked after a 1,095,168-byte non-empty pre-power drain,
missing required marker after power, and baseline-sized TFTP fetches. The
marker-visible values remain retained evidence only.

phase11-rp1-observed-gpio-status-run-unique-closeout-20260608 accepts the
run-unique chain as observed-gpio-status-run-unique-capture-blocked-frontier-closed.
The accepted frontier is limited to the source/evidence-backed GPIO14
STATUS/CTRL observed-aperture contract, local/static core, serial-drain repair
procedure, run-unique capture marker contract, run-unique no-MMIO control, and
committed real Pi 5 capture-staging blocker. It does not accept GPIO14
STATUS/CTRL visibility, GPIO ownership, event generation, interrupt
pending/delivery, GIC acknowledgement, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or phase transition. No worker-owned follow-up task is
created; supervisor planning is required for the next Milestone 11.2 frontier.

phase11-pi5-boot-staging-identity-repair-core-20260608 accepts
pi5-boot-staging-identity-v1 as the next staging discriminator. It explains the
prior retained marker-visible GPIO14 STATUS/CTRL text as non-decisive because
TFTP/final identity matched the baseline tree, and it requires selected-tree
identity, expected TFTP fetch bytes, final pre-restore selected-tree identity,
and restore proof before serial output can support a hardware claim.

phase11-pi5-boot-staging-identity-known-good-control-pi5-20260608 accepts the
no-MMIO/no-RP1/no-GIC known-good control under that repaired procedure. The
control selected tree
35a30932a7f8e76d8cfa657b7419ec1d5e7e8ce450c5ae898c32e957636734f1, retained
two 49,072-byte candidate TFTP fetches, kept final pre-restore identity on the
selected tree, passed the run-unique and boot-staging identity checkers, and
restored the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This
accepts only the control output/capture path.

phase11-rp1-observed-gpio-status-after-staging-repair-pi5-20260608 accepts the
real Pi 5 read-only observed GPIO14 STATUS/CTRL visibility proof. The candidate
selected tree
5a499384497595de18d05f250fe146352d964953c9ff759642cc8d20384e0ea6, retained
two 49,784-byte candidate TFTP fetches, kept final pre-restore identity on the
selected tree, retained 38 task-owned result markers, reported
gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, passed the
run-unique and boot-staging identity checkers, and restored the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

phase11-rp1-observed-gpio-status-after-staging-repair-closeout-20260608 accepts
the chain as observed-gpio14-status-ctrl-visible-frontier-closed. The accepted
frontier is limited to the source/evidence-backed GPIO14 STATUS/CTRL
observed-aperture contract, local/static real/control core, serial-drain
repair, run-unique capture marker contract, boot-staging identity
discriminator, no-MMIO/no-RP1/no-GIC control proof, and real read-only observed
GPIO14 STATUS/CTRL visibility proof. It does not accept GPIO ownership, event
generation, interrupt pending/delivery, GIC acknowledgement, endpoint
ownership, broad RP1 mapping, pad/RIO/clock/reset ownership, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.
Same-shaped GPIO14 STATUS/CTRL hardware reruns are closed unless future
supervisor planning supplies a different discriminator or new acceptance
criteria. The worker creates no follow-up task from this closeout; supervisor
planning is required for the next Milestone 11.2 feature slice.

phase11-rp1-observed-gpio-ownership-route-source-contract-20260608 accepts the
next bounded source/evidence contract,
phase11-rp1-observed-gpio-ownership-route-source-contract-v1. It is a
qualitatively different observed-aperture ownership/route preflight after the
accepted GPIO14 STATUS/CTRL visibility proof, not a same-shaped STATUS/CTRL
rerun and not a retry of the prior source-expected 0x1f ownership blocker.
The selected target is
rp1-gpio14-ownership-route-observed-aperture-preflight-read.

Allowed read-only loads are GPIO14 STATUS/CTRL at
0x1c_000d_0070/0x1c_000d_0074, IO_BANK0 INTE/INTS at
0x1c_000d_011c/0x1c_000d_0124, RIO0 OUT/OE/IN at
0x1c_000e_0000/0x1c_000e_0004/0x1c_000e_0008, GPIO14 pad control at
0x1c_000f_003c, and the accepted read-only INTID 160 GIC route status
registers at 0x10_7fff_9114, 0x10_7fff_9214, 0x10_7fff_9314, and
0x10_7fff_a018. The report must decode GPIO14 function, bank
source-enable/source-status, RIO state, pad state, and parent route status.

The accepted classifications are observed-gpio14-ownership-route-preflight-visible,
observed-gpio14-ownership-preflight-blocked-non-gpio-function,
observed-gpio14-ownership-preflight-blocked-route-or-source-state,
observed-gpio14-ownership-preflight-sentinel,
observed-gpio14-ownership-preflight-all-ones,
observed-gpio14-ownership-preflight-zero,
observed-gpio14-ownership-preflight-no-return-or-trap,
observed-gpio14-ownership-preflight-inconclusive-capture,
no-mmio-observed-gpio14-ownership-route-control-visible, and
staging/build-blocker. This accepts only the read-only observed-aperture
source contract and paired control requirement. GPIO ownership, event
generation, interrupt pending/delivery, GIC acknowledgement, handler
ownership, GPIO/RIO/pad/INTE writes, parent-route masking writes, DMA/cache,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.

phase11-rp1-observed-gpio-ownership-route-core-20260608 implements that
contract as the local/static real candidate and paired no-MMIO/no-RP1/no-GIC
control. phase11-rp1-observed-gpio-ownership-route-control-pi5-20260608
accepts the paired control output path on Pi 5 with decisive V3/run-unique and
boot-staging identity evidence, two 48,528-byte candidate TFTP fetches, final
selected-tree identity, and restore proof; no GPIO/RP1/GIC/PCIe hardware
behavior is accepted from the control.

phase11-rp1-observed-gpio-ownership-route-pi5-20260608 accepts the real
read-only observed-aperture preflight visibility as
observed-gpio14-ownership-preflight-blocked-non-gpio-function. The accepted
Pi 5 run retained selected tree
e6ded87c576967c770223930463864fc081443467d6e00fbe108f29fa9e33fd2, two
50,496-byte da591740/kernel_2712.img TFTP fetches, final selected-tree
identity, V3 and boot-staging checker success, marker-visible output, and
restore to the baseline tree. The result reported GPIO14 FUNCSEL=4 / uart0,
IO_BANK0 INTE/INTS clear, INTID160 not enabled/pending/active, and HPPIR
spurious 1023.

phase11-rp1-observed-gpio-ownership-route-closeout-20260608 closes the chain
as observed-gpio14-ownership-route-preflight-non-gpio-blocker-frontier-closed.
The accepted frontier is limited to the source contract, local/static
implementation, control proof, and real read-only preflight classification
that GPIO14 is currently muxed to UART0. GPIO ownership, event generation,
interrupt pending generation, interrupt delivery, GIC acknowledgement, handler
ownership, broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes, DMA/cache,
networking, SSH, Milestone 11.3, and phase transition remain unaccepted.
Supervisor planning is required for the next Milestone 11.2 frontier before
any write-backed GPIO event or interrupt-delivery work.

phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609 accepts
the next bounded source/evidence contract,
phase11-rp1-observed-gpio16-ownership-event-source-contract-v1. It is a
read-only observed-aperture GPIO16 ownership/event preflight after GPIO14 was
classified as UART0, not a same-shaped GPIO14 rerun and not a retry of the
prior source-expected 0x1f GPIO16 event discriminator. The selected target is
rp1-gpio16-ownership-event-observed-aperture-preflight-read.

Allowed read-only loads are GPIO16 STATUS/CTRL at
0x1c_000d_0080/0x1c_000d_0084, IO_BANK0 INTE/INTS at
0x1c_000d_011c/0x1c_000d_0124, RIO0 OUT/OE/IN at
0x1c_000e_0000/0x1c_000e_0004/0x1c_000e_0008, GPIO16 pad control at
0x1c_000f_0044, and the accepted read-only INTID 160 GIC route status
registers at 0x10_7fff_9114, 0x10_7fff_9214, 0x10_7fff_9314, and
0x10_7fff_a018. The report must decode GPIO16 function, bank
source-enable/source-status, RIO state, pad state, and parent route status.

The accepted classifications are
observed-gpio16-ownership-event-preflight-visible,
observed-gpio16-ownership-preflight-blocked-non-gpio-function,
observed-gpio16-ownership-preflight-blocked-route-or-source-state,
observed-gpio16-ownership-preflight-sentinel,
observed-gpio16-ownership-preflight-all-ones,
observed-gpio16-ownership-preflight-zero,
observed-gpio16-ownership-preflight-no-return-or-trap,
observed-gpio16-ownership-preflight-inconclusive-capture,
no-mmio-observed-gpio16-ownership-event-control-visible, and
staging/build-blocker. This accepts only the read-only observed-aperture source
contract and paired control requirement. GPIO ownership, event generation,
interrupt pending/delivery, GIC acknowledgement, handler ownership,
GPIO/RIO/pad/INTE/CTRL writes, GPIO14 ownership changes, parent-route masking
writes, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted.

phase11-rp1-observed-gpio16-ownership-event-core-20260609 accepts the
local/static real/control implementation of that read-only observed-aperture
GPIO16 preflight. The real candidate reads only the accepted observed GPIO16
STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad, and read-only
INTID 160 GIC route status registers; it performs no GPIO/RIO/pad/INTE/CTRL
writes, no IRQRESET, no event generation, and no action/restore sequence. The
paired no-MMIO control preserves the report shape with not-constructed address
fields and constructs no RP1 or GIC MMIO address. This accepts only
local/static/archive evidence; Pi 5 control and real proofs remain queued
before any hardware behavior, GPIO ownership, or event/readiness claim.

phase11-pi5-run-unique-serial-visibility-discriminator-core-20260609 repairs
the local/static run-unique checker after the GPIO16 no-MMIO control blocker.
The blocker had decisive staging/TFTP/final/restore evidence, stale GPIO14
serial still visible, and no exact required-marker match, but the post-power
serial window did contain the task-owned `capture-nonce=` token while pre-power
drain output did not. The accepted discriminator keeps the selected-tree, TFTP,
final-identity, and restore gates, then treats the run-unique nonce token as the
current-run serial visibility proof. Stale-before-power, absent-after-power, and
staging-mismatch fixtures remain rejected. This accepts only the checker repair
and replay behavior; the next serialized GPIO16 no-MMIO control proof still
must run before any real GPIO16 Pi 5 preflight or hardware behavior claim.

phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry-20260609 accepts
the paired no-MMIO GPIO16 control output proof as visible on Pi 5 under the
repaired run-unique discriminator. The accepted run staged tree
cdb35bef8b7fbd5b68df9c76a58fbb410e20522d46aed6b77319002b0be6bd19, retained
two 48,744-byte da591740/kernel_2712.img fetches, proved the task-owned nonce
absent before power and present after power, passed boot-staging identity, and
restored to the baseline tree. No GPIO/RP1/GIC/PCIe hardware behavior is
accepted from this control.

phase11-rp1-observed-gpio16-ownership-event-pi5-20260609 accepts the real
read-only GPIO16 ownership/event preflight as
observed-gpio16-ownership-preflight-blocked-non-gpio-function. The accepted
run staged tree
908eadd18fab1ba826d2dba92125649383a4857ed39ea18af125feb721a637c3, retained
two 50,640-byte da591740/kernel_2712.img fetches, passed V3 and boot-staging
identity checks, retained marker-visible output, and restored to the baseline
tree. The result reported GPIO16 FUNCSEL=31 / unknown, IO_BANK0 INTE/INTS
clear for GPIO16, RIO GPIO16 OUT/OE/IN false, pad input disabled, pad output
disabled, INTID160 not enabled/pending/active, and HPPIR spurious 1023. This
accepts only selected read-only GPIO16 preflight visibility/classification, not
GPIO ownership, event generation, interrupt delivery, or a phase transition.

phase11-rp1-observed-gpio16-ownership-event-closeout-20260609 closes this
chain as
observed-gpio16-ownership-event-preflight-non-gpio-blocker-frontier-closed.
The accepted frontier is limited to the source contract, local/static
implementation, repaired no-MMIO control proof, and real read-only GPIO16
non-GPIO-function blocker classification. Same-shaped GPIO16 preflight reruns
are not progress without new acceptance criteria or a new discriminator. GPIO
function changes, write-backed event setup, interrupt-delivery work, broad RP1
ownership, DMA/cache, networking, SSH, Milestone 11.3, and phase transition
remain unaccepted. Supervisor planning is required for the next Milestone 11.2
feature slice.

phase11-rp1-clock-reset-dependency-source-contract-20260609 accepts the next
source-only Milestone 11.2 contract,
phase11-rp1-clock-reset-dependency-source-contract-v1. It selects a read-only
observed-aperture identity and clock-manager dependency preflight before any
Talos-owned RP1 GPIO, event, or driver setup work. Allowed loads are
SYSINFO_CHIP_ID/SYSINFO_PLATFORM at 0x1c00000000/0x1c00000004, PLL_SYS_CS at
0x1c00020000, CLK_SYS_CTRL/DIV_INT/SEL at
0x1c00018014/0x1c00018018/0x1c00018020, CLK_SLOW_SYS_CTRL at 0x1c00018024, and
CLK_UART_CTRL/DIV_INT/SEL at 0x1c00018054/0x1c00018058/0x1c00018060. The
contract uses the accepted observed 0x1c RP1 aperture and retains the 0x1f
SYSINFO/clock sentinel closeout as comparator context only. No reset-controller
read is selected because retained Linux source exposes reset_control_reset, not
a bounded safe read-only reset-status register. This accepts only a source
contract and paired no-MMIO control requirement; live RP1 identity, clock/reset
ownership, clock/reset writes, GPIO function changes, event generation,
interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3, and phase
transition remain unaccepted. The local/static core task is mechanically
unblocked after the committed source contract.

phase11-rp1-clock-reset-dependency-core-20260609 implements that contract as
local/static real and paired no-MMIO control candidates. The real candidate
performs only the accepted observed-aperture SYSINFO and clock-manager loads
and emits the stable dependency report; the control preserves the report shape
while constructing no RP1, GPIO, clock/reset, PCIe/MIP, GIC, DMA, or other
forbidden MMIO address.

phase11-rp1-clock-reset-dependency-control-pi5-20260609 accepts the paired
no-MMIO control output path as no-mmio-clock-reset-dependency-control-visible.
The proof retained selected tree
3f48e70435914a0ca3deb160c517a32205643c3fbd9547d407387895ae417aba, two
48,640-byte da591740/kernel_2712.img fetches, nonce-visible serial output,
boot-staging checker success, final selected-tree identity, and restore to the
baseline tree. This accepts only the control output/capture path.

phase11-rp1-clock-reset-dependency-pi5-20260609 accepts the real read-only
preflight as observed-clock-reset-dependency-blocked-system-clock-disabled.
The proof retained selected tree
ef7b62b81d097a52bda724d2173c982fa512e2b6541541514abebd6d8db1422f, two
49,496-byte da591740/kernel_2712.img fetches, V3 and boot-staging identity
checker success, marker-visible output, and restore to the baseline tree. The
result reported expected chip identity, PLL_SYS locked, CLK_UART enabled, no
selected clock sentinel, and CLK_SYS/CLK_SLOW_SYS enable bits false.

phase11-rp1-clock-reset-dependency-closeout-20260609 closes that chain as
clock-reset-dependency-preflight-system-clock-blocker-frontier-closed. The
accepted frontier is limited to the source-backed read-only SYSINFO and
clock-manager dependency contract, local/static implementation, no-MMIO
control proof, and real Pi 5 system-clock-disabled blocker classification.
Clock/reset ownership, reset-controller ownership, clock/reset writes, GPIO
ownership, GPIO function changes, event generation, interrupt delivery,
DMA/cache, networking, SSH, Milestone 11.3, and phase transition remain
unaccepted. Same-shaped dependency preflight reruns are not progress without
new acceptance criteria or a new discriminator. Supervisor planning is
required for the next Milestone 11.2 feature slice.

phase11-rp1-irq-clock-gpio-milestone-closeout-20260609 accepts the Milestone
11.2 checkpoint as
rp1-irq-clock-gpio-milestone-112-blocker-checkpoint-accepted. The accepted
checkpoint reconciles source-backed interrupt routing, GIC-visible INTID 160
read-only status, GPIO bank source-status, GPIO14 UART0 function blocker,
GPIO16 FUNCSEL=31 / unknown blocker, and observed SYSINFO/clock-manager
system-clock-disabled blocker evidence. The Milestone 11.2 acceptance
condition is satisfied by captured blockers with serial/lab/TFTP/restore
evidence, not by a working write-backed GPIO/status-LED diagnostic or
interrupt-delivery proof. GPIO ownership, GPIO/RIO/pad/INTE/CTRL writes,
event generation, interrupt pending generation, interrupt delivery,
IAR/EOIR acknowledgement, handler ownership, clock/reset ownership,
clock/reset writes, DMA/cache behavior, networking, SSH, Milestone 11.3
behavior, and phase transition remain unaccepted. Same-shaped GPIO14, GPIO16,
GIC route-status, GPIO bank source-status, and SYSINFO/clock-manager
dependency reruns are not progress without new supervisor-planned
discriminators or acceptance criteria. The mechanically unblocked next task is
the queued DMA/cache source inventory, which is source inspection only.

phase11-rp1-dma-cache-source-inventory-20260609 accepts a source/static
Milestone 11.3 inventory as rp1-dma-cache-source-inventory-accepted. The
accepted frontier is limited to retained Raspberry Pi Linux dma-ranges, RP1 DMA
controller identity, selected iommu5 attachment inventory, and Talos DMA/cache
ownership/API gaps. It records that rp1_dma is a Synopsys AXI DMA controller
with 8 channels and 64 targets, that RP1 inbound RAM and peripheral
addressability depends on source dma-ranges, that existing Talos cache-line
helpers are only secondary-core publication utilities, and that the low-tail
allocator span is not a DMA-safe buffer contract. DMA behavior, DMA engine
programming, IOMMU policy, cache-coherent driver policy, high-memory
allocation, Ethernet, storage, networking, SSH, Milestone 12 work, hardware
validation, and Milestone 11.3 acceptance by implication remain unaccepted. The
mechanically unblocked next task is the queued local/static DMA/cache substrate
contract.

phase11-rp1-dma-cache-contract-20260609 accepts the local/static
phase11-rp1-dma-cache-substrate-contract-v1 as
rp1-dma-cache-substrate-contract-accepted. The accepted frontier is limited to
ownership boundaries, pure API/evidence fields, and validator requirements for
DMA buffer descriptors, RP1 dma-ranges-derived address translation,
direction-specific cache-maintenance semantics, and explicit IOMMU
classification. Existing SMP cache helpers remain source evidence only and are
not accepted as a driver DMA API. Working DMA behavior, DMA engine programming,
descriptor rings, executed cache maintenance for driver buffers,
cache-coherent/non-cacheable/IOMMU-backed driver policy, DMA-safe allocation or
pinning, RP1 Ethernet readiness, storage readiness, networking, SSH, hardware
validation, Milestone 12 work, and Milestone 11.3 completion by implication
remain unaccepted. The contract makes a local/static DMA/cache substrate core
mechanically objective, but no worker-owned follow-up task exists yet;
supervisor planning is required before implementation continues.

phase11-rp1-dma-cache-substrate-core-20260609 accepts the bounded
local/static substrate core as
rp1-dma-cache-substrate-core-local-static-accepted. The accepted frontier is
limited to descriptor/cache/address/IOMMU vocabulary, RP1 RAM/peripheral
translation helpers, pure validators for the accepted low-tail owned span, and
evidence fields for the contract/source ids, CPU/RP1 addresses, alignment,
direction, cacheability, IOMMU classification, and validation results. Focused
tests prove one valid RP1 RAM-window descriptor plus rejected alignment,
ownership-span, high-memory, reserved-memory, translation, cacheability, and
IOMMU inputs. Working DMA behavior, DMA engine programming, descriptor rings,
executed cache maintenance for driver buffers, cache-coherent/non-cacheable or
IOMMU-backed driver policy, DMA-safe allocation beyond descriptor validation,
RP1 Ethernet readiness, storage readiness, networking, SSH, hardware
validation, Milestone 12 work, and Milestone 11.3 completion by implication
remain unaccepted. The mechanically unblocked next task is the queued
DMA/cache substrate closeout checkpoint.

phase11-rp1-dma-cache-substrate-closeout-20260609 accepts the local/static
DMA/cache substrate checkpoint as
rp1-dma-cache-substrate-local-static-frontier-closed. The accepted frontier is
limited to the source inventory, contract, local/static descriptor vocabulary,
RP1 dma-ranges-derived translation helpers, pure validators, evidence fields,
and focused unit tests already accepted by the substrate core. This does not
accept working DMA, descriptor rings, executed cache maintenance,
cache-coherent/non-cacheable/IOMMU-backed driver policy, DMA-safe allocation or
pinning beyond descriptor validation, RP1 Ethernet readiness, storage
readiness, networking, SSH, hardware validation, Milestone 12 work, or
Milestone 11.3 completion by implication. CPU-visible address alias/equality
policy remains evidence-only before any future driver consumes non-identity or
high-memory buffers. The mechanically unblocked next task is the queued
driver-adjacent DMA/cache source contract, which remains source-contract work
only.

phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609 accepts
phase11-rp1-dma-cache-sync-plan-contract-v1 as
rp1-dma-cache-driver-adjacent-source-contract-accepted. The accepted frontier
is a local/static cache synchronization plan derived only from an accepted
DmaBufferDescriptor, with operation selection tied to descriptor direction and
CPU/device ownership boundary, source-backed 64-byte cache-line coverage,
planned CPU range, CPU/RP1 addresses, cacheability, owner transition, IOMMU
classification, and rejected runtime claims. Existing SMP cache helpers remain
instruction-shape evidence only. Executed cache maintenance for driver buffers,
live barrier ordering, working DMA behavior, descriptor rings, RP1 MMIO, DMA
channel programming, cache-coherent/non-cacheable/IOMMU-backed driver policy,
DMA-safe allocation beyond descriptor validation, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, and Milestone 11.3
completion by implication remain unaccepted. No Pi 5 hardware proof is
mechanically required by this source contract. The next objective boundary is a
local/static cache-sync-plan core, but no explicit worker-owned task exists yet;
supervisor planning is required before implementation continues.

phase11-rp1-dma-cache-sync-plan-core-20260609 accepts the bounded
local/static sync-plan core as
rp1-dma-cache-sync-plan-core-local-static-accepted. The accepted frontier is
limited to DmaCacheSyncBoundary, DmaCacheSyncOperation, DmaCacheSyncPlan,
evidence formatting, accepted descriptor-evidence validation, deterministic
rejection cases, and source-backed 64-byte cache-line coverage. Focused tests
prove valid ToDevice/before, FromDevice/after, and Bidirectional/shared plans
plus rejected overflow, unsupported cacheability/IOMMU, unsupported
direction/boundary, zero-length, non-accepted classification, and evidence
mismatch inputs. Executed cache maintenance for driver buffers, live barrier
ordering, working DMA behavior, RP1 MMIO writes, DMA channel programming,
descriptor rings, interrupt completion, Ethernet, storage, networking, SSH,
hardware validation, Milestone 12 work, and Milestone 11.3 completion by
implication remain unaccepted. The mechanically unblocked next task is the
queued sync-plan closeout checkpoint.

phase11-rp1-dma-cache-sync-plan-closeout-20260609 accepts the sync-plan
checkpoint as rp1-dma-cache-sync-plan-local-static-frontier-closed. This
reconciles the source contract, implementation, tests, evidence, docs, and
retained risks into one accepted local/static frontier: sync-plan vocabulary,
accepted descriptor-evidence derivation, direction/boundary operation
selection, source-backed 64-byte cache-line coverage, deterministic rejection
cases, and focused unit-test evidence. Executed cache maintenance for driver
buffers, live barrier ordering, working DMA behavior, DMA/RP1 MMIO
programming, descriptor rings, coherent/non-cacheable/IOMMU-backed policy,
DMA-safe allocation beyond descriptor validation, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, and Milestone 11.3
completion by implication remain unaccepted. The mechanically unblocked next
task is the queued driver-adjacent diagnostic/source-contract boundary.

phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609 accepts the
next driver-adjacent source contract as
rp1-dma-cache-driver-adjacent-diagnostic-contract-accepted. The accepted
frontier is phase11-rp1-dma-cache-maintenance-sequence-contract-v1: a
local/static instruction/barrier sequence derived only from accepted
DmaCacheSyncPlanEvidence. The contract names static clean, invalidate, and
clean+invalidate cache-line operation vocabulary, a source-backed dsb sy
barrier shape, 64-byte line coverage, descriptor and sync-plan identity,
rejected runtime claims, and local/static classification. Existing SMP cache
helpers remain instruction/barrier-shape evidence only. Executed cache
maintenance for driver buffers, live barrier ordering, working DMA behavior,
RP1 MMIO writes, DMA channel programming, descriptor rings, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, and Milestone 11.3
completion by implication remain unaccepted.

phase11-rp1-dma-cache-maintenance-sequence-core-20260609 accepts the
local/static maintenance-sequence core as
rp1-dma-cache-maintenance-sequence-core-local-static-accepted. The accepted
frontier is the pure derivation of static clean, invalidate, and
clean+invalidate instruction vocabulary plus a source-backed dsb sy barrier
shape from accepted DmaCacheSyncPlanEvidence. The evidence preserves
descriptor and sync-plan identity, 64-byte line coverage, line count,
CPU/RP1 addresses, direction, cacheability, owner transition, IOMMU
classification, rejected runtime claims, and local/static classification.
The validator rejects non-accepted sync-plan classification,
descriptor/sync-plan mismatches, zero covered length, cache-line mismatch,
range overflow, and unsupported runtime claims. Executed cache maintenance for
driver buffers, live barrier ordering, working DMA behavior, RP1 MMIO writes,
DMA channel programming, descriptor rings, Ethernet, storage, networking, SSH,
hardware validation, Milestone 12 work, and Milestone 11.3 completion by
implication remain unaccepted. The next queued boundary is the
maintenance-sequence closeout checkpoint.

phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609 accepts the
maintenance-sequence checkpoint as
rp1-dma-cache-maintenance-sequence-local-static-frontier-closed. This
reconciles the source contract, implementation, tests, evidence, docs, and
retained risks into one accepted local/static frontier: static clean,
invalidate, clean+invalidate, and dsb sy vocabulary derived only from accepted
DmaCacheSyncPlanEvidence, preserving descriptor/sync-plan identity, line
coverage, rejected runtime claims, and local/static classification. Same-shaped
local/static sequence retries are closed unless a later supervisor task
supplies materially different runtime/execution scope, source evidence, or
acceptance criteria. Executed cache maintenance for driver buffers, live
barrier ordering, working DMA behavior, RP1 MMIO writes, DMA channel
programming, descriptor rings, coherent/non-cacheable/IOMMU-backed policy,
DMA-safe allocation beyond descriptor validation, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, and Milestone 11.3
completion by implication remain unaccepted. The mechanically unblocked next
task is the queued runtime/execution-contract boundary.

phase11-rp1-dma-cache-runtime-execution-contract-20260609 accepts the next
contract boundary as rp1-dma-cache-runtime-execution-contract-accepted. The
selected future boundary is
phase11-rp1-dma-cache-maintenance-executor-contract-v1: an architecture-gated
runtime executor contract that may consume only accepted
DmaCacheMaintenanceSequenceEvidence after validating the accepted descriptor,
sync-plan, and maintenance-sequence evidence chain. This is the smallest useful
runtime cache-maintenance boundary before driver DMA evaluation, but it remains
contract-only. Executed cache maintenance for driver buffers, live barrier
ordering, working DMA behavior, RP1 MMIO writes, DMA channel programming,
descriptor rings, interrupt completion, coherent/non-cacheable/IOMMU-backed
policy, DMA-safe allocation beyond descriptor validation, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, and Milestone 11.3
completion by implication remain unaccepted.

phase11-rp1-dma-cache-maintenance-executor-core-20260609 accepts the bounded
architecture-gated maintenance-executor core as
rp1-dma-cache-maintenance-executor-core-accepted. src/dma_cache.rs now exposes
phase11-rp1-dma-cache-maintenance-executor-contract-v1, validates accepted
DmaCacheMaintenanceSequenceEvidence identity before dispatch, maps the accepted
operation vocabulary to dc cvac, dc ivac, dc civac, and a final dsb sy
boundary, and returns runtime-execution evidence while preserving prerequisite
descriptor, sync-plan, sequence, line-coverage, CPU/RP1 address, cacheability,
owner-transition, IOMMU, and rejected-claims fields. Focused unit tests cover
the accepted clean/invalidate/clean+invalidate vocabularies and rejected
contract/classification bypass, cacheability/IOMMU mismatch, missing
rejected-claims identity, zero/mismatched line coverage, overflow, and
unsupported operation/instruction/barrier inputs. Driver DMA completion, RP1
MMIO writes, DMA channel programming, descriptor rings, interrupt completion,
Ethernet, storage, networking, SSH, hardware validation, Milestone 12 work, and
Milestone 11.3 completion by implication remain unaccepted. The next queued
boundary is the maintenance-executor closeout checkpoint.

phase11-rp1-dma-cache-maintenance-executor-closeout-20260609 accepts the
executor checkpoint as rp1-dma-cache-maintenance-executor-frontier-closed. The
accepted frontier remains limited to the descriptor, sync-plan,
maintenance-sequence, and architecture-gated executor evidence chain for
cacheable low-tail buffers with source-unassigned RP1 DMA/IOMMU
classification. The Milestone 11.3 documented DMA buffer ownership and
cache-maintenance rules requirement is partially satisfied by that chain, but
Milestone 11.3 is not complete: no small DMA or driver-adjacent diagnostic,
driver DMA completion, RP1 MMIO/DMA programming, descriptor rings, interrupt
completion, Ethernet, storage, networking, SSH, hardware validation, or
Milestone 12 work is accepted. Same-shaped maintenance-executor retries are
closed unless a future supervisor task supplies materially different
driver-adjacent runtime scope, source evidence, hardware evidence, or
acceptance criteria. The next mechanically objective boundary is the queued
driver-adjacent runtime/source-contract task.

phase11-rp1-dma-cache-driver-adjacent-runtime-contract-20260609 accepts the
next driver-adjacent contract as
rp1-dma-cache-driver-adjacent-runtime-contract-accepted. The selected boundary
is phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1: a local/static
diagnostic envelope that may consume only accepted maintenance-executor
evidence, preserve the descriptor/sync-plan/maintenance-sequence/executor
identity chain, and keep the remaining RP1 DMA channel, descriptor-ring,
interrupt-completion, IOMMU/runtime-policy, allocation/pinning, hardware-proof,
and device-consumer gaps explicit. This is not a small DMA diagnostic
implementation and does not accept driver DMA completion, RP1 MMIO writes, DMA
channel programming, descriptor rings, interrupt completion, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, Milestone 11.3
completion, or a phase transition. The next bounded implementation requires
supervisor planning.

phase11-rp1-dma-cache-driver-diagnostic-envelope-core-20260609 accepts that
bounded local/static envelope core as
rp1-dma-cache-driver-diagnostic-envelope-core-accepted. src/dma_cache.rs now
exposes DmaCacheDriverDiagnosticEnvelope input and evidence vocabulary that
consumes only accepted DmaCacheMaintenanceExecutorEvidence, preserves the
descriptor, sync-plan, maintenance-sequence, and executor evidence identities,
and records unresolved RP1 DMA channel ownership, descriptor-ring
layout/ownership, transfer-completion/interrupt policy, IOMMU/runtime policy,
DMA-safe allocation/pinning, hardware-proof, and device-consumer gaps.
Validators reject non-accepted executor classification, missing prerequisite
ids, invalid line coverage or overflow, unsupported cacheability/IOMMU claims,
missing rejected-runtime-claim identity, and any driver DMA or hardware/device
completion claim. This remains source/local evidence only: no Pi 5 hardware,
RP1 MMIO or DMA programming, descriptor rings, interrupt completion, Ethernet,
storage, networking, SSH, hardware validation, Milestone 12 work, Milestone
11.3 completion, or phase transition is accepted.

phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout-20260609 accepts the
envelope checkpoint as
rp1-dma-cache-driver-diagnostic-envelope-frontier-closed. Same-shaped
local/static envelope retries are closed unless a future supervisor task
supplies materially different runtime or hardware evidence, source scope, or
acceptance criteria. The accepted envelope is sufficient to plan the guarded
small DMA diagnostic source-contract boundary, but this does not accept a small
DMA diagnostic implementation, driver DMA completion, hardware/device
completion, RP1 MMIO/DMA programming, descriptor rings, interrupt completion,
Ethernet, storage, networking, SSH, hardware validation, Milestone 12 work,
Milestone 11.3 completion, or a phase transition. The next boundary remains
source-contract work.

phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609 accepts the
next guarded source contract as
rp1-dma-cache-small-diagnostic-source-contract-accepted. The selected future
boundary is phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1: a
local/static diagnostic plan that may consume only accepted
DmaCacheDriverDiagnosticEnvelopeEvidence plus retained RP1 DMA controller source
facts. It asks whether Talos can connect the accepted cache-maintained low-tail
buffer envelope to the source-backed rp1_dma controller identity without
claiming channel programming, descriptor-ring readiness, transfer completion,
interrupt completion, hardware/device completion, Ethernet, storage,
networking, SSH, hardware validation, Milestone 12 work, Milestone 11.3
completion, or a phase transition. That source-contract task required
supervisor planning before any plan-core implementation could continue.

phase11-rp1-dma-cache-small-diagnostic-plan-core-20260609 accepts the bounded
local/static plan core as
rp1-dma-cache-small-diagnostic-plan-core-local-static-accepted. src/dma_cache.rs
now constructs and formats RP1 DMA small diagnostic plan evidence from accepted
DmaCacheDriverDiagnosticEnvelopeEvidence plus retained rp1_dma controller
source facts. The evidence preserves the source contract id, envelope contract
id, descriptor/sync-plan/maintenance-sequence/executor identities, RP1 DMA
compatible string, RP1 bus base, translated CPU physical base, channel count,
target count, interrupt and clock names, CPU/RP1 buffer addresses, descriptor
length, cache line coverage, direction, cacheability, owner transition, IOMMU
classification, rejected runtime claims, unresolved diagnostic gaps, and
local/static classification. Validators reject invalid envelope/source inputs
and premature runtime/device readiness claims. No Pi 5 hardware validation,
RP1 MMIO/DMA programming, DMA channel ownership, descriptor rings, transfer
completion, interrupt completion, Ethernet, storage, networking, SSH,
Milestone 12 work, Milestone 11.3 completion, or phase transition is accepted.

phase11-rp1-dma-cache-small-diagnostic-plan-closeout-20260609 closes the
local/static plan frontier as
rp1-dma-cache-small-diagnostic-plan-frontier-closed. Same-shaped local/static
small diagnostic plan retries are closed unless a future supervisor task
supplies materially different source evidence, runtime evidence, hardware
evidence, or acceptance criteria. The next bounded boundary is the guarded
hardware-proof source-contract task, which is contract-only and must not run
hardware, publish boot archives, acquire hardwareTestLock, program RP1 MMIO or
DMA channels, create descriptor rings, claim transfer or interrupt completion,
implement Ethernet, storage, networking, SSH, accept Milestone 11.3 completion,
or create a phase transition.

phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract-20260609 accepts
the guarded future proof boundary only as plan visibility/control output. Any
future Pi 5 proof must be separately supervisor-planned, serialized through
hardwareTestLock, and limited to showing that accepted local/static small
diagnostic plan evidence is visible and distinguishable from a paired no-plan
control output on the real target. It must capture candidate identity,
artifact digest, fresh serial cursor, TFTP delta, paired control, restore
evidence, and inconclusive-run triage requirements. This does not accept live
DMA, RP1 MMIO writes, channel ownership, descriptor rings, transfer
completion, interrupt completion, hardware/device completion, Ethernet,
storage, networking, SSH, Milestone 11.3 completion, or a phase transition.

phase11-rp1-dma-cache-small-diagnostic-visibility-core-20260609 and
phase11-rp1-dma-cache-small-diagnostic-visibility-closeout-20260609 close the
local/static visibility-report frontier as
rp1-dma-cache-small-diagnostic-visibility-frontier-closed. The accepted code
constructs candidate visibility evidence from accepted plan evidence and a
paired no-plan control through the same report contract/source path while
withholding accepted plan fields. Same-shaped local/static visibility retries
are closed unless future scope supplies materially different source, runtime,
hardware, or acceptance evidence. The next selected boundary is the guarded
serialized Pi 5 plan visibility/control proof; it must acquire
hardwareTestLock before staging and still must not accept live DMA, RP1 MMIO
writes, channel ownership, descriptor rings, transfer completion, interrupt
completion, hardware/device completion, Ethernet, storage, networking, SSH,
Milestone 11.3 completion, or a phase transition.

phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609 and
phase11-rp1-dma-cache-small-diagnostic-visibility-proof-closeout-20260609 close
the first Pi 5 visibility/control proof as
rp1-dma-cache-small-diagnostic-visibility-proof-control-freshness-blocked. The
candidate rerun accepted only serial-visible plan report output with clean
identity, stable matching TFTP evidence, and restore proof. The paired no-plan
control and known-good control remained marker-visible but were rejected by
serial-drain-not-empty-before-power, so no full paired control proof, live DMA,
RP1 MMIO write, channel ownership, descriptor ring, transfer completion,
interrupt completion, hardware/device completion, Ethernet/storage, networking,
SSH, Milestone 11.3 completion, or phase transition is accepted. Same-shaped
hardware visibility/control retries are closed unless future scope changes the
freshness discriminator or acceptance criteria.

phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry-20260609 accepts
the paired Pi 5 visibility/control proof using the repaired run-unique
freshness discriminator. The no-plan control retained selected tree
3813289a7df48f04313329b90073683fb07eb0188b719290a862af587f86739b, 48,704-byte
da591740/kernel_2712.img fetches, nonce absence before power and presence
after power, boot-staging checker success, and restore to the baseline tree.
The candidate retained selected tree
f0b229ab1da582050f68af75b1de9953e9010b2cc4443ff3ee0002789e7572b2, 49,968-byte
da591740/kernel_2712.img fetches, distinct nonce absence before power and
presence after power, boot-staging checker success, and restore to the
baseline tree. This accepts only candidate/control visibility of the accepted
local/static small diagnostic plan report path. Live DMA, RP1 MMIO writes,
channel ownership, descriptor rings, transfer completion, interrupt
completion, hardware/device completion, Ethernet/storage, networking, SSH,
Milestone 11.3 completion, and phase transition remain unaccepted.

phase11-rp1-dma-cache-small-diagnostic-visibility-v3-closeout-20260609 closes
the run-unique visibility/control hardware proof frontier as
rp1-dma-cache-small-diagnostic-visibility-v3-frontier-closed. Same-shaped
visibility/control hardware retries are closed unless future supervisor scope
supplies materially different source evidence, runtime evidence, hardware
evidence, freshness requirements, or acceptance criteria. The selected next
checkpoint is
phase11-rp1-dma-cache-milestone-11-3-closeout-20260609, limited to accepted
Milestone 11.3 evidence and retained risks. Live DMA, RP1 MMIO writes, channel
ownership, descriptor rings, transfer completion, interrupt completion,
hardware/device completion, Ethernet/storage, networking, SSH, Milestone 12
work, and phase transition remain unaccepted.

phase11-rp1-dma-cache-milestone-11-3-closeout-20260609 accepts the Milestone
11.3 DMA/IOMMU/cache-maintenance checkpoint as
rp1-dma-cache-milestone-11-3-frontier-closed. The documented DMA buffer
ownership and cache-maintenance rule criterion is satisfied by the accepted
source inventory, substrate contract/core, cache-sync plan, static maintenance
sequence, and architecture-gated executor chain in src/dma_cache.rs. The small
DMA or driver-adjacent diagnostic criterion is satisfied only at the accepted
driver-adjacent/local-static and Pi 5 visibility/control levels: the diagnostic
envelope and local/static small diagnostic plan exist before networking depends
on DMA, and the run-unique Pi 5 proof shows candidate/control visibility of the
accepted report path. This checkpoint does not accept live DMA, RP1 MMIO
writes, channel ownership, descriptor-ring construction or ownership, transfer
completion, interrupt completion, hardware/device completion,
Ethernet/storage readiness, networking, SSH, Milestone 12 work, Phase 12
planning, or a phase transition. The explicit follow-up is the Phase 11
hardware-substrate closeout checkpoint before any Phase 12 research,
descriptor-ring/channel-ownership, live DMA, networking, or later work starts.

phase11-rp1-hardware-substrate-closeout-20260609 accepts Phase 11 as
rp1-hardware-substrate-phase11-frontier-closed. The closed frontier is limited
to substrate/research facts: RP1/PCIe address/path and host-link visibility
with retained endpoint/bridge blockers, the accepted Milestone 11.2
interrupt/clock/GPIO blocker checkpoint, and the accepted Milestone 11.3
DMA/cache documented/local-static/visibility frontier. The checkpoint selects
phase12-rp1-ethernet-source-inventory-20260609 as the exact next task and
allows only source-only Phase 12.1 Ethernet research. Live DMA, RP1 MMIO/DMA
programming, descriptor-ring construction or ownership, channel ownership,
transfer completion, interrupt completion, clock/reset ownership,
GPIO/event ownership, Ethernet/storage readiness, packet I/O, networking,
sockets, SSH, and Phase 12 implementation remain unaccepted.

Milestone 11.2: RP1 Interrupts, Clocks, and GPIO

- Trace RP1 interrupt delivery into the BCM2712/GIC path.
- Identify clock/reset dependencies needed before Talos-owned RP1 drivers.
- Add a narrow GPIO or status-LED diagnostic only after mapping and interrupt
  assumptions are understood.

Acceptance criteria:

- RP1 interrupt routing is documented with source references.
- A minimal RP1 diagnostic works or the blocker is captured with serial
  evidence.

Milestone 11.3: DMA, IOMMU, and Cache Maintenance

Status: accepted checkpoint
phase11-rp1-dma-cache-milestone-11-3-closeout-20260609, limited to the
documented/local-static/visibility frontier and retained risks above.

- Determine RP1 DMA addressability, dma-ranges, IOMMU behavior, and
  cache-coherency requirements.
- Define kernel APIs for cache clean/invalidate and DMA-safe buffers before
  Ethernet or block drivers use DMA.

Acceptance criteria:

- DMA buffer ownership and cache-maintenance rules are documented.
- A small DMA or driver-adjacent diagnostic exists before networking depends on
  DMA.

## Phase 12: Networking and SSH Development Access

Goal: reach Talos over the network and make the system usable without serial.

Milestone 12.1: RP1 Ethernet Research Spike

Status: source inventory, path ADR, GEM MID source contract, local/static GEM
MID diagnostic report core, diagnostic closeout, and serialized GEM MID Pi 5
proof accepted in
phase12-rp1-ethernet-source-inventory-20260609 and
phase12-rp1-ethernet-path-adr-20260609, with source contract
phase12-rp1-ethernet-gem-mid-source-contract-20260609 and report core
phase12-rp1-ethernet-gem-mid-diagnostic-core-20260609 and closeout
phase12-rp1-ethernet-gem-mid-diagnostic-closeout-20260609, followed by
phase12-rp1-ethernet-gem-mid-pi5-proof-20260609. The accepted
inventory is source-only: RP1 Ethernet is `raspberrypi,rp1-gem` /
`cdns,macb` at RP1 bus `0xc0_40100000` with source CPU physical translation
`0x1f00100000`, `RP1_INT_ETH`, RP1 Ethernet clocks, RGMII PHY mode, and PHY
reset through RP1 GPIO32. The accepted ADR chooses the direct RP1 Cadence GEM
path, staged behind hardware-substrate proofs rather than immediate driver
work. The accepted source contract names exactly one future read-only target:
`MACB_MID` offset `0x00fc`, source RP1 bus `0xc0_401000fc`, source CPU
physical `0x1f001000fc`, width 32, little-endian volatile load. The accepted
local/static report core constructs a candidate report for that contract and a
paired no-Ethernet/no-MMIO control that withholds Ethernet MMIO target fields.
The accepted closeout selects a serialized Pi 5 GEM MID visibility/control
proof as the next bounded step. The accepted Pi 5 proof showed the control
report path is visible without constructing Ethernet MMIO, but the candidate
read returned `raw=0xdeaddead` at `0x1f001000fc`, classified as
`rp1-ethernet-gem-mid-blocked-address-decode-sentinel`. The accepted blocker
reconciliation refines that as
`rp1-ethernet-gem-mid-retained-0x1f-window-sentinel`: retained sources still
support the `0x1f001000fc` translation, while accepted Phase 11 evidence
shows `0xdeaddead` on translated `0x1f` RP1 reads and visible observed
`0x1c` RP1 sysinfo/clock/GPIO reads. Live GEM visibility, Ethernet driver
readiness, broad live MMIO, descriptor rings, packet I/O, networking, sockets,
SSH, and Phase 12.2 implementation remain unaccepted. The next selected
discriminator is a local/static same-run report with observed
`SYSINFO_CHIP_ID` at `0x1c00000000` as positive control plus `MACB_MID` at
`0x1f001000fc`, paired with a no-MMIO/no-Ethernet control.

phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610
accepts the repaired pi5-capture-chain-v4 serialized hardware proof as
`observed-rp1-positive-control-gem-mid-0x1f-window-sentinel`. Candidate and
control evidence now join selected-tree identity, expected TFTP fetch bytes,
run-unique serial markers, final pre-restore identity, and restore proof. The
candidate observed `SYSINFO_CHIP_ID` at `0x1c00000000` as `0x20001927`
and translated `MACB_MID` at `0x1f001000fc` as `0xdeaddead`; the paired
control retained the no-MMIO/no-Ethernet marker without constructing RP1 or
Ethernet MMIO targets. This accepts the repaired capture path and the retained
`0x1f` sentinel result, not live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, packet I/O, DMA/descriptors, interrupts,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout-20260610 closes
same-shaped GEM MID decode-discriminator hardware retries. The accepted
frontier is now: the repaired capture chain is decisive for this discriminator,
same-run RP1 SYSINFO_CHIP_ID is visible at `0x1c00000000` as `0x20001927`,
and translated `MACB_MID` at `0x1f001000fc` remains `0xdeaddead`. No queued
follow-up is mechanically objective from that sentinel result alone; supervisor
planning is required for a different bounded discriminator or bridge/window
dependency slice before more Phase 12.1 work.

phase12-rp1-ethernet-observed-window-contract-20260610 accepts that different
bounded discriminator as a source/evidence contract. The observed-window
candidate target is `MACB_MID` at `0x1c001000fc`, computed from observed RP1
base `0x1c00000000` plus the retained rp1_eth/MACB_MID offset `0x001000fc`.
The translated target `0x1f001000fc` remains only a comparator/sentinel, and
the paired control must construct no observed RP1, translated comparator, or
Ethernet MMIO target. This is not live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, RP1 MMIO writes, DMA/descriptors,
interrupts, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-observed-window-discriminator-core-20260610 and closeout
phase12-rp1-ethernet-observed-window-discriminator-closeout-20260610 accept
the local/static observed-window candidate/control report surface and select
the serialized Pi 5 proof as the next bounded task. The proof must acquire
hardwareTestLock and join candidate/control evidence through selected-tree
identity, expected TFTP fetch bytes, run-unique serial marker freshness, final
pre-restore identity, restore proof, and task-owned JSON. It may classify only
an observed-window visible read, observed-window sentinel/fault with SYSINFO
positive-control retained, or a precise staging/capture blocker, and still
does not accept broad Ethernet readiness, driver readiness, packet I/O, DMA,
descriptor rings, interrupts, networking, sockets, SSH, Phase 12.2, or a phase
transition.

phase12-rp1-ethernet-observed-window-discriminator-pi5-proof-20260610 accepts
the serialized Pi 5 proof as observed-window-macb-mid-visible. Candidate and
control capture-chain-v4 identity joins passed; the candidate read
SYSINFO_CHIP_ID at 0x1c00000000 as 0x20001927 and observed-window MACB_MID at
0x1c001000fc as raw 0x70109, idnum 0x7, rev 0x109. This remains read-only
identity evidence only and does not accept Ethernet driver readiness, broad
Ethernet MMIO readiness, RP1 MMIO writes, DMA, descriptor rings, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-observed-window-discriminator-proof-closeout-20260610
closes that observed-window discriminator proof frontier. Same-shaped
observed-window hardware retries are closed for this candidate/control pair;
the accepted boundary is only the read-only MACB_MID identity result with
paired no-MMIO/no-Ethernet control evidence. No explicit queued follow-up is
mechanically objective from this closeout alone, so supervisor planning is
required for the next bounded Phase 12.1 prerequisite or ownership slice before
any Ethernet driver, RP1 MMIO write, DMA, descriptor-ring, interrupt, packet
I/O, networking, sockets, SSH, Phase 12.2, or phase-transition work.

phase12-rp1-ethernet-prereq-ownership-source-contract-20260610 accepts that
next ownership slice as a source-backed contract. It preserves observed-window
MACB_MID visibility only as identity context, then reconciles rp1_eth source
facts for RP1_INT_ETH, pclk/hclk/tsu_clk/tx_clk, RP1 clock ids, RGMII-ID PHY
mode, phy1, RP1 GPIO32 active-low PHY reset, MDIO/PHY handling, interrupt
completion, DMA, and descriptor-ring dependencies against accepted Phase 11
frontiers. The selected follow-up is local/static candidate/control ownership
report construction; no new hardware read or write-backed ownership claim is
selected by this contract. Ethernet driver readiness, broad Ethernet MMIO
readiness, RP1 MMIO writes, clock/reset ownership, GPIO32 or PHY reset
ownership, MDIO transactions, interrupt delivery/completion, DMA, descriptor
rings, packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition
remain unaccepted.

phase12-rp1-ethernet-prereq-ownership-report-core-20260610 accepts the
local/static report-core implementation selected by that source contract. The
candidate report preserves source-backed prerequisite metadata for rp1_eth
clocks, RP1_INT_ETH, RGMII-ID phy1, RP1 GPIO32 active-low reset, PHY/MDIO
policy, DMA/descriptor dependency policy, and context-only observed-window
MACB_MID identity. The paired control withholds candidate-only prerequisite
facts and carries no-ownership-no-ethernet-rp1-ethernet-prereq-control. This
is not hardware evidence and does not accept Ethernet readiness, RP1 MMIO
writes, clock/reset/GPIO/PHY/MDIO ownership, DMA, descriptor rings, packet
I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-prereq-ownership-report-closeout-20260610 closes that
local/static report frontier and selects the serialized Pi 5 prerequisite proof
as the next bounded task. phase12-rp1-ethernet-prereq-ownership-pi5-proof-20260610
accepts that proof as
rp1-ethernet-prereq-ownership-report-visibility-control-output. Candidate and
control capture-chain-v4 identity joins passed; the candidate serial output
printed the accepted prerequisite report fields, while the paired control used
the same report path and withheld candidate-only prerequisite facts. This is
hardware report visibility/control evidence only and does not accept
hardware/runtime prerequisite ownership, Ethernet readiness, RP1 MMIO writes,
clock/reset/GPIO/PHY/MDIO ownership, DMA, descriptor rings, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-prereq-ownership-proof-closeout-20260610 closes the
prerequisite report visibility frontier. Same-shaped report visibility hardware
retries are closed for this candidate/control pair: the accepted proof shows
only that the candidate prerequisite metadata and paired no-ownership/
no-Ethernet control report path are visible under capture-chain-v4. No explicit
queued follow-up is mechanically objective from report visibility alone, so
supervisor planning is required for the next bounded Phase 12.1 prerequisite
ownership or implementation slice before any runtime prerequisite ownership,
Ethernet driver work, RP1 MMIO writes, DMA, descriptor rings, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase-transition work.

phase12-rp1-ethernet-clock-reset-ownership-contract-20260610 accepts the next
bounded clock/reset ownership contract after supervisor planning. The accepted
input frontier is still observed-window MACB_MID identity plus prerequisite
report visibility/control output. The contract names pclk/hclk/tsu_clk/tx_clk,
their RP1 clock IDs, Linux macb_clk_init enable behavior, the shared-clock
safety rule for RP1_CLK_SYS pclk/hclk, and the absence of an accepted Pi 5
rp1_eth reset-controller target. It requires a read-only baseline and exact
future register/restore contract before any write-backed ownership, keeps PHY
reset in the GPIO32/MDIO slice, and selects only the local/static clock-reset
guard core as the next bounded follow-up. Ethernet driver readiness, broad
Ethernet MMIO readiness, clock/reset writes or ownership, PHY/MDIO, DMA,
descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and
phase transition remain unaccepted.

phase12-rp1-ethernet-clock-reset-guard-core-20260610 accepts local/static
candidate/control guard report construction only. The candidate carries
observed-window MACB_MID identity as context, pclk/hclk/tsu_clk/tx_clk source
facts, RP1 clock IDs, shared-clock policy, read-only baseline requirements,
future write-backed invariants, rejected claims, and retained risks; the paired
control uses the same report path while withholding candidate-only clock/reset
facts. Validators reject runtime ownership, writes, hardware readiness, and
downstream Ethernet or phase claims.

phase12-rp1-ethernet-clock-reset-guard-closeout-20260610 closes the
local/static guard frontier. Same-shaped local/static guard retries are closed
for this candidate/control pair, and the next selected bounded task is the
serialized read-only clock/reset baseline Pi 5 proof. That proof may classify
only read-only baseline visibility/current-state, a precise sentinel/fault or
staging/capture blocker with identity retained, or a paired control result. It
does not authorize clock/reset ownership, RP1 MMIO or clock/reset writes,
GPIO32/PHY reset ownership, MDIO/PHY, DMA, descriptors, interrupts, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-clock-reset-readonly-baseline-pi5-proof-20260610 accepts
the serialized Pi 5 read-only baseline report proof as
rp1-ethernet-clock-reset-readonly-baseline-report-visibility-control-output.
Candidate/control capture-chain-v4 joins passed. The candidate selected tree
047815dc8bfde65c28be5d4a5844eb5bf83c4dc60749d7a9c76c8dce402599c3 fetched
da591740/kernel_2712.img at 50056 bytes and retained 19 run-unique serial
markers with observed-window MACB_MID context 0x1c001000fc/raw 0x70109/idnum
0x7/rev 0x109 plus pclk/hclk/tsu_clk/tx_clk baseline facts. The paired control
selected tree 16745426bc0d0f1cc2b1844f48d6e656a8c900afb6fcca42caee5553afc7f4fd
fetched da591740/kernel_2712.img at 49176 bytes and retained 25 run-unique
serial markers while withholding candidate-only clock/reset facts. The lab boot
tree was restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. This proof
accepts report visibility/control output only; it does not accept clock/reset
ownership, writes, reset-controller ownership, PHY/MDIO, DMA, packet I/O,
networking, SSH, Phase 12.2, or a phase transition.

phase12-rp1-ethernet-clock-reset-readonly-baseline-closeout-20260610 closes
the read-only baseline proof frontier as
rp1-ethernet-clock-reset-readonly-baseline-frontier-closed. Same-shaped
read-only baseline report visibility hardware retries are closed for this
candidate/control pair. Report visibility/control output alone does not make a
write-backed clock/reset ownership task mechanically objective, so supervisor
planning is required for the next explicit Phase 12.1 clock/reset ownership
slice with source-backed register/restore, shared-clock safety,
reset-controller, GPIO32/PHY, MDIO/PHY, interrupt, or DMA/descriptor
acceptance criteria. Clock/reset ownership, RP1 MMIO or clock/reset writes,
Ethernet driver readiness, PHY/MDIO/GPIO32 ownership, DMA, descriptors,
interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
transition remain unaccepted.

- Study RP1 Ethernet as exposed by Linux device tree: rp1_eth is compatible with raspberrypi,rp1-gem and cdns,macb, behind RP1 PCIe address space.
- Decide whether to implement the Cadence GEM path directly, reuse a no_std driver if viable, or stage networking through a simpler transport first.
- Capture RP1 PCIe, RP1 interrupt routing, clocks, DMA, IOMMU, PHY reset, and cache-coherency implications. RP1 is not a simple fixed MMIO block from the CPU's point of view.

Acceptance criteria:

- A design note or ADR records the chosen Ethernet path.
- Unknown hardware behavior has diagnostic tasks.

Milestone 12.2: Network Device Abstraction

- Reuse the Daedalus idea of a small NetworkDevice trait, but revise it for Talos needs.
- Keep packet parsing and protocol logic testable without hardware.

Acceptance criteria:

- Ethernet, ARP, and IP parsing tests run in QEMU or host-side unit tests.
- Driver-specific code is isolated from protocol code.

Milestone 12.3: IP Stack

- Prefer smoltcp for no_std TCP/IP evaluation rather than hand-rolling TCP
  unless a concrete Talos constraint rules it out.
- Implement packet buffers, ARP, IPv4, ICMP, UDP/TCP, and socket integration.

Acceptance criteria:

- Talos responds to ping on the lab network.
- Talos can establish a TCP connection or accept one through a test service.

Milestone 12.4: Socket Integration

- Integrate sockets with the existing descriptor table, scheduler, blocking I/O,
  poll/wakeup model, and process lifetime rules.
- Add network diagnostics as user programs where possible, not kernel-only
  command paths.

Acceptance criteria:

- User programs can open sockets through the normal syscall/libc path.
- A network diagnostic program can accept or initiate a TCP connection.
- Blocking network I/O does not stall unrelated tasks.

Milestone 12.5: Entropy, Crypto, and SSH Strategy

- Bring up a kernel entropy source suitable for SSH host keys and session crypto.
- Evaluate porting an existing SSH server before writing one. OpenSSH is the
  compatibility target, but a smaller Rust SSH server may be a better first
  user-space port if it fits Talos libc/std and crypto constraints sooner.
- Define host key provisioning, authorized key storage, authentication policy, time requirements, heap-pressure expectations, and failure modes.

Acceptance criteria:

- ADR records the SSH implementation strategy.
- Entropy and key-management diagnostics exist before accepting SSH connections.

Milestone 12.6: SSH and Shell

- Implement or port the chosen SSH server and connect it to the existing local
  shell, PTY/TTY, descriptor, process, and filesystem model.
- Use SSH as the preferred path for user-space development and testing once it
  is reliable. Kernel changes may still use TFTP and lab power control, but
  user programs should not require serial-only workflows.

Acceptance criteria:

- User can connect remotely and run a shell.
- Multiple programs or commands can make progress concurrently.
- User-space programs can be copied, launched, and tested over SSH without using
  serial as the primary interaction channel.

## Phase 13: Toward a Useful Unix-Like System

Goal: grow from a local and remote shell into a practical small OS.

Milestones:

- Process spawning and wait/exit status.
- Permissions and user model sufficient for local experimentation.
- More complete POSIX compatibility review.
- Package/update workflow for user-space programs.
- Broader utility and service ports.
- Native build tools may be explored incrementally, but self-hosting GCC, LLVM,
  or rustc remains a north-star objective outside the committed roadmap.

Acceptance criteria:

- The shell can run separate programs, pipe output, inspect files, and operate
  locally or over SSH.
- Documentation explains how each major subsystem works and what POSIX gaps remain.

## Rolling Documentation Requirements

Each milestone should update at least one of:

- roadmap status
- task record
- architecture doc
- hardware note
- ADR
- lab runbook

Source-backed findings should cite URLs or local file references. Serial logs and boot attempts should be saved when they influence design decisions.
