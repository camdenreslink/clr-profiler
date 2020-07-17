use crate::errors::Error;
use uuid::Uuid;

type FunctionID = i32;
type IUnknown = i32;
type AppDomainID = i32;
type HRESULT = i32;
type AssemblyID = i32;
type ModuleID = i32;
type ClassID = i32;
type BOOL = i32;
type COR_PRF_JIT_CACHE = i32;
type thread_id = i32;
type DWORD = i32;
type GUID = i32;
type COR_PRF_TRANSITION_REASON = i32;
type COR_PRF_SUSPEND_REASON = i32;
type ObjectID = i32;
type ULONG = i32;
type UINT_PTR = i32;
type REFGUID = i32;
type VOID = i32;
type ThreadID = i32;
type WCHAR = i32;
type int = i32;
type COR_PRF_GC_REASON = i32;
type COR_PRF_GC_ROOT_KIND = i32;
type COR_PRF_GC_ROOT_FLAGS = i32;
type GCHandleID = i32;
type UINT = i32;
type ReJITID = i32;
type mdMethodDef = i32;
type ICorProfilerFunctionControl = i32;
type SIZE_T = i32;
type ICorProfilerAssemblyReferenceProvider = i32;
type LPCBYTE = i32;

pub trait CorProfilerCallback {
    fn initialize(p_icorprofiler_info_unk: IUnknown) -> Result<(), Error>;

    fn shutdown() -> Result<(), Error>;

    fn app_domain_creation_started(app_domain_id: AppDomainID) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_creation_finished(
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_shutdown_started(app_domain_id: AppDomainID) -> Result<(), Error> {
        Ok(())
    }

    fn app_domain_shutdown_finished(
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_load_started(assembly_id: AssemblyID) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_load_finished(assembly_id: AssemblyID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn assembly_unload_started(assembly_id: AssemblyID) -> Result<(), Error> {
        Ok(())
    }

    fn assembly_unload_finished(assembly_id: AssemblyID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_load_started(module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }

    fn module_load_finished(module_id: ModuleID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_unload_started(module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }

    fn module_unload_finished(module_id: ModuleID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_attached_to_assembly(
        module_id: ModuleID,
        assembly_id: AssemblyID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn class_load_started(class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn class_load_finished(class_id: ClassID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn class_unload_started(class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn class_unload_finished(class_id: ClassID, hr_status: HRESULT) -> Result<(), Error> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn function_unload_started(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn jit_compilation_started(
        function_id: FunctionID,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_compilation_finished(
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_cached_function_search_started(
        function_id: FunctionID,
        pb_use_cached_function: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_cached_function_search_finished(
        function_id: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn jit_function_pitched(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn jit_inlining(
        caller_id: FunctionID,
        callee_id: FunctionID,
        pf_should_inline: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn thread_created(thread_id: thread_id) -> Result<(), Error> {
        Ok(())
    }

    fn thread_destroyed(thread_id: thread_id) -> Result<(), Error> {
        Ok(())
    }

    fn thread_assigned_to_os_thread(
        managed_thread_id: thread_id,
        os_thread_id: DWORD,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_invocation_started() -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_sending_message(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_receiving_reply(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_client_invocation_finished() -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_receiving_message(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_invocation_started() -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_invocation_returned() -> Result<(), Error> {
        Ok(())
    }

    fn remoting_server_sending_reply(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error> {
        Ok(())
    }

    fn unmanaged_to_managed_transition(
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn managed_to_unmanaged_transition(
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_started(suspend_reason: COR_PRF_SUSPEND_REASON) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_finished() -> Result<(), Error> {
        Ok(())
    }

    fn runtime_suspend_aborted() -> Result<(), Error> {
        Ok(())
    }

    fn runtime_resume_started() -> Result<(), Error> {
        Ok(())
    }

    fn runtime_resume_finished() -> Result<(), Error> {
        Ok(())
    }

    fn runtime_thread_suspended(thread_id: thread_id) -> Result<(), Error> {
        Ok(())
    }

    fn runtime_thread_resumed(thread_id: thread_id) -> Result<(), Error> {
        Ok(())
    }

    fn moved_references(
        c_moved_object_id_ranges: ULONG,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn object_allocated(object_id: ObjectID, class_id: ClassID) -> Result<(), Error> {
        Ok(())
    }

    fn objects_allocated_by_class(
        c_class_count: ULONG,
        class_ids: &[ClassID],
        c_objects: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn object_references(
        object_id: ObjectID,
        class_id: ClassID,
        c_object_refs: ULONG,
        object_ref_ids: &[ObjectID],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn root_references(c_root_refs: ULONG, root_ref_ids: &[ObjectID]) -> Result<(), Error> {
        Ok(())
    }

    fn exception_thrown(thrown_object_id: ObjectID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_function_enter(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_function_leave() -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_filter_enter(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_filter_leave() -> Result<(), Error> {
        Ok(())
    }

    fn exception_search_catcher_found(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_os_handler_enter(_unused: UINT_PTR) -> Result<(), Error> {
        Ok(())
    }

    fn exception_os_handler_leave(_unused: UINT_PTR) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_function_enter(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_function_leave() -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_finally_enter(function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_unwind_finally_leave() -> Result<(), Error> {
        Ok(())
    }

    fn exception_catcher_enter(function_id: FunctionID, object_id: ObjectID) -> Result<(), Error> {
        Ok(())
    }

    fn exception_catcher_leave() -> Result<(), Error> {
        Ok(())
    }

    fn com_classic_vtable_created(
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: VOID,
        c_slots: ULONG,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn com_classic_vtable_destroyed(
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: VOID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn exception_clr_catcher_found() -> Result<(), Error> {
        Ok(())
    }

    fn exception_clr_catcher_execute() -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback2: CorProfilerCallback {
    fn thread_name_changed(
        thread_id: ThreadID,
        cch_name: ULONG,
        name: &[WCHAR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_started(
        c_generations: int,
        generation_collected: &[BOOL],
        reason: COR_PRF_GC_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn surviving_references(
        c_surviving_object_id_ranges: ULONG,
        object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_finished() -> Result<(), Error> {
        Ok(())
    }

    fn finalizeable_object_queued(
        finalizer_Flags: DWORD,
        object_ID: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn root_references_2(
        c_root_refs: ULONG,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        root_ids: &[UINT_PTR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_created(handle_id: GCHandleID, initial_object_id: ObjectID) -> Result<(), Error> {
        Ok(())
    }

    fn handle_destroyed(handle_id: GCHandleID) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback3: CorProfilerCallback2 {
    fn initialize_for_attach(
        p_cor_profiler_info_unk: IUnknown,
        pv_client_data: VOID,
        cb_client_data: UINT,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn profiler_attach_complete() -> Result<(), Error> {
        Ok(())
    }

    fn profiler_detach_succeeded() -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback4: CorProfilerCallback3 {
    fn rejit_compilation_started(
        function_id: FunctionID,
        rejit_id: ReJITID,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn get_rejit_parameters(
        module_id: ModuleID,
        method_id: mdMethodDef,
        p_function_control: ICorProfilerFunctionControl,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn rejit_compilation_finished(
        function_id: FunctionID,
        rejit_id: ReJITID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn rejit_error(
        module_id: ModuleID,
        method_id: mdMethodDef,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), Error> {
        Ok(())
    }

    fn moved_references_2(
        c_moved_object_id_ranges: ULONG,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[SIZE_T],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn surviving_references_2(
        c_surviving_object_id_ranges: ULONG,
        object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[SIZE_T],
    ) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback5: CorProfilerCallback4 {
    fn conditional_weak_table_element_references(
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
        wsz_assembly_path: WCHAR,
        p_asm_ref_provider: ICorProfilerAssemblyReferenceProvider,
    ) -> Result<(), Error> {
        Ok(())
    }
}
pub trait CorProfilerCallback7: CorProfilerCallback6 {
    fn module_in_memory_symbols_updated(module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }
}

pub trait CorProfilerCallback8: CorProfilerCallback7 {
    fn dynamic_method_jit_compilation_started(
        function_id: FunctionID,
        f_is_safe_to_block: BOOL,
        p_il_header: LPCBYTE,
        c_bil_header: ULONG,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn dynamic_method_jit_compilation_finished(
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
