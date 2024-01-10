use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SampleCount(pub usize);

#[derive(Resource)]
pub struct MeanCount(pub usize);
