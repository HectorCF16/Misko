use client::Client;
use eframe::egui;
use eframe::{NativeOptions, Renderer};
use egui::style::Margin;
use egui::{CentralPanel, Grid, InputState, Pos2, Ui, Vec2};
use mouse_input::clicks::{Clicks, MouseButton};
use mouse_input::point::Point;
use mouse_input::MouseInputs;
use std::fmt::format;
use std::io::Error;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::Arc;
use winit::event::Touch;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

struct MyEguiApp {
    screen: Screens,
}

enum Screens {
    Connection(ConnectionScreen),
    TouchPad(TouchPadScreen),
    Menu(MenuScreen),
}

struct TouchPadScreen {
    client: Client,
    last_position: Point,
    just_clicked: bool,
    drawing_mode: bool,
}

struct ConnectionScreen {
    focus: Focus,
    ip: String,
    port: String,
    password: String,
}

struct MenuScreen {
    there_is_connection: bool,
}

enum Focus {
    Ip,
    Port,
    Password,
    NoFocus,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        MyEguiApp {
            screen: Screens::Connection(ConnectionScreen {
                focus: Focus::NoFocus,
                ip: "192.168.1.74".to_owned(),
                port: "3333".to_owned(),
                password: "205990267".to_owned(),
            }),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //default pixels_per_point in egui are 2.75
            ctx.input_mut().pixels_per_point = 4.0;
            match &mut self.screen {
                Screens::Connection(connection_screen) => {
                    let password = connection_screen.password.parse();
                    let mut login_try = false;

                    Grid::new("connection_form").show(ui, |ui| {
                        //this is done cause egui in android currently doesn't support, i think
                        //(IDEA) I could implement a system for adjusting the position of the elements
                        //with percentages of the screen size
                        ui.label("   ");
                        ui.label("Ip:");
                        if ui.button(connection_screen.ip.clone()).clicked() {
                            connection_screen.focus = Focus::Ip;
                        }
                        ui.end_row();
                        ui.label("   ");
                        ui.label("Port:");
                        if ui.button(connection_screen.port.clone()).clicked() {
                            connection_screen.focus = Focus::Port;
                        }
                        ui.end_row();
                        ui.label("   ");
                        ui.label("Password:");
                        if ui.button(connection_screen.password.clone()).clicked() {
                            connection_screen.focus = Focus::Password;
                        }
                        ui.end_row();
                        ui.label("         ");

                        login_try = ui.button("Try to connect").clicked();
                    });

                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");

                    match &mut connection_screen.focus {
                        Focus::Ip => {
                            create_numpad(ui, &mut connection_screen.ip);
                        }
                        Focus::Port => {
                            create_numpad(ui, &mut connection_screen.port);
                        }
                        Focus::Password => {
                            create_numpad(ui, &mut connection_screen.password);
                        }
                        Focus::NoFocus => {}
                    }
                    if login_try {
                        match password {
                            Ok(password) => {
                                match connection_request(
                                    format!("{}:{}", connection_screen.ip, connection_screen.port)
                                        .as_str(),
                                    password,
                                ) {
                                    Ok(client) => {
                                        self.screen = Screens::TouchPad(TouchPadScreen {
                                            client,
                                            last_position: Point::new(0, 0),
                                            just_clicked: false,
                                            drawing_mode: false,
                                        });
                                    }
                                    Err(error) => {
                                        ui.label(format!("{}", error).as_str());
                                    }
                                }
                            }
                            Err(_) => {
                                ui.heading("The password must be an integer!!!");
                            }
                        }
                    }
                }
                Screens::TouchPad(touchpad) => {
                    let optional_position = ctx.pointer_latest_pos();
                    let position_difference;
                    let mut clicks = Clicks::new(false, false, MouseButton::Left);

                    match optional_position {
                        Some(position) => {
                            //ui.label(format!("{}, {}", position.x, position.y));
                            if touchpad.last_position.equals(Point::new(0, 0)) {
                                position_difference = Point::new(0, 0);
                                if touchpad.drawing_mode {
                                    clicks = Clicks::new(true, false, MouseButton::Left);
                                }
                            } else {
                                position_difference = Point::new(
                                    (position.x as i32 - touchpad.last_position.x) * 3 / 2,
                                    (position.y as i32 - touchpad.last_position.y) * 3 / 2,
                                );
                            }

                            touchpad.last_position =
                                Point::new(position.x as i32, position.y as i32);
                        }
                        None => {
                            if touchpad.drawing_mode {
                                clicks = Clicks::new(false, true, MouseButton::Left);
                            }
                            touchpad.last_position = Point::new(0, 0);
                            position_difference = Point::new(0, 0);
                        }
                    }

                    if ui
                        .add(
                            egui::Button::new("drawing mode")
                                .min_size(egui::Vec2::new(150.0, 75.0)),
                        )
                        .clicked()
                    {
                        touchpad.drawing_mode = !touchpad.drawing_mode;
                    }

                    if !touchpad.drawing_mode {
                        Grid::new("touchpad_buttons").show(ui, |ui| {
                            if ui
                                .add(
                                    egui::Button::new("left_click")
                                        .min_size(egui::Vec2::new(150.0, 75.0)),
                                )
                                .clicked()
                            {
                                clicks = Clicks::new(true, false, MouseButton::Left);
                                touchpad.just_clicked = true;
                            } else {
                                if touchpad.just_clicked {
                                    clicks = Clicks::new(false, true, MouseButton::Left);
                                }
                                touchpad.just_clicked = false;
                            }
                        });
                    }
                    let mouse_input = MouseInputs::new(position_difference, clicks);

                    touchpad.client.send_mouse_input(mouse_input);
                }
                Screens::Menu(menu) => {
                    if menu.there_is_connection {
                        ui.button("Close connection.");
                    }
                },
            }
        });
    }
}

fn create_numpad(ui: &mut Ui, input_reference: &mut String) {
    Grid::new("num_pad").show(ui, |ui| {
        ui.label("       ");
        button("7", ui, input_reference);
        button("8", ui, input_reference);
        button("7", ui, input_reference);

        ui.end_row();

        ui.label("       ");
        button("4", ui, input_reference);
        button("5", ui, input_reference);
        button("6", ui, input_reference);

        ui.end_row();

        ui.label("       ");
        button("1", ui, input_reference);
        button("2", ui, input_reference);
        button("3", ui, input_reference);

        ui.end_row();

        ui.label("       ");
        button("0", ui, input_reference);
        button(".", ui, input_reference);
        if ui
            .add(egui::Button::new("erase").min_size(Vec2::new(40.0, 40.0)))
            .clicked()
        {
            input_reference.pop();
        }
    });
}

fn button(num: &str, ui: &mut Ui, input_reference: &mut String) {
    if ui
        .add(egui::Button::new(num).min_size(Vec2::new(40.0, 40.0)))
        .clicked()
    {
        input_reference.push_str(num);
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

fn connection_request(address: &str, password: i32) -> Result<Client, Error> {
    handle_connection(TcpStream::connect(address), password)
}

fn handle_connection(tcp_stream: Result<TcpStream, Error>, password: i32) -> Result<Client, Error> {
    let client: Result<Client, Error>;
    match tcp_stream {
        Ok(stream) => {
            client = Ok(login(stream, password));
        }
        Err(e) => {
            client = Err(e);
        }
    }
    println!("Terminated.");

    client
}

fn login(stream: TcpStream, password: i32) -> Client {
    println!("Successfully connected to server in port 3333");
    let mut client = Client::new(stream);

    client.send_password(password);
    client
}

mod client {
    use crate::{mouse_input::MouseInputs, transform_i32_to_array_of_u8};
    use std::{io::Write, net::TcpStream};

    pub struct Client {
        stream: TcpStream,
    }

    impl Client {
        pub fn new(stream: TcpStream) -> Self {
            Client { stream }
        }

        pub fn send_password(&mut self, password: i32) {
            let password_array = transform_i32_to_array_of_u8(password);

            self.stream.write(&password_array);
        }

        pub fn send_mouse_input(&mut self, mouse_input: MouseInputs) {
            let message_array = mouse_input.get_byte_array();

            self.stream.write(&message_array);
        }

        pub fn close_connection(&mut self) {
            self.stream.shutdown(std::net::Shutdown::Both);
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

            for i in 0..8 {
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

fn connection_error(e: Error) {
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
    let negative_bit = if is_negative { 128 as u8 } else { 0 as u8 };
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
