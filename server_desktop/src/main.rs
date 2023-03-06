use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use message::MouseInputByteArray;
use four_bytes_into_i32::transform_array_of_u8_to_i32;

const MESSAGE_NUMBER_OF_BYTES: usize = 50;
const BITS_32_NUMBER_OF_BYTES: usize = 4;

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
            //println!("{:#?}", message);
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
        return true;
    }
    check_password(correct_password, message)
}

fn execute_input_message(message: [u8; MESSAGE_NUMBER_OF_BYTES]) {
    if !is_zero(message) {
        //there should be a state flag for knowing what kind of input is going to be recieved
        let mouse_input_byte_array = MouseInputByteArray::new(message);
        let mouse_inputs = mouse_input::MouseInputs::new(mouse_input_byte_array);
        mouse_inputs.execute_mouse_inputs();
    }
}


fn check_password(correct_password: i32, message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool {
    correct_password == data_to_password(message)
}

fn data_to_password(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> i32 {
    let mut password_array = [0 as u8; BITS_32_NUMBER_OF_BYTES];
    for i in 0..BITS_32_NUMBER_OF_BYTES{
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



mod mouse_input {
    use enigo::*;
    use crate::{four_bytes_into_i32::transform_array_of_u8_to_i32, message::MouseInputByteArray};
    

    pub struct MouseInputs {
        position: point::Point,
        left_click: bool,
        release_left_click: bool,
        right_click: bool,
        release_right_click: bool,
    }

    impl MouseInputs {
        pub fn new(mouse_inputs_byte_array: MouseInputByteArray) -> Self {
            let position = get_position_from_byte_array(mouse_inputs_byte_array);

            Self {
                position,
                left_click: false,
                release_left_click: false,
                right_click: false,
                release_right_click: false,
            }
            
        }

        pub fn execute_mouse_inputs(&self){
            
            move_relative(&self.position);
            println!("x: {}, y: {}", self.position.x, self.position.y);
        }

    }

    fn get_position_from_byte_array(mouse_inputs_byte_array: MouseInputByteArray) -> crate::mouse_input::point::Point {
        let x = transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_x_position_bytes());
        let y = transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_y_position_bytes());
        point::Point::new(x, y)
    }
    
    fn move_relative(point: &crate::mouse_input::point::Point) {
        let mut enigo = Enigo::new();
        enigo.mouse_move_relative(point.x, point.y);
    }

    mod point {
        pub struct Point{
            pub x: i32,
            pub y: i32
        }

        impl Point {
            pub fn new(x: i32, y: i32) -> Self {
                Point { x, y }
            }
        }
    }
}

mod message {
    use crate::{MESSAGE_NUMBER_OF_BYTES, BITS_32_NUMBER_OF_BYTES};

    pub struct MouseInputByteArray{
        message: [u8; MESSAGE_NUMBER_OF_BYTES]
    }

    impl MouseInputByteArray {
        pub fn new (message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> Self {
            MouseInputByteArray { message }
        }

        pub fn get_mouse_x_position_bytes(&self) -> [u8; BITS_32_NUMBER_OF_BYTES]{
            let mut x_position_bytes = [0 as u8; BITS_32_NUMBER_OF_BYTES];

            for i in 0..4 {
                x_position_bytes[i] = self.message[i];
            }

            x_position_bytes
        }

        pub fn get_mouse_y_position_bytes(&self) -> [u8; BITS_32_NUMBER_OF_BYTES]{
            let mut y_position_bytes = [0 as u8; BITS_32_NUMBER_OF_BYTES];
            
            for i in 0..4 {
                y_position_bytes[i] = self.message[i + 4];
            }

            println!("{:#?}", y_position_bytes);

            y_position_bytes
        }
    }
}

mod four_bytes_into_i32 {
    const BITS_32_NUMBER_OF_BYTES: usize = 4;

    pub fn transform_array_of_u8_to_i32(array: [u8; BITS_32_NUMBER_OF_BYTES]) -> i32 {
        let mut number: i32;
        //first element is the most valuable (significative)
        let first_element = array[0];
        let is_negative = first_element >= 128;
        let mut first_element_without_negative_bit = first_element;

        if is_negative {
            first_element_without_negative_bit = first_element - 128;
        }

        let addition: i32 = i32::from(first_element_without_negative_bit) * 256_i32.pow(3);
        number = addition;
        //println!("byte 0: {}", first_element_without_negative_bit);

        for i in 1..BITS_32_NUMBER_OF_BYTES {
            let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
            number += addition;
            //println!("byte {}: {}", i, array[i]);
        }

        number = number * (-1 as i32).pow(is_negative as u32);
        return number;
    }

    

}