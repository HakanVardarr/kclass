use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_tasks::TaskPoolBuilder;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

const WIDTH: f32 = 800.0;
const HEIGTH: f32 = 600.0;
const K: usize = 5;
const COLORS: &[Color] = &[
    Color::WHITE,
    Color::BLUE,
    Color::PINK,
    Color::YELLOW,
    Color::GREEN,
    Color::GRAY,
    Color::PURPLE,
    Color::ALICE_BLUE,
    Color::FUCHSIA,
    Color::MAROON,
    Color::CYAN,
    Color::VIOLET,
    Color::RED,
    Color::ANTIQUE_WHITE,
];

#[derive(Component)]
struct Sample;

#[derive(Resource)]
struct Samples {
    items: Vec<Vec2>,
}

impl Samples {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn generate_cluster(&mut self, center: Vec2, radius: f32, count: u64) {
        for _ in 0..count {
            let angle = thread_rng().gen_range(0.0..1.0) * 2.0 * PI;
            let mag = thread_rng().gen_range(0.0..1.0);
            let sample = Vec2::new(
                center.x + angle.cos() * mag * radius,
                center.y + angle.sin() * mag * radius,
            );

            self.items.push(sample);
        }
    }
}

#[derive(Component)]
struct Mean;

#[derive(Resource)]
struct Means {
    means: Vec<Vec2>,
}

impl Means {
    fn new() -> Self {
        let mut means = vec![];
        for _ in 0..K {
            means.push(Vec2::new(
                thread_rng().gen_range(-WIDTH / 2.0..WIDTH / 2.0),
                thread_rng().gen_range(-HEIGTH / 2.0..HEIGTH / 2.0),
            ));
        }

        Self { means }
    }
}

fn sample_sample(samples: &mut Samples) {
    samples.generate_cluster(Vec2::new(0.0, 0.0), 400.0, 10_000);
}

fn main() {
    let mut samples = Samples::new();
    sample_sample(&mut samples);

    let means = Means::new();

    App::new()
        .insert_resource(samples)
        .insert_resource(means)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "K Class".to_string(),
                resolution: (WIDTH, HEIGTH).into(),
                resizable: true,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, regenerate_samples)
        .run();
}

fn setup(
    means: Res<Means>,
    samples: Res<Samples>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(
                24.0 / 255.0,
                24.0 / 255.0,
                24.0 / 255.0,
            )),
        },
        ..Default::default()
    });

    for (i, mean) in means.means.iter().enumerate() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::default().into()).into(),
                material: materials.add(COLORS[i].into()),
                transform: Transform::from_translation(Vec3::new(mean.x, mean.y, 1.0))
                    .with_scale(Vec3::new(8.0, 8.0, 0.0)),
                ..Default::default()
            },
            Mean,
        ));
    }

    for sample in &samples.items {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::default().into()).into(),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(
                    sample.x,
                    sample.y,
                    thread_rng().gen_range(0.0..1000.0),
                ))
                .with_scale(Vec3::new(3.0, 3.0, 0.0)),
                ..Default::default()
            },
            Sample,
        ));
    }
}

fn regenerate_samples(
    keyboard_input: Res<Input<KeyCode>>,
    mut means: ResMut<Means>,
    mut samples: ResMut<Samples>,
    mut sample_query: Query<
        (&mut Transform, &Handle<ColorMaterial>),
        (Without<Mean>, With<Sample>),
    >,
    mut mean_query: Query<&mut Transform, (With<Mean>, Without<Sample>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut count = vec![vec![]; K];
    // if keyboard_input.just_pressed(KeyCode::Space) {
    for (i, &sample) in samples.items.iter().enumerate() {
        let mut smallest = f32::MAX;
        let mut idx = 10;
        for (k, &mean) in means.means.iter().enumerate() {
            let distance = ((mean.x - sample.x).powf(2.0) + (mean.y - sample.y).powf(2.0)).sqrt();
            if distance < smallest {
                smallest = distance;
                idx = k;
            }
        }

        if let Some((_, handle)) = sample_query.iter_mut().nth(i) {
            let new_material = materials.get_mut(handle).unwrap();
            new_material.color = COLORS[idx];
        }
        count[idx].push(sample);
    }

    for (id, c) in count.iter().enumerate() {
        if c.len() == 0 {
            let x = thread_rng().gen_range(-WIDTH / 2.0..WIDTH / 2.0);
            let y = thread_rng().gen_range(-HEIGTH / 2.0..HEIGTH / 2.0);
            if let Some(mut transform) = mean_query.iter_mut().nth(id) {
                transform.translation.x = x;
                transform.translation.y = y;
            }
            if let Some(pos) = means.means.iter_mut().nth(id) {
                pos.x = x;
                pos.y = y;
            }
        }
    }

    for (id, c) in count.iter().enumerate() {
        if c.len() != 0 {
            if let Some(pos) = means.means.iter_mut().nth(id) {
                *pos = Vec2::ZERO;
                for p in c {
                    *pos += *p;
                }

                pos.x /= c.len() as f32;
                pos.y /= c.len() as f32;

                if let Some(mut transform) = mean_query.iter_mut().nth(id) {
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                }
            }
        }
    }
    // }
    if keyboard_input.just_pressed(KeyCode::R) {
        *samples = Samples::new();
        sample_sample(&mut samples);
        *means = Means::new();

        for (idx, (mut transform, handle)) in sample_query.iter_mut().enumerate() {
            let new_material = materials.get_mut(handle).unwrap();
            new_material.color = Color::RED;

            transform.translation.x = samples.items[idx].x;
            transform.translation.y = samples.items[idx].y;
        }

        for (idx, mut transform) in mean_query.iter_mut().enumerate() {
            transform.translation.x = means.means[idx].x;
            transform.translation.y = means.means[idx].y;
        }
    }
}
