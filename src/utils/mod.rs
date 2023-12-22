pub(crate) mod test_helpers {
    use std::error::Error;
    use robotics_lib::world::tile::Content::Coin;
    use robotics_lib::world::tile::Tile;
    use crate::coordinates::map_coordinate::MapCoordinate;

    pub fn print_grid(world_map: &Vec<Vec<Tile>>, known_tiles: &Option<Vec<Vec<Option<Tile>>>>, robot_pos: (usize,usize)) {
        let x = robot_pos.1;
        let y = robot_pos.0;
        println!("printing world map");
        println!("robot_pos: ({},{})",x, y);
        for (height,row) in world_map.iter().enumerate() {
            for (width, tile) in row.iter().enumerate() {
                if robot_pos == (height, width) {
                    print!("R");
                }
                else if tile.content == Coin(1) {
                    print!("o");
                }
                else {
                    print!("_");
                }
            }
            println!("");
        }

        match known_tiles {
            Some(vector) => {
                println!("printing known tiles:");
                println!("robot_pos: ({},{})",x, y);
                for (height, row) in vector.iter().enumerate() {
                    for (width, tile) in row.iter().enumerate() {
                        match tile {
                            Some(t) => {
                                if robot_pos == (height, width) {
                                    print!("R");
                                }
                                else if t.content == Coin(1) {
                                    print!("o");
                                }
                                else {
                                    print!("x");
                                }
                            }
                            None => print!("_"),
                        }
                    }
                    println!("");
                }
            }

            None => println!("known tiles is None"),
        }
    }
}