use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Vector {
    pub row: isize,
    pub col: isize,
}

impl Vector {
    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }

    pub fn mul(&self, by: isize) -> Vector {
        Vector {
            row: self.row * by,
            col: self.col * by,
        }
    }

    pub fn div(&self, by: isize) -> Vector {
        Vector {
            row: self.row / by,
            col: self.col / by,
        }
    }
}

pub fn find_inner_points(input_loop: Vec<Vector>) -> HashSet<Vector> {
    // to allow us to draw rays to points without awkward interactions, we'll shift all the points
    // slightly out of the plane
    let expanded_loop = input_loop.iter().map(|p| p.mul(2)).collect::<Vec<_>>();
    let filled_loop = expanded_loop
        .windows(2)
        .chain(std::iter::once(
            &[expanded_loop[expanded_loop.len() - 1], expanded_loop[0]][..],
        )) // include the last and first element pair, which windows _doesn't_
        .map(|w| {
            let [curr, next]: [_; 2] = w.try_into().unwrap();
            let mid_pos = curr.add(&next).div(2);

            vec![curr, mid_pos]
        })
        .flatten()
        .collect::<HashSet<_>>();

    // now we just need to draw rays from offset positions from the side of the map and keep a count
    // of the number of intersections of the loop. Odd number => inner point.

    let size = expanded_loop.iter().fold(
        (expanded_loop[0].clone(), expanded_loop[0].clone()),
        |(mut min, mut max), v| {
            let min_vec = Vector {
                row: min.row.min(v.row),
                col: min.col.min(v.col),
            };
            let max_vec = Vector {
                row: max.row.max(v.row),
                col: max.col.max(v.col),
            };

            (min_vec, max_vec)
        },
    );

    let mut inner_points = HashSet::new();
    for r in size.0.row..=size.1.row {
        let mut loop_intersects = 0;
        for c in size.0.col..=size.1.col {
            let pos = Vector { row: r, col: c };

            if filled_loop.contains(&pos) {
                loop_intersects += 1;
            } else {
                if (pos.row % 2 == 1 && pos.col % 2 == 1) && loop_intersects % 2 == 1 {
                    inner_points.insert(pos);
                }
            }
        }
    }

    // we have counted some "new" positions though that aren't strictly a tile originally! Remap the
    // inner points and ignore any that map to the loop

    inner_points
        .iter()
        .map(|v| v.add(&Vector { row: -1, col: -1 }))
        .filter(|x| !filled_loop.contains(x))
        .collect()
}
