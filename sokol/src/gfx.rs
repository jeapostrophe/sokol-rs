//! sokol::gfx - simple 3D API wrapper
//!
//! A Rust API to the [sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_gfx.h)
//! header-only C library.

use std::fmt;
use std::os::raw::c_void;

mod ffi {
    use std::ffi::CString;
    use std::fmt;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_void;
    use std::ptr::null;

    use crate::app::ffi::*;

    const _SG_INVALID_ID: usize = 0;
    const _SG_NUM_SHADER_STAGES: usize = 2;
    const SG_NUM_INFLIGHT_FRAMES: usize = 2;
    const SG_MAX_COLOR_ATTACHMENTS: usize = 4;
    const SG_MAX_SHADERSTAGE_BUFFERS: usize = 8;
    const SG_MAX_SHADERSTAGE_IMAGES: usize = 12;
    const SG_MAX_SHADERSTAGE_UBS: usize = 4;
    const SG_MAX_UB_MEMBERS: usize = 16;
    const SG_MAX_VERTEX_ATTRIBUTES: usize = 16;
    const SG_MAX_MIPMAPS: usize = 16;
    const _SG_MAX_TEXTUREARRAY_LAYERS: usize = 128;

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgPassAction {
        _start_canary: u32,
        colors: [super::SgColorAttachmentAction; SG_MAX_COLOR_ATTACHMENTS],
        depth: super::SgDepthAttachmentAction,
        stencil: super::SgStencilAttachmentAction,
        _end_canary: u32,
    }

    impl SgPassAction {
        pub fn make(pass_action: &super::SgPassAction) -> SgPassAction {
            let mut pa = SgPassAction {
                _start_canary: 0,
                colors: Default::default(),
                depth: pass_action.depth,
                stencil: pass_action.stencil,
                _end_canary: 0,
            };

            for (idx, color_action) in pass_action.colors.iter().enumerate() {
                pa.colors[idx] = *color_action;
            }

            pa
        }
    }

    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct SgBindings {
        _start_canary: u32,
        vertex_buffers: [super::SgBuffer; SG_MAX_SHADERSTAGE_BUFFERS],
        vertex_buffer_offsets: [c_int; SG_MAX_SHADERSTAGE_BUFFERS],
        index_buffer: super::SgBuffer,
        index_buffer_offset: c_int,
        vs_images: [super::SgImage; SG_MAX_SHADERSTAGE_IMAGES],
        fs_images: [super::SgImage; SG_MAX_SHADERSTAGE_IMAGES],
        _end_canary: u32,
    }

    impl SgBindings {
        pub fn make(bindings: &super::SgBindings) -> SgBindings {
            let mut b = SgBindings {
                index_buffer: (*bindings).index_buffer,
                index_buffer_offset: (*bindings).index_buffer_offset,
                ..Default::default()
            };

            Self::collect_buffers(&mut b, bindings);

            b
        }

        fn collect_buffers(bindings: &mut SgBindings,
                           src: &super::SgBindings) {
            for (idx, vb) in src.vertex_buffers.iter().enumerate() {
                bindings.vertex_buffers[idx] = *vb;
            }

            for (idx, vb_offs) in src.vertex_buffer_offsets.iter().enumerate() {
                bindings.vertex_buffer_offsets[idx] = *vb_offs;
            }

            for (idx, img) in src.vs_images.iter().enumerate() {
                bindings.vs_images[idx] = *img;
            }

            for (idx, img) in src.fs_images.iter().enumerate() {
                bindings.fs_images[idx] = *img;
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgDesc {
        _start_canary: u32,
        pub desc: super::SgDesc,
        mtl_device: *const c_void,
        mtl_renderpass_descriptor_cb: unsafe extern fn() -> *const c_void,
        mtl_drawable_cb: unsafe extern fn() -> *const c_void,
        mtl_global_uniform_buffer_size: c_int,
        mtl_sampler_cache_size: c_int,
        d3d11_device: *const c_void,
        d3d11_device_context: *const c_void,
        d3d11_render_target_view_cb: unsafe extern fn() -> *const c_void,
        d3d11_depth_stencil_view_cb: unsafe extern fn() -> *const c_void,
        _end_canary: u32,
    }

    impl SgDesc {
        pub fn make(desc: &super::SgDesc) -> SgDesc {
            unsafe {
                SgDesc {
                    _start_canary: 0,
                    desc: *desc,
                    mtl_device: sapp_metal_get_device(),
                    mtl_renderpass_descriptor_cb: sapp_metal_get_renderpass_descriptor,
                    mtl_drawable_cb: sapp_metal_get_drawable,
                    mtl_global_uniform_buffer_size: 0,
                    mtl_sampler_cache_size: 0,
                    d3d11_device: sapp_d3d11_get_device(),
                    d3d11_device_context: sapp_d3d11_get_device_context(),
                    d3d11_render_target_view_cb: sapp_d3d11_get_render_target_view,
                    d3d11_depth_stencil_view_cb: sapp_d3d11_get_depth_stencil_view,
                    _end_canary: 0,
                }
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgBufferDesc {
        _start_canary: u32,
        size: c_int,
        buffer_type: super::SgBufferType,
        usage: super::SgUsage,
        content: *const c_void,
        label: *const c_char,
        gl_buffers: [u32; SG_NUM_INFLIGHT_FRAMES],
        mtl_buffers: [*const c_void; SG_NUM_INFLIGHT_FRAMES],
        d3d11_buffer: *const c_void,
        _end_canary: u32,
    }

    impl SgBufferDesc {
        pub fn make<T>(content: Option<&T>, desc: &super::SgBufferDesc) -> SgBufferDesc {
            let ptr = if content.is_some() {
                content.unwrap() as *const T
            } else {
                null()
            };

            SgBufferDesc {
                _start_canary: 0,
                size: desc.size as c_int,
                buffer_type: desc.buffer_type,
                usage: desc.usage,
                content: ptr as *const c_void,
                label: null(),
                gl_buffers: [0, 0],
                mtl_buffers: [null(), null()],
                d3d11_buffer: null(),
                _end_canary: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct SgSubImageContent {
        ptr: *const c_void,
        size: c_int,
    }

    impl Default for SgSubImageContent {
        fn default() -> Self {
            Self {
                ptr: null(),
                size: 0,
            }
        }
    }

    #[repr(C)]
    pub struct SgImageContent {
        subimage: [SgSubImageContent; 6 * SG_MAX_MIPMAPS],
    }

    impl Default for SgImageContent {
        fn default() -> Self {
            Self {
                subimage: [
                    SgSubImageContent {
                        ..Default::default()
                    }; 96
                ]
            }
        }
    }

    // We can't [derive(Debug)] SgImageContent because fixed-arrays don't impl
    // Debug if their size is over 32.
    impl fmt::Debug for SgImageContent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            struct Helper([SgSubImageContent; 6 * SG_MAX_MIPMAPS]);

            impl fmt::Debug for Helper {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Debug::fmt(&self.0[..], formatter)
                }
            }

            f.debug_struct("SgImageContent")
                .field("subimage", &Helper(self.subimage))
                .finish()
        }
    }

    impl SgImageContent {
        pub fn make<T>(content: Option<&[(*const T, i32)]>) -> SgImageContent {
            let mut cnt = SgImageContent {
                ..Default::default()
            };

            match content {
                None => {}
                Some(content) => {
                    for (idx, (data, size)) in content.iter().enumerate() {
                        let ptr = *data as *const T;

                        cnt.subimage[idx] = SgSubImageContent {
                            ptr: ptr as *const c_void,
                            size: *size as i32,
                        };
                    }
                }
            };

            cnt
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgImageDesc {
        _start_canary: u32,
        image_type: super::SgImageType,
        render_target: bool,
        width: c_int,
        height: c_int,
        depth_or_layers: c_int,
        num_mipmaps: c_int,
        usage: super::SgUsage,
        pixel_format: super::SgPixelFormat,
        sample_count: c_int,
        min_filter: super::SgFilter,
        mag_filter: super::SgFilter,
        wrap_u: super::SgWrap,
        wrap_v: super::SgWrap,
        wrap_w: super::SgWrap,
        max_anisotropy: u32,
        min_lod: f32,
        max_lod: f32,
        content: SgImageContent,
        label: *const c_char,
        gl_textures: [u32; SG_NUM_INFLIGHT_FRAMES],
        mtl_textures: [*const c_void; SG_NUM_INFLIGHT_FRAMES],
        d3d11_texture: *const c_void,
        _end_canary: u32,
    }

    impl SgImageDesc {
        pub fn make<T>(content: Option<&[(*const T, i32)]>, desc: &super::SgImageDesc) -> SgImageDesc {
            SgImageDesc {
                _start_canary: 0,
                image_type: desc.image_type,
                render_target: desc.render_target,
                width: desc.width,
                height: desc.height,
                depth_or_layers: desc.depth_or_layers,
                num_mipmaps: desc.num_mipmaps,
                usage: desc.usage,
                pixel_format: desc.pixel_format,
                sample_count: desc.sample_count,
                min_filter: desc.min_filter,
                mag_filter: desc.mag_filter,
                wrap_u: desc.wrap_u,
                wrap_v: desc.wrap_v,
                wrap_w: desc.wrap_w,
                max_anisotropy: desc.max_anisotropy,
                min_lod: desc.min_lod,
                max_lod: desc.max_lod,
                content: SgImageContent::make(content),
                label: null(),
                gl_textures: [0; SG_NUM_INFLIGHT_FRAMES],
                mtl_textures: [null(); SG_NUM_INFLIGHT_FRAMES],
                d3d11_texture: null(),
                _end_canary: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct SgShaderAttrDesc {
        name: *const c_char,
        sem_name: *const c_char,
        sem_index: c_int,
    }

    impl Default for SgShaderAttrDesc {
        fn default() -> Self {
            SgShaderAttrDesc {
                name: null(),
                sem_name: null(),
                sem_index: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct SgShaderUniformDesc {
        name: *const c_char,
        uniform_type: super::SgUniformType,
        array_count: c_int,
    }

    impl Default for SgShaderUniformDesc {
        fn default() -> SgShaderUniformDesc {
            SgShaderUniformDesc {
                name: null(),
                uniform_type: super::SgUniformType::_Invalid,
                array_count: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    struct SgShaderUniformBlockDesc {
        size: c_int,
        uniforms: [SgShaderUniformDesc; SG_MAX_UB_MEMBERS],
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct SgShaderImageDesc {
        name: *const c_char,
        image_type: super::SgImageType,
    }

    impl Default for SgShaderImageDesc {
        fn default() -> Self {
            SgShaderImageDesc {
                name: null(),
                image_type: super::SgImageType::_Default,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    struct SgShaderStageDesc {
        source: *const c_char,
        byte_code: *const u8,
        byte_code_size: c_int,
        entry: *const c_char,
        uniform_blocks: [SgShaderUniformBlockDesc; SG_MAX_SHADERSTAGE_UBS],
        images: [SgShaderImageDesc; SG_MAX_SHADERSTAGE_IMAGES],
    }

    impl Default for SgShaderStageDesc {
        fn default() -> Self {
            SgShaderStageDesc {
                source: null(),
                byte_code: null(),
                byte_code_size: 0,
                entry: null(),
                uniform_blocks: [
                    Default::default(); SG_MAX_SHADERSTAGE_UBS
                ],
                images: [
                    Default::default(); SG_MAX_SHADERSTAGE_IMAGES
                ],
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgShaderDesc {
        _start_canary: u32,
        attrs: [SgShaderAttrDesc; SG_MAX_VERTEX_ATTRIBUTES],
        vs: SgShaderStageDesc,
        fs: SgShaderStageDesc,
        label: *const c_char,
        _end_canary: u32,
    }

    impl SgShaderDesc {
        pub fn make(desc: &super::SgShaderDesc) -> SgShaderDesc {
            let from_str = |s: Option<&str>| -> *const c_char {
                if s.is_some() {
                    let source = s.unwrap();
                    CString::new(source).unwrap().into_raw()
                } else {
                    null()
                }
            };

            let (vs_bytes, vs_size) = Self::collect_bytecode(desc.vs.byte_code);
            let (fs_bytes, fs_size) = Self::collect_bytecode(desc.fs.byte_code);

            let mut shd = SgShaderDesc {
                _start_canary: 0,
                attrs: Default::default(),
                vs: SgShaderStageDesc {
                    source: from_str(desc.vs.source),
                    byte_code: vs_bytes,
                    byte_code_size: vs_size,
                    entry: from_str(desc.vs.entry),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: from_str(desc.fs.source),
                    byte_code: fs_bytes,
                    byte_code_size: fs_size,
                    entry: from_str(desc.fs.entry),
                    ..Default::default()
                },
                label: null(),
                _end_canary: 0,
            };

            Self::collect_attrs(&mut shd, &desc.attrs);

            Self::collect_uniform_blocks(&mut shd.vs, &desc.vs.uniform_blocks);
            Self::collect_images(&mut shd.vs, &desc.vs.images);

            Self::collect_uniform_blocks(&mut shd.fs, &desc.fs.uniform_blocks);
            Self::collect_images(&mut shd.fs, &desc.fs.images);

            shd
        }

        fn collect_bytecode(b: Option<&[u8]>) -> (*const u8, c_int) {
            if b.is_some() {
                let bytes = b.unwrap();
                let bytes_len = bytes.len() as i32;
                (bytes.as_ptr(), bytes_len)
            } else {
                (null(), 0)
            }
        }

        fn collect_attrs(desc: &mut SgShaderDesc,
                         src: &[super::SgShaderAttrDesc]) {
            for (idx, attr) in src.iter().enumerate() {
                let name = CString::new(attr.name).unwrap();
                let sem_name = CString::new(attr.sem_name).unwrap();

                desc.attrs[idx] = SgShaderAttrDesc {
                    name: name.into_raw(),
                    sem_name: sem_name.into_raw(),
                    sem_index: attr.sem_index,
                };
            }
        }

        fn collect_uniforms(desc: &mut SgShaderUniformBlockDesc,
                            src: &[super::SgShaderUniformDesc]) {
            for (idx, u) in src.iter().enumerate() {
                let dst = &mut desc.uniforms[idx];

                let name = CString::new(u.name).unwrap();

                dst.name = name.into_raw();
                dst.uniform_type = u.uniform_type;
                dst.array_count = u.array_count;
            }
        }

        fn collect_uniform_blocks(desc: &mut SgShaderStageDesc,
                                  src: &[super::SgShaderUniformBlockDesc]) {
            for (idx, ub) in src.iter().enumerate() {
                let dst = &mut desc.uniform_blocks[idx];
                dst.size = ub.size;
                SgShaderDesc::collect_uniforms(dst, &ub.uniforms);
            }
        }

        fn collect_images(desc: &mut SgShaderStageDesc,
                          src: &[super::SgShaderImageDesc]) {
            for (idx, img) in src.iter().enumerate() {
                let dst = &mut desc.images[idx];

                let name = CString::new(img.name).unwrap();

                dst.name = name.into_raw();
                dst.image_type = img.image_type;
            }
        }
    }

    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct SgBufferLayoutDesc {
        pub stride: c_int,
        pub step_func: super::SgVertexStep,
        pub step_rate: c_int,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgVertexAttrDesc {
        buffer_index: c_int,
        offset: c_int,
        format: super::SgVertexFormat,
    }

    impl Default for SgVertexAttrDesc {
        fn default() -> Self {
            SgVertexAttrDesc {
                buffer_index: 0,
                offset: 0,
                format: super::SgVertexFormat::_Invalid,
            }
        }
    }

    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct SgLayoutDesc {
        buffers: [SgBufferLayoutDesc; SG_MAX_SHADERSTAGE_BUFFERS],
        attrs: [SgVertexAttrDesc; SG_MAX_VERTEX_ATTRIBUTES],
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgBlendState {
        enabled: bool,
        src_factor_rgb: super::SgBlendFactor,
        dst_factor_rgb: super::SgBlendFactor,
        op_rgb: super::SgBlendOp,
        src_factor_alpha: super::SgBlendFactor,
        dst_factor_alpha: super::SgBlendFactor,
        op_alpha: super::SgBlendOp,
        color_write_mask: u8,
        color_attachment_count: c_int,
        color_format: super::SgPixelFormat,
        depth_format: super::SgPixelFormat,
        blend_color: [f32; 4],
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgPipelineDesc {
        _start_canary: u32,
        layout: SgLayoutDesc,
        shader: super::SgShader,
        primitive_type: super::SgPrimitiveType,
        index_type: super::SgIndexType,
        depth_stencil: super::SgDepthStencilState,
        blend: SgBlendState,
        rasterizer: super::SgRasterizerState,
        label: *const c_char,
        _end_canary: u32,
    }

    impl SgPipelineDesc {
        pub fn make(desc: &super::SgPipelineDesc) -> SgPipelineDesc {
            let blend = desc.blend;

            let mut pip = SgPipelineDesc {
                _start_canary: 0,
                layout: Default::default(),
                shader: (*desc).shader,
                primitive_type: (*desc).primitive_type,
                index_type: (*desc).index_type,
                depth_stencil: (*desc).depth_stencil,
                blend: SgBlendState {
                    enabled: blend.enabled,
                    src_factor_rgb: blend.src_factor_rgb,
                    dst_factor_rgb: blend.dst_factor_rgb,
                    op_rgb: blend.op_rgb,
                    src_factor_alpha: blend.src_factor_alpha,
                    dst_factor_alpha: blend.dst_factor_alpha,
                    op_alpha: blend.op_alpha,
                    color_write_mask: blend.color_write_mask.bits(),
                    color_attachment_count: blend.color_attachment_count,
                    color_format: blend.color_format,
                    depth_format: blend.depth_format,
                    blend_color: blend.blend_color,
                },
                rasterizer: (*desc).rasterizer,
                label: null(),
                _end_canary: 0,
            };

            SgPipelineDesc::collect_layout_buffers(&mut pip.layout, &desc.layout.buffers);
            SgPipelineDesc::collect_layout_attrs(&mut pip.layout, &desc.layout.attrs);

            pip
        }

        fn collect_layout_buffers(desc: &mut SgLayoutDesc,
                                  src: &[super::SgBufferLayoutDesc]) {
            for (idx, buf) in src.iter().enumerate() {
                desc.buffers[idx] = SgBufferLayoutDesc {
                    stride: buf.stride as c_int,
                    step_func: buf.step_func,
                    step_rate: buf.step_rate,
                };
            }
        }

        fn collect_layout_attrs(desc: &mut SgLayoutDesc,
                                src: &[super::SgVertexAttrDesc]) {
            for (idx, attr) in src.iter().enumerate() {
                desc.attrs[idx] = SgVertexAttrDesc {
                    buffer_index: attr.buffer_index,
                    offset: attr.offset,
                    format: attr.format,
                };
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SgPassDesc {
        _start_canary: u32,
        color_attachments: [super::SgAttachmentDesc; SG_MAX_COLOR_ATTACHMENTS],
        depth_stencil_attachment: super::SgAttachmentDesc,
        label: *const c_char,
        _end_canary: u32,
    }

    impl SgPassDesc {
        pub fn make(desc: &super::SgPassDesc) -> SgPassDesc {
            let mut pass = SgPassDesc {
                _start_canary: 0,
                color_attachments: Default::default(),
                depth_stencil_attachment: desc.depth_stencil_attachment,
                label: null(),
                _end_canary: 0,
            };

            for (idx, att) in desc.color_attachments.iter().enumerate() {
                pass.color_attachments[idx] = *att;
            }

            pass
        }
    }

    extern {
        pub fn sg_setup(desc: *const SgDesc);
        pub fn sg_shutdown();
        pub fn sg_isvalid() -> bool;
        pub fn sg_query_desc() -> SgDesc;
        pub fn sg_query_backend() -> super::SgBackend;
        pub fn sg_query_features() -> super::SgFeatures;
        pub fn sg_reset_state_cache();

        pub fn sg_make_buffer(desc: *const SgBufferDesc) -> super::SgBuffer;
        pub fn sg_make_image(desc: *const SgImageDesc) -> super::SgImage;
        pub fn sg_make_shader(desc: *const SgShaderDesc) -> super::SgShader;
        pub fn sg_make_pipeline(desc: *const SgPipelineDesc) -> super::SgPipeline;
        pub fn sg_make_pass(desc: *const SgPassDesc) -> super::SgPass;

        pub fn sg_destroy_buffer(buf: super::SgBuffer);
        pub fn sg_destroy_image(img: super::SgImage);
        pub fn sg_destroy_shader(shd: super::SgShader);
        pub fn sg_destroy_pipeline(pip: super::SgPipeline);
        pub fn sg_destroy_pass(pass: super::SgPass);

        pub fn sg_update_buffer(buf: super::SgBuffer, data_ptr: *const c_void, data_size: c_int);
        pub fn sg_update_image(img: super::SgImage, data: *const SgImageContent);
        pub fn sg_append_buffer(buf: super::SgBuffer, data_ptr: *const c_void, data_size: c_int) -> c_int;
        pub fn sg_query_buffer_overflow(buf: super::SgBuffer) -> bool;

        pub fn sg_query_buffer_state(buf: super::SgBuffer) -> super::SgResourceState;
        pub fn sg_query_image_state(img: super::SgImage) -> super::SgResourceState;
        pub fn sg_query_shader_state(shd: super::SgShader) -> super::SgResourceState;
        pub fn sg_query_pipeline_state(pip: super::SgPipeline) -> super::SgResourceState;
        pub fn sg_query_pass_state(pass: super::SgPass) -> super::SgResourceState;

        pub fn sg_begin_default_pass(pass_action: *const SgPassAction,
                                     width: c_int,
                                     height: c_int);
        pub fn sg_begin_pass(pass: super::SgPass,
                             pass_action: *const SgPassAction);
        pub fn sg_apply_viewport(x: c_int, y: c_int,
                                 width: c_int, height: c_int,
                                 origin_top_left: bool);
        pub fn sg_apply_scissor_rect(x: c_int, y: c_int,
                                     width: c_int, height: c_int,
                                     origin_top_left: bool);
        pub fn sg_apply_pipeline(pip: super::SgPipeline);
        pub fn sg_apply_bindings(bindings: *const SgBindings);
        pub fn sg_apply_uniforms(stage: super::SgShaderStage,
                                 ub_index: c_int,
                                 data: *const c_void,
                                 num_bytes: c_int);
        pub fn sg_draw(base_element: c_int,
                       num_elements: c_int,
                       num_instances: c_int);
        pub fn sg_end_pass();

        pub fn sg_commit();
    }
}

/*
    resource id typedefs
*/

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgBuffer {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgImage {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgShader {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgPipeline {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgPass {
    id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgContext {
    id: i32,
}

/*
    enums
*/

#[repr(C)]
#[derive(Debug)]
pub enum SgBackend {
    GLCORE33,
    GLES2,
    GLES3,
    D3D11,
    MetalIOS,
    MetalMacOS,
    Dummy,
}

#[repr(C)]
#[derive(Debug)]
pub struct SgFeatures {
    pub Instancing: bool,
    pub OriginTopLeft: bool,
    pub MultipleRenderTarget: bool,
    pub MSAARenderTargets: bool,
    pub ImageType3D: bool,
    pub ImageTypeArray: bool,
    pub ImageClampToBorder: bool,
    pub MRTIndependentBlendState: bool,
    pub MRTIndependentWriteMask: bool,
}

#[repr(C)]
#[derive(Debug)]
pub enum SgResourceState {
    Initial,
    Alloc,
    Valid,
    Failed,
    Invalid,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgUsage {
    _Default,
    Immutable,
    Dynamic,
    Stream,
}

impl Default for SgUsage {
    fn default() -> Self {
        SgUsage::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgBufferType {
    _Default,
    VertexBuffer,
    IndexBuffer,
}

impl Default for SgBufferType {
    fn default() -> Self {
        SgBufferType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgIndexType {
    _Default,
    None,
    UInt16,
    UInt32,
}

impl Default for SgIndexType {
    fn default() -> Self {
        SgIndexType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgImageType {
    _Default,
    Texture2D,
    TextureCube,
    Texture3D,
    TextureArray,
}

impl Default for SgImageType {
    fn default() -> Self {
        SgImageType::_Default
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SgCubeFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[repr(C)]
#[derive(Debug)]
pub enum SgShaderStage {
    Vertex,
    Fragment,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgPixelFormat {
    _Default,
    None,
    RGBA8,
    RGB8,
    RGBA4,
    RGB5,
    RGB5A1,
    RGB10A2,
    RGBA32F,
    RGBA16F,
    R32F,
    R16F,
    L8,
    DXT1,
    DXT3,
    DXT5,
    Depth,
    DepthStencil,
    PVRTC2RGB,
    PVRTC4RGB,
    PVRTC2RGBA,
    PVRTC4RGBA,
    ETC2RGB8,
    ETC2SRGB8,
}

impl Default for SgPixelFormat {
    fn default() -> Self {
        SgPixelFormat::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgPrimitiveType {
    _Default,
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl Default for SgPrimitiveType {
    fn default() -> Self {
        SgPrimitiveType::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgFilter {
    _Default,
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl Default for SgFilter {
    fn default() -> Self {
        SgFilter::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgWrap {
    _Default,
    Repeat,
    ClampToEdge,
    MirrorRepeat,
}

impl Default for SgWrap {
    fn default() -> Self {
        SgWrap::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgVertexFormat {
    _Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    Byte4N,
    UByte4,
    UByte4N,
    Short2,
    Short2N,
    Short4,
    Short4N,
    UInt10N2,
}

impl Default for SgVertexFormat {
    fn default() -> Self {
        SgVertexFormat::_Invalid
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgVertexStep {
    _Default,
    PerVertex,
    PerInstance,
}

impl Default for SgVertexStep {
    fn default() -> Self {
        SgVertexStep::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgUniformType {
    _Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Mat4,
}

impl Default for SgUniformType {
    fn default() -> Self {
        SgUniformType::_Invalid
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgCullMode {
    _Default,
    None,
    Front,
    Back,
}

impl Default for SgCullMode {
    fn default() -> Self {
        SgCullMode::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgFaceWinding {
    _Default,
    CCW,
    CW,
}

impl Default for SgFaceWinding {
    fn default() -> Self {
        SgFaceWinding::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgCompareFunc {
    _Default,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl Default for SgCompareFunc {
    fn default() -> Self {
        SgCompareFunc::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgStencilOp {
    _Default,
    Keep,
    Zero,
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

impl Default for SgStencilOp {
    fn default() -> Self {
        SgStencilOp::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgBlendFactor {
    _Default,
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
    BlendAlpha,
    OneMinusBlendAlpha,
}

impl Default for SgBlendFactor {
    fn default() -> Self {
        SgBlendFactor::_Default
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgBlendOp {
    _Default,
    Add,
    Subtract,
    ReverseSubtract,
}

impl Default for SgBlendOp {
    fn default() -> Self {
        SgBlendOp::_Default
    }
}

bitflags! {
    #[derive(Default)]
    pub struct SgColorMask: u8 {
        const NONE = 0x10;
        const R = 0x01;
        const G = 0x02;
        const B = 0x04;
        const A = 0x08;
        const RGB = 0x07;
        const RGBA = 0x0f;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SgAction {
    _Default,
    Clear,
    Load,
    DontCare,
}

impl Default for SgAction {
    fn default() -> SgAction {
        SgAction::_Default
    }
}

/*
    structs
*/

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgColorAttachmentAction {
    pub action: SgAction,
    pub val: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgDepthAttachmentAction {
    pub action: SgAction,
    pub val: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgStencilAttachmentAction {
    pub action: SgAction,
    pub val: u8,
}

#[derive(Default, Debug)]
pub struct SgPassAction {
    pub colors: Vec<SgColorAttachmentAction>,
    pub depth: SgDepthAttachmentAction,
    pub stencil: SgStencilAttachmentAction,
}

#[derive(Default, Debug)]
pub struct SgBindings {
    pub vertex_buffers: Vec<SgBuffer>,
    pub vertex_buffer_offsets: Vec<i32>,
    pub index_buffer: SgBuffer,
    pub index_buffer_offset: i32,
    pub vs_images: Vec<SgImage>,
    pub fs_images: Vec<SgImage>,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgDesc {
    pub buffer_pool_size: i32,
    pub image_pool_size: i32,
    pub shader_pool_size: i32,
    pub pipeline_pool_size: i32,
    pub pass_pool_size: i32,
    pub context_pool_size: i32,
    pub gl_force_gles2: bool,
}

#[derive(Default, Debug)]
pub struct SgBufferDesc {
    pub size: usize,
    pub buffer_type: SgBufferType,
    pub usage: SgUsage,
}

pub const SG_BUFFER_CONTENT_NONE: Option<&u8> = None;

#[derive(Default, Debug)]
pub struct SgImageDesc {
    pub image_type: SgImageType,
    pub render_target: bool,
    pub width: i32,
    pub height: i32,
    pub depth_or_layers: i32,
    pub num_mipmaps: i32,
    pub usage: SgUsage,
    pub pixel_format: SgPixelFormat,
    pub sample_count: i32,
    pub min_filter: SgFilter,
    pub mag_filter: SgFilter,
    pub wrap_u: SgWrap,
    pub wrap_v: SgWrap,
    pub wrap_w: SgWrap,
    pub max_anisotropy: u32,
    pub min_lod: f32,
    pub max_lod: f32,
}

pub const SG_IMAGE_CONTENT_NONE: Option<&[(*const u8, i32)]> = None;

#[derive(Default, Debug)]
pub struct SgShaderAttrDesc<'a> {
    pub name: &'a str,
    pub sem_name: &'a str,
    pub sem_index: i32,
}

#[derive(Default, Debug)]
pub struct SgShaderUniformDesc<'a> {
    pub name: &'a str,
    pub uniform_type: SgUniformType,
    pub array_count: i32,
}

#[derive(Default, Debug)]
pub struct SgShaderUniformBlockDesc<'a> {
    pub size: i32,
    pub uniforms: Vec<SgShaderUniformDesc<'a>>,
}

#[derive(Default, Debug)]
pub struct SgShaderImageDesc<'a> {
    pub name: &'a str,
    pub image_type: SgImageType,
}

#[derive(Default, Debug)]
pub struct SgShaderStageDesc<'a> {
    pub source: Option<&'a str>,
    pub byte_code: Option<&'a [u8]>,
    pub entry: Option<&'a str>,
    pub uniform_blocks: Vec<SgShaderUniformBlockDesc<'a>>,
    pub images: Vec<SgShaderImageDesc<'a>>,
}

#[derive(Debug)]
pub struct SgShaderDesc<'a> {
    pub attrs: Vec<SgShaderAttrDesc<'a>>,
    pub vs: SgShaderStageDesc<'a>,
    pub fs: SgShaderStageDesc<'a>,
}

#[derive(Default, Debug)]
pub struct SgBufferLayoutDesc {
    pub stride: usize,
    pub step_func: SgVertexStep,
    pub step_rate: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgStencilState {
    pub fail_op: SgStencilOp,
    pub depth_fail_op: SgStencilOp,
    pub pass_op: SgStencilOp,
    pub compare_func: SgCompareFunc,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgDepthStencilState {
    pub stencil_front: SgStencilState,
    pub stencil_back: SgStencilState,
    pub depth_compare_func: SgCompareFunc,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub stencil_ref: u8,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct SgBlendState {
    pub enabled: bool,
    pub src_factor_rgb: SgBlendFactor,
    pub dst_factor_rgb: SgBlendFactor,
    pub op_rgb: SgBlendOp,
    pub src_factor_alpha: SgBlendFactor,
    pub dst_factor_alpha: SgBlendFactor,
    pub op_alpha: SgBlendOp,
    pub color_write_mask: SgColorMask,
    pub color_attachment_count: i32,
    pub color_format: SgPixelFormat,
    pub depth_format: SgPixelFormat,
    pub blend_color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgRasterizerState {
    pub alpha_to_coverage_enabled: bool,
    pub cull_mode: SgCullMode,
    pub face_winding: SgFaceWinding,
    pub sample_count: i32,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

#[derive(Default, Debug)]
pub struct SgVertexAttrDesc {
    pub buffer_index: i32,
    pub offset: i32,
    pub format: SgVertexFormat,
}

#[derive(Default, Debug)]
pub struct SgLayoutDesc {
    pub buffers: Vec<SgBufferLayoutDesc>,
    pub attrs: Vec<SgVertexAttrDesc>,
}

#[derive(Default, Debug)]
pub struct SgPipelineDesc {
    pub shader: SgShader,
    pub layout: SgLayoutDesc,
    pub primitive_type: SgPrimitiveType,
    pub index_type: SgIndexType,
    pub depth_stencil: SgDepthStencilState,
    pub blend: SgBlendState,
    pub rasterizer: SgRasterizerState,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SgAttachmentDescValue {
    pub face: i32,
    pub layer: i32,
    pub slice: i32,
}

impl Default for SgAttachmentDescValue {
    fn default() -> Self {
        Self {
            face: 0,
        }
    }
}

impl fmt::Debug for SgAttachmentDescValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.debug_struct("(union) SgAttachmentDescValue")
                .field("face", &self.face)
                .field("layer", &self.layer)
                .field("slice", &self.slice)
                .finish()
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SgAttachmentDesc {
    pub image: SgImage,
    pub mip_level: i32,
    pub u: SgAttachmentDescValue,
}

#[derive(Default, Debug)]
pub struct SgPassDesc {
    pub color_attachments: Vec<SgAttachmentDesc>,
    pub depth_stencil_attachment: SgAttachmentDesc,
}

/*
    functions
*/

pub fn sg_setup(desc: &SgDesc) {
    unsafe {
        ffi::sg_setup(&ffi::SgDesc::make(desc));
    }
}

pub fn sg_shutdown() {
    unsafe {
        ffi::sg_shutdown();
    }
}

pub fn sg_isvalid() -> bool {
    unsafe {
        ffi::sg_isvalid()
    }
}

pub fn sg_query_desc() -> SgDesc {
    unsafe {
        let desc = ffi::sg_query_desc();
        desc.desc
    }
}

pub fn sg_query_backend() -> SgBackend {
    unsafe {
        ffi::sg_query_backend()
    }
}

pub fn sg_query_features() -> SgFeatures {
    unsafe {
        ffi::sg_query_features()
    }
}

pub fn sg_reset_state_cache() {
    unsafe {
        ffi::sg_reset_state_cache();
    }
}

pub fn sg_make_buffer<T>(content: Option<&T>, desc: &SgBufferDesc) -> SgBuffer {
    unsafe {
        ffi::sg_make_buffer(&ffi::SgBufferDesc::make(content, desc))
    }
}

pub fn sg_make_image<T>(content: Option<&[(*const T, i32)]>, desc: &SgImageDesc) -> SgImage {
    unsafe {
        ffi::sg_make_image(&ffi::SgImageDesc::make(content, desc))
    }
}

pub fn sg_make_shader(desc: &SgShaderDesc) -> SgShader {
    unsafe {
        ffi::sg_make_shader(&ffi::SgShaderDesc::make(desc))
    }
}

pub fn sg_make_pipeline(desc: &SgPipelineDesc) -> SgPipeline {
    unsafe {
        ffi::sg_make_pipeline(&ffi::SgPipelineDesc::make(desc))
    }
}

pub fn sg_make_pass(desc: &SgPassDesc) -> SgPass {
    unsafe {
        ffi::sg_make_pass(&ffi::SgPassDesc::make(desc))
    }
}

pub fn sg_destroy_buffer(buf: SgBuffer) {
    unsafe {
        ffi::sg_destroy_buffer(buf);
    }
}

pub fn sg_destroy_image(img: SgImage) {
    unsafe {
        ffi::sg_destroy_image(img);
    }
}

pub fn sg_destroy_shader(shd: SgShader) {
    unsafe {
        ffi::sg_destroy_shader(shd);
    }
}

pub fn sg_destroy_pipeline(pip: SgPipeline) {
    unsafe {
        ffi::sg_destroy_pipeline(pip);
    }
}

pub fn sg_destroy_pass(pass: SgPass) {
    unsafe {
        ffi::sg_destroy_pass(pass);
    }
}

pub fn sg_update_buffer<T>(buf: SgBuffer, content: &T, content_size: i32) {
    unsafe {
        let ptr = content as *const T;
        ffi::sg_update_buffer(buf, ptr as *const c_void, content_size);
    }
}

pub fn sg_update_image<T>(img: SgImage, content: &[(*const T, i32)]) {
    unsafe {
        ffi::sg_update_image(img, &ffi::SgImageContent::make(Some(content)));
    }
}

pub fn sg_append_buffer<T>(buf: SgBuffer, content: &T, content_size: i32) -> i32 {
    unsafe {
        let ptr = content as *const T;
        ffi::sg_append_buffer(buf, ptr as *const c_void, content_size)
    }
}

pub fn sg_query_buffer_overflow(buf: SgBuffer) -> bool {
    unsafe {
        ffi::sg_query_buffer_overflow(buf)
    }
}

pub fn sg_query_buffer_state(buf: SgBuffer) -> SgResourceState {
    unsafe {
        ffi::sg_query_buffer_state(buf)
    }
}

pub fn sg_query_image_state(img: SgImage) -> SgResourceState {
    unsafe {
        ffi::sg_query_image_state(img)
    }
}

pub fn sg_query_shader_state(shd: SgShader) -> SgResourceState {
    unsafe {
        ffi::sg_query_shader_state(shd)
    }
}

pub fn sg_query_pipeline_state(pip: SgPipeline) -> SgResourceState {
    unsafe {
        ffi::sg_query_pipeline_state(pip)
    }
}

pub fn sg_query_pass_state(pass: SgPass) -> SgResourceState {
    unsafe {
        ffi::sg_query_pass_state(pass)
    }
}

pub fn sg_begin_default_pass(pass_action: &SgPassAction, width: i32, height: i32) {
    let action = ffi::SgPassAction::make(pass_action);
    unsafe {
        ffi::sg_begin_default_pass(&action, width, height);
    }
}

pub fn sg_begin_pass(pass: SgPass,
                     pass_action: &SgPassAction) {
    let action = ffi::SgPassAction::make(pass_action);
    unsafe {
        ffi::sg_begin_pass(pass, &action);
    }
}

pub fn sg_apply_viewport(x: i32, y: i32,
                         width: i32, height: i32,
                         origin_top_left: bool) {
    unsafe {
        ffi::sg_apply_viewport(x, y, width, height, origin_top_left);
    }
}

pub fn sg_apply_scissor_rect(x: i32, y: i32,
                             width: i32, height: i32,
                             origin_top_left: bool) {
    unsafe {
        ffi::sg_apply_scissor_rect(x, y, width, height, origin_top_left);
    }
}

pub fn sg_apply_pipeline(pip: SgPipeline) {
    unsafe {
        ffi::sg_apply_pipeline(pip);
    }
}

pub fn sg_apply_bindings(bindings: &SgBindings) {
    unsafe {
        ffi::sg_apply_bindings(&ffi::SgBindings::make(bindings));
    }
}

pub fn sg_apply_uniforms<T>(stage: SgShaderStage,
                            ub_index: i32,
                            data: &T,
                            num_bytes: i32) {
    let ptr = data as *const T;

    unsafe {
        ffi::sg_apply_uniforms(stage,
                               ub_index,
                               ptr as *const c_void,
                               num_bytes);
    }
}

pub fn sg_draw(base_element: i32,
               num_elements: i32,
               num_instances: i32) {
    unsafe {
        ffi::sg_draw(base_element, num_elements, num_instances);
    }
}

pub fn sg_end_pass() {
    unsafe {
        ffi::sg_end_pass();
    }
}

pub fn sg_commit() {
    unsafe {
        ffi::sg_commit();
    }
}
