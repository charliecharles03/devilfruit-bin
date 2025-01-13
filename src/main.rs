use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use vulkano::{
    command_buffer::allocator::StandardCommandBufferAlloc, device::{Device, DeviceExtensions, Queue, QueueFlags}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, swapchain::Surface, Version, VulkanLibrary
};

use std::{default, error:: Error, sync::Arc};

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();
    print!("{:?}",event_loop);
    App::new(&event_loop);
}


struct App{
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    command_buffer_allocator: Arc<StandardCommandBufferAlloc>,
}


impl App{
    fn new(event_loop: &EventLoop<()>){
        println!("{:?}",event_loop);
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = Surface::required_extensions(event_loop);

        let instance = Instance::new(
            library,
            InstanceCreateInfo { 
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY, 
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .unwrap();

        let device_extensions = DeviceExtensions{
            khr_swapchain: true,
            ..DeviceExtensions::empty() 
        };

         let res = instance
            .enumerate_physical_devices()
            .unwrap().map(|A|{
                return A.api_version().major;
            });
    }
}
