use rsolid::*;

const SCALE: f64 = 1.5;

fn main() {
    let mut out = vec![];

    macro_rules! shape {
        ($name:literal, $w:expr, $h:expr) => {{
            let shape = square([$w / SCALE, $h / SCALE]) >> linear_extrude(2.0);
            out.push(($name, shape));
        }};
    }

    shape!("room", 226.0, 189.5);
    shape!("west-elm-credenza", 80.0, 19.0);
    shape!("article-credenza", 71.0, 17.5);
    shape!("short-back", 29.0, 6.0);
    shape!("long-back", 35.0, 6.0);
    shape!("base", 35.0, 29.0);
    shape!("chair", 32.0, 36.0);
    shape!("rug", 16.5 * 12.0, 13.75 * 12.0);

    let targets = &["amf"];
    // let targets = &[];

    let settings = fragment_count(50).preview(25);

    out.iter().for_each(|(name, shape)| {
        let v = settings.apply(shape);
        rsolid::export!(v, format!("{name}"), targets);
    });
}
