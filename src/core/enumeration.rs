// WARNING: Generated by `build/main.rs`, do NOT change manually!

bitflags! {
    pub flags ImageOperands: u32 {
        const ImageOperandsNone = 0x0000,
        const ImageOperandsBias = 0x0001,
        const ImageOperandsLod = 0x0002,
        const ImageOperandsGrad = 0x0004,
        const ImageOperandsConstOffset = 0x0008,
        const ImageOperandsOffset = 0x0010,
        const ImageOperandsConstOffsets = 0x0020,
        const ImageOperandsSample = 0x0040,
        const ImageOperandsMinLod = 0x0080,
    }
}

bitflags! {
    pub flags FPFastMathMode: u32 {
        const FPFastMathModeNone = 0x0000,
        const FPFastMathModeNotNaN = 0x0001,
        const FPFastMathModeNotInf = 0x0002,
        const FPFastMathModeNSZ = 0x0004,
        const FPFastMathModeAllowRecip = 0x0008,
        const FPFastMathModeFast = 0x0010,
    }
}

bitflags! {
    pub flags SelectionControl: u32 {
        const SelectionControlNone = 0x0000,
        const SelectionControlFlatten = 0x0001,
        const SelectionControlDontFlatten = 0x0002,
    }
}

bitflags! {
    pub flags LoopControl: u32 {
        const LoopControlNone = 0x0000,
        const LoopControlUnroll = 0x0001,
        const LoopControlDontUnroll = 0x0002,
        const LoopControlDependencyInfinite = 0x0004,
        const LoopControlDependencyLength = 0x0008,
    }
}

bitflags! {
    pub flags FunctionControl: u32 {
        const FunctionControlNone = 0x0000,
        const FunctionControlInline = 0x0001,
        const FunctionControlDontInline = 0x0002,
        const FunctionControlPure = 0x0004,
        const FunctionControlConst = 0x0008,
    }
}

bitflags! {
    pub flags MemorySemantics: u32 {
        const MemorySemanticsRelaxed = 0x0000,
        const MemorySemanticsNone = 0x0000,
        const MemorySemanticsSequentiallyConsistent = 0x0010,
        const MemorySemanticsAcquire = 0x0002,
        const MemorySemanticsRelease = 0x0004,
        const MemorySemanticsAcquireRelease = 0x0008,
        const MemorySemanticsUniformMemory = 0x0040,
        const MemorySemanticsSubgroupMemory = 0x0080,
        const MemorySemanticsWorkgroupMemory = 0x0100,
        const MemorySemanticsCrossWorkgroupMemory = 0x0200,
        const MemorySemanticsAtomicCounterMemory = 0x0400,
        const MemorySemanticsImageMemory = 0x0800,
    }
}

bitflags! {
    pub flags MemoryAccess: u32 {
        const MemoryAccessNone = 0x0000,
        const MemoryAccessVolatile = 0x0001,
        const MemoryAccessAligned = 0x0002,
        const MemoryAccessNontemporal = 0x0004,
    }
}

bitflags! {
    pub flags KernelProfilingInfo: u32 {
        const KernelProfilingInfoNone = 0x0000,
        const KernelProfilingInfoCmdExecTime = 0x0001,
    }
}

pub enum SourceLanguage {
    SourceLanguageUnknown = 0,
    SourceLanguageESSL = 1,
    SourceLanguageGLSL = 2,
    SourceLanguageOpenCL_C = 3,
    SourceLanguageOpenCL_CPP = 4,
}

pub enum ExecutionModel {
    ExecutionModelVertex = 0,
    ExecutionModelTessellationControl = 1,
    ExecutionModelTessellationEvaluation = 2,
    ExecutionModelGeometry = 3,
    ExecutionModelFragment = 4,
    ExecutionModelGLCompute = 5,
    ExecutionModelKernel = 6,
}

pub enum AddressingModel {
    AddressingModelLogical = 0,
    AddressingModelPhysical32 = 1,
    AddressingModelPhysical64 = 2,
}

pub enum MemoryModel {
    MemoryModelSimple = 0,
    MemoryModelGLSL450 = 1,
    MemoryModelOpenCL = 2,
}

pub enum ExecutionMode {
    ExecutionModeInvocations = 0,
    ExecutionModeSpacingEqual = 1,
    ExecutionModeSpacingFractionalEven = 2,
    ExecutionModeSpacingFractionalOdd = 3,
    ExecutionModeVertexOrderCw = 4,
    ExecutionModeVertexOrderCcw = 5,
    ExecutionModePixelCenterInteger = 6,
    ExecutionModeOriginUpperLeft = 7,
    ExecutionModeOriginLowerLeft = 8,
    ExecutionModeEarlyFragmentTests = 9,
    ExecutionModePointMode = 10,
    ExecutionModeXfb = 11,
    ExecutionModeDepthReplacing = 12,
    ExecutionModeDepthGreater = 14,
    ExecutionModeDepthLess = 15,
    ExecutionModeDepthUnchanged = 16,
    ExecutionModeLocalSize = 17,
    ExecutionModeLocalSizeHint = 18,
    ExecutionModeInputPoints = 19,
    ExecutionModeInputLines = 20,
    ExecutionModeInputLinesAdjacency = 21,
    ExecutionModeTriangles = 22,
    ExecutionModeInputTrianglesAdjacency = 23,
    ExecutionModeQuads = 24,
    ExecutionModeIsolines = 25,
    ExecutionModeOutputVertices = 26,
    ExecutionModeOutputPoints = 27,
    ExecutionModeOutputLineStrip = 28,
    ExecutionModeOutputTriangleStrip = 29,
    ExecutionModeVecTypeHint = 30,
    ExecutionModeContractionOff = 31,
    ExecutionModeInitializer = 33,
    ExecutionModeFinalizer = 34,
    ExecutionModeSubgroupSize = 35,
    ExecutionModeSubgroupsPerWorkgroup = 36,
}

pub enum StorageClass {
    StorageClassUniformConstant = 0,
    StorageClassInput = 1,
    StorageClassUniform = 2,
    StorageClassOutput = 3,
    StorageClassWorkgroup = 4,
    StorageClassCrossWorkgroup = 5,
    StorageClassPrivate = 6,
    StorageClassFunction = 7,
    StorageClassGeneric = 8,
    StorageClassPushConstant = 9,
    StorageClassAtomicCounter = 10,
    StorageClassImage = 11,
}

pub enum Dim {
    Dim1D = 0,
    Dim2D = 1,
    Dim3D = 2,
    DimCube = 3,
    DimRect = 4,
    DimBuffer = 5,
    DimSubpassData = 6,
}

pub enum SamplerAddressingMode {
    SamplerAddressingModeNone = 0,
    SamplerAddressingModeClampToEdge = 1,
    SamplerAddressingModeClamp = 2,
    SamplerAddressingModeRepeat = 3,
    SamplerAddressingModeRepeatMirrored = 4,
}

pub enum SamplerFilterMode {
    SamplerFilterModeNearest = 0,
    SamplerFilterModeLinear = 1,
}

pub enum ImageFormat {
    ImageFormatUnknown = 0,
    ImageFormatRgba32f = 1,
    ImageFormatRgba16f = 2,
    ImageFormatR32f = 3,
    ImageFormatRgba8 = 4,
    ImageFormatRgba8Snorm = 5,
    ImageFormatRg32f = 6,
    ImageFormatRg16f = 7,
    ImageFormatR11fG11fB10f = 8,
    ImageFormatR16f = 9,
    ImageFormatRgba16 = 10,
    ImageFormatRgb10A2 = 11,
    ImageFormatRg16 = 12,
    ImageFormatRg8 = 13,
    ImageFormatR16 = 14,
    ImageFormatR8 = 15,
    ImageFormatRgba16Snorm = 16,
    ImageFormatRg16Snorm = 17,
    ImageFormatRg8Snorm = 18,
    ImageFormatR16Snorm = 19,
    ImageFormatR8Snorm = 20,
    ImageFormatRgba32i = 21,
    ImageFormatRgba16i = 22,
    ImageFormatRgba8i = 23,
    ImageFormatR32i = 24,
    ImageFormatRg32i = 25,
    ImageFormatRg16i = 26,
    ImageFormatRg8i = 27,
    ImageFormatR16i = 28,
    ImageFormatR8i = 29,
    ImageFormatRgba32ui = 30,
    ImageFormatRgba16ui = 31,
    ImageFormatRgba8ui = 32,
    ImageFormatR32ui = 33,
    ImageFormatRgb10a2ui = 34,
    ImageFormatRg32ui = 35,
    ImageFormatRg16ui = 36,
    ImageFormatRg8ui = 37,
    ImageFormatR16ui = 38,
    ImageFormatR8ui = 39,
}

pub enum ImageChannelOrder {
    ImageChannelOrderR = 0,
    ImageChannelOrderA = 1,
    ImageChannelOrderRG = 2,
    ImageChannelOrderRA = 3,
    ImageChannelOrderRGB = 4,
    ImageChannelOrderRGBA = 5,
    ImageChannelOrderBGRA = 6,
    ImageChannelOrderARGB = 7,
    ImageChannelOrderIntensity = 8,
    ImageChannelOrderLuminance = 9,
    ImageChannelOrderRx = 10,
    ImageChannelOrderRGx = 11,
    ImageChannelOrderRGBx = 12,
    ImageChannelOrderDepth = 13,
    ImageChannelOrderDepthStencil = 14,
    ImageChannelOrdersRGB = 15,
    ImageChannelOrdersRGBx = 16,
    ImageChannelOrdersRGBA = 17,
    ImageChannelOrdersBGRA = 18,
    ImageChannelOrderABGR = 19,
}

pub enum ImageChannelDataType {
    ImageChannelDataTypeSnormInt8 = 0,
    ImageChannelDataTypeSnormInt16 = 1,
    ImageChannelDataTypeUnormInt8 = 2,
    ImageChannelDataTypeUnormInt16 = 3,
    ImageChannelDataTypeUnormShort565 = 4,
    ImageChannelDataTypeUnormShort555 = 5,
    ImageChannelDataTypeUnormInt101010 = 6,
    ImageChannelDataTypeSignedInt8 = 7,
    ImageChannelDataTypeSignedInt16 = 8,
    ImageChannelDataTypeSignedInt32 = 9,
    ImageChannelDataTypeUnsignedInt8 = 10,
    ImageChannelDataTypeUnsignedInt16 = 11,
    ImageChannelDataTypeUnsignedInt32 = 12,
    ImageChannelDataTypeHalfFloat = 13,
    ImageChannelDataTypeFloat = 14,
    ImageChannelDataTypeUnormInt24 = 15,
    ImageChannelDataTypeUnormInt101010_2 = 16,
}

pub enum FPRoundingMode {
    FPRoundingModeRTE = 0,
    FPRoundingModeRTZ = 1,
    FPRoundingModeRTP = 2,
    FPRoundingModeRTN = 3,
}

pub enum LinkageType {
    LinkageTypeExport = 0,
    LinkageTypeImport = 1,
}

pub enum AccessQualifier {
    AccessQualifierReadOnly = 0,
    AccessQualifierWriteOnly = 1,
    AccessQualifierReadWrite = 2,
}

pub enum FunctionParameterAttribute {
    FunctionParameterAttributeZext = 0,
    FunctionParameterAttributeSext = 1,
    FunctionParameterAttributeByVal = 2,
    FunctionParameterAttributeSret = 3,
    FunctionParameterAttributeNoAlias = 4,
    FunctionParameterAttributeNoCapture = 5,
    FunctionParameterAttributeNoWrite = 6,
    FunctionParameterAttributeNoReadWrite = 7,
}

pub enum Decoration {
    DecorationRelaxedPrecision = 0,
    DecorationSpecId = 1,
    DecorationBlock = 2,
    DecorationBufferBlock = 3,
    DecorationRowMajor = 4,
    DecorationColMajor = 5,
    DecorationArrayStride = 6,
    DecorationMatrixStride = 7,
    DecorationGLSLShared = 8,
    DecorationGLSLPacked = 9,
    DecorationCPacked = 10,
    DecorationBuiltIn = 11,
    DecorationNoPerspective = 13,
    DecorationFlat = 14,
    DecorationPatch = 15,
    DecorationCentroid = 16,
    DecorationSample = 17,
    DecorationInvariant = 18,
    DecorationRestrict = 19,
    DecorationAliased = 20,
    DecorationVolatile = 21,
    DecorationConstant = 22,
    DecorationCoherent = 23,
    DecorationNonWritable = 24,
    DecorationNonReadable = 25,
    DecorationUniform = 26,
    DecorationSaturatedConversion = 28,
    DecorationStream = 29,
    DecorationLocation = 30,
    DecorationComponent = 31,
    DecorationIndex = 32,
    DecorationBinding = 33,
    DecorationDescriptorSet = 34,
    DecorationOffset = 35,
    DecorationXfbBuffer = 36,
    DecorationXfbStride = 37,
    DecorationFuncParamAttr = 38,
    DecorationFPRoundingMode = 39,
    DecorationFPFastMathMode = 40,
    DecorationLinkageAttributes = 41,
    DecorationNoContraction = 42,
    DecorationInputAttachmentIndex = 43,
    DecorationAlignment = 44,
    DecorationMaxByteOffset = 45,
}

pub enum BuiltIn {
    BuiltInPosition = 0,
    BuiltInPointSize = 1,
    BuiltInClipDistance = 3,
    BuiltInCullDistance = 4,
    BuiltInVertexId = 5,
    BuiltInInstanceId = 6,
    BuiltInPrimitiveId = 7,
    BuiltInInvocationId = 8,
    BuiltInLayer = 9,
    BuiltInViewportIndex = 10,
    BuiltInTessLevelOuter = 11,
    BuiltInTessLevelInner = 12,
    BuiltInTessCoord = 13,
    BuiltInPatchVertices = 14,
    BuiltInFragCoord = 15,
    BuiltInPointCoord = 16,
    BuiltInFrontFacing = 17,
    BuiltInSampleId = 18,
    BuiltInSamplePosition = 19,
    BuiltInSampleMask = 20,
    BuiltInFragDepth = 22,
    BuiltInHelperInvocation = 23,
    BuiltInNumWorkgroups = 24,
    BuiltInWorkgroupSize = 25,
    BuiltInWorkgroupId = 26,
    BuiltInLocalInvocationId = 27,
    BuiltInGlobalInvocationId = 28,
    BuiltInLocalInvocationIndex = 29,
    BuiltInWorkDim = 30,
    BuiltInGlobalSize = 31,
    BuiltInEnqueuedWorkgroupSize = 32,
    BuiltInGlobalOffset = 33,
    BuiltInGlobalLinearId = 34,
    BuiltInSubgroupSize = 36,
    BuiltInSubgroupMaxSize = 37,
    BuiltInNumSubgroups = 38,
    BuiltInNumEnqueuedSubgroups = 39,
    BuiltInSubgroupId = 40,
    BuiltInSubgroupLocalInvocationId = 41,
    BuiltInVertexIndex = 42,
    BuiltInInstanceIndex = 43,
}

pub enum Scope {
    ScopeCrossDevice = 0,
    ScopeDevice = 1,
    ScopeWorkgroup = 2,
    ScopeSubgroup = 3,
    ScopeInvocation = 4,
}

pub enum GroupOperation {
    GroupOperationReduce = 0,
    GroupOperationInclusiveScan = 1,
    GroupOperationExclusiveScan = 2,
}

pub enum KernelEnqueueFlags {
    KernelEnqueueFlagsNoWait = 0,
    KernelEnqueueFlagsWaitKernel = 1,
    KernelEnqueueFlagsWaitWorkGroup = 2,
}

pub enum Capability {
    CapabilityMatrix = 0,
    CapabilityShader = 1,
    CapabilityGeometry = 2,
    CapabilityTessellation = 3,
    CapabilityAddresses = 4,
    CapabilityLinkage = 5,
    CapabilityKernel = 6,
    CapabilityVector16 = 7,
    CapabilityFloat16Buffer = 8,
    CapabilityFloat16 = 9,
    CapabilityFloat64 = 10,
    CapabilityInt64 = 11,
    CapabilityInt64Atomics = 12,
    CapabilityImageBasic = 13,
    CapabilityImageReadWrite = 14,
    CapabilityImageMipmap = 15,
    CapabilityPipes = 17,
    CapabilityGroups = 18,
    CapabilityDeviceEnqueue = 19,
    CapabilityLiteralSampler = 20,
    CapabilityAtomicStorage = 21,
    CapabilityInt16 = 22,
    CapabilityTessellationPointSize = 23,
    CapabilityGeometryPointSize = 24,
    CapabilityImageGatherExtended = 25,
    CapabilityStorageImageMultisample = 27,
    CapabilityUniformBufferArrayDynamicIndexing = 28,
    CapabilitySampledImageArrayDynamicIndexing = 29,
    CapabilityStorageBufferArrayDynamicIndexing = 30,
    CapabilityStorageImageArrayDynamicIndexing = 31,
    CapabilityClipDistance = 32,
    CapabilityCullDistance = 33,
    CapabilityImageCubeArray = 34,
    CapabilitySampleRateShading = 35,
    CapabilityImageRect = 36,
    CapabilitySampledRect = 37,
    CapabilityGenericPointer = 38,
    CapabilityInt8 = 39,
    CapabilityInputAttachment = 40,
    CapabilitySparseResidency = 41,
    CapabilityMinLod = 42,
    CapabilitySampled1D = 43,
    CapabilityImage1D = 44,
    CapabilitySampledCubeArray = 45,
    CapabilitySampledBuffer = 46,
    CapabilityImageBuffer = 47,
    CapabilityImageMSArray = 48,
    CapabilityStorageImageExtendedFormats = 49,
    CapabilityImageQuery = 50,
    CapabilityDerivativeControl = 51,
    CapabilityInterpolationFunction = 52,
    CapabilityTransformFeedback = 53,
    CapabilityGeometryStreams = 54,
    CapabilityStorageImageReadWithoutFormat = 55,
    CapabilityStorageImageWriteWithoutFormat = 56,
    CapabilityMultiViewport = 57,
    CapabilitySubgroupDispatch = 58,
    CapabilityNamedBarrier = 59,
    CapabilityPipeStorage = 60,
}

