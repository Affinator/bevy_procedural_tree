# bevy_procedural_tree
Procedural 3D trees for bevy - ported from the javascript ez-tree repository with some adjustments to better fit bevy and some changes to the semantics of the parameters due to personal opinion.

![Showroom example](/images/showroom.jpg)

## Features
* Mesh generation based on given TreeMeshSettings (a standard Mesh3d)
* Generation by global TreeMeshSettings or per instance (chosen per entity)
* User can provide a material for the branches and leafs separately 
* Auto regeneration of the meshes when the settings change
* Optional use of u32_indices for the mesh (default is u16; see `u32_indices` feature in Cargo.toml)

## Usage
See the showroom example: ```cargo run --example showroom --features "inspector perf_ui"```

In the showroom are two trees: The tree in the middle uses the global `TreeMeshSettings` resource. The tree to the side uses the `TreeMeshSettings` component, which can be modified on the entity itself via the inspector.

### Quick start (with TreeProceduralGenerationPlugin)
1. To enable auto generation: add the `TreeProceduralGenerationPlugin` to your app
2. (Optional) Modify the `TreeMeshSettings` and `TreeDefaultMaterials` to your liking
3. Spawn an entity and add the `Tree`component

Internally this will generate the Mesh3d for the entity and a child entity for the mesh of the leaves. It will apply the materials from the `TreeDefaultMaterials` resource, or from a provided override.

### Quick start (without TreeProceduralGenerationPlugin)
1. use `bevy_procedural_tree::meshgen::generate_tree_meshes()` to generate two meshes (branches/trunk mesh and leaves mesh)
2. use the meshes for anything you like

### Explanation of the most important structs
#### TreeMeshSettings resource
Defines the general structure of the generated 3d mesh. Every parameter is documented.

#### TreeDefaultTextures resource
Defines the default materials used by trees which do not use the override.

#### Tree component
Added to an entity to generate a new tree. It has 4 parameters:
* a seed to make this tree unique (using the same seed, with the same TreeMeshSettings produces the same tree mesh)
* an optional override for the `TreeMeshSettings` resource
* an optional override for the `TreeDefaultMaterials` bark material
* an optional override for the `TreeDefaultMaterials` leaf material

## Possible ToDos
* Do not regenerate the whole tree each time the settings change (but do partial updates)
* Provide an example vertex shader for wind
* Implement "growing"
* Caching of already generated trees (i.e. with the lru crate)
* Multiple LODs
* Different "normal" modes (currently just orthogonal to the surface; i.e. inspiration: [Reddit: Fluffy trees](https://www.reddit.com/r/Unity3D/comments/jhwfkj/fluffy_trees_using_custom_shader_that_turns_quad/))

## Future research
* How to generalize materials to not force the user to provide a StandardMaterial

## Supported Bevy Versions

| Bevy    | bevy_procedural_tree |
| ------- | ----- |
| 0.16    | 0.1   |

## Acknowledgements
* https://github.com/dgreenheck/ez-tree
* https://ambientcg.com
* https://polyhaven.com
