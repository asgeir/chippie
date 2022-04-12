mod error;
mod font;
mod instructions;

use font::FONT_ROM;
use std::default::Default;
use std::fmt::format;

pub use error::Chip8InterpreterError;
pub use instructions::Chip8Instruction;

pub const BASE_ADDRESS: u16 = 0x200;
pub const MEMORY_SIZE: u16 = 4096;
pub const STACK_SIZE: usize = 32;
pub const REGISTER_COUNT: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const MAX_ROM_SIZE: u16 = MEMORY_SIZE - BASE_ADDRESS;

const TICKS_PER_SECOND: usize = 500;
const TIMER_FREQUENCY: usize = 60;
const TIMER_TICK_INTERVAL: usize = TICKS_PER_SECOND / TIMER_FREQUENCY;

#[derive(Copy, Clone)]
pub struct Chip8InterpreterState {
    /// Registers
    pub registers: [u8; REGISTER_COUNT],
    /// Call stack
    pub stack: [u16; STACK_SIZE],
    /// Program memory
    pub memory: [u8; MEMORY_SIZE as usize],
    /// Currently displayed screen data
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    /// Currently held input keys
    pub input_keys: u32,
    /// Address for indexing operations
    pub i: u16,
    /// Sound timer
    pub st: u8,
    /// Delay timer
    pub dt: u8,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: usize,
}

impl Default for Chip8InterpreterState {
    fn default() -> Self {
        let mut state = Self {
            registers: [0; REGISTER_COUNT],
            stack: [0; STACK_SIZE],
            memory: [0; MEMORY_SIZE as usize],
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            input_keys: 0,
            i: 0,
            st: 0,
            dt: 0,
            pc: BASE_ADDRESS,
            sp: 0,
        };

        let font_mem = &mut state.memory[..FONT_ROM.len()];
        font_mem.copy_from_slice(&FONT_ROM);

        state
    }
}

pub struct Chip8Interpreter {
    state: Chip8InterpreterState,
    /// Keeps track of when to tick st and dt relative to master clock
    timer_counter: usize,
}

impl Chip8Interpreter {
    pub fn new() -> Self {
        let mut interp = Chip8Interpreter {
            state: Default::default(),
            timer_counter: 0,
        };

        interp.reset();
        interp
    }

    pub fn is_sound_playing(&self) -> bool {
        self.state.st > 1
    }

    pub fn state(&self) -> &Chip8InterpreterState {
        &self.state
    }

    pub fn reset(&mut self) {
        self.state = Default::default();
        self.timer_counter = 0;
    }

    pub fn try_read_instruction(
        &self,
        address: usize,
    ) -> Result<Chip8Instruction, Chip8InterpreterError> {
        if address >= (MEMORY_SIZE as usize) - 2 {
            return Err(Chip8InterpreterError::MemoryAccessError);
        }
        let opcode =
            ((self.state.memory[address] as u16) << 8) | (self.state.memory[address + 1] as u16);
        Chip8Instruction::try_from(opcode)
    }

    pub fn try_load_rom(&mut self, rom: &[u8]) -> Result<(), Chip8InterpreterError> {
        if rom.len() > MAX_ROM_SIZE as usize {
            return Err(Chip8InterpreterError::RomFileTooLarge);
        }

        let mem =
            &mut self.state.memory[(BASE_ADDRESS as usize)..(BASE_ADDRESS as usize + rom.len())];
        mem.copy_from_slice(rom);
        Ok(())
    }

    pub fn set_input_keys(&mut self, input_keys: u32) {
        self.state.input_keys = input_keys;
    }

    pub fn tick(&mut self) -> Result<(), Chip8InterpreterError> {
        if (self.state.pc + 1) >= MEMORY_SIZE {
            return Err(Chip8InterpreterError::ProgramCounterOutOfBounds(
                self.state.pc,
            ));
        }

        // If next instruction is WaitForKey we can only continue if we have input
        let opcode = ((self.state.memory[self.state.pc as usize] as u16) << 8)
            | (self.state.memory[self.state.pc as usize + 1] as u16);
        let instruction = Chip8Instruction::try_from(opcode)?;
        if let Chip8Instruction::WaitForKey { .. } = instruction {
            if self.state.input_keys == 0 {
                return Ok(());
            }
        }

        // Instruction preconditions have been met
        self.state.pc += 2;
        self.dispatch(instruction)?;

        self.update_timers();

        Ok(())
    }

    fn update_timers(&mut self) {
        self.timer_counter += 1;
        if self.timer_counter >= TIMER_TICK_INTERVAL {
            self.timer_counter = 0;

            if self.state.st > 0 {
                self.state.st -= 1;
            }

            if self.state.dt > 0 {
                self.state.dt -= 1;
            }
        }
    }

    fn dispatch(&mut self, instruction: Chip8Instruction) -> Result<(), Chip8InterpreterError> {
        match instruction {
            Chip8Instruction::NoOperation => Ok(()),
            Chip8Instruction::Syscall { .. } => Ok(()),
            Chip8Instruction::Random { register, mask } => {
                self.state.registers[register] = rand::random::<u8>() & mask;
                Ok(())
            }

            Chip8Instruction::Call { address } => {
                if self.state.sp > (STACK_SIZE - 1) {
                    return Err(Chip8InterpreterError::CallStackDepthExceeded);
                }

                self.state.stack[self.state.sp] = self.state.pc;
                self.state.sp += 1;
                self.state.pc = address;
                Ok(())
            }
            Chip8Instruction::Return => {
                if self.state.sp == 0 {
                    return Err(Chip8InterpreterError::CallStackEmpty);
                }

                self.state.sp -= 1;
                self.state.pc = self.state.stack[self.state.sp];
                Ok(())
            }
            Chip8Instruction::StoreRegisters { count } => {
                let mut cursor = self.state.i as usize;
                if (cursor + count) > MEMORY_SIZE.into() {
                    return Err(Chip8InterpreterError::MemoryAccessError);
                }

                for i in 0..count {
                    self.state.memory[cursor] = self.state.registers[i];
                    cursor += 1;
                }
                Ok(())
            }
            Chip8Instruction::LoadRegisters { count } => {
                let mut cursor = self.state.i as usize;
                if (cursor + count) > MEMORY_SIZE.into() {
                    return Err(Chip8InterpreterError::MemoryAccessError);
                }

                for i in 0..count {
                    self.state.registers[i] = self.state.memory[cursor as usize];
                    cursor += 1;
                }
                Ok(())
            }

            Chip8Instruction::Jump { address } => {
                self.state.pc = address;
                Ok(())
            }
            Chip8Instruction::JumpRelative { address } => {
                if (self.state.registers[0] as u16 + address) > (MEMORY_SIZE - 1) {
                    return Err(Chip8InterpreterError::MemoryAccessError);
                }

                self.state.pc = self.state.registers[0] as u16 + address;
                Ok(())
            }

            Chip8Instruction::ClearScreen => {
                self.state.screen = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
                Ok(())
            }
            Chip8Instruction::SelectCharacter { register } => {
                self.state.i = self.state.registers[register] as u16 * 5;
                Ok(())
            }
            Chip8Instruction::StoreBcd { register } => {
                if (self.state.i + 3) > MEMORY_SIZE {
                    return Err(Chip8InterpreterError::MemoryAccessError);
                }

                self.state.memory[self.state.i as usize] = self.state.registers[register] / 100;
                self.state.memory[self.state.i as usize + 1] =
                    (self.state.registers[register] / 10) % 10;
                self.state.memory[self.state.i as usize + 2] = self.state.registers[register] % 10;
                Ok(())
            }
            Chip8Instruction::Draw { x, y, len } => {
                let pos_x = self.state.registers[x] as usize;
                let pos_y = self.state.registers[y] as usize;

                let mut set_flag = false;
                for sprite_row_index in 0..len {
                    let sprite_row = self.state.memory[self.state.i as usize + sprite_row_index];

                    let pixel_pos_y = (pos_y + sprite_row_index) % SCREEN_HEIGHT;
                    let screen_line = &mut self.state.screen[pixel_pos_y];
                    for i in 0..8 {
                        let pixel_pos_x = (pos_x + 7 - i) % SCREEN_WIDTH;
                        let old_val = screen_line[pixel_pos_x];
                        screen_line[pixel_pos_x] ^= (sprite_row >> i) & 1;

                        if old_val > 0 && screen_line[pixel_pos_x] == 0 {
                            set_flag = true;
                        }
                    }
                }
                self.state.registers[15] = if set_flag { 1 } else { 0 };
                Ok(())
            }

            Chip8Instruction::SkipIfEqualValue { register, value } => {
                if self.state.registers[register] == value {
                    self.state.pc += 2;
                }
                Ok(())
            }
            Chip8Instruction::SkipIfEqualRegister { x, y } => {
                if self.state.registers[x] == self.state.registers[y] {
                    self.state.pc += 2;
                }
                Ok(())
            }
            Chip8Instruction::SkipIfNotEqualValue { register, value } => {
                if self.state.registers[register] != value {
                    self.state.pc += 2;
                }
                Ok(())
            }
            Chip8Instruction::SkipIfNotEqualRegister { x, y } => {
                if self.state.registers[x] != self.state.registers[y] {
                    self.state.pc += 2;
                }
                Ok(())
            }
            Chip8Instruction::SkipIfKeyPressed { register } => {
                if self.state.registers[register] > 15 {
                    return Err(Chip8InterpreterError::InvalidInputKey(
                        self.state.registers[register],
                    ));
                }
                if self.state.input_keys & (1u32 << self.state.registers[register]) > 0 {
                    self.state.pc += 2;
                }
                Ok(())
            }
            Chip8Instruction::SkipIfKeyNotPressed { register } => {
                if self.state.registers[register] > 15 {
                    return Err(Chip8InterpreterError::InvalidInputKey(
                        self.state.registers[register],
                    ));
                }
                if self.state.input_keys & (1u32 << self.state.registers[register]) == 0 {
                    self.state.pc += 2;
                }
                Ok(())
            }

            Chip8Instruction::SetIndex { address } => {
                self.state.i = address;
                Ok(())
            }
            Chip8Instruction::AddIndex { register } => {
                self.state.i = self
                    .state
                    .i
                    .wrapping_add(self.state.registers[register] as u16);
                Ok(())
            }

            Chip8Instruction::LoadValue { register, value } => {
                self.state.registers[register] = value;
                Ok(())
            }
            Chip8Instruction::Copy { x, y } => {
                self.state.registers[x] = self.state.registers[y];
                Ok(())
            }
            Chip8Instruction::ReadDelayTimer { register } => {
                self.state.registers[register] = self.state.dt;
                Ok(())
            }
            Chip8Instruction::SetDelayTimer { register } => {
                self.state.dt = self.state.registers[register];
                Ok(())
            }
            Chip8Instruction::SetSoundTimer { register } => {
                self.state.st = self.state.registers[register];
                Ok(())
            }
            Chip8Instruction::WaitForKey { register } => {
                if self.state.input_keys == 0 {
                    Err(Chip8InterpreterError::ExpectingInputKey)
                } else {
                    for i in 0..16 {
                        if (self.state.input_keys & (1u32 << i)) > 0 {
                            self.state.registers[register] = i;
                            break;
                        }
                    }
                    Ok(())
                }
            }

            Chip8Instruction::AddValue { register, value } => {
                let (sum, carry) = self.state.registers[register].overflowing_add(value);
                self.state.registers[register] = sum;
                self.state.registers[15] = if carry { 0 } else { 1 };
                Ok(())
            }
            Chip8Instruction::AddRegister { x, y } => {
                let (sum, carry) = self.state.registers[x].overflowing_add(self.state.registers[y]);
                self.state.registers[x] = sum;
                self.state.registers[15] = if carry { 1 } else { 0 };
                Ok(())
            }
            Chip8Instruction::SubtractVxVy { x, y } => {
                let (sub, borrow) =
                    self.state.registers[x].overflowing_sub(self.state.registers[y]);
                self.state.registers[x] = sub;
                self.state.registers[15] = if borrow { 0 } else { 1 };
                Ok(())
            }
            Chip8Instruction::SubtractVyVx { x, y } => {
                let (sub, borrow) =
                    self.state.registers[y].overflowing_sub(self.state.registers[x]);
                self.state.registers[x] = sub;
                self.state.registers[15] = if borrow { 0 } else { 1 };
                Ok(())
            }

            Chip8Instruction::Or { x, y } => {
                self.state.registers[x] = self.state.registers[x] | self.state.registers[y];
                Ok(())
            }
            Chip8Instruction::And { x, y } => {
                self.state.registers[x] = self.state.registers[x] & self.state.registers[y];
                Ok(())
            }
            Chip8Instruction::Xor { x, y } => {
                self.state.registers[x] = self.state.registers[x] ^ self.state.registers[y];
                Ok(())
            }
            Chip8Instruction::ShiftRight { x, .. } => {
                let carry = self.state.registers[x] & 1;
                self.state.registers[x] = self.state.registers[x] >> 1;
                self.state.registers[15] = carry;
                Ok(())
            }
            Chip8Instruction::ShiftLeft { x, .. } => {
                let carry = self.state.registers[x] >> 7;
                self.state.registers[x] = self.state.registers[x] << 1;
                self.state.registers[15] = carry;
                Ok(())
            }
        }
    }
}
