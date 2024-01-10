use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Sample {
    pub x: f32,
    pub y: f32,
}

impl Sample {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Mean {
    pub x: f32,
    pub y: f32,
    pub last_tick: f32,
}

impl Mean {
    pub fn new(x: f32, y: f32, last_tick: f32) -> Self {
        Self { x, y, last_tick }
    }
}
