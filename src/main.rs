
use bevy::prelude::*;
use rand::{random, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Set the window's parameters, note we're setting the window to always be on top.
                transparent: true,
                decorations: true,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, show_ball)
        .add_systems(Update, move_ball) // Add move_ball to Update schedule
        .run();
}

#[derive(Component)]
pub struct Player {
    pub name: String,
}

// System to setup a 2D camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn show_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_texture: Handle<Image> = asset_server.load("textures/ball_blue_small.png");
    
    // In modern Bevy, we use SpriteBundle again for 2D sprites
    commands.spawn((
        Sprite {
            image: ball_texture,
            ..default()
        },
        // Add our Player component
        Player {
            name: "Ball".to_string(),
        },
    ));
}

// System to move all entities with a Player component.
fn move_ball(time: Res<Time>, window: Query<&Window>, mut query: Query<(&Player, &mut Transform)>) {
    let mut rng = rand::rng();
    // Generate a random u32 between 0 and 100 (inclusive).
    // Get the primary window
    if let Ok(window) = window.get_single() {
        // In Bevy, (0,0) is the center so the extents are half the window size.
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;
        let speed = 25.0;
        
        for (player, mut transform) in query.iter_mut() {
            // Update the ball's position.
            transform.translation.x += time.delta_secs() * speed * rng.gen_range(-100.0..100.0);
            transform.translation.y += time.delta_secs() * speed * rng.gen_range(-100.0..100.0);
            // Restrict the ball's position to the window bounds.
            transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
            transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
            
            println!(
                "Player {} is at position {:?}",
                player.name, transform.translation
            );
        }
    }
}