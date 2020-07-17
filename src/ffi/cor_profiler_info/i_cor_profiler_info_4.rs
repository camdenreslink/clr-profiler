#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo4<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo4<()> {
    // 0D8FDCAA-6257-47BF-B1BF-94DAC88466EE
    pub const iid: GUID = GUID {
        data1: 0x0D8FDCAA,
        data2: 0x6257,
        data3: 0x47BF,
        data4: [0xB1, 0xBF, 0x94, 0xDA, 0xC8, 0x84, 0x66, 0xEE],
    };
}
