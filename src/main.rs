use crossterm::{
    cursor,
    event::{
        DisableFocusChange,
        EnableFocusChange,
        Event,
        KeyCode,
        KeyModifiers,
        poll,
        read,
    },
    ExecutableCommand,
    style::{
        Color,
        Print,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        Clear,
        ClearType,
        disable_raw_mode, 
        enable_raw_mode, 
        size,
    },
    QueueableCommand,
};
use rand::Rng;
use std::{
    io::{
        Result,
        stdout,
        Write
    },
    time::Duration,
};

mod game;
use crate::game::Game;
use crate::game::world::World;
use crate::game::world::Screen;
use crate::game::world::scenery::Scenery;
use crate::game::world::scenery::Road;
use crate::game::world::scenery::Clouds;
use crate::game::world::objects::Objects;
use crate::game::world::objects::Trex;
use crate::game::world::objects::TrexStatus;
use crate::game::world::objects::Cactuses;
use crate::game::world::Theme;
use crate::game::Scores;
use crate::game::GameStatus;
use crate::game::Utils;
use crate::game::world::scenery::LineStatus;
use crate::game::world::objects::TrexMoveDirection;

fn main() -> Result<()> {
    let mut game = Game {
        world: World {
            screen: Screen {
                width: size().unwrap().0,
                height: size().unwrap().1,
            },
            scenery: Scenery {
                road: Road {
                    next_top_stone_distance: 0,
                    next_bottom_stone_distance: 0,
                    next_grain_distance: 0,
                    top_line: Vec::new(),
                    bottom_line: Vec::new(),
                    ground: Vec::new(), 
                },
                clouds: Clouds {},
            },
            objects: Objects {
                trex: Trex {
                    pixels: [(0,0); 7],
                    height: 0,
                    max_height: 10,
                    status: TrexStatus::OnGround,
                    origin: (2, size().unwrap().1 - 3),
                },
                cactuses: Cactuses {
                    pixels: Vec::new(),
                    next_cactus_distance: 0,
                },
            },
            theme: Theme {
                background: Color::Blue,
                trex: Color::Green,
                trex_eye: Color::White,
                collided_trex: Color::Red,
                cloud: Color::Cyan,
                cactus: Color::Green,
            },
        },
        scores: Scores {
            highest: 0,
            current: 0,
        },
        status: GameStatus::Paused,
        utils: Utils {
            rng: rand::thread_rng(),
            stdout: stdout(),
        },
    };

    // change terminal settings
    enable_raw_mode()?;
    game.utils.stdout.execute(EnableFocusChange)?;

    fn initiate_world(game: &mut Game) {
        // TODO: the range's maximum should be equal to terminal width if
        // the width is larger than 2000
        game.world.scenery.road.next_top_stone_distance = game.utils.rng.gen_range(50..100);
        game.world.scenery.road.next_bottom_stone_distance = game.utils.rng.gen_range(50..100);
        game.world.scenery.road.next_grain_distance = game.utils.rng.gen_range(10..20);
        game.world.objects.cactuses.next_cactus_distance = game.utils.rng.gen_range(100..200);
        for _ in 0..game.world.screen.width {
            next_frame(game);
        }

        // initiate trex pixels
        game.world.objects.trex.pixels = [
            // legs
            (game.world.objects.trex.origin.0 + 2, game.world.objects.trex.origin.1 - 0 + 1),
            (game.world.objects.trex.origin.0 + 0, game.world.objects.trex.origin.1 - 0 + 1),
            // body
            (game.world.objects.trex.origin.0 + 0, game.world.objects.trex.origin.1 - 1 + 1),
            (game.world.objects.trex.origin.0 + 1, game.world.objects.trex.origin.1 - 1 + 1),
            (game.world.objects.trex.origin.0 + 2, game.world.objects.trex.origin.1 - 1 + 1),
            (game.world.objects.trex.origin.0 + 2, game.world.objects.trex.origin.1 - 2 + 1),
            // head
            (game.world.objects.trex.origin.0 + 3, game.world.objects.trex.origin.1 - 2 + 1),
        ];
    }

    // check events
    fn check_events(game: &mut Game) {
        for _ in 0..5 {
            if poll(Duration::from_millis(10)).unwrap() {
                match read().unwrap() {
                    Event::FocusLost => {game.status = GameStatus::Paused;},
                    Event::Key(event) => {
                        match (event.code, event.modifiers) {
                            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                                match game.status {
                                    GameStatus::Paused => game.status = GameStatus::Running,
                                    GameStatus::Running => game.status = GameStatus::Paused,
                                    _ => {},
                                }
                            },
                            (KeyCode::Char(' '), KeyModifiers::NONE) => {
                                if game.world.objects.trex.status == TrexStatus::OnGround {
                                    game.world.objects.trex.status = TrexStatus::Rising;
                                }
                            },
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                game.status = GameStatus::Closed;
                            }
                            _ => {},
                        }
                    },
                    Event::Resize(width, height) => println!("New size {}x{}", width, height),
                    _ => (),
                }
            }
        }
    }

    // draw world
    fn draw(game: &mut Game) -> Result<()>{
        game.utils.stdout.queue(cursor::Hide)?;
        game.utils.stdout.queue(Clear(ClearType::All))?;
        game.utils.stdout.queue(Clear(ClearType::Purge))?;

        // draw top line
        game.utils.stdout.queue(cursor::MoveTo(0, game.world.screen.height - 3))?;
        for line_status in &game.world.scenery.road.top_line {
            match line_status {
                LineStatus::StoneStart => game.utils.stdout.queue(Print("/")),
                LineStatus::StoneMiddle => game.utils.stdout.queue(Print("-")),
                LineStatus::StoneEnd => game.utils.stdout.queue(Print("\\")),
                LineStatus::Line => game.utils.stdout.queue(Print("_")),
            }?;
        }

        // draw bottom line
        game.utils.stdout.queue(cursor::MoveTo(0, game.world.screen.height - 1))?;
        for line_status in &game.world.scenery.road.bottom_line {
            match line_status {
                LineStatus::StoneStart => game.utils.stdout.queue(Print("/")),
                LineStatus::StoneMiddle => game.utils.stdout.queue(Print("-")),
                LineStatus::StoneEnd => game.utils.stdout.queue(Print("\\")),
                LineStatus::Line => game.utils.stdout.queue(Print("_")),
            }?;
        }

        // draw ground
        game.utils.stdout.queue(cursor::MoveTo(0, game.world.screen.height - 2))?;
        for is_grain in &game.world.scenery.road.ground {
            if *is_grain {
                game.utils.stdout.queue(Print("."))?;
            } else {
                game.utils.stdout.queue(cursor::MoveRight(1))?;
            }
        }

        // draw cactuses
        for pixel in game.world.objects.cactuses.pixels.iter() {
            game.utils.stdout.queue(cursor::MoveTo(pixel.0, pixel.1))?;
            game.utils.stdout.queue(Print("█"))?;
        }

        // draw trex
        game.utils.stdout.queue(SetForegroundColor(game.world.theme.trex))?;

        for (index, trex_pixel) in game.world.objects.trex.pixels.iter().enumerate() {
            game.utils.stdout.queue(cursor::MoveTo(trex_pixel.0, trex_pixel.1))?;
            match index {
                0..=1 => game.utils.stdout.queue(Print("▙")),
                2..=6 => game.utils.stdout.queue(Print("█")),
                _ => {
                    game.utils.stdout.queue(SetBackgroundColor(game.world.theme.trex))?;
                    game.utils.stdout.queue(SetForegroundColor(game.world.theme.trex_eye))?;
                    game.utils.stdout.queue(Print("O"))
                },
            }?;
        }

        game.utils.stdout.queue(ResetColor)?;

        game.utils.stdout.flush()?;
        Ok(())
    }

    // create next frame
    fn next_frame(game: &mut Game) {
        // generate top line
        match game.world.scenery.road.next_top_stone_distance {
            2 => game.world.scenery.road.top_line.push(LineStatus::StoneStart),
            1 => game.world.scenery.road.top_line.push(LineStatus::StoneMiddle),
            0 => {
                game.world.scenery.road.top_line.push(LineStatus::StoneEnd);
                game.world.scenery.road.next_top_stone_distance = game.utils.rng.gen_range(50..100);
            },
            _ => game.world.scenery.road.top_line.push(LineStatus::Line),
        }
        game.world.scenery.road.next_top_stone_distance -= 1;
        
        // generate bottom line
        match game.world.scenery.road.next_bottom_stone_distance {
            2 => game.world.scenery.road.bottom_line.push(LineStatus::StoneStart),
            1 => game.world.scenery.road.bottom_line.push(LineStatus::StoneMiddle),
            0 => {
                game.world.scenery.road.bottom_line.push(LineStatus::StoneEnd);
                game.world.scenery.road.next_bottom_stone_distance = game.utils.rng.gen_range(50..100);
            },
            _ => game.world.scenery.road.bottom_line.push(LineStatus::Line),
        }
        game.world.scenery.road.next_bottom_stone_distance -= 1;

        // generate ground
        if game.world.scenery.road.next_grain_distance == 0 {
            game.world.scenery.road.ground.push(true);
            game.world.scenery.road.next_grain_distance = game.utils.rng.gen_range(10..20);
        } else {
            game.world.scenery.road.ground.push(false);
        }
        game.world.scenery.road.next_grain_distance -= 1;
        
        // move cactuses
        for pixel in game.world.objects.cactuses.pixels.iter_mut() {
            pixel.0 -= 1;
        }

        // generate cactus
        if game.world.objects.cactuses.next_cactus_distance == 0 {
            let cactus_form: u8 = game.utils.rng.gen_range(1..=3);
            let mut cactus_pixels: Vec<(u16, u16)> = Vec::new();
            match cactus_form {
                1 => {
                    cactus_pixels = vec![
                        (game.world.screen.width + 1, game.world.screen.height - 2),
                        (game.world.screen.width + 1, game.world.screen.height - 3),
                        (game.world.screen.width + 1, game.world.screen.height - 4),
                        (game.world.screen.width + 1, game.world.screen.height - 5),
                        (game.world.screen.width + 2, game.world.screen.height - 3),
                        (game.world.screen.width + 3, game.world.screen.height - 3),
                        (game.world.screen.width + 3, game.world.screen.height - 4),
                    ];
                },
                2 => {
                    cactus_pixels = vec![
                        (game.world.screen.width + 4, game.world.screen.height - 2),
                        (game.world.screen.width + 4, game.world.screen.height - 3),
                        (game.world.screen.width + 4, game.world.screen.height - 4),
                        (game.world.screen.width + 4, game.world.screen.height - 5),
                        (game.world.screen.width + 3, game.world.screen.height - 3),
                        (game.world.screen.width + 2, game.world.screen.height - 3),
                        (game.world.screen.width + 2, game.world.screen.height - 4),
                    ];
                },
                3 => {
                    cactus_pixels = vec![
                        (game.world.screen.width + 4, game.world.screen.height - 2),
                        (game.world.screen.width + 4, game.world.screen.height - 3),
                        (game.world.screen.width + 4, game.world.screen.height - 4),
                        (game.world.screen.width + 4, game.world.screen.height - 5),
                        (game.world.screen.width + 3, game.world.screen.height - 3),
                        (game.world.screen.width + 2, game.world.screen.height - 3),
                        (game.world.screen.width + 2, game.world.screen.height - 4),
                        (game.world.screen.width + 5, game.world.screen.height - 3),
                        (game.world.screen.width + 6, game.world.screen.height - 3),
                        (game.world.screen.width + 6, game.world.screen.height - 4),
                    ];
                },
                _ => (),
            }
            game.world.objects.cactuses.pixels.extend(cactus_pixels);
            
            game.world.objects.cactuses.next_cactus_distance = game.utils.rng.gen_range(100..200);
        }
        game.world.objects.cactuses.next_cactus_distance -= 1;

        // move trex
        match game.world.objects.trex.status {
            TrexStatus::Rising => {
                if game.world.objects.trex.height < game.world.objects.trex.max_height {
                    move_trex(game, TrexMoveDirection::Up);
                } else {
                    game.world.objects.trex.status = TrexStatus::Falling;
                }
            },
            TrexStatus::Falling => {
                if game.world.objects.trex.height > 0 {
                    move_trex(game, TrexMoveDirection::Down);
                } else {
                    game.world.objects.trex.status = TrexStatus::OnGround;
                }
            },
            _ => {
            }
        }
    }
    
    fn move_trex(game: &mut Game, direction : TrexMoveDirection) {
        if direction  == TrexMoveDirection::Up {
            game.world.objects.trex.height += 1;
        } else {
            game.world.objects.trex.height -= 1;
        }

        for pixel in game.world.objects.trex.pixels.iter_mut() {
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

    fn delete_first_frame(game: &mut Game) {
        game.world.scenery.road.top_line.remove(0);
        game.world.scenery.road.bottom_line.remove(0);
        game.world.scenery.road.ground.remove(0);

        // delete cactus pixels that are out of screen
        game.world.objects.cactuses.pixels.retain(|pixel| pixel.0 > 0);
    }

    fn check_collision(game: &mut Game) {
        if game.world.objects.cactuses.pixels.iter().any(|pixel| game.world.objects.trex.pixels.contains(pixel)) {
            game.status = GameStatus::Over;
        }
    }

    fn check_game_status(game: &mut Game) {
        match game.status {
            GameStatus::Over => {
                game.world.theme.trex = Color::Red;
            },
            _ => {}
        }
    }

    fn control_flow(game: &mut Game) -> Result<()> {
        initiate_world(game);
        draw(game)?;
        loop {
            check_events(game);
            match game.status {
                GameStatus::Paused | GameStatus::Over => continue,
                GameStatus::Closed => {
                    game.utils.stdout.queue(cursor::Show)?;
                    game.utils.stdout.queue(Clear(ClearType::Purge))?;
                    game.utils.stdout.queue(Clear(ClearType::All))?;
                    game.utils.stdout.queue(cursor::MoveTo(0, 0))?;
                    game.utils.stdout.queue(DisableFocusChange)?;
                    disable_raw_mode()?;
                    return Ok(());
                },
                _ => {},
            };
            next_frame(game);
            delete_first_frame(game);
            check_collision(game);
            check_game_status(game);
            draw(game)?;
        }
    }

    control_flow(&mut game)?;
    Ok(())
}