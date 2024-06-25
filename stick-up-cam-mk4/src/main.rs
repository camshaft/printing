use rsolid::*;

const BASE_D: f64 = 52.85;
const BASE_R: f64 = BASE_D * 0.5;
const BASE_H: f64 = 1.25;
const BASE_W: f64 = 1.23;
const BASE_OR: f64 = BASE_R + BASE_W;
const TAB_W: f64 = 50.0;
const TAB_H: f64 = 8.0;
const MOUNT_CIRCLE_D: f64 = 37.0;
const MOUNT_CIRCLE_R: f64 = MOUNT_CIRCLE_D * 0.5;
const BOTTOM_SHIFT: f64 = 17.38;
const LIP_H: f64 = 4.79;
const MOUNT_D: f64 = BASE_H * 2.0 + 2.0;

const RACK_THICKNESS: f64 = 7.0;
const RACK_H: f64 = 40.0;

const CLIP_H: f64 = 6.35;
const CLIP_W: f64 = 11.87;
const CLIP_THICKNESS: f64 = 1.85;

fn base_inner() -> Object<2> {
    circle(BASE_R).into()
}

fn base_outer() -> Object<2> {
    circle(BASE_OR).into()
}

fn lip() -> Object<2> {
    (square([100.0, LIP_H]) >> fwd(BASE_OR - LIP_H) >> left(50)).intersection(base_outer())
}

fn existing_mount_shape() -> Object {
    let base_inner = &base_inner();
    let base_outer = &base_outer();

    let lip = lip();

    let side_a = square([100.0, 25.88]).center(true) >> right(50) >> right(BASE_OR) >> left(5.79);
    let side_b = side_a.clone() >> mirror([1, 0, 0]);
    let side = (side_a + side_b).intersection(base_outer);

    let tabs = square([TAB_W, TAB_H]) >> left(25) >> back(BASE_R - BOTTOM_SHIFT);

    let mount_circle = (circle(MOUNT_CIRCLE_R) >> back(10.0)).intersection(base_outer);

    let ring = base_outer.clone() - base_inner + lip + side - tabs + mount_circle;

    ring >> linear_extrude(BASE_H).center(true)
}

fn mount_shape() -> Object {
    let base_d = 52.90;
    let base_r = base_d * 0.5;
    let margin = 1.0;
    let mink_v = circle(0.3);

    let side = square([TAB_W - margin, TAB_H - margin]).center(true)
        >> back(base_r - BOTTOM_SHIFT - TAB_H * 0.5);

    let top_tab = circle(21.3) >> scale([1.0, 0.90, 1.0]) >> back(3.5);

    let bottom_mask = square([TAB_W, 50.0]).center(false)
        >> mirror([0, 1, 0])
        >> left(TAB_W * 0.5)
        >> back(base_r - BOTTOM_SHIFT - margin * 0.5);

    let circle_mask = circle(MOUNT_CIRCLE_R + margin) >> back(BASE_R - BOTTOM_SHIFT + margin * 0.5);

    let tab_shape = (side.clone() + &top_tab - &bottom_mask - &circle_mask).minkowski(mink_v);

    let first_tab = tab_shape >> linear_extrude(BASE_H).center(true) >> down(BASE_H);
    let lip_mask = lip() >> back(10.7);

    let tab_mount = (top_tab - &bottom_mask - &circle_mask - lip_mask).minkowski(mink_v)
        >> linear_extrude(MOUNT_D).center(true)
        >> up(1);

    first_tab + tab_mount
}

fn bracket() -> Object {
    cube([256.0, RACK_THICKNESS, RACK_H]).center(true).into()
}

fn clip() -> Object {
    let s = square([RACK_H + CLIP_THICKNESS, CLIP_THICKNESS]) >> left(RACK_H * 0.5);
    let top = square([CLIP_THICKNESS, CLIP_W]) >> right(RACK_H * 0.5);
    let bottom = &top >> mirror([1, 0, 0]);
    let clip = square([CLIP_H, CLIP_THICKNESS])
        >> right(RACK_H * 0.5 - CLIP_H + CLIP_THICKNESS)
        >> fwd(CLIP_W - CLIP_THICKNESS);
    let ball = circle(CLIP_THICKNESS * 0.5 * 1.2)
        >> fwd(CLIP_W - CLIP_THICKNESS * 0.6)
        >> left(RACK_H * 0.5 + 0.2);

    let mink_v = circle(0.5);

    let shape = (s + top + bottom + clip + ball).minkowski(mink_v);
    let shape = shape >> scale([1.03, 1.0, 1.0]);

    shape
        >> linear_extrude(BASE_OR * 2.0 + CLIP_THICKNESS).center(true)
        >> yrot(-90)
        >> mirror([0, 1, 0])
        >> fwd(CLIP_W * 0.5)
}

fn main() {
    let mount = existing_mount_shape().bg() + mount_shape();
    let clip = clip() + bracket().bg();
    let h = clip + (mount >> xrot(90) >> fwd(CLIP_W * 0.5 + MOUNT_D * 0.5 + 1.0));
    let out = h >> fragment_count(200).preview(50);

    std::fs::create_dir_all("target/rsolid").unwrap();
    std::fs::write("target/rsolid/stick-up-cam-mk4.scad", out.to_string()).unwrap();
}
