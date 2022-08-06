mod player;
mod globalconstants;
mod obstacle;
use bracket_lib::prelude::*;
use crate::player::Player;
use crate::obstacle::Obstacle;

const FRAME_DURATION: f32 = 20.0;
const OBSTACLE_X_INTERVAL: f32 = 30.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State{
    mode: GameMode,
    player: Player,
    frame_time: f32,
    score: i32,
    obstacles: Vec<Obstacle>,
}

impl State {
    fn new() -> State {
        Self {
            mode: GameMode::Menu,
            player: Player::new(3, 3),
            frame_time: 0.0,
            score: 0,
            obstacles: Self::build_initial_obstacles(),
        }
    }
    fn build_initial_obstacles() -> Vec<Obstacle> {
        vec![
            Obstacle::new(60.0, 0),
            Obstacle::new(90.0, 1),
            Obstacle::new(120.0, 2),
            Obstacle::new(150.0, 3),
            Obstacle::new(180.0, 4),
            Obstacle::new(210.0, 5),
        ]
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.update(ctx);
            for obstacle in &mut self.obstacles {
                obstacle.update();
                if obstacle.check_collision(&self.player) {
                    self.mode = GameMode::End;
                }
            }
            if self.player.y >= globalconstants::SCREEN_HEIGHT {
                self.mode = GameMode::End;
            }
        }
        self.check_flap_input(ctx);
        self.player.render(ctx);
        self.obstacles_render_and_maintain_list(ctx);
        ctx.print(1, 1, format!("Score: {}", self.score));
    }
    fn check_flap_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }
    }
    fn obstacles_render_and_maintain_list(&mut self, ctx: &mut BTerm) {
        for obstacle in &mut self.obstacles {
            obstacle.render(ctx, self.player.x);
            if !obstacle.score_accounted_for && (obstacle.x as i32) < self.player.x {
                obstacle.score_accounted_for = true;
                self.score += 1;
            }
        }
        let first_obstacle = &self.obstacles[0];
        if first_obstacle.x < -20.0 {
            self.obstacles.remove(0);
            let last_x = self.obstacles[self.obstacles.len() - 1].x;
            self.obstacles.push(Obstacle::new(last_x + OBSTACLE_X_INTERVAL, self.score));
        }
    }
    fn print_options_screen(&mut self, ctx: &mut BTerm, heading: &str) {
        ctx.print_centered(5, heading);
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn restart(&mut self) {
        self.player.reset();
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacles = Self::build_initial_obstacles();
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        match self.mode {
            GameMode::Menu => self.print_options_screen(ctx, "Welcome to Flappy Bird"),
            GameMode::End => self.print_options_screen(ctx, "Git good scrub"),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(globalconstants::SCREEN_WIDTH, globalconstants::SCREEN_HEIGHT)?
        .with_title("Flappy Joe")
        .build()?;
    main_loop(ctx, State::new())
}
