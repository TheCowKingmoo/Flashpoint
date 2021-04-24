use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::PerspectiveProjection;

mod camera_stuff;
// yoinked from https://github.com/bevyengine/bevy/blob/main/examples/3d/orthographic.rs

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))             // sets the background color
        .insert_resource(Msaa { samples: 4 })                               // Anti Al
        .insert_resource(WindowDescriptor {                                 // Default Windows Settings
            title: "Flashpoint".to_string(),
            width: 1920.,
            height: 1080.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())                        // log diag
        .add_plugin(FrameTimeDiagnosticsPlugin::default())                  // framerate
       // .add_startup_system(setup.system())
        .add_startup_system(create_grid.system())
        //.add_system(cursor_grab_system.system())                            // grabs the mouse
        .add_startup_system(camera_stuff::spawn_scene.system())
        .add_system(camera_stuff::pan_orbit_camera.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())       // calls exit program by pressing escape
        .run();
}



// used to quit the program
fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}


// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera);

    // plane
  //  commands.spawn_bundle(PbrBundle {
  //      mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
  //      material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
  //      ..Default::default()
  //  });

/*
    // cubes
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..Default::default()
    });
    */
    
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..Default::default()
    });
}


// shows how i can use a mouse/key event and do something about it
fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Up) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn create_grid (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let texture_handle = asset_server.load("textures/box.png");

    let grid_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        ..Default::default()
    });

    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.}));

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                // Change material according to position to get alternating pattern
                material: grid_material.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}








