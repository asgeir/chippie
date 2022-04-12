use thiserror::Error;

#[derive(Error, Debug)]
pub enum Chip8InterpreterError {
    #[error("ROM file is too large to load")]
    RomFileTooLarge,
    #[error("Invalid instruction")]
    InvalidInstruction(u16),
    #[error("Program counter out of bounds")]
    ProgramCounterOutOfBounds(u16),
    #[error("Call stack depth exceeded")]
    CallStackDepthExceeded,
    #[error("Call stack is empty")]
    CallStackEmpty,
    #[error("Memory access error")]
    MemoryAccessError,
    #[error("Invalid input key")]
    InvalidInputKey(u8),
    #[error("Expecting input key")]
    ExpectingInputKey,
}
