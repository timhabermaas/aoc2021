use itertools::Itertools;

type Point = (i32, i32);
type V2 = (i32, i32);

#[derive(Debug, Clone)]
struct Rect {
    from: Point,
    to: Point,
}

impl Rect {
    fn contains(&self, point: &Point) -> bool {
        self.from.0 <= point.0
            && self.to.0 >= point.0
            && self.from.1 <= point.1
            && self.to.1 >= point.1
    }

    fn bottom_right(&self) -> Point {
        (self.to.0, self.from.1)
    }
}

#[derive(Debug, Clone)]
struct Path {
    current_pos: Point,
    current_vec: V2,
    max_pos: Point,
}

impl Iterator for Path {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos.1 < self.max_pos.1 {
            return None;
        }

        let old_pos = self.current_pos;

        self.current_pos.0 += self.current_vec.0;
        self.current_pos.1 += self.current_vec.1;

        self.current_vec.0 += -1 * self.current_vec.0.signum();
        self.current_vec.1 -= 1;

        Some(old_pos)
    }
}

fn velocitys_on_target(start: Point, rect: Rect) -> impl Iterator<Item = V2> {
    (1..=283)
        .cartesian_product(-107..1000)
        .filter_map(move |(x, y)| {
            let v = (x, y);
            let mut path = Path {
                current_pos: start,
                current_vec: v,
                max_pos: rect.bottom_right(),
            };

            path.find(|pos| rect.contains(pos)).map(|_| v)
        })
}

fn main() {
    let rect = Rect {
        from: (230, -107),
        to: (283, -57),
    };

    let part_1 = velocitys_on_target((0, 0), rect.clone())
        .flat_map(|v| {
            Path {
                current_pos: (0, 0),
                current_vec: v,
                max_pos: rect.bottom_right(),
            }
            .map(|p| p.1)
        })
        .max()
        .unwrap();
    println!("Part 1: {:?}", part_1);

    let part_2 = velocitys_on_target((0, 0), rect.clone()).count();
    println!("Part 2: {:?}", part_2);
}
