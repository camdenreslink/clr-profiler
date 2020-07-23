#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo10<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo10<()> {
    // 2F1B5152-C869-40C9-AA5F-3ABE026BD720
    pub const IID: GUID = GUID {
        data1: 0x2F1B5152,
        data2: 0xC869,
        data3: 0x40C9,
        data4: [0xAA, 0x5F, 0x3A, 0xBE, 0x02, 0x6B, 0xD7, 0x20],
    };
}
