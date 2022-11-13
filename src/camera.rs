use bevy::prelude::*;

use crate::*;

#[derive(Debug, Clone)]
struct CameraMovement {
    forward: Vec3,
    left: Vec3,
    speed: f32,
    rotate_speed: f32

}
impl CameraMovement {
    fn new(camera: Transform) -> CameraMovement {
        CameraMovement {
            forward: CameraMovement::create_forward(camera),
            left: CameraMovement::create_left(camera),
            speed: 4.0,
            rotate_speed: 0.4        
        }
    }

    fn create_left(camera:  Transform) -> Vec3 {
        let mut left = camera.left();
        left.y = 0.0;
        left = left.normalize();  
        left  
    }    

    fn create_forward(camera: Transform) -> Vec3 {
        let mut forward = camera.forward();
        forward.y = 0.0;
        forward = forward.normalize();    
        forward
    }
}

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let movement = CameraMovement::new(camera.clone());
    camera_keyboard_control(&mut camera, movement, keyboard, time);
}

// struct M2<'a, 'b> {
//     movement: &'b CameraMovement,
//     time: &'a Time
// }
// impl<'a, 'b> M2<'a, 'b> {
//     fn new(movement: &'b CameraMovement, time: &'a Time) -> Self {
//         M2 {
//             movement,
//             time
//         }
//     }    
// }


#[derive(Debug, Clone)]
struct Movement<'a, 'b> {
    movement: &'a CameraMovement,
    time: &'b Time
}
impl<'a, 'b> Movement<'a, 'b> {
    fn new(movement: &'a CameraMovement, time: &'b Time) -> Self {
        Movement {
            movement,
            time
        }
    }    

    fn delta(&self) -> f32 {
        self.time.delta_seconds()
    }

    fn delta_speed(&self) -> f32 {
        self.delta() * self.movement.speed
    }

    fn forward(&self) -> Vec3 {
        self.movement.forward * self.delta_speed()
    }

    fn left(&self) -> Vec3 {
        self.movement.left * self.delta_speed()
    }

    fn angle_rotate(&self) -> f32 {
        self.movement.rotate_speed * self.delta()
    }
    
}

fn camera_keyboard_control(camera: &mut Mut<Transform>, movement: CameraMovement, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    // let m2 = M2::new(&movement, &time);
    let mov: Movement = Movement::new(&movement, &time);
    if keyboard.pressed(KeyCode::W) {
        camera.translation += mov.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= mov.forward();
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += mov.left();
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= mov.left();
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, mov.angle_rotate())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -mov.angle_rotate())
    }    
}
