use crate::{
    errors::Error,
    ffi::{
        int, mdMethodDef, AppDomainID, AssemblyID, ClassID, CorProfilerAssemblyReferenceProvider,
        CorProfilerFunctionControl, CorProfilerInfo, FunctionID, GCHandleID, ModuleID, ObjectID,
        ReJITID, ThreadID, BOOL, COR_PRF_GC_REASON, COR_PRF_GC_ROOT_FLAGS, COR_PRF_GC_ROOT_KIND,
        COR_PRF_JIT_CACHE, COR_PRF_SUSPEND_REASON, COR_PRF_TRANSITION_REASON, DWORD, GUID, HRESULT,
        LPCBYTE, REFGUID, SIZE_T, UINT, UINT_PTR, ULONG, WCHAR,
    },
};
use std::ffi::c_void;

pub trait CorProfilerCallback {
    fn initialize(&mut self, p_icorprofiler_info_unk: CorProfilerInfo) -> Result<(), Error> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_creation_started(&mut self, app_domain_id: AppDomainID) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_creation_finished(
        &mut self,
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_shutdown_started(&mut self, app_domain_id: AppDomainID) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_shutdown_finished(
        &mut self,
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_load_started(&mut self, assembly_id: AssemblyID) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_load_finished(
        &mut self,
        assembly_id: AssemblyID,
        hr_status: HRESULT,
    ) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn assembly_unload_started(&mut self, assembly_id: AssemblyID) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_unload_finished(
        &mut self,
        assembly_id: AssemblyID,
        hr_status: HRESULT,
    ) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_load_started(&mut self, module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }

    fn module_load_finished(
        &mut self,
        module_id: ModuleID,
        hr_status: HRESULT,
    ) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_unload_started(&mut self, module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }

    fn module_unload_finished(
        &mut self,
        module_id: ModuleID,
        hr_status: HRESULT,
    ) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_attached_to_assembly(
        &mut self,
        module_id: ModuleID,
        assembly_id: AssemblyID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn class_load_started(&mut self, class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn class_load_finished(&mut self, class_id: ClassID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn class_unload_started(&mut self, class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn class_unload_finished(
        &mut self,
        class_id: ClassID,
        hr_status: HRESULT,
    ) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn function_unload_started(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_cached_function_search_started(
        &mut self,
        function_id: FunctionID,
        pb_use_cached_function: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_cached_function_search_finished(
        &mut self,
        function_id: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_function_pitched(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn jit_inlining(
        &mut self,
        caller_id: FunctionID,
        callee_id: FunctionID,
        pf_should_inline: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn thread_created(&mut self, thread_id: ThreadID) -> Result<(), Error> {
        Ok(())
    }

    fn thread_destroyed(&mut self, thread_id: ThreadID) -> Result<(), Error> {
        Ok(())
    }

    fn thread_assigned_to_os_thread(
        &mut self,
        managed_thread_id: ThreadID,
        os_thread_id: DWORD,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_invocation_started(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_sending_message(
        &mut self,
        p_cookie: GUID,
        f_is_async: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_receiving_reply(
        &mut self,
        p_cookie: GUID,
        f_is_async: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_invocation_finished(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_receiving_message(
        &mut self,
        p_cookie: GUID,
        f_is_async: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_invocation_started(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_invocation_returned(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_sending_reply(
        &mut self,
        p_cookie: GUID,
        f_is_async: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn unmanaged_to_managed_transition(
        &mut self,
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn managed_to_unmanaged_transition(
        &mut self,
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_started(
        &mut self,
        suspend_reason: COR_PRF_SUSPEND_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_finished(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_aborted(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_resume_started(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_resume_finished(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_thread_suspended(&mut self, thread_id: ThreadID) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_thread_resumed(&mut self, thread_id: ThreadID) -> Result<(), Error> {
        Ok(())
    }

    fn moved_references(
        &mut self,
        c_moved_object_id_ranges: ULONG,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn object_allocated(&mut self, object_id: ObjectID, class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn objects_allocated_by_class(
        &mut self,
        c_class_count: ULONG,
        class_ids: &[ClassID],
        c_objects: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn object_references(
        &mut self,
        object_id: ObjectID,
        class_id: ClassID,
        c_object_refs: ULONG,
        object_ref_ids: &[ObjectID],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn root_references(
        &mut self,
        c_root_refs: ULONG,
        root_ref_ids: &[ObjectID],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ObjectID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_function_enter(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_function_leave(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_filter_enter(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_filter_leave(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_catcher_found(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_os_handler_enter(&mut self, _unused: UINT_PTR) -> Result<(), Error> {
        Ok(())
    }

    fn exception_os_handler_leave(&mut self, _unused: UINT_PTR) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_function_enter(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_function_leave(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_finally_enter(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_finally_leave(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn exception_catcher_enter(
        &mut self,
        function_id: FunctionID,
        object_id: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn exception_catcher_leave(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn com_classic_vtable_created(
        &mut self,
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: *const c_void,
        c_slots: ULONG,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn com_classic_vtable_destroyed(
        &mut self,
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: *const c_void,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn exception_clr_catcher_found(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn exception_clr_catcher_execute(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback2: CorProfilerCallback {
    fn thread_name_changed(
        &mut self,
        thread_id: ThreadID,
        cch_name: ULONG,
        name: &[WCHAR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_started(
        &mut self,
        c_generations: int,
        generation_collected: &[BOOL],
        reason: COR_PRF_GC_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn surviving_references(
        &mut self,
        c_surviving_object_id_ranges: ULONG,
        object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn finalizeable_object_queued(
        &mut self,
        finalizer_flags: DWORD,
        object_id: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn root_references_2(
        &mut self,
        c_root_refs: ULONG,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        root_ids: &[UINT_PTR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_created(
        &mut self,
        handle_id: GCHandleID,
        initial_object_id: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_destroyed(&mut self, handle_id: GCHandleID) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback3: CorProfilerCallback2 {
    fn initialize_for_attach(
        &mut self,
        p_cor_profiler_info_unk: CorProfilerInfo,
        pv_client_data: *const c_void,
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
pub trait CorProfilerCallback5: CorProfilerCallback4 {
    fn conditional_weak_table_element_references(
        &mut self,
        c_root_refs: ULONG,
        key_ref_ids: &[ObjectID],
        value_ref_ids: &[ObjectID],
        root_ids: &[GCHandleID],
    ) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback6: CorProfilerCallback5 {
    fn get_assembly_references(
        &mut self,
        wsz_assembly_path: WCHAR,
        p_asm_ref_provider: CorProfilerAssemblyReferenceProvider,
    ) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback7: CorProfilerCallback6 {
    fn module_in_memory_symbols_updated(&mut self, module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }
}

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

pub trait CorProfilerCallback9: CorProfilerCallback8 {
    fn dynamic_method_unloaded(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }
}
