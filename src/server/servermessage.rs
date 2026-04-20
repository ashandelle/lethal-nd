use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReliableServerMessage {
    // CreatePlayer {
    //     entity: Entity,
    //     translation: [f32; 3],
    //     rotation: f32,
    //     name: String,
    // },
    // RemoveEntities {
    //     entities: Vec<Entity>,
    // },
    // Move {
    //     entity: Entity,
    //     translation: [f32; 3],
    //     rotation: f32,
    // },
}