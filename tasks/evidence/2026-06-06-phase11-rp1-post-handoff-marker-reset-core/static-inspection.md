Static image/header/symbol inspection

image=target/rpi5-rp1-post-handoff-marker-reset-core-boot-tree/kernel_2712.img sha256=42367beda5de1d0564417e6267a59bd5ae5b770798fa4a3cbb3c0ce101554350 size=51736 text_offset=0 header_image_size=51736 flags=12 magic=ARMd

Selected symbols:
0000000000200000 T __kernel_start
0000000000200000 T _start
00000000002011d8 T rust_entry
0000000000202e20 t _RNvNtNtCsan2nlJD4ir9_5talos6target4rpi544run_rp1_post_handoff_marker_reset_diagnostic
0000000000207e80 R __rodata_start
000000000020ca18 R __kernel_image_end
000000000020ca20 B __bss_start
000000000020ca30 B __heap_start

Disassembly snippets:

target/aarch64-talos-rpi5-bcm2712/debug/talos:	file format elf64-littleaarch64

Disassembly of section .text.boot:

0000000000200000 <_start>:
  200000: 14000010     	b	0x200040 <_start+0x40>
		...
  200010: 18 ca 00 00  	.word	0x0000ca18
  200014: 00 00 00 00  	.word	0x00000000
  200018: 0c 00 00 00  	.word	0x0000000c
		...
  200038: 41 52 4d 64  	.word	0x644d5241
  20003c: 00 00 00 00  	.word	0x00000000
  200040: aa0003f3     	mov	x19, x0
  200044: d5381040     	mrs	x0, CPACR_EL1
  200048: b26c0400     	orr	x0, x0, #0x300000
  20004c: d5181040     	msr	CPACR_EL1, x0
  200050: d5033fdf     	isb
  200054: d503201f     	nop
  200058: 10064e40     	adr	x0, 0x20ca20 <__bss_start>
  20005c: d503201f     	nop
  200060: 10064e81     	adr	x1, 0x20ca30 <__heap_start>
  200064: eb01001f     	cmp	x0, x1
  200068: 54000062     	b.hs	0x200074 <_start+0x74>
  20006c: f800841f     	str	xzr, [x0], #0x8
  200070: 17fffffd     	b	0x200064 <_start+0x64>
  200074: 90000a60     	adrp	x0, 0x34c000 <__stack_bottom+0x3f5d0>
  200078: 9128c000     	add	x0, x0, #0xa30
  20007c: 9100001f     	mov	sp, x0
  200080: aa1303e0     	mov	x0, x19
  200084: 94000455     	bl	0x2011d8 <rust_entry>
  200088: d503205f     	wfe
  20008c: 17ffffff     	b	0x200088 <_start+0x88>

Disassembly of section .vectors:

0000000000200800 <__exception_vectors>:
  200800: d10083ff     	sub	sp, sp, #0x20
  200804: a90007e0     	stp	x0, x1, [sp]
  200808: d2800000     	mov	x0, #0x0                // =0
  20080c: f9000be0     	str	x0, [sp, #0x10]
  200810: 14001d3b     	b	0x207cfc <__exception_entry_frame>
  200814: d503201f     	nop
  200818: d503201f     	nop
  20081c: d503201f     	nop
  200820: d503201f     	nop
  200824: d503201f     	nop
  200828: d503201f     	nop
  20082c: d503201f     	nop
  200830: d503201f     	nop
  200834: d503201f     	nop
  200838: d503201f     	nop
  20083c: d503201f     	nop
  200840: d503201f     	nop
  200844: d503201f     	nop
  200848: d503201f     	nop
  20084c: d503201f     	nop
  200850: d503201f     	nop
  200854: d503201f     	nop
  200858: d503201f     	nop
  20085c: d503201f     	nop
--
  2011c4: 3738006a     	tbnz	w10, #0x7, 0x2011d0 <__rustc::rust_begin_unwind+0xe0>
  2011c8: f10006b5     	subs	x21, x21, #0x1
  2011cc: 54ffffa1     	b.ne	0x2011c0 <__rustc::rust_begin_unwind+0xd0>
  2011d0: d5033f9f     	dsb	sy
  2011d4: 9400009f     	bl	0x201450 <talos::arch::aarch64::halt>

00000000002011d8 <rust_entry>:
  2011d8: f81f0ffe     	str	x30, [sp, #-0x10]!
  2011dc: 2a1f03e0     	mov	w0, wzr
  2011e0: 94000104     	bl	0x2015f0 <talos::target::rpi5::write_early_phase_line>
  2011e4: 9400070f     	bl	0x202e20 <talos::target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic>

00000000002011e8 <talos::runtime_console::write_default_console_output::<talos::pl011::Pl011>>:
  2011e8: d10103ff     	sub	sp, sp, #0x40
  2011ec: a9034ffe     	stp	x30, x19, [sp, #0x30]
  2011f0: aa0803f3     	mov	x19, x8
  2011f4: d291b068     	mov	x8, #0x8d83             // =36227
  2011f8: f9000fff     	str	xzr, [sp, #0x18]
  2011fc: a9402809     	ldp	x9, x10, [x0]
  201200: f2a00408     	movk	x8, #0x20, lsl #16
  201204: f2c00008     	movk	x8, #0x0, lsl #32
  201208: f2e00008     	movk	x8, #0x0, lsl #48
  20120c: a9022be9     	stp	x9, x10, [sp, #0x20]
  201210: 5280020a     	mov	w10, #0x10              // =16
  201214: a900abe8     	stp	x8, x10, [sp, #0x8]
  201218: 370000c2     	tbnz	w2, #0x0, 0x201230 <talos::runtime_console::write_default_console_output::<talos::pl011::Pl011>+0x48>
  20121c: 910023e0     	add	x0, sp, #0x8
  201220: 94000083     	bl	0x20142c <<talos::runtime_console::RuntimeConsole<talos::pl011::Pl011> as core::fmt::Write>::write_fmt (.llvm.1960748047795824501)>
  201224: 360000e0     	tbz	w0, #0x0, 0x201240 <talos::runtime_console::write_default_console_output::<talos::pl011::Pl011>+0x58>
  201228: 52800028     	mov	w8, #0x1                // =1
  20122c: 14000006     	b	0x201244 <talos::runtime_console::write_default_console_output::<talos::pl011::Pl011>+0x5c>
  201230: d341fc42     	lsr	x2, x2, #1
  201234: 910023e0     	add	x0, sp, #0x8
  201238: 94000015     	bl	0x20128c <<talos::runtime_console::RuntimeConsole<talos::pl011::Pl011> as core::fmt::Write>::write_str (.llvm.1960748047795824501)>
  20123c: 3707ff60     	tbnz	w0, #0x0, 0x201228 <talos::runtime_console::write_default_console_output::<talos::pl011::Pl011>+0x40>
  201240: aa1f03e8     	mov	x8, xzr
  201244: a9412be9     	ldp	x9, x10, [sp, #0x10]
  201248: f94007eb     	ldr	x11, [sp, #0x8]
  20124c: a9002e68     	stp	x8, x11, [x19]
  201250: a9012a69     	stp	x9, x10, [x19, #0x10]
  201254: a9434ffe     	ldp	x30, x19, [sp, #0x30]
--
  202e0c: 3738006a     	tbnz	w10, #0x7, 0x202e18 <talos::target::rpi5::write_early_phase_line+0x1828>
  202e10: f10006b5     	subs	x21, x21, #0x1
  202e14: 54ffffa1     	b.ne	0x202e08 <talos::target::rpi5::write_early_phase_line+0x1818>
  202e18: d5033f9f     	dsb	sy
  202e1c: 1400001c     	b	0x202e8c <OUTLINED_FUNCTION_0>

0000000000202e20 <talos::target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic>:
  202e20: a9bf57fe     	stp	x30, x21, [sp, #-0x10]!
  202e24: d291fc00     	mov	x0, #0x8fe0             // =36832
  202e28: 52800701     	mov	w1, #0x38               // =56
  202e2c: f2a00400     	movk	x0, #0x20, lsl #16
  202e30: f2c00000     	movk	x0, #0x0, lsl #32
  202e34: f2e00000     	movk	x0, #0x0, lsl #48
  202e38: 97fff988     	bl	0x201458 <talos::target::rpi5::write_early_static>
  202e3c: d2920300     	mov	x0, #0x9018             // =36888
  202e40: 528008e1     	mov	w1, #0x47               // =71
  202e44: f2a00400     	movk	x0, #0x20, lsl #16
  202e48: f2c00000     	movk	x0, #0x0, lsl #32
  202e4c: f2e00000     	movk	x0, #0x0, lsl #48
  202e50: 97fff982     	bl	0x201458 <talos::target::rpi5::write_early_static>
  202e54: d2820009     	mov	x9, #0x1000             // =4096
  202e58: f2afa009     	movk	x9, #0x7d00, lsl #16
  202e5c: f2c00209     	movk	x9, #0x10, lsl #32
  202e60: d2a00415     	mov	x21, #0x200000          // =2097152
  202e64: b940192a     	ldr	w10, [x9, #0x18]
  202e68: 3738006a     	tbnz	w10, #0x7, 0x202e74 <talos::target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic+0x54>
  202e6c: f10006b5     	subs	x21, x21, #0x1
  202e70: 54ffffa1     	b.ne	0x202e64 <talos::target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic+0x44>
  202e74: d5033f9f     	dsb	sy
  202e78: 52800120     	mov	w0, #0x9                // =9
  202e7c: 72b08000     	movk	w0, #0x8400, lsl #16
  202e80: d4000003     	smc	#0
  202e84: d5033fdf     	isb
  202e88: 17fffffc     	b	0x202e78 <talos::target::rpi5::run_rp1_post_handoff_marker_reset_diagnostic+0x58>

0000000000202e8c <OUTLINED_FUNCTION_0>:
  202e8c: f84107f5     	ldr	x21, [sp], #0x10
  202e90: d65f03c0     	ret

0000000000202e94 <<talos::arch::aarch64::exceptions::ExceptionVector>::name>:
  202e94: d2926f08     	mov	x8, #0x9378             // =37752
  202e98: d2927f09     	mov	x9, #0x93f8             // =37880
  202e9c: f2a00408     	movk	x8, #0x20, lsl #16
  202ea0: f2a00409     	movk	x9, #0x20, lsl #16
  202ea4: f2c00008     	movk	x8, #0x0, lsl #32
  202ea8: f2c00009     	movk	x9, #0x0, lsl #32
  202eac: f2e00008     	movk	x8, #0x0, lsl #48
  202eb0: f2e00009     	movk	x9, #0x0, lsl #48
  202eb4: f8607901     	ldr	x1, [x8, x0, lsl #3]
  202eb8: f8607920     	ldr	x0, [x9, x0, lsl #3]
  202ebc: d65f03c0     	ret

0000000000202ec0 <talos::arch::aarch64::exceptions::write_exception_class>:
  202ec0: a9bf4ffe     	stp	x30, x19, [sp, #-0x10]!
  202ec4: aa0003f3     	mov	x19, x0
  202ec8: d2925c00     	mov	x0, #0x92e0             // =37600
  202ecc: 52800221     	mov	w1, #0x11               // =17
  202ed0: f2a00400     	movk	x0, #0x20, lsl #16
  202ed4: f2c00000     	movk	x0, #0x0, lsl #32
  202ed8: f2e00000     	movk	x0, #0x0, lsl #48
  202edc: 97fff95f     	bl	0x201458 <talos::target::rpi5::write_early_static>
  202ee0: 531a7e73     	lsr	w19, w19, #26
  202ee4: d1008268     	sub	x8, x19, #0x20
  202ee8: f100151f     	cmp	x8, #0x5

String checks:
rpi5-rp1-post-handoff-marker-reset: post-handoff-marker
rpi5-rp1-post-handoff-marker-reset: classification=marker-before-reset
