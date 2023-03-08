use std::net::{TcpStream};
use std::io::Error;
use client::Client;
use mouse_input::MouseInputs;
use mouse_input::point::Point;
use mouse_input::clicks::{Clicks, MouseButton};
//mod client;
 
fn main() {
    //this two variables would be entered by the user
    let address = "localhost:3333";
    let password = 205990267;

    let client_wrapped = connection_request(address, password);

    match client_wrapped {
        Ok(client) => send_mouse_inputs(client),
        Err(e) => connection_error(e),
    }
}
 
fn connection_request(address: &str, password: i32) ->Result<Client, Error>{
    handle_connection(TcpStream::connect(address), password)
}
 

fn handle_connection(tcp_stream: Result<TcpStream, Error>, password: i32) ->  Result<Client, Error>{
    let client:  Result<Client, Error>;
    match tcp_stream {
        Ok(stream) => {
            client = Ok(login(stream, password));
        },
        Err(e) => {
            client = Err(e);
        }
    }
    println!("Terminated.");

    client
}

fn login(stream: TcpStream, password: i32) -> Client{
    println!("Successfully connected to server in port 3333");
    let mut client = Client::new(stream);

    client.send_password(password);
    client
}

 

fn send_mouse_inputs(mut client: Client){
    let position = Point::new(10, 10);
    let clicks = Clicks::new(false, false, MouseButton::Left);
    let mouse_input = MouseInputs::new(position, clicks);

    client.send_mouse_input(mouse_input);
}

 

mod client {
    use std::{net::TcpStream, io::Write};
    use crate::{transform_i32_to_array_of_u8, mouse_input::{MouseInputs}};

    pub struct Client {
        stream: TcpStream,
    }

    impl Client {
        pub fn new(stream: TcpStream) -> Self {
            Client { stream }
        }

        pub fn send_password(&mut self, password: i32){
            let password_array = transform_i32_to_array_of_u8(password);

            self.stream.write(&password_array);
        }

        pub fn send_mouse_input(&mut self, mouse_input: MouseInputs){
            let message_array = mouse_input.get_byte_array();

            self.stream.write(&message_array);
        }
    }    

}

 

mod mouse_input {
    use clicks::Clicks;
    use point::Point;

    pub struct MouseInputs {
        position: point::Point,
        clicks: clicks::Clicks,
    }

    impl MouseInputs {
        pub fn new(position: Point, clicks: Clicks) -> Self {
            let position = position;
            let clicks = clicks;
            Self { position, clicks }
        }

        pub fn get_byte_array(&self) -> [u8; 9] {
            let mut message = [0 as u8; 9];
            let position_array = self.position.get_byte_array();
            let clicks_byte = self.clicks.get_byte();

            for i in 0..8{
                message[i] = position_array[i];
            }

            message[8] = clicks_byte;

            message
        }
    }

    pub mod point {
        use crate::transform_i32_to_array_of_u8;

        pub struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            pub fn new(x: i32, y: i32) -> Self {
                Point { x, y }
            }
 
            pub fn get_byte_array(&self) -> [u8; 8] {
                let mut position_array = [0 as u8; 8];

                let x_array = transform_i32_to_array_of_u8(self.x);
                let y_array = transform_i32_to_array_of_u8(self.y);

                for i in 0..4 {
                    position_array[i] = x_array[i];
                    position_array[i + 4] = y_array[i];
                }

                position_array
            }
        }
    }

    pub mod clicks {
        pub enum MouseButton {
            Right,
            Left,
        }

        pub struct Clicks {
            down: bool,
            up: bool,
            button: MouseButton,
        }

        impl Clicks {
            pub fn new(down: bool, up: bool, button: MouseButton) -> Self {
                Clicks { down, up, button }
            }

            pub fn get_byte(&self) -> u8 {
                let mut byte = 0 as u8;
                byte = byte + 128 * self.down as u8;
                byte = byte + 64 * self.up as u8;

                let right = match self.button {
                    MouseButton::Right => true,
                    MouseButton::Left => false,
                };

                byte = byte + 32 * right as u8;

                byte
            }
        }
    }
}

fn connection_error(e: Error){
    println!("Failed to connect: {}", e);
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