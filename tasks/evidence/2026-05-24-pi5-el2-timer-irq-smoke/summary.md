# Pi 5 EL2 Timer IRQ Smoke Evidence

Task: phase4-pi5-gic400-el2-timer-smoke-20260524

Evidence level: serial hardware boot/output plus TFTP/archive proof.

- Archive: target/talos-rpi5-timer-irq-boot.tar.gz
- Archive sha256: 1861b6978b505381fd28ffb21320f1db9434405c4ce44af69354d6e1e82f5bb2
- Kernel image sha256: 850902110e96af341e595f1493c0802f742e6618ad57546f0f37dc06236d3e0a
- Kernel image size: 86429 bytes
- TFTP proof: tftp-delta.json shows kernel_2712.img served to 10.42.1.4 at 86429 bytes.
- Serial proof: serial-observe.json shows irq-count=1, vector=5, iar=0x0000001a, intid=26, unexpected=0, post-IRQ workload progress, and rpi5-timer-irq-smoke: PASS.
- Safe state: pre-run snapshot pre-phase4-pi5-timer-irq-20260524T044757Z restored after capture.
