/*
By Tate Larkin

TODO:
more encapsulation and modularaization
*/

use winit::
{
    event::*,
    event_loop::{ ControlFlow, EventLoop },
    window::{ WindowBuilder },
};

// utils is my module
use utils::state::State;

mod utils;


fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // initializing the State
    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| { match event
        {
            Event::WindowEvent
            {
                ref event,
                window_id,
            } 
            if window_id == window.id() => 
            {
                if !state.input(event)
                {
                    // match for different events that can be triggered
                    // like close requested, resized, etc.
                    match event
                    {

                        // if close window signal --->
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput
                        {
                            input:
                                KeyboardInput
                                {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        // ---> exit
                        } => *control_flow = ControlFlow::Exit,

                        // if window resized --->
                        WindowEvent::Resized(physical_size) =>
                        {
                        // ---> resize
                            state.resize(*physical_size);
                        },

                        // if scale factor changed --->
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
                        {
                        // ---> resize
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        },
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() =>
            {
                state.update();
                match state.render()
                {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            Event::MainEventsCleared =>
            {
                // RedrawRequested will only trigger once, unless we manually request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

 