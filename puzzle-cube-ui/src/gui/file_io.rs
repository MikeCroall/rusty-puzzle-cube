use std::time::{SystemTime, UNIX_EPOCH};

use three_d::{
    Camera, ColorMaterial, Context, CpuTexture, DepthTexture2D, Gm, InstancedMesh, Interpolation,
    RenderTarget, Texture2D, TextureData, Viewport, Wrapping,
};
use three_d_asset::{Error, io::Serialize as _};

use super::defaults::clear_state;

pub(super) fn save_as_image(
    ctx: &Context,
    viewport: Viewport,
    camera: &Camera,
    tiles: &Gm<InstancedMesh, ColorMaterial>,
) -> Result<(), Error> {
    let mut texture = Texture2D::new_empty::<[u8; 4]>(
        ctx,
        viewport.width,
        viewport.height,
        Interpolation::Linear,
        Interpolation::Linear,
        None,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let mut depth_texture = DepthTexture2D::new::<f32>(
        ctx,
        viewport.width,
        viewport.height,
        Wrapping::ClampToEdge,
        Wrapping::ClampToEdge,
    );
    let pixels = RenderTarget::new(
        texture.as_color_target(None),
        depth_texture.as_depth_target(),
    )
    .clear(clear_state())
    .render(camera, tiles, &[])
    .read_color();

    three_d_asset::io::save(
        &CpuTexture {
            data: TextureData::RgbaU8(pixels),
            width: texture.width(),
            height: texture.height(),
            ..Default::default()
        }
        .serialize(format!(
            "img/rusty-puzzle-cube-{}.png",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis()
        ))?,
    )?;
    Ok(())
}
