/*
* Inspiration taken with great thanks from: https://github.com/dgreenheck/ez-tree
*/

use bevy::prelude::*;

#[cfg(feature="inspector")]
use bevy_inspector_egui::prelude::*;

use crate::enums::{BarkType, LeafBillboard, LeafType, TreeType};


#[cfg(feature="inspector")]
#[derive(Resource, Reflect, InspectorOptions, Debug, Clone, PartialEq)]
#[reflect(Resource, InspectorOptions)]
pub struct TreeSettings {
    pub seed: u32,
    pub tree_type: TreeType,
    pub bark: BarkParams,
    pub branch: BranchParams,
    pub leaves: LeafParams,
}


#[cfg(not(feature="inspector"))]
#[derive(Resource, Reflect, Debug, Clone, PartialEq)]
#[reflect(Resource)]
pub struct TreeSettings {
    pub seed: u32,
    pub tree_type: TreeType,
    pub bark: BarkParams,
    pub branch: BranchParams,
    pub leaves: LeafParams,
}


impl Default for TreeSettings {
    fn default() -> Self {
        Self {
            seed: 0,
            tree_type: TreeType::Deciduous,
            bark: BarkParams::default(),
            branch: BranchParams::default(),
            leaves: LeafParams::default(),
        }
    }
}






#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct BarkParams {
    pub bark_type: BarkType,  
    pub tint: Color,
    pub flat_shading: bool,
    pub textured: bool,
    pub texture_scale: Vec2,
}

impl Default for BarkParams {
    fn default() -> Self {
        Self {
            bark_type: BarkType::Oak,
            tint: Color::WHITE,
            flat_shading: false,
            textured: true,
            texture_scale: Vec2 { x: 1.0, y: 1.0 },
        }
    }
}


/**
 * All branches have a random angle to their parent branch/trunk.
 * This branch force controls a direction vector and an amount to lerp between the random direction and this vector by the given strength.
 * This can be used i.e. for trees that generally have branches that point in a specific direction (i.e. up:Aspen or down:Willow).
 */
#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct BranchForce {
    pub direction: Vec3,
    pub strength: f32,
}

impl Default for BranchForce {
    fn default() -> Self {
        Self {
            direction: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            strength: 0.01,
        }
    }
}

/**
 * amount of recursion for branches (0 = only trunk, no branches)
 */
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BranchRecursionLevel {
    Zero = 0,
    One  = 1,
    Two  = 2,
    Three= 3,
    Four = 4,
}

impl TryFrom<u8> for BranchRecursionLevel {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(BranchRecursionLevel::Zero),
            1 => Ok(BranchRecursionLevel::One),
            2 => Ok(BranchRecursionLevel::Two),
            3 => Ok(BranchRecursionLevel::Three),
            4 => Ok(BranchRecursionLevel::Four),
            _ => Err(()),
        }
    }
}

impl From<BranchRecursionLevel> for u8 {
    fn from(z: BranchRecursionLevel) -> u8 { z as u8 }
}

impl From<BranchRecursionLevel> for usize {
    fn from(z: BranchRecursionLevel) -> usize { z as usize }
}


#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct BranchParams {
    /// amount of recursion for branches (0 = only trunk, no branches)
    pub levels: BranchRecursionLevel,

    /// angle of child branch(es) to parent branch/trunk per level 0..3
    pub angle: [f32; 4],

    /// amount of children per level 0..3
    pub children: [u8; 4],

    /// Control the general direction of branches
    pub force: BranchForce,

    /// curling/twisting per level
    pub gnarliness: [f32; 4],

    /// length per level
    pub length: [f32; 4],

    /// radius per level
    pub radius: [f32; 4],

    /// how many sections each brach has per level (along its length; more sections = more polygons)
    pub sections: [u8; 4],

    /// how many segments each branch has per section per level (how 'round' the mesh is; more segments = more polygons)
    pub segments: [u8; 4],

    /// when to start adding child branches along the length of the branch (0..1) per level
    pub start: [f32; 4],

    /// taper per level
    pub taper: [f32; 4],

    /// twist per Level
    pub twist: [f32; 4],
}

impl Default for BranchParams {
    fn default() -> Self {
        Self {
            levels: BranchRecursionLevel::Three,
            angle: [0.0, 70.0, 60.0, 60.0],
            children: [7, 7, 5, 2],
            force: BranchForce::default(),
            gnarliness: [0.15, 0.20, 0.30, 0.02],
            length: [3.0, 2.0, 1.5, 0.5],
            radius: [0.5, 0.3, 0.3, 0.2],
            sections: [12, 10, 8, 6],
            segments: [8, 6, 4, 3],
            start: [0.0, 0.4, 0.3, 0.3],
            taper: [0.7, 0.7, 0.7, 0.7],
            twist: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

/**
 * Leaves are only added to the last level of branches.
 * Control how they look like and how they are positioned relative to the last level of branches (or on the trunk if levels = 0).
 */
#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct LeafParams {
    /// leaf texture to use
    pub leaf_type: LeafType,
    /// single or double/perpendicular
    pub leaf_billboard: LeafBillboard,
    /// angle of leaves relative to parent branch/trunk in degrees
    pub angle: f32,
    /// amount of leaves
    pub count: u32,
    /// when leaves start relative to the length of the branch (0..1)
    pub start: f32,
    /// average size of leaves
    pub size: f32,
    /// variance of leaf sizes 
    pub size_variance: f32,
    /// tint color for leaves
    pub tint: Color,
    /// alpha-test-threshold (0..1)
    pub alpha_test: f32,
}

impl Default for LeafParams {
    fn default() -> Self {
        Self {
            leaf_type: LeafType::Oak,
            leaf_billboard: LeafBillboard::Double,
            angle: 10.0,
            count: 1,
            start: 0.0,
            size: 2.5,
            size_variance: 0.7,
            tint: Color::WHITE,
            alpha_test: 0.5,
        }
    }
}

