use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};

struct DecomposedMessage {
    position_x: i32,
    position_y: i32,
    press: bool,
    release: bool,
    right: bool
}

fn transform_i32_to_array_of_u8(number: i32) -> [u8; 4] {
    //si el byte mes significatiu es major que 128 significa que el nombre es negatiu
    println!("number: {}", number);
    let is_negative = number < 0;
    println!("negative {}", is_negative);
    let positive_number = number * (-1_i32).pow(is_negative as u32);
    println!("{}", positive_number);
    let mut result_array = [0 as u8; 4];
    //suposant que el byte en el lloc 0 de l'array es el mes significatiu i el que porta el bit negatiu
    
    for i in 0..3 {
        print!("byte {}:", 3 - i);
        result_array[3 - i] = ((positive_number >> 8 * i) & 0xff) as u8;
        println!("{}", result_array[3 - i]);
    }
    print!("byte 0:");
    let negative_bit = if is_negative {128 as u8} else {0 as u8};
    result_array[0] = ((positive_number >> 8 * 3) & 0xff) as u8 + negative_bit;
    println!("{}", result_array[0]);
    
    println!("");

    result_array
}

fn compose_message(decomposed_message: DecomposedMessage) -> [u8; 9] {
    let x_array = transform_i32_to_array_of_u8(decomposed_message.position_x);
    let y_array = transform_i32_to_array_of_u8(decomposed_message.position_y);
    let mut message_array = [0 as u8; 9];

    for i in 0..4 {
        message_array[i] = x_array[i];
        message_array[i + 4] = y_array[i];
        
    }

    message_array[8] = ((decomposed_message.press as u8) * 4) | ((decomposed_message.release as u8) * 2) | (decomposed_message.right as u8);

    println!("9th bit: {}", message_array[8]);
    return message_array;
}


fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut x = -1;
            let mut y = -1;

            /*loop{
                let ten_millis = time::Duration::from_millis(200);
                thread::sleep(ten_millis);
                if x < 0 && x > -11 {
                    x -=1;
                }



                let decomposed_message = DecomposedMessage{
                    position_x: x,
                    position_y: y,
                    press: false,
                    release: false,
                    right: false,
                };
                let msg = compose_message(decomposed_message);
                
                stream.write(&msg).unwrap();
                println!("Sent Hello, awaiting reply...");
            }*/
            let decomposed_message = DecomposedMessage{
                position_x: 40,
                position_y: 0,
                press: true,
                release: false,
                right: false,
            };
            let msg = compose_message(decomposed_message);
            stream.write(&msg).unwrap();
            thread::sleep(time::Duration::from_millis(400));

            let decomposed_message = DecomposedMessage{
                position_x: 40,
                position_y: 0,
                press: true,
                release: false,
                right: false,
            };
            let msg = compose_message(decomposed_message);
            stream.write(&msg).unwrap();
            thread::sleep(time::Duration::from_millis(400));


            let decomposed_message = DecomposedMessage{
                position_x: 0,
                position_y: 40,
                press: false,
                release: false,
                right: false,
            };
            let msg = compose_message(decomposed_message);
            stream.write(&msg).unwrap();
            thread::sleep(time::Duration::from_millis(400));
            let decomposed_message = DecomposedMessage{
                position_x: -40,
                position_y: 0,
                press: false,
                release: false,
                right: false,
            };
            let msg = compose_message(decomposed_message);
            stream.write(&msg).unwrap();
            thread::sleep(time::Duration::from_millis(400));
            let decomposed_message = DecomposedMessage{
                position_x: 0,
                position_y: -40,
                press: false,
                release: true,
                right: false,
            };
            let msg = compose_message(decomposed_message);
            stream.write(&msg).unwrap();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}