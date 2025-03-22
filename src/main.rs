use raylib::prelude::*;



fn main() {
    let (mut rl, thread) = open_window("Rings Field");
    //let (w, h) = (1920, 1080); //working with some example code some translations are needed

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0), // Position
        Vector3::new(0.0, 1.0, -1.0), // Target
        Vector3::new(0.0, 1.0, 0.0), // Up vector
        90.0,); // FOV


    let mut model = rl.load_model(&thread ,"assets/robot.glb").unwrap();
    let position = Vector3::new(0.0, 0.0, 0.0);

    let mut animation_index: u32= 2;
    let mut animation_current_frame: u32 = 0;
    let animation_model = rl.load_model_animations(&thread, "assets/robot.glb").expect("Failed to load animation");
    let animation_count: i32 = animation_model.len() as i32;
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            animation_index = (animation_index + 1) % animation_count as u32;

        } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            animation_index = (animation_index + animation_count as u32 - 1) % animation_count as u32;
        }



        animation_index = animation_index % animation_count as u32;

        let anim = &animation_model[animation_index as usize];
        if anim.frameCount > 0 {
            animation_current_frame = (animation_current_frame + 1) % anim.frameCount as u32;

            //dbg!(animation_index, animation_count, anim.frameCount, anim.boneCount, model.boneCount);
            rl.update_model_animation(&thread, &mut model, anim, animation_current_frame as i32);
        } else {
            eprintln!("Error: Animation {} has 0 frames!", animation_index);
        }
        let mut drawing = rl.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);

        drawing.draw_mode3D(camera, |mut mode_3d, _camera| {
            mode_3d.draw_model(&model, position, 0.2, Color::WHITE);
            mode_3d.draw_grid(10, 1.0);
        });

        drawing.draw_fps(10, 10)

    }


}

fn open_window(name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title(name)
        .build();

    rl.set_target_fps(120);

    (rl, thread)
}
