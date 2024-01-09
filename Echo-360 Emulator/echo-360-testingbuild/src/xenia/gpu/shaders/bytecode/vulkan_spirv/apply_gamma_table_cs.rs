// Generated with `xb buildshaders`.
#if 0
; SPIR-V
; Version: 1.0
; Generator: Khronos Glslang Reference Front End; 10
; Bound: 24687
; Schema: 0
               OpCapability Shader
               OpCapability SampledBuffer
               OpCapability StorageImageExtendedFormats
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %5663 "main" %gl_GlobalInvocationID
               OpExecutionMode %5663 LocalSize 16 8 1
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %_struct_993 0 Offset 0
               OpDecorate %_struct_993 Block
               OpDecorate %5759 DescriptorSet 1
               OpDecorate %5759 Binding 0
               OpDecorate %5945 DescriptorSet 0
               OpDecorate %5945 Binding 0
               OpDecorate %3258 DescriptorSet 2
               OpDecorate %3258 Binding 0
               OpDecorate %3258 NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
       %1282 = OpTypeFunction %void
       %uint = OpTypeInt 32 0
     %v2uint = OpTypeVector %uint 2
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
%_struct_993 = OpTypeStruct %v2uint
%_ptr_PushConstant__struct_993 = OpTypePointer PushConstant %_struct_993
       %4495 = OpVariable %_ptr_PushConstant__struct_993 PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_v2uint = OpTypePointer PushConstant %v2uint
       %bool = OpTypeBool
     %v2bool = OpTypeVector %bool 2
      %float = OpTypeFloat 32
        %150 = OpTypeImage %float 2D 0 0 0 1 Unknown
%_ptr_UniformConstant_150 = OpTypePointer UniformConstant %150
       %5759 = OpVariable %_ptr_UniformConstant_150 UniformConstant
      %v2int = OpTypeVector %int 2
    %v4float = OpTypeVector %float 4
    %v3float = OpTypeVector %float 3
  %float_255 = OpConstant %float 255
  %float_0_5 = OpConstant %float 0.5
        %154 = OpTypeImage %float Buffer 0 0 0 1 Unknown
%_ptr_UniformConstant_154 = OpTypePointer UniformConstant %154
       %5945 = OpVariable %_ptr_UniformConstant_154 UniformConstant
     %uint_0 = OpConstant %uint 0
     %uint_1 = OpConstant %uint 1
    %float_1 = OpConstant %float 1
        %166 = OpTypeImage %float 2D 0 0 0 2 Rgb10A2
%_ptr_UniformConstant_166 = OpTypePointer UniformConstant %166
       %3258 = OpVariable %_ptr_UniformConstant_166 UniformConstant
    %uint_16 = OpConstant %uint 16
     %uint_8 = OpConstant %uint 8
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_16 %uint_8 %uint_1
      %10264 = OpUndef %v4float
        %939 = OpConstantComposite %v3float %float_0_5 %float_0_5 %float_0_5
       %5663 = OpFunction %void None %1282
      %15110 = OpLabel
               OpSelectionMerge %21573 None
               OpSwitch %uint_0 %12914
      %12914 = OpLabel
      %13761 = OpLoad %v3uint %gl_GlobalInvocationID
      %21717 = OpVectorShuffle %v2uint %13761 %13761 0 1
       %7760 = OpAccessChain %_ptr_PushConstant_v2uint %4495 %int_0
      %13378 = OpLoad %v2uint %7760
      %23437 = OpUGreaterThanEqual %v2bool %21717 %13378
      %23076 = OpAny %bool %23437
               OpSelectionMerge %18302 DontFlatten
               OpBranchConditional %23076 %21992 %18302
      %21992 = OpLabel
               OpBranch %21573
      %18302 = OpLabel
      %24004 = OpLoad %150 %5759
      %10533 = OpBitcast %v2int %21717
       %6680 = OpImageFetch %v4float %24004 %10533 Lod %int_0
      %16242 = OpVectorShuffle %v3float %6680 %6680 0 1 2
      %13907 = OpVectorTimesScalar %v3float %16242 %float_255
      %16889 = OpFAdd %v3float %13907 %939
      %11099 = OpConvertFToU %v3uint %16889
      %18624 = OpLoad %154 %5945
      %15435 = OpCompositeExtract %uint %11099 0
      %24686 = OpBitcast %int %15435
       %8410 = OpImageFetch %v4float %18624 %24686
       %9324 = OpCompositeExtract %float %8410 2
      %17732 = OpCompositeInsert %v4float %9324 %10264 0
      %12852 = OpCompositeExtract %uint %11099 1
      %12866 = OpBitcast %int %12852
       %8411 = OpImageFetch %v4float %18624 %12866
       %9325 = OpCompositeExtract %float %8411 1
      %17733 = OpCompositeInsert %v4float %9325 %17732 1
      %12853 = OpCompositeExtract %uint %11099 2
      %12867 = OpBitcast %int %12853
       %8412 = OpImageFetch %v4float %18624 %12867
       %9343 = OpCompositeExtract %float %8412 0
      %16362 = OpCompositeInsert %v4float %9343 %17733 2
      %15634 = OpCompositeInsert %v4float %float_1 %16362 3
      %16359 = OpLoad %166 %3258
               OpImageWrite %16359 %10533 %15634
               OpBranch %21573
      %21573 = OpLabel
               OpReturn
               OpFunctionEnd
#endif

const uint32_t apply_gamma_table_cs[] = {
    0x07230203, 0x00010000, 0x0008000A, 0x0000606F, 0x00000000, 0x00020011,
    0x00000001, 0x00020011, 0x0000002E, 0x00020011, 0x00000031, 0x0006000B,
    0x00000001, 0x4C534C47, 0x6474732E, 0x3035342E, 0x00000000, 0x0003000E,
    0x00000000, 0x00000001, 0x0006000F, 0x00000005, 0x0000161F, 0x6E69616D,
    0x00000000, 0x00000F48, 0x00060010, 0x0000161F, 0x00000011, 0x00000010,
    0x00000008, 0x00000001, 0x00040047, 0x00000F48, 0x0000000B, 0x0000001C,
    0x00050048, 0x000003E1, 0x00000000, 0x00000023, 0x00000000, 0x00030047,
    0x000003E1, 0x00000002, 0x00040047, 0x0000167F, 0x00000022, 0x00000001,
    0x00040047, 0x0000167F, 0x00000021, 0x00000000, 0x00040047, 0x00001739,
    0x00000022, 0x00000000, 0x00040047, 0x00001739, 0x00000021, 0x00000000,
    0x00040047, 0x00000CBA, 0x00000022, 0x00000002, 0x00040047, 0x00000CBA,
    0x00000021, 0x00000000, 0x00030047, 0x00000CBA, 0x00000019, 0x00040047,
    0x00000B0F, 0x0000000B, 0x00000019, 0x00020013, 0x00000008, 0x00030021,
    0x00000502, 0x00000008, 0x00040015, 0x0000000B, 0x00000020, 0x00000000,
    0x00040017, 0x00000011, 0x0000000B, 0x00000002, 0x00040017, 0x00000014,
    0x0000000B, 0x00000003, 0x00040020, 0x00000291, 0x00000001, 0x00000014,
    0x0004003B, 0x00000291, 0x00000F48, 0x00000001, 0x0003001E, 0x000003E1,
    0x00000011, 0x00040020, 0x0000065E, 0x00000009, 0x000003E1, 0x0004003B,
    0x0000065E, 0x0000118F, 0x00000009, 0x00040015, 0x0000000C, 0x00000020,
    0x00000001, 0x0004002B, 0x0000000C, 0x00000A0B, 0x00000000, 0x00040020,
    0x0000028E, 0x00000009, 0x00000011, 0x00020014, 0x00000009, 0x00040017,
    0x0000000F, 0x00000009, 0x00000002, 0x00030016, 0x0000000D, 0x00000020,
    0x00090019, 0x00000096, 0x0000000D, 0x00000001, 0x00000000, 0x00000000,
    0x00000000, 0x00000001, 0x00000000, 0x00040020, 0x00000313, 0x00000000,
    0x00000096, 0x0004003B, 0x00000313, 0x0000167F, 0x00000000, 0x00040017,
    0x00000012, 0x0000000C, 0x00000002, 0x00040017, 0x0000001D, 0x0000000D,
    0x00000004, 0x00040017, 0x00000018, 0x0000000D, 0x00000003, 0x0004002B,
    0x0000000D, 0x00000540, 0x437F0000, 0x0004002B, 0x0000000D, 0x000000FC,
    0x3F000000, 0x00090019, 0x0000009A, 0x0000000D, 0x00000005, 0x00000000,
    0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x00040020, 0x00000317,
    0x00000000, 0x0000009A, 0x0004003B, 0x00000317, 0x00001739, 0x00000000,
    0x0004002B, 0x0000000B, 0x00000A0A, 0x00000000, 0x0004002B, 0x0000000B,
    0x00000A0D, 0x00000001, 0x0004002B, 0x0000000D, 0x0000008A, 0x3F800000,
    0x00090019, 0x000000A6, 0x0000000D, 0x00000001, 0x00000000, 0x00000000,
    0x00000000, 0x00000002, 0x0000000B, 0x00040020, 0x00000323, 0x00000000,
    0x000000A6, 0x0004003B, 0x00000323, 0x00000CBA, 0x00000000, 0x0004002B,
    0x0000000B, 0x00000A3A, 0x00000010, 0x0004002B, 0x0000000B, 0x00000A22,
    0x00000008, 0x0006002C, 0x00000014, 0x00000B0F, 0x00000A3A, 0x00000A22,
    0x00000A0D, 0x00030001, 0x0000001D, 0x00002818, 0x0006002C, 0x00000018,
    0x000003AB, 0x000000FC, 0x000000FC, 0x000000FC, 0x00050036, 0x00000008,
    0x0000161F, 0x00000000, 0x00000502, 0x000200F8, 0x00003B06, 0x000300F7,
    0x00005445, 0x00000000, 0x000300FB, 0x00000A0A, 0x00003272, 0x000200F8,
    0x00003272, 0x0004003D, 0x00000014, 0x000035C1, 0x00000F48, 0x0007004F,
    0x00000011, 0x000054D5, 0x000035C1, 0x000035C1, 0x00000000, 0x00000001,
    0x00050041, 0x0000028E, 0x00001E50, 0x0000118F, 0x00000A0B, 0x0004003D,
    0x00000011, 0x00003442, 0x00001E50, 0x000500AE, 0x0000000F, 0x00005B8D,
    0x000054D5, 0x00003442, 0x0004009A, 0x00000009, 0x00005A24, 0x00005B8D,
    0x000300F7, 0x0000477E, 0x00000002, 0x000400FA, 0x00005A24, 0x000055E8,
    0x0000477E, 0x000200F8, 0x000055E8, 0x000200F9, 0x00005445, 0x000200F8,
    0x0000477E, 0x0004003D, 0x00000096, 0x00005DC4, 0x0000167F, 0x0004007C,
    0x00000012, 0x00002925, 0x000054D5, 0x0007005F, 0x0000001D, 0x00001A18,
    0x00005DC4, 0x00002925, 0x00000002, 0x00000A0B, 0x0008004F, 0x00000018,
    0x00003F72, 0x00001A18, 0x00001A18, 0x00000000, 0x00000001, 0x00000002,
    0x0005008E, 0x00000018, 0x00003653, 0x00003F72, 0x00000540, 0x00050081,
    0x00000018, 0x000041F9, 0x00003653, 0x000003AB, 0x0004006D, 0x00000014,
    0x00002B5B, 0x000041F9, 0x0004003D, 0x0000009A, 0x000048C0, 0x00001739,
    0x00050051, 0x0000000B, 0x00003C4B, 0x00002B5B, 0x00000000, 0x0004007C,
    0x0000000C, 0x0000606E, 0x00003C4B, 0x0005005F, 0x0000001D, 0x000020DA,
    0x000048C0, 0x0000606E, 0x00050051, 0x0000000D, 0x0000246C, 0x000020DA,
    0x00000002, 0x00060052, 0x0000001D, 0x00004544, 0x0000246C, 0x00002818,
    0x00000000, 0x00050051, 0x0000000B, 0x00003234, 0x00002B5B, 0x00000001,
    0x0004007C, 0x0000000C, 0x00003242, 0x00003234, 0x0005005F, 0x0000001D,
    0x000020DB, 0x000048C0, 0x00003242, 0x00050051, 0x0000000D, 0x0000246D,
    0x000020DB, 0x00000001, 0x00060052, 0x0000001D, 0x00004545, 0x0000246D,
    0x00004544, 0x00000001, 0x00050051, 0x0000000B, 0x00003235, 0x00002B5B,
    0x00000002, 0x0004007C, 0x0000000C, 0x00003243, 0x00003235, 0x0005005F,
    0x0000001D, 0x000020DC, 0x000048C0, 0x00003243, 0x00050051, 0x0000000D,
    0x0000247F, 0x000020DC, 0x00000000, 0x00060052, 0x0000001D, 0x00003FEA,
    0x0000247F, 0x00004545, 0x00000002, 0x00060052, 0x0000001D, 0x00003D12,
    0x0000008A, 0x00003FEA, 0x00000003, 0x0004003D, 0x000000A6, 0x00003FE7,
    0x00000CBA, 0x00040063, 0x00003FE7, 0x00002925, 0x00003D12, 0x000200F9,
    0x00005445, 0x000200F8, 0x00005445, 0x000100FD, 0x00010038,
};
