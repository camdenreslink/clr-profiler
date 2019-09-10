extern crate libc;
use libc::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
type IID = GUID;
type REFCLSID = *const IID;
type REFIID = *const IID;
type LPVOID = *mut c_void;
type HRESULT = i32;

#[no_mangle]
#[allow(overflowing_literals)]
pub extern "C" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppv: *mut LPVOID) -> HRESULT {
    if rclsid.is_null() {
        return 0x8000_4005;
    }
    unsafe {
        println!("{:?}", *rclsid);
    }
    0x8000_4005
}
