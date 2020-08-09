#![allow(non_upper_case_globals)]
use crate::cil::{check_flag, il_u16, il_u32, il_u8, nearest_multiple, Error, Instruction};
use std::slice;

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
    fn parse_from_bytes(method_il: &[u8]) -> Result<Self, Error> {
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
            let code_size = il_u32(method_il, 4)?;
            let local_var_sig_tok = il_u32(method_il, 8)?;
            let fat_header = FatMethodHeader {
                more_sects,
                init_locals,
                max_stack,
                code_size,
                local_var_sig_tok,
            };
            Ok(MethodHeader::Fat(fat_header))
        } else {
            Err(Error::InvalidMethodHeader)
        }
    }
    fn more_sects(method_header_flags: u8) -> bool {
        check_flag(
            method_header_flags,
            MethodHeaderFlags::CorILMethod_MoreSects.bits(),
        )
    }
    fn init_locals(method_header_flags: u8) -> bool {
        check_flag(
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
bitflags! {
    pub struct SectionHeaderFlags: u8 {
        const CorILMethod_Sect_EHTable = 0x1;
        const CorILMethod_Sect_OptILTable = 0x2;
        const CorILMethod_Sect_FatFormat = 0x40;
        const CorILMethod_Sect_MoreSects = 0x80;
    }
}
bitflags! {
    pub struct ExceptionHandlingClauseFlags: u8 {
        const COR_ILEXCEPTION_CLAUSE_EXCEPTION = 0x0;
        const COR_ILEXCEPTION_CLAUSE_FILTER = 0x1;
        const COR_ILEXCEPTION_CLAUSE_FINALLY = 0x2;
        const COR_ILEXCEPTION_CLAUSE_FAULT = 0x4;
    }
}
#[derive(Debug)]
pub struct FatSectionHeader {
    pub is_eh_table: bool,
    pub more_sects: bool,
    /// Note that this really should be u24, but no such type exists.
    pub data_size: u32,
}
#[derive(Debug)]
pub struct FatSectionClause {
    pub is_exception: bool,
    pub is_filter: bool,
    pub is_finally: bool,
    pub is_fault: bool,
    pub try_offset: u32,
    pub try_length: u32,
    pub handler_offset: u32,
    pub handler_length: u32,
    pub class_token_or_filter_offset: u32,
}
impl FatSectionClause {
    const LENGTH: usize = 24;
    pub fn parse_from_bytes(il: &[u8]) -> Result<Self, Error> {
        let flags = il_u8(il, 0)?;
        let is_exception = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_EXCEPTION.bits(),
        );
        let is_filter = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FILTER.bits(),
        );
        let is_finally = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FINALLY.bits(),
        );
        let is_fault = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FAULT.bits(),
        );
        let try_offset = il_u32(il, 4)?;
        let try_length = il_u32(il, 8)?;
        let handler_offset = il_u32(il, 12)?;
        let handler_length = il_u32(il, 16)?;
        let class_token_or_filter_offset = il_u32(il, 20)?;
        Ok(FatSectionClause {
            is_exception,
            is_filter,
            is_finally,
            is_fault,
            try_offset,
            try_length,
            handler_offset,
            handler_length,
            class_token_or_filter_offset,
        })
    }
}
#[derive(Debug)]
pub struct SmallSectionHeader {
    pub is_eh_table: bool,
    pub more_sects: bool,
    pub data_size: u8,
}
#[derive(Debug)]
pub struct SmallSectionClause {
    pub is_exception: bool,
    pub is_filter: bool,
    pub is_finally: bool,
    pub is_fault: bool,
    pub try_offset: u16,
    pub try_length: u8,
    pub handler_offset: u16,
    pub handler_length: u8,
    pub class_token_or_filter_offset: u32,
}
impl SmallSectionClause {
    const LENGTH: usize = 12;
    pub fn parse_from_bytes(il: &[u8]) -> Result<Self, Error> {
        let flags = il_u8(il, 0)?;
        let is_exception = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_EXCEPTION.bits(),
        );
        let is_filter = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FILTER.bits(),
        );
        let is_finally = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FINALLY.bits(),
        );
        let is_fault = check_flag(
            flags,
            ExceptionHandlingClauseFlags::COR_ILEXCEPTION_CLAUSE_FAULT.bits(),
        );
        let try_offset = il_u16(il, 2)?;
        let try_length = il_u8(il, 4)?;
        let handler_offset = il_u16(il, 5)?;
        let handler_length = il_u8(il, 7)?;
        let class_token_or_filter_offset = il_u32(il, 8)?;
        Ok(SmallSectionClause {
            is_exception,
            is_filter,
            is_finally,
            is_fault,
            try_offset,
            try_length,
            handler_offset,
            handler_length,
            class_token_or_filter_offset,
        })
    }
}
#[derive(Debug)]
pub enum Section {
    FatSection(FatSectionHeader, Vec<FatSectionClause>),
    SmallSection(SmallSectionHeader, Vec<SmallSectionClause>),
}
impl Section {
    fn parse_from_bytes(il: &[u8]) -> Result<Self, Error> {
        let header_flags = il[0];
        let is_eh_table = Self::is_eh_table(header_flags);
        let more_sects = Self::more_sects(header_flags);
        if Self::is_small(header_flags) {
            let data_size = il_u8(il, 1)?;
            let small_header = SmallSectionHeader {
                is_eh_table,
                more_sects,
                data_size,
            };
            let clause_bytes = &il[4..(data_size as usize)];
            let clauses = Self::get_small_clauses(clause_bytes)?;
            Ok(Section::SmallSection(small_header, clauses))
        } else if Self::is_fat(header_flags) {
            let byte_1 = il_u8(il, 1)?;
            let byte_2 = il_u8(il, 2)?;
            let byte_3 = il_u8(il, 3)?;
            let data_size = u32::from_le_bytes([byte_1, byte_2, byte_3, 0]);
            let fat_header = FatSectionHeader {
                is_eh_table,
                more_sects,
                data_size,
            };
            let clause_bytes = &il[4..(data_size as usize)];
            let clauses = Self::get_fat_clauses(clause_bytes)?;
            Ok(Section::FatSection(fat_header, clauses))
        } else {
            Err(Error::InvalidSectionHeader)
        }
    }
    pub fn data_size(&self) -> usize {
        match self {
            Self::FatSection(header, _) => header.data_size as usize,
            Self::SmallSection(header, _) => header.data_size as usize,
        }
    }
    fn is_small(section_header_flags: u8) -> bool {
        !Self::is_fat(section_header_flags)
    }
    fn is_fat(section_header_flags: u8) -> bool {
        check_flag(
            section_header_flags,
            SectionHeaderFlags::CorILMethod_Sect_FatFormat.bits(),
        )
    }
    fn is_eh_table(section_header_flags: u8) -> bool {
        check_flag(
            section_header_flags,
            SectionHeaderFlags::CorILMethod_Sect_EHTable.bits(),
        )
    }
    fn more_sects(section_header_flags: u8) -> bool {
        check_flag(
            section_header_flags,
            SectionHeaderFlags::CorILMethod_Sect_MoreSects.bits(),
        )
    }
    fn get_fat_clauses(il: &[u8]) -> Result<Vec<FatSectionClause>, Error> {
        let mut index = 0;
        let mut clauses = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let clause = FatSectionClause::parse_from_bytes(il)?;
            index += FatSectionClause::LENGTH;
            clauses.push(clause);
        }
        Ok(clauses)
    }
    fn get_small_clauses(il: &[u8]) -> Result<Vec<SmallSectionClause>, Error> {
        let mut index = 0;
        let mut clauses = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let clause = SmallSectionClause::parse_from_bytes(il)?;
            index += SmallSectionClause::LENGTH;
            clauses.push(clause);
        }
        Ok(clauses)
    }
}
#[derive(Debug)]
pub struct Method {
    pub method_header: MethodHeader,
    pub instructions: Vec<Instruction>,
    pub sections: Vec<Section>,
}
impl Method {
    pub fn new(method_header: *const u8, method_size: u32) -> Result<Self, Error> {
        let body = unsafe { slice::from_raw_parts(method_header, method_size as usize) };
        let method_header = MethodHeader::parse_from_bytes(&body)?;
        let (instructions_start, instructions_end): (usize, usize) = match &method_header {
            MethodHeader::Fat(header) => (12, (12 + header.code_size - 1) as usize),
            MethodHeader::Tiny(header) => (1, header.code_size as usize),
        };
        let instruction_bytes = &body[instructions_start..=instructions_end];
        let instructions = Self::get_instructions(instruction_bytes)?;
        let sections = match &method_header {
            MethodHeader::Fat(header) if header.more_sects => {
                let sections_start = nearest_multiple(4, instructions_end + 1); // Sections must be DWORD aligned
                let sections_bytes = &body[sections_start..];
                Self::get_sections(sections_bytes)?
            }
            _ => Vec::new(),
        };
        Ok(Method {
            method_header,
            instructions,
            sections,
        })
    }
    fn get_instructions(il: &[u8]) -> Result<Vec<Instruction>, Error> {
        let mut index = 0;
        let mut instructions = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let instruction = Instruction::parse_from_bytes(il)?;
            index += instruction.length;
            instructions.push(instruction);
        }
        Ok(instructions)
    }
    fn get_sections(il: &[u8]) -> Result<Vec<Section>, Error> {
        let mut index = 0;
        let mut sections = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let section = Section::parse_from_bytes(il)?;
            index += section.data_size();
            sections.push(section);
        }
        Ok(sections)
    }
}
