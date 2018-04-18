use fnv::FnvHashMap;

use amethyst_assets::Handle;
use amethyst_core::specs::Fetch;
use amethyst_renderer::{Material, Texture, TextureOffset};
use minterpolate::InterpolationPrimitive;

use {AnimationSampling, ApplyData, BlendMethod};

/// Textures used by texture animations
#[derive(Debug, Default)]
pub struct MaterialTextureSet {
    textures: FnvHashMap<usize, Handle<Texture>>,
    texture_inverse: FnvHashMap<Handle<Texture>, usize>,
}

impl MaterialTextureSet {
    pub fn new() -> Self {
        MaterialTextureSet {
            textures: FnvHashMap::default(),
            texture_inverse: FnvHashMap::default(),
        }
    }

    pub fn handle(&self, index: usize) -> Option<Handle<Texture>> {
        self.textures.get(&index).cloned()
    }

    pub fn index(&self, handle: &Handle<Texture>) -> Option<usize> {
        self.texture_inverse.get(handle).cloned()
    }

    pub fn insert(&mut self, index: usize, handle: Handle<Texture>) {
        self.textures.insert(index, handle.clone());
        self.texture_inverse.insert(handle, index);
    }

    pub fn remove(&mut self, index: usize) {
        if let Some(handle) = self.textures.remove(&index) {
            self.texture_inverse.remove(&handle);
        }
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.texture_inverse.clear();
    }
}

/// Sampler primitive for Material animations
/// Note that material can only ever be animated with `Step`, or a panic will occur.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MaterialPrimitive {
    Texture(usize),
    Offset((f32, f32), (f32, f32)),
}

impl InterpolationPrimitive for MaterialPrimitive {
    fn add(&self, _: &Self) -> Self {
        panic!("Cannot add MaterialPrimitive")
    }

    fn sub(&self, _: &Self) -> Self {
        panic!("Cannot sub MaterialPrimitive")
    }

    fn mul(&self, _: f32) -> Self {
        panic!("Cannot mul MaterialPrimitive")
    }

    fn dot(&self, _: &Self) -> f32 {
        panic!("Cannot dot MaterialPrimitive")
    }

    fn magnitude2(&self) -> f32 {
        panic!("Cannot magnitude2 MaterialPrimitive")
    }

    fn magnitude(&self) -> f32 {
        panic!("Cannot magnitude MaterialPrimitive")
    }

    fn normalize(&self) -> Self {
        panic!("Cannot normalize MaterialPrimitive")
    }
}

/// Channels that are animatable on `Material`
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MaterialChannel {
    AlbedoTexture,
    AlbedoOffset,
    EmissionTexture,
    EmissionOffset,
    NormalTexture,
    NormalOffset,
    MetallicTexture,
    MetallicOffset,
    RoughnessTexture,
    RoughnessOffset,
    AmbientOcclusionTexture,
    AmbientOcclusionOffset,
    CaveatTexture,
    CaveatOffset,
}

impl<'a> ApplyData<'a> for Material {
    type ApplyData = Fetch<'a, MaterialTextureSet>;
}

fn offset(offset: &TextureOffset) -> MaterialPrimitive {
    MaterialPrimitive::Offset(offset.u, offset.v)
}

fn texture_offset(u: (f32, f32), v: (f32, f32)) -> TextureOffset {
    TextureOffset { u, v }
}

impl AnimationSampling for Material {
    type Primitive = MaterialPrimitive;
    type Channel = MaterialChannel;

    fn apply_sample(
        &mut self,
        channel: &Self::Channel,
        data: &Self::Primitive,
        extra: &Fetch<MaterialTextureSet>,
    ) {
        match (*channel, *data) {
            (MaterialChannel::AlbedoTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.albedo = handle;
                }
            }
            (MaterialChannel::EmissionTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.emission = handle;
                }
            }
            (MaterialChannel::NormalTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.normal = handle;
                }
            }
            (MaterialChannel::MetallicTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.metallic = handle;
                }
            }
            (MaterialChannel::RoughnessTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.roughness = handle;
                }
            }
            (MaterialChannel::AmbientOcclusionTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.ambient_occlusion = handle;
                }
            }
            (MaterialChannel::CaveatTexture, MaterialPrimitive::Texture(i)) => {
                if let Some(handle) = extra.handle(i) {
                    self.caveat = handle;
                }
            }

            (MaterialChannel::AlbedoOffset, MaterialPrimitive::Offset(u, v)) => {
                self.albedo_offset = texture_offset(u, v)
            }
            (MaterialChannel::EmissionOffset, MaterialPrimitive::Offset(u, v)) => {
                self.emission_offset = texture_offset(u, v)
            }
            (MaterialChannel::NormalOffset, MaterialPrimitive::Offset(u, v)) => {
                self.normal_offset = texture_offset(u, v)
            }
            (MaterialChannel::MetallicOffset, MaterialPrimitive::Offset(u, v)) => {
                self.metallic_offset = texture_offset(u, v)
            }
            (MaterialChannel::RoughnessOffset, MaterialPrimitive::Offset(u, v)) => {
                self.roughness_offset = texture_offset(u, v)
            }
            (MaterialChannel::AmbientOcclusionOffset, MaterialPrimitive::Offset(u, v)) => {
                self.ambient_occlusion_offset = texture_offset(u, v)
            }
            (MaterialChannel::CaveatOffset, MaterialPrimitive::Offset(u, v)) => {
                self.caveat_offset = texture_offset(u, v)
            }

            _ => panic!("Bad combination of data in Material animation"),
        }
    }

    fn current_sample(
        &self,
        channel: &Self::Channel,
        extra: &Fetch<MaterialTextureSet>,
    ) -> Self::Primitive {
        match *channel {
            MaterialChannel::AlbedoTexture => {
                MaterialPrimitive::Texture(extra.index(&self.albedo).unwrap())
            }
            MaterialChannel::EmissionTexture => {
                MaterialPrimitive::Texture(extra.index(&self.emission).unwrap())
            }
            MaterialChannel::NormalTexture => {
                MaterialPrimitive::Texture(extra.index(&self.normal).unwrap())
            }
            MaterialChannel::MetallicTexture => {
                MaterialPrimitive::Texture(extra.index(&self.metallic).unwrap())
            }
            MaterialChannel::RoughnessTexture => {
                MaterialPrimitive::Texture(extra.index(&self.roughness).unwrap())
            }
            MaterialChannel::AmbientOcclusionTexture => {
                MaterialPrimitive::Texture(extra.index(&self.ambient_occlusion).unwrap())
            }
            MaterialChannel::CaveatTexture => {
                MaterialPrimitive::Texture(extra.index(&self.caveat).unwrap())
            }
            MaterialChannel::AlbedoOffset => offset(&self.albedo_offset),
            MaterialChannel::EmissionOffset => offset(&self.emission_offset),
            MaterialChannel::NormalOffset => offset(&self.normal_offset),
            MaterialChannel::MetallicOffset => offset(&self.metallic_offset),
            MaterialChannel::RoughnessOffset => offset(&self.roughness_offset),
            MaterialChannel::AmbientOcclusionOffset => offset(&self.ambient_occlusion_offset),
            MaterialChannel::CaveatOffset => offset(&self.caveat_offset),
        }
    }

    fn default_primitive(_: &Self::Channel) -> Self::Primitive {
        panic!("Blending is not applicable to Material animation")
    }

    fn blend_method(&self, _: &Self::Channel) -> Option<BlendMethod> {
        None
    }
}
