#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! A self-contained crate implementing safe types for the local APIC
//! registers on x86_64 systems.
//!
//! More information on the local APIC standard can be obtained on
//! chapter 16: Local APIC from the
//! [AMD Architecture Programmer's Manual Vol. 2: System Programming](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf).
//!
//! This crate does not depend on the standard Rust library and uses
//! [`#![forbid(unsafe_code)]`](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html#how-safe-and-unsafe-interact).
//!
//! # Usage
//!
//! The main type of this crate is the [`LocalApic`] structure. It has
//! no invalid representations, and has every APIC register laid out
//! as needed so it can be safely laid over mapped memory, provided
//! the region has the right alignment.
//!
//! All the registers in the structure provide safe setters and
//! getters to access the bitfields in those registers, as well as
//! methods to convert the registers to and from raw bytes, thanks to
//! the [modular_bitfield](https://docs.rs/modular-bitfield/latest/modular_bitfield/)
//! crate.

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

/// Local APIC registers.
#[repr(C, align(16))]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct LocalApic {
	__reserved1: [Reserved; 2],
	/// APIC ID Register.
	pub apic_id: ApicId,
	/// ACIC Version Register.
	pub apic_version: ApicVersion,
	__reserved2: [Reserved; 4],
	/// Task Priority Register (TPR).
	pub task_priority: PriorityRegister,
	/// Arbitration Priority Register (APR).
	pub arb_priority: PriorityRegister,
	/// Processor Priority Register (PPR).
	pub processor_priority: PriorityRegister,
	/// End of Interrupt Register (EOI).
	pub eoi: EndOfInterrupt,
	__reserved7: [u32; 4],
	/// Logical Destination Register.
	pub logical_dst: LogicalDestination,
	/// Destination Format Register.
	pub dst_format: DestinationFormat,
	/// Spurious Interrupt Vector Register.
	pub spurious_iv: SpuriousInterruptVector,
	/// In-Service Register (ISR).
	pub in_service: [BitfieldRegister; 8],
	/// Trigger Mode Register (TMR).
	pub trigger_mode: [BitfieldRegister; 8],
	/// Interrupt Request Register (IRR).
	pub interrupt_request: [BitfieldRegister; 8],
	/// Error Status Register (ESR).
	pub error_status: ErrorStatus,
	__reserved8: [Reserved; 7],
	/// Interrupt Command Register Low (bits 31:0).
	pub interrupt_cmd_low: InterruptCmdLow,
	/// Interrupt Command Register High (bits 63:32).
	pub interrupt_cmd_high: InterruptCmdHigh,
	/// Timer Local Vector Table Entry.
	pub timer_lvt: TimerLVT,
	/// Thermal Local Vector Table Entry.
	pub thermal_lvt: ThermalLVT,
	/// Performance Counter Local Vector Table Entry.
	pub performance_lvt: PerfLVT,
	/// Local Interrupt 0 Vector Table Entry
	pub lint0_lvt: LIntLVT,
	/// Local Interrupt 1 Vector Table Entry
	pub lint1_lvt: LIntLVT,
	/// Error Vector Table Entry.
	pub error_lvt: ErrorLVT,
	/// Timer Initial Count Register.
	pub timer_icr: TimerCount,
	/// Timer Current Count Register.
	pub timer_ccr: TimerCount,
	__reserved9: [Reserved; 4],
	/// Timer Divide Configuration Register.
	pub timer_dcr: TimerDivConf,
	__reserved10: Reserved,
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
struct Reserved([u32; 4]);

/// Local APIC register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ApicId {
	#[skip]
	__: B24,
	pub apic_id: B4,
	#[skip]
	__: B4,
	#[skip]
	__: B96,
}

/// ACIC Version Register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ApicVersion {
	pub version: u8,
	#[skip]
	__: u8,
	pub max_lvt: B8,
	#[skip]
	__: u8,
	#[skip]
	__: B96,
}

/// Priority structure for the [`TPR`](LocalApic::task_priority),
/// [`APR`](LocalApic::arb_priority) or
/// [`PPR`](LocalApic::processor_priority).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct PriorityRegister {
	pub priority: u8,
	#[skip]
	__: B24,
	#[skip]
	__: B96,
}

/// End of Interrupt Register (EOI).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct EndOfInterrupt {
	pub eoi: u32,
	#[skip]
	__: B96,
}

/// Logical Destination Register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct LogicalDestination {
	#[skip]
	__: B24,
	pub logical_dst: u8,
	#[skip]
	__: B96,
}

/// Destination Format Register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct DestinationFormat {
	#[skip]
	__: B28,
	pub model: B4,
	#[skip]
	__: B96,
}

/// Spurious Interrupt Vector Register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SpuriousInterruptVector {
	pub spurious_vector: u8,
	pub apic_enabled: B1,
	pub focus_cpu: B1,
	#[skip]
	__: B22,
	#[skip]
	__: B96,
}

/// Bitfield structure for the
/// [`ISR`](LocalApic::in_service), [`TMR`](LocalApic::trigger_mode)
/// or [`IRR`](LocalApic::interrupt_request).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct BitfieldRegister {
	pub bitfield: u32,
	#[skip]
	__: B96,
}

/// Error Status Register (ESR).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ErrorStatus {
	pub send_cs: B1,
	pub recv_cs: B1,
	pub send_accept: B1,
	pub recv_accept: B1,
	#[skip]
	__: B1,
	pub send_illegal_vector: B1,
	pub recv_illegal_vector: B1,
	pub illegal_register_addr: B1,
	#[skip]
	__: B24,
	#[skip]
	__: B96,
}

/// Interrupt Command Register Low (bits 31:0).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct InterruptCmdLow {
	pub vector: u8,
	pub delivery_mode: B3,
	pub destination_mode: B1,
	pub delivery_status: B1,
	#[skip]
	__: B1,
	pub level: B1,
	pub trigger: B1,
	#[skip]
	__: B2,
	pub shorthand: B2,
	#[skip]
	__: B12,
	#[skip]
	__: B96,
}

/// Interrupt Command Register High (bits 63:32).
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct InterruptCmdHigh {
	#[skip]
	__: B24,
	pub dst: u8,
	#[skip]
	__: B96,
}

/// Timer Local Vector Table Entry.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct TimerLVT {
	pub vector: u8,
	#[skip]
	__: B4,
	pub delivery_status: B1,
	#[skip]
	__: B3,
	pub mask: B1,
	pub timer_mode: B1,
	#[skip]
	__: B14,
	#[skip]
	__: B96,
}

/// Thermal Local Vector Table Entry.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ThermalLVT {
	pub vector: u8,
	pub delivery_mode: B3,
	#[skip]
	__: B1,
	pub delivery_status: B1,
	#[skip]
	__: B3,
	pub mask: B1,
	#[skip]
	__: B15,
	#[skip]
	__: B96,
}

/// Performance Counter Local Vector Table Entry.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct PerfLVT {
	pub vector: u8,
	pub delivery_mode: B3,
	#[skip]
	__: B1,
	pub delivery_status: B1,
	#[skip]
	__: B3,
	pub mask: B1,
	#[skip]
	__: B15,
	#[skip]
	__: B96,
}

/// Structure for [Local Interrupt 0](LocalApic::lint0_lvt) and
/// [1](LocalApic::lint1_lvt) Vector Table Entries.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct LIntLVT {
	pub vector: u8,
	pub delivery_mode: B3,
	#[skip]
	__: B1,
	pub delivery_status: B1,
	pub polarity: B1,
	pub remote_irr: B1,
	pub trigger: B1,
	pub mask: B1,
	#[skip]
	__: B15,
	#[skip]
	__: B96,
}

/// Error Vector Table Entry.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct ErrorLVT {
	pub vector: u8,
	#[skip]
	__: B4,
	pub delivery_status: B1,
	#[skip]
	__: B3,
	pub mask: B1,
	#[skip]
	__: B15,
	#[skip]
	__: B96,
}

/// Structure for [Initial](LocalApic::timer_icr) and
/// [Current](LocalApic::timer_ccr) Timer Count Registers.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct TimerCount {
	pub count: u32,
	#[skip]
	__: B96,
}

/// Timer Divide Configuration Register.
#[bitfield(bits = 128)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct TimerDivConf {
	pub divisor: B4,
	#[skip]
	__: B28,
	#[skip]
	__: B96,
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::mem;
	use memoffset::offset_of;

	#[test]
	fn test_offsets() {
		// Offsets from Table 16-2. APIC Registers - AMD Architecture
		// Programmer's Manual Vol. 2: System Programming
		assert_eq!(offset_of!(LocalApic, apic_id), 0x20);
		assert_eq!(offset_of!(LocalApic, apic_version), 0x30);
		assert_eq!(offset_of!(LocalApic, task_priority), 0x80);
		assert_eq!(offset_of!(LocalApic, arb_priority), 0x90);
		assert_eq!(offset_of!(LocalApic, processor_priority), 0xa0);
		assert_eq!(offset_of!(LocalApic, eoi), 0xb0);
		assert_eq!(offset_of!(LocalApic, logical_dst), 0xd0);
		assert_eq!(offset_of!(LocalApic, dst_format), 0xe0);
		assert_eq!(offset_of!(LocalApic, spurious_iv), 0xf0);
		assert_eq!(offset_of!(LocalApic, in_service), 0x100);
		assert_eq!(offset_of!(LocalApic, trigger_mode), 0x180);
		assert_eq!(offset_of!(LocalApic, interrupt_request), 0x200);
		assert_eq!(offset_of!(LocalApic, error_status), 0x280);
		assert_eq!(offset_of!(LocalApic, interrupt_cmd_low), 0x300);
		assert_eq!(offset_of!(LocalApic, interrupt_cmd_high), 0x310);
		assert_eq!(offset_of!(LocalApic, timer_lvt), 0x320);
		assert_eq!(offset_of!(LocalApic, thermal_lvt), 0x330);
		assert_eq!(offset_of!(LocalApic, performance_lvt), 0x340);
		assert_eq!(offset_of!(LocalApic, lint0_lvt), 0x350);
		assert_eq!(offset_of!(LocalApic, lint1_lvt), 0x360);
		assert_eq!(offset_of!(LocalApic, error_lvt), 0x370);
		assert_eq!(offset_of!(LocalApic, timer_icr), 0x380);
		assert_eq!(offset_of!(LocalApic, timer_ccr), 0x390);
		assert_eq!(offset_of!(LocalApic, timer_dcr), 0x3e0);

		assert_eq!(mem::size_of::<LocalApic>(), 0x400);
	}

	#[test]
	fn set_field() {
		let id = ApicId::default().with_apic_id(3);
		let bytes = id.into_bytes()[..4].try_into().unwrap();
		let val = u32::from_le_bytes(bytes);
		assert_eq!(val, 0x3000000);
	}
}
