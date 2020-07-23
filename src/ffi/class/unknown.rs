#![allow(non_snake_case)]
use crate::ffi::IUnknown;

#[repr(C)]
pub struct UnknownVtbl {
    pub IUnknown: IUnknown<Unknown>,
}

#[repr(C)]
pub struct Unknown {
    pub lpVtbl: *const UnknownVtbl,
}
