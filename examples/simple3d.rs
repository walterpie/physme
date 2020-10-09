use bevy::prelude::*;
use physme::prelude3d::*;

#[derive(Default)]
pub struct CharacterController {
    on_ground: bool,
    jump: bool,
}

fn main() {
    let mut builder = App::build();
    builder
        .add_default_plugins()
        .add_plugin(Physics3dPlugin)
        .add_resource(GlobalGravity(Vec3::new(0.0, -9.8, 0.0)))
        .add_resource(GlobalFriction(0.90))
        .add_resource(GlobalStep(0.5))
        .add_startup_system(setup.system());
    let character_system = CharacterControllerSystem::default().system(builder.resources_mut());
    builder.add_system(character_system);
    builder.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(shape::Cube { size: 0.5 }.into());
    let bigcube = meshes.add(shape::Cube { size: 8.0 }.into());
    let smallcube = meshes.add(shape::Cube { size: 0.2 }.into());
    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: cube,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec3::new(0.0, 1.0, 0.0)),
        )
        .with(CharacterController::default())
        .with_children(|parent| {
            parent
                .spawn((Shape::from(Size3::new(1.0, 1.0, 1.0)),))
                .spawn(Camera3dComponents {
                    transform: Transform::from_translation_rotation(
                        Vec3::new(0.0, 8.0, 8.0),
                        Quat::from_rotation_x(-45.0_f32.to_radians()),
                    ),
                    ..Default::default()
                });
        })
        .spawn(PbrComponents {
            mesh: cube,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec3::new(0.0, 5.0, 0.0)),
        )
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(1.0, 1.0, 1.0)),));
        })
        .spawn(PbrComponents {
            mesh: bigcube,
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Infinite)
                .with_status(Status::Static)
                .with_position(Vec3::new(0.0, -8.0, 0.0)),
        )
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(16.0, 16.0, 16.0)),));
        })
        .spawn(PbrComponents {
            mesh: smallcube,
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Infinite)
                .with_status(Status::Static)
                .with_position(Vec3::new(-3.0, 0.0, -3.0)),
        )
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(0.4, 0.4, 0.4)),));
        });
}

#[derive(Default)]
pub struct CharacterControllerSystem {
    reader: EventReader<Manifold>,
}

impl CharacterControllerSystem {
    pub fn system(self, res: &mut Resources) -> Box<dyn System> {
        let system = character_system.system();
        res.insert_local(system.id(), self);
        system
    }
}

fn character_system(
    mut state: Local<CharacterControllerSystem>,
    input: Res<Input<KeyCode>>,
    manifolds: Res<Events<Manifold>>,
    mut query: Query<(Mut<CharacterController>, Mut<RigidBody>)>,
) {
    for manifold in state.reader.iter(&manifolds) {
        if manifold.normals.y() < 0.0 {
            if let Ok(mut controller) = query.get_mut::<CharacterController>(manifold.body1) {
                controller.on_ground = true;
            }
        } else if manifold.normals.y() > 0.0 {
            if let Ok(mut controller) = query.get_mut::<CharacterController>(manifold.body2) {
                controller.on_ground = true;
            }
        }
    }

    for (mut controller, mut body) in &mut query.iter() {
        if input.just_pressed(KeyCode::Space) {
            controller.jump = true;
        }
        if controller.on_ground {
            if controller.jump {
                body.apply_force(Vec3::new(0.0, 500.0, 0.0));
                controller.jump = false;
            }
        }
        if input.pressed(KeyCode::W) {
            body.apply_impulse(Vec3::new(0.0, 0.0, -0.5));
        }
        if input.pressed(KeyCode::S) {
            body.apply_impulse(Vec3::new(0.0, 0.0, 0.5));
        }
        if input.pressed(KeyCode::A) {
            body.apply_impulse(Vec3::new(-0.5, 0.0, 0.0));
        }
        if input.pressed(KeyCode::D) {
            body.apply_impulse(Vec3::new(0.5, 0.0, 0.0));
        }
        controller.on_ground = false;
    }
}
