use std::{fs::File, io::Read};

use rand::random;

const FONT_SET: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

#[derive(PartialEq)]
pub enum STATE{
    RUNNING, STOPED, PAUSED
}

pub struct Chip8{
    pub key: [bool; 16],

    pub display: [[bool; 32]; 64],
    pub mem: [u8; 4096],
    pub v: [u8; 16],
    pub stack: [u16; 16],

    pub pc: u16,
    pub sp: u16,
    pub i: u16,

    pub dt: u8,
    pub st: u8,

    pub kp: u8,
    pub ku: bool,

    pub state: STATE
}

impl Chip8 {
   
    fn loadFonts(&mut self){
        
        for i in 0..FONT_SET.len() {
            self.mem[i] = FONT_SET[i];
        }
    }
    fn reset(&mut self) {
        self.key = [false; 16];

        self.mem = [0; 4096];
        self.v = [0; 16];
        self.stack = [0; 16];

        self.pc = 0x200;
        self.sp = 0;
        self.i = 0;

        self.dt = 0;
        self.st = 0;

        self.loadFonts();
        self.clear_display();
    }
    //mem
    fn write(&mut self, adress: usize, value: u8){
        self.mem[adress] = value; 
    }
    fn get(&mut self, index: u16) -> u8{
        return self.mem[index as usize];
    }
    fn clearMem(&mut self){
        for i in 0x200..4096 {
           self.write(i, 0);
        }
    }
    //cpu
    fn fetch(&mut self) -> u16{
        let higher_byte = self.mem[self.pc as usize] as u16;
        let lower_byte = self.mem[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;

        return op;
    }

    fn next(&mut self){
        if(self.pc+2 < 4090){
            self.pc += 2;
        }
    }

    fn addRoutine(&mut self, adress: u16 ){
        print!("added routine");
        self.stack[self.sp as usize] = adress;
        self.sp += 1;
    }
    fn returnRoutine(&mut self) -> u16{
        print!("returned");
        self.sp -= 1;
        return self.stack[self.sp as usize];
    }
    fn jump(&mut self, adress: u16){
        self.pc = adress;
    }

    fn getV(&mut self, v: u16) -> u16{
        return self.v[v as usize] as u16;
    }
    fn setV(&mut self, v: u16, value: u16){
        self.v[v as usize] = value as u8;
    }
    //timer
    fn decreaseDT(&mut self){
        if(self.dt > 0){
            self.dt -= 1;
        }
    }
    fn decreaseST(&mut self){
        if(self.st > 0){
            print!("beeeeeep");
            self.st -= 1;
        }
    }

    //display
    fn set_pixel(&mut self, x: usize, y: usize) {
        self.display[x][y] ^= self.display[x][y];
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.display[x][y]
    }

    fn clear_display(&mut self) {
        for row in 0..32 {
            for col in 0..64 {
                self.display[col][row] = false;
            }
        }
    }
    

    //debug stuff
    fn dump_memory(&mut self, i1: usize, i2: usize){
        for i in i1..i2 {
            println!("memory at adress {i} 0x{:x} ",self.mem[i])
        }
    }
}

pub fn start() -> Chip8{
    print!("starting chip8");
    let mut chip8: Chip8 = Chip8 { 
        key: [false; 16],

        display: [[false; 32]; 64],
        mem: [0; 4096],
        v: [0; 16],
        stack: [0; 16],

        pc: 0x200, 
        sp: 0,
        i: 0,

        dt: 0,
        st: 0,

        kp: 0,
        ku: false,

        state: STATE::STOPED
    };
    chip8.loadFonts();

    print!("started");
    return chip8;

}

pub fn load(mut file: File, chip8: &mut Chip8){
    chip8.reset();
    // Read the contents of the file into a byte array
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Write each byte in the buffer to the virtual memory starting at address 0x200
    for (i, &byte) in buffer.iter().enumerate() {
        let address = 0x200 + i;
        chip8.write(address, byte);
    }
    chip8.state = STATE::RUNNING;
    chip8.write(0x1ff, 4);
    chip8.write(0x1fe, 1);

}

pub fn cycle(chip8: &mut Chip8, instructions: usize) { 
    if(chip8.state == STATE::RUNNING){
        for i in 0..instructions {
            decode(chip8);
        }
        chip8.decreaseDT();
        chip8.decreaseST();
    }
    
}

fn decode(chip8: &mut Chip8){
    let opcode = chip8.fetch();
    execute(chip8, opcode);
   
}

fn execute(chip8: &mut Chip8, opcode: u16){
    /* values table (the copy and paste power :P )
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    let n = opcode & 0x000F;
    let nn = opcode & 0x00FF;
    let nnn = opcode & 0x0FFF;
    */
    println!("> executing opcode [0x{:x}] \n  >> pc[0x{:x}] <<",opcode, chip8.pc);
   
    match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x0000 => chip8.next(),
            0x00E0 => x00E0(chip8),
            0x00EE => x00EE(chip8, opcode),
            _ => println!("error, opcode 0x{:x} isn't implemented or does not exist", opcode),
        },
        0x1000 => x1NNN(chip8, opcode),
        0x2000 => x2NNN(chip8, opcode),
        0x3000 => x3XNN(chip8, opcode),
        0x4000 => x4XNN(chip8, opcode),
        0x5000 => x5XY0(chip8, opcode),
        0x6000 => x6XNN(chip8, opcode),
        0x7000 => x7XNN(chip8, opcode),
        0x8000 => match opcode & 0x000F {
            0x0000 => x8XY0(chip8, opcode),
            0x0001 => x8XY1(chip8, opcode),
            0x0002 => x8XY2(chip8, opcode),
            0x0003 => x8XY3(chip8, opcode),
            0x0004 => x8XY4(chip8, opcode),
            0x0005 => x8XY5(chip8, opcode),
            0x0006 => x8XY6(chip8, opcode),
            0x0007 => x8XY7(chip8, opcode),
            0x000E => x8XYE(chip8, opcode),
            _ => println!("error, opcode 0x{:x} isn't implemented or does not exist", opcode),
        },
        0x9000 => x9XY0(chip8, opcode),
        0xA000 => xANNN(chip8, opcode),
        0xB000 => xBNNN(chip8, opcode),
        0xC000 => xCXNN(chip8, opcode),
        0xD000 => xDXYN(chip8, opcode),
        0xE000 => match opcode & 0x00FF {
            0x009E => xEX9E(chip8, opcode),
            0x00A1 => xEXA1(chip8, opcode),
            _ => println!("error, opcode 0x{:x} isn't implemented or does not exist", opcode),
        },
        0xF000 => match opcode & 0x00FF {
            0x0007 => xFX07(chip8, opcode),
            0x000A => xFX0A(chip8, opcode),
            0x0015 => xFX15(chip8, opcode),
            0x0018 => xFX18(chip8, opcode),
            0x001E => xFX1E(chip8, opcode),
            0x0029 => xFX29(chip8, opcode),
            0x0033 => xFX33(chip8, opcode),
            0x0055 => xFX55(chip8, opcode),
            0x0065 => xFX65(chip8, opcode),
            _ => println!("error, opcode 0x{:x} isn't implemented or does not exist", opcode),
        },
        _ => println!("error, opcode 0x{:x} isn't implemented or does not exist", opcode)
    }

    chip8.ku = false;
    
}
fn x00E0(chip8: &mut Chip8) {
    // Clears the display
    chip8.clear_display();
    chip8.next();
}

fn x00EE(chip8: &mut Chip8, opcode: u16) {
    // Returns from a subroutine
   chip8.pc = chip8.stack[chip8.sp as usize];
   chip8.sp -= 1;
   chip8.next();
   
}

fn x1NNN(chip8: &mut Chip8, opcode: u16) {
    // Jumps to address NNN
    let nnn = opcode & 0x0FFF;
    chip8.jump(nnn);
}

fn x2NNN(chip8: &mut Chip8, opcode: u16) {
    // Calls subroutine at NNN
    let nnn = opcode & 0x0FFF;
    chip8.sp += 1;
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.jump(nnn);
    
}

fn x3XNN(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if VX equals NN
    let x = (opcode & 0x0F00) >> 8;
    let nn = opcode & 0x00FF;
    if (chip8.v[x as usize] == nn as u8) {
        chip8.next();
    }
    chip8.next();
    
}

fn x4XNN(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if VX doesn't equal NN
    let x = (opcode & 0x0F00) >> 8;
    let nn = opcode & 0x00FF;
    if (chip8.v[x as usize] != nn as u8) {
        chip8.next();
    }
    chip8.next();
}

fn x5XY0(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if VX equals VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    if (chip8.v[x as usize] == chip8.v[y as usize]) {
        chip8.next();
    }
    chip8.next();
}

fn x6XNN(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to NN
    let x = (opcode & 0x0F00) >> 8;
    let nn = opcode & 0x00FF;

    chip8.v[x as usize] = nn as u8;
    chip8.next();
}

fn x7XNN(chip8: &mut Chip8, opcode: u16) {
    // Adds NN to VX
    let x = (opcode & 0x0F00) >> 8;
    let nn = opcode & 0x00FF;

    chip8.v[x as usize] += nn as u8;
    chip8.next();
}

fn x8XY0(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to the value of VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    chip8.v[x as usize] = chip8.v[y as usize];
    chip8.next();
}

fn x8XY1(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to VX OR VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    chip8.v[x as usize] |= chip8.v[y as usize];
    chip8.v[0xf] = 0;
    chip8.next();
}

fn x8XY2(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to VX AND VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    chip8.v[x as usize] &= chip8.v[y as usize];
    chip8.v[0xf] = 0;
    chip8.next();
}

fn x8XY3(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to VX XOR VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    chip8.v[x as usize] ^= chip8.v[y as usize];
    chip8.v[0xf] = 0;
    chip8.next();
}

fn x8XY4(chip8: &mut Chip8, opcode: u16) {
    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    let (sum, carry) = chip8.v[x as usize].overflowing_add(chip8.v[y as usize]);
    chip8.v[0xf] = if carry { 1 } else { 0 };
    chip8.v[x as usize] = sum;

    chip8.next();
}

fn x8XY5(chip8: &mut Chip8, opcode: u16) {
    // Vx is subtracted from Vy. VF is set to 1 when there's a borrow, and 0 when there isn't
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    let sub: usize = (chip8.v[x as usize]-chip8.v[y as usize]) as usize;

    chip8.v[0xf] = 0;
    if(chip8.v[y as usize] < chip8.v[x as usize]){
        chip8.v[0xf] = 1;
    }
    chip8.v[x as usize] = sub as u8;
    chip8.next();
}

fn x8XY6(chip8: &mut Chip8, opcode: u16) {
    // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;

    chip8.v[x as usize] = chip8.v[y as usize];

    let shiftedBit = chip8.v[x as usize] & 0x01;
    chip8.v[x as usize] = (chip8.getV(x) >> 1) as u8;
    chip8.v[0xf] = shiftedBit;
    chip8.next();
}

fn x8XY7(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    chip8.v[x as usize] = chip8.v[y as usize] - chip8.v[x as usize];

    chip8.v[0xf] = 0;
    if(chip8.getV(x) < chip8.getV(y)){
        chip8.v[0xf] = 1;
    }

    
    chip8.next();
}

fn x8XYE(chip8: &mut Chip8, opcode: u16) {
    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
        // Stores the least significant bit of VX in VF and then shifts VX to the right by 1
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
    
        chip8.v[x as usize] = chip8.v[y as usize];
        let shiftedBit = chip8.v[x as usize] >> 7;
        chip8.v[x as usize] = (chip8.getV(x) << 1) as u8;
        chip8.v[0xf] = shiftedBit;
        chip8.next();
}
fn x9XY0(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if VX doesn't equal VY
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    if(chip8.getV(x) != chip8.getV(y)){
        chip8.next();
    }
    chip8.next();
}

fn xANNN(chip8: &mut Chip8, opcode: u16) {
    // Sets I to the address NNN
    let nnn = opcode & 0x0FFF;
    chip8.i = nnn;
    chip8.next();
}

fn xBNNN(chip8: &mut Chip8, opcode: u16) {
    // Jumps to the address NNN plus V0
    let nnn = opcode & 0x0FFF;
    chip8.jump(nnn + chip8.v[0] as u16)
    
}

fn xCXNN(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to a random number AND NN
    let x = (opcode & 0x0F00) >> 8;
    let nn = opcode & 0x00FF;
    chip8.setV(x, (random::<u8>() & nn as u8) as u16);
    chip8.next();
}

fn xDXYN(chip8: &mut Chip8, opcode: u16) {
    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. 
    // Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change 
    // after the execution of this instruction. VF is set to 1 if any screen pixels are flipped from set 
    // to unset when the sprite is drawn, and to 0 if that doesn’t happen
    let x = chip8.v[((opcode & 0x0F00) >> 8) as usize] as usize;
    let y = chip8.v[((opcode & 0x00F0) >> 4) as usize] as usize;

    let height = (opcode & 0x000F) as usize;

    chip8.v[0xF] = 0;

    for yline in 0..height {

        let pixel = chip8.get(chip8.i + yline as u16);

        for xline in 0..8 {
            if (pixel & (0b1000_0000 >> xline)) != 0 {

                let xi = (x + xline) % 64;
                let yi = (y + yline) % 32;

                if chip8.get_pixel(xi, yi) {
                    chip8.v[0xF] = 1;
                }

                chip8.display[xi][yi] ^= true;
            }
        }
    }
    chip8.next();
}

fn xEX9E(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if the key stored in VX is pressed
    let x = (opcode & 0x0F00) >> 8;
    let vx = chip8.getV(x);
    if(chip8.key[vx as usize]){
        chip8.next();
    }
    chip8.next();
}

fn xEXA1(chip8: &mut Chip8, opcode: u16) {
    // Skips the next instruction if the key stored in VX isn't pressed
    let x = (opcode & 0x0F00) >> 8;

    let vx = chip8.v[x as usize];
    if(!chip8.key[vx as usize]){
        chip8.next();
    }
    chip8.next();

}

fn xFX07(chip8: &mut Chip8, opcode: u16) {
    // Sets VX to the value of the delay timer
    let x = (opcode & 0x0F00) >> 8;
    chip8.v[x as usize] =  chip8.dt; 
    chip8.next();
}

fn xFX0A(chip8: &mut Chip8, opcode: u16) {
    // A key press is awaited, and then stored in VX
    let x = (opcode & 0x0F00) >> 8;

    if(chip8.ku){
        chip8.v[x as usize] = chip8.kp & 0xf;
        chip8.next();
    }else {
        chip8.pc -= 2;
    }
    chip8.next();
}

fn xFX15(chip8: &mut Chip8, opcode: u16) {
    // Sets the delay timer to VX
    let x = (opcode & 0x0F00) >> 8;
    chip8.dt = chip8.v[x as usize];
    chip8.next();
}

fn xFX18(chip8: &mut Chip8, opcode: u16) {
    // Sets the sound timer to VX
    let x = (opcode & 0x0F00) >> 8;
    chip8.st = chip8.v[x as usize];
    chip8.next();
}

fn xFX1E(chip8: &mut Chip8, opcode: u16) {
    // Adds VX to I. VF is not affected
    let x = (opcode & 0x0F00) >> 8;
    chip8.i += chip8.v[x as usize] as u16;
    chip8.next();
}

fn xFX29(chip8: &mut Chip8, opcode: u16) {
    // Sets I to the location of the sprite for the character in VX. 
    // Characters 0-F (in hexadecimal) are represented by a 4x5 font
    let x = (opcode & 0x0F00) >> 8;
    chip8.i =  (chip8.v[x as usize] * 5) as u16;
    chip8.next();
}

fn xFX33(chip8: &mut Chip8, opcode: u16) {
    // Stores the binary-coded decimal representation of VX, with the most significant of three 
    // digits at the address in I, the middle digit at I plus 1, and the least significant digit 
    // at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit 
    // in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
    let x = (opcode & 0x0F00) >> 8;

    let vx = chip8.getV(x) as f32; 

    let hundreds = (vx / 100.0).floor() as u8;
    let tens = ((vx / 10.0) % 10.0).floor() as u8;
    let ones = (vx % 10.0) as u8;

    chip8.write(chip8.i as usize, hundreds);
    chip8.write((chip8.i + 1) as usize, tens);
    chip8.write((chip8.i + 2) as usize, ones);
    chip8.next();
}

fn xFX55(chip8: &mut Chip8, opcode: u16) {
    // Stores V0 to VX (including VX) in memory starting at address I. I is then set to I + X + 1
    let x = (opcode & 0x0F00) >> 8;

    for i in 0..=x {
        chip8.write((chip8.i + i) as usize, chip8.v[x as usize])
    }
    chip8.i += 1;
    chip8.next();
}

fn xFX65(chip8: &mut Chip8, opcode: u16) {
    // Fills V0 to VX (including VX) with values from memory starting at address I. I is then set to I + X + 1
    let x = (opcode & 0x0F00) >> 8;
  
    for i in 0..=x {
        chip8.v[i as usize] = chip8.get(chip8.i + i);        
    }
    chip8.i += 1;
    chip8.next();
}




