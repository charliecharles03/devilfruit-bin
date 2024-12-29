use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();
    // Create a window
    let window = WindowBuilder::new()
        .with_title("Winit Example")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window");
    println!("Window created: {:?}", window);
    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        // Set control flow to Wait, minimizing CPU usage
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window close requested");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    println!("Mouse moved to: {:?}", position);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    println!("Mouse button {:?} was {:?}", button, state);

                }
                WindowEvent::Resized(size) => {
                    println!("Window resized: {:?}", size);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    println!("Keyboard input: {:?}", input);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Application logic goes here
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Drawing logic goes here
                println!("Redrawing the window");
            }
            _ => {}
        }
    });
}
