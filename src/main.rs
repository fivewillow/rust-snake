use ::rand::prelude::*;
use macroquad::prelude::*;

const SQUARE: i32 = 10;
const WINDOW_WIDTH: i32 = 80 * SQUARE;
const WINDOW_HEIGHT: i32 = 62 * SQUARE;
const MAP_WIDTH: i32 = 80 * SQUARE;
const MAP_HEIGHT: i32 = 60 * SQUARE;

type Point = (i32, i32);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Snake {
    head: Point,
    body: Vec<Point>,
    direction: Direction,
    count: i32,
}

trait Actions {
    fn move_to(&mut self);
    fn has_eaten_food(&mut self, food: &Point) -> bool;
    fn is_collision(&self, min_point: Point, max_point: Point) -> bool;
}

impl Snake {
    fn new(point: Point) -> Snake {
        let body = Vec::new();
        Snake {
            head: point,
            body,
            direction: Direction::Left,
            count: 0,
        }
    }
}

impl Actions for Snake {
    fn move_to(&mut self) {
        let step = get_step(&self.direction);
        //println!("step : {:?}", step);
        self.body.insert(0, (self.head.0, self.head.1));
        self.head = (
            self.head.0 + (step.0 * SQUARE),
            self.head.1 + (step.1 * SQUARE),
        );
    }

    fn has_eaten_food(&mut self, food: &Point) -> bool {
        if self.head == *food {
            self.count += 1;
            return true;
        } else {
            self.body.truncate(self.count as usize);
        }
        false
    }

    fn is_collision(&self, min_point: Point, max_point: Point) -> bool {
        if self.head.0 <= min_point.0 || (self.head.0 >= max_point.0) {
            return true;
        }

        if self.head.1 <= min_point.1 || self.head.1 >= max_point.1 {
            return true;
        }

        for i in self.body.iter() {
            if self.head == *i {
                return true;
            }
        }

        false
    }
}

fn get_step(dir: &Direction) -> Point {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Rust Snake"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}
fn handle_input(snake: &mut Snake) {
    if is_key_down(KeyCode::Up) && snake.direction != Direction::Down {
        snake.direction = Direction::Up;
    } else if is_key_down(KeyCode::Down) && snake.direction != Direction::Up {
        snake.direction = Direction::Down;
    } else if is_key_down(KeyCode::Left) && snake.direction != Direction::Right {
        snake.direction = Direction::Left;
    } else if is_key_down(KeyCode::Right) && snake.direction != Direction::Left {
        snake.direction = Direction::Right;
    }
}

fn rander(snake: &mut Snake, food: &Point) {
    clear_background(LIGHTGRAY);

    // draw map
    draw_rectangle(
        SQUARE as f32,
        (WINDOW_HEIGHT - MAP_HEIGHT + SQUARE) as f32,
        MAP_WIDTH as f32 - (SQUARE as f32 * 2.),
        MAP_HEIGHT as f32 - (SQUARE as f32 * 2.),
        BLACK,
    );

    // draw food
    draw_rectangle(
        food.0 as f32,
        food.1 as f32,
        SQUARE as f32,
        SQUARE as f32,
        GOLD,
    );

    // draw snake head
    draw_rectangle(
        snake.head.0 as f32,
        snake.head.1 as f32,
        SQUARE as f32,
        SQUARE as f32,
        GREEN,
    );

    // draw snake body
    for i in snake.body.iter() {
        draw_rectangle(
            i.0 as f32,
            i.1 as f32,
            SQUARE as f32,
            SQUARE as f32,
            DARKGREEN,
        );
    }

    // draw score
    let msg = format!("Score: {}", (snake.count * 10));
    let font_size = 25;

    draw_text(msg.as_str(), 15., 25., font_size as f32, BLACK);
}

fn gen_food(min: Point, max: Point) -> Point {
    let mut rng = thread_rng();
    let min_x = min.0 / SQUARE;
    let max_x = max.0 / SQUARE;
    let min_y = (min.1 / SQUARE) + 1;
    let max_y = (max.1 / SQUARE) - 1;

    (
        rng.gen_range(min_x..max_x) * SQUARE,
        rng.gen_range(min_y..max_y) * SQUARE,
    )
}

fn rander_error(score: i32) {
    clear_background(RED);

    draw_rectangle(
        SQUARE as f32,
        (WINDOW_HEIGHT - MAP_HEIGHT + SQUARE) as f32,
        MAP_WIDTH as f32 - (SQUARE as f32 * 2.),
        MAP_HEIGHT as f32 - (SQUARE as f32 * 2.),
        BLACK,
    );

    let msg = "GAME OVER.\nPress [Enter] to play again.";
    let font_size = 30;
    let msg_size = measure_text(msg, None, font_size, 1.);
    let msg_x = MAP_WIDTH as f32 / 2. - msg_size.width / 2.;
    let msg_y = MAP_HEIGHT as f32 / 2. - msg_size.height / 2.;

    draw_text(msg, msg_x, msg_y, font_size as f32, RED);

    // draw score
    let msg = format!("Score: {}", score);
    let font_size = 25;

    draw_text(msg.as_str(), 15., 25., font_size as f32, BLACK);
}

fn increase_speed(snake: &Snake, speed: &mut f64) {
    if snake.count > 100 {
        *speed = 0.03;
    } else if snake.count > 60 {
        *speed = 0.05;
    } else if snake.count > 30 {
        *speed = 0.075;
    } else if snake.count > 10 {
        *speed = 0.1;
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut speed = 0.15;
    let mut updated_time = get_time();
    let mut snake = Snake::new((400, 300));

    let mut is_gameover = false;
    let min_point = (SQUARE, (WINDOW_HEIGHT - MAP_HEIGHT));
    let max_point = (MAP_WIDTH - SQUARE, WINDOW_HEIGHT - SQUARE);
    let mut food: Point = gen_food(min_point, max_point);

    loop {
        if is_gameover {
            if is_key_down(KeyCode::Enter) {
                snake = Snake::new((400, 300));
                food = gen_food(min_point, max_point);
                is_gameover = false;
                speed = 0.15;
            }

            rander_error(snake.count * 10);
        } else {
            if get_time() - updated_time > speed {
                updated_time = get_time();
                handle_input(&mut snake);
                snake.move_to();
                if snake.has_eaten_food(&food) {
                    food = gen_food(min_point, max_point);
                }

                is_gameover = snake.is_collision(min_point, max_point);
                increase_speed(&snake, &mut speed);
            }
            rander(&mut snake, &food);
        }

        next_frame().await
    }
}
