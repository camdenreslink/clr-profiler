#![allow(non_snake_case)]
use crate::ffi::{
    mdCustomAttribute, mdEvent, mdFieldDef, mdMemberRef, mdMethodDef, mdModuleRef, mdParamDef,
    mdPermission, mdProperty, mdSignature, mdString, mdToken, mdTypeDef, mdTypeRef, mdTypeSpec,
    CorSaveSize, Unknown, COR_FIELD_OFFSET, COR_SECATTR, DWORD, GUID, HRESULT, LPCWSTR,
    PCCOR_SIGNATURE, PCOR_SIGNATURE, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct IMetaDataEmit<T> {
    pub SetModuleProps: unsafe extern "system" fn(this: &T, szName: LPCWSTR) -> HRESULT,
    pub Save: unsafe extern "system" fn(this: &T, szName: LPCWSTR, dwSaveFlags: DWORD) -> HRESULT,
    pub SaveToStream: unsafe extern "system" fn(
        this: &T,
        pIStream: *const Unknown, // TODO: IStream
        dwSaveFlags: DWORD,
    ) -> HRESULT,
    pub GetSaveSize:
        unsafe extern "system" fn(this: &T, fSave: CorSaveSize, pdwSaveSize: *mut DWORD) -> HRESULT,
    pub DefineTypeDef: unsafe extern "system" fn(
        this: &T,
        szTypeDef: LPCWSTR,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
        ptd: *mut mdTypeDef,
    ) -> HRESULT,
    pub DefineNestedType: unsafe extern "system" fn(
        this: &T,
        szTypeDef: LPCWSTR,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
        tdEncloser: mdTypeDef,
        ptd: *mut mdTypeDef,
    ) -> HRESULT,
    pub SetHandler: unsafe extern "system" fn(this: &T, pUnk: *const Unknown) -> HRESULT,
    pub DefineMethod: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        dwMethodFlags: DWORD,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        ulCodeRVA: ULONG,
        dwImplFlags: DWORD,
        pmd: *const mdMethodDef,
    ) -> HRESULT,
    pub DefineMethodImpl: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        tkBody: mdToken,
        tkDecl: mdToken,
    ) -> HRESULT,
    pub DefineTypeRefByName: unsafe extern "system" fn(
        this: &T,
        tkResolutionScope: mdToken,
        szName: LPCWSTR,
        ptr: *mut mdTypeRef,
    ) -> HRESULT,
    pub DefineImportType: unsafe extern "system" fn(
        this: &T,
        pAssemImport: *const Unknown, // TODO: Should be MetaDataAssemblyImport
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        pImport: *const Unknown, // TODO: MetaDataImport
        tdImport: mdTypeDef,
        pAssemEmit: *const Unknown, // TODO: MetaDataAssemblyEmit
        ptr: *mut mdTypeRef,
    ) -> HRESULT,
    pub DefineMemberRef: unsafe extern "system" fn(
        this: &T,
        tkImport: mdToken,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmr: *mut mdMemberRef,
    ) -> HRESULT,
    pub DefineImportMember: unsafe extern "system" fn(
        this: &T,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        pImport: *const Unknown, // IMetaDataImport
        mbMember: mdToken,
        pAssemEmit: *const Unknown, // IMetaDataAssemblyEmit
        tkParent: mdToken,
        pmr: *mut mdMemberRef,
    ) -> HRESULT,
    pub DefineEvent: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szEvent: LPCWSTR,
        dwEventFlags: DWORD,
        tkEventType: mdToken,
        mdAddOn: mdMethodDef,
        mdRemoveOn: mdMethodDef,
        mdFire: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
        pmdEvent: *mut mdEvent,
    ) -> HRESULT,
    pub SetClassLayout: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        dwPackSize: DWORD,
        rFieldOffsets: *const COR_FIELD_OFFSET,
        ulClassSize: ULONG,
    ) -> HRESULT,
    pub DeleteClassLayout: unsafe extern "system" fn(this: &T, td: mdTypeDef) -> HRESULT,
    pub DeleteFieldMarshal: unsafe extern "system" fn(this: &T, tk: mdToken) -> HRESULT,
    pub DefinePermissionSet: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        dwAction: DWORD,
        pvPermission: *const c_void,
        cbPermission: ULONG,
        ppm: *mut mdPermission,
    ) -> HRESULT,
    pub SetRVA: unsafe extern "system" fn(this: &T, md: mdMethodDef, ulRVA: ULONG) -> HRESULT,
    pub GetTokenFromSig: unsafe extern "system" fn(
        this: &T,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        pmsig: *mut mdSignature,
    ) -> HRESULT,
    pub DefineModuleRef:
        unsafe extern "system" fn(this: &T, szName: LPCWSTR, pmur: *mut mdModuleRef) -> HRESULT,
    pub SetParent: unsafe extern "system" fn(this: &T, mr: mdMemberRef, tk: mdToken) -> HRESULT,
    pub GetTokenFromTypeSpec: unsafe extern "system" fn(
        this: &T,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        ptypespec: *mut mdTypeSpec,
    ) -> HRESULT,
    pub SaveToMemory:
        unsafe extern "system" fn(this: &T, pbData: *mut c_void, cbData: ULONG) -> HRESULT,
    pub DefineUserString: unsafe extern "system" fn(
        this: &T,
        szString: LPCWSTR,
        cchString: ULONG,
        pstk: *mut mdString,
    ) -> HRESULT,
    pub DeleteToken: unsafe extern "system" fn(this: &T, tkObj: mdToken) -> HRESULT,
    pub SetTypeDefProps: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
    ) -> HRESULT,
    pub SetEventProps: unsafe extern "system" fn(
        this: &T,
        ev: mdEvent,
        dwEventFlags: DWORD,
        tkEventType: mdToken,
        mdAddOn: mdMethodDef,
        mdRemoveOn: mdMethodDef,
        mdFire: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
    ) -> HRESULT,
    pub SetPermissionSetProps: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        dwAction: DWORD,
        pvPermission: *const c_void,
        cbPermission: ULONG,
        ppm: *mut mdPermission,
    ) -> HRESULT,
    pub DefinePinvokeMap: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        dwMappingFlags: DWORD,
        szImportName: LPCWSTR,
        mrImportDLL: mdModuleRef,
    ) -> HRESULT,
    pub SetPinvokeMap: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        dwMappingFlags: DWORD,
        szImportName: LPCWSTR,
        mrImportDLL: mdModuleRef,
    ) -> HRESULT,
    pub DeletePinvokeMap: unsafe extern "system" fn(this: &T, tk: mdToken) -> HRESULT,
    pub DefineCustomAttribute: unsafe extern "system" fn(
        this: &T,
        tkOwner: mdToken,
        tkCtor: mdToken,
        pCustomAttribute: *const c_void,
        cbCustomAttribute: ULONG,
        pcv: *mut mdCustomAttribute,
    ) -> HRESULT,
    pub SetCustomAttributeValue: unsafe extern "system" fn(
        this: &T,
        pcv: mdCustomAttribute,
        pCustomAttribute: *const c_void,
        cbCustomAttribute: ULONG,
    ) -> HRESULT,
    pub DefineField: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szName: LPCWSTR,
        dwFieldFlags: DWORD,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        pmd: *mut mdFieldDef,
    ) -> HRESULT,
    pub DefineProperty: unsafe extern "system" fn(
        this: &T,
        td: mdTypeDef,
        szProperty: LPCWSTR,
        dwPropFlags: DWORD,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        mdSetter: mdMethodDef,
        mdGetter: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
        pmdProp: *mut mdProperty,
    ) -> HRESULT,
    pub DefineParam: unsafe extern "system" fn(
        this: &T,
        md: mdMethodDef,
        ulParamSeq: ULONG,
        szName: LPCWSTR,
        dwParamFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        ppd: *mut mdParamDef,
    ) -> HRESULT,
    pub SetFieldProps: unsafe extern "system" fn(
        this: &T,
        fd: mdFieldDef,
        dwFieldFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
    ) -> HRESULT,
    pub SetPropertyProps: unsafe extern "system" fn(
        this: &T,
        pr: mdProperty,
        dwPropFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        mdSetter: mdMethodDef,
        mdGetter: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
    ) -> HRESULT,
    pub SetParamProps: unsafe extern "system" fn(
        this: &T,
        pd: mdParamDef,
        szName: LPCWSTR,
        dwParamFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *mut c_void,
        cchValue: ULONG,
    ) -> HRESULT,
    pub DefineSecurityAttributeSet: unsafe extern "system" fn(
        this: &T,
        tkObj: mdToken,
        rSecAttrs: *const COR_SECATTR,
        cSecAttrs: ULONG,
        pulErrorAttr: *mut ULONG,
    ) -> HRESULT,
    pub ApplyEditAndContinue: unsafe extern "system" fn(
        this: &T,
        pImport: *const Unknown, // TODO: Which actual class?
    ) -> HRESULT,
    pub TranslateSigWithScope: unsafe extern "system" fn(
        this: &T,
        pAssemImport: *const Unknown, // TODO: IMetaDataAssemblyImport
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        import: *const Unknown, // TODO: IMetaDataImport
        pbSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pAssemEmit: *const Unknown, // TODO: IMetaDataAssemblyEmit
        emit: *const T,
        pvTranslatedSig: PCOR_SIGNATURE,
        cbTranslatedSigMax: ULONG,
        pcbTranslatedSig: *mut ULONG,
    ) -> HRESULT,
    pub SetMethodImplFlags:
        unsafe extern "system" fn(this: &T, md: mdMethodDef, dwImplFlags: DWORD) -> HRESULT,
    pub SetFieldRVA: unsafe extern "system" fn(this: &T, fd: mdFieldDef, ulRVA: ULONG) -> HRESULT,
    pub Merge: unsafe extern "system" fn(
        this: &T,
        pImport: *const Unknown,       // TODO: IMetaDataImport
        pHostMapToken: *const Unknown, // TODO: IMapToken
        pHandler: *const Unknown,
    ) -> HRESULT,
    pub MergeEnd: unsafe extern "system" fn(this: &T) -> HRESULT,
}

impl IMetaDataEmit<()> {
    // BA3FEE4C-ECB9-4E41-83B7-183FA41CD859
    pub const IID: GUID = GUID {
        data1: 0xBA3FEE4C,
        data2: 0xECB9,
        data3: 0x4E41,
        data4: [0x83, 0xB7, 0x18, 0x3F, 0xA4, 0x1C, 0xD8, 0x59],
    };
}