use bevy::prelude::*;
use crate::players::info;
use crate::animation::play;

pub trait Movement {
    fn movement(
        &self,
        p: &mut info::Player,
        player: &mut AnimationPlayer,
        animations: play::CharacterAnimations,
    );
}

pub enum Tier {
    Basic,        // High # clicks, low impact_radius, low impact_extent.
    Intermediate, // High # clicks, high impact_radius, low impact_extent.
    Advanced,     // Low # clicks, low impact_radius, high impact_extent.
    God,          // Low # clicks, high impact_radius, high impact_extent.
}

pub enum Effect {
    Positive,
    Negative,
}

pub enum PowerType {
    HealthManip,    // Damage or increase health
    TranformManip,  // Change transform
    AnimationManip, // Change animation
    Object,         // Spawn object
}

pub enum Medium {
    Guesture,
    Weapon,
}

pub enum Affected {
    Me,
    Other,
}