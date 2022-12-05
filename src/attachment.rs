use crate::wepon::Weapon;

pub struct Attachment {
	variant: AttachmentType,
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