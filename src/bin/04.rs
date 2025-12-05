advent_of_code::solution!(4);

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn get_at(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self.data[y][x] = '.'
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let max_rows = self.height();
        let max_columns = self.width();

        let deltas: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut neighbors = Vec::new();

        for (dr, dc) in deltas.iter() {
            let new_row_i = row as isize + dr;
            let new_col_i = col as isize + dc;

            let is_valid_row = new_row_i >= 0 && new_row_i < max_rows as isize;
            let is_valid_col = new_col_i >= 0 && new_col_i < max_columns as isize;

            if is_valid_row && is_valid_col {
                neighbors.push((new_row_i as usize, new_col_i as usize))
            }
        }
        neighbors
    }

    pub fn get_neighbors_values(&self, row: usize, col: usize) -> Vec<char> {
        let neighbor_coordinates = self.get_neighbors(row, col);
        let mut neighbor_values: Vec<char> = Vec::new();
        for neighbor in neighbor_coordinates {
            neighbor_values.push(self.get_at(neighbor.0, neighbor.1))
        }

        neighbor_values
    }

    pub fn print_self(&self) {
        Self::print_grid(&self.data)
    }

    pub fn remove_accessible_rolls(&mut self) -> usize {
        // returns number of rolls removed
        let mut accessible_coordinates: Vec<(usize, usize)> = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get_at(x, y) == '@' {
                    let neighbor_values: Vec<char> = self.get_neighbors_values(x, y);
                    let neighboring_rolls: usize =
                        neighbor_values.iter().filter(|&&c| c == '@').count();
                    if neighboring_rolls < 4 {
                        accessible_coordinates.push((x, y));
                    }
                }
            }
        }

        let removed_rolls: usize = accessible_coordinates.len();
        for roll in accessible_coordinates {
            self.remove(roll.0, roll.1);
        }

        removed_rolls
    }

    // Remember that x is columns and max_x is width, y is rows and max_y is height
    pub fn print_overlay(&self, overlay: Vec<(usize, usize)>) {
        let mut overlaid_grid: Vec<Vec<char>> = Vec::new();
        for y in 0..self.height() {
            let mut row: Vec<char> = Vec::new();
            for x in 0..self.width() {
                if overlay.contains(&(x, y)) {
                    row.push('x');
                } else {
                    row.push(self.get_at(x, y));
                }
            }
            overlaid_grid.push(row);
        }

        Self::print_grid(&overlaid_grid);
    }

    fn print_grid(grid: &Vec<Vec<char>>) {
        for row in grid.iter() {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }
}

fn convert_to_2d_array(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid {
        data: convert_to_2d_array(input),
    };
    // println!("grid is");
    // grid.print_self();

    let max_rows = grid.height();
    let max_columns = grid.width();
    let mut accessible_rolls: usize = 0;
    let mut accessible_coordinates: Vec<(usize, usize)> = Vec::new();

    for i in 0..max_rows {
        for j in 0..max_columns {
            if grid.get_at(i, j) == '@' {
                let neighbor_values: Vec<char> = grid.get_neighbors_values(i, j);
                let neighboring_rolls: usize =
                    neighbor_values.iter().filter(|&&c| c == '@').count();
                if neighboring_rolls < 4 {
                    accessible_rolls += 1;
                    accessible_coordinates.push((i, j));
                }
            }
        }
    }

    // println!("Overlay grid is:");
    // grid.print_overlay(accessible_coordinates);

    Some(accessible_rolls.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    // Initialize grid
    let mut grid = Grid {
        data: convert_to_2d_array(input),
    };

    // As long as there is >0 accessible rolls,
    // check for accessible rolls
    // and remove them.

    let mut total_removed_rolls: usize = 0;
    loop {
        let removed_rolls = grid.remove_accessible_rolls();
        if removed_rolls == 0 {
            break;
        } else {
            total_removed_rolls += removed_rolls;
        }
    }

    // println!("Before removing accessible rolls, grid was:");
    // grid.print_self();
    // let removed_rolls: usize = grid.remove_accessible_rolls();
    // println!("After removing {removed_rolls} accessible rolls, grid is:");
    // grid.print_self();

    //

    Some(total_removed_rolls.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
