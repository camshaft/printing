use rsolid::*;

const BAND_W: f64 = 10.25;
const BAND_T: f64 = 1.04;
const CLIP_T: f64 = 1.5;
const CLIP_W: f64 = BAND_W + CLIP_T * 2.0;
const CLIP_SPACING: f64 = 1.5;

fn clip() -> Object {
    let clip = cube([CLIP_W, CLIP_W, CLIP_T]).center(true);

    let top = clip >> down(CLIP_T);
    let bottom = clip >> up(CLIP_T);
    let right = cube([CLIP_W, CLIP_T, CLIP_SPACING * 3.0]).center(true) >> fwd(BAND_W * 0.5 + 1.0);
    let left = &right >> mirror([0, 1, 0]);

    let shape = top + bottom + right + left;

    let mask = {
        let shift = 0.001;
        let depth = CLIP_SPACING * 1.5 + shift;

        let top = mask::face::cube(
            CLIP_W + shift,
            CLIP_W + CLIP_SPACING * 0.335 + shift,
            mask::fillet(0.3),
        ) >> up(depth);

        let bottom = &top >> mirror([0, 0, 1]);

        let right = mask::face::cube(CLIP_W + shift, depth * 2.0, mask::fillet(0.3))
            >> rotate_x(-90)
            >> fwd(CLIP_W * 0.5 + 0.25 + shift);

        let left = &right >> mirror([0, 1, 0]);

        top + bottom + right + left
    };

    shape - mask.dbg()
}

fn band() -> Object {
    cube([200.0, BAND_W, BAND_T]).center(true).into()
}

fn star() -> Object {
    import::svg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/star.svg"))
        >> left(105)
        >> back(105)
        >> scale([0.05, 0.05, 1.0])
        >> linear_extrude(2.0)
        >> up(1.0)
}

fn heart() -> Object {
    import::svg(concat!(env!("CARGO_MANIFEST_DIR"), "/src/heart.svg"))
        >> left(105)
        >> back(105)
        >> scale([0.05, 0.05, 1.0])
        >> linear_extrude(2.0)
        >> up(1.0)
}

fn main() {
    let main = band().bg() + clip();
    let star = main.clone() + star();
    let heart = main.clone() + heart();

    let settings = fragment_count(150).preview(25);

    let star = settings.apply(&star);
    let heart = settings.apply(&heart);

    let targets = &["amf"];

    rsolid::export!(star, "star", targets);
    rsolid::export!(heart, "heart", targets);
}
