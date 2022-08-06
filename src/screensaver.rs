use bracket_lib::prelude::*;

struct State{
    x: i32,
    y: i32,
    tick_step: i32
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm){
        ctx.cls();
        self.tick_step += 1;
        if self.tick_step > 25 {
            self.x = (self.x + 1) % 80;
            self.y = (self.y + 1) % 50;
            self.tick_step = 0;
        }
        ctx.print(self.x, self.y, "thug life"); 
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("screensaver")
        .build()?;
    main_loop(ctx, State{x:0, y:0, tick_step: 15})
}
