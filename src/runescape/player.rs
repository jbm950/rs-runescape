use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
pub struct Player {
    pub skills: Vec<Skill>,
}

#[derive(Clone, Default, Deserialize)]
pub struct Skill {
    name: String,
    level: u64,
    xp: u64,
}

impl Player {
    pub fn iter_levels(&self) -> impl Iterator<Item = (&str, u64)> {
        self.skills
            .iter()
            .filter(|skill| skill.name != "Overall")
            .map(|skill| (skill.name.as_str(), skill.level))
    }

    pub fn iter_xp(&self) -> impl Iterator<Item = (&str, u64)> {
        self.skills
            .iter()
            .filter(|skill| skill.name != "Overall")
            .map(|skill| (skill.name.as_str(), skill.xp))
    }
}
