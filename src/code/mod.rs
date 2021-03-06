use std::collections::HashMap;

use crate::error;
use crate::error::Error;

lazy_static! {
    static ref DEFINITIONS: HashMap<OpCode, Definition<isize>> = {
        let mut m = HashMap::new();
        m.insert(
            OpCode::OpConstant,
            Definition::new("OpConstant".to_string(), 2),
        );
        m
    };
}

// u8 - byte
type Instructions = Vec<u8>;
type Opcode = u8;

// OpCodes for the VM
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum OpCode {
    OpConstant = 0,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Definition<T> {
    name: String,
    operand_widths: Vec<T>,
}

impl Definition<isize> {
    fn new(name: String, operand_len: isize) -> Definition<isize> {
        Definition {
            name,
            operand_widths: vec![operand_len],
        }
    }
}

fn lookup(op: OpCode) -> Result<&'static Definition<isize>, error::Error> {
    match DEFINITIONS.get(&op) {
        None => Err(Error {
            code: 1,
            message: "not found".to_string(),
        }),
        Some(val) => Ok(val),
    }
}

// according to the opcode split the operand into 1 or 2 8bit words
fn make(op: OpCode, operands: Vec<isize>) -> Vec<u8> {
    let def = DEFINITIONS.get(&op).unwrap();

    let mut instructions_len = 1;

    for k in &def.operand_widths {
        instructions_len += k;
    }

    let mut instruction: Vec<u8> = Vec::with_capacity(instructions_len as usize);
    instruction.push(op as u8);

    for (i, o) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        match width {
            // u8
            1 => {}
            // u16
            // we need to convert into 2 bytes
            2 => {
                instruction.push((*o >> 8) as u8); // upper 8bits
                instruction.push(*o as u8); // lower 8bits
            }
            _ => {}
        }
    }

    instruction
}

#[cfg(test)]
mod tests {
    use crate::code::make;
    use crate::code::OpCode::OpConstant;

    #[test]
    fn test_make() {
        let res = make(OpConstant, vec![65534]);
        assert_eq!(res, vec![0, 255, 254]);
    }

    #[test]
    fn test_make_2() {
        let res = make(OpConstant, vec![65536]);
        assert_eq!(res, vec![0, 0, 0]);
    }
}
