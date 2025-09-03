use core::f32;
use std::f32::consts::PI;

use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use fastrand::Rng;

use crate::{enums::TreeType, settings::{BranchRecursionLevel, TreeSettings}};

#[derive(Debug, Clone)]
struct BranchGenState {
    pub origin: Vec3,
    pub orientation: Quat,
    pub length: f32,
    pub start_radius: f32,
    pub taper: f32,
    pub twist: f32,
    pub gnarliness: f32,
    pub level: usize,
    pub recursion_count: usize,
    pub sections: usize,
    pub segments: usize,
}

#[derive(Debug, Clone)]
struct SectionData {
    pub origin: Vec3,
    pub orientation: Quat,
    pub radius: f32
}

pub(crate) fn generate_branches(settings: &TreeSettings, rng: &mut Rng) -> Mesh { 
    let state: BranchGenState = BranchGenState {
        origin: Vec3::ZERO,
        orientation: Quat::IDENTITY,
        length: settings.branch.length[0],
        start_radius: settings.branch.trunk_base_radius,
        taper: settings.branch.taper[0],
        twist: settings.branch.twist[0],
        gnarliness: settings.branch.gnarliness[0],
        level: 0,
        recursion_count: 0,
        sections: settings.branch.sections[0] as usize,
        segments: settings.branch.segments[0] as usize,
    };
    generate_branches_internal(settings, state, rng)
}

fn generate_branches_internal(settings: &TreeSettings, state: BranchGenState, rng: &mut Rng) -> Mesh { 
    // Allocate mesh attributes
    let mut positions: Vec<[f32; 3]> = Vec::new(); //with_capacity(rings * ring_stride);
    let mut normals:   Vec<[f32; 3]> = Vec::new();  //with_capacity(rings * ring_stride);
    let mut uvs:       Vec<[f32; 2]> = Vec::new(); //with_capacity(rings * ring_stride);
    let mut colors:    Vec<[f32; 4]> = Vec::new(); //with_capacity(rings * ring_stride);
    let mut indices: Vec<u16> = Vec::new(); //with_capacity(state.sections * state.segments * 6);

    recurse_a_branch(settings, state, rng, &mut positions, &mut normals, &mut uvs, &mut indices, &mut colors);
    
    // build mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U16(indices));

    mesh
}

#[allow(clippy::too_many_arguments)]
fn recurse_a_branch(
    settings: &TreeSettings,
    state: BranchGenState,
    rng: &mut Rng,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u16>,
    colors: &mut Vec<[f32; 4]>
) 
{       
    let indices_start: u16 = positions.len() as u16;
    // local section storage    
    let mut sections: Vec<SectionData> = Vec::with_capacity(state.sections);
    
    // calculate the length of each section (one vertical ring)
        
    // give the different parts of a Deciduous branch a different length based on the level (lower level = more length)
    // the sum should be equal to the target length (state.length for Deciduous trunks; at level 0)
    // target formula: (max_level - current_level + 1) / sum of (possible_levels+1)
    let target_pieces: f32 = (1..=(settings.branch.levels as usize+1)).sum::<usize>() as f32;
    let factor_for_length: f32 = if state.level > 0 {1.0} else {
        match settings.tree_type {
            TreeType::Deciduous => (settings.branch.levels as usize - state.recursion_count + 1) as f32 / target_pieces,
            TreeType::Evergreen => 1.0,
        }
    };

    let section_length = state.length / state.sections as f32 * factor_for_length;

    // create state for iterations
    let mut section_orientation = state.orientation;
    let mut section_origin = state.origin;
    let mut section_radius = state.start_radius;

    // precalculate force quat (to not do it in the loop)
    // TODO would be even faster if done one level higher (i.e. in the state)
    let branch_force_dir = if settings.branch.force.direction.length_squared() >= f32::EPSILON {
        settings.branch.force.direction.normalize()
    } else {
        Vec3::Y
    };
    let branch_force_quat = Quat::from_rotation_arc(Vec3::Y, branch_force_dir);

    // amount of taper to add per each section to reach target taper; each level may have a different target taper
    // for Evergreen we need 'sections' steps, so that at the top we have the target taper
    // for Deciduous we need even more steps, due to the trunk being build from sections*levels parts
    let taper_amount_per_section = match settings.tree_type {
        TreeType::Deciduous => f32::powf(1.0 - state.taper, (1.0/state.sections as f32) / (f32::from(settings.branch.levels) + 1.0)),
        TreeType::Evergreen => f32::powf(1.0 - state.taper, 1.0/state.sections as f32), 
    };
    
    // iterate over sections + one final ring
    // the =sections is needed because to have x sections, we need x+1 rows of vertices
    for section_counter in 0..=state.sections {
        // update radius
        if section_counter == state.sections && ((state.recursion_count == settings.branch.levels as usize) || matches!(settings.tree_type, TreeType::Evergreen)) {
            // last ring of the last section of the last level is a tip
            section_radius = f32::EPSILON;
        } 
    
        // save the first vertex to create a ring in the end
        let mut first_pos = Vec3::ZERO;
        let mut first_nrm = Vec3::ZERO;
        let mut first_v  = 0.0;
    
        // for each segment create a single vertex
        for segment_counter in 0..state.segments {
            let angle = (2.0 * PI * segment_counter as f32) / state.segments as f32;
            let (sin, cos) = angle.sin_cos();    
            
            let local_pos = Vec3::new(cos * section_radius, 0.0, sin * section_radius);
            let local_normal = Vec3::new(cos, 0.0, sin);
    
            let vertex = (section_orientation * local_pos) + section_origin;
            let normal = (section_orientation * local_normal).normalize();
    
            let u = segment_counter as f32 / state.segments as f32;
            let v = if section_counter % 2 == 0 { 0.0 } else { 1.0 };
        
            if segment_counter == 0 {
                first_pos = vertex;
                first_nrm = normal;
                first_v = v;
            }            
            positions.push(vertex.to_array());
            normals.push(normal.to_array());
            uvs.push([u,v]);
            // color code levels for debugging
            match BranchRecursionLevel::try_from(state.recursion_count as u8).unwrap() {
                BranchRecursionLevel::Zero => colors.push([1.0, 0.0, 0.0, 1.0]),
                BranchRecursionLevel::One => colors.push([0.0, 1.0, 0.0, 1.0]),
                BranchRecursionLevel::Two => colors.push([0.0, 0.0, 1.0, 1.0]),
                BranchRecursionLevel::Three => colors.push([0.0, 1.0, 1.0, 1.0]),
                //BranchRecursionLevel::Four => colors.push([1.0, 1.0, 1.0, 1.0]),
            }
            
        } // END for each segment
    
        // duplicate of the first vertex to create a full ring (with different uv)
        positions.push(first_pos.to_array());
        normals.push(first_nrm.to_array());
        uvs.push([1.0, first_v]);
        // color code levels for debugging
        match BranchRecursionLevel::try_from(state.recursion_count as u8).unwrap() {
            BranchRecursionLevel::Zero => colors.push([1.0, 0.0, 0.0, 1.0]),
            BranchRecursionLevel::One => colors.push([0.0, 1.0, 0.0, 1.0]),
            BranchRecursionLevel::Two => colors.push([0.0, 0.0, 1.0, 1.0]),
            BranchRecursionLevel::Three => colors.push([0.0, 1.0, 1.0, 1.0]),
            //BranchRecursionLevel::Four => colors.push([1.0, 1.0, 1.0, 1.0]),
        }
    
        // save section data for later allow branches to grow from them
        sections.push(SectionData {
            origin: section_origin,
            orientation: section_orientation,
            radius: section_radius
        });
    
        //
        // Update section parameters for next section
        //
        if section_counter < state.sections {
            // Gnarliness: random tilt around x and z
            let gn = state.gnarliness * 0.4 / section_radius.sqrt(); // 0.4 chosen by trial and error to look natural (values between 0..1 make the most sense now; larger is still possible)
            let dx = (rng.f32() - 0.5) * gn;
            let dz = (rng.f32() - 0.5) * gn;
            let q_gnarl = Quat::from_euler(EulerRot::XYZ, dx, 0.0, dz);

            // twist around y-axis
            let q_twist = Quat::from_axis_angle(Vec3::Y, state.twist);

            // apply gnarl and twist
            section_orientation = (q_gnarl * section_orientation) * q_twist;

            // slerp the target orientation in the direction of the branch.force based on the given strength and radius of the branch
            let radius_factor = 1.0 - (section_radius / settings.branch.force.radius_cutoff).clamp(0.0, 1.0);
            let strength_per_radius = (settings.branch.force.strength * radius_factor / 2.0).clamp(0.0, 1.0); // 2 chosen by trial and error to look natural (values between 0..1 make the most sense now; larger is still possible)
            section_orientation = section_orientation.slerp(branch_force_quat, strength_per_radius);

            // taper
            section_radius *= taper_amount_per_section;

            // direction (go along the branch)
            let up = section_orientation * Vec3::Y;
            section_origin += up * section_length;
        }        
    } // END for each section
    
    // Indices (triangles) are build around the ring per segment
    let ring_stride: u16 = state.segments as u16 + 1;    
    for i in 0..state.sections as u16 {
        for j in 0..state.segments as u16 {
            let a: u16 = i * ring_stride        + j         + indices_start;
            let b: u16 = i * ring_stride        + (j + 1)   + indices_start;
            let c: u16 = a + ring_stride;
            let d: u16 = b + ring_stride;
    
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }    

    if matches!(settings.tree_type, TreeType::Deciduous) && state.level == 0 {
        if state.recursion_count < settings.branch.levels as usize {
            // Deciduous trunks are build itnernally from multiple continous branches (for nicer branch generation)
            let additional_trunk_part = BranchGenState {
                origin: section_origin,
                orientation: section_orientation,
                length: state.length, 
                start_radius: section_radius,
                taper: state.taper,
                twist: state.twist,
                gnarliness: state.gnarliness,
                level: state.level,
                recursion_count: state.recursion_count + 1,
                // Section count and segment count must be same as parent branch
                // since the child branch is growing from the end of the parent branch           
                sections: state.sections,
                segments: state.segments,
            };
            recurse_a_branch(settings, additional_trunk_part, rng, positions, normals, uvs, indices, colors);
        }
        else {
            // TODO generate leaves
        }
    }

    if state.recursion_count == settings.branch.levels as usize {
        // TODO generate leaves
    }
    else {
        for child_branch_state in generate_child_branches(
            settings.branch.children[state.recursion_count],
            state.recursion_count + 1,
            &sections,
            settings,
            rng
        ) {
            recurse_a_branch(settings, child_branch_state, rng, positions, normals, uvs, indices, colors);
        }
    }
}



fn generate_child_branches (
    count: u8,
    level: usize,
    parent_sections: &[SectionData],
    settings: &TreeSettings,
    rng: &mut Rng,
) -> Vec<BranchGenState> {
    if count == 0 || parent_sections.is_empty(){
        return Vec::new();
    }

    let radial_offset: f32 = rng.f32(); 
    let section_count_minus_one: usize = parent_sections.len().saturating_sub(1);  

    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        // lowest start position along the parent branch as a factor
        let child_start_factor = f32::lerp(settings.branch.start[level], 1.0, rng.f32());

        // calculate a factor between two sections based on the possible range
        let child_branch_pos = child_start_factor * section_count_minus_one as f32;
        let section_index = child_branch_pos.floor() as usize;
        let branch_height_factor = (child_branch_pos - section_index as f32).clamp(0.0, 1.0);

        // calculate target sections where to place the branch
        let section_a_index = section_index; 
        let section_b_index = (section_index + 1).min(section_count_minus_one);
        let section_a = &parent_sections[section_a_index];
        let section_b = &parent_sections[section_b_index];

        // interpolate the placement between section a and b
        let child_branch_origin = section_a.origin.lerp(section_b.origin, branch_height_factor);

        // calculate radius
        // TODO is this correct?
        let radius_setting = settings.branch.radius_factor[level];
        let parent_radius = f32::lerp(section_a.radius, section_b.radius, branch_height_factor);
        let child_branch_radius = radius_setting * parent_radius;

        // orient along the parent sections
        let parent_orientation = section_b.orientation.slerp(section_a.orientation, branch_height_factor);

        // calculate needed angles 
        let radial_angle = 2.0 * std::f32::consts::PI * (radial_offset + (i as f32) / (count as f32));
        let angle_rad = settings.branch.angle[level].to_radians();
        let q1 = Quat::from_axis_angle(Vec3::X, angle_rad);
        let q2 = Quat::from_axis_angle(Vec3::Y, radial_angle);
        let child_quat = parent_orientation * q2 * q1;

        // target length
        let mut child_len = settings.branch.length[level];
        if settings.tree_type == TreeType::Evergreen {
            child_len *= 1.0 - child_start_factor;
        }

        out.push(BranchGenState {
            origin: child_branch_origin,
            orientation: child_quat,
            length: child_len,
            start_radius: child_branch_radius,
            level,
            recursion_count: level,
            taper: settings.branch.taper[level],
            twist: settings.branch.twist[level],
            gnarliness: settings.branch.gnarliness[level],
            sections: settings.branch.sections[level].into(),
            segments: settings.branch.segments[level].into()
        });
    }

    out
}
