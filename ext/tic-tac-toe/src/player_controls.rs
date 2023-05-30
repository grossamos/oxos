use oxos_syscall::{wait_for_gpio_to_flip, get_gpio_values, uart_send};

pub fn get_field_input() -> (u32, u32) {
    let mut x: u32;
    let y: u32;

    loop {
        wait_for_gpio_to_flip(20);

        let mut pins = [false; 32];
        pins[1] = true;
        pins[7] = true;
        pins[8] = true;
        pins[10] = true;
        pins[9] = true;
        pins[11] = true;

        let values = get_gpio_values(pins);

        let x_row = [
            get_pin_value(1, values),
            get_pin_value(7, values),
            get_pin_value(8, values),
        ];
        let y_row = [
            get_pin_value(10, values),
            get_pin_value(9, values),
            get_pin_value(11, values),
        ];

        x = match get_true_index(x_row) {
            Some(val) => val,
            None => continue,
        };
        y = match get_true_index(y_row) {
            Some(val) => val,
            None => continue,
        };

        break;
    }
    (x, y)
}

fn get_pin_value(pin_num: usize, values: [Option<bool>; 32]) -> bool {
    match values[pin_num]{
        Some(val) => val,
        None => panic!(),
    }
}

fn get_true_index(row: [bool; 3]) -> Option<u32> {
    let mut sum = 0;
    for i in row {
        if i {
            sum += 1;
        }
    }
    if sum != 1 {
        uart_send("invalid sum");
        return None;
    } 

    for i in 0..row.len() {
        if row[i] {
            return Some(i as u32)
        }
    }
    panic!();
}
