use crate::cil::{Error, Opcode};

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

pub enum Operand {
    None,
    TypeReference,
    CallSite,
    MethodReference,
    FieldReference,
    StringValue,
    SbyteValue,
    ByteValue,
    IntValue,
    LongValue,
    FloatValue,
    DoubleValue,
    Target,
    Targets,
    VariableDefinition,
    ParameterDefinition,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Operand,
}

impl Instruction {
    /// Attempts to parse the first instruction at the beginning
    /// of the given byte array. Array must be at a valid
    pub fn parse_from_bytes(method_header: *const u8) -> Result<Self, Error> {
        todo!()
    }
}
