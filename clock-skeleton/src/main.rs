use rsolid::*;

fn back_plate() -> Object {
    import::stl("/home/cameron/Projects/3d-models/pendulum/Skeleton/Back_plate.stl")
        >> fwd(41)
        >> down(80)
}

fn middle_plate() -> Object {
    import::stl("/home/cameron/Projects/3d-models/pendulum/Skeleton/Middle_plate.stl")
        >> fwd(40.73)
        >> down(80)
}

fn top_plate() -> Object {
    import::stl("/home/cameron/Projects/3d-models/pendulum/Skeleton/Front_plate.stl")
        >> fwd(72.3)
        >> down(95.25)
}

fn mask() -> Object<2> {
    mask::fillet(7.5)
}

fn back_plate_masks() -> Object {
    let w = 62.51;
    let h = 100.0;
    let top_shift = 8.0;

    let top_mask = square([10, 10]).center(true) >> left(10.0);
    let top_shape = mask() - top_mask;

    let a = &top_shape >> rotate_y(180) >> left(w) >> fwd(h - top_shift);
    let b = &top_shape >> right(w) >> fwd(h - top_shift);
    let c = mask() >> rotate_z(180) >> left(w) >> back(h);
    let d = mask() >> rotate_z(-90) >> right(w) >> back(h);

    let shape = a + b + c + d;
    shape >> linear_extrude(100).center(true)
}

fn middle_plate_masks() -> Object {
    let w = 62.5;
    let h = 100.0;
    let top_shift = 8.0;

    let top_mask = square([10, 10]).center(true) >> left(10.0);
    let top_shape = mask() - top_mask;

    let a = &top_shape >> rotate_y(180) >> left(w) >> fwd(h - top_shift);
    let b = &top_shape >> right(w) >> fwd(h - top_shift);
    let c = mask() >> rotate_z(180) >> left(w) >> back(h);
    let d = mask() >> rotate_z(-90) >> right(w) >> back(h);

    let shape = a + b + c + d;
    shape >> linear_extrude(100).center(true)
}

fn top_plate_masks() -> Object {
    let w = 62.51;
    let h = 100.0;
    let top_shift = 8.0;

    let top_mask = square([10, 10]).center(true) >> left(10.0);
    let top_shape = mask() - top_mask;

    let a = &top_shape >> rotate_y(180) >> left(w) >> fwd(h - top_shift);
    let b = &top_shape >> right(w) >> fwd(h - top_shift);
    let c = mask() >> rotate_z(180) >> left(w) >> back(h);
    let d = mask() >> rotate_z(-90) >> right(w) >> back(h);

    let shape = a + b + c + d;
    shape >> linear_extrude(100).center(true)
}

fn top_plate_fill() -> Object {
    let a = square([100.0, 28.0]).center(true) >> back(82);
    let b = square([40.0, 20.0]).center(true) >> rotate_z(30.0) >> back(70) >> left(30.0);

    let c = triangle::right(40.0, 40.0) >> rotate_z(180) >> fwd(75) >> left(23.5);

    let d = square([48.0, 15.0]).center(true) >> rotate_z(-33) >> fwd(73) >> left(24);

    let e = triangle::right(12.0, 12.0) >> rotate_z(-90) >> fwd(85) >> left(43) >> dbg();

    let mut shape = b + c + d + e;
    shape += &shape >> mirror([1, 0, 0]);
    shape += a;

    shape >> linear_extrude(1.0)
}

fn main() {
    let h = back_plate() - back_plate_masks().dbg();
    //let h = middle_plate() - middle_plate_masks().dbg();
    //let h = top_plate() + top_plate_fill() - top_plate_masks();
    let out = h >> fragment_count(200).preview(50);

    rsolid::export!(out, &["amf"]);
}
