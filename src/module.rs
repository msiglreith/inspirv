
/// A numerical ID to refer to an object, type, function, label, ...
pub struct Id(u32);

// SPIR-V tool IDs
// Following: https://github.com/KhronosGroup/SPIRV-Headers/blob/master/include/spirv/spir-v.xml
pub enum GeneratorId {
	Khronos = 0,
	LunarG = 1,
	Valve = 2,
	Codeplay = 3,
	Nvidia = 4,
	ARM = 5,
	LlvmSpirv = 6,
	SpirvTools = 7,
	Glslang = 8,
	Qualcomm = 9,
	AMD = 10,
	Intel = 11,
}

pub struct LogicalModule {

}
