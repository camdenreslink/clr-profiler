#![allow(non_snake_case)]
use super::CorProfilerInfo;
use crate::ffi::{
    int, AppDomainID, AssemblyID, ClassID, FunctionID, GCHandleID, ICorProfilerCallback,
    ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5,
    ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8, ICorProfilerCallback9,
    IUnknown, ModuleID, ObjectID, ThreadID, BOOL, COR_PRF_GC_REASON, COR_PRF_GC_ROOT_FLAGS,
    COR_PRF_GC_ROOT_KIND, COR_PRF_JIT_CACHE, COR_PRF_MONITOR, COR_PRF_SUSPEND_REASON,
    COR_PRF_TRANSITION_REASON, DWORD, E_NOINTERFACE, GUID, HRESULT, LPVOID, REFGUID, REFIID, S_OK,
    UINT_PTR, ULONG, WCHAR,
};
use std::{
    ffi::c_void,
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

#[repr(C)]
pub struct CorProfilerCallbackVtbl<'a> {
    pub IUnknown: IUnknown<CorProfilerCallback<'a>>,
    pub ICorProfilerCallback: ICorProfilerCallback<CorProfilerCallback<'a>>,
    pub ICorProfilerCallback2: ICorProfilerCallback2<CorProfilerCallback<'a>>,
    // pub ICorProfilerCallback3: ICorProfilerCallback3<CorProfilerCallback>,
    // pub ICorProfilerCallback4: ICorProfilerCallback4<CorProfilerCallback>,
    // pub ICorProfilerCallback5: ICorProfilerCallback5<CorProfilerCallback>,
    // pub ICorProfilerCallback6: ICorProfilerCallback6<CorProfilerCallback>,
    // pub ICorProfilerCallback7: ICorProfilerCallback7<CorProfilerCallback>,
    // pub ICorProfilerCallback8: ICorProfilerCallback8<CorProfilerCallback>,
    // pub ICorProfilerCallback9: ICorProfilerCallback9<CorProfilerCallback>
}

#[repr(C)]
pub struct CorProfilerCallback<'a> {
    pub lpVtbl: *const CorProfilerCallbackVtbl<'a>,
    ref_count: AtomicU32,
    cor_profiler_info: Option<&'a CorProfilerInfo>,
}

impl<'a> CorProfilerCallback<'a> {
    pub fn new<'b>() -> &'b mut CorProfilerCallback<'a> {
        let profiler = CorProfilerCallback {
            lpVtbl: &CorProfilerCallbackVtbl {
                IUnknown: IUnknown {
                    QueryInterface: Self::query_interface,
                    AddRef: Self::add_ref,
                    Release: Self::release,
                },
                ICorProfilerCallback: ICorProfilerCallback {
                    Initialize: Self::Initialize,
                    Shutdown: Self::Shutdown,
                    AppDomainCreationStarted: Self::AppDomainCreationStarted,
                    AppDomainCreationFinished: Self::AppDomainCreationFinished,
                    AppDomainShutdownStarted: Self::AppDomainShutdownStarted,
                    AppDomainShutdownFinished: Self::AppDomainShutdownFinished,
                    AssemblyLoadStarted: Self::AssemblyLoadStarted,
                    AssemblyLoadFinished: Self::AssemblyLoadFinished,
                    AssemblyUnloadStarted: Self::AssemblyUnloadStarted,
                    AssemblyUnloadFinished: Self::AssemblyUnloadFinished,
                    ModuleLoadStarted: Self::ModuleLoadStarted,
                    ModuleLoadFinished: Self::ModuleLoadFinished,
                    ModuleUnloadStarted: Self::ModuleUnloadStarted,
                    ModuleUnloadFinished: Self::ModuleUnloadFinished,
                    ModuleAttachedToAssembly: Self::ModuleAttachedToAssembly,
                    ClassLoadStarted: Self::ClassLoadStarted,
                    ClassLoadFinished: Self::ClassLoadFinished,
                    ClassUnloadStarted: Self::ClassUnloadStarted,
                    ClassUnloadFinished: Self::ClassUnloadFinished,
                    FunctionUnloadStarted: Self::FunctionUnloadStarted,
                    JITCompilationStarted: Self::JITCompilationStarted,
                    JITCompilationFinished: Self::JITCompilationFinished,
                    JITCachedFunctionSearchStarted: Self::JITCachedFunctionSearchStarted,
                    JITCachedFunctionSearchFinished: Self::JITCachedFunctionSearchFinished,
                    JITFunctionPitched: Self::JITFunctionPitched,
                    JITInlining: Self::JITInlining,
                    ThreadCreated: Self::ThreadCreated,
                    ThreadDestroyed: Self::ThreadDestroyed,
                    ThreadAssignedToOSThread: Self::ThreadAssignedToOSThread,
                    RemotingClientInvocationStarted: Self::RemotingClientInvocationStarted,
                    RemotingClientSendingMessage: Self::RemotingClientSendingMessage,
                    RemotingClientReceivingReply: Self::RemotingClientReceivingReply,
                    RemotingClientInvocationFinished: Self::RemotingClientInvocationFinished,
                    RemotingServerReceivingMessage: Self::RemotingServerReceivingMessage,
                    RemotingServerInvocationStarted: Self::RemotingServerInvocationStarted,
                    RemotingServerInvocationReturned: Self::RemotingServerInvocationReturned,
                    RemotingServerSendingReply: Self::RemotingServerSendingReply,
                    UnmanagedToManagedTransition: Self::UnmanagedToManagedTransition,
                    ManagedToUnmanagedTransition: Self::ManagedToUnmanagedTransition,
                    RuntimeSuspendStarted: Self::RuntimeSuspendStarted,
                    RuntimeSuspendFinished: Self::RuntimeSuspendFinished,
                    RuntimeSuspendAborted: Self::RuntimeSuspendAborted,
                    RuntimeResumeStarted: Self::RuntimeResumeStarted,
                    RuntimeResumeFinished: Self::RuntimeResumeFinished,
                    RuntimeThreadSuspended: Self::RuntimeThreadSuspended,
                    RuntimeThreadResumed: Self::RuntimeThreadResumed,
                    MovedReferences: Self::MovedReferences,
                    ObjectAllocated: Self::ObjectAllocated,
                    ObjectsAllocatedByClass: Self::ObjectsAllocatedByClass,
                    ObjectReferences: Self::ObjectReferences,
                    RootReferences: Self::RootReferences,
                    ExceptionThrown: Self::ExceptionThrown,
                    ExceptionSearchFunctionEnter: Self::ExceptionSearchFunctionEnter,
                    ExceptionSearchFunctionLeave: Self::ExceptionSearchFunctionLeave,
                    ExceptionSearchFilterEnter: Self::ExceptionSearchFilterEnter,
                    ExceptionSearchFilterLeave: Self::ExceptionSearchFilterLeave,
                    ExceptionSearchCatcherFound: Self::ExceptionSearchCatcherFound,
                    ExceptionOSHandlerEnter: Self::ExceptionOSHandlerEnter,
                    ExceptionOSHandlerLeave: Self::ExceptionOSHandlerLeave,
                    ExceptionUnwindFunctionEnter: Self::ExceptionUnwindFunctionEnter,
                    ExceptionUnwindFunctionLeave: Self::ExceptionUnwindFunctionLeave,
                    ExceptionUnwindFinallyEnter: Self::ExceptionUnwindFinallyEnter,
                    ExceptionUnwindFinallyLeave: Self::ExceptionUnwindFinallyLeave,
                    ExceptionCatcherEnter: Self::ExceptionCatcherEnter,
                    ExceptionCatcherLeave: Self::ExceptionCatcherLeave,
                    COMClassicVTableCreated: Self::COMClassicVTableCreated,
                    COMClassicVTableDestroyed: Self::COMClassicVTableDestroyed,
                    ExceptionCLRCatcherFound: Self::ExceptionCLRCatcherFound,
                    ExceptionCLRCatcherExecute: Self::ExceptionCLRCatcherExecute,
                },
                ICorProfilerCallback2: ICorProfilerCallback2 {
                    ThreadNameChanged: Self::ThreadNameChanged,
                    GarbageCollectionStarted: Self::GarbageCollectionStarted,
                    SurvivingReferences: Self::SurvivingReferences,
                    GarbageCollectionFinished: Self::GarbageCollectionFinished,
                    FinalizeableObjectQueued: Self::FinalizeableObjectQueued,
                    RootReferences2: Self::RootReferences2,
                    HandleCreated: Self::HandleCreated,
                    HandleDestroyed: Self::HandleDestroyed,
                },
            },
            ref_count: AtomicU32::new(1), // TODO: Why does ref_count have to start at 1? Isn't 0 more appropriate? Why is release called by profiling api without calling add_ref?
            cor_profiler_info: None,
        };
        Box::leak(Box::new(profiler))
    }

    // DF63A541-5A33-4611-8829-F4E495985EE3
    pub const CLSID: GUID = GUID {
        data1: 0xDF63A541,
        data2: 0x5A33,
        data3: 0x4611,
        data4: [0x88, 0x29, 0xF4, 0xE4, 0x95, 0x98, 0x5E, 0xE3],
    };
}

// IUnknown
impl<'a> CorProfilerCallback<'a> {
    pub unsafe extern "system" fn query_interface(
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        println!(
            "CorProfilerCallback hit query_interface! Querying riid: {:?}",
            *riid
        );
        if *riid == IUnknown::IID
            || *riid == ICorProfilerCallback::IID
            || *riid == ICorProfilerCallback2::IID
            || *riid == ICorProfilerCallback3::IID
        //|| *riid == ICorProfilerCallback4Com::IID
        //|| *riid == ICorProfilerCallback5Com::IID
        //|| *riid == ICorProfilerCallback6Com::IID
        //|| *riid == ICorProfilerCallback7Com::IID
        //|| *riid == ICorProfilerCallback8Com::IID
        //|| *riid == ICorProfilerCallback9Com::IID
        {
            *ppvObject = self as *mut CorProfilerCallback as LPVOID;
            self.add_ref();
            S_OK
        } else {
            *ppvObject = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    pub unsafe extern "system" fn add_ref(&mut self) -> ULONG {
        println!(
            "CorProfilerCallback hit add_ref! Ref count is: {}",
            self.ref_count.load(Ordering::Relaxed)
        );
        // TODO: Which ordering is appropriate?
        let prev_ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed);
        prev_ref_count + 1
    }

    pub unsafe extern "system" fn release(&mut self) -> ULONG {
        println!(
            "CorProfilerCallback hit release! Ref count is: {}",
            self.ref_count.load(Ordering::Relaxed)
        );
        // Ensure we are not trying to release the memory twice if
        // client calls release despite the ref_count being zero.
        // TODO: Which ordering is appropriate?
        if self.ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Cannot release the COM object, it has already been released.");
        }

        let prev_ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed);
        let ref_count = prev_ref_count - 1;

        if ref_count == 0 {
            drop(Box::from_raw(self as *mut CorProfilerCallback));
        }

        ref_count
    }
}

// ICorProfilerCallback
impl<'a> CorProfilerCallback<'a> {
    unsafe extern "system" fn Initialize(
        &mut self,
        pICorProfilerInfoUnk: *const CorProfilerInfo,
    ) -> HRESULT {
        println!("ICorProfilerCallback::Initialize called!");
        self.cor_profiler_info = pICorProfilerInfoUnk.as_ref();
        let event_mask: DWORD = COR_PRF_MONITOR::COR_PRF_ALL as DWORD;
        if let Some(cor_profiler_info) = self.cor_profiler_info {
            (cor_profiler_info
                .lpVtbl
                .as_ref()
                .unwrap()
                .ICorProfilerInfo
                .SetEventMask)(cor_profiler_info, event_mask);
        }
        S_OK
    }
    unsafe extern "system" fn Shutdown(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::Shutdown called!");
        S_OK
    }
    unsafe extern "system" fn AppDomainCreationStarted(
        &mut self,
        appDomainId: AppDomainID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AppDomainCreationStarted called!");
        S_OK
    }
    unsafe extern "system" fn AppDomainCreationFinished(
        &mut self,
        appDomainId: AppDomainID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AppDomainCreationFinished called!");
        S_OK
    }
    unsafe extern "system" fn AppDomainShutdownStarted(
        &mut self,
        appDomainId: AppDomainID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AppDomainShutdownStarted called!");
        S_OK
    }
    unsafe extern "system" fn AppDomainShutdownFinished(
        &mut self,
        appDomainId: AppDomainID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AppDomainShutdownFinished called!");
        S_OK
    }
    unsafe extern "system" fn AssemblyLoadStarted(&mut self, assemblyId: AssemblyID) -> HRESULT {
        println!("ICorProfilerCallback::AssemblyLoadStarted called!");
        S_OK
    }
    unsafe extern "system" fn AssemblyLoadFinished(
        &mut self,
        assemblyId: AssemblyID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AssemblyLoadFinished called!");
        S_OK
    }
    unsafe extern "system" fn AssemblyUnloadStarted(&mut self, assemblyId: AssemblyID) -> HRESULT {
        println!("ICorProfilerCallback::AssemblyUnloadStarted called!");
        S_OK
    }
    unsafe extern "system" fn AssemblyUnloadFinished(
        &mut self,
        assemblyId: AssemblyID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::AssemblyUnloadFinished called!");
        S_OK
    }
    unsafe extern "system" fn ModuleLoadStarted(&mut self, moduleId: ModuleID) -> HRESULT {
        println!("ICorProfilerCallback::ModuleLoadStarted called!");
        S_OK
    }
    unsafe extern "system" fn ModuleLoadFinished(
        &mut self,
        moduleId: ModuleID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ModuleLoadFinished called!");
        S_OK
    }
    unsafe extern "system" fn ModuleUnloadStarted(&mut self, moduleId: ModuleID) -> HRESULT {
        println!("ICorProfilerCallback::ModuleUnloadStarted called!");
        S_OK
    }
    unsafe extern "system" fn ModuleUnloadFinished(
        &mut self,
        moduleId: ModuleID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ModuleUnloadFinished called!");
        S_OK
    }
    unsafe extern "system" fn ModuleAttachedToAssembly(
        &mut self,
        moduleId: ModuleID,
        AssemblyId: AssemblyID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ModuleAttachedToAssembly called!");
        S_OK
    }
    unsafe extern "system" fn ClassLoadStarted(&mut self, classId: ClassID) -> HRESULT {
        println!("ICorProfilerCallback::ClassLoadStarted called!");
        S_OK
    }
    unsafe extern "system" fn ClassLoadFinished(
        &mut self,
        classId: ClassID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ClassLoadFinished called!");
        S_OK
    }
    unsafe extern "system" fn ClassUnloadStarted(&mut self, classId: ClassID) -> HRESULT {
        println!("ICorProfilerCallback::ClassUnloadStarted called!");
        S_OK
    }
    unsafe extern "system" fn ClassUnloadFinished(
        &mut self,
        classId: ClassID,
        hrStatus: HRESULT,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ClassUnloadFinished called!");
        S_OK
    }
    unsafe extern "system" fn FunctionUnloadStarted(&mut self, functionId: FunctionID) -> HRESULT {
        println!("ICorProfilerCallback::FunctionUnloadStarted called!");
        S_OK
    }
    unsafe extern "system" fn JITCompilationStarted(
        &mut self,
        functionId: FunctionID,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::JITCompilationStarted called!");
        S_OK
    }
    unsafe extern "system" fn JITCompilationFinished(
        &mut self,
        functionId: FunctionID,
        hrStatus: HRESULT,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::JITCompilationFinished called!");
        S_OK
    }
    unsafe extern "system" fn JITCachedFunctionSearchStarted(
        &mut self,
        functionId: FunctionID,
        pbUseCachedFunction: *mut BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::JITCachedFunctionSearchStarted called!");
        S_OK
    }
    unsafe extern "system" fn JITCachedFunctionSearchFinished(
        &mut self,
        functionId: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> HRESULT {
        println!("ICorProfilerCallback::JITCachedFunctionSearchFinished called!");
        S_OK
    }
    unsafe extern "system" fn JITFunctionPitched(&mut self, functionId: FunctionID) -> HRESULT {
        println!("ICorProfilerCallback::JITFunctionPitched called!");
        S_OK
    }
    unsafe extern "system" fn JITInlining(
        &mut self,
        callerId: FunctionID,
        calleeId: FunctionID,
        pfShouldInline: *mut BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::JITInlining called!");
        S_OK
    }
    unsafe extern "system" fn ThreadCreated(&mut self, threadId: ThreadID) -> HRESULT {
        println!("ICorProfilerCallback::ThreadCreated called!");
        S_OK
    }
    unsafe extern "system" fn ThreadDestroyed(&mut self, threadId: ThreadID) -> HRESULT {
        println!("ICorProfilerCallback::ThreadDestroyed called!");
        S_OK
    }
    unsafe extern "system" fn ThreadAssignedToOSThread(
        &mut self,
        managedThreadId: ThreadID,
        osThreadId: DWORD,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ThreadAssignedToOSThread called!");
        S_OK
    }
    unsafe extern "system" fn RemotingClientInvocationStarted(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RemotingClientInvocationStarted called!");
        S_OK
    }
    unsafe extern "system" fn RemotingClientSendingMessage(
        &mut self,
        pCookie: *const GUID,
        fIsAsync: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RemotingClientSendingMessage called!");
        S_OK
    }
    unsafe extern "system" fn RemotingClientReceivingReply(
        &mut self,
        pCookie: *const GUID,
        fIsAsync: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RemotingClientReceivingReply called!");
        S_OK
    }
    unsafe extern "system" fn RemotingClientInvocationFinished(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RemotingClientInvocationFinished called!");
        S_OK
    }
    unsafe extern "system" fn RemotingServerReceivingMessage(
        &mut self,
        pCookie: *const GUID,
        fIsAsync: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RemotingServerReceivingMessage called!");
        S_OK
    }
    unsafe extern "system" fn RemotingServerInvocationStarted(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RemotingServerInvocationStarted called!");
        S_OK
    }
    unsafe extern "system" fn RemotingServerInvocationReturned(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RemotingServerInvocationReturned called!");
        S_OK
    }
    unsafe extern "system" fn RemotingServerSendingReply(
        &mut self,
        pCookie: *const GUID,
        fIsAsync: BOOL,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RemotingServerSendingReply called!");
        S_OK
    }
    unsafe extern "system" fn UnmanagedToManagedTransition(
        &mut self,
        functionId: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> HRESULT {
        println!("ICorProfilerCallback::UnmanagedToManagedTransition called!");
        S_OK
    }
    unsafe extern "system" fn ManagedToUnmanagedTransition(
        &mut self,
        functionId: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ManagedToUnmanagedTransition called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeSuspendStarted(
        &mut self,
        suspendReason: COR_PRF_SUSPEND_REASON,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeSuspendStarted called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeSuspendFinished(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeSuspendFinished called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeSuspendAborted(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeSuspendAborted called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeResumeStarted(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeResumeStarted called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeResumeFinished(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeResumeFinished called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeThreadSuspended(&mut self, threadId: ThreadID) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeThreadSuspended called!");
        S_OK
    }
    unsafe extern "system" fn RuntimeThreadResumed(&mut self, threadId: ThreadID) -> HRESULT {
        println!("ICorProfilerCallback::RuntimeThreadResumed called!");
        S_OK
    }
    unsafe extern "system" fn MovedReferences(
        &mut self,
        cMovedObjectIDRanges: ULONG,
        oldObjectIDRangeStart: *const ObjectID,
        newObjectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const ULONG,
    ) -> HRESULT {
        println!("ICorProfilerCallback::MovedReferences called!");
        S_OK
    }
    unsafe extern "system" fn ObjectAllocated(
        &mut self,
        objectId: ObjectID,
        classId: ClassID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ObjectAllocated called!");
        S_OK
    }
    unsafe extern "system" fn ObjectsAllocatedByClass(
        &mut self,
        cClassCount: ULONG,
        classIds: *const ClassID,
        cObjects: *const ULONG,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ObjectsAllocatedByClass called!");
        S_OK
    }
    unsafe extern "system" fn ObjectReferences(
        &mut self,
        objectId: ObjectID,
        classId: ClassID,
        cObjectRefs: ULONG,
        objectRefIds: *const ObjectID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ObjectReferences called!");
        S_OK
    }
    unsafe extern "system" fn RootReferences(
        &mut self,
        cRootRefs: ULONG,
        rootRefIds: *const ObjectID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::RootReferences called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionThrown(&mut self, thrownObjectId: ObjectID) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionThrown called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionSearchFunctionEnter(
        &mut self,
        functionId: FunctionID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionSearchFunctionEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionSearchFunctionLeave(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionSearchFunctionLeave called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionSearchFilterEnter(
        &mut self,
        functionId: FunctionID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionSearchFilterEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionSearchFilterLeave(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionSearchFilterLeave called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionSearchCatcherFound(
        &mut self,
        functionId: FunctionID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionSearchCatcherFound called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionOSHandlerEnter(&mut self, __unused: UINT_PTR) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionOSHandlerEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionOSHandlerLeave(&mut self, __unused: UINT_PTR) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionOSHandlerLeave called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionUnwindFunctionEnter(
        &mut self,
        functionId: FunctionID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionUnwindFunctionEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionUnwindFunctionLeave(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionUnwindFunctionLeave called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionUnwindFinallyEnter(
        &mut self,
        functionId: FunctionID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionUnwindFinallyEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionUnwindFinallyLeave(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionUnwindFinallyLeave called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionCatcherEnter(
        &mut self,
        functionId: FunctionID,
        objectId: ObjectID,
    ) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionCatcherEnter called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionCatcherLeave(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionCatcherLeave called!");
        S_OK
    }
    unsafe extern "system" fn COMClassicVTableCreated(
        &mut self,
        wrappedClassId: ClassID,
        implementedIID: REFGUID,
        pVTable: *const c_void,
        cSlots: ULONG,
    ) -> HRESULT {
        println!("ICorProfilerCallback::COMClassicVTableCreated called!");
        S_OK
    }
    unsafe extern "system" fn COMClassicVTableDestroyed(
        &mut self,
        wrappedClassId: ClassID,
        implementedIID: REFGUID,
        pVTable: *const c_void,
    ) -> HRESULT {
        println!("ICorProfilerCallback::COMClassicVTableDestroyed called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionCLRCatcherFound(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionCLRCatcherFound called!");
        S_OK
    }
    unsafe extern "system" fn ExceptionCLRCatcherExecute(&mut self) -> HRESULT {
        println!("ICorProfilerCallback::ExceptionCLRCatcherExecute called!");
        S_OK
    }
}

// ICorProfilerCallback2
impl<'a> CorProfilerCallback<'a> {
    unsafe extern "system" fn ThreadNameChanged(
        &mut self,
        threadId: ThreadID,
        cchName: ULONG,
        name: *const WCHAR,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::ThreadNameChanged called!");
        S_OK
    }
    unsafe extern "system" fn GarbageCollectionStarted(
        &mut self,
        cGenerations: int,
        generationCollected: *const BOOL,
        reason: COR_PRF_GC_REASON,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::GarbageCollectionStarted called!");
        S_OK
    }
    unsafe extern "system" fn SurvivingReferences(
        &mut self,
        cSurvivingObjectIDRanges: ULONG,
        objectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const ULONG,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::SurvivingReferences called!");
        S_OK
    }
    unsafe extern "system" fn GarbageCollectionFinished(&mut self) -> HRESULT {
        println!("ICorProfilerCallback2::GarbageCollectionFinished called!");
        S_OK
    }
    unsafe extern "system" fn FinalizeableObjectQueued(
        &mut self,
        finalizerFlags: DWORD,
        objectID: ObjectID,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::FinalizeableObjectQueued called!");
        S_OK
    }
    unsafe extern "system" fn RootReferences2(
        &mut self,
        cRootRefs: ULONG,
        rootRefIds: *const ObjectID,
        rootKinds: *const COR_PRF_GC_ROOT_KIND,
        rootFlags: *const COR_PRF_GC_ROOT_FLAGS,
        rootIds: *const UINT_PTR,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::RootReferences2 called!");
        S_OK
    }
    unsafe extern "system" fn HandleCreated(
        &mut self,
        handleId: GCHandleID,
        initialObjectId: ObjectID,
    ) -> HRESULT {
        println!("ICorProfilerCallback2::HandleCreated called!");
        S_OK
    }
    unsafe extern "system" fn HandleDestroyed(&mut self, handleId: GCHandleID) -> HRESULT {
        println!("ICorProfilerCallback2::HandleDestroyed called!");
        S_OK
    }
}
