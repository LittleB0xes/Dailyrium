use bracket_lib::prelude::RGBA;
use bracket_lib::prelude::*;

mod living_entity;
use living_entity::LivingEntity;

mod engine;
use engine::Action;
use engine::*;

struct State {
    player: LivingEntity,
}

impl State {
    fn new() -> State {
        State {
            player: LivingEntity::new(5, 5),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        action_manager(&mut self.player);
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket World");

        ctx.set(
            self.player.x,
            self.player.y,
            RGBA::named(WHITE),
            RGBA::named(BLUE),
            self.player.glyph,
        );

        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Up => self.player.action = Action::MOVE(0, -1),
                VirtualKeyCode::Down => self.player.action = Action::MOVE(0, 1),
                VirtualKeyCode::Right => self.player.action = Action::MOVE(1, 0),
                VirtualKeyCode::Left => self.player.action = Action::MOVE(-1, 0),
                _ => {}
            },
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .with_tile_dimensions(12, 12) // Scale the font
        .build()?;

    let gs: State = State::new();
    main_loop(context, gs)
}
