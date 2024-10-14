use crate::prelude::*;

// чертёж игрока
pub struct Player {
    position: Point // Point то же, что и (x,y)
}
impl Player {
    // создание нового игрока
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    // отрисовка игрока (рендер)
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.position.x, self.position.y, WHITE, BLACK, to_cp437('@'))
    }

    // обработка пользовательского ввода с клавиатуры / движение игрока
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            // delta - для хранения изменения позиции игрока
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };
            // вычисление новой позиции игрока
            let new_position = self.position + delta;
            // если перемещение возможно - выполнить
            if map.can_enter_tile(new_position) {
                self.position = new_position
            }
        }
    }
}