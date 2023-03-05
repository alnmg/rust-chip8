#![allow(arithmetic_overflow)]
mod CHIP8;

use CHIP8::Chip8;
use minifb::{Key, Window, WindowOptions};
use std::fs::File;

use rfd::FileDialog;

fn main(){
    println!("hello");
    init();
}

//copied from source example

const SCALE: usize = 10;
const WIDTH: usize = 64 * SCALE;
const HEIGHT: usize = 32 * SCALE;

const HZ: usize = 60;

pub fn init() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "chip 8 emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros((1000000/HZ).try_into().unwrap())));

    
    let mut Chip8 = CHIP8::start();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if(window.is_key_down(Key::O)){
        
            let file = FileDialog::new()
            .add_filter("chip8", &["ch8"])
            .set_directory("/")
            .pick_file();
        
    
            let file = File::open(file.unwrap().as_path()).expect("Failed to open file");

            CHIP8::load(file, &mut Chip8);
        }

        //input
        if(window.is_key_down(Key::Key1)){
            Chip8.key[0x1] = true;
        }else if (window.is_key_released(Key::Key1)) {
            Chip8.kp = 0x1;
            Chip8.ku = true;
            Chip8.key[0x1] = false;
        }
        if(window.is_key_down(Key::Key2)){
            Chip8.key[0x2] = true;
        }else if (window.is_key_released(Key::Key2)) {
            Chip8.kp = 0x2;
            Chip8.ku = true;
            Chip8.key[0x2] = false;
        }
        if(window.is_key_down(Key::Key3)){
            Chip8.key[0x3] = true;
        }else if (window.is_key_released(Key::Key3)) {
            Chip8.kp = 0x3;
            Chip8.ku = true;
            Chip8.key[0x3] = false;
        }
        if(window.is_key_down(Key::Key4)){
            Chip8.key[0xc] = true;
        }else if (window.is_key_released(Key::Key4)) {
            Chip8.kp = 0xC;
            Chip8.ku = true;
            Chip8.key[0xc] = false;
        }

        if(window.is_key_down(Key::Q)){
            Chip8.key[0x4] = true;
        }else if (window.is_key_released(Key::Q)) {
            Chip8.kp = 0x4;
            Chip8.ku = true;
            Chip8.key[0x4] = false;
        }
        if(window.is_key_down(Key::E)){
            Chip8.key[0x5] = true;
        }else if (window.is_key_released(Key::E)) {
            Chip8.kp = 0x5;
            Chip8.ku = true;
            Chip8.key[0x5] = false;
        }
        if(window.is_key_down(Key::R)){ 
            Chip8.key[0x6] = true;
        }else if (window.is_key_released(Key::R)) {
            Chip8.kp = 0x6;
            Chip8.ku = true;
            Chip8.key[0x6] = false;
        }
        if(window.is_key_down(Key::E)){
            Chip8.key[0xD] = true;
        }else if (window.is_key_released(Key::E)) {
            Chip8.kp = 0xD;
            Chip8.ku = true;
            Chip8.key[0xD] = false;
        }
        
        if(window.is_key_down(Key::A)){
            Chip8.key[0x7] = true;
        }else if (window.is_key_released(Key::A)) {
            Chip8.kp = 0x7;
            Chip8.ku = true;
            Chip8.key[0x7] = false;
        }
        if(window.is_key_down(Key::S)){
            Chip8.key[0x8] = true;
        }else if (window.is_key_released(Key::S)) {
            Chip8.kp = 0x8;
            Chip8.ku = true;
            Chip8.key[0x8] = false;
        }
        if(window.is_key_down(Key::D)){
            Chip8.key[0x9] = true;
        }else if (window.is_key_released(Key::D)) {
            Chip8.kp = 0x9;
            Chip8.ku = true;
            Chip8.key[0x9] = false;
        }
        if(window.is_key_down(Key::F)){
            Chip8.key[0xE] = true;
        }else if (window.is_key_released(Key::F)) {
            Chip8.kp = 0xE;
            Chip8.ku = true;
            Chip8.key[0xE] = false;
        }

        if(window.is_key_down(Key::Z)){
            Chip8.key[0xA] = true;
        }else if (window.is_key_released(Key::Z)) {
            Chip8.kp = 0xA;
            Chip8.ku = true;
            Chip8.key[0xA] = false;
        }
        if(window.is_key_down(Key::X)){
            Chip8.key[0xB] = true;
        }else if (window.is_key_released(Key::X)) {
            Chip8.kp = 0xB;
            Chip8.ku = true;
            Chip8.key[0xB] = false;
        }
        if(window.is_key_down(Key::C)){
            Chip8.key[0x0] = true;
        }else if (window.is_key_released(Key::C)) {
            Chip8.kp = 0x0;
            Chip8.ku = true;
            Chip8.key[0x0] = false;
        }
        if(window.is_key_down(Key::V)){
            Chip8.key[0xF] = true;
        }else if (window.is_key_released(Key::V)) {
            Chip8.kp = 0xf;
            Chip8.ku = true;
            Chip8.key[0xF] = false;
        }

        //cicle
        CHIP8::cycle(&mut Chip8, 8);

        //draw screen
        for y in 0..32 {
            for x in 0..64 {
                let pixel = Chip8.display[x][y];

                // Desenha o pixel escalado na tela
                for sy in 0..SCALE {
                    for sx in 0..SCALE {
                        let buffer_x = x * SCALE + sx;
                        let buffer_y = y * SCALE + sy;

                        let index = buffer_y * WIDTH + buffer_x;
                        if pixel {
                            buffer[index] = 0xFFFFFFFF; // Branco
                        } else {
                            buffer[index] = 0xFF000000; // Preto
                        }
                    }
                }
            }
        }

        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

