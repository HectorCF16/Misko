use eframe::egui;
use eframe::{NativeOptions, Renderer};
use std::fmt::format;
use std::net::{TcpStream};
use std::io::Error;
use std::str::FromStr;
use std::sync::Arc;
use client::Client;
use mouse_input::MouseInputs;
use mouse_input::point::Point;
use mouse_input::clicks::{Clicks, MouseButton};
use egui::{Pos2, Grid, Ui};
use winit::event::Touch;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

struct MyEguiApp {
    screen: Screens,
}

enum Screens {
    Connection(ConnectionScreen),
    TouchPad(TouchPadScreen),
}

struct TouchPadScreen {
    client: Client,
    last_position: Point, 
    just_clicked: bool,
    drawing_mode: bool,
}

struct ConnectionScreen {
    port_clicked: bool
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        MyEguiApp { screen: Screens::Connection(ConnectionScreen { port_clicked: false })}
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.screen {
                Screens::Connection(connection_screen) => {
                    let mut ip = "192.168.1.74";
                    let mut port = "3333".to_owned();
                    let mut password = "205990267";

                    /*
                    Grid::new("num_pad").show(ui, |ui| {
                        //port = button("7", ui, port);
                        ui.button("8");
                        ui.button("9");
                        ui.button("erase");
                        ui.end_row();
                        ui.button("4");
                        ui.button("5");
                        ui.button("6");
                        ui.button("enter");
                        ui.end_row();
                        ui.button("1");
                        ui.button("2");
                        ui.button("3");
                        ui.end_row();
                        ui.button("0");
                        self.screen = Screens::Connection(ConnectionScreen { port_clicked: true });

                    });
                     
                    
                    if ui.button(port).clicked() || connection_screen.port_clicked { 
                        
                    }; */
                    if ui.text_edit_singleline(&mut ip).clicked() {

                    }
                    if ui.text_edit_singleline(&mut password).clicked() {

                    }
                    let password = password.parse();

                    if ui.button("Try to connect").clicked() {
                        match password {
                            Ok(password) =>  {
                                match connection_request(format!("{}:{}", ip, port).as_str(), password) {
                                    Ok(client) => {
                                        self.screen = Screens::TouchPad(TouchPadScreen { client, last_position: Point::new(0,0), just_clicked: false, drawing_mode: false });
                                    },
                                    Err(error) => {
                                        ui.label(format!("{}", error).as_str());
                                    },
                                }
                            },
                            Err(_) => {
                                ui.label("password must be an integer!!!");
                            },
                        }
                        
                    }
                },
                Screens::TouchPad(touchpad) => {
                    let optional_position = ctx.pointer_latest_pos();
                    let position_difference;
                    let mut clicks = Clicks::new(false, false, MouseButton::Left);

                    match optional_position {
                        Some(position) => {
                            //ui.label(format!("{}, {}", position.x, position.y));
                            if touchpad.last_position.equals(Point::new(0,0)){
                                position_difference = Point::new(0,0);
                                if touchpad.drawing_mode {
                                    clicks = Clicks::new(true, false,MouseButton::Left);
                                }
                            }
                            else {
                                position_difference = Point::new( position.x as i32 - touchpad.last_position.x,  position.y as i32 - touchpad.last_position.y);
                            }

                            touchpad.last_position = Point::new(position.x as i32, position.y as i32);
                        },
                        None => {
                            if touchpad.drawing_mode {
                                clicks = Clicks::new(false, true, MouseButton::Left);
                            }
                            touchpad.last_position = Point::new(0,0);
                            position_difference = Point::new(0,0);
                        },
                    }

                    if !touchpad.drawing_mode {
                    
                        Grid::new("touchpad_buttons").show(ui, |ui| {
                            if ui.add(egui::Button::new("left_click").min_size(egui::Vec2::new(200.0,100.0))).clicked() {
                                clicks = Clicks::new(true, false, MouseButton::Left);
                                touchpad.just_clicked = true;
                            }
                            /*
                            if ui.button("left click").clicked() {
                                clicks = Clicks::new(true, false, MouseButton::Left);
                                touchpad.just_clicked = true;
                                //ui.label(format!());
                            } */
                            else {
                                if touchpad.just_clicked {
                                    clicks = Clicks::new(false, true, MouseButton::Left);
                                }
                                touchpad.just_clicked = false;
                            }/*
                            if ui.button("right click").clicked() {
                                //clicks = Clicks::new(true, false, MouseButton::Right);
                                //touchpad.just_clicked = true;
                            }
                            else {
                                if touchpad.just_clicked {
                                    
                                }
                                touchpad.just_clicked = false;
                            }
                            */
                        });
                    } 
                    ui.label(format!("{}", clicks.get_byte() as i32));
                    let mouse_input = MouseInputs::new(position_difference, clicks);
                    let left_click = Clicks::new(true, false, MouseButton::Left);
                    ui.label(format!("Left: {}", left_click.get_byte()));
                    let right_click = Clicks::new(true, false, MouseButton::Right);
                    ui.label(format!("Right: {}", right_click.get_byte()));
                    if ui.add(egui::Button::new("drawing mode").min_size(egui::Vec2::new(200.0, 100.0))).clicked() {
                        touchpad.drawing_mode = !touchpad.drawing_mode;
                    }
                    
                    touchpad.client.send_mouse_input(mouse_input);
                    
                },
            }
            
                
            /*
                let text;
                let position_difference;
                let touch_initiated;
                let tap_detected;
                match optional_position {
                    Some(position) => {
                        text = format!("{}, {}", position.x, position.y);
                        if self.screen.last_position.equals(Point::new(0,0)){
                            position_difference = Point::new(0,0);
                            touch_initiated = true;
                            self.screen.detecting_tap = true;
                        }
                        else {
                            position_difference = Point::new( position.x as i32 - self.screen.last_position.x,  position.y as i32 - self.screen.last_position.y);
                            touch_initiated = false;
                            if self.screen.detecting_tap {
                                self.screen.tap_detector -= 1;
                            }
                            if self.screen.tap_detector < 0 {
                                self.screen.detecting_tap = false;
                                self.screen.tap_detector = 8;
                            }
                        }
                        self.screen.last_position = Point::new(position.x as i32, position.y as i32);
                        tap_detected = false
                        
                    },
                    None => {
                        text = String::new();
                        self.screen.last_position = Point::new(0,0);
                        position_difference = Point::new(0,0);
                        
                        touch_initiated = false;
                        if self.screen.detecting_tap {
                            self.screen.tap_detector -= 1;
                        }
                        if self.screen.tap_detector < 0 {
                            self.screen.detecting_tap = false;
                            tap_detected = true;
                            self.screen.tap_detector = 8;
                        }
                        else {
                            tap_detected = false;
                        }
                    },
                }
                
                let connectionStatus;
                let mut message = "".to_owned();
                match &mut self.client {
                    Ok(client) => {
                        if ui.button("left click").clicked() {
                            let mouse_input = MouseInputs::new(Point::new(0,0), Clicks::new(true, false, MouseButton::Right));
                            for i in mouse_input.get_byte_array() {
                                message = format!("{} {}", message, i);
                            }
                            client.send_mouse_input(mouse_input);
                        }
                        if tap_detected {
                            ui.label("arhsioetharoishtoiar");
                            let mouse_input = MouseInputs::new(Point::new(0,0), Clicks::new(true, false, MouseButton::Left));
                            for i in mouse_input.get_byte_array() {
                                message = format!("{} {}", message, i);
                            }
                            client.send_mouse_input(mouse_input);
                        }
                        else {
                            let mouse_input = MouseInputs::new(position_difference, Clicks::new(false, false, MouseButton::Left));
                            client.send_mouse_input(mouse_input);
                        }
                        connectionStatus = "connected";
                        
                    },
                    Err(error) =>{
                        connectionStatus = stringify!(error);
                    },
                }
                ui.label("Hello");
                ui.label("Hello");
                ui.label("Hello");
                ui.label("Hello");
                ui.label("Hello");
                ui.label("Hello");
                ui.label(text);
                ui.label(connectionStatus);
                ui.label(format!("{},{}", self.screen.last_position.x, self.screen.last_position.y));
                if touch_initiated {
                    ui.label("touch initiated");
                }
                if tap_detected {
                    ui.label("tap detected");
                }
                ui.label(message);
                 */
                //ui.label(format!("{},{}", position_difference.x, position_difference.y));
        });
    }
}

fn create_numpad(ui: &mut Ui, port: &mut String) {
    Grid::new("num_pad").show(ui, |ui| {
        let seven = "7";
        if ui.button("7").clicked() {
            port.push_str(seven);
        };
        ui.button("8");
        ui.button("9");
        ui.button("erase");
        ui.end_row();
        ui.button("4");
        ui.button("5");
        ui.button("6");
        ui.button("enter");
        ui.end_row();
        ui.button("1");
        ui.button("2");
        ui.button("3");
        ui.end_row();
        ui.button("0");

    });
}

fn button(num: &str,ui: &mut Ui, port: String) -> String {
    let port = &mut port.clone();
    if ui.button(num).clicked() {
        port.push_str(num);
        port.to_string()
    } else {
        port.to_string()
    }
}
 

fn _main(mut options: NativeOptions) {
    options.renderer = Renderer::Wgpu;
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
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

 

fn send_mouse_inputs( client:  &mut Client){
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
            pub x: i32,
            pub y: i32,
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

            pub fn equals(&self, other: Point) -> bool {
                self.x == other.x && self.y == other.y
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

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));

    let mut options = NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));
    _main(options);
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn) // Default Log Level
        .parse_default_env()
        .init();

    _main(NativeOptions::default());
}
