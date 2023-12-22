pub(crate) mod test_helpers {
    use std::error::Error;
    use robotics_lib::world::tile::Content::Coin;
    use robotics_lib::world::tile::Tile;
    use crate::coordinates::map_coordinate::MapCoordinate;

    pub fn print_grid(world_map: &Vec<Vec<Tile>>, known_tiles: &Option<Vec<Vec<Option<Tile>>>>) {
        println!("printing world map");
        for row in world_map.iter() {
            for tile in row.iter() {
                if tile.content == Coin(1) {
                    print!("o");
                }
                else {
                    print!(" ");
                }
            }
            println!("");
        }

        match known_tiles {
            Some(vector) => {
                println!("printing known tiles:");
                for row in vector.iter() {
                    for tile in row.iter() {
                        match tile {
                            Some(t) => {
                                if t.content == Coin(1) {
                                    print!("o");
                                }
                                else {
                                    print!("x");
                                }
                            }
                            None => print!(" "),
                        }
                    }
                    println!("");
                }
            }

            None => println!("known tiles is None"),
        }
    }
}