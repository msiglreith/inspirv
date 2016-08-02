
// SPIR-V generators IDs
// Following: https://github.com/KhronosGroup/SPIRV-Headers/blob/master/include/spirv/spir-v.xml
enum_from_primitive! {
#[derive(Debug)]
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
}}

#[derive(Debug)]
pub enum Generator {
    Id(GeneratorId),
    Unknown(u32),
}

/// Header description following SPIR-V specs(1.1), Section 2.3
pub struct Header {
    pub magic_number: u32,
    pub version: (u32, u32), // major, minor
    pub generator: Generator,
    pub bound: u32,
}

pub struct LogicalModule {

}
