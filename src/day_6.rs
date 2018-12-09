use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

fn manhattan_dist(p1: &Point, p2: &Point) -> u32 {
    let abs_diff = |a, b| if a > b { a - b } else { b - a };
    abs_diff(p1.x, p2.x) + abs_diff(p1.y, p2.y)
}

fn closest_coords<'a>(coords: &'a [Point], p: &Point) -> Vec<&'a Point> {
    if coords.len() == 0 {
        return vec![];
    }

    let mut ranked: Vec<(&Point, u32)> =
        coords.iter().map(|c| (c, manhattan_dist(&p, c))).collect();
    ranked.sort_by(|(_c1, dist1), (_c2, dist2)| dist1.cmp(dist2));
    let smallest_dist = ranked[0].1;
    ranked
        .iter()
        .take_while(|(_c, dist)| *dist == smallest_dist)
        .map(|(c, _dist)| *c)
        .collect()
}

fn calc_areas(coords: &[Point], max_point: &Point) -> HashMap<Point, HashSet<Point>> {
    let mut areas = HashMap::new();

    for x in 0..max_point.x + 1 {
        for y in 0..max_point.y + 1 {
            let p = Point { x, y };
            if coords.contains(&p) {
                continue;
            }
            let closest_coords = closest_coords(&coords, &p);
            if closest_coords.len() != 1 {
                continue;
            }

            areas
                .entry(*closest_coords[0])
                .and_modify(|s: &mut HashSet<Point>| {
                    s.insert(p);
                }).or_insert_with(|| {
                    let mut s = HashSet::new();
                    s.insert(p);
                    s
                });
        }
    }

    areas
}

fn filter_finite(
    areas: HashMap<Point, HashSet<Point>>,
    max_point: &Point,
) -> HashMap<Point, HashSet<Point>> {
    areas
        .into_iter()
        .filter(|(_coord, closest_points)| {
            let touches_edge = closest_points
                .iter()
                .any(|p| p.x == 0 || p.x == max_point.x || p.y == 0 || p.y == max_point.y);
            !touches_edge
        }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_areas() {
        let coords = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ];

        let max_x = coords
            .iter()
            .max_by(|p1, p2| p1.x.cmp(&p2.x))
            .map(|p| p.x)
            .unwrap();
        let max_y = coords
            .iter()
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .map(|p| p.y)
            .unwrap();
        let max_point = Point { x: max_x, y: max_y };

        let areas = calc_areas(&coords, &max_point);

        let finite_areas = filter_finite(areas, &max_point);

        assert_eq!(
            finite_areas.keys().collect::<HashSet<&Point>>(),
            vec![coords[3], coords[4]].iter().collect()
        );
    }

}