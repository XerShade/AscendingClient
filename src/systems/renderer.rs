use graphics::{
    wgpu::{InstanceFlags, PresentMode},
    *,
};
use serde::{Deserialize, Serialize};
use winit::dpi::PhysicalSize;

pub mod fade;

pub use fade::*;

use crate::{
    data_types::*, game_content::*, Audio, Config, ItemData, NpcData, ShopData,
    TextureAllocation,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientAdapterPowerSettings {
    LowPower,
    HighPower,
}

impl ClientAdapterPowerSettings {
    pub fn parse_enum(&self) -> AdapterPowerSettings {
        match self {
            ClientAdapterPowerSettings::HighPower => {
                AdapterPowerSettings::HighPower
            }
            ClientAdapterPowerSettings::LowPower => {
                AdapterPowerSettings::LowPower
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientPresentMode {
    AutoVsync,
    AutoNoVsync,
    Fifo,
    FifoRelaxed,
    Immediate,
    Mailbox,
}

impl ClientPresentMode {
    pub fn parse_enum(&self) -> PresentMode {
        match self {
            ClientPresentMode::AutoVsync => PresentMode::AutoVsync,
            ClientPresentMode::AutoNoVsync => PresentMode::AutoNoVsync,
            ClientPresentMode::Fifo => PresentMode::Fifo,
            ClientPresentMode::FifoRelaxed => PresentMode::FifoRelaxed,
            ClientPresentMode::Immediate => PresentMode::Immediate,
            ClientPresentMode::Mailbox => PresentMode::Mailbox,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientGPUInstances {
    None,
    Debug,
    Validation,
    All,
}

impl ClientGPUInstances {
    pub fn to_flag(&self) -> InstanceFlags {
        match self {
            ClientGPUInstances::None => InstanceFlags::empty(),
            ClientGPUInstances::Debug => InstanceFlags::DEBUG,
            ClientGPUInstances::Validation => InstanceFlags::VALIDATION,
            ClientGPUInstances::All => InstanceFlags::debugging(),
        }
    }
}

pub struct TextCaret {
    pub visible: bool,
    pub index: Option<usize>,
    pub timer: f32,
}

pub struct DatabaseHolder {
    pub item: Vec<ItemData>,
    pub shop: Vec<ShopData>,
    pub npc: Vec<NpcData>,
}

pub struct SystemHolder {
    pub gfx: GfxCollection,
    pub renderer: GpuRenderer,
    pub size: PhysicalSize<f32>,
    pub scale: f64,
    pub resource: TextureAllocation,
    pub fade: Fade,
    pub map_fade: MapFade,
    pub config: Config,
    pub base: DatabaseHolder,
    pub audio: Audio,
    pub caret: TextCaret,
    pub try_once: bool,
    pub fps: usize,
}

pub struct State<Controls>
where
    Controls: camera::controls::Controls,
{
    /// World Camera Controls and time. Deturmines how the world is looked at.
    pub system: System<Controls>,
    /// Atlas Groups for Textures in GPU
    pub image_atlas: AtlasSet,
    pub text_atlas: TextAtlas,
    pub map_atlas: AtlasSet,
    pub ui_atlas: AtlasSet,
    /// Rendering Buffers and other shared data.
    pub image_renderer: ImageRenderer,
    pub text_renderer: TextRenderer,
    pub map_renderer: MapRenderer,
    pub ui_renderer: RectRenderer,
}

impl<Controls> Pass for State<Controls>
where
    Controls: camera::controls::Controls,
{
    fn render(
        &mut self,
        renderer: &GpuRenderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: renderer.frame_buffer().as_ref().unwrap(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.25,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(
                wgpu::RenderPassDepthStencilAttachment {
                    view: renderer.depth_buffer(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: wgpu::StoreOp::Store,
                    }),
                },
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Lets set the System's Shader information here, mostly Camera, Size and Time
        pass.set_bind_group(0, self.system.bind_group(), &[]);
        // Lets set the Reusable Vertices and Indicies here.
        // This is used for each Renderer, Should be more performant since it is shared.
        pass.set_vertex_buffer(0, renderer.buffer_object.vertices());
        pass.set_index_buffer(
            renderer.buffer_object.indices(),
            wgpu::IndexFormat::Uint32,
        );

        pass.render_map(renderer, &self.map_renderer, &self.map_atlas, 0);
        pass.render_map(renderer, &self.map_renderer, &self.map_atlas, 1);
        for layer in 0..=5 {
            pass.render_map(
                renderer,
                &self.map_renderer,
                &self.map_atlas,
                layer,
            );
            pass.render_image(
                renderer,
                &self.image_renderer,
                &self.image_atlas,
                &self.system,
                layer,
            );
            pass.render_text(
                renderer,
                &self.text_renderer,
                &self.text_atlas,
                layer,
            );
            pass.render_rects(
                renderer,
                &self.ui_renderer,
                &self.ui_atlas,
                &self.system,
                layer,
            );
        }
    }
}

pub fn add_image_to_buffer<Controls>(
    systems: &mut SystemHolder,
    graphics: &mut State<Controls>,
) where
    Controls: camera::controls::Controls,
{
    systems.gfx.collection.iter_mut().for_each(|data| {
        if data.1.visible {
            match &mut data.1.gfx {
                GfxType::Image(image) => {
                    graphics.image_renderer.image_update(
                        image,
                        &mut systems.renderer,
                        &mut graphics.image_atlas,
                        data.1.layer,
                    );
                }
                GfxType::Rect(rect) => {
                    graphics.ui_renderer.rect_update(
                        rect,
                        &mut systems.renderer,
                        &mut graphics.ui_atlas,
                        data.1.layer,
                    );
                }
                GfxType::Text(text) => {
                    graphics
                        .text_renderer
                        .text_update(
                            text,
                            &mut graphics.text_atlas,
                            &mut systems.renderer,
                            data.1.layer,
                        )
                        .unwrap();
                }
                GfxType::Map(map) => {
                    graphics.map_renderer.map_update(
                        map,
                        &mut systems.renderer,
                        &mut graphics.map_atlas,
                        [0, 1],
                    );
                }
            }
        }
    });
}