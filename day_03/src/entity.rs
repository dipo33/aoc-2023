pub type Position = (i32, i32);

pub fn neighbours((x, y): Position) -> Vec<Position> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

#[derive(Debug, Clone, Copy)]
pub enum Item {
    Symbol(char),
    PartLabel(u32),
}

pub enum BlueprintItem {
    Item(Item),
    LeftRedirect(u8),
    Blank,
}

pub struct Blueprint {
    pub grid: Vec<Vec<BlueprintItem>>,
}

impl Blueprint {
    pub fn new(grid: Vec<Vec<BlueprintItem>>) -> Self {
        Self { grid }
    }

    pub fn pos_iter(&self) -> BlueprintPosIter {
        BlueprintPosIter::new(self)
    }

    fn within(&self, (x, y): Position) -> bool {
        x >= 0 && y >= 0 && y < self.grid.len() as i32 && x < self.grid[0].len() as i32
    }

    pub fn get_item_at(&self, pos @ (x, y): Position) -> (Position, Option<Item>) {
        if !self.within(pos) {
            return (pos, None);
        }

        match self.grid[y as usize][x as usize] {
            BlueprintItem::Item(item) => (pos, Some(item)),
            BlueprintItem::LeftRedirect(l) => self.get_item_at((x - l as i32, y)),
            _ => (pos, None)
        }
    }
}

pub struct BlueprintPosIter<'a> {
    blueprint: &'a Blueprint,
    current_pos: (i32, i32),
}

impl<'a> BlueprintPosIter<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            current_pos: (0, 0),
        }
    }
}

impl<'a> Iterator for BlueprintPosIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos.1 >= self.blueprint.grid.len() as i32 {
            return None;
        }

        let result = self.current_pos;

        self.current_pos.0 += 1;
        if self.current_pos.0 >= self.blueprint.grid[0].len() as i32 {
            self.current_pos.0 = 0;
            self.current_pos.1 += 1;
        }

        Some(result)
    }
}
