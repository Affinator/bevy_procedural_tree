# Rough sketch:
    - DONE create showroom
    - DONE create single tree
    - Try to modify existing tree instead of regenerating everything
    - Implement "growing"
    - DONE allow creating multiple trees
    - allow managing tree generation (incl. caching already generated trees)
    - TODO how to generalize bark and leaf materials (to not force StandardMaterial)

# Misc:
- only the last level branches get their tip closed -> change to every level
- UVs are per section/segment, not by surface area -> calculate and fix
- DONE probably update to u32 indices for more complex trees
- Allow separate "normal" modes: one from the generation, one just pointing away outside (fluffy), etc
- resolution (LOD) of meshes should be flexible
    - regen the meshes if the desired resolution changes