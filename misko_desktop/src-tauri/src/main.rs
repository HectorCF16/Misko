#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use server::Server;
use local_ip_address::local_ip;

#[tauri::command]
fn run_server(password: &str) -> String{
    match FromStr::from_str(password) { 
        Ok(password_numbers) => {
            let server = Server::new("3333", password_numbers);
            server.listen();
            "Fine".to_owned()
        },
        Err(_) => "Password has to be numeric".to_owned()
    }
}

#[tauri::command]
fn get_ip() -> String {
    match local_ip() {
        Ok(local_ip_address) => format!("Your ip {:?}:{}", local_ip_address,":3333"),
        Err(_) => "".to_owned(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_server, get_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


mod server {
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    const MESSAGE_NUMBER_OF_BYTES: usize = 9;
    const BITS_32_NUMBER_OF_BYTES: usize = 4;
    const CLICK_BYTE_POSITION: usize = 8;

    pub struct Server {
        listener: TcpListener,
        password: i32
    }

    impl Server {
        pub fn new(port: &str, password: i32) -> Self {
            let address = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(address).unwrap();
            Server { listener, password }
        }

        pub fn listen(&self) {
            for stream in self.listener.incoming() {
                connect_to_client(stream, self.password);
            }
        }
    }

    fn connect_to_client(result: Result<TcpStream, std::io::Error>, password: i32) {
        match result {
            Ok(stream) => {
                successful_tcp_connection(stream, password);
            }
            Err(e) => {
                connection_failed(e);
            }
        }
    }

    fn successful_tcp_connection(stream: TcpStream, password: i32) {
        println!("New connection: {}", stream.peer_addr().unwrap());

        thread::spawn(move || handle_client_connection(stream, password));
    }

    fn handle_client_connection(stream: TcpStream, password: i32) {
        let mut client_handler = client::ClientHandler::new(stream, password);
        client_handler.handle_client_messages();
    }

    fn connection_failed(e: std::io::Error) {
        println!("Error: {}", e);
    }

    mod client {
        use std::{
            io::Read,
            net::{TcpStream},
        };

        use crate::server::{
            four_bytes_into_i32::transform_array_of_u8_to_i32, message::InputByteArray, mouse_input,
        };

        const MESSAGE_NUMBER_OF_BYTES: usize = 9;

        pub struct ClientHandler {
            stream: TcpStream,
            correct_password: i32,
            has_password_entered: bool,
        }
        impl ClientHandler {
            pub fn new(stream: TcpStream, correct_password: i32) -> Self {
                ClientHandler {
                    stream,
                    correct_password,
                    has_password_entered: false,
                }
            }

            pub fn handle_client_messages(&mut self) {
                let mut message: [u8; MESSAGE_NUMBER_OF_BYTES];
                while {
                    message = [0 as u8; MESSAGE_NUMBER_OF_BYTES];

                    self.stream.read(&mut message);

                    self.message_handle(Ok(message))
                } {}
            }

            fn message_handle(
                &mut self,
                read_message: Result<[u8; MESSAGE_NUMBER_OF_BYTES], std::io::Error>,
            ) -> bool {
                match read_message {
                    Ok(message) => {
                        self.report_when_message_is_not_empty(message);
                        true
                    }
                    Err(_) => false,
                }
            }

            fn report_when_message_is_not_empty(&mut self, message: [u8; MESSAGE_NUMBER_OF_BYTES]) {
                let message_byte_array = InputByteArray::new(message);
                if message_byte_array.is_empty() {
                    return;
                }
                self.check_password(message_byte_array);
            }

            fn message_recieved(&mut self, message: InputByteArray) {
                execute_inputs(message);
            }

            fn check_password(&mut self, message: InputByteArray) {
                if !self.has_password_entered {
                    self.has_password_entered = self.validate_password(message);
                    return;
                }
                self.message_recieved(message);
            }

            fn validate_password(&self, input_byte_array: InputByteArray) -> bool {
                let password_array = input_byte_array.get_password();
                let entered_password = transform_array_of_u8_to_i32(password_array);
                self.compare_password(entered_password)
            }

            pub fn compare_password(&self, entered_password: i32) -> bool {
                entered_password == self.correct_password
            }
        }

        fn execute_inputs(input_byte_array: InputByteArray) {
            for i in input_byte_array.message {
                print!("{} ", i);
            }

            println!();
            let mouse_inputs = mouse_input::MouseInputs::new(input_byte_array);
            
            mouse_inputs.execute_mouse_inputs();
        }
    }

    mod mouse_input {
        use core::fmt;

        use crate::server::{
            four_bytes_into_i32::transform_array_of_u8_to_i32, message::InputByteArray,
            mouse_input::point::Point,
        };
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
                println!("{}", self);
                if self.clicks.is_up() {
                    mouse_up(self.clicks.get_button());
                }
            }
        }

        impl fmt::Display for MouseInputs {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}, {}  up: {}, down: {}, but: {:#?}", self.position.x, self.position.y, self.clicks.is_up(), self.clicks.is_down(), self.clicks.get_button())
            }
        }

        fn get_position_from_byte_array(mouse_inputs_byte_array: &InputByteArray) -> Point {
            let x =
                transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_x_position_bytes());
            let y =
                transform_array_of_u8_to_i32(mouse_inputs_byte_array.get_mouse_y_position_bytes());
            point::Point::new(x, y)
        }

        fn move_relative(point: &Point) {
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
                    let down = array_of_bits[7];
                    let up = array_of_bits[6];
                    let right = array_of_bits[5];
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
                }
                result
            }
        }
    }

    mod message {
        use crate::server::{BITS_32_NUMBER_OF_BYTES, MESSAGE_NUMBER_OF_BYTES};

        use super::CLICK_BYTE_POSITION;

        pub struct InputByteArray {
            pub message: [u8; MESSAGE_NUMBER_OF_BYTES],
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


                y_position_bytes
            }

            pub fn get_mouse_clicks(&self) -> u8 {
                self.message[CLICK_BYTE_POSITION]
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

            for i in 1..BITS_32_NUMBER_OF_BYTES {
                let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
                number += addition;
            }

            number = number * (-1 as i32).pow(is_negative as u32);
            return number;
        }
    }
}
