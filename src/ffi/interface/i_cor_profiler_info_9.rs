#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo9<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo9<()> {
    // 008170DB-F8CC-4796-9A51-DC8AA0B47012
    pub const IID: GUID = GUID {
        data1: 0x008170DB,
        data2: 0xF8CC,
        data3: 0x4796,
        data4: [0x9A, 0x51, 0xDC, 0x8A, 0xA0, 0xB4, 0x70, 0x12],
    };
}
