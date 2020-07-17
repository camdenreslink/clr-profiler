#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo<()> {
    // 28B5557D-3F3F-48b4-90B2-5F9EEA2F6C48
    pub const iid: GUID = GUID {
        data1: 0x28B5557D,
        data2: 0x3F3F,
        data3: 0x48b4,
        data4: [0x90, 0xB2, 0x5F, 0x9E, 0xEA, 0x2F, 0x6C, 0x48],
    };
}
