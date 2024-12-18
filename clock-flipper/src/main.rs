use rsolid::*;

const HEIGHT: f64 = 25.75;
const WIDTH: f64 = 72.1;
const INNER_W: f64 = 67.25;
const CUTOUT_W: f64 = (WIDTH - INNER_W) * 0.5;
const CUTOUT_H: f64 = 7.5;
const THICKNESS: f64 = 0.4;
const TAB_W: f64 = 1.25;

fn shape() -> Object<2> {
    let mut base = square([WIDTH, HEIGHT]).center(true).into_object();

    let mask = mask::fillet(2.7) >> right(WIDTH * 0.5) >> fwd(HEIGHT * 0.5);

    base -= &mask;
    base -= mask >> mirror([1, 0, 0]);

    let cutout = square([CUTOUT_W, CUTOUT_H]).center(true)
        >> right((WIDTH - CUTOUT_W) * 0.5)
        >> back((HEIGHT - CUTOUT_H) * 0.5 - TAB_W);

    base -= &cutout;
    base -= cutout >> mirror([1, 0, 0]);

    let mut tab = square(TAB_W).center(true).into_object();
    tab += circle(TAB_W * 0.5) >> right(TAB_W * 0.5);

    let tab = tab >> right(WIDTH * 0.5) >> back((HEIGHT - TAB_W) * 0.5);

    base += &tab;
    base += tab >> mirror([1, 0, 0]);

    base
}

fn out() -> Object {
    shape() >> linear_extrude(THICKNESS)
}

fn main() {
    let out = out();

    let targets = &["amf"];
    // let targets = &[];

    let settings = fragment_count(50).preview(25);

    let v = settings.apply(&out);
    rsolid::export!(v, targets);
}
