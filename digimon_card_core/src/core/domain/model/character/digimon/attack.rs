use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::fight::damage::DamageBuilder;
use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::status::attack::AttackOrdinal;

impl Digimon {
    pub fn attack(&self, attack_ordinal: &AttackOrdinal, opponent_digimon: &Digimon) -> Digimon {
        let attribute_effects = Effects::of(&self.attribute, &opponent_digimon.attribute);
        let damage = DamageBuilder::new()
            .attack(attack_ordinal.to_attack(&self))
            .effects(attribute_effects)
            .build();

        let opponent_hit_point = opponent_digimon.hit_point.clone().damaged(damage);

        Digimon {
            name: opponent_digimon.name.clone(),
            attribute: opponent_digimon.attribute.clone(),
            hit_point: opponent_hit_point,
            primary_attack: opponent_digimon.primary_attack.clone(),
            secondary_attack: opponent_digimon.secondary_attack.clone(),
            tertiary_attack: opponent_digimon.tertiary_attack.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::status::attack::{Attack, AttackOrdinal};
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

    #[test]
    fn test_attack() {
        let actual = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint::value_of(600),
            primary_attack: Attack::value_of(300),
            secondary_attack: Attack::value_of(200),
            tertiary_attack: Attack::value_of(100),
        }
            .attack(
                &AttackOrdinal::SECONDARY,
                &Digimon {
                    name: "ガブモン".to_string(),
                    attribute: Attribute::DATA,
                    hit_point: HitPoint::value_of(700),
                    primary_attack: Attack::value_of(250),
                    secondary_attack: Attack::value_of(180),
                    tertiary_attack: Attack::value_of(90),
                });

        let expected = Digimon {
            name: "ガブモン".to_string(),
            attribute: Attribute::DATA,
            hit_point: HitPoint {
                value: 600,
                max: 700,
                min: 0,
            },
            primary_attack: Attack::value_of(250),
            secondary_attack: Attack::value_of(180),
            tertiary_attack: Attack::value_of(90),
        };

        assert_eq!(actual, expected)
    }
}