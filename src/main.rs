use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1,"Hello bro");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Demon Slayer")
        .build()?;

    main_loop(context, State {})
}
