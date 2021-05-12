use staticmap::{
    tools::{Color, LineBuilder},
    Error, StaticMapBuilder,
};

fn main() -> Result<(), Error> {
    let mut map = StaticMapBuilder::new()
        .width(300)
        .height(400)
        .url_template("vg.no")
        .padding((10, 0))
        .build()
        .unwrap();

    let lat: &[f64] = &[52.5, 48.9];
    let lon: Vec<f64> = vec![13.4, 2.3];

    let red = Color::new(true, 255, 0, 0, 255);
    let white = Color::new(true, 255, 255, 255, 255);

    let line = LineBuilder::new()
        .lat_coordinates(lat.into_iter().copied())
        .lon_coordinates(lon.clone())
        .width(3.)
        .simplify(true)
        .color(red)
        .build()?;

    let underline = LineBuilder::new()
        .lat_coordinates(lat.into_iter().copied())
        .lon_coordinates(lon)
        .width(5.)
        .simplify(true)
        .color(white)
        .build()?;

    map.add_tool(underline);
    map.add_tool(line);

    map.save_png("line.png")?;

    Ok(())
}
