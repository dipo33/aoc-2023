pub enum SchematicItem {
    Blank,
    Symbol(char),
    PartLabel { part_number: u32, left: usize, right: usize },
}

pub struct Schematic {
    pub grid: Vec<Vec<SchematicItem>>,
}

impl Schematic {
    pub fn is_within_grid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.grid[0].len() as i32 && y >= 0 && y < self.grid.len() as i32
    }
    pub fn iter_with_positions(&self) -> impl Iterator<Item=(&SchematicItem, usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, item)| (item, x, y))
        })
    }

    pub fn is_part_from_engine(&self, x: usize, y: usize) -> bool {
        return match self.grid[y][x] {
            SchematicItem::PartLabel { left, .. } => {
                if left == 0 {
                    self.item_surroundings(x, y)
                        .iter()
                        .any(|&(x, y)| matches!(self.grid[y][x], SchematicItem::Symbol(_)))
                } else {
                    false
                }
            }
            _ => false,
        };
    }

    pub fn is_part_gear(&self, x: usize, y: usize) -> bool {
        return match self.grid[y][x] {
            SchematicItem::Symbol(c) => {
                if c == '*' {
                    self.item_surroundings(x, y)
                        .iter()
                        .any(|&(x, y)| matches!(self.grid[y][x], SchematicItem::Symbol(_)))
                } else {
                    false
                }
            }
            _ => false,
        };
    }

    pub fn item_surroundings(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (ix, iy) = (x as i32, y as i32);
        match self.grid[y][x] {
            SchematicItem::PartLabel { left, right, .. } => {
                let (left, right) = (left as i32, right as i32);
                let mut surroundings = vec![
                    (ix - left - 1, iy),
                    (ix + right + 1, iy),
                ];

                for i in (ix - left - 1)..(ix + right + 2) {
                    surroundings.push((i, iy - 1));
                    surroundings.push((i, iy + 1));
                }

                surroundings
                    .iter()
                    .filter(|&&(nx, ny)| self.is_within_grid(nx, ny))
                    .map(|&(nx, ny)| (nx as usize, ny as usize))
                    .collect()
            }
            _ => {
                let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
                directions.iter()
                    .map(|(dx, dy)| (ix + dx, iy + dy))
                    .filter(|&(nx, ny)| self.is_within_grid(nx, ny))
                    .map(|(nx, ny)| (nx as usize, ny as usize))
                    .collect()
            }
        }
    }

    pub fn engine_parts(&self) -> Vec<&SchematicItem> {
        self.iter_with_positions()
            .filter(|&(_, x, y)| self.is_part_from_engine(x, y))
            .map(|(item, ..)| item)
            .collect()
    }

    pub fn gears(&self) -> Vec<&SchematicItem> {
        self.iter_with_positions()
            .filter(|&(_, x, y)| self.is_part_from_engine(x, y))
            .map(|(item, ..)| item)
            .collect()
    }
}
