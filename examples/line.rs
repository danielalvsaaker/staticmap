use staticmap::{
    tools::{Color, LineBuilder},
    StaticMapBuilder, StaticMapError,
};

fn main() -> Result<(), StaticMapError> {
    let mut map = StaticMapBuilder::default()
        .width(300)
        .height(400)
        .padding((10, 0))
        .build()
        .unwrap();

    let lat: &[f64] = &[52.5, 48.9];
    let lon: Vec<f64> = vec![13.4, 2.3];

    let red = Color::new(true, 255, 0, 0, 255);
    let white = Color::new(true, 255, 255, 255, 255);

    let line = LineBuilder::default()
        .lat_coordinates(lat)
        .lon_coordinates(lon.clone())
        .width(3.)
        .simplify(true)
        .color(red)
        .build()
        .unwrap();

    let underline = LineBuilder::default()
        .lat_coordinates(lat)
        .lon_coordinates(lon)
        .width(5.)
        .simplify(true)
        .color(white)
        .build()
        .unwrap();

    map.add_line(underline);
    map.add_line(line);

    map.save_png("line.png")?;

    Ok(())
}
