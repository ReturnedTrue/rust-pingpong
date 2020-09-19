#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate ggez;
extern crate rand;

use ggez::{nalgebra::Point2, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, MeshBuilder, DrawParam, DrawMode, Text, TextFragment, Font};
use ggez::event::{self, KeyCode, KeyMods, EventsLoop};
use ggez::conf::{self, WindowMode, WindowSetup};
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
    WINDOW_SIZE: Point2<f32>,
    PADDLE_SIZE: Point2<f32>,
    LINE_WIDTH: f32,
    font: Font,
    ball: Object,
    p1: Player,
    p2: Player
}

impl Game {
    pub fn new(context: &Context) -> Game {
        let (x, y) = graphics::size(context);
        let PADDLE_SIZE = Point2::new(x * 0.025, y * 0.1);
        let LINE_WIDTH = x * 0.01; 

        return Game {
            WINDOW_SIZE: Point2::new(x, y),
            PADDLE_SIZE: PADDLE_SIZE,
            LINE_WIDTH: LINE_WIDTH,

            font: Font::default(),

            ball: Object {
                position: Point2::new(0.0, 0.0),
                velocity: Point2::new(0.0, 0.0)
            },
            
            p1: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(LINE_WIDTH, (y / 2.0) - PADDLE_SIZE.y),
                    velocity: Point2::new(0.0, 0.0)  
                }
            },
            
            p2: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(x - ((PADDLE_SIZE.x * 2.0) + (LINE_WIDTH / 2.0)), (y / 2.0) - PADDLE_SIZE.y),
                    velocity: Point2::new(0.0, 0.0)   
                }
            }
        };
    }

    pub fn start(&mut self, context: &mut Context, events_loop: &mut EventsLoop) {
        if (self.random(1, 3) == 1) {
            self.create_ball(false, context);
        } else {
            self.create_ball(true, context);
        }
        
        self.font = Font::new(context, "/Sansation_Regular.ttf")
            .expect("Loaded font");

        match event::run(context, events_loop, self) {
            Ok(_) => println!("Success!"),
            Err(error) => println!("An error occurred: {}", error)
        }
    }

    fn create_ball(&mut self, right_side: bool, context: &mut Context) {
        self.ball.position = Point2::new(self.WINDOW_SIZE.x / 2.0, self.WINDOW_SIZE.y / 2.0);

        let (mut horizontal, mut vertical) = (self.random(3.0, 6.0), self.random(2.0, 4.0));

        if (right_side) {
            horizontal = -horizontal;
        }

        if (self.random(1, 3) == 1) {
            vertical = -vertical;
        }

        self.ball.velocity = Point2::new(horizontal, vertical);
    }

    
    fn random<T: rand::distributions::uniform::SampleUniform>(&mut self, min: T, max: T) -> T {
        return rand::thread_rng().gen_range(min, max);
    }

    fn add_points(&mut self, point1: Point2<f32>, point2: Point2<f32>) -> Point2<f32> {
        return Point2::new(point1.x + point2.x, point1.y + point2.y);
    }

    fn draw_text(&mut self, context: &mut Context, text: std::string::String, position: Point2<f32>) -> () {
        graphics::draw(context, &Text::new(TextFragment::new(text.to_string()).font(self.font)), DrawParam::default().dest(position));
    }
}


impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
          
        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let DEFAULT_PARAMS = DrawParam::default();
        let FILL_MODE = DrawMode::fill();

        let LINE_COLOR = graphics::WHITE;
        let PADDLE_COLOR = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

        let BALL_RADIUS = self.WINDOW_SIZE.x * 0.025;

        let MIDDLE_X = (self.WINDOW_SIZE.x / 2.0) - self.LINE_WIDTH;
        let END_X = self.WINDOW_SIZE.x - self.LINE_WIDTH * 2.0;

        let backgroundMesh = MeshBuilder::new()
            .line(&[ Point2::new(MIDDLE_X, 0.0), Point2::new(MIDDLE_X, self.WINDOW_SIZE.y) ], self.LINE_WIDTH, LINE_COLOR)? // Middle
            .line(&[ Point2::new(0.0, 0.0), Point2::new(0.0, self.WINDOW_SIZE.y) ], self.LINE_WIDTH, LINE_COLOR)? // Left
            .line(&[ Point2::new(END_X, 0.0), Point2::new(END_X, self.WINDOW_SIZE.y) ], self.LINE_WIDTH, LINE_COLOR)? // Right
            .circle(FILL_MODE, Point2::new(self.WINDOW_SIZE.x / 2.0 - self.LINE_WIDTH, self.WINDOW_SIZE.y / 2.0), self.WINDOW_SIZE.x * 0.05, 0.05, LINE_COLOR) // Middle circle
            .build(context)?;

        graphics::draw(context, &backgroundMesh, DEFAULT_PARAMS)?;

        let (paddle1new, paddle2new) = (self.p1.paddle.position.y + self.p1.paddle.velocity.y, self.p2.paddle.position.y + self.p2.paddle.velocity.y);

        if (paddle1new > 0.0 && self.WINDOW_SIZE.y > (paddle1new + self.PADDLE_SIZE.y)) {
            self.p1.paddle.position = self.add_points(self.p1.paddle.position, self.p1.paddle.velocity);
        }

        if (paddle2new > 0.0 && self.WINDOW_SIZE.y > (paddle2new + self.PADDLE_SIZE.y)) {
            self.p2.paddle.position = self.add_points(self.p2.paddle.position, self.p2.paddle.velocity);
        }

        self.ball.position = self.add_points(self.ball.position, self.ball.velocity);

        let mainMesh = MeshBuilder::new()
            .circle(FILL_MODE, self.ball.position, BALL_RADIUS, 0.05, graphics::Color::new(255.0, 0.0, 0.0, 1.0)) // Ball
            .rectangle(FILL_MODE, graphics::Rect::new(self.p1.paddle.position.x, self.p1.paddle.position.y, self.PADDLE_SIZE.x, self.PADDLE_SIZE.y), PADDLE_COLOR) // p1 paddle
            .rectangle(FILL_MODE, graphics::Rect::new(self.p2.paddle.position.x, self.p2.paddle.position.y, self.PADDLE_SIZE.x, self.PADDLE_SIZE.y), PADDLE_COLOR) // p2 paddle
            .build(context)?;

        graphics::draw(context, &mainMesh, DEFAULT_PARAMS)?;

        // Top/bottom wall collision
        if (BALL_RADIUS >= self.ball.position.y || (self.ball.position.y + BALL_RADIUS) >= self.WINDOW_SIZE.y) {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // p1 paddle collision
        if (self.p1.paddle.position.x >= (self.ball.position.x - BALL_RADIUS - self.PADDLE_SIZE.x)) {
            if (self.ball.position.y >= self.p1.paddle.position.y && (self.p1.paddle.position.y + self.PADDLE_SIZE.y) >= self.ball.position.y) {
                self.ball.velocity.x = -(self.ball.velocity.x * 1.1);
                // self.ball.velocity.y = -self.ball.velocity.y;

            } else {
                self.p2.score += 1;
                self.create_ball(false, context);
            }
        }

        // p2 paddle collision
        if ((self.ball.position.x + BALL_RADIUS) >= self.p2.paddle.position.x) {
            if (self.ball.position.y >= self.p2.paddle.position.y && (self.p2.paddle.position.y + self.PADDLE_SIZE.y) >= self.ball.position.y) {
                self.ball.velocity.x = -(self.ball.velocity.x * 1.1);
                // self.ball.velocity.y = -self.ball.velocity.y;

            } else {
                self.p1.score += 1;
                self.create_ball(true, context);
            }
        }

        self.draw_text(context, format!("Score: {}", self.p1.score), Point2::new(self.WINDOW_SIZE.x * 0.2, self.WINDOW_SIZE.y * 0.1));
        self.draw_text(context, format!("Score: {}", self.p2.score), Point2::new(self.WINDOW_SIZE.x * 0.7, self.WINDOW_SIZE.y * 0.1));

        return graphics::present(context);
    }

    fn key_down_event(&mut self, context: &mut Context, keycode: KeyCode, mods: KeyMods, repeat: bool) {
        if (keycode == KeyCode::W) {
            self.p1.paddle.velocity.y = -8.0;

        } else if (keycode == KeyCode::S) {
            self.p1.paddle.velocity.y = 8.0;

        } else if (keycode == KeyCode::Up) {
            self.p2.paddle.velocity.y = -8.0;

        } else if (keycode == KeyCode::Down) {
            self.p2.paddle.velocity.y = 8.0;
        }
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, mods: KeyMods) {
        if (keycode == KeyCode::W || keycode == KeyCode::S) {
            self.p1.paddle.velocity.y = 0.0;

        } else if (keycode == KeyCode::Up || keycode == KeyCode::Down) {
            self.p2.paddle.velocity.y = 0.0;
        }
    }
}

fn main() {
    let mode = WindowMode {
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

    let setup = WindowSetup {
        title: "Ping Pong".to_owned(),
        samples: conf::NumSamples::Zero,
        vsync: true,
        icon: "/icon.png".to_owned(),
        srgb: true
    };

    let (mut context, mut events_loop) = ContextBuilder::new("Ping Pong", "ReturnedTrue")
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .expect("ggez couldn't create context");

    Game::new(&context).start(&mut context, &mut events_loop);
}