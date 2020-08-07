#![allow(non_upper_case_globals)]
use crate::cil::Error;
use std::{convert::TryInto, slice};

bitflags! {
    pub struct MethodHeaderFlags: u8 {
        const CorILMethod_FatFormat = 0x3;
        const CorILMethod_TinyFormat = 0x2;
        const CorILMethod_MoreSects = 0x8;
        const CorILMethod_InitLocals = 0x10;
    }
}
#[derive(Debug)]
pub struct FatMethodHeader {
    pub more_sects: bool,
    pub init_locals: bool,
    pub max_stack: u16,
    pub code_size: u32,
    pub local_var_sig_tok: u32,
}
impl FatMethodHeader {
    pub const SIZE: u8 = 3;
}
#[derive(Debug)]
pub struct TinyMethodHeader {
    pub code_size: u8,
}
#[derive(Debug)]
pub enum MethodHeader {
    Fat(FatMethodHeader),
    Tiny(TinyMethodHeader),
}
impl MethodHeader {
    fn new(method_il: &[u8]) -> Result<Self, Error> {
        let header_flags = method_il[0];
        if Self::is_tiny(header_flags) {
            // In a tiny header, the first 6 bits encode the code size
            let code_size = method_il[0] >> 2;
            let tiny_header = TinyMethodHeader { code_size };
            Ok(MethodHeader::Tiny(tiny_header))
        } else if Self::is_fat(header_flags) {
            let more_sects = Self::more_sects(header_flags);
            let init_locals = Self::init_locals(header_flags);
            let max_stack = u16::from_le_bytes([method_il[2], method_il[3]]);
            let code_size_bytes = [method_il[4], method_il[5], method_il[6], method_il[7]];
            let code_size = u32::from_le_bytes(code_size_bytes);
            let local_var_sig_tok_bytes =
                [method_il[8], method_il[9], method_il[10], method_il[11]];
            let local_var_sig_tok = u32::from_le_bytes(local_var_sig_tok_bytes);
            let fat_header = FatMethodHeader {
                more_sects,
                init_locals,
                max_stack,
                code_size,
                local_var_sig_tok,
            };
            Ok(MethodHeader::Fat(fat_header))
        } else {
            Err(Error::InvalidMethodHeaderFlags)
        }
    }
    fn check_flag(flags: u8, flag: u8) -> bool {
        (flags & flag) == flag
    }
    fn more_sects(method_header_flags: u8) -> bool {
        Self::check_flag(
            method_header_flags,
            MethodHeaderFlags::CorILMethod_MoreSects.bits(),
        )
    }
    fn init_locals(method_header_flags: u8) -> bool {
        Self::check_flag(
            method_header_flags,
            MethodHeaderFlags::CorILMethod_InitLocals.bits(),
        )
    }
    fn is_tiny(method_header_flags: u8) -> bool {
        // Check only the 2 least significant bits
        (method_header_flags & 0b00000011) == MethodHeaderFlags::CorILMethod_TinyFormat.bits()
    }
    fn is_fat(method_header_flags: u8) -> bool {
        // Check only the 2 least significant bits
        (method_header_flags & 0b00000011) == MethodHeaderFlags::CorILMethod_FatFormat.bits()
    }
}
#[derive(Debug)]
pub struct Method {
    pub body: Vec<u8>,
    pub method_header: MethodHeader,
}
impl Method {
    pub fn new(method_header: *const u8, method_size: u32) -> Result<Self, Error> {
        let body_slice = unsafe { slice::from_raw_parts(method_header, method_size as usize) };
        let body = body_slice.to_vec();
        let method_header = MethodHeader::new(&body)?;
        Ok(Method {
            body: body,
            method_header,
        })
    }
}
