use crate::digimon::core::domain::model::status::hit_point::hit_point::HitPoint;

mod digimon;

fn main() {
    let hit_point = HitPoint::build(100);
    println!("{:#?}", hit_point)
}
