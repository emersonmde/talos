# Hash Reconciliation Matrix

Task id: phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47-20260701

| Source | Archive SHA-256 | Selected member/path | Kernel SHA-256 | Kernel bytes | Disposition |
| --- | --- | --- | --- | --- | --- |
| v45 classification.json | 72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 | da591740/kernel_2712.img | 96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6 | 152896 | stale selected-kernel SHA claim; corrected/quarantined by v47 |
| v45 static/kernel-sha256.txt | n/a | target/tmp/selected-normal-runtime-kernel-main-v45-boot-tree/kernel_2712.img | 2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 | 152896 | matches recomputed archive member |
| v47 recomputation from tarball | 72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 | ./da591740/kernel_2712.img | 2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 | 152896 | authoritative selected publication contract |
| v45 task/docs claim before correction | 72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 | da591740/kernel_2712.img | 96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6 | 152896 | stale selected-kernel SHA claim; corrected with note |
| v46 blocker evidence | 72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 | da591740/kernel_2712.img | expected 96057d2f8970808011a308f7b3a92da6feb85097b44590947a1ac145f85c6be6, observed 2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 | 152896 | valid blocker reason; no publication occurred |
| v47 authoritative contract | 72c28bafe9bedd1474fd1dfb19db101e9078122506db1b6f13bbbebdad383f19 | da591740/kernel_2712.img, exact tar member ./da591740/kernel_2712.img | 2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 | 152896 | use for v48 preflight |

Conclusion: the archive itself was stable and internally reviewable. The
96057d2f... value was a stale metadata claim, not the selected member bytes in
target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz. The v48 preflight
must publish only the archive contract above and must fail closed if the served
da591740/kernel_2712.img bytes differ from
2f89f98cf9403ccee58c20f8014f3c5fd83accb2f99da2f35b4b1eec6928cdc5 or 152,896
bytes.
