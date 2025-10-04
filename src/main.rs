use std::{thread, time};

fn print_top_bottom_border(size: i32) {
    for _ in 0..size {
        print!("-");
    }
    println!();
}

fn print_side(size: i32, character: char, whitespace: i32) {
    for _ in 0..whitespace {
        print!(" ");
    }
    print!("|");
    for _ in 0..size {
        print!("{character}");
    }
    println!("|");
}

fn print_thinner_side(size: i32, character: char, is_upper: bool) {
    if is_upper {
        print!(" \\");
    } else {
        print!(" /");
    }
    for _ in 0..size {
        print!("{character}");
    }
    if is_upper {
        print!("/");
    } else {
        print!("\\");
    }
    println!();
}

fn main() {
    const SIZE: i32 = 12;
    let animation_duration = time::Duration::from_secs(1);

    let mut characters: [char; 3] = ['*', ' ', ' '];

    loop {
        print_top_bottom_border(SIZE + 2);

        print_side(SIZE, characters[0], 0);
        print_side(SIZE, characters[0], 0);
        print_side(SIZE, characters[0], 0);

        print_thinner_side(SIZE - 2, characters[1], true);
        print_side(SIZE - 6, characters[1], 3);
        print_thinner_side(SIZE - 2, characters[1], false);

        print_side(SIZE, characters[2], 0);
        print_side(SIZE, characters[2], 0);
        print_side(SIZE, characters[2], 0);

        print_top_bottom_border(SIZE + 2);

        characters.rotate_right(1);

        thread::sleep(animation_duration);
        print!("\x1B[2J\x1B[1;1H");
    }
}
