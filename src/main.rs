#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate ggez;
extern crate rand;

use ggez::{graphics, event, conf, nalgebra::Point2, Context, ContextBuilder, GameResult};
use rand::Rng;

struct Object {
    position: Point2<f32>,
    velocity: Point2<f32>
}

struct Player {
    score: i32,
    paddle: Object
}

struct Game {
    window_size: Point2<f32>,
    ball: Object,
    p1: Player,
    p2: Player
}

impl Game {
    pub fn new(context: &Context) -> Game {
        let (x, y) = graphics::size(context);

        return Game {
            window_size: Point2::new(x, y),

            ball: Object {
                position: Point2::new(0.0, 0.0),
                velocity: Point2::new(0.0, 0.0)
            },
            
            p1: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(0.0, 0.0),
                    velocity: Point2::new(0.0, 0.0)  
                }
            },
            
            p2: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(0.0, 0.0),
                    velocity: Point2::new(0.0, 0.0)   
                }
            }
        };
    }

    pub fn start(&mut self, context: &mut Context, events_loop: &mut event::EventsLoop) {
        match event::run(context, events_loop, self) {
            Ok(_) => println!("Success!"),
            Err(error) => println!("An error occurred: {}", error)
        }
        
        if (self.random(1, 2) == 1) {
            self.create_ball(false, context);
        } else {
            self.create_ball(true, context);
        }


    }

    fn create_ball(&mut self, right_side: bool, context: &mut Context) {
        let (mut horizontal, vertical) = (self.random(2.0, 4.0), self.random(1.0, 3.0));
        
        self.ball.position = Point2::new(self.window_size.x / 2.0, self.window_size.y / 2.0);
        
        if (right_side) {
            horizontal = -horizontal;
        }

        self.ball.velocity = Point2::new(horizontal, vertical);
    }

    
    fn random<T: rand::distributions::uniform::SampleUniform>(&mut self, min: T, max: T) -> T {
        return rand::thread_rng().gen_range(min, max);
    }
}


impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
          
        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let LINE_WIDTH = self.window_size.x / 100.0; // 1% of screen
        let LINE_COLOR = graphics::WHITE;

        let MIDDLE_X = (self.window_size.x / 2.0) - LINE_WIDTH;
        let END_X = self.window_size.x - (LINE_WIDTH * 2.0);

        let backgroundMesh = graphics::MeshBuilder::new()
            .line(&[ Point2::new(MIDDLE_X, 0.0), Point2::new(MIDDLE_X, self.window_size.y) ], LINE_WIDTH, LINE_COLOR)? // Middle
            .line(&[ Point2::new(0.0, 0.0), Point2::new(0.0, self.window_size.y) ], LINE_WIDTH, LINE_COLOR)? // Left
            .line(&[ Point2::new(END_X, 0.0), Point2::new(END_X, self.window_size.y) ], LINE_WIDTH, LINE_COLOR)? // Right
            .circle(graphics::DrawMode::fill(), Point2::new(self.window_size.x / 2.0 - LINE_WIDTH, self.window_size.y / 2.0), self.window_size.x * 0.05, 0.05, LINE_COLOR)
            .build(context)?;

        graphics::draw(
            context,
            &backgroundMesh,
            graphics::DrawParam::default()
        )?;


        return graphics::present(context);
    }
}

fn main() {
    let mode = conf::WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: true,
        fullscreen_type: conf::FullscreenType::Windowed,
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

    let (mut context, mut events_loop) = ContextBuilder::new("Ping Pong", "ReturnedTrue")
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .expect("ggez couldn't create context");

    let mut my_game = Game::new(&context);

    my_game.start(&mut context, &mut events_loop);
}

// TODO: Move onto paddle / text drawing and key handling