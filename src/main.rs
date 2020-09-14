#[ignore(unused_parens)]

extern crate ggez;
extern crate rand;

use ggez::{graphics, event, conf, Context, ContextBuilder, GameResult};
use rand::Rng;

struct Object {
    position: [f32; 2],
    velocity: [f32; 2]     
}

struct Player {
    score: i32,
    paddle: Object
}

struct Game {
    context: &mut Context,
    ball: Object,
    p1: Player,
    p2: Player
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        return Game {
            context: context,
            ball: Object {
                position: [0.0, 0.0],
                velocity: [0.0, 0.0]
            },
            
            p1: Player {
                score: 0,
                paddle: Object {
                    position: [0.0, 0.0],
                    velocity: [0.0, 0.0]   
                }
            },
            
            p2: Player {
                score: 0,
                paddle: Object {
                    position: [0.0, 0.0],
                    velocity: [0.0, 0.0]   
                }
            }
        };
    }

    pub fn random(min: i32, max: i32) -> i32 {
        return rand::thread_rng().gen_range(min, max);
    }

    pub fn start(&mut self) {
        
    }

    fn create_ball(&mut self, right_side: bool) {
        let (x, y) = graphics::size(&self.context);
        let (mut horizontal, vertical) = (self::random(2, 4), self::random(1, 3));

        self.ball.position = [x / 2.0, y / 2.0];
        
        if (right_side) {
            horizontal = -horizontal;
        }

        self.ball.velocity = [horizontal, vertical];
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
          
        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);


        return graphics::present(context);
    }
}

fn main() {
    let mode = conf::WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: true,
        fullscreen_type: conf::FullscreenType::Desktop,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    }; 

    let setup = conf::WindowSetup {
        title: "Ping Pong".to_owned(),
        samples: conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true
    };

    let (mut context, mut event_loop) = ContextBuilder::new("Ping Pong", "ReturnedTrue")
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .expect("ggez couldn't create context");

    let mut my_game = Game::new(&context);

    match event::run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Success!"),
        Err(error) => println!("An error occurred: {}", error)
    }
}

// TODO: Fix finding random in self and add lifetime specifier