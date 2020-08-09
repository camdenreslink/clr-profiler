use crate::cil::{
    il_f32, il_f64, il_i16, il_i32, il_i64, il_i8, il_u32, il_u8, Error, Opcode, OperandParams,
};

// <int32 (target)>
// beq <int32 (target)>

// <int8 (target)>
// beq.s <int8 (target)>

// <typeTok>
// box <typeTok>

// <method>
// call <method>

// <callsitedescr>
// calli <callsitedescr>

// <class>
// castclass <class>

// <thisType>
// constrained. <thisType>

// <field>
// ldfld <field>

// <uint8 (num)>
// ldarg.s <uint8 (num)>

// <uint16 (num)>
// ldarg <uint16 (num)>

// <int8 (num)>
// ldc.i4.s <int8 (num)>

// <int32 (num)>
// ldc.i4 <int32 (num)>

// <int64 (num)>
// ldc.i8 <int64 (num)>

// <float32 (num)>
// ldc.r4 <float32 (num)>

// <float64 (num)>
// ldc.r8 <float64 (num)>

// <uint16 (indx)>
// ldloc <uint16 (indx)>

// <uint8 (indx)>
// ldloc.s <uint8 (indx)>

// <string>
// ldstr <string>

// <etype>
// newarr <etype>

// <ctor>
// newobj <ctor>

// { typecheck, rangecheck, nullcheck }
// no. { typecheck, rangecheck, nullcheck }

// <type>
// refanyval <type>

// <uint32, int32, int32 (t1..tN)>
// switch <uint32, int32, int32 (t1..tN)>

// (alignment)
// unaligned. (alignment)

// <valuetype>
// unbox <valuetype>
#[derive(Debug)]
pub enum Operand {
    InlineNone,
    ShortInlineVar(u8),
    InlineVar(i16),
    ShortInlineI(i8),
    InlineI(i32),
    InlineI8(i64),
    ShortInlineR(f32),
    InlineR(f64),
    InlineMethod(u32),
    InlineSig(u32),
    ShortInlineBrTarget(i8),
    InlineBrTarget(i32),
    InlineSwitch(u32, Vec<i32>),
    InlineType(u32),
    InlineString(u32),
    InlineField(u32),
    InlineTok(u32),
}
impl Operand {
    pub fn length(&self) -> usize {
        match self {
            Self::InlineNone => 0,
            Self::ShortInlineVar(_) => 1,
            Self::InlineVar(_) => 2,
            Self::ShortInlineI(_) => 1,
            Self::InlineI(_) => 4,
            Self::InlineI8(_) => 8,
            Self::ShortInlineR(_) => 4,
            Self::InlineR(_) => 8,
            Self::InlineMethod(_) => 4,
            Self::InlineSig(_) => 4,
            Self::ShortInlineBrTarget(_) => 1,
            Self::InlineBrTarget(_) => 4,
            Self::InlineSwitch(length, _) => *length as usize + 1,
            Self::InlineType(_) => 4,
            Self::InlineString(_) => 4,
            Self::InlineField(_) => 4,
            Self::InlineTok(_) => 4,
        }
    }
}
#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Operand,
    pub length: usize,
}

impl Instruction {
    /// Attempts to parse the first instruction at the beginning
    /// of the given byte array. Array must be at a valid instruction
    /// boundary.
    pub fn parse_from_bytes(il: &[u8]) -> Result<Self, Error> {
        let byte_1 = il_u8(il, 0)?;
        let opcode = if byte_1 == 0xFE {
            // In this case, we have a multibyte opcode
            let byte_2 = il_u8(il, 1)?;
            Opcode::from_byte_pair((byte_1, byte_2))
        } else {
            Ok(Opcode::from_byte(byte_1))
        }?;
        let operand_index = opcode.length as usize;
        let operand = match &opcode.operand_params {
            OperandParams::InlineNone => Operand::InlineNone,
            OperandParams::ShortInlineVar => {
                let val = il_u8(il, operand_index)?;
                Operand::ShortInlineVar(val)
            }
            OperandParams::InlineVar => {
                let val = il_i16(il, operand_index)?;
                Operand::InlineVar(val)
            }
            OperandParams::ShortInlineI => {
                let val = il_i8(il, operand_index)?;
                Operand::ShortInlineI(val)
            }
            OperandParams::InlineI => {
                let val = il_i32(il, operand_index)?;
                Operand::InlineI(val)
            }
            OperandParams::InlineI8 => {
                let val = il_i64(il, operand_index)?;
                Operand::InlineI8(val)
            }
            OperandParams::ShortInlineR => {
                let val = il_f32(il, operand_index)?;
                Operand::ShortInlineR(val)
            }
            OperandParams::InlineR => {
                let val = il_f64(il, operand_index)?;
                Operand::InlineR(val)
            }
            OperandParams::InlineMethod => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineMethod(val)
            }
            OperandParams::InlineSig => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineSig(val)
            }
            OperandParams::ShortInlineBrTarget => {
                let val = il_i8(il, operand_index)?;
                Operand::ShortInlineBrTarget(val)
            }
            OperandParams::InlineBrTarget => {
                let val = il_i32(il, operand_index)?;
                Operand::InlineBrTarget(val)
            }
            OperandParams::InlineSwitch => {
                let length = il_u32(il, operand_index)?;
                let mut val: Vec<i32> = Vec::with_capacity(length as usize);
                for i in 1..=length {
                    let target_index = operand_index + ((i * 4) as usize);
                    let target = il_i32(il, target_index)?;
                    val.push(target);
                }
                Operand::InlineSwitch(length, val)
            }
            OperandParams::InlineType => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineType(val)
            }
            OperandParams::InlineString => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineString(val)
            }
            OperandParams::InlineField => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineField(val)
            }
            OperandParams::InlineTok => {
                let val = il_u32(il, operand_index)?;
                Operand::InlineTok(val)
            }
        };
        let length = opcode.length as usize + operand.length();
        Ok(Instruction {
            opcode: opcode,
            operand: operand,
            length,
        })
    }
}
