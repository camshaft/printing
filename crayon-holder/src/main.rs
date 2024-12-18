use rsolid::*;

const BASE_H: f64 = 116.0;
const BASE_W: f64 = 93.25;
const BASE_D: f64 = 20.0 * 1.5;
const BASE_ROUNDING: f64 = 11.0;
const S_SLOT_W: f64 = 14.0;
const S_SLOT_H: f64 = 26.5;
const S_SLOT_SPACING: f64 = 4.5;
const R_SLOT_D: f64 = 13.15;
const R_SLOT_R: f64 = R_SLOT_D * 0.5;

fn base_shape() -> Object<2> {
    square([BASE_W - BASE_ROUNDING, BASE_H - BASE_ROUNDING])
        .center(true)
        .minkowski(circle(BASE_ROUNDING))
}

fn s_slot() -> Object<2> {
    let w = S_SLOT_W;
    let h = S_SLOT_H;
    let f = &mask::fillet(2.0);
    let mut mask = f >> fwd(h / 2.0) >> right(w / 2.0);
    mask += f >> rotate_z(90) >> fwd(h / 2.0) >> left(w / 2.0);
    mask += f >> rotate_z(180) >> back(h / 2.0) >> left(w / 2.0);
    mask += f >> rotate_z(-90) >> back(h / 2.0) >> right(w / 2.0);

    square([w, h]).center(true) - mask
}

fn s_slots() -> Object<2> {
    let spacing = S_SLOT_W * 1.5 + S_SLOT_SPACING;

    let shape = s_slot() + (s_slot() >> back(S_SLOT_H * 1.20 + S_SLOT_SPACING));

    let first = shape >> left(spacing * 1.5) >> back(S_SLOT_H * 0.25);

    let mut s = first.clone();

    for i in 1..4 {
        s += &first >> right(spacing * i as f64);
    }

    s
}

fn r_slot() -> Object<2> {
    circle(R_SLOT_R).into()
}

fn r_slots() -> Object<2> {
    let spacing = 8.5;
    let first = r_slot() >> fwd(BASE_H * 0.5 - spacing) >> left(BASE_W * 0.5 - spacing);

    let spacing = spacing + R_SLOT_D + R_SLOT_R * 0.5;
    let first = first.clone() + (first >> back(spacing));

    let mut shape = first.clone();

    for i in 1..4 {
        shape += &first >> right(spacing * i as f64);
    }

    shape
}

fn base() -> Object {
    let shape = base_shape() - s_slots() - r_slots();

    shape >> linear_extrude(BASE_D)
}

fn bottom() -> Object {
    let shape = base_shape();

    shape >> linear_extrude(4.0)
}

fn main() {
    let main = base() + bottom();

    let settings = fragment_count(150).preview(25);

    let main = settings.apply(&main);

    let targets = &["amf"];

    rsolid::export!(main, targets);
}
