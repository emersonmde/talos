# Static/Image Comparison

Compared the rebuilt production timer/preemption Pi 5 image against the accepted multicore preemption Pi 5 image.

Both images share identical _start instructions through the primary early UART path; differences before Rust are limited to arm64 Image header size and secondary branch target relocation caused by normal text layout.

Multicore:
103144 target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-multicore-preemption.img
d0730fd3ed640767fabe18e329af91c6348f0e99911d2444db1fb31b704d10e8  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-multicore-preemption.img
                    0               103144
                   12

Production timer/preemption:
104136 target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-production-timer-preemption.img
fdf8858d0740c0d7bf4fc0df884d4052d8309fd9c020ba65e5df1472198e7dfa  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-production-timer-preemption.img
                    0               104136
                   12
