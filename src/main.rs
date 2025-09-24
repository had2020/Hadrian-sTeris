use TerimalRtdm::*;
//use rand::Rng;
//rand::rng().random_range(0..=4);
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    clear(&mut app);
    raw_mode(true);
    show_cursor(false);

    let tick_delay = Mutex::new(1.0);
    // 20 rows, and 10 cols
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for _ in 0..21 {
        grid.push(Vec::with_capacity(10));
    }

    thread::spawn(move || {
        let mut app1 = App::new();
        let mut grid_clone = grid.clone();
        Text::new().show(&mut app1, "test", pos!(0, 0));
        for i in 0..grid_clone.len() {
            for j in 0..grid_clone[i].len() {
                let pixel = if grid_clone[i][j] { "🔳" } else { "⬛️" };
                Text::new().show(&mut app1, "test", pos!(i, j));
            }
        }
        thread::sleep(Duration::from_secs_f64(*tick_delay.lock().unwrap()));
    });

    loop {
        if Key::o().pressed(&mut app, KeyType::Esc) {
            break;
        }

        render(&app);
        collect_presses(&mut app);
    }
    restore_terminal();
}
