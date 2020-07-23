#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, IUnknown};
#[repr(C)]
pub struct CorProfilerInfoVtbl {
    pub IUnknown: IUnknown<CorProfilerInfo>,
    pub ICorProfilerInfo: ICorProfilerInfo<CorProfilerInfo>,
    pub ICorProfilerInfo2: ICorProfilerInfo2<CorProfilerInfo>,
    pub ICorProfilerInfo3: ICorProfilerInfo3<CorProfilerInfo>,
    // pub ICorProfilerInfo4: ICorProfilerInfo4<CorProfilerInfo>,
    // pub ICorProfilerInfo5: ICorProfilerInfo5<CorProfilerInfo>,
    // pub ICorProfilerInfo6: ICorProfilerInfo6<CorProfilerInfo>,
    // pub ICorProfilerInfo7: ICorProfilerInfo7<CorProfilerInfo>,
    // pub ICorProfilerInfo8: ICorProfilerInfo8<CorProfilerInfo>,
    // pub ICorProfilerInfo9: ICorProfilerInfo9<CorProfilerInfo>,
    // pub ICorProfilerInfo10: ICorProfilerInfo10<CorProfilerInfo>,
}

#[repr(C)]
pub struct CorProfilerInfo {
    pub lpVtbl: *const CorProfilerInfoVtbl,
}
