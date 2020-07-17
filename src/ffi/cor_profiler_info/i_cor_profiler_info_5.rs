#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo5<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo5<()> {
    // 07602928-CE38-4B83-81E7-74ADAF781214
    pub const iid: GUID = GUID {
        data1: 0x07602928,
        data2: 0xCE38,
        data3: 0x4B83,
        data4: [0x81, 0xE7, 0x74, 0xAD, 0xAF, 0x78, 0x12, 0x14],
    };
}
