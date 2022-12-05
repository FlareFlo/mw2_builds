use crate::attachment::{Attachment, AttachmentType};
use crate::weapon::Weapon;

/// Type of item unlocked per weapon-level
pub enum Unlockable {
	Attachment(Attachment),
	Category(AttachmentType),
	Weapon(Weapon),
}