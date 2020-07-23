#![allow(non_snake_case)]
use crate::ffi::GUID;
use std::marker::PhantomData;

#[repr(C)]
pub struct ICorProfilerCallback9<T> {
    // TODO: fill in FFI interface functions here
    phantom: PhantomData<T>,
}

impl ICorProfilerCallback9<()> {
    // 27583EC3-C8F5-482F-8052-194B8CE4705A
    pub const IID: GUID = GUID {
        data1: 0x27583EC3,
        data2: 0xC8F5,
        data3: 0x482F,
        data4: [0x80, 0x52, 0x19, 0x4B, 0x8C, 0xE4, 0x70, 0x5A],
    };
}
