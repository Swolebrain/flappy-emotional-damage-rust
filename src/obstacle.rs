use bracket_lib::prelude::*;
use crate::globalconstants::*;
use crate::Player;

pub const PIPE_WIDTH: i32 = 5;
pub const PIPE_SPEED: f32 = 2.0;

pub struct Obstacle {
    pub x: f32,
    gap_y: i32, // the centerpoint of the gap
    size: i32, // size of the gap
    pub score_accounted_for: bool,
}

impl Obstacle {
    pub fn new(x: f32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: x,
            gap_y: random.range(10, 40),
            size: i32::max(6, 20 - score),
            score_accounted_for: false,
        }
    }

    pub fn update(&mut self) {
        self.x -= PIPE_SPEED;
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let self_x = self.x as i32;
        let top_pipe_height = self.top_pipe_height();
        // render top pipe
        for y in 0..top_pipe_height {
            for x in self_x..(self_x + PIPE_WIDTH){
                ctx.set(
                    x,
                    y,
                    RED,
                    BLACK,
                    to_cp437('|')
                );
            }
        }
        let top_of_bottom_pipe = self.top_of_bottom_pipe();
        // render bottom pipe
        for y in top_of_bottom_pipe..SCREEN_HEIGHT {
            for x in self_x..(self_x + PIPE_WIDTH){
                ctx.set(
                    x,
                    y,
                    RED,
                    BLACK,
                    to_cp437('|')
                );
            }
        }
    }

    pub fn check_collision(&mut self, player: &Player) -> bool {
        let self_x = self.x as i32;
        let bottom_of_top_pipe = self.top_pipe_height();
        let top_of_bottom_pipe = self.top_of_bottom_pipe();
        let x_match = player.x >= self_x && player.x < (self_x + PIPE_WIDTH);
        let y_match = player.y < bottom_of_top_pipe || player.y >= top_of_bottom_pipe;
        x_match && y_match
    }

    fn top_pipe_height(&mut self) -> i32 {
        self.gap_y - self.size / 2
    }

    fn top_of_bottom_pipe(&mut self) -> i32 {
        self.gap_y + self.size / 2
    }
}

