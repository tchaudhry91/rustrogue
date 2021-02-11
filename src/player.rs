use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest = Position {
            x: min(79, max(0, pos.x + delta_x)),
            y: min(49, max(0, pos.y + delta_y)),
        };
        if map[xy_idx(dest.x, dest.y)] == TileType::Wall {
            return;
        }
        pos.x = dest.x;
        pos.y = dest.y;
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player Movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
