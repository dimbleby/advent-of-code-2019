use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum TileContent {
    Bug,
    Space,
}

impl From<char> for TileContent {
    fn from(x: char) -> Self {
        match x {
            '#' => Self::Bug,
            _ => Self::Space,
        }
    }
}

#[derive(Copy, Clone)]
struct Grid {
    cells: [[TileContent; 5]; 5],
}

impl Grid {
    fn new(cells: [[TileContent; 5]; 5]) -> Self {
        Self { cells }
    }

    fn empty() -> Self {
        let cells = [[TileContent::Space; 5]; 5];
        Self::new(cells)
    }

    fn biodiversity(&self) -> u32 {
        let mut diversity = 0;
        for (y, line) in self.cells.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell == &TileContent::Bug {
                    diversity += 1 << (5 * x + y);
                }
            }
        }
        diversity
    }

    fn count_bugs(&self) -> usize {
        let mut count = 0;
        for line in self.cells.iter() {
            for cell in line.iter() {
                if cell == &TileContent::Bug {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_neighbours(&self, counts: &mut [[usize; 5]; 5]) {
        for (y, line) in counts.iter_mut().enumerate() {
            for (x, count) in line.iter_mut().enumerate() {
                if x > 0 && self.cells[x - 1][y] == TileContent::Bug {
                    *count += 1;
                }
                if x < 4 && self.cells[x + 1][y] == TileContent::Bug {
                    *count += 1;
                }
                if y > 0 && self.cells[x][y - 1] == TileContent::Bug {
                    *count += 1;
                }
                if y < 4 && self.cells[x][y + 1] == TileContent::Bug {
                    *count += 1;
                }
            }
        }
    }

    fn update(&mut self, counts: &[[usize; 5]; 5]) {
        for (y, line) in self.cells.iter_mut().enumerate() {
            for (x, cell) in line.iter_mut().enumerate() {
                let count = counts[x][y];
                if *cell == TileContent::Bug && count != 1 {
                    *cell = TileContent::Space;
                } else if *cell == TileContent::Space && (count == 1 || count == 2) {
                    *cell = TileContent::Bug;
                }
            }
        }
    }

    fn evolve(&mut self) {
        let mut counts = [[0; 5]; 5];
        self.count_neighbours(&mut counts);
        self.update(&counts);
    }
}

struct MultiGrid {
    grids: [Grid; 201],
}

impl MultiGrid {
    fn new(starter: Grid) -> Self {
        let mut grids = [Grid::empty(); 201];
        grids[100] = starter;
        Self { grids }
    }

    fn count_bugs(&self) -> usize {
        self.grids.iter().map(|g| g.count_bugs()).sum()
    }

    fn evolve(&mut self) {
        let mut counts = [[[0; 5]; 5]; 201];

        // Start by getting the naive counts.
        for (idx, grid) in self.grids.iter_mut().enumerate() {
            grid.count_neighbours(&mut counts[idx]);
        }

        // Add neighbours from outer layers.
        for (idx, count) in counts.iter_mut().enumerate().skip(1) {
            if self.grids[idx - 1].cells[2][1] == TileContent::Bug {
                for line in count.iter_mut() {
                    line[0] += 1;
                }
            }
            if self.grids[idx - 1].cells[1][2] == TileContent::Bug {
                for y in 0..5 {
                    count[0][y] += 1;
                }
            }
            if self.grids[idx - 1].cells[3][2] == TileContent::Bug {
                for y in 0..5 {
                    count[4][y] += 1;
                }
            }
            if self.grids[idx - 1].cells[2][3] == TileContent::Bug {
                for line in count.iter_mut() {
                    line[4] += 1;
                }
            }
        }

        // Add neighbours from inner layers.
        for (idx, count) in counts.iter_mut().enumerate().take(200) {
            for x in 0..5 {
                if self.grids[idx + 1].cells[x][0] == TileContent::Bug {
                    count[2][1] += 1;
                }
                if self.grids[idx + 1].cells[x][4] == TileContent::Bug {
                    count[2][3] += 1;
                }
            }
            for y in 0..5 {
                if self.grids[idx + 1].cells[0][y] == TileContent::Bug {
                    count[1][2] += 1;
                }
                if self.grids[idx + 1].cells[4][y] == TileContent::Bug {
                    count[3][2] += 1;
                }
            }
        }

        // Now update everything.
        for (idx, grid) in self.grids.iter_mut().enumerate() {
            for (y, line) in grid.cells.iter_mut().enumerate() {
                for (x, cell) in line.iter_mut().enumerate() {
                    if (x == 2) && (y == 2) {
                        continue;
                    }
                    let count = counts[idx][x][y];
                    if *cell == TileContent::Bug && count != 1 {
                        *cell = TileContent::Space;
                    } else if *cell == TileContent::Space && (count == 1 || count == 2) {
                        *cell = TileContent::Bug;
                    }
                }
            }
        }
    }
}

pub(crate) fn day24() {
    let input = File::open("data/day24.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let mut cells = [[TileContent::Space; 5]; 5];
    for (y, line) in buffered.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            cells[x][y] = TileContent::from(c);
        }
    }

    // Part one.
    let mut grid = Grid::new(cells);
    let mut seen = HashSet::new();
    loop {
        let biodiversity = grid.biodiversity();
        if !seen.insert(biodiversity) {
            println!("Part one answer is: {}", biodiversity);
            break;
        }
        grid.evolve();
    }

    // Part two.
    let starter = Grid::new(cells);
    let mut multigrid = MultiGrid::new(starter);
    for _ in 0..200 {
        multigrid.evolve();
    }
    let answer = multigrid.count_bugs();
    println!("Part two answer is: {}", answer);
}
