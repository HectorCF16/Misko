pub mod mouse_input {
    use enigo::*;


    const MESSAGE_NUMBER_OF_BYTES: usize = 50;
    const BYTE_NUMBER_OF_BYTES: usize = 4;

    pub struct MouseInputs {
        position_x: i32,
        position_y: i32,
        left_click: bool,
        release_left_click: bool,
        right_click: bool,
        release_right_click: bool,
    }

    impl MouseInputs {
        pub fn new(data: [u8; MESSAGE_NUMBER_OF_BYTES]) -> Self {
            let mut x_array = [0 as u8; BYTE_NUMBER_OF_BYTES];
            let mut y_array = [0 as u8; BYTE_NUMBER_OF_BYTES];

            for i in 0..BYTE_NUMBER_OF_BYTES {
                x_array[i] = data[i];
                y_array[i] = data[i + BYTE_NUMBER_OF_BYTES];
            }

            let x = transform_array_of_u8_to_i32(x_array);
            let y = transform_array_of_u8_to_i32(y_array);

            Self {
                position_x: x,
                position_y: y,
                left_click: false,
                release_left_click: false,
                right_click: false,
                release_right_click: false,
            }
            
        }

        pub fn execute_mouse_inputs(&self){
            move_relative(self.position_x, self.position_y);
            println!("x: {}, y: {}", self.position_x, self.position_y);
        }
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

    

    
    fn move_relative(x: i32, y: i32) {
        let mut enigo = Enigo::new();
        enigo.mouse_move_relative(x, y);
    }

    
}
