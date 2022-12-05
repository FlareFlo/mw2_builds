use std::fmt::{Display, Formatter};
use crate::weapon::Weapon;

/// Type of requirement needed to unlock
pub enum UnlockRequirement {
	PlayerLevel(usize),
	Weapon(Weapon, usize),
	Custom(String),
	Unknown,
}

impl Display for UnlockRequirement {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}",
			match self {
				UnlockRequirement::PlayerLevel(level) => {
					format!("Your player level needs to be a minimum of {level}")
				}
				UnlockRequirement::Weapon(weapon, level) => {
					format!("You have to progress {} to level {}", weapon.to_string(), level)
				}
				UnlockRequirement::Custom(custom_text) => {
					custom_text.clone()
				}
				UnlockRequirement::Unknown => {
					"Unknown unlock requirement".to_owned()
				}
			}
		)
	}
}