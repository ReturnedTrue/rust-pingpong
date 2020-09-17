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
    paddle_size: Point2<f32>,
    line_width: f32,
    ball: Object,
    p1: Player,
    p2: Player
}

impl Game {
    pub fn new(context: &Context) -> Game {
        let (x, y) = graphics::size(context);
        let paddle_size = Point2::new(x * 0.025, y * 0.1);
        let line_width = x * 0.01; 

        return Game {
            window_size: Point2::new(x, y),
            paddle_size: paddle_size,
            line_width: line_width,


            ball: Object {
                position: Point2::new(0.0, 0.0),
                velocity: Point2::new(0.0, 0.0)
            },
            
            p1: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(line_width, (y / 2.0) - paddle_size.y),
                    velocity: Point2::new(0.0, 0.0)  
                }
            },
            
            p2: Player {
                score: 0,
                paddle: Object {
                    position: Point2::new(x - ((paddle_size.x * 2.0) + (line_width / 2.0)), (y / 2.0) - paddle_size.y),
                    velocity: Point2::new(0.0, 0.0)   
                }
            }
        };
    }

    pub fn start(&mut self, context: &mut Context, events_loop: &mut event::EventsLoop) {
        if (self.random(1, 3) == 1) {
            self.create_ball(false, context);
        } else {
            self.create_ball(true, context);
        }

        match event::run(context, events_loop, self) {
            Ok(_) => println!("Success!"),
            Err(error) => println!("An error occurred: {}", error)
        }
    }

    fn create_ball(&mut self, right_side: bool, context: &mut Context) {
        let (mut horizontal, mut vertical) = (self.random(3.0, 6.0), self.random(2.0, 4.0));
        
        self.ball.position = Point2::new(self.window_size.x / 2.0, self.window_size.y / 2.0);
        
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
}


impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
          
        return Ok(());
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let LINE_COLOR = graphics::WHITE;
        let PADDLE_COLOR = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

        let BALL_RADIUS = self.window_size.x * 0.025;

        let MIDDLE_X = (self.window_size.x / 2.0) - self.line_width;
        let END_X = self.window_size.x - self.line_width * 2.0;

        let backgroundMesh = graphics::MeshBuilder::new()
            .line(&[ Point2::new(MIDDLE_X, 0.0), Point2::new(MIDDLE_X, self.window_size.y) ], self.line_width, LINE_COLOR)? // Middle
            .line(&[ Point2::new(0.0, 0.0), Point2::new(0.0, self.window_size.y) ], self.line_width, LINE_COLOR)? // Left
            .line(&[ Point2::new(END_X, 0.0), Point2::new(END_X, self.window_size.y) ], self.line_width, LINE_COLOR)? // Right
            .circle(graphics::DrawMode::fill(), Point2::new(self.window_size.x / 2.0 - self.line_width, self.window_size.y / 2.0), self.window_size.x * 0.05, 0.05, LINE_COLOR) // Middle circle
            .build(context)?;

        graphics::draw(context, &backgroundMesh, graphics::DrawParam::default())?;

        let (paddle1new, paddle2new) = (self.p1.paddle.position.y + self.p1.paddle.velocity.y, self.p2.paddle.position.y + self.p2.paddle.velocity.y);

        if (paddle1new > 0.0 && self.window_size.y > (paddle1new + self.paddle_size.y)) {
            self.p1.paddle.position = self.add_points(self.p1.paddle.position, self.p1.paddle.velocity);
        }

        if (paddle2new > 0.0 && self.window_size.y > (paddle2new + self.paddle_size.y)) {
            self.p2.paddle.position = self.add_points(self.p2.paddle.position, self.p2.paddle.velocity);
        }

        self.ball.position = self.add_points(self.ball.position, self.ball.velocity);

        let mainMesh = graphics::MeshBuilder::new()
            .circle(graphics::DrawMode::fill(), self.ball.position, BALL_RADIUS, 0.05, graphics::Color::new(255.0, 0.0, 0.0, 1.0)) // Ball
            .rectangle(graphics::DrawMode::fill(), graphics::Rect::new(self.p1.paddle.position.x, self.p1.paddle.position.y, self.paddle_size.x, self.paddle_size.y), PADDLE_COLOR) // p1 paddle
            .rectangle(graphics::DrawMode::fill(), graphics::Rect::new(self.p2.paddle.position.x, self.p2.paddle.position.y, self.paddle_size.x, self.paddle_size.y), PADDLE_COLOR) // p2 paddle
            .build(context)?;

        graphics::draw(context, &mainMesh, graphics::DrawParam::default())?;

        // Top/bottom wall collision
        if (BALL_RADIUS >= self.ball.position.y || (self.ball.position.y + BALL_RADIUS) >= self.window_size.y) {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // p1 paddle collision
        if (self.p1.paddle.position.x >= (self.ball.position.x - BALL_RADIUS - self.paddle_size.x)) {
            if (self.ball.position.y >= self.p1.paddle.position.y && (self.p1.paddle.position.y + self.paddle_size.y) >= self.ball.position.y) {
                self.ball.velocity.x = -(self.ball.velocity.x * 1.1);
                // self.ball.velocity.y = -self.ball.velocity.y;

            } else {
                self.p2.score += 1;
                self.create_ball(false, context);
            }
        }

        // p2 paddle collision
        if ((self.ball.position.x + BALL_RADIUS) >= self.p2.paddle.position.x) {
            if (self.ball.position.y >= self.p2.paddle.position.y && (self.p2.paddle.position.y + self.paddle_size.y) >= self.ball.position.y) {
                self.ball.velocity.x = -(self.ball.velocity.x * 1.1);
                // self.ball.velocity.y = -self.ball.velocity.y;

            } else {
                self.p1.score += 1;
                self.create_ball(true, context);
            }
        }

        let score1 = graphics::Text::new(format!("Score: {}", self.p1.score));
        let score2 = graphics::Text::new(format!("Score: {}", self.p2.score));

        graphics::draw(context, &score1, graphics::DrawParam::default().dest(Point2::new(self.window_size.x * 0.1, self.window_size.y * 0.1)))?;
        graphics::draw(context, &score2, graphics::DrawParam::default().dest(Point2::new(self.window_size.x * 0.8, self.window_size.y * 0.1)))?;

        return graphics::present(context);
    }

    fn key_down_event(&mut self, context: &mut Context, keycode: event::KeyCode, mods: event::KeyMods, repeat: bool) {
        if (keycode == event::KeyCode::W) {
            self.p1.paddle.velocity.y = -8.0;

        } else if (keycode == event::KeyCode::S) {
            self.p1.paddle.velocity.y = 8.0;

        } else if (keycode == event::KeyCode::Up) {
            self.p2.paddle.velocity.y = -8.0;

        } else if (keycode == event::KeyCode::Down) {
            self.p2.paddle.velocity.y = 8.0;
        }
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: event::KeyCode, mods: event::KeyMods) {
        if (keycode == event::KeyCode::W || keycode == event::KeyCode::S) {
            self.p1.paddle.velocity.y = 0.0;

        } else if (keycode == event::KeyCode::Up || keycode == event::KeyCode::Down) {
            self.p2.paddle.velocity.y = 0.0;
        }
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