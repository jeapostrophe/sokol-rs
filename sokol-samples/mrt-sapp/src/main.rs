extern crate nalgebra_glm as glm;
extern crate sokol;

use std::f32::consts::PI;
use std::mem;

use sokol::app::*;
use sokol::gfx::*;

const MSAA_SAMPLES: i32 = 4;

struct MRT {
    offscreen_pass_desc: SgPassDesc,
    offscreen_pass: SgPass,
    offscreen_pipeline: SgPipeline,
    offscreen_bindings: SgBindings,
    fsq_pipeline: SgPipeline,
    fsq_bindings: SgBindings,
    dbg_pipeline: SgPipeline,
    dbg_bindings: SgBindings,
    offscreen_pass_action: SgPassAction,
    default_pass_action: SgPassAction,
    rx: f32,
    ry: f32,
}

impl MRT {
    fn create_offscreen_pass(&mut self, width: i32, height: i32) {
        sg_destroy_pass(self.offscreen_pass);
        for att in &self.offscreen_pass_desc.color_attachments {
            sg_destroy_image(att.image);
        }
        sg_destroy_image(self.offscreen_pass_desc.depth_stencil_attachment.image);

        let offscreen_sample_count = if sg_query_features().MSAARenderTargets {
            MSAA_SAMPLES
        } else {
            1
        };

        let color_img_desc = SgImageDesc {
            render_target: true,
            width,
            height,
            min_filter: SgFilter::Linear,
            mag_filter: SgFilter::Linear,
            wrap_u: SgWrap::ClampToEdge,
            wrap_v: SgWrap::ClampToEdge,
            sample_count: offscreen_sample_count,
            ..Default::default()
        };
        let depth_img_desc = SgImageDesc {
            pixel_format: SgPixelFormat::Depth,
            ..color_img_desc
        };
        self.offscreen_pass_desc = SgPassDesc {
            color_attachments: vec![
                SgPassAttachmentDesc {
                    image: sg_make_image(SG_IMAGE_CONTENT_NONE, &color_img_desc),
                    ..Default::default()
                },
                SgPassAttachmentDesc {
                    image: sg_make_image(SG_IMAGE_CONTENT_NONE, &color_img_desc),
                    ..Default::default()
                },
                SgPassAttachmentDesc {
                    image: sg_make_image(SG_IMAGE_CONTENT_NONE, &color_img_desc),
                    ..Default::default()
                },
            ],
            depth_stencil_attachment: SgPassAttachmentDesc {
                image: sg_make_image(SG_IMAGE_CONTENT_NONE, &depth_img_desc),
                ..Default::default()
            },
        };
        self.offscreen_pass = sg_make_pass(&self.offscreen_pass_desc);

        self.fsq_bindings.fs_images.clear();
        for att in &self.offscreen_pass_desc.color_attachments {
            self.fsq_bindings.fs_images.push(att.image);
        }
    }
}

impl SApp for MRT {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        self.create_offscreen_pass(sapp_width(), sapp_height());

        let cube_vertices: [f32; 96] = [
            -1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0,
            1.0, 1.0, -1.0, 1.0,
            -1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0, 0.8,
            1.0, -1.0, 1.0, 0.8,
            1.0, 1.0, 1.0, 0.8,
            -1.0, 1.0, 1.0, 0.8,
            -1.0, -1.0, -1.0, 0.6,
            -1.0, 1.0, -1.0, 0.6,
            -1.0, 1.0, 1.0, 0.6,
            -1.0, -1.0, 1.0, 0.6,
            1.0, -1.0, -1.0, 0.4,
            1.0, 1.0, -1.0, 0.4,
            1.0, 1.0, 1.0, 0.4,
            1.0, -1.0, 1.0, 0.4,
            -1.0, -1.0, -1.0, 0.5,
            -1.0, -1.0, 1.0, 0.5,
            1.0, -1.0, 1.0, 0.5,
            1.0, -1.0, -1.0, 0.5,
            -1.0, 1.0, -1.0, 0.7,
            -1.0, 1.0, 1.0, 0.7,
            1.0, 1.0, 1.0, 0.7,
            1.0, 1.0, -1.0, 0.7,
        ];

        let cube_vbuf = sg_make_buffer(
            Some(&cube_vertices),
            &SgBufferDesc {
                size: mem::size_of_val(&cube_vertices),
                ..Default::default()
            },
        );

        let cube_indices: [u16; 36] = [
            0, 1, 2, 0, 2, 3,
            6, 5, 4, 7, 6, 4,
            8, 9, 10, 8, 10, 11,
            14, 13, 12, 15, 14, 12,
            16, 17, 18, 16, 18, 19,
            22, 21, 20, 23, 22, 20
        ];

        let cube_ibuf = sg_make_buffer(
            Some(&cube_indices),
            &SgBufferDesc {
                size: mem::size_of_val(&cube_indices),
                buffer_type: SgBufferType::IndexBuffer,
                ..Default::default()
            },
        );

        let (cube_vs_src, cube_fs_src) = match sg_query_backend() {
            SgBackend::D3D11 => (
                "cbuffer params: register(b0) {
                  float4x4 mvp;
                };
                struct vs_in {
                  float4 pos: POSITION;
                  float bright: BRIGHT;
                };
                struct vs_out {
                  float bright: BRIGHT;
                  float4 pos: SV_Position;
                };
                vs_out main(vs_in inp) {
                  vs_out outp;
                  outp.pos = mul(mvp, inp.pos);
                  outp.bright = inp.bright;
                  return outp;
                }",
                "struct fs_out {
                  float4 c0: SV_Target0;
                  float4 c1: SV_Target1;
                  float4 c2: SV_Target2;
                };
                fs_out main(float b: BRIGHT) {
                  fs_out outp;
                  outp.c0 = float4(b, 0.0, 0.0, 1.0);
                  outp.c1 = float4(0.0, b, 0.0, 1.0);
                  outp.c2 = float4(0.0, 0.0, b, 1.0);
                  return outp;
                }"
            ),
            SgBackend::MetalMacOS => (
                "#include <metal_stdlib>
                using namespace metal;
                struct params_t {
                  float4x4 mvp;
                };
                struct vs_in {
                  float4 pos [[attribute(0)]];
                  float bright [[attribute(1)]];
                };
                struct vs_out {
                  float4 pos [[position]];
                  float bright;
                };
                vertex vs_out _main(vs_in in [[stage_in]], constant params_t& params [[buffer(0)]]) {
                  vs_out out;
                  out.pos = params.mvp * in.pos;
                  out.bright = in.bright;
                  return out;
                }",
                "#include <metal_stdlib>
                using namespace metal;
                struct fs_out {
                  float4 color0 [[color(0)]];
                  float4 color1 [[color(1)]];
                  float4 color2 [[color(2)]];
                };
                fragment fs_out _main(float bright [[stage_in]]) {
                  fs_out out;
                  out.color0 = float4(bright, 0.0, 0.0, 1.0);
                  out.color1 = float4(0.0, bright, 0.0, 1.0);
                  out.color2 = float4(0.0, 0.0, bright, 1.0);
                  return out;
                }"
            ),
            SgBackend::GLCORE33 => (
                "#version 330
                uniform mat4 mvp;
                in vec4 position;
                in float bright0;
                out float bright;
                void main() {
                  gl_Position = mvp * position;
                  bright = bright0;
                }",
                "#version 330
                in float bright;
                layout(location=0) out vec4 frag_color_0;
                layout(location=1) out vec4 frag_color_1;
                layout(location=2) out vec4 frag_color_2;
                void main() {
                  frag_color_0 = vec4(bright, 0.0, 0.0, 1.0);
                  frag_color_1 = vec4(0.0, bright, 0.0, 1.0);
                  frag_color_2 = vec4(0.0, 0.0, bright, 1.0);
                }"
            ),
            _ => panic!()
        };

        let cube_shd = sg_make_shader(
            &SgShaderDesc {
                attrs: vec![
                    SgShaderAttrDesc {
                        name: "position",
                        sem_name: "POSITION",
                        ..Default::default()
                    },
                    SgShaderAttrDesc {
                        name: "bright0",
                        sem_name: "BRIGHT",
                        ..Default::default()
                    },
                ],
                vs: SgShaderStageDesc {
                    source: Some(cube_vs_src),
                    uniform_blocks: vec!(
                        SgShaderUniformBlockDesc {
                            size: 64,
                            uniforms: vec!(
                                SgShaderUniformDesc {
                                    name: "mvp",
                                    uniform_type: SgUniformType::Mat4,
                                    ..Default::default()
                                }
                            ),
                        }
                    ),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: Some(cube_fs_src),
                    ..Default::default()
                },
            },
        );

        self.offscreen_pipeline = sg_make_pipeline(
            &SgPipelineDesc {
                layout: SgLayoutDesc {
                    buffers: vec!(
                        SgBufferLayoutDesc {
                            stride: 16,
                            ..Default::default()
                        }
                    ),
                    attrs: vec!(
                        SgVertexAttrDesc {
                            format: SgVertexFormat::Float3,
                            offset: 0,
                            ..Default::default()
                        },
                        SgVertexAttrDesc {
                            format: SgVertexFormat::Float,
                            offset: 12,
                            ..Default::default()
                        },
                    ),
                },
                shader: cube_shd,
                index_type: SgIndexType::UInt16,
                depth: SgDepthState {
                    pixel_format: SgPixelFormat::Depth,
                    compare: SgCompareFunc::LessEqual,
                    write_enabled: true,
                    ..Default::default()
                },
                colors: vec![
                    SgColorState { ..Default::default() },
                    SgColorState { ..Default::default() },
                    SgColorState { ..Default::default() },
                ],
                cull_mode: SgCullMode::Back,
                sample_count: MSAA_SAMPLES,
                ..Default::default()
            }
        );

        self.offscreen_bindings = SgBindings {
            vertex_buffers: vec!(cube_vbuf),
            index_buffer: cube_ibuf,
            ..Default::default()
        };

        let quad_vertices: [f32; 8] = [
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0
        ];

        let quad_vbuf = sg_make_buffer(
            Some(&quad_vertices),
            &SgBufferDesc {
                size: mem::size_of_val(&quad_vertices),
                ..Default::default()
            },
        );

        let (fsq_vs_src, fsq_fs_src) = match sg_query_backend() {
            SgBackend::D3D11 => (
                "cbuffer params {
                  float2 offset;
                };
                struct vs_in {
                  float2 pos: POSITION;
                };
                struct vs_out {
                  float2 uv0: TEXCOORD0;
                  float2 uv1: TEXCOORD1;
                  float2 uv2: TEXCOORD2;
                  float4 pos: SV_Position;
                };
                vs_out main(vs_in inp) {
                  vs_out outp;
                  outp.pos = float4(inp.pos*2.0-1.0, 0.5, 1.0);
                  outp.uv0 = inp.pos + float2(offset.x, 0.0);
                  outp.uv1 = inp.pos + float2(0.0, offset.y);
                  outp.uv2 = inp.pos;
                  return outp;
                }",
                "Texture2D<float4> tex0: register(t0);
                Texture2D<float4> tex1: register(t1);
                Texture2D<float4> tex2: register(t2);
                sampler smp0: register(s0);
                sampler smp1: register(s1);
                sampler smp2: register(s2);
                struct fs_in {
                  float2 uv0: TEXCOORD0;
                  float2 uv1: TEXCOORD1;
                  float2 uv2: TEXCOORD2;
                };
                float4 main(fs_in inp): SV_Target0 {
                  float3 c0 = tex0.Sample(smp0, inp.uv0).xyz;
                  float3 c1 = tex1.Sample(smp1, inp.uv1).xyz;
                  float3 c2 = tex2.Sample(smp2, inp.uv2).xyz;
                  float4 c = float4(c0 + c1 + c2, 1.0);
                  return c;
                }"
            ),
            SgBackend::MetalMacOS => (
                "#include <metal_stdlib>
                using namespace metal;
                struct params_t {
                  float2 offset;
                };
                struct vs_in {
                  float2 pos [[attribute(0)]];
                };
                struct vs_out {
                  float4 pos [[position]];
                  float2 uv0;
                  float2 uv1;
                  float2 uv2;
                };
                vertex vs_out _main(vs_in in [[stage_in]], constant params_t& params [[buffer(0)]]) {
                  vs_out out;
                  out.pos = float4(in.pos*2.0-1.0, 0.5, 1.0);
                  out.uv0 = in.pos + float2(params.offset.x, 0.0);
                  out.uv1 = in.pos + float2(0.0, params.offset.y);
                  out.uv2 = in.pos;
                  return out;
                }",
                "#include <metal_stdlib>
                using namespace metal;
                struct fs_in {
                  float2 uv0;
                  float2 uv1;
                  float2 uv2;
                };
                fragment float4 _main(fs_in in [[stage_in]],
                  texture2d<float> tex0 [[texture(0)]], sampler smp0 [[sampler(0)]],
                  texture2d<float> tex1 [[texture(1)]], sampler smp1 [[sampler(1)]],
                  texture2d<float> tex2 [[texture(2)]], sampler smp2 [[sampler(2)]])
                {
                  float3 c0 = tex0.sample(smp0, in.uv0).xyz;
                  float3 c1 = tex1.sample(smp1, in.uv1).xyz;
                  float3 c2 = tex2.sample(smp2, in.uv2).xyz;
                  return float4(c0 + c1 + c2, 1.0);
                }"
            ),
            SgBackend::GLCORE33 => (
                "#version 330
                uniform vec2 offset;
                in vec2 pos;
                out vec2 uv0;
                out vec2 uv1;
                out vec2 uv2;
                void main() {
                  gl_Position = vec4(pos*2.0-1.0, 0.5, 1.0);
                  uv0 = pos + vec2(offset.x, 0.0);
                  uv1 = pos + vec2(0.0, offset.y);
                  uv2 = pos;
                }",
                "#version 330
                uniform sampler2D tex0;
                uniform sampler2D tex1;
                uniform sampler2D tex2;
                in vec2 uv0;
                in vec2 uv1;
                in vec2 uv2;
                out vec4 frag_color;
                void main() {
                  vec3 c0 = texture(tex0, uv0).xyz;
                  vec3 c1 = texture(tex1, uv1).xyz;
                  vec3 c2 = texture(tex2, uv2).xyz;
                  frag_color = vec4(c0 + c1 + c2, 1.0);
                }"
            ),
            _ => panic!()
        };

        let fsq_shd = sg_make_shader(
            &SgShaderDesc {
                attrs: vec![
                    SgShaderAttrDesc {
                        name: "pos",
                        sem_name: "POSITION",
                        ..Default::default()
                    },
                ],
                vs: SgShaderStageDesc {
                    source: Some(fsq_vs_src),
                    uniform_blocks: vec!(
                        SgShaderUniformBlockDesc {
                            size: 8,
                            uniforms: vec!(
                                SgShaderUniformDesc {
                                    name: "offset",
                                    uniform_type: SgUniformType::Float2,
                                    ..Default::default()
                                }
                            ),
                        }
                    ),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: Some(fsq_fs_src),
                    images: vec![
                        SgShaderImageDesc {
                            name: "tex0",
                            image_type: SgImageType::Texture2D,
                        },
                        SgShaderImageDesc {
                            name: "tex1",
                            image_type: SgImageType::Texture2D,
                        },
                        SgShaderImageDesc {
                            name: "tex2",
                            image_type: SgImageType::Texture2D,
                        },
                    ],
                    ..Default::default()
                },
            },
        );

        self.fsq_pipeline = sg_make_pipeline(
            &SgPipelineDesc {
                layout: SgLayoutDesc {
                    attrs: vec![
                        SgVertexAttrDesc {
                            format: SgVertexFormat::Float2,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                shader: fsq_shd,
                primitive_type: SgPrimitiveType::TriangleStrip,
                ..Default::default()
            }
        );

        self.fsq_bindings = SgBindings {
            vertex_buffers: vec!(quad_vbuf),
            fs_images: vec![
                self.offscreen_pass_desc.color_attachments[0].image,
                self.offscreen_pass_desc.color_attachments[1].image,
                self.offscreen_pass_desc.color_attachments[2].image,
            ],
            ..Default::default()
        };

        let (dbg_vs_src, dbg_fs_src) = match sg_query_backend() {
            SgBackend::D3D11 => (
                "struct vs_in {
                  float2 pos: POSITION;
                };
                struct vs_out {
                  float2 uv: TEXCOORD0;
                  float4 pos: SV_Position;
                };
                vs_out main(vs_in inp) {
                  vs_out outp;
                  outp.pos = float4(inp.pos*2.0-1.0, 0.5, 1.0);
                  outp.uv = inp.pos;
                  return outp;
                }",
                "Texture2D<float4> tex: register(t0);
                sampler smp: register(s0);
                float4 main(float2 uv: TEXCOORD0): SV_Target0 {
                  return float4(tex.Sample(smp, uv).xyz, 1.0);
                }"
            ),
            SgBackend::MetalMacOS => (
                "#include <metal_stdlib>
                using namespace metal;
                struct vs_in {
                  float2 pos [[attribute(0)]];
                };
                struct vs_out {
                  float4 pos [[position]];
                  float2 uv;
                };
                vertex vs_out _main(vs_in in [[stage_in]]) {
                  vs_out out;
                  out.pos = float4(in.pos*2.0-1.0, 0.5, 1.0);
                  out.uv = in.pos;
                  return out;
                }",
                "#include <metal_stdlib>
                using namespace metal;
                fragment float4 _main(float2 uv [[stage_in]], texture2d<float> tex [[texture(0)]], sampler smp [[sampler(0)]]) {
                  return float4(tex.sample(smp, uv).xyz, 1.0);
                }"
            ),
            SgBackend::GLCORE33 => (
                "#version 330
                in vec2 pos;
                out vec2 uv;
                void main() {
                  gl_Position = vec4(pos*2.0-1.0, 0.5, 1.0);
                  uv = pos;
                }",
                "#version 330
                uniform sampler2D tex;
                in vec2 uv;
                out vec4 frag_color;
                void main() {
                  frag_color = vec4(texture(tex,uv).xyz, 1.0);
                }"
            ),
            _ => panic!()
        };

        self.dbg_pipeline = sg_make_pipeline(&SgPipelineDesc {
            layout: SgLayoutDesc {
                attrs: vec![
                    SgVertexAttrDesc {
                        format: SgVertexFormat::Float2,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            primitive_type: SgPrimitiveType::TriangleStrip,
            shader: sg_make_shader(&SgShaderDesc {
                attrs: vec![
                    SgShaderAttrDesc {
                        name: "pos",
                        sem_name: "POSITION",
                        ..Default::default()
                    },
                ],
                vs: SgShaderStageDesc {
                    source: Some(dbg_vs_src),
                    ..Default::default()
                },
                fs: SgShaderStageDesc {
                    source: Some(dbg_fs_src),
                    images: vec![
                        SgShaderImageDesc {
                            name: "tex",
                            image_type: SgImageType::Texture2D,
                        },
                    ],
                    ..Default::default()
                },
            }),
            ..Default::default()
        });

        self.dbg_bindings = SgBindings {
            vertex_buffers: vec![quad_vbuf],
            ..Default::default()
        };
    }

    fn sapp_frame(&mut self) {
        let w: f32 = sapp_width() as f32;
        let h: f32 = sapp_height() as f32;

        let proj = glm::perspective(w / h, 60.0 * PI / 180.0, 0.01, 10.0);
        let view = glm::look_at(
            &glm::vec3(0.0, 1.5, 6.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.rx += 1.0;
        self.ry += 2.0;
        let rxm = glm::rotation(self.rx * PI / 180.0, &glm::vec3(1.0, 0.0, 0.0));
        let rym = glm::rotation(self.ry * PI / 180.0, &glm::vec3(0.0, 1.0, 0.0));
        let model = rxm * rym;

        let mvp: [[f32; 4]; 4] = (view_proj * model).into();

        sg_begin_pass(self.offscreen_pass, &self.offscreen_pass_action);
        sg_apply_pipeline(self.offscreen_pipeline);
        sg_apply_bindings(&self.offscreen_bindings);
        sg_apply_uniforms(SgShaderStage::Vertex, 0, &mvp, 64);
        sg_draw(0, 36, 1);
        sg_end_pass();

        let offset: [f32; 2] = [
            (self.rx * 0.01).sin() * 0.1, (self.ry * 0.01).sin() * 0.1
        ];

        sg_begin_default_pass(&self.default_pass_action, sapp_width(), sapp_height());
        sg_apply_pipeline(self.fsq_pipeline);
        sg_apply_bindings(&self.fsq_bindings);
        sg_apply_uniforms(SgShaderStage::Vertex, 0, &offset, 8);
        sg_draw(0, 4, 1);

        sg_apply_pipeline(self.dbg_pipeline);
        for i in 0..3 {
            sg_apply_viewport(i * 100, 0, 100, 100, false);
            self.dbg_bindings.fs_images = vec![self.offscreen_pass_desc.color_attachments[i as usize].image];
            sg_apply_bindings(&self.dbg_bindings);
            sg_draw(0, 4, 1);
        }

        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }

    fn sapp_event(&mut self, event: SAppEvent) {
        if event.event_type == SAppEventType::Resized {
            self.create_offscreen_pass(event.framebuffer_width, event.framebuffer_height);
        }
    }
}

fn main() {
    let mrt_app = MRT {
        offscreen_pass_desc: Default::default(),
        offscreen_pass: Default::default(),
        offscreen_pipeline: Default::default(),
        offscreen_bindings: Default::default(),
        fsq_pipeline: Default::default(),
        fsq_bindings: Default::default(),
        dbg_pipeline: Default::default(),
        dbg_bindings: Default::default(),
        offscreen_pass_action: SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.25, 0.0, 0.0, 1.0],
                },
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.0, 0.25, 0.0, 1.0],
                },
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.0, 0.0, 0.25, 1.0],
                },
            ),
            ..Default::default()
        },
        default_pass_action: SgPassAction {
            ..Default::default()
        },
        rx: 0.0,
        ry: 0.0,
    };

    let title = format!("mrt-sapp.rs");

    let exit_code = sapp_run(
        mrt_app,
        SAppDesc {
            width: 800,
            height: 600,
            sample_count: MSAA_SAMPLES,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
