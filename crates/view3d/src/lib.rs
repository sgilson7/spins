//! spins_view3d: the 3D view of Spins, in the look of Protection of the
//! Spheres. It reads the table the core describes (the same `TableView` JSON
//! the 2D views draw) and builds WebGL instances from it, in Q16 integers.
//! It decides nothing about the game: every number it shows came from the
//! core. Held to the core's rules by `view3d_is_held_to_the_core_rules`.
//!
//! `fx`, `geom`, `mesh` and `render` are copied from POTS (`sgilson7/pots`).

pub mod fx;
pub mod geom;
pub mod mesh;
pub mod render;
pub mod scene;
