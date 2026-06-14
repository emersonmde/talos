#!/bin/sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)"
RPI5="$ROOT/src/target/rpi5.rs"
MAIN="$ROOT/src/main.rs"
BUILD="$ROOT/build.rs"

require() {
    file=$1
    pattern=$2
    label=$3
    if ! grep -Fq "$pattern" "$file"; then
        printf 'missing %s: %s\n' "$label" "$pattern" >&2
        exit 1
    fi
}

reject() {
    file=$1
    pattern=$2
    label=$3
    if grep -Fq "$pattern" "$file"; then
        printf 'unexpected %s: %s\n' "$label" "$pattern" >&2
        exit 1
    fi
}

require "$BUILD" 'rpi5_rp1_ethernet_phy1_autoneg_restart_candidate' 'candidate build scenario'
require "$BUILD" 'rpi5_rp1_ethernet_phy1_autoneg_restart_no_mdio_control' 'control build scenario'
require "$MAIN" 'run_rp1_ethernet_phy1_autoneg_restart_candidate' 'candidate dispatch'
require "$MAIN" 'run_rp1_ethernet_phy1_autoneg_restart_no_mdio_control' 'control dispatch'

require "$RPI5" 'const BMCR_ANENABLE: u16 = 0x1000;' 'BMCR_ANENABLE bit'
require "$RPI5" 'const BMCR_ANRESTART: u16 = 0x0200;' 'BMCR_ANRESTART bit'
require "$RPI5" 'const BMCR_WRITE_MAN_FRAME_PREFIX: u32 = 0x5082_0000;' 'corrected Clause 22 PHY1 BMCR write frame prefix'
require "$RPI5" 'bmcr_write_value = pre_bmcr_raw | BMCR_ANENABLE | BMCR_ANRESTART;' 'preserve pre-BMCR plus autoneg bits'
require "$RPI5" 'if pre_bmcr.isolate {' 'isolate precondition'
require "$RPI5" 'bmcr_write_count = 1;' 'single guarded BMCR write count'
require "$RPI5" 'touched-fields=BMCR_ANENABLE,BMCR_ANRESTART' 'touched fields report'
require "$RPI5" 'pre-bmcr=' 'pre BMCR report field'
require "$RPI5" 'pre-bmsr=' 'pre BMSR report field'
require "$RPI5" 'pre-anar=' 'pre ANAR report field'
require "$RPI5" 'pre-anlpar=' 'pre ANLPAR report field'
require "$RPI5" 'post-bmcr=' 'post BMCR report field'
require "$RPI5" 'post-bmsr-first=' 'post first BMSR report field'
require "$RPI5" 'post-bmsr-second=' 'post second BMSR report field'
require "$RPI5" 'post-anar=' 'post ANAR report field'
require "$RPI5" 'post-anlpar=' 'post ANLPAR report field'
require "$RPI5" 'passive-macb-nsr-link=' 'passive MACB_NSR_LINK comparator'
require "$RPI5" 'classification=no-mdio-no-macb-phy1-autoneg-restart-control' 'control classification'
require "$RPI5" 'target=none controller=none compatible=none' 'control no target'
require "$RPI5" 'bmcr-write-value=withheld bmcr-write-count=0 touched-fields=none' 'control withheld BMCR fields'
require "$RPI5" 'mdio-man-transactions-performed=false' 'control no MDIO/MAN transaction'
require "$RPI5" 'macb-read-performed=false macb-write-performed=false' 'control no MACB access'
require "$RPI5" 'phy-reset-or-gpio32-action=false link-forcing=false' 'rejected reset/link forcing claims'
require "$RPI5" 'claims-phase-12-2=false claims-phase-transition=false' 'rejected phase claims'
cat <<'JSON'
{
  "classification": "phy1-autoneg-restart-guard-core-static-review-passed",
  "evidence_level": "task-owned static source review",
  "candidate": {
    "scenario_registered": true,
    "dispatch_registered": true,
    "corrected_target": "Clause 22 PHY1 BMCR register 0x00 through MAN prefix 0x50820000",
    "allowed_write_value": "pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART",
    "allowed_write_bits": ["BMCR_ANENABLE", "BMCR_ANRESTART"],
    "write_count": "exactly one guarded write after MPE and BMCR_ISOLATE preconditions",
    "post_fields": [
      "post-bmcr",
      "post-bmsr-first",
      "post-bmsr-second",
      "post-anar",
      "post-anlpar",
      "passive-macb-nsr-link"
    ],
    "rejected_claims_recorded": true
  },
  "control": {
    "scenario_registered": true,
    "dispatch_registered": true,
    "target_constructed": false,
    "mdio_man_transactions": false,
    "macb_access": false,
    "candidate_only_fields": "withheld",
    "classification": "no-mdio-no-macb-phy1-autoneg-restart-control"
  }
}
JSON
