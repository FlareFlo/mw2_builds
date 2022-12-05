use crate::weapon::Weapon;

pub struct Attachment {
	variant: AttachmentType,
	// Not every part has tuning
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
	Guard,
	RearGrip,
	Grip,
	Stock,
}

// Not sure how to represent this, floats are probably best right now
pub struct Tuning {
	x: f64,
	y: f64,
}