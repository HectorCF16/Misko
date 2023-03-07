use four_bytes_into_i32::transform_array_of_u8_to_i32;
use message::InputByteArray;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

const MESSAGE_NUMBER_OF_BYTES: usize = 50;
const BITS_32_NUMBER_OF_BYTES: usize = 4;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        connect(stream);
    }
    // close the socket server
    drop(listener);
}

fn connect(result: Result<TcpStream, std::io::Error>) {
    match result {
        Ok(mut stream) => {
            successful_tcp_connection(stream);
        }
        Err(e) => {
            connection_failed(e);
        }
    }
}

fn successful_tcp_connection(stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    thread::spawn(move || handle_client_connection(stream));
}

fn connection_failed(e: std::io::Error) {
    println!("Error: {}", e);
}

fn handle_client_connection(mut stream: TcpStream) {
    let mut message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];
    let correct_password = 205990267;
    let mut client_handler = client::ClientHandler::new(stream, correct_password);
    client_handler.handle_client_messages();
}


fn check_password(correct_password: i32, message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> bool {
    correct_password == data_to_password(message)
}

fn data_to_password(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> i32 {
    let mut password_array = [0 as u8; BITS_32_NUMBER_OF_BYTES];
    for i in 0..BITS_32_NUMBER_OF_BYTES {
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


mod client {
    use std::{net::{Shutdown, TcpListener, TcpStream}, io::Read, error::Error};

    use crate::{message::InputByteArray, four_bytes_into_i32::transform_array_of_u8_to_i32, mouse_input};

    const MESSAGE_NUMBER_OF_BYTES: usize = 50;

    const BITS_32_NUMBER_OF_BYTES: usize = 4;
 

    pub struct ClientHandler {
        stream: TcpStream,
        correct_password: i32,
        has_password_entered: bool,
    }
    impl ClientHandler {
        pub fn new(stream: TcpStream, correct_password: i32) -> Self {
            ClientHandler { stream, correct_password, has_password_entered: false}
        } 

        pub fn compare_password(&mut self, entered_password: i32) -> bool{
            self.has_password_entered = entered_password == self.correct_password;
            self.has_password_entered
        }

        pub fn handle_client_messages (&mut self){
            let mut message: [u8; MESSAGE_NUMBER_OF_BYTES];
            while {
                message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];

                self.stream.read(&mut message);

                self.message_handle(Ok(message))
            } {}
        }

        fn message_handle(
            &mut self,
            read_message: Result<[u8; MESSAGE_NUMBER_OF_BYTES], std::io::Error>
        ) -> bool {
            match read_message {
                Ok(message) => {
                    self.message_recieved(message);                    
                    true
                }
                Err(_) => {               
                    false
                }
            }
        }

        fn message_recieved(&mut self, message: [u8; MESSAGE_NUMBER_OF_BYTES]){
            let input_byte_array = InputByteArray::new(message);
            if !self.has_password_entered {
                println!("hola1");
                self.has_password_entered = self.check_password(input_byte_array);
            } else {
                println!("hola2");
                execute_inputs(input_byte_array);
            }
        }

        fn check_password(&mut self, input_byte_array: InputByteArray) -> bool {
            let password_array = input_byte_array.get_password();
            let entered_password = transform_array_of_u8_to_i32(password_array);
            self.compare_password(entered_password)
        }
        
    }

    fn execute_inputs(input_byte_array: InputByteArray){
        if !input_byte_array.is_empty(){
            let mouse_inputs = mouse_input::MouseInputs::new(input_byte_array);
            mouse_inputs.execute_mouse_inputs();
        }
    }

    
}



mod mouse_input {
    use crate::{four_bytes_into_i32::transform_array_of_u8_to_i32, message::InputByteArray};
    use enigo::*;

    pub struct MouseInputs {
        position: point::Point,
        clicks: clicks::Clicks,
    }

    impl MouseInputs {
        pub fn new(mouse_inputs_byte_array: InputByteArray) -> Self {
            let position = get_position_from_byte_array(&mouse_inputs_byte_array);
            let clicks = clicks::Clicks::new(mouse_inputs_byte_array.get_mouse_clicks());
            Self { position, clicks }
        }

        pub fn execute_mouse_inputs(&self) {
            if self.clicks.is_down() {
                mouse_down(self.clicks.get_button());
            }
            move_relative(&self.position);
            println!("x: {}, y: {}", self.position.x, self.position.y);
            if self.clicks.is_up() {
                mouse_up(self.clicks.get_button());
            }
        }
    }

    fn get_position_from_byte_array(
        mouse_inputs_byte_array: &InputByteArray,
    ) -> crate::mouse_input::point::Point {
        let x = transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_x_position_bytes());
        let y = transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_y_position_bytes());
        point::Point::new(x, y)
    }

    fn move_relative(point: &crate::mouse_input::point::Point) {
        let mut enigo = Enigo::new();
        enigo.mouse_move_relative(point.x, point.y);
    }

    fn mouse_down(button: MouseButton) {
        let mut enigo = Enigo::new();
        enigo.mouse_down(button);
    }

    fn mouse_up(button: MouseButton) {
        let mut enigo = Enigo::new();
        enigo.mouse_up(button);
    }

    mod point {
        pub struct Point {
            pub x: i32,
            pub y: i32,
        }

        impl Point {
            pub fn new(x: i32, y: i32) -> Self {
                Point { x, y }
            }
        }
    }

    mod clicks {

        use enigo::MouseButton;

        pub struct Clicks {
            down: bool,
            up: bool,
            button: MouseButton,
        }

        impl Clicks {
            pub fn new(byte: u8) -> Self {
                let array_of_bits = u8_to_bool_array(byte);
                let down = array_of_bits[0];
                let up = array_of_bits[1];
                let right = array_of_bits[2];
                let button: MouseButton;
                if !right {
                    button = MouseButton::Left;
                } else {
                    button = MouseButton::Right;
                }

                Clicks { down, up, button }
            }

            pub fn is_down(&self) -> bool {
                self.down
            }

            pub fn is_up(&self) -> bool {
                self.up
            }

            pub fn get_button(&self) -> MouseButton {
                self.button
            }
        }
        //byte management function
        fn u8_to_bool_array(byte: u8) -> [bool; 8] {
            let mut result = [false; 8];
            let mut new_byte = byte;
            for i in (0..8).rev() {
                let bit_value = 2_u8.pow(i as u32);
                result[i] = new_byte >= bit_value;
                new_byte = new_byte - bit_value * result[i] as u8;
                print!("{}", bit_value);
            }
            println!("");
            result
        }
    }
}

mod message {
    use crate::{BITS_32_NUMBER_OF_BYTES, MESSAGE_NUMBER_OF_BYTES};

    pub struct InputByteArray {
        message: [u8; MESSAGE_NUMBER_OF_BYTES],
    }

    impl InputByteArray {
        pub fn new(message: [u8; MESSAGE_NUMBER_OF_BYTES]) -> Self {
            InputByteArray { message }
        }

        pub fn get_mouse_x_position_bytes(&self) -> [u8; BITS_32_NUMBER_OF_BYTES] {
            let mut x_position_bytes = [0 as u8; BITS_32_NUMBER_OF_BYTES];

            for i in 0..4 {
                x_position_bytes[i] = self.message[i];
            }

            x_position_bytes
        }

        pub fn get_mouse_y_position_bytes(&self) -> [u8; BITS_32_NUMBER_OF_BYTES] {
            let mut y_position_bytes = [0 as u8; BITS_32_NUMBER_OF_BYTES];

            for i in 0..4 {
                y_position_bytes[i] = self.message[i + 4];
            }

            println!("{:#?}", y_position_bytes);

            y_position_bytes
        }

        pub fn get_mouse_clicks(&self) -> u8 {
            self.message[BITS_32_NUMBER_OF_BYTES]
        }

        pub fn get_password(&self) -> [u8; BITS_32_NUMBER_OF_BYTES] {
            let mut password_bytes = [0 as u8; BITS_32_NUMBER_OF_BYTES];
            for i in 0..4 {
                password_bytes[i] = self.message[i];
            }
            password_bytes
        }

        pub fn is_empty(&self) -> bool {
            for byte in self.message {
                if byte != 0 as u8 {
                    return false;
                }
            }
            true
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
