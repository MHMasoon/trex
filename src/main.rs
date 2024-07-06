use crossterm::terminal::size;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::enable_raw_mode;
use crossterm::style::Print;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crossterm::QueueableCommand;
use crossterm::cursor;
use crossterm::event::poll;
use crossterm::event::Event;
use crossterm::event::read;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

struct World {
    length: u16,
    height: u16,
    next_top_stone_distance: u16,
    next_bottom_stone_distance: u16,
    next_grain_distance: u16,
    next_cactus_distance: u8,
    rng: ThreadRng,
    top_line: Vec<LineStatus>,
    bottom_line: Vec<LineStatus>,
    ground: Vec<bool>,
    cactuses_pixels: Vec<(u16, u16)>,
    trex_pixels: [(u16, u16); 7],
    trex_status: TrexStatus,
    trex_height: u16,
    trex_max_height: u16,
    game_status: GameStatus,
    stdout: Stdout,
}

enum GameStatus {
    Paused,
    Running,
    Over,
    Closed,
}

#[derive(PartialEq)]
enum TrexStatus {
    OnGround,
    Rising,
    Falling,
}

enum LineStatus {
    Line,
    StoneStart,
    StoneMiddle,
    StoneEnd,
}

#[derive(PartialEq)]
enum TrexMoveDirection {
    Up,
    Down,
}

fn main() {
    // change terminal settings
    enable_raw_mode();

    // create world
    let mut world = World {
        length: size().unwrap().0,
        height: size().unwrap().1,
        rng: rand::thread_rng(),
        stdout: stdout(),
        next_top_stone_distance: 0,
        next_bottom_stone_distance: 0,
        next_grain_distance: 0,
        next_cactus_distance: 0,
        top_line: Vec::new(),
        bottom_line: Vec::new(),
        ground: Vec::new(),
        cactuses_pixels: Vec::new(),
        trex_pixels: [(0, 0); 7],
        trex_height: 0,
        trex_max_height: 10,
        trex_status: TrexStatus::OnGround,
        game_status: GameStatus::Paused,
    };
    
    fn initiate_world(world: &mut World) {
        // TODO: the range's maximum should be equal to terminal width if
        // the width is larger than 2000
        world.next_top_stone_distance = world.rng.gen_range(50..100);
        world.next_bottom_stone_distance = world.rng.gen_range(50..100);
        world.next_grain_distance = world.rng.gen_range(10..20);
        world.next_cactus_distance = world.rng.gen_range(100..200);
        for _ in 0..world.length {
            next_frame(world);
        }

        // initiate trex pixels
        let trex_x_origin: u16 = 2;
        let trex_y_origin: u16 = world.height - 3;
        world.trex_pixels = [
            (trex_x_origin + 2, trex_y_origin - 0 + 1),
            (trex_x_origin + 0, trex_y_origin - 0 + 1),
            (trex_x_origin + 0, trex_y_origin - 1 + 1),
            (trex_x_origin + 1, trex_y_origin - 1 + 1),
            (trex_x_origin + 2, trex_y_origin - 1 + 1),
            (trex_x_origin + 2, trex_y_origin - 2 + 1),
            (trex_x_origin + 3, trex_y_origin - 2 + 1),
        ];
    }

    // check events
    fn check_events(world: &mut World) {
        for _ in 0..5 {
            // `poll()` waits for an `Event` for a given time period
            if poll(Duration::from_millis(0)).unwrap() {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match read().unwrap() {
                    Event::FocusGained => println!("FocusGained"),
                    Event::FocusLost => println!("FocusLost"),
                    Event::Key(event) => {
                        match (event.code, event.modifiers) {
                            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                                match world.game_status {
                                    GameStatus::Paused => world.game_status = GameStatus::Running,
                                    GameStatus::Running => world.game_status = GameStatus::Paused,
                                    _ => {},
                                }
                            },
                            (KeyCode::Char(' '), KeyModifiers::NONE) => {
                                if world.trex_status == TrexStatus::OnGround {
                                    world.trex_status = TrexStatus::Rising;
                                }
                            },
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                world.game_status = GameStatus::Closed;
                            }
                            _ => {},
                        }
                    },
                    Event::Mouse(event) => println!("{:?}", event),
                    #[cfg(feature = "bracketed-paste")]
                    Event::Paste(data) => println!("Pasted {:?}", data),
                    Event::Resize(width, height) => println!("New size {}x{}", width, height),
                    _ => println!("Uncovered Event!"),
                }
            } else {
                // Timeout expired and no `Event` is available
            }
        }
    }

    // draw world
    fn draw(world: &mut World) {
        world.stdout.queue(cursor::Hide);
        world.stdout.queue(Clear(ClearType::All));
        world.stdout.queue(Clear(ClearType::Purge));

        // draw top line
        world.stdout.queue(cursor::MoveTo(0, world.height - 3));
        for line_status in &world.top_line {
            match line_status {
                LineStatus::StoneStart => world.stdout.queue(Print("/")),
                LineStatus::StoneMiddle => world.stdout.queue(Print("-")),
                LineStatus::StoneEnd => world.stdout.queue(Print("\\")),
                LineStatus::Line => world.stdout.queue(Print("_")),
            };
        }

        // draw bottom line
        world.stdout.queue(cursor::MoveTo(0, world.height - 1));
        for line_status in &world.bottom_line {
            match line_status {
                LineStatus::StoneStart => world.stdout.queue(Print("/")),
                LineStatus::StoneMiddle => world.stdout.queue(Print("-")),
                LineStatus::StoneEnd => world.stdout.queue(Print("\\")),
                LineStatus::Line => world.stdout.queue(Print("_")),
            };
        }

        // draw ground
        world.stdout.queue(cursor::MoveTo(0, world.height - 2));
        for is_grain in &world.ground {
            if *is_grain {
                world.stdout.queue(Print("."));
            } else {
                world.stdout.queue(cursor::MoveRight(1));
            }
        }

        // draw cactuses
        for pixel in world.cactuses_pixels.iter() {
            world.stdout.queue(cursor::MoveTo(pixel.0, pixel.1));
            world.stdout.queue(Print("█"));
        }

        // draw trex
        for trex_pixel in world.trex_pixels.iter() {
            world.stdout.queue(cursor::MoveTo(trex_pixel.0, trex_pixel.1));
            world.stdout.queue(Print("█"));
        }

        world.stdout.flush();
    }

    // create next frame
    fn next_frame(world: &mut World) {
        // generate top line
        match world.next_top_stone_distance {
            2 => world.top_line.push(LineStatus::StoneStart),
            1 => world.top_line.push(LineStatus::StoneMiddle),
            0 => {
                world.top_line.push(LineStatus::StoneEnd);
                world.next_top_stone_distance = world.rng.gen_range(50..100);
            },
            _ => world.top_line.push(LineStatus::Line),
        }
        world.next_top_stone_distance -= 1;
        
        // generate bottom line
        match world.next_bottom_stone_distance {
            2 => world.bottom_line.push(LineStatus::StoneStart),
            1 => world.bottom_line.push(LineStatus::StoneMiddle),
            0 => {
                world.bottom_line.push(LineStatus::StoneEnd);
                world.next_bottom_stone_distance = world.rng.gen_range(50..100);
            },
            _ => world.bottom_line.push(LineStatus::Line),
        }
        world.next_bottom_stone_distance -= 1;

        // generate ground
        if world.next_grain_distance == 0 {
            world.ground.push(true);
            world.next_grain_distance = world.rng.gen_range(10..20);
        } else {
            world.ground.push(false);
        }
        world.next_grain_distance -= 1;
        
        // move cactuses
        for pixel in world.cactuses_pixels.iter_mut() {
            pixel.0 -= 1;
        }

        // generate cactus
        if world.next_cactus_distance == 0 {
            let cactus_form: u8 = world.rng.gen_range(1..=3);
            let mut cactus_pixels: Vec<(u16, u16)> = Vec::new();
            match cactus_form {
                1 => {
                    cactus_pixels = vec![
                        (world.length + 1, world.height - 2),
                        (world.length + 1, world.height - 3),
                        (world.length + 1, world.height - 4),
                        (world.length + 1, world.height - 5),
                        (world.length + 2, world.height - 3),
                        (world.length + 3, world.height - 3),
                        (world.length + 3, world.height - 4),
                    ];
                },
                2 => {
                    cactus_pixels = vec![
                        (world.length + 4, world.height - 2),
                        (world.length + 4, world.height - 3),
                        (world.length + 4, world.height - 4),
                        (world.length + 4, world.height - 5),
                        (world.length + 3, world.height - 3),
                        (world.length + 2, world.height - 3),
                        (world.length + 2, world.height - 4),
                    ];
                },
                3 => {
                    cactus_pixels = vec![
                        (world.length + 4, world.height - 2),
                        (world.length + 4, world.height - 3),
                        (world.length + 4, world.height - 4),
                        (world.length + 4, world.height - 5),
                        (world.length + 3, world.height - 3),
                        (world.length + 2, world.height - 3),
                        (world.length + 2, world.height - 4),
                        (world.length + 5, world.height - 3),
                        (world.length + 6, world.height - 3),
                        (world.length + 6, world.height - 4),
                    ];
                },
                _ => (),
            }
            world.cactuses_pixels.extend(cactus_pixels);
            
            world.next_cactus_distance = world.rng.gen_range(100..200);
        }
        world.next_cactus_distance -= 1;

        // move trex
        match world.trex_status {
            TrexStatus::Rising => {
                if world.trex_height < world.trex_max_height {
                    move_trex(world, TrexMoveDirection::Up);
                } else {
                    world.trex_status = TrexStatus::Falling;
                }
            },
            TrexStatus::Falling => {
                if world.trex_height > 0 {
                    move_trex(world, TrexMoveDirection::Down);
                } else {
                    world.trex_status = TrexStatus::OnGround;
                }
            },
            _ => {
            }
        }
    }
    
    fn move_trex(world: &mut World, direction : TrexMoveDirection) {
        if direction  == TrexMoveDirection::Up {
            world.trex_height += 1;
        } else {
            world.trex_height -= 1;
        }

        for pixel in world.trex_pixels.iter_mut() {
            match direction {
                TrexMoveDirection::Up => {
                    pixel.1 -= 1;
                },
                TrexMoveDirection::Down => {
                    pixel.1 += 1;
                },
            }
        }
    }

    fn delete_first_frame(world: &mut World) {
        world.top_line.remove(0);
        world.bottom_line.remove(0);
        world.ground.remove(0);

        // delete cactus pixels that are out of screen
        world.cactuses_pixels.retain(|pixel| pixel.0 > 0);
    }

    fn control_flow(world: &mut World) {
        initiate_world(world);
        draw(world);
        loop {
            check_events(world);

            match world.game_status {
                GameStatus::Paused => continue,
                GameStatus::Closed => return,
                _ => {},
            }
            next_frame(world);
            delete_first_frame(world);
            draw(world);
            sleep(Duration::from_millis(50));
        }
    }

    control_flow(&mut world);
}