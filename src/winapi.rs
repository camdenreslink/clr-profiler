use core::ffi::c_void;

#[allow(non_camel_case_types)]
pub type c_int = i32;

#[allow(non_camel_case_types)]
pub type c_long = i32;

#[allow(non_camel_case_types)]
pub type c_ulong = u32;

#[allow(non_camel_case_types)]
pub type c_ushort = u16;

#[allow(non_camel_case_types)]
pub type c_uchar = u8;

pub type BOOL = c_int;

pub type ULONG = c_ulong;

pub type HRESULT = c_long;

pub const S_OK: HRESULT = 0;

#[allow(overflowing_literals)]
pub const E_NOINTERFACE: HRESULT = 0x8000_4002;

#[allow(overflowing_literals)]
pub const E_POINTER: HRESULT = 0x8000_4003;

#[allow(overflowing_literals)]
pub const E_INVALIDARG: HRESULT = 0x8007_0057;

#[allow(overflowing_literals)]
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E;

#[allow(overflowing_literals)]
pub const E_UNEXPECTED: HRESULT = 0x8000_FFFF;

#[allow(overflowing_literals)]
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110;

#[allow(overflowing_literals)]
pub const E_FAIL: HRESULT = 0x8000_4005;

#[repr(C)]
#[derive(Debug)]
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

pub struct IUnknownVtbl {
    pub QueryInterface: extern "system" fn(
        This: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: extern "system" fn(This: *mut IUnknown) -> ULONG,
    pub Release: extern "system" fn(This: *mut IUnknown) -> ULONG,
}

pub struct IUnknown {
    pub lpVtbl: *const IUnknownVtbl,
}

pub struct IClassFactoryVtbl {
    pub parent: IUnknownVtbl,
    pub CreateInstance: extern "system" fn(
        This: *mut IClassFactory,
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub LockServer: extern "system" fn(This: *mut IClassFactory, fLock: BOOL) -> HRESULT,
}

pub struct IClassFactory {
    pub lpVtbl: *const IClassFactoryVtbl,
}

trait ClassFactory {}
