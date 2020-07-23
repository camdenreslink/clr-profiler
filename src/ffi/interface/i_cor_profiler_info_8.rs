#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo8<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo8<()> {
    // C5AC80A6-782E-4716-8044-39598C60CFBF
    pub const IID: GUID = GUID {
        data1: 0xC5AC80A6,
        data2: 0x782E,
        data3: 0x4716,
        data4: [0x80, 0x44, 0x39, 0x59, 0x8C, 0x60, 0xCF, 0xBF],
    };
}
