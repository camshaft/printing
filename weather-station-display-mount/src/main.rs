use rsolid::*;

const PS_W: f64 = 39.12;
const PS_H: f64 = 60.0;
const PS_D: f64 = 26.31;
const PS_CABLE_H: f64 = 15.0;
const PS_CABLE_D: f64 = 8.4;
const PS_CABLE_R: f64 = PS_CABLE_D * 0.5;
const PS_CABLE_OFFSET: f64 = 9.57;

fn power_supply() -> Object {
    let mut s = cube([PS_W, PS_H, PS_D]).center(true).into_object();

    s += cylinder(PS_CABLE_H, PS_CABLE_R).center(true)
        >> rotate_x(90)
        >> fwd((PS_H + PS_CABLE_H) * 0.5)
        >> up(PS_D * 0.5 - PS_CABLE_OFFSET);

    s += cube([30.6, 23.75, 5.5]).center(true)
        >> up((PS_D + 5.5) * 0.5)
        >> back((PS_H + 23.75) * 0.5 - 29.0);

    s
}

fn ps_mount() -> Object {
    let t = 2.0;
    let mut c =
        cube([PS_W + t * 2.0, (PS_H - 31.0) + t * 2.0, PS_D + t * 2.0]).center(true) >> fwd(16.0);

    c -= power_supply() >> scale(1.02);

    c -= {
        let r = PS_D + t * 2.0;

        cylinder(100, r).center(true) >> rotate_y(90) >> down(r * 0.5 + 5.0) >> fwd(0.475)
    };

    c -= cube([PS_CABLE_D - 1.0, 100.0, 22.0]).center(true) >> down(7.5);

    c
}

fn main() {
    let main = power_supply().bg() + ps_mount();

    let settings = fragment_count(150).preview(25);

    let main = settings.apply(&main);

    let targets = &[];

    rsolid::export!(main, targets);
}
