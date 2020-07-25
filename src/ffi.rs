#![allow(non_camel_case_types, non_snake_case)]
mod class;
mod hresult;
mod interface;

pub use self::class::*;
pub use self::hresult::*;
pub use self::interface::*;

use core::ffi::c_void;

// numeric types
pub type c_int = i32;
pub type c_long = i32;
pub type c_uint = u32;
pub type c_ulong = u32;
pub type c_ushort = u16;
pub type c_uchar = u8;
pub type int = c_int;
pub type BOOL = c_int;
pub type USHORT = c_ushort;
pub type UINT = c_uint;
pub type ULONG32 = c_uint;
pub type ULONG = c_ulong;
pub type DWORD = c_ulong;
pub type BYTE = c_uchar;

// char types
pub type wchar_t = u16;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;

// pointer types
pub type UINT_PTR = usize;
pub type ULONG_PTR = usize;
pub type LPCBYTE = *const BYTE;
pub type SIZE_T = ULONG_PTR;
pub type LPVOID = *mut c_void;
pub type HANDLE = *mut c_void;

// guid types
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GUID {
    pub data1: c_ulong,
    pub data2: c_ushort,
    pub data3: c_ushort,
    pub data4: [c_uchar; 8],
}
pub type IID = GUID;
pub type REFGUID = *const GUID;
pub type REFCLSID = *const IID;
pub type REFIID = *const IID;

// profiler-specific pointers
pub type ObjectID = UINT_PTR;
pub type FunctionID = UINT_PTR;
pub type AppDomainID = UINT_PTR;
pub type AssemblyID = UINT_PTR;
pub type ModuleID = UINT_PTR;
pub type ClassID = UINT_PTR;
pub type ThreadID = UINT_PTR;
pub type GCHandleID = UINT_PTR;
pub type ReJITID = UINT_PTR;
pub type ProcessID = UINT_PTR;
pub type ContextID = UINT_PTR;
pub type COR_PRF_FRAME_INFO = UINT_PTR;
pub type COR_PRF_ELT_INFO = UINT_PTR;
pub type COR_SIGNATURE = BYTE;
pub type PCCOR_SIGNATURE = *const COR_SIGNATURE;
#[repr(C)]
pub union FunctionIDOrClientID {
    functionID: FunctionID,
    clientID: UINT_PTR,
}

// token types
pub type mdToken = ULONG32;
pub type mdMethodDef = mdToken;
pub type mdTypeDef = mdToken;
pub type mdFieldDef = mdToken;

// function pointer types
pub type FunctionEnter = unsafe extern "system" fn(funcID: FunctionID) -> ();
pub type FunctionLeave = unsafe extern "system" fn(funcID: FunctionID) -> ();
pub type FunctionTailcall = unsafe extern "system" fn(funcID: FunctionID) -> ();
pub type FunctionIDMapper =
    unsafe extern "system" fn(funcId: FunctionID, pbHookFunction: *mut BOOL) -> UINT_PTR;
pub type FunctionEnter2 = unsafe extern "system" fn(
    funcId: FunctionID,
    clientData: UINT_PTR,
    func: COR_PRF_FRAME_INFO,
    argumentInfo: *const COR_PRF_FUNCTION_ARGUMENT_INFO,
) -> ();
pub type FunctionLeave2 = unsafe extern "system" fn(
    funcId: FunctionID,
    clientData: UINT_PTR,
    func: COR_PRF_FRAME_INFO,
    retvalRange: *const COR_PRF_FUNCTION_ARGUMENT_RANGE,
) -> ();
pub type FunctionTailcall2 = unsafe extern "system" fn(
    funcId: FunctionID,
    clientData: UINT_PTR,
    func: COR_PRF_FRAME_INFO,
) -> ();
pub type FunctionIDMapper2 = unsafe extern "system" fn(
    funcId: FunctionID,
    clientData: *const c_void,
    pbHookFunction: *mut BOOL,
) -> UINT_PTR;
pub type FunctionEnter3 =
    unsafe extern "system" fn(functionIDOrClientID: FunctionIDOrClientID) -> ();
pub type FunctionLeave3 =
    unsafe extern "system" fn(functionIDOrClientID: FunctionIDOrClientID) -> ();

pub type FunctionTailcall3 =
    unsafe extern "system" fn(functionIDOrClientID: FunctionIDOrClientID) -> ();

pub type FunctionEnter3WithInfo = unsafe extern "system" fn(
    functionIDOrClientID: FunctionIDOrClientID,
    eltInfo: COR_PRF_ELT_INFO,
) -> ();

pub type FunctionLeave3WithInfo = unsafe extern "system" fn(
    functionIDOrClientID: FunctionIDOrClientID,
    eltInfo: COR_PRF_ELT_INFO,
) -> ();

pub type FunctionTailcall3WithInfo = unsafe extern "system" fn(
    functionIDOrClientID: FunctionIDOrClientID,
    eltInfo: COR_PRF_ELT_INFO,
) -> ();
pub type StackSnapshotCallback = unsafe extern "system" fn(
    funcId: FunctionID,
    ip: UINT_PTR,
    frameInfo: COR_PRF_FRAME_INFO,
    contextSize: ULONG32,
    context: *const BYTE,
    clientData: *const c_void,
) -> HRESULT;
pub type ObjectReferenceCallback = unsafe extern "system" fn(
    root: ObjectID,
    reference: *const ObjectID,
    clientData: *const c_void,
) -> BOOL;

// profiler types
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_JIT_CACHE {
    COR_PRF_CACHED_FUNCTION_FOUND = 0,
    COR_PRF_CACHED_FUNCTION_NOT_FOUND = 1,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_TRANSITION_REASON {
    COR_PRF_TRANSITION_CALL = 0,
    COR_PRF_TRANSITION_RETURN = 1,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_SUSPEND_REASON {
    COR_PRF_SUSPEND_OTHER = 0,
    COR_PRF_SUSPEND_FOR_GC = 1,
    COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN = 2,
    COR_PRF_SUSPEND_FOR_CODE_PITCHING = 3,
    COR_PRF_SUSPEND_FOR_SHUTDOWN = 4,
    COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER = 6,
    COR_PRF_SUSPEND_FOR_GC_PREP = 7,
    COR_PRF_SUSPEND_FOR_REJIT = 8,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_GC_REASON {
    COR_PRF_GC_INDUCED = 1,
    COR_PRF_GC_OTHER = 0,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_GC_ROOT_KIND {
    COR_PRF_GC_ROOT_STACK = 1,
    COR_PRF_GC_ROOT_FINALIZER = 2,
    COR_PRF_GC_ROOT_HANDLE = 3,
    COR_PRF_GC_ROOT_OTHER = 0,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_GC_ROOT_FLAGS {
    COR_PRF_GC_ROOT_PINNING = 1,
    COR_PRF_GC_ROOT_WEAKREF = 2,
    COR_PRF_GC_ROOT_INTERIOR = 4,
    COR_PRF_GC_ROOT_REFCOUNTED = 8,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_IL_MAP {
    pub oldOffset: ULONG32,
    pub newOffset: ULONG32,
    pub fAccurate: BOOL,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    pub ilOffset: ULONG32,
    pub nativeStartOffset: ULONG32,
    pub nativeEndOffset: ULONG32,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_FIELD_OFFSET {
    pub ridOfField: mdFieldDef,
    pub ulOffset: ULONG,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_CODE_INFO {
    pub startAddress: UINT_PTR,
    pub size: SIZE_T,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_STATIC_TYPE {
    COR_PRF_FIELD_NOT_A_STATIC = 0,
    COR_PRF_FIELD_APP_DOMAIN_STATIC = 1,
    COR_PRF_FIELD_THREAD_STATIC = 2,
    COR_PRF_FIELD_CONTEXT_STATIC = 4,
    COR_PRF_FIELD_RVA_STATIC = 8,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_GC_GENERATION {
    COR_PRF_GC_GEN_0 = 0,
    COR_PRF_GC_GEN_1 = 1,
    COR_PRF_GC_GEN_2 = 2,
    COR_PRF_GC_LARGE_OBJECT_HEAP = 3,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_GC_GENERATION_RANGE {
    pub generation: COR_PRF_GC_GENERATION,
    pub rangeStart: ObjectID,
    pub rangeLength: UINT_PTR,
    pub rangeLengthReserved: UINT_PTR,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_CLAUSE_TYPE {
    COR_PRF_CLAUSE_NONE = 0,
    COR_PRF_CLAUSE_FILTER = 1,
    COR_PRF_CLAUSE_CATCH = 2,
    COR_PRF_CLAUSE_FINALLY = 3,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_EX_CLAUSE_INFO {
    pub clauseType: COR_PRF_CLAUSE_TYPE,
    pub programCounter: UINT_PTR,
    pub framePointer: UINT_PTR,
    pub shadowStackPointer: UINT_PTR,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    pub startAddress: UINT_PTR,
    pub length: ULONG,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    pub numRanges: ULONG,
    pub totalArgumentSize: ULONG,
    pub ranges: [COR_PRF_FUNCTION_ARGUMENT_RANGE; 1],
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_RUNTIME_TYPE {
    COR_PRF_DESKTOP_CLR = 1,
    COR_PRF_CORE_CLR = 2,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum CorElementType {
    ELEMENT_TYPE_END = 0x00,
    ELEMENT_TYPE_VOID = 0x01,
    ELEMENT_TYPE_BOOLEAN = 0x02,
    ELEMENT_TYPE_CHAR = 0x03,
    ELEMENT_TYPE_I1 = 0x04,
    ELEMENT_TYPE_U1 = 0x05,
    ELEMENT_TYPE_I2 = 0x06,
    ELEMENT_TYPE_U2 = 0x07,
    ELEMENT_TYPE_I4 = 0x08,
    ELEMENT_TYPE_U4 = 0x09,
    ELEMENT_TYPE_I8 = 0x0a,
    ELEMENT_TYPE_U8 = 0x0b,
    ELEMENT_TYPE_R4 = 0x0c,
    ELEMENT_TYPE_R8 = 0x0d,
    ELEMENT_TYPE_STRING = 0x0e,

    // every type above PTR will be simple type
    ELEMENT_TYPE_PTR = 0x0f,   // PTR <type>
    ELEMENT_TYPE_BYREF = 0x10, // BYREF <type>

    // Please use ELEMENT_TYPE_VALUETYPE. ELEMENT_TYPE_VALUECLASS is deprecated.
    ELEMENT_TYPE_VALUETYPE = 0x11,   // VALUETYPE <class Token>
    ELEMENT_TYPE_CLASS = 0x12,       // CLASS <class Token>
    ELEMENT_TYPE_VAR = 0x13,         // a class type variable VAR <number>
    ELEMENT_TYPE_ARRAY = 0x14, // MDARRAY <type> <rank> <bcount> <bound1> ... <lbcount> <lb1> ...
    ELEMENT_TYPE_GENERICINST = 0x15, // GENERICINST <generic type> <argCnt> <arg1> ... <argn>
    ELEMENT_TYPE_TYPEDBYREF = 0x16, // TYPEDREF  (it takes no args) a typed referece to some other type

    ELEMENT_TYPE_I = 0x18,       // native integer size
    ELEMENT_TYPE_U = 0x19,       // native unsigned integer size
    ELEMENT_TYPE_FNPTR = 0x1b, // FNPTR <complete sig for the function including calling convention>
    ELEMENT_TYPE_OBJECT = 0x1c, // Shortcut for System.Object
    ELEMENT_TYPE_SZARRAY = 0x1d, // Shortcut for single dimension zero lower bound array
    // SZARRAY <type>
    ELEMENT_TYPE_MVAR = 0x1e, // a method type variable MVAR <number>

    // This is only for binding
    ELEMENT_TYPE_CMOD_REQD = 0x1f, // required C modifier : E_T_CMOD_REQD <mdTypeRef/mdTypeDef>
    ELEMENT_TYPE_CMOD_OPT = 0x20,  // optional C modifier : E_T_CMOD_OPT <mdTypeRef/mdTypeDef>

    // This is for signatures generated internally (which will not be persisted in any way).
    ELEMENT_TYPE_INTERNAL = 0x21, // INTERNAL <typehandle>

    // Note that this is the max of base type excluding modifiers
    ELEMENT_TYPE_MAX = 0x22, // first invalid element type

    ELEMENT_TYPE_MODIFIER = 0x40,
    ELEMENT_TYPE_SENTINEL = 0x01 | CorElementType::ELEMENT_TYPE_MODIFIER as isize, // sentinel for varargs
    ELEMENT_TYPE_PINNED = 0x05 | CorElementType::ELEMENT_TYPE_MODIFIER as isize,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct OSINFO {
    dwOSPlatformId: DWORD,   // Operating system platform.
    dwOSMajorVersion: DWORD, // OS Major version.
    dwOSMinorVersion: DWORD, // OS Minor version.
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ASSEMBLYMETADATA {
    usMajorVersion: USHORT,   // Major Version.
    usMinorVersion: USHORT,   // Minor Version.
    usBuildNumber: USHORT,    // Build Number.
    usRevisionNumber: USHORT, // Revision Number.
    szLocale: LPWSTR,         // Locale.
    cbLocale: ULONG,          // [IN/OUT] Size of the buffer in wide chars/Actual size.
    rProcessor: *const DWORD, // Processor ID array.
    ulProcessor: ULONG, // [IN/OUT] Size of the Processor ID array/Actual # of entries filled in.
    rOS: *const OSINFO, // OSINFO array.
    ulOS: ULONG,        // [IN/OUT]Size of the OSINFO array/Actual # of entries filled in.
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pbPublicKeyOrToken: *const c_void,
    cbPublicKeyOrToken: ULONG,
    szName: LPCWSTR,
    pMetaData: *const ASSEMBLYMETADATA,
    pbHashValue: *const c_void,
    cbHashValue: ULONG,
    dwAssemblyRefFlags: DWORD,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_FUNCTION {
    functionId: FunctionID,
    reJitId: ReJITID,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_MONITOR {
    COR_PRF_MONITOR_NONE = 0,
    COR_PRF_MONITOR_FUNCTION_UNLOADS = 0x1,
    COR_PRF_MONITOR_CLASS_LOADS = 0x2,
    COR_PRF_MONITOR_MODULE_LOADS = 0x4,
    COR_PRF_MONITOR_ASSEMBLY_LOADS = 0x8,
    COR_PRF_MONITOR_APPDOMAIN_LOADS = 0x10,
    COR_PRF_MONITOR_JIT_COMPILATION = 0x20,
    COR_PRF_MONITOR_EXCEPTIONS = 0x40,
    COR_PRF_MONITOR_GC = 0x80,
    COR_PRF_MONITOR_OBJECT_ALLOCATED = 0x100,
    COR_PRF_MONITOR_THREADS = 0x200,
    COR_PRF_MONITOR_REMOTING = 0x400,
    COR_PRF_MONITOR_CODE_TRANSITIONS = 0x800,
    COR_PRF_MONITOR_ENTERLEAVE = 0x1000,
    COR_PRF_MONITOR_CCW = 0x2000,
    COR_PRF_MONITOR_REMOTING_COOKIE = (0x4000 | COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING as isize),
    COR_PRF_MONITOR_REMOTING_ASYNC = (0x8000 | COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING as isize),
    COR_PRF_MONITOR_SUSPENDS = 0x10000,
    COR_PRF_MONITOR_CACHE_SEARCHES = 0x20000,
    COR_PRF_ENABLE_REJIT = 0x40000,
    COR_PRF_ENABLE_INPROC_DEBUGGING = 0x80000,
    COR_PRF_ENABLE_JIT_MAPS = 0x100000,
    COR_PRF_DISABLE_INLINING = 0x200000,
    COR_PRF_DISABLE_OPTIMIZATIONS = 0x400000,
    COR_PRF_ENABLE_OBJECT_ALLOCATED = 0x800000,
    COR_PRF_MONITOR_CLR_EXCEPTIONS = 0x1000000,
    COR_PRF_MONITOR_ALL = 0x107ffff,
    COR_PRF_ENABLE_FUNCTION_ARGS = 0x2000000,
    COR_PRF_ENABLE_FUNCTION_RETVAL = 0x4000000,
    COR_PRF_ENABLE_FRAME_INFO = 0x8000000,
    COR_PRF_ENABLE_STACK_SNAPSHOT = 0x10000000,
    COR_PRF_USE_PROFILE_IMAGES = 0x20000000,
    COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST = 0x40000000,
    COR_PRF_DISABLE_ALL_NGEN_IMAGES = 0x80000000,
    COR_PRF_ALL = 0x8fffffff,
    COR_PRF_REQUIRE_PROFILE_IMAGE = (COR_PRF_MONITOR::COR_PRF_USE_PROFILE_IMAGES as isize
        | COR_PRF_MONITOR::COR_PRF_MONITOR_CODE_TRANSITIONS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_ENTERLEAVE as isize,
    COR_PRF_ALLOWABLE_AFTER_ATTACH = (((((((((COR_PRF_MONITOR::COR_PRF_MONITOR_THREADS
        as isize
        | COR_PRF_MONITOR::COR_PRF_MONITOR_MODULE_LOADS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_ASSEMBLY_LOADS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_APPDOMAIN_LOADS as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_GC as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_SUSPENDS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_CLASS_LOADS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_JIT_COMPILATION as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_REJIT as isize,
    COR_PRF_MONITOR_IMMUTABLE = (((((((((((((COR_PRF_MONITOR::COR_PRF_MONITOR_CODE_TRANSITIONS
        as isize
        | COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING_COOKIE as isize)
        | COR_PRF_MONITOR::COR_PRF_MONITOR_REMOTING_ASYNC as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_INPROC_DEBUGGING as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_JIT_MAPS as isize)
        | COR_PRF_MONITOR::COR_PRF_DISABLE_OPTIMIZATIONS as isize)
        | COR_PRF_MONITOR::COR_PRF_DISABLE_INLINING as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_OBJECT_ALLOCATED as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_FUNCTION_ARGS as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_FUNCTION_RETVAL as isize)
        | COR_PRF_MONITOR::COR_PRF_ENABLE_FRAME_INFO as isize)
        | COR_PRF_MONITOR::COR_PRF_USE_PROFILE_IMAGES as isize)
        | COR_PRF_MONITOR::COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST as isize)
        | COR_PRF_MONITOR::COR_PRF_DISABLE_ALL_NGEN_IMAGES as isize,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COR_PRF_HIGH_MONITOR {
    COR_PRF_HIGH_MONITOR_NONE = 0,
    COR_PRF_HIGH_ADD_ASSEMBLY_REFERENCES = 0x1,
    COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED = 0x2,
    COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS = 0x4,
    COR_PRF_HIGH_DISABLE_TIERED_COMPILATION = 0x8,
    COR_PRF_HIGH_BASIC_GC = 0x10,
    COR_PRF_HIGH_MONITOR_GC_MOVED_OBJECTS = 0x20,
    // TODO: Duplicate discriminate values aren't allowed in rust c-like enums
    // COR_PRF_HIGH_REQUIRE_PROFILE_IMAGE = 0,
    COR_PRF_HIGH_MONITOR_LARGEOBJECT_ALLOCATED = 0x40,
    COR_PRF_HIGH_ALLOWABLE_AFTER_ATTACH =
        (((COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED as isize
            | COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS as isize)
            | COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_BASIC_GC as isize)
            | COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_GC_MOVED_OBJECTS as isize)
            | COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_LARGEOBJECT_ALLOCATED as isize,
    // TODO: Duplicate discriminate values aren't allowed in rust c-like enums
    // COR_PRF_HIGH_MONITOR_IMMUTABLE = COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_DISABLE_TIERED_COMPILATION as isize,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct COR_PRF_METHOD {
    moduleId: ModuleID,
    methodId: mdMethodDef,
}
