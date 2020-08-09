use crate::cil::{
    il_f32, il_f64, il_i16, il_i32, il_i64, il_i8, il_u32, il_u8, Error, Opcode, OperandParams,
};

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
            Self::InlineSwitch(length, _) => ((*length + 1) * 4) as usize,
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
    pub fn from_bytes(il: &[u8]) -> Result<Self, Error> {
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
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        if self.opcode.length == 1 {
            bytes.push(self.opcode.byte_2);
        } else if self.opcode.length == 2 {
            bytes.push(self.opcode.byte_1);
            bytes.push(self.opcode.byte_2);
        }
        match &self.operand {
            Operand::InlineNone => (),
            Operand::ShortInlineVar(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineVar(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineI(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineI(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineI8(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineR(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineR(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineMethod(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineSig(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::ShortInlineBrTarget(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineBrTarget(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineSwitch(length, val) => {
                bytes.extend_from_slice(&length.to_le_bytes());
                println!(
                    "{}!!! {}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!! {:?}!!!",
                    length,
                    val.len(),
                    length.to_le_bytes(),
                    val[0].to_le_bytes(),
                    val[1].to_le_bytes(),
                    val[2].to_le_bytes(),
                    val[3].to_le_bytes(),
                    val[4].to_le_bytes(),
                );
                let mut target_bytes: Vec<u8> =
                    val.iter().flat_map(|s| s.to_le_bytes().to_vec()).collect();
                bytes.append(&mut target_bytes);
            }
            Operand::InlineType(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineString(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineField(val) => bytes.extend_from_slice(&val.to_le_bytes()),
            Operand::InlineTok(val) => bytes.extend_from_slice(&val.to_le_bytes()),
        }

        bytes
    }
}
