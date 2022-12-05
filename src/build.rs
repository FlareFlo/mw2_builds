use crate::attachment::Attachment;
use crate::weapon::Weapon;

pub struct Build {
	base: Weapon,
	attachments: [Attachment; 5],
}

pub struct Blueprint {
	name: String,
	build: Build,
}

pub enum WeaponClass {
	AssaultRifle,
	BattleRifle,
	SMG,
	Shotgun,
	LMG,
	MarksmanRifle,
	SniperRifle,
	Melee,
}