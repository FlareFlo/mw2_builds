use std::fmt::{Display, Formatter};
use crate::weapon::Weapon;

pub enum UnlockType {
	PlayerLevel(usize),
	Weapon(Weapon, usize),
	Custom(String),
	Unknown,
}

impl Display for UnlockType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}",
			match self {
				UnlockType::PlayerLevel(level) => {
					format!("Your player level needs to be a minimum of {level}")
				}
				UnlockType::Weapon(weapon, level) => {
					format!("You have to progress {} to level {}", weapon.to_string(), level)
				}
				UnlockType::Custom(custom_text) => {
					custom_text.clone()
				}
				UnlockType::Unknown => {
					"Unknown unlock requirement".to_owned()
				}
			}
		)
	}
}