use crate::as_texture;
use crate::backend::RenderTargetMode;
use crate::buffer_pool::TexturePool;
use crate::descriptors::Descriptors;
use crate::filters::{FilterSource, VERTEX_BUFFERS_DESCRIPTION_FILTERS};
use crate::surface::target::CommandTarget;
use crate::utils::SampleCountMap;
use bytemuck::{Pod, Zeroable};
use ruffle_render::filters::{
    DisplacementMapFilter as DisplacementMapFilterArgs, DisplacementMapFilterMode,
};
use std::sync::OnceLock;
use swf::Rectangle;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
struct DisplacementMapUniform {
    color: [f32; 4],
    components: u32, // 00000000 00000000 XXXXXXXX YYYYYYYY
    mode: u32,       // 0 wrap, 1 clamp, 2 ignore, 3 color
    scale_x: f32,
    scale_y: f32,
    source_width: f32,
    source_height: f32,
    map_width: f32,
    map_height: f32,
    offset_x: f32,
    offset_y: f32,
    viewscale_x: f32,
    viewscale_y: f32,
}

pub struct DisplacementMapFilter {
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    pipelines: SampleCountMap<OnceLock<wgpu::RenderPipeline>>,
}

impl DisplacementMapFilter {
    pub fn new(device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                            DisplacementMapUniform,
                        >() as u64),
                    },
                    count: None,
                },
            ],
            label: create_debug_label!("Displacement map filter binds").as_deref(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        Self {
            pipelines: Default::default(),
            pipeline_layout,
            bind_group_layout,
        }
    }

    fn pipeline(&self, descriptors: &Descriptors, msaa_sample_count: u32) -> &wgpu::RenderPipeline {
        self.pipelines.get_or_init(msaa_sample_count, || {
            let label = create_debug_label!("Displacement Map Filter ({} msaa)", msaa_sample_count);
            descriptors
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: label.as_deref(),
                    layout: Some(&self.pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &descriptors.shaders.displacement_map_filter,
                        entry_point: "main_vertex",
                        buffers: &VERTEX_BUFFERS_DESCRIPTION_FILTERS,
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        polygon_mode: wgpu::PolygonMode::default(),
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: msaa_sample_count,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &descriptors.shaders.displacement_map_filter,
                        entry_point: "main_fragment",
                        targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                    }),
                    multiview: None,
                })
        })
    }

    pub fn calculate_dest_rect(
        &self,
        _filter: &DisplacementMapFilterArgs,
        source_rect: Rectangle<i32>,
    ) -> Rectangle<i32> {
        source_rect
        // [NA] TODO: This *appears* to be correct, but I'm not entirely sure why Flash does this.
        // This is commented out for now because Flash actually might need us to resize the texture *after* we make it,
        // which is unsupported in our current architecture as of time of writing.

        // if filter.mode == DisplacementMapFilterMode::Color {
        //     Rectangle {
        //         x_min: source_rect.x_min - ((filter.scale_x / 2.0).floor() as i32),
        //         x_max: source_rect.x_max + (filter.scale_x.floor() as i32),
        //         y_min: source_rect.y_min - ((filter.scale_y / 2.0).floor() as i32),
        //         y_max: source_rect.y_max + (filter.scale_y.floor() as i32),
        //     }
        // } else {
        //     source_rect
        // }
    }

    pub fn apply(
        &self,
        descriptors: &Descriptors,
        texture_pool: &mut TexturePool,
        draw_encoder: &mut wgpu::CommandEncoder,
        source: &FilterSource,
        filter: &DisplacementMapFilterArgs,
    ) -> Option<CommandTarget> {
        let sample_count = source.texture.sample_count();
        let format = source.texture.format();
        let pipeline = self.pipeline(descriptors, sample_count);

        let target = CommandTarget::new(
            descriptors,
            texture_pool,
            wgpu::Extent3d {
                width: source.size.0,
                height: source.size.1,
                depth_or_array_layers: 1,
            },
            format,
            sample_count,
            RenderTargetMode::FreshWithColor(wgpu::Color::TRANSPARENT),
            draw_encoder,
        );
        let source_view = source.texture.create_view(&Default::default());
        let map_handle = filter.map_bitmap.clone()?;
        let map_texture = as_texture(&map_handle);
        let map_view = map_texture.texture.create_view(&Default::default());
        let buffer = descriptors
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: create_debug_label!("Filter arguments").as_deref(),
                contents: bytemuck::cast_slice(&[DisplacementMapUniform {
                    color: [
                        f32::from(filter.color.r) / 255.0,
                        f32::from(filter.color.g) / 255.0,
                        f32::from(filter.color.b) / 255.0,
                        f32::from(filter.color.a) / 255.0,
                    ],
                    components: ((filter.component_x as u32) << 8) | (filter.component_y as u32),
                    mode: match filter.mode {
                        DisplacementMapFilterMode::Wrap => 0,
                        DisplacementMapFilterMode::Clamp => 1,
                        DisplacementMapFilterMode::Ignore => 2,
                        DisplacementMapFilterMode::Color => 3,
                    },
                    scale_x: filter.scale_x,
                    scale_y: filter.scale_y,
                    source_width: source.texture.width() as f32,
                    source_height: source.texture.height() as f32,
                    map_width: map_texture.texture.width() as f32,
                    map_height: map_texture.texture.height() as f32,
                    offset_x: filter.map_point.0 as f32,
                    offset_y: filter.map_point.1 as f32,
                    viewscale_x: filter.viewscale_x,
                    viewscale_y: filter.viewscale_y,
                }]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let vertices = source.vertices(&descriptors.device);
        let filter_group = descriptors
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: create_debug_label!("Filter group").as_deref(),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&source_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&map_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(
                            descriptors.bitmap_samplers.get_sampler(true, true),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(
                            descriptors.bitmap_samplers.get_sampler(false, false),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: buffer.as_entire_binding(),
                    },
                ],
            });
        let mut render_pass = draw_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: create_debug_label!("Displacement map filter").as_deref(),
            color_attachments: &[target.color_attachments()],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(pipeline);

        render_pass.set_bind_group(0, &filter_group, &[]);

        render_pass.set_vertex_buffer(0, vertices.slice(..));
        render_pass.set_index_buffer(
            descriptors.quad.indices.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..6, 0, 0..1);
        drop(render_pass);
        Some(target)
    }
}
