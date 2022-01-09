 
use std::{thread, time};

const WIDTH: i64 = 10;
const HEIGHT: i64 = 20;
const SIZE: usize = WIDTH as usize * HEIGHT as usize;

fn display(state: [bool; SIZE]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let v = if state[(y * WIDTH + x) as usize] { "X " } else { "  " };
            print!("{}", v); 
        }
        println!()
    }
}

fn init() -> [bool; SIZE] {
    let mut state: [bool; SIZE] = [false; SIZE];
    set(0, 2, &mut state);
    set(1, 2, &mut state);
    set(2, 2, &mut state);
    set(2, 1, &mut state);
    set(1, 0, &mut state);
    state
}

fn set(x: i64, y: i64, state: &mut [bool; SIZE]) {
    if x < WIDTH && y < HEIGHT {
        state[(y * WIDTH + x) as usize] = true;
    }
}

fn get(x: i64, y: i64, state: [bool; SIZE]) -> bool {
    if x < 0 || x  >= WIDTH { false }
    else if y < 0 || y >= HEIGHT { false }
    else { state[(y * WIDTH + x) as usize] }
}

fn next_gen(state: [bool; SIZE]) -> [bool; SIZE] {
    let mut nstate: [bool; SIZE] = [false; SIZE];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let nc = get_neigh_count(x, y, state);
            let nv = if get(x, y, state) {
                if nc <= 1 || nc >= 4 {
                    false
                } else {
                    true
                }   
            } else {
                if nc == 3 {
                    true
                } else {
                    false
                }
            };
            if nv { set(x, y, &mut nstate) }
        }
    }

    return nstate;
}

fn get_neigh_count(x: i64, y: i64, state: [bool; SIZE]) -> i32 {
    let mut count :i32 = 0;

    for x2 in -1..2 {
        for y2 in -1..2 {
            if !(x2 == 0 && y2 == 0) {
                count += get(x + x2, y + y2, state) as i32
            }
        }
    }

    return count;
}


fn main() {
    let mut state: [bool; SIZE] = init();
    display(state);

    for _i in 0..1000 {
        let nstate = next_gen(state);
        if state == nstate { break }
        state = nstate;
        thread::sleep(time::Duration::from_millis(200));
        print!("\x1B[{}A", HEIGHT);
        display(state);
    }

}
