use std::fmt::{Display, Formatter};
use crate::build::WeaponClass;
use crate::unlock::unlockable::Unlockable;

/// A weapon as found in-game, without any attachments on it, if you are looking for a Blueprint/Build you might refer to [Build](crate::build::Build)
pub struct Weapon {
	name: String,
	class: WeaponClass,
	unlocks: Vec<Unlockable>
}

// Placeholder for proper weapon rendering
impl Display for Weapon {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}