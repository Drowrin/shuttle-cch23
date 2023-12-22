use std::collections::HashMap;

use axum::{routing, Router};
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use sqlx::PgPool;

async fn find_unpaired(body: String) -> String {
    "🎁".repeat(
        body.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .fold(0, |acc, val| acc ^ val),
    )
}

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt()
    }
}

async fn rocket(body: String) -> String {
    let mut iter = body.lines();
    let num_stars: usize = iter.next().unwrap().parse().unwrap();
    let stars: Vec<Point> = iter
        .by_ref()
        .take(num_stars)
        .map(|line| line.split(" ").collect_tuple().unwrap())
        .map(|(x, y, z)| Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
        .collect();
    let num_portals: usize = iter.next().unwrap().parse().unwrap();
    let mut portals: HashMap<usize, Vec<usize>> = HashMap::new();
    for (from, to) in iter
        .take(num_portals)
        .map(|line| line.split(" ").collect_tuple().unwrap())
        .map(|(from, to)| (from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap()))
    {
        portals.entry(from).or_default().push(to);
        portals.entry(to).or_default().push(from);
    }

    let src: usize = 0;
    let dst: usize = num_stars - 1;

    let (path, _) = dijkstra(
        &src,
        |p| portals[p].iter().map(|v| (*v, 1)).collect::<Vec<_>>(),
        |p| *p == dst,
    )
    .unwrap();

    let distance: f32 = path
        .iter()
        .tuple_windows()
        .map(|(f, t)| stars[*f].distance(&stars[*t]))
        .sum();

    format!("{} {:.3}", path.len() - 1, distance)
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/22/integers", routing::post(find_unpaired))
        .route("/22/rocket", routing::post(rocket))
}
