use TerimalRtdm::*;
//use rand::Rng;
//rand::rng().random_range(0..=4);
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn rotate_right(shape: Vec<(i8, i8)>) -> Vec<(i8, i8)> {
    for i in 0..shape.len() {
        if shape.0 > 0 {
            shape.0 = -shape.0;
        } else if shape.0 < 0 {

        }
    }
}

fn main() {
    let mut app = App::new();
    clear(&mut app);
    raw_mode(true);
    show_cursor(false);

    let tick_delay = Mutex::new(1.0);
    let highscore = Mutex::new(0); //  https://doc.rust-lang.org/book/ch16-03-shared-state.html
    // 20 rows, and 10 cols
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for _ in 0..21 {
        grid.push(Vec::with_capacity(10));
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_secs_f64(*tick_delay.lock().unwrap()));
    });
    
    let shape_falling: bool = false;
    let cur_shape: Vec<(i8, i8)> = Vec::new();

    loop {
        if Key::o().pressed(&mut app, KeyType::Esc) {
            break;
        }

        render(&app);
        collect_presses(&mut app);
    }
    restore_terminal();
}
