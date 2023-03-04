pub mod mouse_inputs;

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use mouse_inputs::mouse_input::MouseInputs;

const MESSAGE_NUMBER_OF_BYTES: usize = 50;
const BYTE_NUMBER_OF_BYTES: usize = 4;

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

fn handle_client(mut stream: TcpStream) {
    let mut message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];
    let correct_password = 205990267;
    let mut has_password_entered = false;
    while match stream.read(&mut message) {
        Ok(_size) => {
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

fn message_entered(has_password_entered: bool, correct_password: i32, message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool {
    if has_password_entered {
        execute_input_message(message);
    }
    check_password(correct_password, message)
}

fn execute_input_message(message: [u8; MESSAGE_NUMBER_OF_BYTES]) {
    if !is_zero(message) {
        //there should be a state flag for knowing what kind of input is going to be recieved
        let mouse_inputs = MouseInputs::new(message);
        mouse_inputs.execute_mouse_inputs();
    }
}


fn check_password(correct_password: i32, message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool {
    correct_password == data_to_password(message)
}

fn data_to_password(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> i32 {
    let mut password_array = [0 as u8; BYTE_NUMBER_OF_BYTES];
    for i in 0..BYTE_NUMBER_OF_BYTES{
        password_array[i] = data[i];
    }

    transform_array_of_u8_to_i32(password_array)
} 


fn transform_array_of_u8_to_i32(array: [u8; BYTE_NUMBER_OF_BYTES]) -> i32 {
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

    for i in 1..BYTE_NUMBER_OF_BYTES {
        let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
        number += addition;
        println!("byte {}: {}", i, array[i]);
    }

    number = number * (-1 as i32).pow(is_negative as u32);
    return number;
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