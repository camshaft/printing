use rsolid::*;

const INNER_R: f64 = 55.0;
const OUTER_R: f64 = 90.0;
const BOTTOM_CUTOUT_R: f64 = 7.68;

fn dial() -> Object<2> {
    let tick_weight = 2.0;
    let out = 71.5;

    let number = |name: &str, rot: i64| {
        import::svg(format!("{}/src/{name}.svg", env!("CARGO_MANIFEST_DIR")))
            >> translate([-250 / 2, -180, 0])
            >> scale([0.11, 0.11, 1.0])
            >> rotate([0, 180, rot])
    };

    let twelve = number("xii", 0) >> fwd(out) >> right(3);
    let three = number("iii", 90) >> left(out);
    let six = number("vi", 180) >> back(out + 2.8) >> left(2.4);
    let nine = number("ix", -90) >> right(out);

    let mut shape = twelve + three + six + nine;

    for time in [1, 2, 4, 5, 7, 8, 10, 11] {
        let degrees = (time * (360 / 12)) as f64;
        let x = degrees.to_radians().sin() * out;
        let y = degrees.to_radians().cos() * out;
        let tick = circle(tick_weight * 0.8) >> translate([x, y, 0.0]);
        shape += tick;
    }

    shape
}

fn bottom_cutout(scale: f64) -> Object<2> {
    circle(BOTTOM_CUTOUT_R * scale) >> back(52.82)
}

fn face_cutout() -> Object<2> {
    let inner = circle(INNER_R);
    let cutout = bottom_cutout(1.0);

    inner + cutout
}

fn face() -> Object {
    let outer = circle(OUTER_R);

    let shape = outer - face_cutout();
    let dial = shape.clone() - dial();

    let plate = shape >> linear_extrude(1);
    let dial = dial >> linear_extrude(1) >> down(1);

    plate + dial
}

fn outer_rim() -> Object {
    let r = OUTER_R;
    let h = 5.0;
    let t = 1.0;
    let lip_h = 1.0;
    let outer = cylinder(h, 90).center(true);
    let lip = cone(lip_h, r, r + 1.0).center(true) >> up(h * 0.5 - lip_h * 0.5);
    let inner = cylinder(h + 2.0, r - t).center(true);

    let rim = outer + lip - inner;

    rim >> up(h * 0.5)
}

fn inner_rim() -> Object {
    let r = INNER_R;
    let h = 5.0;
    let t = 2.0;
    let rim = circle(r + t) - face_cutout();
    let rim = rim >> linear_extrude(h);

    let bottom_rim = bottom_cutout(1.15) >> linear_extrude(h);

    let mask = face_cutout() >> linear_extrude(100).center(true);

    let lip_h = 1.0;

    let mut lip = cylinder(lip_h, r).center(true).into_object();
    lip -= cone(lip_h + 0.001, r, r - 1.0).center(true);
    lip = lip >> up(h - lip_h * 0.5);
    lip -= bottom_cutout(1.0) >> linear_extrude(100).center(true);

    rim + bottom_rim - mask + lip
}

fn center_rim() -> Object {
    let r = 73.5;
    let t = 2.0;
    let h = 5.0;
    let outer = circle(r);
    let inner = circle(r - t);
    let shape = outer - inner;

    shape >> linear_extrude(h)
}

fn posts() -> Object {
    let post = {
        let a = cylinder(13, 6);

        let h = 18.5;
        let b = cylinder(h, 4);
        let chamfer = mask::face::cylinder(4, mask::chamfer(1.0, 1.0)) >> up(h);

        a + b - chamfer
    };

    let x = 60;
    let y = 40.7;
    let a = &post >> right(x) >> back(y);
    let b = &post >> left(x) >> back(y);
    let c = &post >> right(x) >> fwd(y);
    let d = &post >> left(x) >> fwd(y);

    a + b + c + d
}

fn main() {
    let h = face() + outer_rim() + inner_rim() + center_rim() + posts();
    let out = h >> fragment_count(200).preview(50);

    rsolid::export!(out, &["amf"]);
}
