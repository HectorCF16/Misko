use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};

fn transform_i32_to_array_of_u8(number: i32) -> [u8; 4] {
    let mut result_array = [0 as u8; 4];
    //suposant que el byte en el lloc 0 de l'array es el menys significatiu
    for i in 0..4 {
        print!("bit {}:", 3 - i);
        result_array[3 - i] = ((number >> 8 * i) & 0xff) as u8;
        println!("{}", result_array[3 - i]);
    }
    println!("");

    result_array
}

fn message_from_coordinates(x: i32, y: i32) -> [u8; 8] {
    let x_array = transform_i32_to_array_of_u8(x);
    let y_array = transform_i32_to_array_of_u8(y);
    let mut message_array = [0 as u8; 8];

    for i in 0..4 {
        message_array[i] = x_array[i];
        message_array[i + 4] = y_array[i];
        
    }

    return message_array;
}


fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut x = 0;
            let mut y = 0;
            loop{
                let ten_millis = time::Duration::from_millis(200);
                thread::sleep(ten_millis);
                let msg = message_from_coordinates(x, y);
                x+=1;
                y+=1;
                
                stream.write(&msg).unwrap();
                println!("Sent Hello, awaiting reply...");
            }
            
            
            /*let mut data = [0 as u8; 8]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == &msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }*/
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}