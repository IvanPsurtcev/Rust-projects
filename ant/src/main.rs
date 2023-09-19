use png::{Encoder, BitDepth, ColorType};

const SIZE: usize = 1024;
const START: usize = SIZE / 2;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up, Right, Down, Left
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn main() {
    let mut grid = [[false; SIZE]; SIZE];
    let mut x = START;
    let mut y = START;
    let mut direction = Direction::Up;
    let mut out_of_bounds = false;

    while !out_of_bounds {
        if grid[y][x] {
            direction = turn_left(direction);
            grid[y][x] = false;
        } else {
            direction = turn_right(direction);
            grid[y][x] = true;
        }

        match direction {
            Direction::Up => if y == 0 { out_of_bounds = true } else { y -= 1 },
            Direction::Right => if x == SIZE - 1 { out_of_bounds = true } else { x += 1 },
            Direction::Down => if y == SIZE - 1 { out_of_bounds = true } else { y += 1 },
            Direction::Left => if x == 0 { out_of_bounds = true } else { x -= 1 },
        }
    }

    let mut black_cells = 0;
    let path = "ant.png";
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = Encoder::new(w, SIZE as u32, SIZE as u32);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::One);
    let mut writer = encoder.write_header().unwrap();

    let mut compressed_buffer: Vec<u8> = Vec::with_capacity((SIZE * SIZE) / 8);
    for row in &grid {
        let mut current_byte = 0u8;
        let mut bit_position = 0;

        for &cell in row {
            if !cell {
                current_byte |= 1 << (7 - bit_position);
            } else {
                black_cells += 1;
            }

            bit_position += 1;
            if bit_position == 8 {
                compressed_buffer.push(current_byte);
                current_byte = 0;
                bit_position = 0;
            }
        }
    }

    writer.write_image_data(&compressed_buffer).unwrap();

    println!("Number of black cells: {}", black_cells);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right(Direction::Up), Direction::Right);
        assert_eq!(turn_right(Direction::Right), Direction::Down);
        assert_eq!(turn_right(Direction::Down), Direction::Left);
        assert_eq!(turn_right(Direction::Left), Direction::Up);
    }

    #[test]
    fn test_turn_left() {
        assert_eq!(turn_left(Direction::Up), Direction::Left);
        assert_eq!(turn_left(Direction::Right), Direction::Up);
        assert_eq!(turn_left(Direction::Down), Direction::Right);
        assert_eq!(turn_left(Direction::Left), Direction::Down);
    }

    #[test]
    fn test_ant_movement_on_white_cell() {
        let mut x = START;
        let mut y = START;
        let mut direction = Direction::Up;

        assert_eq!(direction, Direction::Up);
        direction = turn_right(direction);
        match direction {
            Direction::Up => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        }
        assert_eq!(x, START + 1);
        assert_eq!(y, START);
        assert_eq!(direction, Direction::Right);
    }

    #[test]
    fn test_ant_reaching_boundary() {
        let mut x = SIZE / 2;
        let mut y = 0;
        let mut direction = Direction::Up;
        let mut out_of_bounds = false;

        if y == 0 {
            out_of_bounds = true;
        } else {
            match direction {
                Direction::Up => y -= 1,
                Direction::Right => x += 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
            }
        }
        assert_eq!(out_of_bounds, true);

        x = SIZE - 1;
        y = SIZE / 2;
        direction = Direction::Right;
        out_of_bounds = false;
        if x == SIZE - 1 {
            out_of_bounds = true;
        } else {
            match direction {
                Direction::Up => y -= 1,
                Direction::Right => x += 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
            }
        }
        assert_eq!(out_of_bounds, true);
    }
}
