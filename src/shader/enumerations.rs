// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Enumerations used to specify information about shaders
//!
//! # References
//! [D3D11 Shader Enumerations, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476175(v=vs.85).aspx)

#![allow(non_camel_case_types)]

use common_version::enumerations::{ D3D_CBUFFER_TYPE, D3D_RESOURCE_RETURN_TYPE,
	D3D_TESSELLATOR_DOMAIN, D3D_TESSELLATOR_PARTITIONING,
	D3D_TESSELLATOR_OUTPUT_PRIMITIVE };

pub type D3D11_CBUFFER_TYPE = D3D_CBUFFER_TYPE;
pub type D3D11_RESOURCE_RETURN_TYPE = D3D_RESOURCE_RETURN_TYPE;
pub type D3D11_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN;
pub type D3D11_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING;
pub type D3D11_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE;

#[repr(C)] pub enum D3D_PARAMETER_FLAGS {
	D3D_PF_NONE = 0x00000000,
	D3D_PF_IN = 0x00000001,
	D3D_PF_OUT = 0x00000002,
	D3D_PF_FORCE_DWORD = 0x7fffffff
}
#[repr(C)] pub enum D3D11_SHADER_TYPE {
	D3D11_VERTEX_SHADER = 1,
	D3D11_HULL_SHADER = 2,
	D3D11_DOMAIN_SHADER = 3,
	D3D11_GEOMETRY_SHADER = 4,
	D3D11_PIXEL_SHADER = 5,
	D3D11_COMPUTE_SHADER = 6
}
#[repr(C)] pub enum D3D11_TRACE_GS_INPUT_PRIMITIVE {
	D3D11_TRACE_GS_INPUT_PRIMITIVE_UNDEFINED = 0,
	D3D11_TRACE_GS_INPUT_PRIMITIVE_POINT = 1,
	D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE = 2,
	D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE = 3,
	D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE_ADJ = 6,
	D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE_ADJ = 7
}
#[repr(C)] pub enum D3D11_TRACE_REGISTER_TYPE {
	D3D11_TRACE_OUTPUT_NULL_REGISTER = 0,
	D3D11_TRACE_INPUT_REGISTER,
	D3D11_TRACE_INPUT_PRIMITIVE_ID_REGISTER,
	D3D11_TRACE_IMMEDIATE_CONSTANT_BUFFER,
	D3D11_TRACE_TEMP_REGISTER,
	D3D11_TRACE_INDEXABLE_TEMP_REGISTER,
	D3D11_TRACE_OUTPUT_REGISTER,
	D3D11_TRACE_OUTPUT_DEPTH_REGISTER,
	D3D11_TRACE_CONSTANT_BUFFER,
	D3D11_TRACE_IMMEDIATE32,
	D3D11_TRACE_SAMPLER,
	D3D11_TRACE_RESOURCE,
	D3D11_TRACE_RASTERIZER,
	D3D11_TRACE_OUTPUT_COVERAGE_MASK,
	D3D11_TRACE_STREAM,
	D3D11_TRACE_THIS_POINTER,
	D3D11_TRACE_OUTPUT_CONTROL_POINT_ID_REGISTER,
	D3D11_TRACE_INPUT_FORK_INSTANCE_ID_REGISTER,
	D3D11_TRACE_INPUT_JOIN_INSTANCE_ID_REGISTER,
	D3D11_TRACE_INPUT_CONTROL_POINT_REGISTER,
	D3D11_TRACE_OUTPUT_CONTROL_POINT_REGISTER,
	D3D11_TRACE_INPUT_PATCH_CONSTANT_REGISTER,
	D3D11_TRACE_INPUT_DOMAIN_POINT_REGISTER,
	D3D11_TRACE_UNORDERED_ACCESS_VIEW,
	D3D11_TRACE_THREAD_GROUP_SHARED_MEMORY,
	D3D11_TRACE_INPUT_THREAD_ID_REGISTER,
	D3D11_TRACE_INPUT_THREAD_GROUP_ID_REGISTER,
	D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_REGISTER,
	D3D11_TRACE_INPUT_COVERAGE_MASK_REGISTER,
	D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_FLATTENED_REGISTER,
	D3D11_TRACE_INPUT_GS_INSTANCE_ID_REGISTER,
	D3D11_TRACE_OUTPUT_DEPTH_GREATER_EQUAL_REGISTER,
	D3D11_TRACE_OUTPUT_DEPTH_LESS_EQUAL_REGISTER,
	D3D11_TRACE_IMMEDIATE64,
	D3D11_TRACE_INPUT_CYCLE_COUNTER_REGISTER,
	D3D11_TRACE_INTERFACE_POINTER,
}
#[repr(C)] pub enum D3D11_SHADER_VERSION_TYPE {
	D3D11_SHVER_PIXEL_SHADER = 0,
	D3D11_SHVER_VERTEX_SHADER = 1,
	D3D11_SHVER_GEOMETRY_SHADER = 2,
	
	// D3D11 Shaders
	D3D11_SHVER_HULL_SHADER = 3,
	D3D11_SHVER_DOMAIN_SHADER = 4,
	D3D11_SHVER_COMPUTE_SHADER = 5,

	D3D11_SHVER_RESERVED0 = 0xFFF0,
}