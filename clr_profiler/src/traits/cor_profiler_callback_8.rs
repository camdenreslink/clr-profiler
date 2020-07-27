#![allow(unused_variables)]
use crate::{
    errors::Error,
    ffi::{FunctionID, BOOL, HRESULT, LPCBYTE, ULONG},
    CorProfilerCallback7,
};

pub trait CorProfilerCallback8: CorProfilerCallback7 {
    fn dynamic_method_jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        f_is_safe_to_block: BOOL,
        p_il_header: LPCBYTE,
        c_bil_header: ULONG,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn dynamic_method_jit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }
}
