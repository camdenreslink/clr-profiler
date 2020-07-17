#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerCallback2<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerCallback2<()> {
    // 8A8CC829-CCF2-49FE-BBAE-0F022228071A
    pub const iid: GUID = GUID {
        data1: 0x8A8CC829,
        data2: 0xCCF2,
        data3: 0x49FE,
        data4: [0xBB, 0xAE, 0x0F, 0x02, 0x22, 0x28, 0x07, 0x1A],
    };
}
