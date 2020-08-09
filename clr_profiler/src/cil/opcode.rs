use crate::cil::Error;

#[derive(Debug)]
pub enum StackBehaviorPop {
    Pop0,
    Pop1,
    VarPop,
    PopI,
    Pop1Pop1,
    PopIPopI,
    PopIPopI8,
    PopIPopR4,
    PopIPopR8,
    PopRef,
    PopRefPop1,
    PopIPop1,
    PopRefPopI,
    PopRefPopIPopI,
    PopRefPopIPopI8,
    PopRefPopIPopR4,
    PopRefPopIPopR8,
    PopRefPopIPopRef,
    PopRefPopIPop1,
    PopIPopIPopI,
}
#[derive(Debug)]
pub enum StackBehaviorPush {
    Push0,
    Push1,
    PushI,
    PushRef,
    PushI8,
    PushR4,
    PushR8,
    Push1Push1,
    VarPush,
}
#[derive(Debug)]
pub enum OperandParams {
    InlineNone,
    ShortInlineVar,
    InlineVar,
    ShortInlineI,
    InlineI,
    InlineI8,
    ShortInlineR,
    InlineR,
    InlineMethod,
    InlineSig,
    ShortInlineBrTarget,
    InlineBrTarget,
    InlineSwitch,
    InlineType,
    InlineString,
    InlineField,
    InlineTok,
}
#[derive(Debug)]
pub enum OpcodeKind {
    Primitive,
    Macro,
    ObjModel,
    Internal,
    Prefix,
}
#[derive(Debug)]
pub enum ControlFlow {
    Next,
    Break,
    Return,
    Branch,
    CondBranch,
    Call,
    Throw,
    Meta,
}
#[derive(Debug)]
pub struct Opcode {
    pub name: &'static str,
    pub stack_behavior_pop: StackBehaviorPop,
    pub stack_behavior_push: StackBehaviorPush,
    pub operand_params: OperandParams,
    pub opcode_kind: OpcodeKind,
    pub length: u8,
    pub byte_1: u8,
    pub byte_2: u8,
    pub control_flow: ControlFlow,
}

impl Opcode {
    pub const fn new(
        name: &'static str,
        stack_behavior_pop: StackBehaviorPop,
        stack_behavior_push: StackBehaviorPush,
        operand_params: OperandParams,
        opcode_kind: OpcodeKind,
        length: u8,
        byte_1: u8,
        byte_2: u8,
        control_flow: ControlFlow,
    ) -> Self {
        Opcode {
            name,
            stack_behavior_pop,
            stack_behavior_push,
            operand_params,
            opcode_kind,
            length,
            byte_1,
            byte_2,
            control_flow,
        }
    }
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => CEE_NOP,
            0x01 => CEE_BREAK,
            0x02 => CEE_LDARG_0,
            0x03 => CEE_LDARG_1,
            0x04 => CEE_LDARG_2,
            0x05 => CEE_LDARG_3,
            0x06 => CEE_LDLOC_0,
            0x07 => CEE_LDLOC_1,
            0x08 => CEE_LDLOC_2,
            0x09 => CEE_LDLOC_3,
            0x0A => CEE_STLOC_0,
            0x0B => CEE_STLOC_1,
            0x0C => CEE_STLOC_2,
            0x0D => CEE_STLOC_3,
            0x0E => CEE_LDARG_S,
            0x0F => CEE_LDARGA_S,
            0x10 => CEE_STARG_S,
            0x11 => CEE_LDLOC_S,
            0x12 => CEE_LDLOCA_S,
            0x13 => CEE_STLOC_S,
            0x14 => CEE_LDNULL,
            0x15 => CEE_LDC_I4_M1,
            0x16 => CEE_LDC_I4_0,
            0x17 => CEE_LDC_I4_1,
            0x18 => CEE_LDC_I4_2,
            0x19 => CEE_LDC_I4_3,
            0x1A => CEE_LDC_I4_4,
            0x1B => CEE_LDC_I4_5,
            0x1C => CEE_LDC_I4_6,
            0x1D => CEE_LDC_I4_7,
            0x1E => CEE_LDC_I4_8,
            0x1F => CEE_LDC_I4_S,
            0x20 => CEE_LDC_I4,
            0x21 => CEE_LDC_I8,
            0x22 => CEE_LDC_R4,
            0x23 => CEE_LDC_R8,
            0x24 => CEE_UNUSED49,
            0x25 => CEE_DUP,
            0x26 => CEE_POP,
            0x27 => CEE_JMP,
            0x28 => CEE_CALL,
            0x29 => CEE_CALLI,
            0x2A => CEE_RET,
            0x2B => CEE_BR_S,
            0x2C => CEE_BRFALSE_S,
            0x2D => CEE_BRTRUE_S,
            0x2E => CEE_BEQ_S,
            0x2F => CEE_BGE_S,
            0x30 => CEE_BGT_S,
            0x31 => CEE_BLE_S,
            0x32 => CEE_BLT_S,
            0x33 => CEE_BNE_UN_S,
            0x34 => CEE_BGE_UN_S,
            0x35 => CEE_BGT_UN_S,
            0x36 => CEE_BLE_UN_S,
            0x37 => CEE_BLT_UN_S,
            0x38 => CEE_BR,
            0x39 => CEE_BRFALSE,
            0x3A => CEE_BRTRUE,
            0x3B => CEE_BEQ,
            0x3C => CEE_BGE,
            0x3D => CEE_BGT,
            0x3E => CEE_BLE,
            0x3F => CEE_BLT,
            0x40 => CEE_BNE_UN,
            0x41 => CEE_BGE_UN,
            0x42 => CEE_BGT_UN,
            0x43 => CEE_BLE_UN,
            0x44 => CEE_BLT_UN,
            0x45 => CEE_SWITCH,
            0x46 => CEE_LDIND_I1,
            0x47 => CEE_LDIND_U1,
            0x48 => CEE_LDIND_I2,
            0x49 => CEE_LDIND_U2,
            0x4A => CEE_LDIND_I4,
            0x4B => CEE_LDIND_U4,
            0x4C => CEE_LDIND_I8,
            0x4D => CEE_LDIND_I,
            0x4E => CEE_LDIND_R4,
            0x4F => CEE_LDIND_R8,
            0x50 => CEE_LDIND_REF,
            0x51 => CEE_STIND_REF,
            0x52 => CEE_STIND_I1,
            0x53 => CEE_STIND_I2,
            0x54 => CEE_STIND_I4,
            0x55 => CEE_STIND_I8,
            0x56 => CEE_STIND_R4,
            0x57 => CEE_STIND_R8,
            0x58 => CEE_ADD,
            0x59 => CEE_SUB,
            0x5A => CEE_MUL,
            0x5B => CEE_DIV,
            0x5C => CEE_DIV_UN,
            0x5D => CEE_REM,
            0x5E => CEE_REM_UN,
            0x5F => CEE_AND,
            0x60 => CEE_OR,
            0x61 => CEE_XOR,
            0x62 => CEE_SHL,
            0x63 => CEE_SHR,
            0x64 => CEE_SHR_UN,
            0x65 => CEE_NEG,
            0x66 => CEE_NOT,
            0x67 => CEE_CONV_I1,
            0x68 => CEE_CONV_I2,
            0x69 => CEE_CONV_I4,
            0x6A => CEE_CONV_I8,
            0x6B => CEE_CONV_R4,
            0x6C => CEE_CONV_R8,
            0x6D => CEE_CONV_U4,
            0x6E => CEE_CONV_U8,
            0x6F => CEE_CALLVIRT,
            0x70 => CEE_CPOBJ,
            0x71 => CEE_LDOBJ,
            0x72 => CEE_LDSTR,
            0x73 => CEE_NEWOBJ,
            0x74 => CEE_CASTCLASS,
            0x75 => CEE_ISINST,
            0x76 => CEE_CONV_R_UN,
            0x77 => CEE_UNUSED58,
            0x78 => CEE_UNUSED1,
            0x79 => CEE_UNBOX,
            0x7A => CEE_THROW,
            0x7B => CEE_LDFLD,
            0x7C => CEE_LDFLDA,
            0x7D => CEE_STFLD,
            0x7E => CEE_LDSFLD,
            0x7F => CEE_LDSFLDA,
            0x80 => CEE_STSFLD,
            0x81 => CEE_STOBJ,
            0x82 => CEE_CONV_OVF_I1_UN,
            0x83 => CEE_CONV_OVF_I2_UN,
            0x84 => CEE_CONV_OVF_I4_UN,
            0x85 => CEE_CONV_OVF_I8_UN,
            0x86 => CEE_CONV_OVF_U1_UN,
            0x87 => CEE_CONV_OVF_U2_UN,
            0x88 => CEE_CONV_OVF_U4_UN,
            0x89 => CEE_CONV_OVF_U8_UN,
            0x8A => CEE_CONV_OVF_I_UN,
            0x8B => CEE_CONV_OVF_U_UN,
            0x8C => CEE_BOX,
            0x8D => CEE_NEWARR,
            0x8E => CEE_LDLEN,
            0x8F => CEE_LDELEMA,
            0x90 => CEE_LDELEM_I1,
            0x91 => CEE_LDELEM_U1,
            0x92 => CEE_LDELEM_I2,
            0x93 => CEE_LDELEM_U2,
            0x94 => CEE_LDELEM_I4,
            0x95 => CEE_LDELEM_U4,
            0x96 => CEE_LDELEM_I8,
            0x97 => CEE_LDELEM_I,
            0x98 => CEE_LDELEM_R4,
            0x99 => CEE_LDELEM_R8,
            0x9A => CEE_LDELEM_REF,
            0x9B => CEE_STELEM_I,
            0x9C => CEE_STELEM_I1,
            0x9D => CEE_STELEM_I2,
            0x9E => CEE_STELEM_I4,
            0x9F => CEE_STELEM_I8,
            0xA0 => CEE_STELEM_R4,
            0xA1 => CEE_STELEM_R8,
            0xA2 => CEE_STELEM_REF,
            0xA3 => CEE_LDELEM,
            0xA4 => CEE_STELEM,
            0xA5 => CEE_UNBOX_ANY,
            0xA6 => CEE_UNUSED5,
            0xA7 => CEE_UNUSED6,
            0xA8 => CEE_UNUSED7,
            0xA9 => CEE_UNUSED8,
            0xAA => CEE_UNUSED9,
            0xAB => CEE_UNUSED10,
            0xAC => CEE_UNUSED11,
            0xAD => CEE_UNUSED12,
            0xAE => CEE_UNUSED13,
            0xAF => CEE_UNUSED14,
            0xB0 => CEE_UNUSED15,
            0xB1 => CEE_UNUSED16,
            0xB2 => CEE_UNUSED17,
            0xB3 => CEE_CONV_OVF_I1,
            0xB4 => CEE_CONV_OVF_U1,
            0xB5 => CEE_CONV_OVF_I2,
            0xB6 => CEE_CONV_OVF_U2,
            0xB7 => CEE_CONV_OVF_I4,
            0xB8 => CEE_CONV_OVF_U4,
            0xB9 => CEE_CONV_OVF_I8,
            0xBA => CEE_CONV_OVF_U8,
            0xBB => CEE_UNUSED50,
            0xBC => CEE_UNUSED18,
            0xBD => CEE_UNUSED19,
            0xBE => CEE_UNUSED20,
            0xBF => CEE_UNUSED21,
            0xC0 => CEE_UNUSED22,
            0xC1 => CEE_UNUSED23,
            0xC2 => CEE_REFANYVAL,
            0xC3 => CEE_CKFINITE,
            0xC4 => CEE_UNUSED24,
            0xC5 => CEE_UNUSED25,
            0xC6 => CEE_MKREFANY,
            0xC7 => CEE_UNUSED59,
            0xC8 => CEE_UNUSED60,
            0xC9 => CEE_UNUSED61,
            0xCA => CEE_UNUSED62,
            0xCB => CEE_UNUSED63,
            0xCC => CEE_UNUSED64,
            0xCD => CEE_UNUSED65,
            0xCE => CEE_UNUSED66,
            0xCF => CEE_UNUSED67,
            0xD0 => CEE_LDTOKEN,
            0xD1 => CEE_CONV_U2,
            0xD2 => CEE_CONV_U1,
            0xD3 => CEE_CONV_I,
            0xD4 => CEE_CONV_OVF_I,
            0xD5 => CEE_CONV_OVF_U,
            0xD6 => CEE_ADD_OVF,
            0xD7 => CEE_ADD_OVF_UN,
            0xD8 => CEE_MUL_OVF,
            0xD9 => CEE_MUL_OVF_UN,
            0xDA => CEE_SUB_OVF,
            0xDB => CEE_SUB_OVF_UN,
            0xDC => CEE_ENDFINALLY,
            0xDD => CEE_LEAVE,
            0xDE => CEE_LEAVE_S,
            0xDF => CEE_STIND_I,
            0xE0 => CEE_CONV_U,
            0xE1 => CEE_UNUSED26,
            0xE2 => CEE_UNUSED27,
            0xE3 => CEE_UNUSED28,
            0xE4 => CEE_UNUSED29,
            0xE5 => CEE_UNUSED30,
            0xE6 => CEE_UNUSED31,
            0xE7 => CEE_UNUSED32,
            0xE8 => CEE_UNUSED33,
            0xE9 => CEE_UNUSED34,
            0xEA => CEE_UNUSED35,
            0xEB => CEE_UNUSED36,
            0xEC => CEE_UNUSED37,
            0xED => CEE_UNUSED38,
            0xEE => CEE_UNUSED39,
            0xEF => CEE_UNUSED40,
            0xF0 => CEE_UNUSED41,
            0xF1 => CEE_UNUSED42,
            0xF2 => CEE_UNUSED43,
            0xF3 => CEE_UNUSED44,
            0xF4 => CEE_UNUSED45,
            0xF5 => CEE_UNUSED46,
            0xF6 => CEE_UNUSED47,
            0xF7 => CEE_UNUSED48,
            0xF8 => CEE_PREFIX7,
            0xF9 => CEE_PREFIX6,
            0xFA => CEE_PREFIX5,
            0xFB => CEE_PREFIX4,
            0xFC => CEE_PREFIX3,
            0xFD => CEE_PREFIX2,
            0xFE => CEE_PREFIX1,
            0xFF => CEE_PREFIXREF,
        }
    }
    pub fn from_byte_pair(pair: (u8, u8)) -> Result<Self, Error> {
        // #define STP1 0xFE
        // #define REFPRE 0xFF
        match pair {
            (0xFE, 0x00) => Ok(CEE_ARGLIST),
            (0xFE, 0x01) => Ok(CEE_CEQ),
            (0xFE, 0x02) => Ok(CEE_CGT),
            (0xFE, 0x03) => Ok(CEE_CGT_UN),
            (0xFE, 0x04) => Ok(CEE_CLT),
            (0xFE, 0x05) => Ok(CEE_CLT_UN),
            (0xFE, 0x06) => Ok(CEE_LDFTN),
            (0xFE, 0x07) => Ok(CEE_LDVIRTFTN),
            (0xFE, 0x08) => Ok(CEE_UNUSED56),
            (0xFE, 0x09) => Ok(CEE_LDARG),
            (0xFE, 0x0A) => Ok(CEE_LDARGA),
            (0xFE, 0x0B) => Ok(CEE_STARG),
            (0xFE, 0x0C) => Ok(CEE_LDLOC),
            (0xFE, 0x0D) => Ok(CEE_LDLOCA),
            (0xFE, 0x0E) => Ok(CEE_STLOC),
            (0xFE, 0x0F) => Ok(CEE_LOCALLOC),
            (0xFE, 0x10) => Ok(CEE_UNUSED57),
            (0xFE, 0x11) => Ok(CEE_ENDFILTER),
            (0xFE, 0x12) => Ok(CEE_UNALIGNED),
            (0xFE, 0x13) => Ok(CEE_VOLATILE),
            (0xFE, 0x14) => Ok(CEE_TAILCALL),
            (0xFE, 0x15) => Ok(CEE_INITOBJ),
            (0xFE, 0x16) => Ok(CEE_CONSTRAINED),
            (0xFE, 0x17) => Ok(CEE_CPBLK),
            (0xFE, 0x18) => Ok(CEE_INITBLK),
            (0xFE, 0x19) => Ok(CEE_UNUSED69),
            (0xFE, 0x1A) => Ok(CEE_RETHROW),
            (0xFE, 0x1B) => Ok(CEE_UNUSED51),
            (0xFE, 0x1C) => Ok(CEE_SIZEOF),
            (0xFE, 0x1D) => Ok(CEE_REFANYTYPE),
            (0xFE, 0x1E) => Ok(CEE_READONLY),
            (0xFE, 0x1F) => Ok(CEE_UNUSED53),
            (0xFE, 0x20) => Ok(CEE_UNUSED54),
            (0xFE, 0x21) => Ok(CEE_UNUSED55),
            (0xFE, 0x22) => Ok(CEE_UNUSED70),
            _ => Err(Error::InvalidCilOpcode),
        }
    }
}

use self::ControlFlow::*;
use self::OpcodeKind::*;
use self::OperandParams::*;
use self::StackBehaviorPop::*;
use self::StackBehaviorPush::*;

pub const CEE_NOP: Opcode = Opcode::new(
    "nop", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0x00, Next,
);
pub const CEE_BREAK: Opcode = Opcode::new(
    "break", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0x01, Break,
);
pub const CEE_LDARG_0: Opcode = Opcode::new(
    "ldarg.0", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x02, Next,
);
pub const CEE_LDARG_1: Opcode = Opcode::new(
    "ldarg.1", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x03, Next,
);
pub const CEE_LDARG_2: Opcode = Opcode::new(
    "ldarg.2", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x04, Next,
);
pub const CEE_LDARG_3: Opcode = Opcode::new(
    "ldarg.3", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x05, Next,
);
pub const CEE_LDLOC_0: Opcode = Opcode::new(
    "ldloc.0", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x06, Next,
);
pub const CEE_LDLOC_1: Opcode = Opcode::new(
    "ldloc.1", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x07, Next,
);
pub const CEE_LDLOC_2: Opcode = Opcode::new(
    "ldloc.2", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x08, Next,
);
pub const CEE_LDLOC_3: Opcode = Opcode::new(
    "ldloc.3", Pop0, Push1, InlineNone, Macro, 1, 0xFF, 0x09, Next,
);
pub const CEE_STLOC_0: Opcode = Opcode::new(
    "stloc.0", Pop1, Push0, InlineNone, Macro, 1, 0xFF, 0x0A, Next,
);
pub const CEE_STLOC_1: Opcode = Opcode::new(
    "stloc.1", Pop1, Push0, InlineNone, Macro, 1, 0xFF, 0x0B, Next,
);
pub const CEE_STLOC_2: Opcode = Opcode::new(
    "stloc.2", Pop1, Push0, InlineNone, Macro, 1, 0xFF, 0x0C, Next,
);
pub const CEE_STLOC_3: Opcode = Opcode::new(
    "stloc.3", Pop1, Push0, InlineNone, Macro, 1, 0xFF, 0x0D, Next,
);
pub const CEE_LDARG_S: Opcode = Opcode::new(
    "ldarg.s",
    Pop0,
    Push1,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x0E,
    Next,
);
pub const CEE_LDARGA_S: Opcode = Opcode::new(
    "ldarga.s",
    Pop0,
    PushI,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x0F,
    Next,
);
pub const CEE_STARG_S: Opcode = Opcode::new(
    "starg.s",
    Pop1,
    Push0,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x10,
    Next,
);
pub const CEE_LDLOC_S: Opcode = Opcode::new(
    "ldloc.s",
    Pop0,
    Push1,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x11,
    Next,
);
pub const CEE_LDLOCA_S: Opcode = Opcode::new(
    "ldloca.s",
    Pop0,
    PushI,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x12,
    Next,
);
pub const CEE_STLOC_S: Opcode = Opcode::new(
    "stloc.s",
    Pop1,
    Push0,
    ShortInlineVar,
    Macro,
    1,
    0xFF,
    0x13,
    Next,
);
pub const CEE_LDNULL: Opcode = Opcode::new(
    "ldnull", Pop0, PushRef, InlineNone, Primitive, 1, 0xFF, 0x14, Next,
);
pub const CEE_LDC_I4_M1: Opcode = Opcode::new(
    "ldc.i4.m1",
    Pop0,
    PushI,
    InlineNone,
    Macro,
    1,
    0xFF,
    0x15,
    Next,
);
pub const CEE_LDC_I4_0: Opcode = Opcode::new(
    "ldc.i4.0", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x16, Next,
);
pub const CEE_LDC_I4_1: Opcode = Opcode::new(
    "ldc.i4.1", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x17, Next,
);
pub const CEE_LDC_I4_2: Opcode = Opcode::new(
    "ldc.i4.2", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x18, Next,
);
pub const CEE_LDC_I4_3: Opcode = Opcode::new(
    "ldc.i4.3", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x19, Next,
);
pub const CEE_LDC_I4_4: Opcode = Opcode::new(
    "ldc.i4.4", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x1A, Next,
);
pub const CEE_LDC_I4_5: Opcode = Opcode::new(
    "ldc.i4.5", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x1B, Next,
);
pub const CEE_LDC_I4_6: Opcode = Opcode::new(
    "ldc.i4.6", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x1C, Next,
);
pub const CEE_LDC_I4_7: Opcode = Opcode::new(
    "ldc.i4.7", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x1D, Next,
);
pub const CEE_LDC_I4_8: Opcode = Opcode::new(
    "ldc.i4.8", Pop0, PushI, InlineNone, Macro, 1, 0xFF, 0x1E, Next,
);
pub const CEE_LDC_I4_S: Opcode = Opcode::new(
    "ldc.i4.s",
    Pop0,
    PushI,
    ShortInlineI,
    Macro,
    1,
    0xFF,
    0x1F,
    Next,
);
pub const CEE_LDC_I4: Opcode = Opcode::new(
    "ldc.i4", Pop0, PushI, InlineI, Primitive, 1, 0xFF, 0x20, Next,
);
pub const CEE_LDC_I8: Opcode = Opcode::new(
    "ldc.i8", Pop0, PushI8, InlineI8, Primitive, 1, 0xFF, 0x21, Next,
);
pub const CEE_LDC_R4: Opcode = Opcode::new(
    "ldc.r4",
    Pop0,
    PushR4,
    ShortInlineR,
    Primitive,
    1,
    0xFF,
    0x22,
    Next,
);
pub const CEE_LDC_R8: Opcode = Opcode::new(
    "ldc.r8", Pop0, PushR8, InlineR, Primitive, 1, 0xFF, 0x23, Next,
);
pub const CEE_UNUSED49: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0x24, Next,
);
pub const CEE_DUP: Opcode = Opcode::new(
    "dup", Pop1, Push1Push1, InlineNone, Primitive, 1, 0xFF, 0x25, Next,
);
pub const CEE_POP: Opcode = Opcode::new(
    "pop", Pop1, Push0, InlineNone, Primitive, 1, 0xFF, 0x26, Next,
);
pub const CEE_JMP: Opcode = Opcode::new(
    "jmp",
    Pop0,
    Push0,
    InlineMethod,
    Primitive,
    1,
    0xFF,
    0x27,
    Call,
);
pub const CEE_CALL: Opcode = Opcode::new(
    "call",
    VarPop,
    VarPush,
    InlineMethod,
    Primitive,
    1,
    0xFF,
    0x28,
    Call,
);
pub const CEE_CALLI: Opcode = Opcode::new(
    "calli", VarPop, VarPush, InlineSig, Primitive, 1, 0xFF, 0x29, Call,
);
pub const CEE_RET: Opcode = Opcode::new(
    "ret", VarPop, Push0, InlineNone, Primitive, 1, 0xFF, 0x2A, Return,
);
pub const CEE_BR_S: Opcode = Opcode::new(
    "br.s",
    Pop0,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x2B,
    Branch,
);
pub const CEE_BRFALSE_S: Opcode = Opcode::new(
    "brfalse.s",
    PopI,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x2C,
    CondBranch,
);
pub const CEE_BRTRUE_S: Opcode = Opcode::new(
    "brtrue.s",
    PopI,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x2D,
    CondBranch,
);
pub const CEE_BEQ_S: Opcode = Opcode::new(
    "beq.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x2E,
    CondBranch,
);
pub const CEE_BGE_S: Opcode = Opcode::new(
    "bge.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x2F,
    CondBranch,
);
pub const CEE_BGT_S: Opcode = Opcode::new(
    "bgt.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x30,
    CondBranch,
);
pub const CEE_BLE_S: Opcode = Opcode::new(
    "ble.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x31,
    CondBranch,
);
pub const CEE_BLT_S: Opcode = Opcode::new(
    "blt.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x32,
    CondBranch,
);
pub const CEE_BNE_UN_S: Opcode = Opcode::new(
    "bne.un.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x33,
    CondBranch,
);
pub const CEE_BGE_UN_S: Opcode = Opcode::new(
    "bge.un.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x34,
    CondBranch,
);
pub const CEE_BGT_UN_S: Opcode = Opcode::new(
    "bgt.un.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x35,
    CondBranch,
);
pub const CEE_BLE_UN_S: Opcode = Opcode::new(
    "ble.un.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x36,
    CondBranch,
);
pub const CEE_BLT_UN_S: Opcode = Opcode::new(
    "blt.un.s",
    Pop1Pop1,
    Push0,
    ShortInlineBrTarget,
    Macro,
    1,
    0xFF,
    0x37,
    CondBranch,
);
pub const CEE_BR: Opcode = Opcode::new(
    "br",
    Pop0,
    Push0,
    InlineBrTarget,
    Primitive,
    1,
    0xFF,
    0x38,
    Branch,
);
pub const CEE_BRFALSE: Opcode = Opcode::new(
    "brfalse",
    PopI,
    Push0,
    InlineBrTarget,
    Primitive,
    1,
    0xFF,
    0x39,
    CondBranch,
);
pub const CEE_BRTRUE: Opcode = Opcode::new(
    "brtrue",
    PopI,
    Push0,
    InlineBrTarget,
    Primitive,
    1,
    0xFF,
    0x3A,
    CondBranch,
);
pub const CEE_BEQ: Opcode = Opcode::new(
    "beq",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x3B,
    CondBranch,
);
pub const CEE_BGE: Opcode = Opcode::new(
    "bge",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x3C,
    CondBranch,
);
pub const CEE_BGT: Opcode = Opcode::new(
    "bgt",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x3D,
    CondBranch,
);
pub const CEE_BLE: Opcode = Opcode::new(
    "ble",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x3E,
    CondBranch,
);
pub const CEE_BLT: Opcode = Opcode::new(
    "blt",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x3F,
    CondBranch,
);
pub const CEE_BNE_UN: Opcode = Opcode::new(
    "bne.un",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x40,
    CondBranch,
);
pub const CEE_BGE_UN: Opcode = Opcode::new(
    "bge.un",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x41,
    CondBranch,
);
pub const CEE_BGT_UN: Opcode = Opcode::new(
    "bgt.un",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x42,
    CondBranch,
);
pub const CEE_BLE_UN: Opcode = Opcode::new(
    "ble.un",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x43,
    CondBranch,
);
pub const CEE_BLT_UN: Opcode = Opcode::new(
    "blt.un",
    Pop1Pop1,
    Push0,
    InlineBrTarget,
    Macro,
    1,
    0xFF,
    0x44,
    CondBranch,
);
pub const CEE_SWITCH: Opcode = Opcode::new(
    "switch",
    PopI,
    Push0,
    InlineSwitch,
    Primitive,
    1,
    0xFF,
    0x45,
    CondBranch,
);
pub const CEE_LDIND_I1: Opcode = Opcode::new(
    "ldind.i1", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x46, Next,
);
pub const CEE_LDIND_U1: Opcode = Opcode::new(
    "ldind.u1", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x47, Next,
);
pub const CEE_LDIND_I2: Opcode = Opcode::new(
    "ldind.i2", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x48, Next,
);
pub const CEE_LDIND_U2: Opcode = Opcode::new(
    "ldind.u2", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x49, Next,
);
pub const CEE_LDIND_I4: Opcode = Opcode::new(
    "ldind.i4", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x4A, Next,
);
pub const CEE_LDIND_U4: Opcode = Opcode::new(
    "ldind.u4", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x4B, Next,
);
pub const CEE_LDIND_I8: Opcode = Opcode::new(
    "ldind.i8", PopI, PushI8, InlineNone, Primitive, 1, 0xFF, 0x4C, Next,
);
pub const CEE_LDIND_I: Opcode = Opcode::new(
    "ldind.i", PopI, PushI, InlineNone, Primitive, 1, 0xFF, 0x4D, Next,
);
pub const CEE_LDIND_R4: Opcode = Opcode::new(
    "ldind.r4", PopI, PushR4, InlineNone, Primitive, 1, 0xFF, 0x4E, Next,
);
pub const CEE_LDIND_R8: Opcode = Opcode::new(
    "ldind.r8", PopI, PushR8, InlineNone, Primitive, 1, 0xFF, 0x4F, Next,
);
pub const CEE_LDIND_REF: Opcode = Opcode::new(
    "ldind.ref",
    PopI,
    PushRef,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x50,
    Next,
);
pub const CEE_STIND_REF: Opcode = Opcode::new(
    "stind.ref",
    PopIPopI,
    Push0,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x51,
    Next,
);
pub const CEE_STIND_I1: Opcode = Opcode::new(
    "stind.i1", PopIPopI, Push0, InlineNone, Primitive, 1, 0xFF, 0x52, Next,
);
pub const CEE_STIND_I2: Opcode = Opcode::new(
    "stind.i2", PopIPopI, Push0, InlineNone, Primitive, 1, 0xFF, 0x53, Next,
);
pub const CEE_STIND_I4: Opcode = Opcode::new(
    "stind.i4", PopIPopI, Push0, InlineNone, Primitive, 1, 0xFF, 0x54, Next,
);
pub const CEE_STIND_I8: Opcode = Opcode::new(
    "stind.i8", PopIPopI8, Push0, InlineNone, Primitive, 1, 0xFF, 0x55, Next,
);
pub const CEE_STIND_R4: Opcode = Opcode::new(
    "stind.r4", PopIPopR4, Push0, InlineNone, Primitive, 1, 0xFF, 0x56, Next,
);
pub const CEE_STIND_R8: Opcode = Opcode::new(
    "stind.r8", PopIPopR8, Push0, InlineNone, Primitive, 1, 0xFF, 0x57, Next,
);
pub const CEE_ADD: Opcode = Opcode::new(
    "add", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x58, Next,
);
pub const CEE_SUB: Opcode = Opcode::new(
    "sub", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x59, Next,
);
pub const CEE_MUL: Opcode = Opcode::new(
    "mul", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5A, Next,
);
pub const CEE_DIV: Opcode = Opcode::new(
    "div", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5B, Next,
);
pub const CEE_DIV_UN: Opcode = Opcode::new(
    "div.un", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5C, Next,
);
pub const CEE_REM: Opcode = Opcode::new(
    "rem", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5D, Next,
);
pub const CEE_REM_UN: Opcode = Opcode::new(
    "rem.un", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5E, Next,
);
pub const CEE_AND: Opcode = Opcode::new(
    "and", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x5F, Next,
);
pub const CEE_OR: Opcode = Opcode::new(
    "or", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x60, Next,
);
pub const CEE_XOR: Opcode = Opcode::new(
    "xor", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x61, Next,
);
pub const CEE_SHL: Opcode = Opcode::new(
    "shl", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x62, Next,
);
pub const CEE_SHR: Opcode = Opcode::new(
    "shr", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x63, Next,
);
pub const CEE_SHR_UN: Opcode = Opcode::new(
    "shr.un", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x64, Next,
);
pub const CEE_NEG: Opcode = Opcode::new(
    "neg", Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x65, Next,
);
pub const CEE_NOT: Opcode = Opcode::new(
    "not", Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0x66, Next,
);
pub const CEE_CONV_I1: Opcode = Opcode::new(
    "conv.i1", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0x67, Next,
);
pub const CEE_CONV_I2: Opcode = Opcode::new(
    "conv.i2", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0x68, Next,
);
pub const CEE_CONV_I4: Opcode = Opcode::new(
    "conv.i4", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0x69, Next,
);
pub const CEE_CONV_I8: Opcode = Opcode::new(
    "conv.i8", Pop1, PushI8, InlineNone, Primitive, 1, 0xFF, 0x6A, Next,
);
pub const CEE_CONV_R4: Opcode = Opcode::new(
    "conv.r4", Pop1, PushR4, InlineNone, Primitive, 1, 0xFF, 0x6B, Next,
);
pub const CEE_CONV_R8: Opcode = Opcode::new(
    "conv.r8", Pop1, PushR8, InlineNone, Primitive, 1, 0xFF, 0x6C, Next,
);
pub const CEE_CONV_U4: Opcode = Opcode::new(
    "conv.u4", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0x6D, Next,
);
pub const CEE_CONV_U8: Opcode = Opcode::new(
    "conv.u8", Pop1, PushI8, InlineNone, Primitive, 1, 0xFF, 0x6E, Next,
);
pub const CEE_CALLVIRT: Opcode = Opcode::new(
    "callvirt",
    VarPop,
    VarPush,
    InlineMethod,
    ObjModel,
    1,
    0xFF,
    0x6F,
    Call,
);
pub const CEE_CPOBJ: Opcode = Opcode::new(
    "cpobj", PopIPopI, Push0, InlineType, ObjModel, 1, 0xFF, 0x70, Next,
);
pub const CEE_LDOBJ: Opcode = Opcode::new(
    "ldobj", PopI, Push1, InlineType, ObjModel, 1, 0xFF, 0x71, Next,
);
pub const CEE_LDSTR: Opcode = Opcode::new(
    "ldstr",
    Pop0,
    PushRef,
    InlineString,
    ObjModel,
    1,
    0xFF,
    0x72,
    Next,
);
pub const CEE_NEWOBJ: Opcode = Opcode::new(
    "newobj",
    VarPop,
    PushRef,
    InlineMethod,
    ObjModel,
    1,
    0xFF,
    0x73,
    Call,
);
pub const CEE_CASTCLASS: Opcode = Opcode::new(
    "castclass",
    PopRef,
    PushRef,
    InlineType,
    ObjModel,
    1,
    0xFF,
    0x74,
    Next,
);
pub const CEE_ISINST: Opcode = Opcode::new(
    "isinst", PopRef, PushI, InlineType, ObjModel, 1, 0xFF, 0x75, Next,
);
pub const CEE_CONV_R_UN: Opcode = Opcode::new(
    "conv.r.un",
    Pop1,
    PushR8,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x76,
    Next,
);
pub const CEE_UNUSED58: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0x77, Next,
);
pub const CEE_UNUSED1: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0x78, Next,
);
pub const CEE_UNBOX: Opcode = Opcode::new(
    "unbox", PopRef, PushI, InlineType, Primitive, 1, 0xFF, 0x79, Next,
);
pub const CEE_THROW: Opcode = Opcode::new(
    "throw", PopRef, Push0, InlineNone, ObjModel, 1, 0xFF, 0x7A, Throw,
);
pub const CEE_LDFLD: Opcode = Opcode::new(
    "ldfld",
    PopRef,
    Push1,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x7B,
    Next,
);
pub const CEE_LDFLDA: Opcode = Opcode::new(
    "ldflda",
    PopRef,
    PushI,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x7C,
    Next,
);
pub const CEE_STFLD: Opcode = Opcode::new(
    "stfld",
    PopRefPop1,
    Push0,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x7D,
    Next,
);
pub const CEE_LDSFLD: Opcode = Opcode::new(
    "ldsfld",
    Pop0,
    Push1,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x7E,
    Next,
);
pub const CEE_LDSFLDA: Opcode = Opcode::new(
    "ldsflda",
    Pop0,
    PushI,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x7F,
    Next,
);
pub const CEE_STSFLD: Opcode = Opcode::new(
    "stsfld",
    Pop1,
    Push0,
    InlineField,
    ObjModel,
    1,
    0xFF,
    0x80,
    Next,
);
pub const CEE_STOBJ: Opcode = Opcode::new(
    "stobj", PopIPop1, Push0, InlineType, Primitive, 1, 0xFF, 0x81, Next,
);
pub const CEE_CONV_OVF_I1_UN: Opcode = Opcode::new(
    "conv.ovf.i1.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x82,
    Next,
);
pub const CEE_CONV_OVF_I2_UN: Opcode = Opcode::new(
    "conv.ovf.i2.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x83,
    Next,
);
pub const CEE_CONV_OVF_I4_UN: Opcode = Opcode::new(
    "conv.ovf.i4.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x84,
    Next,
);
pub const CEE_CONV_OVF_I8_UN: Opcode = Opcode::new(
    "conv.ovf.i8.un",
    Pop1,
    PushI8,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x85,
    Next,
);
pub const CEE_CONV_OVF_U1_UN: Opcode = Opcode::new(
    "conv.ovf.u1.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x86,
    Next,
);
pub const CEE_CONV_OVF_U2_UN: Opcode = Opcode::new(
    "conv.ovf.u2.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x87,
    Next,
);
pub const CEE_CONV_OVF_U4_UN: Opcode = Opcode::new(
    "conv.ovf.u4.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x88,
    Next,
);
pub const CEE_CONV_OVF_U8_UN: Opcode = Opcode::new(
    "conv.ovf.u8.un",
    Pop1,
    PushI8,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x89,
    Next,
);
pub const CEE_CONV_OVF_I_UN: Opcode = Opcode::new(
    "conv.ovf.i.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x8A,
    Next,
);
pub const CEE_CONV_OVF_U_UN: Opcode = Opcode::new(
    "conv.ovf.u.un",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0x8B,
    Next,
);
pub const CEE_BOX: Opcode = Opcode::new(
    "box", Pop1, PushRef, InlineType, Primitive, 1, 0xFF, 0x8C, Next,
);
pub const CEE_NEWARR: Opcode = Opcode::new(
    "newarr", PopI, PushRef, InlineType, ObjModel, 1, 0xFF, 0x8D, Next,
);
pub const CEE_LDLEN: Opcode = Opcode::new(
    "ldlen", PopRef, PushI, InlineNone, ObjModel, 1, 0xFF, 0x8E, Next,
);
pub const CEE_LDELEMA: Opcode = Opcode::new(
    "ldelema", PopRefPopI, PushI, InlineType, ObjModel, 1, 0xFF, 0x8F, Next,
);
pub const CEE_LDELEM_I1: Opcode = Opcode::new(
    "ldelem.i1",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x90,
    Next,
);
pub const CEE_LDELEM_U1: Opcode = Opcode::new(
    "ldelem.u1",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x91,
    Next,
);
pub const CEE_LDELEM_I2: Opcode = Opcode::new(
    "ldelem.i2",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x92,
    Next,
);
pub const CEE_LDELEM_U2: Opcode = Opcode::new(
    "ldelem.u2",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x93,
    Next,
);
pub const CEE_LDELEM_I4: Opcode = Opcode::new(
    "ldelem.i4",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x94,
    Next,
);
pub const CEE_LDELEM_U4: Opcode = Opcode::new(
    "ldelem.u4",
    PopRefPopI,
    PushI,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x95,
    Next,
);
pub const CEE_LDELEM_I8: Opcode = Opcode::new(
    "ldelem.i8",
    PopRefPopI,
    PushI8,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x96,
    Next,
);
pub const CEE_LDELEM_I: Opcode = Opcode::new(
    "ldelem.i", PopRefPopI, PushI, InlineNone, ObjModel, 1, 0xFF, 0x97, Next,
);
pub const CEE_LDELEM_R4: Opcode = Opcode::new(
    "ldelem.r4",
    PopRefPopI,
    PushR4,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x98,
    Next,
);
pub const CEE_LDELEM_R8: Opcode = Opcode::new(
    "ldelem.r8",
    PopRefPopI,
    PushR8,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x99,
    Next,
);
pub const CEE_LDELEM_REF: Opcode = Opcode::new(
    "ldelem.ref",
    PopRefPopI,
    PushRef,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9A,
    Next,
);
pub const CEE_STELEM_I: Opcode = Opcode::new(
    "stelem.i",
    PopRefPopIPopI,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9B,
    Next,
);
pub const CEE_STELEM_I1: Opcode = Opcode::new(
    "stelem.i1",
    PopRefPopIPopI,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9C,
    Next,
);
pub const CEE_STELEM_I2: Opcode = Opcode::new(
    "stelem.i2",
    PopRefPopIPopI,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9D,
    Next,
);
pub const CEE_STELEM_I4: Opcode = Opcode::new(
    "stelem.i4",
    PopRefPopIPopI,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9E,
    Next,
);
pub const CEE_STELEM_I8: Opcode = Opcode::new(
    "stelem.i8",
    PopRefPopIPopI8,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0x9F,
    Next,
);
pub const CEE_STELEM_R4: Opcode = Opcode::new(
    "stelem.r4",
    PopRefPopIPopR4,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0xA0,
    Next,
);
pub const CEE_STELEM_R8: Opcode = Opcode::new(
    "stelem.r8",
    PopRefPopIPopR8,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0xA1,
    Next,
);
pub const CEE_STELEM_REF: Opcode = Opcode::new(
    "stelem.ref",
    PopRefPopIPopRef,
    Push0,
    InlineNone,
    ObjModel,
    1,
    0xFF,
    0xA2,
    Next,
);
pub const CEE_LDELEM: Opcode = Opcode::new(
    "ldelem", PopRefPopI, Push1, InlineType, ObjModel, 1, 0xFF, 0xA3, Next,
);
pub const CEE_STELEM: Opcode = Opcode::new(
    "stelem",
    PopRefPopIPop1,
    Push0,
    InlineType,
    ObjModel,
    1,
    0xFF,
    0xA4,
    Next,
);
pub const CEE_UNBOX_ANY: Opcode = Opcode::new(
    "unbox.any",
    PopRef,
    Push1,
    InlineType,
    ObjModel,
    1,
    0xFF,
    0xA5,
    Next,
);
pub const CEE_UNUSED5: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xA6, Next,
);
pub const CEE_UNUSED6: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xA7, Next,
);
pub const CEE_UNUSED7: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xA8, Next,
);
pub const CEE_UNUSED8: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xA9, Next,
);
pub const CEE_UNUSED9: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAA, Next,
);
pub const CEE_UNUSED10: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAB, Next,
);
pub const CEE_UNUSED11: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAC, Next,
);
pub const CEE_UNUSED12: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAD, Next,
);
pub const CEE_UNUSED13: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAE, Next,
);
pub const CEE_UNUSED14: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xAF, Next,
);
pub const CEE_UNUSED15: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xB0, Next,
);
pub const CEE_UNUSED16: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xB1, Next,
);
pub const CEE_UNUSED17: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xB2, Next,
);
pub const CEE_CONV_OVF_I1: Opcode = Opcode::new(
    "conv.ovf.i1",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB3,
    Next,
);
pub const CEE_CONV_OVF_U1: Opcode = Opcode::new(
    "conv.ovf.u1",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB4,
    Next,
);
pub const CEE_CONV_OVF_I2: Opcode = Opcode::new(
    "conv.ovf.i2",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB5,
    Next,
);
pub const CEE_CONV_OVF_U2: Opcode = Opcode::new(
    "conv.ovf.u2",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB6,
    Next,
);
pub const CEE_CONV_OVF_I4: Opcode = Opcode::new(
    "conv.ovf.i4",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB7,
    Next,
);
pub const CEE_CONV_OVF_U4: Opcode = Opcode::new(
    "conv.ovf.u4",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB8,
    Next,
);
pub const CEE_CONV_OVF_I8: Opcode = Opcode::new(
    "conv.ovf.i8",
    Pop1,
    PushI8,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xB9,
    Next,
);
pub const CEE_CONV_OVF_U8: Opcode = Opcode::new(
    "conv.ovf.u8",
    Pop1,
    PushI8,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xBA,
    Next,
);
pub const CEE_UNUSED50: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xBB, Next,
);
pub const CEE_UNUSED18: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xBC, Next,
);
pub const CEE_UNUSED19: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xBD, Next,
);
pub const CEE_UNUSED20: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xBE, Next,
);
pub const CEE_UNUSED21: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xBF, Next,
);
pub const CEE_UNUSED22: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC0, Next,
);
pub const CEE_UNUSED23: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC1, Next,
);
pub const CEE_REFANYVAL: Opcode = Opcode::new(
    "refanyval",
    Pop1,
    PushI,
    InlineType,
    Primitive,
    1,
    0xFF,
    0xC2,
    Next,
);
pub const CEE_CKFINITE: Opcode = Opcode::new(
    "ckfinite", Pop1, PushR8, InlineNone, Primitive, 1, 0xFF, 0xC3, Next,
);
pub const CEE_UNUSED24: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC4, Next,
);
pub const CEE_UNUSED25: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC5, Next,
);
pub const CEE_MKREFANY: Opcode = Opcode::new(
    "mkrefany", PopI, Push1, InlineType, Primitive, 1, 0xFF, 0xC6, Next,
);
pub const CEE_UNUSED59: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC7, Next,
);
pub const CEE_UNUSED60: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC8, Next,
);
pub const CEE_UNUSED61: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xC9, Next,
);
pub const CEE_UNUSED62: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCA, Next,
);
pub const CEE_UNUSED63: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCB, Next,
);
pub const CEE_UNUSED64: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCC, Next,
);
pub const CEE_UNUSED65: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCD, Next,
);
pub const CEE_UNUSED66: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCE, Next,
);
pub const CEE_UNUSED67: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xCF, Next,
);
pub const CEE_LDTOKEN: Opcode = Opcode::new(
    "ldtoken", Pop0, PushI, InlineTok, Primitive, 1, 0xFF, 0xD0, Next,
);
pub const CEE_CONV_U2: Opcode = Opcode::new(
    "conv.u2", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0xD1, Next,
);
pub const CEE_CONV_U1: Opcode = Opcode::new(
    "conv.u1", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0xD2, Next,
);
pub const CEE_CONV_I: Opcode = Opcode::new(
    "conv.i", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0xD3, Next,
);
pub const CEE_CONV_OVF_I: Opcode = Opcode::new(
    "conv.ovf.i",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xD4,
    Next,
);
pub const CEE_CONV_OVF_U: Opcode = Opcode::new(
    "conv.ovf.u",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xD5,
    Next,
);
pub const CEE_ADD_OVF: Opcode = Opcode::new(
    "add.ovf", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0xD6, Next,
);
pub const CEE_ADD_OVF_UN: Opcode = Opcode::new(
    "add.ovf.un",
    Pop1Pop1,
    Push1,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xD7,
    Next,
);
pub const CEE_MUL_OVF: Opcode = Opcode::new(
    "mul.ovf", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0xD8, Next,
);
pub const CEE_MUL_OVF_UN: Opcode = Opcode::new(
    "mul.ovf.un",
    Pop1Pop1,
    Push1,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xD9,
    Next,
);
pub const CEE_SUB_OVF: Opcode = Opcode::new(
    "sub.ovf", Pop1Pop1, Push1, InlineNone, Primitive, 1, 0xFF, 0xDA, Next,
);
pub const CEE_SUB_OVF_UN: Opcode = Opcode::new(
    "sub.ovf.un",
    Pop1Pop1,
    Push1,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xDB,
    Next,
);
pub const CEE_ENDFINALLY: Opcode = Opcode::new(
    "endfinally",
    Pop0,
    Push0,
    InlineNone,
    Primitive,
    1,
    0xFF,
    0xDC,
    Return,
);
pub const CEE_LEAVE: Opcode = Opcode::new(
    "leave",
    Pop0,
    Push0,
    InlineBrTarget,
    Primitive,
    1,
    0xFF,
    0xDD,
    Branch,
);
pub const CEE_LEAVE_S: Opcode = Opcode::new(
    "leave.s",
    Pop0,
    Push0,
    ShortInlineBrTarget,
    Primitive,
    1,
    0xFF,
    0xDE,
    Branch,
);
pub const CEE_STIND_I: Opcode = Opcode::new(
    "stind.i", PopIPopI, Push0, InlineNone, Primitive, 1, 0xFF, 0xDF, Next,
);
pub const CEE_CONV_U: Opcode = Opcode::new(
    "conv.u", Pop1, PushI, InlineNone, Primitive, 1, 0xFF, 0xE0, Next,
);
pub const CEE_UNUSED26: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE1, Next,
);
pub const CEE_UNUSED27: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE2, Next,
);
pub const CEE_UNUSED28: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE3, Next,
);
pub const CEE_UNUSED29: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE4, Next,
);
pub const CEE_UNUSED30: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE5, Next,
);
pub const CEE_UNUSED31: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE6, Next,
);
pub const CEE_UNUSED32: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE7, Next,
);
pub const CEE_UNUSED33: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE8, Next,
);
pub const CEE_UNUSED34: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xE9, Next,
);
pub const CEE_UNUSED35: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xEA, Next,
);
pub const CEE_UNUSED36: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xEB, Next,
);
pub const CEE_UNUSED37: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xEC, Next,
);
pub const CEE_UNUSED38: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xED, Next,
);
pub const CEE_UNUSED39: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xEE, Next,
);
pub const CEE_UNUSED40: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xEF, Next,
);
pub const CEE_UNUSED41: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF0, Next,
);
pub const CEE_UNUSED42: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF1, Next,
);
pub const CEE_UNUSED43: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF2, Next,
);
pub const CEE_UNUSED44: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF3, Next,
);
pub const CEE_UNUSED45: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF4, Next,
);
pub const CEE_UNUSED46: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF5, Next,
);
pub const CEE_UNUSED47: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF6, Next,
);
pub const CEE_UNUSED48: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 1, 0xFF, 0xF7, Next,
);
pub const CEE_PREFIX7: Opcode = Opcode::new(
    "prefix7", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xF8, Meta,
);
pub const CEE_PREFIX6: Opcode = Opcode::new(
    "prefix6", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xF9, Meta,
);
pub const CEE_PREFIX5: Opcode = Opcode::new(
    "prefix5", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xFA, Meta,
);
pub const CEE_PREFIX4: Opcode = Opcode::new(
    "prefix4", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xFB, Meta,
);
pub const CEE_PREFIX3: Opcode = Opcode::new(
    "prefix3", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xFC, Meta,
);
pub const CEE_PREFIX2: Opcode = Opcode::new(
    "prefix2", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xFD, Meta,
);
pub const CEE_PREFIX1: Opcode = Opcode::new(
    "prefix1", Pop0, Push0, InlineNone, Internal, 1, 0xFF, 0xFE, Meta,
);
pub const CEE_PREFIXREF: Opcode = Opcode::new(
    "prefixref",
    Pop0,
    Push0,
    InlineNone,
    Internal,
    1,
    0xFF,
    0xFF,
    Meta,
);
pub const CEE_ARGLIST: Opcode = Opcode::new(
    "arglist", Pop0, PushI, InlineNone, Primitive, 2, 0xFE, 0x00, Next,
);
pub const CEE_CEQ: Opcode = Opcode::new(
    "ceq", Pop1Pop1, PushI, InlineNone, Primitive, 2, 0xFE, 0x01, Next,
);
pub const CEE_CGT: Opcode = Opcode::new(
    "cgt", Pop1Pop1, PushI, InlineNone, Primitive, 2, 0xFE, 0x02, Next,
);
pub const CEE_CGT_UN: Opcode = Opcode::new(
    "cgt.un", Pop1Pop1, PushI, InlineNone, Primitive, 2, 0xFE, 0x03, Next,
);
pub const CEE_CLT: Opcode = Opcode::new(
    "clt", Pop1Pop1, PushI, InlineNone, Primitive, 2, 0xFE, 0x04, Next,
);
pub const CEE_CLT_UN: Opcode = Opcode::new(
    "clt.un", Pop1Pop1, PushI, InlineNone, Primitive, 2, 0xFE, 0x05, Next,
);
pub const CEE_LDFTN: Opcode = Opcode::new(
    "ldftn",
    Pop0,
    PushI,
    InlineMethod,
    Primitive,
    2,
    0xFE,
    0x06,
    Next,
);
pub const CEE_LDVIRTFTN: Opcode = Opcode::new(
    "ldvirtftn",
    PopRef,
    PushI,
    InlineMethod,
    Primitive,
    2,
    0xFE,
    0x07,
    Next,
);
pub const CEE_UNUSED56: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x08, Next,
);
pub const CEE_LDARG: Opcode = Opcode::new(
    "ldarg", Pop0, Push1, InlineVar, Primitive, 2, 0xFE, 0x09, Next,
);
pub const CEE_LDARGA: Opcode = Opcode::new(
    "ldarga", Pop0, PushI, InlineVar, Primitive, 2, 0xFE, 0x0A, Next,
);
pub const CEE_STARG: Opcode = Opcode::new(
    "starg", Pop1, Push0, InlineVar, Primitive, 2, 0xFE, 0x0B, Next,
);
pub const CEE_LDLOC: Opcode = Opcode::new(
    "ldloc", Pop0, Push1, InlineVar, Primitive, 2, 0xFE, 0x0C, Next,
);
pub const CEE_LDLOCA: Opcode = Opcode::new(
    "ldloca", Pop0, PushI, InlineVar, Primitive, 2, 0xFE, 0x0D, Next,
);
pub const CEE_STLOC: Opcode = Opcode::new(
    "stloc", Pop1, Push0, InlineVar, Primitive, 2, 0xFE, 0x0E, Next,
);
pub const CEE_LOCALLOC: Opcode = Opcode::new(
    "localloc", PopI, PushI, InlineNone, Primitive, 2, 0xFE, 0x0F, Next,
);
pub const CEE_UNUSED57: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x10, Next,
);
pub const CEE_ENDFILTER: Opcode = Opcode::new(
    "endfilter",
    PopI,
    Push0,
    InlineNone,
    Primitive,
    2,
    0xFE,
    0x11,
    Return,
);
pub const CEE_UNALIGNED: Opcode = Opcode::new(
    "unaligned.",
    Pop0,
    Push0,
    ShortInlineI,
    Prefix,
    2,
    0xFE,
    0x12,
    Meta,
);
pub const CEE_VOLATILE: Opcode = Opcode::new(
    "volatile.",
    Pop0,
    Push0,
    InlineNone,
    Prefix,
    2,
    0xFE,
    0x13,
    Meta,
);
pub const CEE_TAILCALL: Opcode = Opcode::new(
    "tail.", Pop0, Push0, InlineNone, Prefix, 2, 0xFE, 0x14, Meta,
);
pub const CEE_INITOBJ: Opcode = Opcode::new(
    "initobj", PopI, Push0, InlineType, ObjModel, 2, 0xFE, 0x15, Next,
);
pub const CEE_CONSTRAINED: Opcode = Opcode::new(
    "constrained.",
    Pop0,
    Push0,
    InlineType,
    Prefix,
    2,
    0xFE,
    0x16,
    Meta,
);
pub const CEE_CPBLK: Opcode = Opcode::new(
    "cpblk",
    PopIPopIPopI,
    Push0,
    InlineNone,
    Primitive,
    2,
    0xFE,
    0x17,
    Next,
);
pub const CEE_INITBLK: Opcode = Opcode::new(
    "initblk",
    PopIPopIPopI,
    Push0,
    InlineNone,
    Primitive,
    2,
    0xFE,
    0x18,
    Next,
);
pub const CEE_UNUSED69: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x19, Next,
);
pub const CEE_RETHROW: Opcode = Opcode::new(
    "rethrow", Pop0, Push0, InlineNone, ObjModel, 2, 0xFE, 0x1A, Throw,
);
pub const CEE_UNUSED51: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x1B, Next,
);
pub const CEE_SIZEOF: Opcode = Opcode::new(
    "sizeof", Pop0, PushI, InlineType, Primitive, 2, 0xFE, 0x1C, Next,
);
pub const CEE_REFANYTYPE: Opcode = Opcode::new(
    "refanytype",
    Pop1,
    PushI,
    InlineNone,
    Primitive,
    2,
    0xFE,
    0x1D,
    Next,
);
pub const CEE_READONLY: Opcode = Opcode::new(
    "readonly.",
    Pop0,
    Push0,
    InlineNone,
    Prefix,
    2,
    0xFE,
    0x1E,
    Meta,
);
pub const CEE_UNUSED53: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x1F, Next,
);
pub const CEE_UNUSED54: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x20, Next,
);
pub const CEE_UNUSED55: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x21, Next,
);
pub const CEE_UNUSED70: Opcode = Opcode::new(
    "unused", Pop0, Push0, InlineNone, Primitive, 2, 0xFE, 0x22, Next,
);
