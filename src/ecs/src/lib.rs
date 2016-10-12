#![crate_name = "amethyst_ecs"]
#![crate_type = "lib"]
#![doc(html_logo_url = "https://tinyurl.com/jtmm43a")]

extern crate specs;
extern crate time;

mod sim;

pub use specs::{Allocator, AntiStorage, CreateEntities, Entities, Entity, EntityBuilder,
                Generation, HashMapStorage, JoinIter, MaskedStorage, NullStorage, Planner, RunArg,
                Storage, SystemInfo, VecStorage, World, InsertResult, Component, Join,
                System as Processor, UnprotectedStorage, Priority};
pub use self::sim::{Simulation, SimBuilder};
