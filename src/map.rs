use crate::prelude::*;

// число всех плиток на карте
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// безопасное назначение, копирование и сравнение
#[derive(Copy, Clone, PartialEq)]
// возможные типы плиток
pub enum TileType {
    Wall,
    Floor
}

// индексирование карты ("row-first encoding") ("сначала строка")
// y - целая часть при делении -> idx/width
// x - остаток при делении -> idx%width
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

// чертёж карты
pub struct Map {
    pub tiles: Vec<TileType>
}
impl Map {
    // создание новой карты
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // отрисовка карты (только видимой части карты)
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // рендеринг только первого консольного слоя (базовый слой)
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x,y)) {

                    let idx = map_idx(x, y);
                    // рендер плитки
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('.')),
                        TileType::Wall => ctx.set(x - camera.left_x, y - camera.top_y, GRAY, BLACK, to_cp437('#'))
                    }
                }
            }
        }
    }

    // находится ли точка(x,y) в пределах карты?
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // можно ли ходить на плитку?
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // "определение нерушимости границ карты и её пределов"
    // если запрашиваемые координаты находятся в пределах карты (не включая границу), то
    // вернуть индекс запрашиваемой плитки (как следствие, разрешить действие с координатой)
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        // если запрашиваемые координаты выходят за пределы карты - вернуть None
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}