package digimon.core.skill;

import digimon.core.status.Attack;

import java.util.List;

public interface Skill {

    Attack getAttack();

    List<SkillEffect> getSkillEffects();
}
