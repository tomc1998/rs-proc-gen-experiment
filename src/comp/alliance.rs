use specs::*;

/// Types of alliances. Currently pretty basic, but could be expanded to be
/// procedurally generated with factions in the future.
pub enum AllianceType {
    Good, Evil
}

impl AllianceType {
    /// Should this entity attack an entity with the given alliance?
    pub fn attacks(&self, other_type: &AllianceType) -> bool {
        match self {
            AllianceType::Good => {
                match other_type {
                    AllianceType::Good => false,
                    AllianceType::Evil => true
                }
            }
            AllianceType::Evil => {
                match other_type {
                    AllianceType::Good => true,
                    AllianceType::Evil => false
                }
            }
        }
    }
}

/// Component to indicate an alliance
#[derive(Component)]
pub struct Alliance {
    pub alliance: AllianceType,
}

impl Alliance {
    pub fn good() -> Alliance {
        Alliance {
            alliance: AllianceType::Good
        }
    }

    pub fn evil() -> Alliance {
        Alliance {
            alliance: AllianceType::Evil
        }
    }
}
