use crate::{
    errors::Error,
    ffi::{CorProfilerInfo, UINT},
    CorProfilerCallback2,
};
use std::ffi::c_void;

pub trait CorProfilerCallback3: CorProfilerCallback2 {
    fn initialize_for_attach(
        &mut self,
        p_cor_profiler_info_unk: &CorProfilerInfo,
        pv_client_data: &[c_void],
        cb_client_data: UINT,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn profiler_attach_complete(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
