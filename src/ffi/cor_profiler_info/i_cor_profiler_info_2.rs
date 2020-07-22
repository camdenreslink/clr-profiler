#![allow(non_snake_case)]
use crate::ffi::{GUID, HRESULT};
use std::ffi::c_void;

type FunctionID = i32;
type IUnknown = i32;
type AppDomainID = i32;
type AssemblyID = i32;
type ModuleID = i32;
type ClassID = i32;
type BOOL = i32;
type COR_PRF_JIT_CACHE = i32;
type thread_id = i32;
type DWORD = i32;
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
type mdTypeDef = i32;
type mdToken = i32;
type HANDLE = i32;
type CorElementType = i32;
type FunctionEnter = i32;
type FunctionLeave = i32;
type FunctionTailcall = i32;
type FunctionIDMapper = i32;
type IMethodMalloc = i32;
type ProcessID = i32;
type COR_IL_MAP = i32;
type ContextID = i32;
type ULONG32 = i32;
type COR_DEBUG_IL_TO_NATIVE_MAP = i32;
type BYTE = i32;
type StackSnapshotCallback = i32;
type FunctionEnter2 = i32;
type FunctionLeave2 = i32;
type FunctionTailcall2 = i32;
type COR_PRF_FRAME_INFO = i32;
type COR_FIELD_OFFSET = i32;
type COR_PRF_CODE_INFO = i32;
type ICorProfilerObjectEnum = i32;
type mdFieldDef = i32;
type COR_PRF_STATIC_TYPE = i32;
type COR_PRF_GC_GENERATION_RANGE = i32;
type COR_PRF_EX_CLAUSE_INFO = i32;

#[repr(C)]
pub struct ICorProfilerInfo2<T> {
    pub DoStackSnapshot: unsafe extern "system" fn(
        this: &T,
        thread: ThreadID,
        callback: *const StackSnapshotCallback,
        infoFlags: ULONG32,
        clientData: *const c_void,
        context: *const BYTE,
        contextSize: ULONG32,
    ) -> HRESULT,
    pub SetEnterLeaveFunctionHooks2: unsafe extern "system" fn(
        this: &T,
        pFuncEnter: *const FunctionEnter2,
        pFuncLeave: *const FunctionLeave2,
        pFuncTailcall: *const FunctionTailcall2,
    ) -> HRESULT,
    pub GetFunctionInfo2: unsafe extern "system" fn(
        this: &T,
        funcId: FunctionID,
        frameInfo: COR_PRF_FRAME_INFO,
        pClassId: *mut ClassID,
        pModuleId: *mut ModuleID,
        pToken: *mut mdToken,
        cTypeArgs: ULONG32,
        pcTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT,
    pub GetStringLayout: unsafe extern "system" fn(
        this: &T,
        pBufferLengthOffset: *mut ULONG,
        pStringLengthOffset: *mut ULONG,
        pBufferOffset: *mut ULONG,
    ) -> HRESULT,
    pub GetClassLayout: unsafe extern "system" fn(
        this: &T,
        classID: ClassID,
        rFieldOffset: *mut COR_FIELD_OFFSET,
        cFieldOffset: ULONG,
        pcFieldOffset: *mut ULONG,
        pulClassSize: *mut ULONG,
    ) -> HRESULT,
    pub GetClassIDInfo2: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pModuleId: *mut ModuleID,
        pTypeDefToken: *mut mdTypeDef,
        pParentClassId: *mut ClassID,
        cNumTypeArgs: ULONG32,
        pcNumTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT,
    pub GetCodeInfo2: unsafe extern "system" fn(
        this: &T,
        functionID: FunctionID,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT,
    pub GetClassFromTokenAndTypeArgs: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        typeDef: mdTypeDef,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pClassID: *mut ClassID,
    ) -> HRESULT,
    pub GetFunctionFromTokenAndTypeArgs: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        funcDef: mdMethodDef,
        classId: ClassID,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pFunctionID: *mut FunctionID,
    ) -> HRESULT,
    pub EnumModuleFrozenObjects: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        ppEnum: *mut *mut ICorProfilerObjectEnum,
    ) -> HRESULT,
    pub GetArrayObjectInfo: unsafe extern "system" fn(
        this: &T,
        objectId: ObjectID,
        cDimensions: ULONG32,
        pDimensionSizes: *mut ULONG32,
        pDimensionLowerBounds: *mut int,
        ppData: *mut *mut BYTE,
    ) -> HRESULT,
    pub GetBoxClassLayout: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pBufferOffset: *mut ULONG32,
    ) -> HRESULT,
    pub GetThreadAppDomain: unsafe extern "system" fn(
        this: &T,
        threadId: ThreadID,
        pAppDomainId: *mut AppDomainID,
    ) -> HRESULT,
    pub GetRVAStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetAppDomainStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        appDomainId: AppDomainID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetThreadStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        threadId: ThreadID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetContextStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        contextId: ContextID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetStaticFieldInfo: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        pFieldInfo: *mut COR_PRF_STATIC_TYPE,
    ) -> HRESULT,
    pub GetGenerationBounds: unsafe extern "system" fn(
        this: &T,
        cObjectRanges: ULONG,
        pcObjectRanges: *mut ULONG,
        ranges: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT,
    pub GetObjectGeneration: unsafe extern "system" fn(
        this: &T,
        objectId: ObjectID,
        range: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT,
    pub GetNotifiedExceptionClauseInfo:
        unsafe extern "system" fn(this: &T, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> HRESULT,
}

impl ICorProfilerInfo2<()> {
    // CC0935CD-A518-487d-B0BB-A93214E65478
    pub const iid: GUID = GUID {
        data1: 0xCC0935CD,
        data2: 0xA518,
        data3: 0x487d,
        data4: [0xB0, 0xBB, 0xA9, 0x32, 0x14, 0xE6, 0x54, 0x78],
    };
}
