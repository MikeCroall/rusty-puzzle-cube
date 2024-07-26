use std::time::{SystemTime, UNIX_EPOCH};

use three_d::{
    Camera, ColorMaterial, ColorTexture, Context, CpuMesh, CpuTexture, DepthTexture2D, FxaaEffect,
    Gm, InstancedMesh, Interpolation, Mat4, Mesh, RenderTarget, Srgba, Texture2D, TextureData,
    Vec3, Viewport, Wrapping,
};
use three_d_asset::{io::Serialize as _, Error};
use tracing::info;

use super::defaults::clear_state;

pub(super) fn save_as_image(
    ctx: &Context,
    viewport: Viewport,
    camera: &Camera,
    tiles: &Gm<InstancedMesh, ColorMaterial>,
    inner_cube: &Gm<Mesh, ColorMaterial>,
    save_non_fxaa_too: bool,
) -> Result<(), Error> {
    // todo currently cube is off-centered as if egui side panel was still present - use the pre-adjusted viewport?

    let file_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let (texture2d, texture_data) = get_original_render(ctx, viewport, camera, tiles, inner_cube);
    if save_non_fxaa_too {
        let file = format!("img/out/rusty-puzzle-cube-{file_id}.png");
        three_d_asset::io::save(
            &CpuTexture {
                data: texture_data,
                width: texture2d.width(),
                height: texture2d.height(),
                ..Default::default()
            }
            .serialize(&file)?,
        )?;
        info!("Saved {file}")
    }

    let (texture2d, texture_data) = fxaa(ctx, &texture2d);
    let file = format!("img/out/rusty-puzzle-cube-{file_id}-fxaa.png");
    three_d_asset::io::save(
        &CpuTexture {
            data: texture_data,
            width: texture2d.width(),
            height: texture2d.height(),
            ..Default::default()
        }
        .serialize(&file)?,
    )?;
    info!("Saved {file}");

    Ok(())
}

fn get_original_render(
    ctx: &Context,
    viewport: Viewport,
    camera: &Camera,
    tiles: &Gm<InstancedMesh, ColorMaterial>,
    inner_cube: &Gm<Mesh, ColorMaterial>,
) -> (Texture2D, TextureData) {
    let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
        ctx,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
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
        color_texture.as_color_target(None),
        depth_texture.as_depth_target(),
    )
    .clear(clear_state())
    .render(camera, tiles.into_iter().chain(inner_cube), &[])
    .read_color();

    (color_texture, TextureData::RgbaU8(pixels))
}

fn fxaa(ctx: &Context, original_cpu_tex: &Texture2D) -> (Texture2D, TextureData) {
    let viewport = Viewport::new_at_origo(original_cpu_tex.width(), original_cpu_tex.height());
    let camera = Camera::new_2d(viewport);
    let flat_geometry = {
        let mut gm = Gm::new(
            Mesh::new(ctx, &CpuMesh::square()),
            ColorMaterial {
                color: Srgba::BLACK,
                ..Default::default()
            },
        );
        gm.set_transformation(
            Mat4::from_translation(Vec3::new(
                original_cpu_tex.width() as f32 / 2.0,
                original_cpu_tex.height() as f32 / 2.0,
                0.0,
            )) * Mat4::from_nonuniform_scale(
                original_cpu_tex.width() as f32 / 2.0,
                original_cpu_tex.height() as f32 / 2.0,
                1.0,
            ),
        );
        gm
    };

    let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
        ctx,
        viewport.width,
        viewport.height,
        Interpolation::Nearest,
        Interpolation::Nearest,
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
        color_texture.as_color_target(None),
        depth_texture.as_depth_target(),
    )
    .render_with_effect(
        &FxaaEffect {},
        &camera,
        &flat_geometry,
        &[],
        Some(ColorTexture::Single(original_cpu_tex)),
        None,
    )
    .read_color();

    (color_texture, TextureData::RgbaU8(pixels))
}
