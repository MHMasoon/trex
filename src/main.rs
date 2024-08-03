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
use std::{
    io::{
        Result,
        stdout,
        Write
    },
    time::Duration,
};

mod game;
use crate::game::{
    Game,
    GameStatus,
    Scores,
    Utils,
    world::{
        objects::{Objects, TrexStatus, Cactuses},
        scenery::{Scenery, Road, Clouds, LineStatus},
        Screen,
        Theme,
        World,
    }    
};

fn main() -> Result<()> {
    let mut game = Game {
        world: World {
            screen: Screen {
                width: size().unwrap().0,
                height: size().unwrap().1,
            },
            scenery: Scenery {
                road: Road {
                    top_stone_distance: 0,
                    bottom_stone_distance: 0,
                    grain_distance: 0,
                    top_line: Vec::new(),
                    bottom_line: Vec::new(),
                    ground: Vec::new(), 
                },
                clouds: Clouds {},
            },
            objects: Objects {
                trex: Default::default(),
                cactuses: Cactuses {
                    pixels: Vec::new(),
                    cactus_distance: 0,
                },
            },
            theme: Theme {
                background: Color::Blue,
                trex: Color::Green,
                trex_eye: Color::White,
                collided_trex: Color::Red,
                cloud: Color::Cyan,
                cactus: Color::Green,
                message: Color::Cyan,
                message_background: Color::Black,
            },
        },
        scores: Scores {
            highest: 0,
            current: 0,
        },
        status: GameStatus::Beginning,
        utils: Utils {
            rng: rand::thread_rng(),
            stdout: stdout(),
        },
    };

    enable_raw_mode()?;
    game.utils.stdout.execute(EnableFocusChange)?;

    control_flow(&mut game)?;
    Ok(())
}

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
                        (KeyCode::Char(' '), KeyModifiers::NONE)
                        | (KeyCode::Up, KeyModifiers::NONE) => {
                            match game.status {
                                GameStatus::Beginning | GameStatus::Paused => {
                                    game.status = GameStatus::Running;
                                },
                                _ => {},
                            }

                            if game.world.objects.trex.status == TrexStatus::OnGround {
                                game.world.objects.trex.status = TrexStatus::Rising;
                            }
                        },
                        (KeyCode::Char('c'), KeyModifiers::CONTROL)
                        | (KeyCode::Char('q'), KeyModifiers::NONE) => {
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

    // print message
    game.utils.stdout.queue(SetBackgroundColor(game.world.theme.message_background));
    game.utils.stdout.queue(SetForegroundColor(game.world.theme.message));
    match game.status {
        GameStatus::Beginning | GameStatus::Paused | GameStatus::Over => {
            let message = game.status.message();
            game.utils.stdout.queue(
                cursor::MoveTo(
                    game.world.screen.width / 2 - message.len() as u16 / 2,
                    game.world.screen.height / 2
                )
            )?;
            game.utils.stdout.queue(Print(game.status.message()))?;
        },
        _ => {},
    }

    game.utils.stdout.queue(ResetColor)?;

    game.utils.stdout.flush()?;
    Ok(())
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
    game.world.initiate();
    draw(game)?;
    loop {
        check_events(game);
        match game.status {
            GameStatus::Paused | GameStatus::Over | GameStatus::Beginning => continue,
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
        game.world.next_frame();
        delete_first_frame(game);
        check_collision(game);
        check_game_status(game);
        draw(game)?;
    }
}
