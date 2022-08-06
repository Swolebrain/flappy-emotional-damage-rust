use bracket_lib::prelude::*;
use crate::globalconstants::*;

const JUMP_SPEED: f32 = 3.5;
const FALL_SPEED_INCREMENT: f32 = 0.35;
const MAX_FALL_SPEED: f32 = 3.5;
const FLAP_TICKS: i8 = 8;

// cannot jump again while Flapped
enum FlapState {
    Idle,
    Flapping,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    velocity: f32,
    flap_state: FlapState,
    tick_counter: i8,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player{
            x,
            y,
            velocity: 0.0,
            flap_state: FlapState::Idle,
            tick_counter: 0
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    pub fn update(&mut self, ctx: &mut BTerm) {
        match self.flap_state {
            FlapState::Flapping => {
                self.tick_counter += 1;
                if self.tick_counter > FLAP_TICKS {
                    self.flap_state = FlapState::Idle;
                }
            }
            _ => {}
        }

        if self.velocity < MAX_FALL_SPEED {
            self.velocity += FALL_SPEED_INCREMENT;
        }
        self.y += self.velocity as i32;
        // self.x = (self.x + 1) % SCREEN_WIDTH;
        if self.y < 0 {
            self.y = 0;
        }
    }
    pub fn flap(&mut self) {
        match self.flap_state {
            FlapState::Flapping => {}
            FlapState::Idle => {
                self.tick_counter = 0;
                self.velocity = -JUMP_SPEED;
                self.flap_state = FlapState::Flapping;
            }
        }
    }
    pub fn reset(&mut self) {
        self.x = 20;
        self.y = SCREEN_HEIGHT/2;
        self.velocity = 0.0;
    }
}