use crossterm::terminal::size;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::style::Print;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crossterm::QueueableCommand;
use crossterm::cursor;

#[derive(Default)]
struct World {
    length: u16,
    height: u16,
    next_stone_distance: u16,
    rng: ThreadRng,
    main_line: Vec<LineStatus>,
    top_ground: Vec<bool>,
    bottom_ground: Vec<bool>,
}

enum LineStatus {
    Line,
    StoneStart,
    StoneMiddle,
    StoneEnd,
}

fn main() {
    // create world
    let mut world = World {
        length: size().unwrap().0,
        height: size().unwrap().1,
        rng: rand::thread_rng(),
        ..Default::default()
    };
    
    fn initiate_world(world: &mut World) {
        // TODO: the range's maximum should be equal to terminal width if
        // the width is larger than 2000
        world.next_stone_distance = world.rng.gen_range(30..70);
        for _ in 0..world.length {
            next_frame(world);
        }
    }
    // draw world
    fn draw(world: &World) {
        let mut stdout = stdout();
        stdout.queue(Clear(ClearType::All));
        stdout.queue(cursor::MoveTo(0, world.height - 4));
        for line_status in &world.main_line {
            match line_status {
                LineStatus::StoneStart => stdout.queue(Print("/")),
                LineStatus::StoneMiddle => stdout.queue(Print("-")),
                LineStatus::StoneEnd => stdout.queue(Print("\\")),
                LineStatus::Line => stdout.queue(Print("_")),
            };
        }
        stdout.flush();
    }

    // create next frame
    fn next_frame(world: &mut World) {
        // generate main line
        match world.next_stone_distance {
            2 => world.main_line.push(LineStatus::StoneStart),
            1 => world.main_line.push(LineStatus::StoneMiddle),
            0 => {
                world.main_line.push(LineStatus::StoneEnd);
                world.next_stone_distance = world.rng.gen_range(30..70);
            },
            _ => world.main_line.push(LineStatus::Line),
        }
        world.next_stone_distance -= 1;
    }
    
    fn delete_first_frame(world: &mut World) {
        world.main_line.remove(0);
    }

    initiate_world(&mut world);
    draw(&world);
    for i in 1..10000 {
        next_frame(&mut world);
        delete_first_frame(&mut world);
        draw(&world);
        sleep(Duration::from_millis(50));
    }
}