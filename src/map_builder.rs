use crate::prelude::*;

// максимальное число комнат на карте
const NUM_ROOMS: usize = 20;

//чертёж билдера
pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    player_start: Point
}
impl MapBuilder {
    // заполнение карты
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
    //
    fn build_random_rooms() {}
}