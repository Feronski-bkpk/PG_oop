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

// индексирование карты ("row-first encoding")
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

    // отрисовка карты (рендер)
    pub fn render(&self, ctx: &mut BTerm) {
        // перебор карты
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // индексирование плитки
                let idx = map_idx(x,y);
                // сопоставление плитки (данной итерации) с типом плитки и
                // отрисовка этой плитки в соответствии с типом (координаты, цвет и символ)
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall =>  ctx.set(x, y, GREEN, BLACK, to_cp437('#'))
                }
            }
        }
    }

    // находится ли точка(x,y) в пределах карты?
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // может ли игрок ходить на плитку?
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // определение индексных координат плитки
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        // если запрашиваемые координаты выходят за пределы карты - вернуть None
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}