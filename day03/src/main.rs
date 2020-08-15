use std::cmp;
use std::fs;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

struct Segment {
    start: Point,
    end: Point,
}

fn get_intersection_point(segment1: &Segment, segment2: &Segment) -> Option<Point> {
    let a1 = segment1.end.y - segment1.start.y;
    let b1 = segment1.start.x - segment1.end.x;
    let c1 = a1 * (segment1.start.x) + b1 * (segment1.start.y);
    let a2 = segment2.end.y - segment2.start.y;
    let b2 = segment2.start.x - segment2.end.x;
    let c2 = a2 * (segment2.start.x) + b2 * (segment2.start.y);
    let determinant = a1 * b2 - a2 * b1;
    if determinant == 0 {
        return None;
    }
    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    let seg1_x_max = cmp::max(segment1.start.x, segment1.end.x);
    let seg1_x_min = cmp::min(segment1.start.x, segment1.end.x);
    let seg1_y_max = cmp::max(segment1.start.y, segment1.end.y);
    let seg1_y_min = cmp::min(segment1.start.y, segment1.end.y);
    let seg2_x_max = cmp::max(segment2.start.x, segment2.end.x);
    let seg2_x_min = cmp::min(segment2.start.x, segment2.end.x);
    let seg2_y_max = cmp::max(segment2.start.y, segment2.end.y);
    let seg2_y_min = cmp::min(segment2.start.y, segment2.end.y);
    if x >= seg1_x_min
        && x <= seg1_x_max
        && x >= seg2_x_min
        && x <= seg2_x_max
        && y >= seg1_y_min
        && y <= seg1_y_max
        && y >= seg2_y_min
        && y <= seg2_y_max
    {
        return Some(Point::new(x, y));
    }
    None
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut wire_segments = Vec::new();

    for line in content.lines() {
        let mut segments = Vec::new();
        let mut start_point = Point::new(0, 0);
        for value in line.split(',') {
            let direction = value.chars().next().unwrap();
            let steps = &value[1..].parse::<i64>().unwrap();

            let end_point = match direction {
                'U' => Point::new(start_point.x, start_point.y + steps),
                'D' => Point::new(start_point.x, start_point.y - steps),
                'L' => Point::new(start_point.x - steps, start_point.y),
                'R' => Point::new(start_point.x + steps, start_point.y),
                _ => panic!(),
            };
            segments.push(Segment {
                start: start_point,
                end: end_point,
            });
            start_point = end_point;
        }
        wire_segments.push(segments);
    }

    let calculate_length = |segment: &Segment| {
        let len_x = (segment.start.x - segment.end.x).abs();
        let len_y = (segment.start.y - segment.end.y).abs();
        len_x + len_y
    };

    let calculate_distance = |p1: &Point, p2: &Point| (p1.x - p2.x).abs() + (p1.y - p2.y).abs();

    let mut min_distance = std::i64::MAX;
    let mut min_steps = std::i64::MAX;
    let mut wire1_steps = 0;
    let mut wire2_steps;

    for wire1_segment in wire_segments[0].iter() {
        wire2_steps = 0;
        wire1_steps += calculate_length(&wire1_segment);

        for wire2_segment in wire_segments[1].iter() {
            wire2_steps += calculate_length(&wire2_segment);
            if let Some(intersection) = get_intersection_point(&wire1_segment, &wire2_segment) {
                let distance = intersection.x.abs() + intersection.y.abs();
                if distance < min_distance && distance != 0 {
                    let dist1_end = calculate_distance(&intersection, &wire1_segment.end);
                    let dist2_end = calculate_distance(&intersection, &wire2_segment.end);
                    min_distance = distance;

                    let steps = wire1_steps + wire2_steps - dist1_end - dist2_end;
                    if steps < min_steps {
                        min_steps = steps;
                    }
                }
            }
        }
    }

    assert_eq!(min_distance, 489);
    assert_eq!(min_steps, 93_654);
}
