
# Resource Scanner Tool

The Resource Scanner Tool is a Rust library that provides functionality to scan an area around a robot for a specified content and pattern. It is designed to work with the [robotics_lib](https://advancedprogramming.disi.unitn.it/crate?name=robotics_lib) library.

## Features

- Scan in various patterns, such as square areas, directional scans, diagonal scans, star and cross patterns.
- Retrieve coordinates and count of discovered tiles containing specific content.
- Handle errors, including cases where the robot doesn't have enough energy or there are no more tiles to discover.

## Usage

```rust
use resource_scanner_tool::tool::resource_scanner::{ResourceScanner, Pattern};

// Create a ResourceScanner instance
let mut scanner = ResourceScanner {};

// Scan in a square area with a side length of 5
let area_scan_result = scanner.scan(&mut world, &mut robot, Pattern::Area(5), content_to_search_for);

// Scan upward with a distance of 3
let up_scan_result = scanner.scan(&mut world, &mut robot, Pattern::DirectionUp(3), content_to_search_for);
```

## Patterns

The library supports the following scanning patterns:

-   `Area(usize)`: Scans in a square area with a side length specified by the `usize` parameter.
-   `DirectionUp(usize)`: Scans in an upward direction with the specified distance.
-   `DirectionRight(usize)`: Scans in a rightward direction with the specified distance.
-   `DirectionLeft(usize)`: Scans in a leftward direction with the specified distance.
-   `DirectionDown(usize)`: Scans in a downward direction with the specified distance.
-   `DiagonalUpperLeft(usize)`: Scans diagonally in the upper-left direction with the specified distance.
-   `DiagonalUpperRight(usize)`: Scans diagonally in the upper-right direction with the specified distance.
-   `DiagonalLowerLeft(usize)`: Scans diagonally in the lower-left direction with the specified distance.
-   `DiagonalLowerRight(usize)`: Scans diagonally in the lower-right direction with the specified distance.
-   `StraightStar(usize)`: Scans in a star pattern in all directions with the specified distance.
-   `DiagonalStar(usize)`: Scans in a star pattern diagonally in all directions with the specified distance.
## Examples

```rust
// Scan in a square area with a side length of 5
use resource_scanner_tool::tool::resource_scanner::Pattern;
let area_scan = Pattern::Area(5);

// Scan upward with a distance of 3
let up_scan = Pattern::DirectionUp(3);
```
## Known Issues

During development, it has been observed that the scan tool does not consistently return the expected output due to the behavior of the `go` interface.

Consider the following scenario:

- The robot, denoted by `R`, starts at coordinate `(0,0)` in a 5x5 grid.
- The desired item is represented by `o`.
- We assume the pattern used is `Area(3)`.
- The robot moves to the right, reaching coordinate `(0,1)` on the next tick.
- Initially, the robot knows nothing about the world, and the robot_map is empty.

|   | 0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| 0 | R |   | o |   |   |
| 1 |   |   |   |   |   |
| 2 |   |   |   |   |   |
| 3 |   |   |   |   |   |
| 4 |   |   |   |   |   |

The expected behavior is that the robot moves to `(0,1)`, and when the scan tool is used, it should return the position of the item in `(0,2)`.

However, the `go` interface, which is used to move the robot, automatically scans its surroundings after every move using the `robot_view` interface. 
Thus, immediately after the move, `robot_map` contains the coordinates highlighted with `*`:

|   | 0 | 1  | 2  | 3 | 4 |
|---|---|----|----|---|---|
| 0 | * | R* | o* |   |   |
| 1 | * | *  | *  |   |   |
| 2 |   |    |    |   |   |
| 3 |   |    |    |   |   |
| 4 |   |    |    |   |   |

When the robot uses the scan tool, it does not return `(0,2)` because that tile is already known to the robot.

**It is therefore recommended to conscientiously _compare the output of the scan tool with the existing knowledge_ of the robot.**

## Contributing

Feel free to actively contribute by opening GitHub issues to report problems, suggest enhancements, or discuss any aspect of the tool's development.
