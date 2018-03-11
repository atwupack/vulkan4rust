use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};

pub fn show_physical_devices() {

    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();

    for device in PhysicalDevice::enumerate(&instance) {
        println!("Name: {}", device.name());
        println!("Type: {:?}", device.ty());
        println!("Features: {:?}", device.supported_features());

        for queue_famliy in device.queue_families() {
            println!("    ID: {:?}", queue_famliy.id());
            println!("    Count: {:?}", queue_famliy.queues_count());
        }


    }
}