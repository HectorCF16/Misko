use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use enigo::*;

const MESSAGE_NUMBER_OF_BYTES: usize = 50;
const BYTE_NUMBER_OF_BYTES: usize = 4;


struct Password {
    enteredPassword: String,
}

struct MouseInputs {
    position_x: i32,
    position_y: i32,
    left_click: bool,
    release_left_click: bool,
    right_click: bool,
    release_right_click: bool,
}

impl MouseInputs{
    pub fn new(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> MouseInputs {
        let mut x_array = [0 as u8; BYTE_NUMBER_OF_BYTES];
        let mut y_array = [0 as u8; BYTE_NUMBER_OF_BYTES];

        for i in 0..BYTE_NUMBER_OF_BYTES {
            x_array[i] = data[i];
            y_array[i] = data[i + BYTE_NUMBER_OF_BYTES];
        }

        let x = transform_array_of_u8_to_i32(x_array);
        let y = transform_array_of_u8_to_i32(y_array);
        
        MouseInputs{
            position_x: x,
            position_y: y,
            left_click: false,
            release_left_click: false,
            right_click: false,
            release_right_click: false,
        }
    }
}

fn execute_mouse_inputs(decomposedMessage: MouseInputs){
    if decomposedMessage.position_x != 0 || decomposedMessage.position_y != 0 {
        move_relative(decomposedMessage.position_x, decomposedMessage.position_y);
    }
    
}

fn move_relative(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
}

fn transform_array_of_u8_to_i32(array: [u8; BYTE_NUMBER_OF_BYTES] ) -> i32 {
    let mut number: i32 = 0;
    //first element is the most valuable (significative)
    let first_element = array[0];
    let is_negative = first_element >= 128;
    let mut first_element_without_negative_bit = first_element;

    if is_negative {
        first_element_without_negative_bit = first_element - 128;
    }

    let addition: i32 = i32::from(first_element_without_negative_bit) * 256_i32.pow(3);
    number = addition;
    println!("byte 0: {}", first_element_without_negative_bit);

    for i in 1..BYTE_NUMBER_OF_BYTES{
        let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
        number += addition;
        println!("byte {}: {}", i , array[i]);
    }
    
    number = number * (-1 as i32).pow(is_negative as u32);
    return number;
}

fn data_to_password(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> i32 {
    let mut password_array = [0 as u8; BYTE_NUMBER_OF_BYTES];
    for i in 0..BYTE_NUMBER_OF_BYTES{
        password_array[i] = data[i];
    }

    transform_array_of_u8_to_i32(password_array)
} 

/*
fn copy_byte_array(source: [u8; ], destination: [u8; BYTE_NUMBER_OF_BYTES]){
    for i in 0..BYTE_NUMBER_OF_BYTES{
        password_array[i] = data[i];
    }
}
 */

fn is_zero(array: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool{
    for byte in array{
        if byte != 0 as u8 {
            return false;
        }
    }
    true
}

fn message_entered(has_password_entered: bool, correct_password: i32, message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool {
    if has_password_entered {
        if !is_zero(message) {
            
            let decomposed_message = MouseInputs::new(message);
            move_relative(decomposed_message.position_x, decomposed_message.position_y);
            println!("x: {}, y: {}", decomposed_message.position_x, decomposed_message.position_y);
        }
    }
    correct_password == data_to_password(message)
}

fn handle_client(mut stream: TcpStream) {
    let mut message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];
    let correct_password = 205990267;
    let mut has_password_entered = false;
    while match stream.read(&mut message) {
        Ok(size) => {
            has_password_entered = message_entered(has_password_entered, correct_password, message);
            message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];
        true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}