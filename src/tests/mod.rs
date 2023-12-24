use crate::coordinates::map_coordinate::MapCoordinate;
use crate::tool::resource_scanner::{Pattern, ResourceScanner};
use crate::utils::test_helpers::print_grid;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{debug, discover_tiles, robot_map};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;

#[cfg(test)]
mod tests {
    use crate::coordinates::map_coordinate::MapCoordinate;
    use crate::errors::tool_errors::ToolError;
    use crate::tool::resource_scanner::{Pattern, ResourceScanner};
    use crate::utils::test_helpers::print_grid;
    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::{debug, discover_tiles, go, robot_map, Direction};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runnable, Runner};
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
    use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use robotics_lib::world::world_generator::Generator;
    use robotics_lib::world::world_generator::World as WorldType;
    use robotics_lib::world::World;
    use std::mem;

    #[test]
    fn test_new_map_coordinate() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);
    }

    #[test]
    fn test_get_width() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_width(), 10);
    }

    #[test]
    fn test_set_width() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_width(15);
        assert_eq!(coordinates.get_width(), 15);
    }

    #[test]
    fn test_get_height() {
        let coordinates = MapCoordinate::new(10, 20);
        assert_eq!(coordinates.get_height(), 20);
    }

    #[test]
    fn test_set_height() {
        let mut coordinates = MapCoordinate::new(10, 20);
        coordinates.set_height(25);
        assert_eq!(coordinates.get_height(), 25);
    }

    #[test]
    fn test_equality() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(10, 20);
        let coordinates3 = MapCoordinate::new(15, 25);

        assert_eq!(coordinates1, coordinates2);
        assert_ne!(coordinates1, coordinates3);
    }

    #[test]
    fn test_addition() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 + coordinates2;
        assert_eq!(result, MapCoordinate::new(15, 30));
    }

    #[test]
    fn test_subtraction() {
        let coordinates1 = MapCoordinate::new(10, 20);
        let coordinates2 = MapCoordinate::new(5, 10);
        let result = coordinates1 - coordinates2;
        assert_eq!(result, MapCoordinate::new(5, 10));
    }

    #[test]
    fn test_from_into_conversion() {
        let tuple_coordinates: (usize, usize) = (10, 20);
        let coordinates: MapCoordinate = tuple_coordinates.into();
        assert_eq!(coordinates.get_width(), 10);
        assert_eq!(coordinates.get_height(), 20);

        let converted_tuple: (usize, usize) = coordinates.into();
        assert_eq!(converted_tuple, (10, 20));
    }

    #[test]
    fn test_debug_display_and_error_traits() {
        // Test Debug trait
        assert_eq!(format!("{:?}", ToolError::InvalidSizeError), "Invalid Size");
        assert_eq!(
            format!("{:?}", ToolError::EmptyCoordinates),
            "Empty Coordinates"
        );
        assert_eq!(
            format!("{:?}", ToolError::NotEnoughEnergy),
            "Not Enough Energy"
        );
        assert_eq!(
            format!("{:?}", ToolError::NoMoreDiscovery),
            "No More Discovery"
        );
        assert_eq!(
            format!("{:?}", ToolError::Other("Custom Error".to_string())),
            "Custom Error"
        );

        // Test Display trait
        assert_eq!(format!("{}", ToolError::InvalidSizeError), "Invalid Size");
        assert_eq!(
            format!("{}", ToolError::EmptyCoordinates),
            "Empty Coordinates"
        );
        assert_eq!(
            format!("{}", ToolError::NotEnoughEnergy),
            "Not Enough Energy"
        );
        assert_eq!(
            format!("{}", ToolError::NoMoreDiscovery),
            "No More Discovery"
        );
        assert_eq!(
            format!("{}", ToolError::Other("Custom Error".to_string())),
            "Custom Error"
        );

        // Test Error trait
        assert_eq!(ToolError::InvalidSizeError.to_string(), "Invalid Size");
        assert_eq!(ToolError::EmptyCoordinates.to_string(), "Empty Coordinates");
        assert_eq!(ToolError::NotEnoughEnergy.to_string(), "Not Enough Energy");
        assert_eq!(ToolError::NoMoreDiscovery.to_string(), "No More Discovery");
        assert_eq!(
            ToolError::Other("Custom Error".to_string()).to_string(),
            "Custom Error"
        );
    }

    #[test]
    fn test_scan_tool_area_3_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::Area(3), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 1, 2, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_area_3_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::Area(3), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,4)
                map[4][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(5, 1, 2, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_area_5_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::Area(5), Content::Coin(0));
                // let (_world, _, robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world, &_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_area_5_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::Area(5), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,4)
                map[4][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 2, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_left_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(1, 2), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (1,2)
                map[2][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 2, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_left_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (1,2)
                map[2][1] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_right_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionRight(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(3, 2), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (3,2)
                map[2][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 2, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_right_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionRight(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (3,2)
                map[2][3] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_up_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionUp(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 1), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,1)
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 3, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_up_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionUp(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,1)
                map[1][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 3, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_down_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionDown(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_down_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DirectionDown(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ul_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result =
                    tool.scan(world, self, Pattern::DiagonalUpperLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(0, 1), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (0,1)
                map[1][0] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 3, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ul_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result =
                    tool.scan(world, self, Pattern::DiagonalUpperLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (0,1)
                map[1][0] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 4, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ur_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(
                    world,
                    self,
                    Pattern::DiagonalUpperRight(2),
                    Content::Coin(0),
                );
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(4, 1), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (4,1)
                map[1][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 3, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ur_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(
                    world,
                    self,
                    Pattern::DiagonalUpperRight(2),
                    Content::Coin(0),
                );
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (4,1)
                map[1][4] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 2, 4, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ll_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result =
                    tool.scan(world, self, Pattern::DiagonalLowerLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 4, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_ll_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result =
                    tool.scan(world, self, Pattern::DiagonalLowerLeft(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_lr_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(
                    world,
                    self,
                    Pattern::DiagonalLowerRight(2),
                    Content::Coin(0),
                );
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 0, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_direction_lr_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(
                    world,
                    self,
                    Pattern::DiagonalLowerRight(2),
                    Content::Coin(0),
                );
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_straight_star_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::StraightStar(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 4, 3, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_straight_star_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::StraightStar(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 4, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_diagonal_star_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DiagonalStar(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 3), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 4, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_tool_diagonal_star_not_found() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::DiagonalStar(2), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    #[should_panic]
    fn test_scan_error() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};

                let result = tool.scan(world, self, Pattern::Area(40), Content::Coin(0));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(None, content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,3)
                map[3][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Coin(1),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 3, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[test]
    fn test_scan_fire() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};
                let result = tool.scan(world, self, Pattern::Area(3), Content::Fire);
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 2), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,2)
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Fire,
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }

    #[should_panic]
    #[test]
    fn test_scan_bin() {
        struct TestRobot(Robot);
        impl Runnable for TestRobot {
            fn process_tick(&mut self, world: &mut World) {
                let mut tool = ResourceScanner {};
                let result = tool.scan(world, self, Pattern::Area(3), Content::Bin(1..3));
                // let (_world,_,robot_pos) = debug(self, world);
                // let _known = robot_map(world);
                // print_grid(&_world,&_known, robot_pos);
                match result {
                    Ok(content) => {
                        assert_eq!(Some((MapCoordinate::new(2, 2), 1)), content);
                    }
                    Err(_) => panic!(),
                }
            }
            fn handle_event(&mut self, event: Event) {
                // println!();
                // println!("{:?}", event);
                // println!();
            }
            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack {
                &mut self.0.backpack
            }
        }

        struct WorldGenerator {
            size: usize,
            spawn_x: usize,
            spawn_y: usize,
            tile_type: TileType,
        }

        impl WorldGenerator {
            fn new(size: usize, spawn_x: usize, spawn_y: usize, tile_type: TileType) -> Self {
                Self {
                    size,
                    spawn_x,
                    spawn_y,
                    tile_type,
                }
            }
        }

        impl Generator for WorldGenerator {
            fn gen(&mut self) -> WorldType {
                let mut map: Vec<Vec<Tile>> = Vec::new();
                // Initialize the map with default tiles
                for _ in 0..self.size {
                    let mut row: Vec<Tile> = Vec::new();
                    for _ in 0..self.size {
                        let tile = Tile {
                            tile_type: self.tile_type,
                            content: Content::None,
                            elevation: 0,
                        };
                        row.push(tile);
                    }
                    map.push(row);
                }
                // add coin in (2,2)
                map[2][2] = Tile {
                    tile_type: self.tile_type,
                    content: Content::Bin(1..8),
                    elevation: 0,
                };

                let environmental_conditions =
                    EnvironmentalConditions::new(&vec![Sunny], 15, 12).unwrap();
                // implementation
                return (
                    map,
                    (self.spawn_y, self.spawn_x),
                    environmental_conditions,
                    10.0,
                    None,
                );
            }
        }

        let r = TestRobot(Robot::new());
        let runner = Runner::new(
            Box::new(r),
            &mut WorldGenerator::new(50, 1, 1, TileType::Grass),
        );
        let _ = runner.unwrap().game_tick();
    }
}
