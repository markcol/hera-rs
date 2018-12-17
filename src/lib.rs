//! An implementation of the Haverford Educational RISC Architecture in Rust.

//#![feature(crate_visibility_modifier)] // needed for edition 2018
#![forbid(overflowing_literals)]
#![warn(
    missing_docs,
    clippy::all,
    rust_2018_idioms,
    intra_doc_link_resolution_failure,
    missing_copy_implementations,
    missing_debug_implementations,
    path_statements,
    trivial_bounds,
    type_alias_bounds,
    unconditional_recursion,
    unions_with_drop_fields,
    while_true,
    bad_style,
    future_incompatible,
    rust_2018_compatibility,
    rust_2018_idioms,
    unused
)]
#![allow(
    dead_code,
    unknown_lints,
    clippy::cyclomatic_complexity,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments
)]

use bitflags::bitflags;
use std::u16;

bitflags! {
    /// Represents the VM status register, which contains a set of flags.
    #[derive(Default)]
    pub struct StatusFlags: u16 {
         /// Set to zero when zero is produced.
        const SIGN     = 0b0000000000000001;
        /// Indicates overflow from a signed arithmetic operation.
        const OVERFLOW = 0b0000000000000010;
        /// Indicates overflow from an unsigned operation.
        const CARRY    = 0b0000000000000100;
    }
}

impl StatusFlags {
    /// Clear all status flags.
    #[inline]
    pub fn reset(&mut self) {
        self.bits = 0;
    }
}

/// Memory is a type to represent the memory of the VM.
pub type Memory = [u16; u16::MAX as usize];

/// Regoster os a type to represent a register of the VM.
pub type Register = u16;

/// A representation of the HERA VM.
#[derive(Copy, Clone)]
pub struct HeraVM {
    /// Status flags.
    pub status_flags: StatusFlags,
    /// Program Counter (PC).
    pub pc: Register,
    /// Register 0. Always holds the value 0.
    pub r0: Register,
    /// Register 1.
    pub r1: Register,
    /// Register 2.
    pub r2: Register,
    /// Register 3.
    pub r3: Register,
    /// Register 4.
    pub r4: Register,
    /// Register 5.
    pub r5: Register,
    /// Register 6.
    pub r6: Register,
    /// Register 7.
    pub r7: Register,
    /// Register 8.
    pub r8: Register,
    /// Register 9.
    pub r9: Register,
    /// Register 10.
    pub r10: Register,
    /// Register 11.
    pub r11: Register,
    /// Register 12.
    pub r12: Register,
    /// Register 13.
    pub r13: Register,
    /// Register 14.
    pub r14: Register,
    /// Register 15.
    pub r15: Register,
    /// Register 16.
    pub r16: Register,
    /// Memory.
    pub memory: Memory,
}

impl HeraVM {
    /// Create a new HeraVM instances and return it.
    pub fn new() -> Self {
        HeraVM {
            ..Default::default()
        }
    }
}

impl Default for HeraVM {
    fn default() -> Self {
        HeraVM {
            status_flags: StatusFlags::empty(),
            pc: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            r16: 0,
            memory: [0; u16::MAX as usize],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flags_default_is_empty() {
        let flags: StatusFlags = Default::default();
        assert!(!flags.is_all());
        assert!(flags.is_empty());
    }

    #[test]
    fn status_flags_reset_clears_all_flags() {
        let mut flags = StatusFlags::all();
        assert_eq!(flags, StatusFlags::SIGN | StatusFlags::OVERFLOW | StatusFlags::CARRY);
        assert!(flags.is_all());
        assert!(!flags.is_empty());
        flags.reset();
        assert!(!flags.is_all());
        assert!(flags.is_empty());
    }

    #[test]
    fn vm_default_all_empty() {
        let vm = HeraVM::new();
        assert_eq!(vm.status_flags, StatusFlags::empty());
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.r0, 0);
        assert_eq!(vm.r1, 0);
        assert_eq!(vm.r2, 0);
        assert_eq!(vm.r3, 0);
        assert_eq!(vm.r4, 0);
        assert_eq!(vm.r5, 0);
        assert_eq!(vm.r6, 0);
        assert_eq!(vm.r7, 0);
        assert_eq!(vm.r8, 0);
        assert_eq!(vm.r9, 0);
        assert_eq!(vm.r10, 0);
        assert_eq!(vm.r11, 0);
        assert_eq!(vm.r12, 0);
        assert_eq!(vm.r13, 0);
        assert_eq!(vm.r14, 0);
        assert_eq!(vm.r15, 0);
        assert_eq!(vm.r16, 0);
    }
}
