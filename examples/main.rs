use staticmap_rs::{Line, StaticMap, Color};

fn main() {
    let mut map = StaticMap {
        width: 400,
        height: 400,
        padding: (0, 0), // (x, y)
        x_center: 0.,
        y_center: 0.,
        url_template: "https://a.tile.openstreetmap.org/%z/%x/%y.png".to_string(),
        tile_size: 256,
        lines: Vec::new(),
        zoom: 0,
    };

    // (Longitude, latitude)
    let coordinates = vec![(13.4, 52.5), (2.3, 48.9)];

    let line = Line {
        coordinates: coordinates.to_owned(),
        color: Color {
            r: 255u8,
            g: 0u8,
            b: 0u8,
            a: 255u8,
        },
        width: 6.,
        simplify: true,
    };

    let underline = Line {
        coordinates: coordinates.to_owned(),
        color: Color {
            r: 255u8,
            g: 255u8,
            b: 255u8,
            a: 255u8,
        },
        width: 15.,
        simplify: true,
    };

    map.add_line(underline);
    map.add_line(line);

    let image = map.render();
    image.save("render.png").unwrap();
}
