struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

// Textures

@group(0) @binding(0) var t_fore: texture_2d<f32>;
@group(0) @binding(1) var t_back: texture_2d<f32>;
@group(0) @binding(2) var t_text: texture_2d<f32>;
@group(0) @binding(3) var t_font: texture_2d<f32>;

struct Uniforms {
    // The number of pixels in a single character cell (width) in the font texture.
    font_width: u32,
    // The number of pixels in a single character cell (height) in the font texture.
    font_height: u32,
    // The scale of the font.  How many pixels on the screen correspond to a single 
    // pixel in the character font.
    font_scale: u32,
}

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    //
    // Convert a vertex index into a vertex
    //
    //  1+-------+3
    //   |       |
    //   |       |
    //   |       |
    //  0+-------+2
    //
    // Index    Coords
    // 0        (0, 0)
    // 1        (0, 1)
    // 2        (1, 0)
    // 3        (1, 1)
    //
    let x = f32(in_vertex_index & 2u) - 1.0;
    let y = (f32(in_vertex_index & 1u) * 2.0) - 1.0;
    out.clip_position = vec4(x, y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(
    @builtin(position) pos: vec4<f32>,
) -> @location(0) vec4<f32> {
    // Calculate the pixel coords
    let p = vec2<f32>(pos.x - 0.5, pos.y - 0.5);
    let font_size_in_texture = vec2<i32>(uniforms.font_width, uniforms.font_height);
    let font_size_on_screen = font_size_in_texture * uniforms.font_scale;

    // Calculate the char coords and the local coords inside a character block.
    // `cp` is the coordinates of the current pixel in character cells.
    // `lp` is the coordinates of the current pixel inside a character cell.
    let cp = i32(p) / font_size_on_screen;
    let lp = i32(p) % font_size_on_screen / uniforms.font_scale;

    // Look up the textures
    let fore = textureLoad(t_fore, cp, 0);
    let back = textureLoad(t_back, cp, 0);
    let text = textureLoad(t_text, cp, 0);

    // Calculate the ASCII character code
    let c = i32(text.x * 255.0);

    // Calculate the character coords in the font texture.  We expect the font
    // texture to be 16*16 characters.
    let fp = vec2<i32>(c % 16, c / 16);

    // Calculate the pixel coords within the font texture
    let p = fp * font_size_in_texture + lp;

    // Fetch the pixel in the font texture
    let font_pixel = textureLoad(t_font, p, 0);

    if font_pixel.r < 0.5 {
        return back;
    } else {
        return fore;
    }
}