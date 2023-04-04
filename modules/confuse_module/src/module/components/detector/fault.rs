extern crate num_traits;
use num_derive::FromPrimitive;

use anyhow::{Context, Error, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromPrimitive, Hash, PartialEq, Eq, Copy, Clone)]
#[repr(i64)]
/// An X86 CPU Fault. Faults are generated by SIMICS' `Core_Exception` HAP and checked
/// against a set of faults that are considered crashes for a given fuzzing campaign. Most
/// of these faults are self-explanatory or documented in the SDM
pub enum X86_64Fault {
    /// Triple Fault
    ///
    /// Triple fault doesn't have a number, so we choose -1 which will never be the exception
    /// number
    Triple = -1,
    Division = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    Double = 8,
    InvalidTss = 10,
    SegmentNotPresent = 11,
    StackSegment = 12,
    GeneralProtection = 13,
    Page = 14,
    X86Fpe = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdFpen = 19,
    VirtualizationException = 20,
    ControlProtectionException = 21,
}

impl TryFrom<i64> for X86_64Fault {
    type Error = Error;

    /// Try to convert an i64 to a fault and fail if the number is unknown
    fn try_from(value: i64) -> Result<Self> {
        num::FromPrimitive::from_i64(value).context("Could not convert to Fault")
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone)]
#[repr(i64)]
/// A fault number, this is defined per architecture.
pub enum Fault {
    X86_64(X86_64Fault),
}
