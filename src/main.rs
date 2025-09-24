use TerimalRtdm::*;
//use rand::Rng;
//rand::rng().random_range(0..=4);
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    enum Tetromino {
        Reflectable {
            d: (Vec<Vec<u8>>, Vec<Vec<u8>>),
        },
        Rotatable {
            d: (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>),
        },
    }

    let tetrominoes: Vec<Tetromino> = vec![
        // I
        Tetromino::Reflectable {
            d: (
                vec![vec![1, 1, 1, 1]],
                vec![vec![1], vec![1], vec![1], vec![1]],
            ),
        },
        // O
        Tetromino::Reflectable {
            d: (
                vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]],
                vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]],
            ),
        },
        // T
        Tetromino::Rotatable {
            d: (
                vec![vec![1, 1, 1], vec![0, 1, 0]],
                vec![vec![1, 0], vec![1, 1], vec![1, 0]],
                vec![vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![0, 1], vec![1, 1], vec![0, 1]],
            ),
        },
        // J
        Tetromino::Rotatable {
            d: (
                vec![vec![0, 1], vec![0, 1], vec![1, 1]],
                vec![vec![1, 1, 1], vec![0, 0, 1]],
                vec![vec![1, 1], vec![1, 0], vec![1, 0]],
                vec![vec![1, 0, 0], vec![1, 1, 1]],
            ),
        },
        // L
        Tetromino::Rotatable {
            d: (
                vec![vec![1, 1], vec![0, 1], vec![0, 1]],
                vec![vec![0, 0, 1], vec![1, 1, 1]],
                vec![vec![1, 0], vec![1, 0], vec![1, 1]],
                vec![vec![1, 1, 1], vec![1, 0, 0]],
            ),
        },
        // S
        Tetromino::Reflectable {
            d: (
                vec![vec![0, 1, 1], vec![1, 1, 0]],
                vec![vec![1, 0], vec![1, 1], vec![0, 1]],
            ),
        },
        // Z
        Tetromino::Reflectable {
            d: (
                vec![vec![1, 1, 0], vec![0, 1, 1]],
                vec![vec![0, 1], vec![1, 1], vec![1, 0]],
            ),
        },
    ];

    let mut app = App::new();
    clear(&mut app);
    raw_mode(true);
    show_cursor(false);

    let tick_delay = Mutex::new(1.0);
    // 20 rows, and 10 cols
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for _ in 0..19 {
        grid.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    thread::spawn(move || {
        loop {
            let mut app1 = App::new();
            let grid_clone = grid.clone();

            clear(&mut app1);
            raw_mode(true);
            show_cursor(false);

            for i in 0..grid_clone.len() {
                for j in 0..grid_clone[i].len() {
                    let color = match grid_clone[i][j] {
                        0 => Color::White,
                        1 => Color::Cyan,
                        2 => Color::Yellow,
                        3 => Color::Magenta,
                        4 => Color::Blue,
                        5 => Color::BrightYellow,
                        6 => Color::Green,
                        7 => Color::Red,
                        _ => Color::BrightMagenta,
                    };
                    Text::new()
                        .background(color)
                        .show(&mut app1, " ", pos!(i, j));
                }
            }

            render(&app1);
            thread::sleep(Duration::from_secs_f64(*tick_delay.lock().unwrap()));
        }
    });

    loop {
        if Key::o().pressed(&mut app, KeyType::Esc) {
            break;
        }
        collect_presses(&mut app);
    }
    restore_terminal();
}
