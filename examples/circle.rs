use staticmap::{
    tools::{CircleBuilder, Color},
    StaticMapBuilder, StaticMapError,
};

fn main() -> Result<(), StaticMapError> {
    let mut map = StaticMapBuilder::default()
        .width(200)
        .height(200)
        .url_template("https://a.tile.osm.org/{z}/{x}/{y}.png")
        .zoom(5)
        .build()
        .unwrap();

    let circle_outline = CircleBuilder::default()
        .lon_coordinate(10.)
        .lat_coordinate(47.)
        .color(Color::new(true, 255, 255, 255, 255))
        .radius(9.)
        .build()
        .unwrap();

    let circle = CircleBuilder::default()
        .lon_coordinate(10.)
        .lat_coordinate(47.)
        .color(Color::new(true, 0, 0, 255, 255))
        .radius(6.)
        .build()
        .unwrap();

    map.add_marker(circle_outline);
    map.add_marker(circle);

    map.save_png("circle.png")?;

    Ok(())
}
