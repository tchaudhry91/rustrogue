use map::TileType;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod rect;
pub use rect::*;
mod map;
pub use map::*;
mod components;
pub use components::*;
mod player;
pub use player::*;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<TileType>>();
        map::draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Buzzfield").build()?;
    let mut gs = State { ecs: World::new() };
    let (game_map, rooms) = map::new_map_rooms_and_corridors();
    let (player_x, player_y) = rooms[0].center();
    gs.ecs.insert(game_map);
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
