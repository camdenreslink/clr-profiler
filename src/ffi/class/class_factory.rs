#![allow(non_snake_case)]
use crate::ffi::{
    CorProfilerCallback, IClassFactoryCom, IUnknown, BOOL, E_NOINTERFACE, HRESULT, LPVOID, REFIID,
    S_OK, ULONG,
};
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};

#[repr(C)]
pub struct ClassFactoryVtbl {
    pub IUnknown: IUnknown<ClassFactory>,
    pub IClassFactory: IClassFactoryCom<ClassFactory>,
}

#[repr(C)]
pub struct ClassFactory {
    pub lpVtbl: *const ClassFactoryVtbl,
    ref_count: AtomicU32,
}

impl ClassFactory {
    pub fn new<'a>() -> &'a mut ClassFactory {
        let class_factory = ClassFactory {
            lpVtbl: &ClassFactoryVtbl {
                IUnknown: IUnknown {
                    QueryInterface: Self::query_interface,
                    AddRef: Self::add_ref,
                    Release: Self::release,
                },
                IClassFactory: IClassFactoryCom {
                    CreateInstance: Self::create_instance,
                    LockServer: Self::lock_server,
                },
            },
            ref_count: AtomicU32::new(0),
        };
        Box::leak(Box::new(class_factory))
    }

    pub unsafe extern "system" fn query_interface(
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        println!("Class Factory hit query_interface!");
        if *riid == IUnknown::IID || *riid == IClassFactoryCom::IID {
            *ppvObject = self as *mut ClassFactory as LPVOID;
            self.add_ref();
            S_OK
        } else {
            *ppvObject = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    pub unsafe extern "system" fn add_ref(&mut self) -> ULONG {
        println!("Class Factory hit add_ref!");
        // TODO: Which ordering is appropriate?
        let prev_ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed);
        prev_ref_count + 1
    }

    pub unsafe extern "system" fn release(&mut self) -> ULONG {
        println!("Class Factory hit release!");
        // Ensure we are not trying to release the memory twice if
        // client calls release despite the ref_count being zero.
        // TODO: Which ordering is appropriate?
        if self.ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Cannot release the COM object, it has already been released.");
        }

        let prev_ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed);
        let ref_count = prev_ref_count - 1;

        if ref_count == 0 {
            drop(Box::from_raw(self as *mut ClassFactory));
        }

        ref_count
    }

    pub unsafe extern "system" fn create_instance(
        &mut self,
        pUnkOuter: *mut IUnknown<()>,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        println!("Class Factory hit create_instance!");
        *ppvObject = CorProfilerCallback::new() as *mut CorProfilerCallback as LPVOID;
        S_OK
    }

    pub extern "system" fn lock_server(&mut self, fLock: BOOL) -> HRESULT {
        println!("Class Factory hit lock_server!");
        S_OK
    }
}
