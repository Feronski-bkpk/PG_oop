#![windows_subsystem="windows"]
#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;


mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

}
use prelude::*;

// чертёж игры
struct State {
    map: Map,
    player: Player
}
impl State {
    // создание новой игры
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start)
        }
    }
}
// для хранения состояния игры (между кадрами)
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx)
    }
}

// main возвращает тип Result (из bracket-lib)
fn main() -> BError {
    // создание терминала
    let context = BTermBuilder::simple80x50()
        .with_title("PG-game")
        .with_fps_cap(30.)
        .build()?;

    // запуск игрового цикла (новой игры)
    main_loop(context, State::new())
}
