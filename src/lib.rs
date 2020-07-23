mod errors;
mod ffi;
mod traits;

use ffi::{ClassFactory, CorProfilerCallback, E_FAIL, HRESULT, LPVOID, REFCLSID, REFIID};

#[no_mangle]
unsafe extern "system" fn DllGetClassObject(
    rclsid: REFCLSID,
    riid: REFIID,
    ppv: *mut LPVOID,
) -> HRESULT {
    if ppv.is_null() || *rclsid != CorProfilerCallback::CLSID {
        println!("rclsid isn't correct.");
        E_FAIL
    } else {
        let class_factory = ClassFactory::new();
        class_factory.query_interface(riid, ppv)
    }
}
