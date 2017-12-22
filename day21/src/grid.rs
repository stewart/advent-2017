use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid {
    contents: Vec<Vec<bool>>
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self { contents: vec![vec![false; size]; size] }
    }

    pub fn stitch(sub_grids: Vec<Self>) -> Self {
        let grid_count = (sub_grids.len() as f64).sqrt() as usize;
        let grid_size = sub_grids[0].size();

        let mut grid = Self::new(grid_size * grid_count);

        for (idx, sub) in sub_grids.iter().enumerate() {
            let gx = idx % grid_count;
            let gy = idx / grid_count;

            for x in 0..grid_size {
                for y in 0..grid_size {
                    let value = sub.get(x, y);
                    grid.set(x + gx * grid_size, y + gy * grid_size, value);
                }
            }
        }

        grid
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.contents[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.contents[x][y] = value;
    }

    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn count(&self) -> usize {
        self.contents.
            iter().
            map(|row| row.iter().filter(|&&c| c).count()).
            sum()
    }

    pub fn flip(&self) -> Self {
        let contents = self.contents.
            iter().
            map(|row| row.iter().cloned().rev().collect()).
            collect();

        Self { contents: contents }
    }

    pub fn rotate(&self) -> Self {
        let size = self.size();
        let mut rotated = Self::new(self.size());

        for x in 0..size {
            for y in 0..size {
                rotated.set(y, size - x - 1, self.get(x, y));
            }
        }

        rotated
    }

    pub fn permutations(&self) -> Vec<Self> {
        let mut permutations = vec![self.clone(), self.flip()];

        for _ in 0..6 {
            let grid = permutations[permutations.len() - 2].rotate();
            permutations.push(grid)
        }

        permutations
    }

    pub fn split(&self) -> Vec<Self> {
        let size = self.size();

        let m = match size {
            _ if size % 2 == 0 => 2,
            _ if size % 3 == 0 => 3,
            _ => { panic!("Can't split grid of size: {}", size) }
        };

        let size = size / m;

        (0..(size * size)).
            map(|idx| {
                let mut grid = Self::new(m);

                let gx = idx % size;
                let gy = idx / size;

                for x in 0..m {
                    for y in 0..m {
                        let value = self.get(x + gx * m, y + gy * m);
                        grid.set(x, y, value);
                    }
                }

                grid
            }).collect()
    }
}

impl Default for Grid {
    fn default() -> Self {
        let contents = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true,  true,  true]
        ];

        Self { contents: contents }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let contents = input.
            split("/").
            map(|row| {
                row.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected char: {}", c)
                }).collect()
            }).collect();

        Ok(Self { contents: contents })
    }
}
