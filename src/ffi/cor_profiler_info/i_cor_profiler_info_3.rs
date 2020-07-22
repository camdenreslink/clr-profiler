#![allow(non_snake_case)]
use crate::ffi::{
    int, mdFieldDef, mdMethodDef, mdToken, mdTypeDef, AppDomainID, AssemblyID, ClassID, ContextID,
    CorElementType, FunctionEnter, FunctionID, ModuleID, ObjectID, ProcessID, ReJITID, ThreadID,
    BOOL, BYTE, COR_DEBUG_IL_TO_NATIVE_MAP, COR_FIELD_OFFSET, COR_IL_MAP, COR_PRF_CODE_INFO,
    COR_PRF_ELT_INFO, COR_PRF_EX_CLAUSE_INFO, COR_PRF_FRAME_INFO, COR_PRF_FUNCTION_ARGUMENT_INFO,
    COR_PRF_FUNCTION_ARGUMENT_RANGE, COR_PRF_GC_GENERATION_RANGE, COR_PRF_GC_REASON,
    COR_PRF_GC_ROOT_FLAGS, COR_PRF_GC_ROOT_KIND, COR_PRF_JIT_CACHE, COR_PRF_RUNTIME_TYPE,
    COR_PRF_STATIC_TYPE, COR_PRF_SUSPEND_REASON, COR_PRF_TRANSITION_REASON, DWORD, GUID, HANDLE,
    HRESULT, LPCBYTE, REFGUID, SIZE_T, UINT, UINT_PTR, ULONG, ULONG32, USHORT, WCHAR,
};
use std::ffi::c_void;

type ICorProfilerFunctionControl = i32;
type ICorProfilerAssemblyReferenceProvider = i32;
type ICorProfilerObjectEnum = i32;
type ICorProfilerFunctionEnum = i32;
type IMethodMalloc = i32;
type ICorProfilerModuleEnum = i32;

type FunctionLeave = i32;
type FunctionTailcall = i32;
type FunctionIDMapper = i32;
type FunctionEnter2 = i32;
type FunctionLeave2 = i32;
type FunctionTailcall2 = i32;
type FunctionIDMapper2 = i32;
type FunctionEnter3 = i32;
type FunctionLeave3 = i32;
type FunctionTailcall3 = i32;
type FunctionEnter3WithInfo = i32;
type FunctionLeave3WithInfo = i32;
type FunctionTailcall3WithInfo = i32;

type StackSnapshotCallback = i32;

#[repr(C)]
pub struct ICorProfilerInfo3<T> {
    pub EnumJITedFunctions:
        unsafe extern "system" fn(this: &T, ppEnum: *mut *mut ICorProfilerFunctionEnum) -> HRESULT,
    pub RequestProfilerDetach:
        unsafe extern "system" fn(this: &T, dwExpectedCompletionMilliseconds: DWORD) -> HRESULT,
    pub SetFunctionIDMapper2: unsafe extern "system" fn(
        this: &T,
        pFunc: *const FunctionIDMapper2,
        clientData: *const c_void,
    ) -> HRESULT,
    pub GetStringLayout2: unsafe extern "system" fn(
        this: &T,
        pStringLengthOffset: *mut ULONG,
        pBufferOffset: *mut ULONG,
    ) -> HRESULT,
    pub SetEnterLeaveFunctionHooks3: unsafe extern "system" fn(
        this: &T,
        pFuncEnter3: *const FunctionEnter3,
        pFuncLeave3: *const FunctionLeave3,
        pFuncTailcall3: *const FunctionTailcall3,
    ) -> HRESULT,
    pub SetEnterLeaveFunctionHooks3WithInfo: unsafe extern "system" fn(
        this: &T,
        pFuncEnter3WithInfo: *const FunctionEnter3WithInfo,
        pFuncLeave3WithInfo: *const FunctionLeave3WithInfo,
        pFuncTailcall3WithInfo: *const FunctionTailcall3WithInfo,
    ) -> HRESULT,
    pub GetFunctionEnter3Info: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
        pcbArgumentInfo: *mut ULONG,
        pArgumentInfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO,
    ) -> HRESULT,
    pub GetFunctionLeave3Info: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
        pRetvalRange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE,
    ) -> HRESULT,
    pub GetFunctionTailcall3Info: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
    ) -> HRESULT,
    pub EnumModules:
        unsafe extern "system" fn(this: &T, ppEnum: *mut *mut ICorProfilerModuleEnum) -> HRESULT,
    pub GetRuntimeInformation: unsafe extern "system" fn(
        this: &T,
        pClrInstanceId: *mut USHORT,
        pRuntimeType: *mut COR_PRF_RUNTIME_TYPE,
        pMajorVersion: *mut USHORT,
        pMinorVersion: *mut USHORT,
        pBuildNumber: *mut USHORT,
        pQFEVersion: *mut USHORT,
        cchVersionString: *const ULONG,
        pcchVersionString: *mut ULONG,
        szVersionString: *mut WCHAR,
    ) -> HRESULT,
    pub GetThreadStaticAddress2: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        appDomainId: AppDomainID,
        threadId: ThreadID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetAppDomainsContainingModule: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        cAppDomainIds: ULONG32,
        pcAppDomainIds: *mut ULONG32,
        appDomainIds: *mut AppDomainID,
    ) -> HRESULT,
    pub GetModuleInfo2: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        ppBaseLoadAddress: *mut LPCBYTE,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAssemblyId: *mut AssemblyID,
        pdwModuleFlags: *mut DWORD,
    ) -> HRESULT,
}

impl ICorProfilerInfo3<()> {
    // B555ED4F-452A-4E54-8B39-B5360BAD32A0
    pub const iid: GUID = GUID {
        data1: 0xB555ED4F,
        data2: 0x452A,
        data3: 0x4E54,
        data4: [0x8B, 0x39, 0xB5, 0x36, 0x0B, 0xAD, 0x32, 0xA0],
    };
}
