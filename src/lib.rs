mod winapi;

use core::ffi::c_void;
use winapi::{
    IClassFactory, IClassFactoryVtbl, IUnknown, IUnknownVtbl, BOOL, HRESULT, LPVOID, REFCLSID,
    REFIID, S_OK, ULONG,
};

#[no_mangle]
#[allow(overflowing_literals)]
pub unsafe extern "C" fn DllGetClassObject(
    rclsid: REFCLSID,
    riid: REFIID,
    ppv: *mut LPVOID,
) -> HRESULT {
    if rclsid.is_null() {
        return 0x8000_4005;
    }
    println!("{:?}", *rclsid);

    let mut class_factory = IClassFactory {
        lpVtbl: &IClassFactoryVtbl {
            parent: IUnknownVtbl {
                QueryInterface: query_interface,
                AddRef: add_ref,
                Release: release,
            },
            CreateInstance: create_instance,
            LockServer: lock_server,
        },
    };

    *ppv = &mut class_factory as *mut IClassFactory as LPVOID;

    S_OK
}

extern "system" fn query_interface(
    This: *mut IUnknown,
    riid: REFIID,
    ppvObject: *mut *mut c_void,
) -> HRESULT {
    println!("Hit query_interface!");
    unsafe {
        *ppvObject = This as *mut c_void;
    }
    S_OK
}

extern "system" fn add_ref(This: *mut IUnknown) -> ULONG {
    println!("Hit add_ref!");
    0
}

extern "system" fn release(This: *mut IUnknown) -> ULONG {
    println!("Hit release!");
    0
}

extern "system" fn create_instance(
    This: *mut IClassFactory,
    pUnkOuter: *mut IUnknown,
    riid: REFIID,
    ppvObject: *mut *mut c_void,
) -> HRESULT {
    println!("Hit create_instance!");
    S_OK
}

extern "system" fn lock_server(This: *mut IClassFactory, fLock: BOOL) -> HRESULT {
    println!("Hit lock_server!");
    S_OK
}
