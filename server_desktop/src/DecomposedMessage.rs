pub struct DecomposedMessage {
    position_x: [u8; 4],
    position_y: [u8; 4],
    left_click: bool,
    release_left_click: bool,
    right_click: bool,
    release_right_click: bool,
}

fn transform_array_of_u8_to_i32(array: [u8; 4] ) -> i32 {
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

    for i in 1..4{
        let addition: i32 = i32::from(array[i]) * 256_i32.pow(3 - (i as u32));
        number += addition;
        println!("byte {}: {}", i , array[i]);
    }
    
    number = number * (-1 as i32).pow(is_negative as u32);
    return number;
}

fn get_is_negative_and_number_from_last_byte(byte: u8) -> (bool, i32) {
    let is_negative = byte >= 128;
    let mut first_element_without_negative_bit = first_element;

    if is_negative {
        first_element_without_negative_bit = first_element - 128;
    }

    let addition: i32 = i32::from(first_element_without_negative_bit) * 256_i32.pow(3);
    (is_negative, number)
}

impl DecomposedMessage{
    pub fn new(data: [u8; 50]) -> (DecomposedMessage){
        let mut x_array = [0 as u8; 4];
        let mut y_array = [0 as u8; 4];
        for i in 0..4 {

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
    
}
