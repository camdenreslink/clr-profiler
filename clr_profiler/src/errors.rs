#![allow(non_camel_case_types)]
use crate::ffi::{
    CLASS_E_NOAGGREGATION, COR_E_INDEXOUTOFRANGE, COR_E_INVALIDOPERATION, COR_E_INVALIDPROGRAM,
    E_FAIL, E_NOINTERFACE, E_OUTOFMEMORY, HRESULT,
};

#[non_exhaustive]
pub enum Error {
    E_FAIL,
    E_NOINTERFACE,
    CLASS_E_NOAGGREGATION,
    E_OUTOFMEMORY,
    COR_E_INVALIDPROGRAM,
    COR_E_INVALIDOPERATION,
    COR_E_INDEXOUTOFRANGE,
}

impl From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        match error {
            Error::E_FAIL => E_FAIL,
            Error::E_NOINTERFACE => E_NOINTERFACE,
            Error::CLASS_E_NOAGGREGATION => CLASS_E_NOAGGREGATION,
            Error::E_OUTOFMEMORY => E_OUTOFMEMORY,
            Error::COR_E_INVALIDPROGRAM => COR_E_INVALIDPROGRAM,
            Error::COR_E_INVALIDOPERATION => COR_E_INVALIDOPERATION,
            Error::COR_E_INDEXOUTOFRANGE => COR_E_INDEXOUTOFRANGE,
        }
    }
}
