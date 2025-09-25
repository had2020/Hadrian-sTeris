use rand::Rng;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Barrier, Mutex,
};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use TerimalRtdm::*;

#[derive(Clone)]
enum Tetromino {
    Reflectable {
        d: (Vec<Vec<u8>>, Vec<Vec<u8>>),
    },
    Rotatable {
        d: (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>),
    },
}

fn main() {
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
            d: (vec![vec![2, 2], vec![2, 2]], vec![vec![2, 2], vec![2, 2]]),
        },
        // T
        Tetromino::Rotatable {
            d: (
                vec![vec![3, 3, 3], vec![0, 3, 0]],
                vec![vec![3, 0], vec![3, 3], vec![3, 0]],
                vec![vec![0, 3, 0], vec![3, 3, 3]],
                vec![vec![0, 3], vec![3, 3], vec![0, 3]],
            ),
        },
        // J
        Tetromino::Rotatable {
            d: (
                vec![vec![0, 4], vec![0, 4], vec![4, 4]],
                vec![vec![4, 4, 4], vec![0, 0, 4]],
                vec![vec![4, 4], vec![4, 0], vec![4, 0]],
                vec![vec![4, 0, 0], vec![4, 4, 4]],
            ),
        },
        // L
        Tetromino::Rotatable {
            d: (
                vec![vec![5, 5], vec![0, 5], vec![0, 5]],
                vec![vec![0, 0, 5], vec![5, 5, 5]],
                vec![vec![5, 0], vec![5, 0], vec![5, 5]],
                vec![vec![5, 5, 5], vec![5, 0, 0]],
            ),
        },
        // S
        Tetromino::Reflectable {
            d: (
                vec![vec![0, 6, 6], vec![6, 6, 0]],
                vec![vec![6, 0], vec![6, 6], vec![0, 6]],
            ),
        },
        // Z
        Tetromino::Reflectable {
            d: (
                vec![vec![7, 7, 0], vec![0, 7, 7]],
                vec![vec![0, 7], vec![7, 7], vec![7, 0]],
            ),
        },
    ];

    let mut app = App::new();
    clear(&mut app);
    raw_mode(true);
    show_cursor(false);

    let cur_ter = Arc::new(Mutex::new({
        let idx = rand::rng().random_range(0..=6);
        match tetrominoes[idx].clone() {
            Tetromino::Reflectable { d } => d.0,
            Tetromino::Rotatable { d } => d.0,
        }
    }));
    let ter_pos = Arc::new(Mutex::new((5usize, 0usize)));
    let grid = Arc::new(Mutex::new(vec![vec![0u8; 10]; 20])); // 20 rows, 10 cols

    let stop = Arc::new(AtomicBool::new(false));
    let start_gate = Arc::new(Barrier::new(3)); // main + 2 workers

    thread::scope(|s| {
        // game tick
        {
            let cur_ter = Arc::clone(&cur_ter);
            let ter_pos = Arc::clone(&ter_pos);
            let grid = Arc::clone(&grid);
            let stop = Arc::clone(&stop);
            let start = Arc::clone(&start_gate);

            s.spawn(move || {
                start.wait();
                let mut highscore = 0;
                let mut fall_delay: f64 = 1.0;
                let mut start: Instant = Instant::now();

                loop {
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }

                    let (grid_snapshot, ter_snapshot, pos_snapshot) = {
                        let g = grid.lock().unwrap().clone();
                        let t = cur_ter.lock().unwrap().clone();
                        let p = *ter_pos.lock().unwrap();
                        (g, t, p)
                    };

                    let mut app1 = App::new();
                    clear(&mut app1);
                    raw_mode(true);
                    show_cursor(false);

                    // grid
                    for y in 0..grid_snapshot.len() {
                        for x in 0..grid_snapshot[y].len() {
                            let color = match grid_snapshot[y][x] {
                                0 => Color::Black,
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
                                .show(&mut app1, "·", pos!(x, y));
                        }
                    }

                    // current tetromino
                    for (dy, row) in ter_snapshot.iter().enumerate() {
                        for (dx, cell) in row.iter().enumerate() {
                            if *cell == 0 {
                                continue;
                            }
                            let color = match *cell {
                                1 => Color::Cyan,
                                2 => Color::Yellow,
                                3 => Color::Magenta,
                                4 => Color::Blue,
                                5 => Color::BrightYellow,
                                6 => Color::Green,
                                7 => Color::Red,
                                _ => Color::BrightMagenta,
                            };
                            Text::new().background(color).show(
                                &mut app1,
                                "·",
                                pos!(pos_snapshot.0 + dx, pos_snapshot.1 + dy),
                            );
                        }
                    }

                    Text::new().show(
                        &mut app1,
                        &format!("Highscore: {} Lines", highscore),
                        pos!(0, 22),
                    );

                    render(&app1);

                    // falling
                    if start.elapsed().as_secs_f64() >= fall_delay {
                        let mut p = ter_pos.lock().unwrap();
                        if p.1 != 20 {
                            p.1 = p.1.saturating_add(1);
                        }
                        start = Instant::now();
                    }

                    thread::sleep(Duration::from_secs_f64(0.1));
                }
            });
        }
        {
            let stop = Arc::clone(&stop);
            let ter_pos = Arc::clone(&ter_pos);
            let start = Arc::clone(&start_gate);

            // input
            s.spawn(move || {
                start.wait();
                let mut app = App::new();

                loop {
                    if Key::o().pressed(&mut app, KeyType::Esc) {
                        stop.store(true, Ordering::Relaxed);
                        break;
                    }
                    if Key::o().pressed(&mut app, KeyType::A)
                        || Key::o().pressed(&mut app, KeyType::LeftArrow)
                    {
                        let mut p = ter_pos.lock().unwrap();
                        p.0 = p.0.saturating_sub(1);
                    }
                    if Key::o().pressed(&mut app, KeyType::D)
                        || Key::o().pressed(&mut app, KeyType::RightArrow)
                    {
                        let mut p = ter_pos.lock().unwrap();
                        p.0 = p.0.saturating_add(1);
                    }
                    if Key::o().pressed(&mut app, KeyType::S)
                        || Key::o().pressed(&mut app, KeyType::DownArrow)
                    {
                        let mut p = ter_pos.lock().unwrap();
                        if p.1 != 20 {
                            p.1 = p.1.saturating_add(1);
                        }
                    }
                    // TODO: rotation on w/UpArrow
                    collect_presses(&mut app);

                    thread::sleep(Duration::from_millis(1));
                }
            });
        }
        start_gate.wait();
    });

    restore_terminal();
}
