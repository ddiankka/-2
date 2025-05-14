// src/homework/hw03.rs

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0  y == HEIGHT - 1 {
                output.push('*');
            } else if x == 0  x == WIDTH - 1 {
                output.push('*');
            } else if x == y * (WIDTH - 1) / (HEIGHT - 1) || x == (HEIGHT - 1 - y) * (WIDTH - 1) / (HEIGHT - 1) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
