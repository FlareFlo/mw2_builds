use crate::weapon::Weapon;

pub enum UnlockType {
	PlayerLevel(usize),
	Weapon(Weapon),
	Custom(String),
	Unknown,
}