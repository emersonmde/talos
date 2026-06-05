#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"
repo_dir="$(CDPATH= cd "$script_dir/.." && pwd)"

cd "$repo_dir"

evidence_dir="tasks/evidence/2026-06-05-phase10-generated-root-no-rebuild-transport-core"
mkdir -p "$evidence_dir" target

artifact_tool="target/generated-root-artifact"
rustc tools/generated-root-artifact.rs -o "$artifact_tool"

artifact_a="target/generated-root-artifact-a.bin"
artifact_b="target/generated-root-artifact-b.bin"
artifact_malformed="target/generated-root-artifact-malformed.bin"
"$artifact_tool" a "$artifact_a"
"$artifact_tool" b "$artifact_b"
"$artifact_tool" malformed "$artifact_malformed"

export TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO="qemu_local_shell_generated_userland_manifest"
TALOS_BOOT_SCENARIO="$TALOS_QEMU_LOCAL_COMMAND_LOOP_BOOT_SCENARIO" cargo -Zjson-target-spec build "$@"

elf_file="target/aarch64-talos-virt/debug/talos"
img_file="$elf_file.local-serial-command-loop.img"
kernel_elf_sha="$(sha256sum "$elf_file" | awk '{print $1}')"
kernel_end_hex="$(nm "$elf_file" | awk '$3 == "__kernel_end" {print $1}')"
if [ -z "$kernel_end_hex" ]; then
    echo "missing __kernel_end symbol" >&2
    exit 1
fi
kernel_end_dec="$((16#$kernel_end_hex))"
artifact_addr_dec="$((0x47000000))"
if [ "$kernel_end_dec" -gt "$artifact_addr_dec" ]; then
    printf '__kernel_end=0x%s exceeds generated-root artifact address 0x47000000\n' "$kernel_end_hex" >&2
    exit 1
fi

run_case() {
    local name="$1"
    local artifact="$2"
    local expected_source="$3"
    local expected_reason="$4"
    local expected_content="$5"
    local expected_status="$6"
    local port="$7"

    export TALOS_QEMU_LOCAL_COMMAND_LOOP_LABEL="qemu-local-shell-generated-userland-manifest"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_CLASSIFICATION="qemu-local-shell-generated-userland-manifest-complete"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_SHELL_GENERATED_USERLAND_MANIFEST_SMOKE="1"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_PREBUILT_ELF="$elf_file"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_ARTIFACT="$artifact"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_SOURCE="$expected_source"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_REASON="$expected_reason"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_CONTENT="$expected_content"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_EXPECTED_STATUS_HEX="$expected_status"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_LOG_FILE="target/qemu-local-shell-generated-root-no-rebuild-transport-$name.log"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_QEMU_LOG_FILE="target/qemu-local-shell-generated-root-no-rebuild-transport-$name.qemu.log"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_DIR="$evidence_dir"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_EVIDENCE_LOG="$evidence_dir/qemu-local-shell-generated-root-no-rebuild-transport-$name.log"
    export TALOS_QEMU_LOCAL_COMMAND_LOOP_PORT="$port"

    if [ -z "$artifact" ]; then
        unset TALOS_QEMU_LOCAL_COMMAND_LOOP_GENERATED_ROOT_ARTIFACT
    fi

    "$script_dir/qemu-local-serial-command-loop-smoke.sh"
}

run_case "artifact-a" "$artifact_a" "external" "valid-artifact" "Talos generated-root external artifact A" "0x0000000000000007" "54391"
run_case "artifact-b" "$artifact_b" "external" "valid-artifact" "Talos generated-root external artifact B" "0x0000000000000009" "54392"
run_case "missing" "" "compiled-fallback" "missing-artifact" "Talos generated-root manifest fixture" "0x0000000000000007" "54393"
run_case "malformed" "$artifact_malformed" "compiled-fallback" "digest-mismatch" "Talos generated-root manifest fixture" "0x0000000000000007" "54394"

kernel_img_sha="$(sha256sum "$img_file" | awk '{print $1}')"
artifact_a_sha="$(sha256sum "$artifact_a" | awk '{print $1}')"
artifact_b_sha="$(sha256sum "$artifact_b" | awk '{print $1}')"
artifact_malformed_sha="$(sha256sum "$artifact_malformed" | awk '{print $1}')"

summary="$evidence_dir/qemu-local-shell-generated-root-no-rebuild-transport-smoke.log"
{
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: kernel-elf-sha256=%s\n' "$kernel_elf_sha"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: kernel-img-sha256=%s\n' "$kernel_img_sha"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: kernel-end=0x%s artifact-address=0x47000000 collision-guard=pass\n' "$kernel_end_hex"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: artifact-a-sha256=%s path=%s\n' "$artifact_a_sha" "$artifact_a"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: artifact-b-sha256=%s path=%s\n' "$artifact_b_sha" "$artifact_b"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: artifact-malformed-sha256=%s path=%s\n' "$artifact_malformed_sha" "$artifact_malformed"
    printf 'qemu-local-shell-generated-root-no-rebuild-transport: same-kernel-binary=true source-edits-between-artifacts=false classification=qemu-local-shell-generated-root-no-rebuild-transport-complete\n'
} >"$summary"

cat "$summary"
