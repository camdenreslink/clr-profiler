extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    let profiler_type = parse_macro_input!(item as Type);
    let output = quote! {
        use clr_profiler::ffi::{ClassFactory as FFIClassFactory, CorProfilerCallback as FFICorProfilerCallback, E_FAIL as FFI_E_FAIL, GUID as FFI_GUID, HRESULT as FFI_HRESULT, LPVOID as FFI_LPVOID, REFCLSID as FFI_REFCLSID, REFIID as FFI_REFIID};
        #[no_mangle]
        unsafe extern "system" fn DllGetClassObject(
            rclsid: FFI_REFCLSID,
            riid: FFI_REFIID,
            ppv: *mut FFI_LPVOID,
        ) -> FFI_HRESULT {
            let profiler = <#profiler_type>::new();
            let clsid = FFI_GUID::from(*profiler.clsid());
            if ppv.is_null() || *rclsid != clsid {
                println!("CLSID didn't match. CLSID: {:?}", clsid);
                FFI_E_FAIL
            } else {
                let class_factory: &mut FFIClassFactory<#profiler_type> = FFIClassFactory::new(profiler);
                class_factory.QueryInterface(riid, ppv)
            }
        }
    };
    output.into()
}
#[proc_macro_derive(CorProfilerCallback)]
pub fn derive_cor_profiler_callback(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback2)]
pub fn derive_cor_profiler_callback_2(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback2 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback3)]
pub fn derive_cor_profiler_callback_3(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback3 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback4)]
pub fn derive_cor_profiler_callback_4(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback4 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback5)]
pub fn derive_cor_profiler_callback_5(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback5 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback6)]
pub fn derive_cor_profiler_callback_6(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback6 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback7)]
pub fn derive_cor_profiler_callback_7(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback7 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback8)]
pub fn derive_cor_profiler_callback_8(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback8 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback9)]
pub fn derive_cor_profiler_callback_9(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback9 for #name {}
    };
    trait_impl.into()
}
