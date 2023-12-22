pub type Index = u32;

#[derive(Debug, Clone, Copy)]
pub enum BlueprintItem {
    Symbol(char),
    PartLabel(u32, Index),
    Blank,
}

pub struct Blueprint {
    grid: Vec<BlueprintItem>,
    row_size: u32,
}

impl Blueprint {
    pub fn new(grid: Vec<BlueprintItem>, row_size: u32) -> Self {
        Self { grid, row_size }
    }

    pub fn indices(&self) -> BlueprintPosIter {
        BlueprintPosIter::new(self)
    }

    fn within(&self, idx: Index) -> bool {
        idx < self.grid.len() as Index
    }

    pub fn get(&self, idx: Index) -> BlueprintItem {
        if !self.within(idx) {
            return BlueprintItem::Blank;
        }

        self.grid[idx as usize]
    }
    pub fn neighbours(&self, idx: Index) -> Vec<Index> {
        vec![
            idx - self.row_size - 1,
            idx - self.row_size,
            idx - self.row_size + 1,
            idx - 1,
            idx + 1,
            idx + self.row_size - 1,
            idx + self.row_size,
            idx + self.row_size + 1,
        ]
    }
}

pub struct BlueprintPosIter<'a> {
    blueprint: &'a Blueprint,
    current_pos: Index,
}

impl<'a> BlueprintPosIter<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            current_pos: 0,
        }
    }
}

impl<'a> Iterator for BlueprintPosIter<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos > self.blueprint.grid.len() as Index {
            return None;
        }

        let result = self.current_pos;

        self.current_pos += 1;
        Some(result)
    }
}
