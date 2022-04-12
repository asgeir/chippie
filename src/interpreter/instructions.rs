use super::error::Chip8InterpreterError;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Chip8Instruction {
    /// No operation
    NoOperation,

    /// Syscall to machine-code function
    Syscall { address: u16 },
    /// Generate random byte and store the masked value in register
    Random { register: usize, mask: u8 },

    /// Call subroutine at address
    Call { address: u16 },
    /// Return from subroutine
    Return,
    /// Store register values in memory
    StoreRegisters { count: usize },
    /// Load register values from memory
    LoadRegisters { count: usize },

    ///  Jump to address
    Jump { address: u16 },
    /// Jump relative to value stored in register 0
    JumpRelative { address: u16 },

    /// Clear screen
    ClearScreen,
    /// Select font character sprite to correspond with value stored in register
    SelectCharacter { register: usize },
    /// Store BCD representation of value from register
    StoreBcd { register: usize },
    /// Display sprite
    Draw { x: usize, y: usize, len: usize },

    /// Skip next instruction if value stored in register is equal to value
    SkipIfEqualValue { register: usize, value: u8 },
    /// Skip next instruction if value stored in register x is equal to value stored in register y
    SkipIfEqualRegister { x: usize, y: usize },
    /// Skip next instruction if value stored in register is not equal to value
    SkipIfNotEqualValue { register: usize, value: u8 },
    /// Skip next instruction if value in register x is not equal to value in register y
    SkipIfNotEqualRegister { x: usize, y: usize },
    /// Skip next instruction if key is pressed (key value is stored in register)
    SkipIfKeyPressed { register: usize },
    /// Skip next instruction if key is not pressed (key value is stored in register)
    SkipIfKeyNotPressed { register: usize },

    /// Set register I to value
    SetIndex { address: u16 },
    /// Add value from register to index address
    AddIndex { register: usize },

    /// Load value into register
    LoadValue { register: usize, value: u8 },
    /// Copy value from register y into register x
    Copy { x: usize, y: usize },
    /// Copy delay timer value into register
    ReadDelayTimer { register: usize },
    /// Set delay timer to value from register
    SetDelayTimer { register: usize },
    /// Set sound timer to value from register
    SetSoundTimer { register: usize },
    /// Wait for key press then store the key value in register
    WaitForKey { register: usize },

    /// Add value to value stored in register
    AddValue { register: usize, value: u8 },
    /// Add value from register y to value stored in register x
    AddRegister { x: usize, y: usize },
    /// Subtract value from register y from value stored in register x
    SubtractVxVy { x: usize, y: usize },
    /// Subtract value in register x from value in register y, storing the result in register x
    SubtractVyVx { x: usize, y: usize },

    /// Or value from register y into value stored in register x
    Or { x: usize, y: usize },
    /// And value from register y into value stored in register x
    And { x: usize, y: usize },
    /// Xor value from register y into value stored in register x
    Xor { x: usize, y: usize },
    /// Copy value from register y into register x, then shift value in register x right by one bit
    ShiftRight { x: usize, y: usize },
    /// Copy value from register y into register x, then shift value in register x left by one bit
    ShiftLeft { x: usize, y: usize },
}

impl Display for Chip8Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Instruction::NoOperation => {
                write!(f, "{}", "NoOp")
            }
            Chip8Instruction::Syscall { address } => {
                write!(f, "Syscall {:04x}", address)
            }
            Chip8Instruction::Random { register, mask } => {
                write!(f, "V{:x} := random & 0x{:02x}", register, mask)
            }
            Chip8Instruction::Call { address } => {
                write!(f, "Call {:04x}", address)
            }
            Chip8Instruction::Return => {
                write!(f, "{}", "Return")
            }
            Chip8Instruction::StoreRegisters { count } => {
                write!(f, "StoreRegisters({})", count)
            }
            Chip8Instruction::LoadRegisters { count } => {
                write!(f, "LoadRegisters({})", count)
            }
            Chip8Instruction::Jump { address } => {
                write!(f, "Jump {:04x}", address)
            }
            Chip8Instruction::JumpRelative { address } => {
                write!(f, "Jump {:04x} + V0", address)
            }
            Chip8Instruction::ClearScreen => {
                write!(f, "{}", "ClearScreen")
            }
            Chip8Instruction::SelectCharacter { register } => {
                write!(f, "SelectCharacter(V{:x})", register)
            }
            Chip8Instruction::StoreBcd { register } => {
                write!(f, "StoreBcd(V{:x})", register)
            }
            Chip8Instruction::Draw { x, y, len } => {
                write!(f, "Draw(x: {}, y: {}, length: {})", x, y, len)
            }
            Chip8Instruction::SkipIfEqualValue { register, value } => {
                write!(f, "SkipNext if V{:x} == {}", register, value)
            }
            Chip8Instruction::SkipIfEqualRegister { x, y } => {
                write!(f, "SkipNext if V{:x} == V{:x}", x, y)
            }
            Chip8Instruction::SkipIfNotEqualValue { register, value } => {
                write!(f, "SkipNext if V{:x} != {}", register, value)
            }
            Chip8Instruction::SkipIfNotEqualRegister { x, y } => {
                write!(f, "SkipNext if V{:x} == V{:x}", x, y)
            }
            Chip8Instruction::SkipIfKeyPressed { register } => {
                write!(f, "SkipNext if Key[V{:x}] == Pressed", register)
            }
            Chip8Instruction::SkipIfKeyNotPressed { register } => {
                write!(f, "SkipNext if Key[V{:x}] == Pressed", register)
            }
            Chip8Instruction::SetIndex { address } => {
                write!(f, "I := {:04x}", address)
            }
            Chip8Instruction::AddIndex { register } => {
                write!(f, "I += V{:x}", register)
            }
            Chip8Instruction::LoadValue { register, value } => {
                write!(f, "V{:x} := {}", register, value)
            }
            Chip8Instruction::Copy { x, y } => {
                write!(f, "V{:x} := V{:x}", x, y)
            }
            Chip8Instruction::ReadDelayTimer { register } => {
                write!(f, "V{:x} += DT", register)
            }
            Chip8Instruction::SetDelayTimer { register } => {
                write!(f, "DT := V{:x}", register)
            }
            Chip8Instruction::SetSoundTimer { register } => {
                write!(f, "ST := V{:x}", register)
            }
            Chip8Instruction::WaitForKey { register } => {
                write!(f, "WaitForKey; V{:x} = Key", register)
            }
            Chip8Instruction::AddValue { register, value } => {
                write!(f, "V{:x} += {}", register, value)
            }
            Chip8Instruction::AddRegister { x, y } => {
                write!(f, "V{:x} += V{:x}", x, y)
            }
            Chip8Instruction::SubtractVxVy { x, y } => {
                write!(f, "V{:x} := V{:x} - V{:x}", x, x, y)
            }
            Chip8Instruction::SubtractVyVx { x, y } => {
                write!(f, "V{:x} := V{:x} - V{:x}", x, y, x)
            }
            Chip8Instruction::Or { x, y } => {
                write!(f, "V{:x} := V{:x} | V{:x}", x, x, y)
            }
            Chip8Instruction::And { x, y } => {
                write!(f, "V{:x} := V{:x} & V{:x}", x, x, y)
            }
            Chip8Instruction::Xor { x, y } => {
                write!(f, "V{:x} := V{:x} ^ V{:x}", x, x, y)
            }
            Chip8Instruction::ShiftRight { x, y } => {
                write!(f, "V{:x} := V{:x} >> 1", x, y)
            }
            Chip8Instruction::ShiftLeft { x, y } => {
                write!(f, "V{:x} := V{:x} << 1", x, y)
            }
        }
    }
}

impl TryFrom<u16> for Chip8Instruction {
    type Error = Chip8InterpreterError;

    fn try_from(opcode: u16) -> Result<Self, Self::Error> {
        match (opcode >> 12) as u8 {
            0x0 => match opcode {
                0x00e0 => Ok(Chip8Instruction::ClearScreen),
                0x00ee => Ok(Chip8Instruction::Return),
                _ => Ok(Chip8Instruction::NoOperation),
                // _ => Ok(Chip8Instruction::Syscall {
                //     address: opcode & 0x0fff,
                // }),
            },
            0x1 => Ok(Chip8Instruction::Jump {
                address: opcode & 0x0fff,
            }),
            0x2 => Ok(Chip8Instruction::Call {
                address: opcode & 0x0fff,
            }),
            0x3 => Ok(Chip8Instruction::SkipIfEqualValue {
                register: ((opcode >> 8) & 0x0f) as usize,
                value: (opcode & 0xff) as u8,
            }),
            0x4 => Ok(Chip8Instruction::SkipIfNotEqualValue {
                register: ((opcode >> 8) & 0x0f) as usize,
                value: (opcode & 0xff) as u8,
            }),
            0x5 => {
                // TODO: invalid instruction if last nibble != 0?
                Ok(Chip8Instruction::SkipIfEqualRegister {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                })
            }
            0x6 => Ok(Chip8Instruction::LoadValue {
                register: ((opcode >> 8) & 0x0f) as usize,
                value: (opcode & 0xff) as u8,
            }),
            0x7 => Ok(Chip8Instruction::AddValue {
                register: ((opcode >> 8) & 0x0f) as usize,
                value: (opcode & 0xff) as u8,
            }),
            0x8 => match opcode & 0x000f {
                0x0 => Ok(Chip8Instruction::Copy {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x1 => Ok(Chip8Instruction::Or {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x2 => Ok(Chip8Instruction::And {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x3 => Ok(Chip8Instruction::Xor {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x4 => Ok(Chip8Instruction::AddRegister {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x5 => Ok(Chip8Instruction::SubtractVxVy {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x6 => Ok(Chip8Instruction::ShiftRight {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0x7 => Ok(Chip8Instruction::SubtractVyVx {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                0xe => Ok(Chip8Instruction::ShiftLeft {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                }),
                _ => Err(Chip8InterpreterError::InvalidInstruction(opcode)),
            },
            0x9 => {
                // TODO: invalid instruction if last nibble != 0?
                Ok(Chip8Instruction::SkipIfNotEqualRegister {
                    x: ((opcode >> 8) & 0x0f) as usize,
                    y: ((opcode >> 4) & 0x0f) as usize,
                })
            }
            0xa => Ok(Chip8Instruction::SetIndex {
                address: opcode & 0x0fff,
            }),
            0xb => Ok(Chip8Instruction::JumpRelative {
                address: opcode & 0x0fff,
            }),
            0xc => Ok(Chip8Instruction::Random {
                register: ((opcode >> 8) & 0x0f) as usize,
                mask: (opcode & 0xff) as u8,
            }),
            0xd => Ok(Chip8Instruction::Draw {
                x: ((opcode >> 8) & 0x0f) as usize,
                y: ((opcode >> 4) & 0x0f) as usize,
                len: (opcode & 0x0f) as usize,
            }),
            0xe => {
                if (opcode & 0xff) == 0x9e {
                    Ok(Chip8Instruction::SkipIfKeyPressed {
                        register: ((opcode >> 8) & 0x0f) as usize,
                    })
                } else if (opcode & 0xff) == 0xa1 {
                    Ok(Chip8Instruction::SkipIfKeyNotPressed {
                        register: ((opcode >> 8) & 0x0f) as usize,
                    })
                } else {
                    Err(Chip8InterpreterError::InvalidInstruction(opcode))
                }
            }
            0xf => {
                let register = ((opcode >> 8) & 0x0f) as usize;
                match opcode & 0xff {
                    0x07 => Ok(Chip8Instruction::ReadDelayTimer { register }),
                    0x0a => Ok(Chip8Instruction::WaitForKey { register }),
                    0x15 => Ok(Chip8Instruction::SetDelayTimer { register }),
                    0x18 => Ok(Chip8Instruction::SetSoundTimer { register }),
                    0x1e => Ok(Chip8Instruction::AddIndex { register }),
                    0x29 => Ok(Chip8Instruction::SelectCharacter { register }),
                    0x33 => Ok(Chip8Instruction::StoreBcd { register }),
                    0x55 => Ok(Chip8Instruction::StoreRegisters {
                        count: register + 1,
                    }),
                    0x65 => Ok(Chip8Instruction::LoadRegisters {
                        count: register + 1,
                    }),
                    _ => Err(Chip8InterpreterError::InvalidInstruction(opcode)),
                }
            }
            _ => unreachable!(),
        }
    }
}
