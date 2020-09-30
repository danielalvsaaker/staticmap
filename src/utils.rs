pub fn into_rgba(target: raqote::DrawTarget) -> Vec<u8> {
    let data = target.get_data();
    let mut output = Vec::with_capacity(data.len() * 4);

    for pixel in data {
        let a = (pixel >> 24) & 0xffu32;
        let mut r = (pixel >> 16) & 0xffu32;
        let mut g = (pixel >> 8) & 0xffu32;
        let mut b = (pixel >> 0) & 0xffu32;

        if a > 0u32 {
            r = r * 255u32 / a;
            g = g * 255u32 / a;
            b = b * 255u32 / a;
        }

        output.push(r as u8);
        output.push(g as u8);
        output.push(b as u8);
        output.push(a as u8);
    }
    output
}

// Rewrite of image::imageops::replace for allowing negative pixel values.
pub fn replace<I, J>(bottom: &mut I, top: &J, x: i32, y: i32)
where
    I: image::GenericImage,
    J: image::GenericImageView<Pixel = I::Pixel>,
{
    let bottom_dims = bottom.dimensions();
    let top_dims = top.dimensions();


    for (top_x, pos_x) in (0..top_dims.0).zip(x..(top_dims.0 as i32 + x)) {
        for (top_y, pos_y) in (0..top_dims.1).zip(y..(top_dims.1 as i32 + y)) {
            if pos_x >= 0 && pos_y >= 0 && pos_x < bottom_dims.0 as i32 && pos_y < bottom_dims.1 as i32 {
                let p = top.get_pixel(top_x, top_y);
                bottom.put_pixel(pos_x as u32, pos_y as u32, p);
            }
        }
    }
}
