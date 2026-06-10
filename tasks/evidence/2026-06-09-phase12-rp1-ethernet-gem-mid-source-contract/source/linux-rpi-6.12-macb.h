Source: https://raw.githubusercontent.com/raspberrypi/linux/rpi-6.12.y/drivers/net/ethernet/cadence/macb.h
Fetched: 2026-06-10
Purpose: retain the minimal Cadence MACB/GEM register-definition evidence
needed by phase12-rp1-ethernet-gem-mid-source-contract-20260609.

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Atmel MACB Ethernet Controller driver
 *
 * Copyright (C) 2004-2006 Atmel Corporation
 */

/* MACB register offsets */
#define MACB_NCR		0x0000 /* Network Control */
#define MACB_NCFGR		0x0004 /* Network Config */
#define MACB_NSR		0x0008 /* Network Status */
#define MACB_TAR		0x000c /* AT91RM9200 only */
#define MACB_TCR		0x0010 /* AT91RM9200 only */
#define MACB_TSR		0x0014 /* Transmit Status */
#define MACB_RBQP		0x0018 /* RX Q Base Address */
#define MACB_TBQP		0x001c /* TX Q Base Address */
#define MACB_RSR		0x0020 /* Receive Status */
#define MACB_ISR		0x0024 /* Interrupt Status */
#define MACB_IER		0x0028 /* Interrupt Enable */
#define MACB_IDR		0x002c /* Interrupt Disable */
#define MACB_IMR		0x0030 /* Interrupt Mask */
#define MACB_MAN		0x0034 /* PHY Maintenance */
#define MACB_PTR		0x0038
#define MACB_PFR		0x003c
#define MACB_FTO		0x0040
#define MACB_SCF		0x0044
#define MACB_MCF		0x0048
#define MACB_FRO		0x004c
#define MACB_FCSE		0x0050
#define MACB_ALE		0x0054
#define MACB_DTF		0x0058
#define MACB_LCOL		0x005c
#define MACB_EXCOL		0x0060
#define MACB_TUND		0x0064
#define MACB_CSE		0x0068
#define MACB_RRE		0x006c
#define MACB_ROVR		0x0070
#define MACB_RSE		0x0074
#define MACB_ELE		0x0078
#define MACB_RJA		0x007c
#define MACB_USF		0x0080
#define MACB_STE		0x0084
#define MACB_RLE		0x0088
#define MACB_TPF		0x008c
#define MACB_HRB		0x0090
#define MACB_HRT		0x0094
#define MACB_SA1B		0x0098
#define MACB_SA1T		0x009c
#define MACB_SA2B		0x00a0
#define MACB_SA2T		0x00a4
#define MACB_SA3B		0x00a8
#define MACB_SA3T		0x00ac
#define MACB_SA4B		0x00b0
#define MACB_SA4T		0x00b4
#define MACB_TID		0x00b8
#define MACB_TPQ		0x00bc
#define MACB_USRIO		0x00c0
#define MACB_WOL		0x00c4
#define MACB_MID		0x00fc
#define MACB_TBQPH		0x04C8
#define MACB_RBQPH		0x04D4

/* Bitfields in MID */
#define MACB_IDNUM_OFFSET			16
#define MACB_IDNUM_SIZE				12
#define MACB_REV_OFFSET				0
#define MACB_REV_SIZE				16
