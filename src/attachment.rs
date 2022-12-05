use crate::weapon::Weapon;

pub struct Attachment {
	variant: AttachmentType,
	tuning: Option<Tuning>,
}

pub enum AttachmentType {
	Muzzle,
	Barrel,
	Optic,
	Underbarrel,
	Receiver,
	Laser,
	Magazine,
}

pub struct Tuning {
	x: f64,
	y: f64,
}