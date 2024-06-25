use rsolid::*;

const BASE_D: f64 = 62.0; // 72.0;
const BASE_R: f64 = BASE_D / 2.0;
const UPPER_D: f64 = 92.0; // 95.0;
const UPPER_R: f64 = UPPER_D / 2.0;
const BASE_H: f64 = 2.5;
const DISH_H: f64 = 2.5;
const SCREW_D: f64 = 5.0;
const BRACKET_H: f64 = 37.0 - BASE_H;

fn hole() -> Object<2> {
    let s = square([36.0, 14.85]).center(true);
    let c = circle(35.6 / 2.0);
    let s2 = square([17.15, 37.75]).center(true);
    let s3 = square([8.25, 47.5]).center(true);
    let notch = square([4.75, 2.3]).center(true);
    let notch_spacing = 52.0 / 2.0;
    let s4 = notch >> fwd(notch_spacing);
    let s5 = notch >> back(notch_spacing);

    s + c + s2 + s3 + s4 + s5
}

fn bottom() -> Object {
    let surface = circle(BASE_R) - hole();
    surface >> linear_extrude(BASE_H) >> down(BRACKET_H / 2.0 + BASE_H)
}

fn outer_dish() -> Object {
    cone(BRACKET_H, BASE_R, UPPER_R).center(true).into()
}

fn dish() -> Object {
    let c = outer_dish();
    let b = cone(BRACKET_H + 1.0, BASE_R - DISH_H, UPPER_R - DISH_H).center(true);

    c - b
}

fn cutout() -> Object {
    let side = &cube([UPPER_D + 5.0, 100.0, 100.0]).center(true);
    let shift = BASE_D - 2.0;
    let a = side.fwd(shift);
    let b = side.back(shift);

    a + b
}

fn screw_holes() -> Object {
    let hole = &cylinder(100, SCREW_D / 2.0).center(true).up(BRACKET_H);
    let distance = 4.0;
    let a = hole.left(UPPER_R - distance);
    let b = hole.right(UPPER_R - distance);

    a + b
}

fn screw_thing() -> Object {
    let hole = &cylinder(12, SCREW_D / 2.0 + 0.95).center(true);
    let distance = 5.0;
    let a = hole.left(UPPER_R - distance);
    let b = hole.right(UPPER_R - distance);

    a + b
}

fn screw_support() -> Object {
    let height = 50.0;
    let support = &cube([13.0, 41.0, height]).center(true).up(BRACKET_H - 10.0);

    let a = support.left(UPPER_R - 4.0);
    let b = support.right(UPPER_R - 4.0);

    (a + b) & outer_dish()
}

fn main() {
    let h = dish() + screw_support() - cutout() + bottom() - screw_holes() - screw_thing();
    let out = h >> fragment_count(200).preview(50);

    std::fs::create_dir_all("target/rsolid").unwrap();
    std::fs::write("target/rsolid/can-attachment.scad", out.to_string()).unwrap();
}
