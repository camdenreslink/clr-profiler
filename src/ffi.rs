#![allow(non_camel_case_types)]
mod class_factory;
mod cor_profiler_callback;
mod cor_profiler_info;
mod hresult;
mod i_unknown;

use core::ffi::c_void;

pub type c_int = i32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_ushort = u16;
pub type c_uchar = u8;
pub type BOOL = c_int;
pub type ULONG = c_ulong;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct GUID {
    pub data1: c_ulong,
    pub data2: c_ushort,
    pub data3: c_ushort,
    pub data4: [c_uchar; 8],
}

pub type IID = GUID;
pub type REFCLSID = *const IID;
pub type REFIID = *const IID;
pub type LPVOID = *mut c_void;

pub use self::class_factory::*;
pub use self::cor_profiler_callback::CorProfilerCallback;
pub use self::cor_profiler_info::CorProfilerInfo;
pub use self::hresult::*;
pub use self::i_unknown::*;
