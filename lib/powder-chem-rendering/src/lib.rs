use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};

pub fn setup() {
	let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
	let instance = Instance::new(
		library,
		InstanceCreateInfo {
				..Default::default()
		},
	)
	.expect("failed to create instance");
}

#[cfg(test)]
mod tests {
		use super::*;

		#[test]
		fn it_works() {
			setup();
		}
}
