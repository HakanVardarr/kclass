use super::resources::SampleCount;
use crate::{
    components::{Mean, Sample},
    resources::MeanCount,
    COLORS,
};
use bevy::{app::AppExit, prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};
pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::rgb(0.01, 0.001, 0.2)))
            .add_systems(Update, handle_exit);
    }
}

fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        writer.send(AppExit)
    }
}

// Sample Plugin

pub struct SamplePlugin {
    sample_count: usize,
}

impl SamplePlugin {
    pub fn new(sample_count: usize) -> Self {
        Self { sample_count }
    }
}

impl Plugin for SamplePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SampleCount(self.sample_count))
            .add_systems(Startup, generate_samples)
            .add_systems(Update, reset_samples);
    }
}

fn generate_samples(
    sample_count: Res<SampleCount>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..sample_count.0 {
        let x = thread_rng().gen_range(-400.0..400.0);
        let y = thread_rng().gen_range(-300.0..300.0);
        let z = thread_rng().gen_range(0.0..300.0);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(3.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW_GREEN)),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            Sample::new(x, y),
        ));
    }
}

fn reset_samples(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sample), With<Sample>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut sample) in &mut query {
            let x = thread_rng().gen_range(-400.0..400.0);
            let y = thread_rng().gen_range(-300.0..300.0);
            transform.translation.x = x;
            transform.translation.y = y;
            sample.x = x;
            sample.y = y;
        }
    }
}

// Means Plugin

pub struct MeansPlugin {
    mean_count: usize,
}

impl MeansPlugin {
    pub fn new(mean_count: usize) -> Self {
        Self { mean_count }
    }
}

impl Plugin for MeansPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeanCount(self.mean_count))
            .add_systems(PostStartup, generate_means)
            .add_systems(Update, reset_means)
            .add_systems(Update, update_means_and_samples);
    }
}

fn generate_means(
    mean_count: Res<MeanCount>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..mean_count.0 {
        let x = thread_rng().gen_range(-400.0..400.0);
        let y = thread_rng().gen_range(-300.0..300.0);
        let z = 300.0;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(0.).into()).into(),
                material: materials.add(ColorMaterial::from(COLORS[i])),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            Mean::new(x, y, 0.0),
        ));
    }
}

fn reset_means(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Mean), With<Mean>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transform, mut mean) in &mut query {
            let x = thread_rng().gen_range(-400.0..400.0);
            let y = thread_rng().gen_range(-300.0..300.0);
            transform.translation.x = x;
            transform.translation.y = y;
            mean.x = x;
            mean.y = y;
        }
    }
}

fn update_means_and_samples(
    mean_count: Res<MeanCount>,
    sample_query: Query<
        (&mut Transform, &mut Sample, &Handle<ColorMaterial>),
        (With<Sample>, Without<Mean>),
    >,
    mut mean_query: Query<(&mut Transform, &mut Mean), (With<Mean>, Without<Sample>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut counts = vec![vec![]; mean_count.0];
    for (_, (_, sample, handle)) in sample_query.iter().enumerate() {
        let mut smallest = f32::MAX;
        let mut idx = -1;

        for (id, (_, mean)) in mean_query.iter().enumerate() {
            let distance = ((mean.x - sample.x).powf(2.0) + (mean.y - sample.y).powf(2.0)).sqrt();
            if distance < smallest {
                smallest = distance;
                idx = id as i32;
            }
        }
        let new_material = materials.get_mut(handle).unwrap();
        new_material.color = COLORS[idx as usize];

        counts[idx as usize].push(sample);
    }

    for (id, count) in counts.iter().enumerate() {
        if count.len() == 0 {
            if let Some((mut transform, mut mean)) = mean_query.iter_mut().nth(id) {
                if mean.last_tick == 50.0 {
                    transform.translation.x = 1920.0 / 2.0;
                    transform.translation.y = 1080.0 / 2.0;
                } else {
                    let x = thread_rng().gen_range(-400.0..400.0);
                    let y = thread_rng().gen_range(-300.0..300.0);
                    transform.translation.x = x;
                    transform.translation.y = y;
                    mean.x = x;
                    mean.y = y;
                    mean.last_tick += 1.0;
                }
            }
        } else {
            if let Some((mut transform, mut mean)) = mean_query.iter_mut().nth(id) {
                let mut x = 0.0;
                let mut y = 0.0;

                for &s in count {
                    x += s.x;
                    y += s.y;
                }

                x /= count.len() as f32;
                y /= count.len() as f32;

                transform.translation.x = x;
                transform.translation.y = y;
                mean.x = x;
                mean.y = y;
                mean.last_tick += 1.0;
            }
        }
    }
}
