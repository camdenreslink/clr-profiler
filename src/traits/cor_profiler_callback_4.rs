use crate::{
    errors::Error,
    ffi::{
        mdMethodDef, CorProfilerFunctionControl, FunctionID, ModuleID, ObjectID, ReJITID, BOOL,
        HRESULT, SIZE_T, ULONG,
    },
    CorProfilerCallback3,
};

pub trait CorProfilerCallback4: CorProfilerCallback3 {
    fn rejit_compilation_started(
        &mut self,
        function_id: FunctionID,
        rejit_id: ReJITID,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn get_rejit_parameters(
        &mut self,
        module_id: ModuleID,
        method_id: mdMethodDef,
        p_function_control: CorProfilerFunctionControl,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn rejit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        rejit_id: ReJITID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn rejit_error(
        &mut self,
        module_id: ModuleID,
        method_id: mdMethodDef,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn moved_references_2(
        &mut self,
        c_moved_object_id_ranges: ULONG,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[SIZE_T],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn surviving_references_2(
        &mut self,
        c_surviving_object_id_ranges: ULONG,
        object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[SIZE_T],
    ) -> Result<(), Error> {
        Ok(())
    }
}
