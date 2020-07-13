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

pub trait Unknown {
    fn query_interface(&mut self, interface_identifier: &Uuid) -> Result<&mut Self, Error>;
}

pub trait ClassFactory: Unknown {
    type Item;
    fn create_instance(&mut self, interface_identifier: &Uuid) -> Result<Self::Item, Error>;
}

pub trait CorProfilerCallback {
    // "176FBED1-A55C-4796-98CA-A9DA0EF883E7"
    fn initialize(p_icorprofiler_info_unk: IUnknown) -> Result<(), Error>;

    fn shutdown() -> Result<(), Error>;

    fn app_domain_creation_started(app_domain_id: AppDomainID) -> Result<(), Error>;

    fn app_domain_creation_finished(
        app_domain_id: AppDomainID,
        hr_status: HRESULT,
    ) -> Result<(), Error>;

    fn app_domain_shutdown_started(app_domain_id: AppDomainID) -> Result<(), Error>;

    fn app_domain_shutdown_finished(
        app_domain_id: AppDomainID,
        hr_status: HRESULT,
    ) -> Result<(), Error>;

    fn assembly_load_started(assembly_id: AssemblyID) -> Result<(), Error>;

    fn assembly_load_finished(assembly_id: AssemblyID, hr_status: HRESULT) -> Result<(), Error>;

    fn assembly_unload_started(assembly_id: AssemblyID) -> Result<(), Error>;

    fn assembly_unload_finished(assembly_id: AssemblyID, hr_status: HRESULT) -> Result<(), Error>;

    fn module_load_started(module_id: ModuleID) -> Result<(), Error>;

    fn module_load_finished(module_id: ModuleID, hr_status: HRESULT) -> Result<(), Error>;

    fn module_unload_started(module_id: ModuleID) -> Result<(), Error>;

    fn module_unload_finished(module_id: ModuleID, hr_status: HRESULT) -> Result<(), Error>;

    fn module_attached_to_assembly(
        module_id: ModuleID,
        assembly_id: AssemblyID,
    ) -> Result<(), Error>;

    fn class_load_started(class_id: ClassID) -> Result<(), Error>;

    fn class_load_finished(class_id: ClassID, hr_status: HRESULT) -> Result<(), Error>;

    fn class_unload_started(class_id: ClassID) -> Result<(), Error>;

    fn class_unload_finished(class_id: ClassID, hr_status: HRESULT) -> Result<(), Error>;

    fn function_unload_started(function_id: FunctionID) -> Result<(), Error>;

    fn jit_compilation_started(
        function_id: FunctionID,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error>;

    fn jit_compilation_finished(
        function_id: FunctionID,
        hr_status: HRESULT,
        f_is_safe_to_block: BOOL,
    ) -> Result<(), Error>;

    fn jit_cached_function_search_started(
        function_id: FunctionID,
        pb_use_cached_function: BOOL,
    ) -> Result<(), Error>;

    fn jit_cached_function_search_finished(
        function_id: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> Result<(), Error>;

    fn jit_function_pitched(function_id: FunctionID) -> Result<(), Error>;

    fn jit_inlining(
        caller_id: FunctionID,
        callee_id: FunctionID,
        pf_should_inline: BOOL,
    ) -> Result<(), Error>;

    fn thread_created(thread_id: thread_id) -> Result<(), Error>;

    fn thread_destroyed(thread_id: thread_id) -> Result<(), Error>;

    fn thread_assigned_to_os_thread(
        managed_thread_id: thread_id,
        os_thread_id: DWORD,
    ) -> Result<(), Error>;

    fn remoting_client_invocation_started() -> Result<(), Error>;

    fn remoting_client_sending_message(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error>;

    fn remoting_client_receiving_reply(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error>;

    fn remoting_client_invocation_finished() -> Result<(), Error>;

    fn remoting_server_receiving_message(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error>;

    fn remoting_server_invocation_started() -> Result<(), Error>;

    fn remoting_server_invocation_returned() -> Result<(), Error>;

    fn remoting_server_sending_reply(p_cookie: GUID, f_is_async: BOOL) -> Result<(), Error>;

    fn unmanaged_to_managed_transition(
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error>;

    fn managed_to_unmanaged_transition(
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), Error>;

    fn runtime_suspend_started(suspend_reason: COR_PRF_SUSPEND_REASON) -> Result<(), Error>;

    fn runtime_suspend_finished() -> Result<(), Error>;

    fn runtime_suspend_aborted() -> Result<(), Error>;

    fn runtime_resume_started() -> Result<(), Error>;

    fn runtime_resume_finished() -> Result<(), Error>;

    fn runtime_thread_suspended(thread_id: thread_id) -> Result<(), Error>;

    fn runtime_thread_resumed(thread_id: thread_id) -> Result<(), Error>;

    fn moved_references(
        c_moved_object_id_ranges: ULONG,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error>;

    fn object_allocated(object_id: ObjectID, class_id: ClassID) -> Result<(), Error>;

    fn objects_allocated_by_class(
        c_class_count: ULONG,
        class_ids: &[ClassID],
        c_objects: &[ULONG],
    ) -> Result<(), Error>;

    fn object_references(
        object_id: ObjectID,
        class_id: ClassID,
        c_object_refs: ULONG,
        object_ref_ids: &[ObjectID],
    ) -> Result<(), Error>;

    fn root_references(c_root_refs: ULONG, root_ref_ids: &[ObjectID]) -> Result<(), Error>;

    fn exception_thrown(thrown_object_id: ObjectID) -> Result<(), Error>;

    fn exception_search_function_enter(function_id: FunctionID) -> Result<(), Error>;

    fn exception_search_function_leave() -> Result<(), Error>;

    fn exception_search_filter_enter(function_id: FunctionID) -> Result<(), Error>;

    fn exception_search_filter_leave() -> Result<(), Error>;

    fn exception_search_catcher_found(function_id: FunctionID) -> Result<(), Error>;

    fn exception_os_handler_enter(_unused: UINT_PTR) -> Result<(), Error>;

    fn exception_os_handler_leave(_unused: UINT_PTR) -> Result<(), Error>;

    fn exception_unwind_function_enter(function_id: FunctionID) -> Result<(), Error>;

    fn exception_unwind_function_leave() -> Result<(), Error>;

    fn exception_unwind_finally_enter(function_id: FunctionID) -> Result<(), Error>;

    fn exception_unwind_finally_leave() -> Result<(), Error>;

    fn exception_catcher_enter(function_id: FunctionID, object_id: ObjectID) -> Result<(), Error>;

    fn exception_catcher_leave() -> Result<(), Error>;

    fn com_classic_vtable_created(
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: VOID,
        c_slots: ULONG,
    ) -> Result<(), Error>;

    fn com_classic_vtable_destroyed(
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: VOID,
    ) -> Result<(), Error>;

    fn exception_clr_catcher_found() -> Result<(), Error>;

    fn exception_clr_catcher_execute() -> Result<(), Error>;
}
pub trait CorProfilerCallback2: CorProfilerCallback {}
pub trait CorProfilerCallback3: CorProfilerCallback2 {}
pub trait CorProfilerCallback4: CorProfilerCallback3 {}
pub trait CorProfilerCallback5: CorProfilerCallback4 {}
pub trait CorProfilerCallback6: CorProfilerCallback5 {}
pub trait CorProfilerCallback7: CorProfilerCallback6 {}

pub trait CorProfilerCallback8: CorProfilerCallback7 {}

pub trait CorProfilerCallback9: CorProfilerCallback8 {
    fn dynamic_method_unloaded(&mut self, function_id: FunctionID) -> Result<(), Error>;
}
