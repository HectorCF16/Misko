use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use enigo::*;

struct DecomposedMessage {
    position_x: [u8; 4],
    position_y: [u8; 4],
    left_click: bool,
    release_left_click: bool,
    right_click: bool,
    release_right_click: bool,
}

fn move_relative(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
}

fn transform_array_of_u8_to_i32(array: [u8; 4] ) -> i32 {
    let mut number: i32 = 0;
    //first element is the most valuable (significative)
    for i in 0..4{
        let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
        number += addition;
    }    
    return number;
}

fn data_to_x_y_arrays(data: [u8; 50]) -> (DecomposedMessage){
    let mut x_array = [0 as u8; 4];
    let mut y_array = [0 as u8; 4];
    for i in 0..4 {
        x_array[i] = data[i];
        y_array[i] = data[i + 4];
    }
    DecomposedMessage{
        position_x: x_array,
        position_y: y_array,
        left_click: false,
        release_left_click: false,
        right_click: false,
        release_right_click: false,
    }
}

fn is_zero(array: [u8; 50]) -> bool{
    for byte in array{
        if byte != 0 as u8 {
            return false;
        }
    }
    true
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    while match stream.read(&mut data) {
        Ok(size) => {
            if !is_zero(data) {
                let decomposed_message = data_to_x_y_arrays(data);

                //print x array
                print!("x_array: ");
                for byte in decomposed_message.position_x{
                    print!("{}", byte);
                }
                println!("");
                
                //print y array
                print!("y_array: ");
                for byte in decomposed_message.position_y{
                    print!("{}", byte);
                }
                println!("");

                let x = transform_array_of_u8_to_i32(decomposed_message.position_x);
                let y = transform_array_of_u8_to_i32(decomposed_message.position_y);
                move_relative(x, y);
                println!("x: {}, y: {}", x, y);
                data = [0 as u8; 50];
            }
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