use crate::domain::status::hit_point::hit_point::HitPoint;

mod domain;

fn main() {
    let hit_point = HitPoint::build(100);
    println!("{:#?}", hit_point)
}
