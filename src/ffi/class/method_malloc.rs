#![allow(non_snake_case)]
use crate::ffi::{IMethodMalloc, IUnknown};

#[repr(C)]
pub struct MethodMallocVtbl {
    pub IUnknown: IUnknown<MethodMalloc>,
    pub IMethodMalloc: IMethodMalloc<MethodMalloc>,
}

#[repr(C)]
pub struct MethodMalloc {
    pub lpVtbl: *const MethodMallocVtbl,
}
