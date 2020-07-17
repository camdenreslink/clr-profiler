#![allow(non_snake_case)]
use super::IUnknown;
// use i_cor_profiler_info::ICorProfilerInfo;
// use i_cor_profiler_info_10::ICorProfilerInfo10;
// use i_cor_profiler_info_2::ICorProfilerInfo2;
// use i_cor_profiler_info_3::ICorProfilerInfo3;
// use i_cor_profiler_info_4::ICorProfilerInfo4;
// use i_cor_profiler_info_5::ICorProfilerInfo5;
// use i_cor_profiler_info_6::ICorProfilerInfo6;
// use i_cor_profiler_info_7::ICorProfilerInfo7;
// use i_cor_profiler_info_8::ICorProfilerInfo8;
// use i_cor_profiler_info_9::ICorProfilerInfo9;

// mod i_cor_profiler_info;
// mod i_cor_profiler_info_10;
// mod i_cor_profiler_info_2;
// mod i_cor_profiler_info_3;
// mod i_cor_profiler_info_4;
// mod i_cor_profiler_info_5;
// mod i_cor_profiler_info_6;
// mod i_cor_profiler_info_7;
// mod i_cor_profiler_info_8;
// mod i_cor_profiler_info_9;

#[repr(C)]
pub struct CorProfilerInfoVtbl {
    pub IUnknown: IUnknown<CorProfilerInfo>,
    // pub ICorProfilerInfo: ICorProfilerInfo<CorProfilerInfo>,
    // pub ICorProfilerInfo2: ICorProfilerInfo2<CorProfilerInfo>,
    // pub ICorProfilerInfo3: ICorProfilerInfo3<CorProfilerInfo>,
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
