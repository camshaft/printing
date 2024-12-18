use rayon::prelude::*;
use rsolid::*;

const TILE_W: f64 = 37.5;
const TILE_D: f64 = 6.75;

const MAGNET_W: f64 = 4.0;
const MAGNET_H: f64 = 4.0;
const MAGNET_L: f64 = 10.0;
const TOLERANCE: f64 = 0.75;

const MAGNET_INSET: f64 = 1.5;
const MAGNET_PLACEMENT: f64 = (TILE_W - MAGNET_H - TOLERANCE) * 0.5 - MAGNET_INSET;

const C_MAGNET_W: f64 = 6.0;
const C_MAGNET_H: f64 = 2.0;

fn magnet_cavity() -> Object {
    cube([
        MAGNET_L + TOLERANCE,
        MAGNET_H + TOLERANCE,
        MAGNET_W + TOLERANCE,
    ])
    .center(true)
    .into()
}

fn c_magnet_cavity() -> Object {
    cylinder(C_MAGNET_H + TOLERANCE, C_MAGNET_W * 0.5 + 0.85)
        .center(true)
        .into()
}

fn pane() -> Object {
    let pane_w = 21.0;
    let pane_t = 0.5;
    cube([pane_w, pane_w, pane_t]).center(true).into()
}

fn tile_square() -> Object {
    let base = cube([TILE_W, TILE_W, TILE_D]).center(true).into_object();

    let mut cavities = magnet_cavity() >> back(MAGNET_PLACEMENT);
    cavities += magnet_cavity() >> fwd(MAGNET_PLACEMENT);
    cavities += magnet_cavity() >> rotate([0.0, 0.0, 90.0]) >> right(MAGNET_PLACEMENT);
    cavities += magnet_cavity() >> rotate([0.0, 0.0, 90.0]) >> left(MAGNET_PLACEMENT);

    base - cavities
}

fn tile_square_ornate() -> Object {
    let chunkiness = 2.5;
    let inset = 3.0;

    let accent = {
        let w = TILE_W * 0.5 - chunkiness - 5.0;
        let line = cube([w, 0.1, 0.1]);
        let line = line.minkowski(sphere(chunkiness));

        let mut out = line.clone();

        out += &line >> rotate([0, 0, -90]);
        out += &line >> rotate([0, 0, -45]);

        out
    };

    let corner = {
        let w = TILE_W - inset * 3.0 - chunkiness;
        let line = cube([w, 0.1, 0.1]);
        let line = line.minkowski(sphere(chunkiness));

        let shift = w * 0.5 + inset;

        let mut out = &line >> back(shift) >> left(shift - inset * 2.0);

        out += &line >> rotate([0, 0, 90]) >> right(shift) >> back(shift);

        out
    };

    let offset = TILE_W * 0.5 - inset - chunkiness;

    let candy = {
        let size = chunkiness * 1.6;
        let mount =
            sphere(size + 0.5) >> scale([1.0, 1.0, 0.7]) >> up(0.25) >> right(0.25) >> back(0.25);

        let candy = sphere(size * 0.7) >> scale([1.0, 1.0, 0.5]) >> up(2.7);

        let out = mount + candy;

        out >> right(size * 0.5 - 1.0) >> back(size * 0.5 - 1.0)
    };

    let mut decorations = accent >> left(offset) >> fwd(offset);
    decorations += corner >> left(inset) >> fwd(inset);
    decorations += candy >> left(offset) >> fwd(offset);

    decorations &= cube([TILE_W, TILE_W, TILE_D]).center(true) >> up(TILE_D * 0.5);

    tile_square() + (decorations >> up(TILE_D * 0.5))
}

fn tile_window() -> Object {
    let mut base = tile_square();

    let mut cutouts = pane() >> scale([1.1, 1.1, 1.1]);

    let cutout_w = 9.0;
    let single_cutout = cube([cutout_w, cutout_w, 20.0]).center(true).into_object();

    let cutout_t = 1.5;
    let shift = (cutout_w + cutout_t) * 0.5;

    macro_rules! cutout {
        ($target:ident, $shape:expr, $shift:expr) => {{
            let shift = $shift;
            $target += $shape >> right(shift) >> back(shift);
            $target += $shape >> left(shift) >> back(shift);
            $target += $shape >> right(shift) >> fwd(shift);
            $target += $shape >> left(shift) >> fwd(shift);
        }};
    }

    cutout!(cutouts, &single_cutout, shift);

    let trim = {
        let width = 0.75;
        let mut outline = square((cutout_w + cutout_t + width) * 2.0)
            .center(true)
            .into_object();

        let mut cutouts = square(0.0).into_object();
        let single_cutout = square(cutout_w + width * 1.999).center(true).into_object();
        cutout!(cutouts, &single_cutout, shift);

        outline -= cutouts;

        let trim = outline >> linear_extrude(0.1);
        trim.minkowski(sphere(0.75)) >> dbg()
    };

    base -= cutouts;

    base += trim >> up(TILE_D * 0.5);

    base
}

fn tile_door() -> Object {
    let inset = 9.0;
    let decoration_w = TILE_W - inset;

    let upper = {
        let mut upper = tile_square();
        let arch_r = TILE_W * 0.75;
        let arch_c = circle(arch_r);
        let arch_offset = TILE_W * 0.4;

        let mut sq = square(decoration_w).center(true).into_object();
        sq -= square(TILE_W - inset - 1.0).center(true);
        sq &= circle(arch_r) >> back(arch_offset);

        let arch = arch_c - circle(arch_r - 1.0);
        let arch = arch >> back(arch_offset);
        let arch = arch & square(decoration_w).center(true);

        let design = sq + arch;

        let design = design >> linear_extrude(0.1);
        let mut design = design.minkowski(sphere(0.75));

        let line = square([0.5, TILE_W - inset - 3.0])
            .center(true)
            .into_object()
            >> back(1);
        let line = (line >> linear_extrude(0.1)).minkowski(sphere(0.75));

        let line_shift = (TILE_W - inset - 1.0) * 0.25;
        design += &line;
        design += &line >> right(line_shift);
        design += &line >> left(line_shift);

        upper += design >> up(TILE_D * 0.5);

        upper
    };

    let lower = {
        let mut lower = tile_square();

        let mut design = square(decoration_w).center(true).into_object();

        design -= square(TILE_W - inset - 1.0).center(true);

        let design = design >> linear_extrude(0.1);
        let mut design = design.minkowski(sphere(0.75));

        let line = square([0.5, TILE_W - inset]).center(true).into_object();
        let line = (line >> linear_extrude(0.1)).minkowski(sphere(0.75));

        let line_shift = (TILE_W - inset - 1.0) * 0.25;
        design += &line;
        design += &line >> right(line_shift);
        design += &line >> left(line_shift);

        lower += design >> up(TILE_D * 0.5);

        lower
    };

    (upper >> fwd(TILE_W * 0.5)) + (lower >> back(TILE_W * 0.5))
}

fn tile_decoration_mask() -> Object {
    cube([TILE_W, TILE_W, TILE_D]).center(true).up(TILE_D * 0.5)
}

fn tile_roof() -> Object {
    let mut tile = tile_square();

    let shingles = {
        let radius = TILE_W * 0.5 / 3.0;
        let mut shingle = circle(radius) - circle(radius - 1.0);
        shingle -= square(radius * 2.0).center(true) >> fwd(radius);
        let shingle = shingle >> linear_extrude(0.1);
        let shingle = shingle.minkowski(sphere(0.75));

        let lower_radius = radius - (0.75 * 0.5);

        let mut row = shingle.clone();
        row += &shingle >> right(radius * 2.0);
        row += &shingle >> left(radius * 2.0);

        let mut double = row.clone();
        double += &row >> fwd(radius) >> right(radius);
        double += &row >> fwd(radius) >> left(radius);

        let mut rows = double.clone();
        rows += &double >> fwd(lower_radius * 2.0);
        rows += &double >> back(lower_radius * 2.0);

        rows & tile_decoration_mask()
    };

    tile += shingles >> up(TILE_D * 0.5);

    tile
}

fn tile_icicle() -> Object {
    let mut tile = tile_square();

    let icicles = {
        let chunkiness = 0.75;

        let icicles = svg!("icicle.svg")
            >> linear_extrude(0.00001)
            >> left(TILE_W * 0.5)
            >> back(TILE_W * 0.5 + chunkiness);
        let icicles = icicles.minkowski(sphere(chunkiness));

        icicles & tile_decoration_mask()
    };

    tile += icicles >> up(TILE_D * 0.5);

    tile
}

fn wreath() -> Object {
    let radius = TILE_W * 0.5 - 10.0;
    let thickness = 5.4;
    let ring = circle(thickness) >> right(radius);
    let ring = ring >> rotate_extrude();

    let mut wreath = ring & (cube(500).center(true) >> up(250));

    wreath -= cylinder(2.5, 6.0 * 0.5 + 1.0) >> fwd(radius) >> up(1.0) >> dbg();

    wreath
}

fn gumdrop() -> Object {
    let base =
        svg!("gumdrop.svg") >> mirror([1, 0, 0]) >> right(13.14) >> rotate_extrude() >> scale(0.4);

    let cavity = c_magnet_cavity() >> up(2.0);

    base - cavity
}

fn snowman_ball(size: f64) -> Object {
    let mut ball = sphere(size * 0.5).into_object();

    let height = size - 2.0;

    ball &= cube([size, size, size - 3.0]).center(true);

    ball -= c_magnet_cavity() >> up((height * 0.5) - 3.0);
    ball -= c_magnet_cavity() >> down((height * 0.5) - 3.0);

    ball
}

fn snowman_hat() -> Object {
    let brim = cylinder(1.0, 7.0).center(true);
    let b_h = 8.0;
    let bucket = cylinder(b_h, 5.0).center(true);
    let bucket = bucket >> up(b_h * 0.5);

    let mut hat = brim + bucket;

    hat -= c_magnet_cavity() >> up(2.0);

    hat
}

fn roof_dims(diagonal: f64, horizontal: f64) -> [f64; 3] {
    let w = TILE_W * horizontal;
    let h = {
        let b = w * 0.5;
        let c = TILE_W * diagonal;
        (c.powi(2) - b.powi(2)).sqrt()
    };
    let a = (h / (w * 0.5)).atan().to_degrees();
    [w, h, a]
}

fn tile_triangle(diagonal: f64, horizontal: f64) -> Object {
    let [w, h, angle] = roof_dims(diagonal, horizontal);
    let tile = polygon([[0.0, 0.0], [w * 0.5, h], [w, 0.0]]).into_object();

    let tile = tile >> linear_extrude(TILE_D) >> down(TILE_D * 0.5);

    let cavities = {
        let shift = TOLERANCE * 0.175;

        let center_cavity = |obj: Object| {
            obj >> right(MAGNET_PLACEMENT + MAGNET_W - shift) >> fwd(MAGNET_H - shift)
        };

        let bottom_cavity = center_cavity(magnet_cavity());

        let mut row = bottom_cavity.clone();
        row += &bottom_cavity >> right(TILE_W);
        row += &bottom_cavity >> right(TILE_W * 2.0);

        let angled_c = &row >> mirror([0, 1, 0]) >> rotate([0.0, 0.0, angle]);

        let mut cavities = if diagonal < 3.0 && horizontal > 1.0 {
            &bottom_cavity >> right(TILE_W)
        } else {
            row
        };
        cavities += &angled_c;
        cavities += &angled_c >> mirror([1, 0, 0]) >> right(w);

        // TODO decoration cavities

        cavities += center_cavity(&bottom_cavity >> rotate([0, 0, 90]))
            >> right(TILE_W + MAGNET_W)
            >> back(1.0);

        cavities
    };

    tile - cavities
}

fn roof_fascia(diagonal: f64, horizontal: f64) -> Object {
    let mut out = roof_ridge(diagonal, horizontal, false);

    out &= cube([TILE_W, TILE_W, TILE_W]) >> back(TILE_W) >> down(TILE_W * 0.5);

    out >> rotate_y(-90)
}

fn roof_ridge(diagonal: f64, horizontal: f64, flat: bool) -> Object {
    let [w, h, _angle] = roof_dims(diagonal, horizontal);
    let roof = polygon([[0.0, 0.0], [w * 0.5, h], [w, 0.0]])
        >> mirror([0, 1, 0])
        >> fwd(h)
        >> left(w * 0.5)
        >> scale([0.75, 1.0, 1.0]);

    let top = {
        let mut shape = circle(TILE_D + 0.5).into_object();

        if flat {
            shape &= square((TILE_D * 2.0) * 0.99).center(true);
        }

        (roof.clone() & shape) >> linear_extrude(TILE_W)
    };

    let bottom = {
        let shape = circle(3.0).into_object();

        (roof.clone() & shape)
            >> mirror([0, 1, 0])
            >> linear_extrude(TILE_W - TILE_D * 2.0)
            >> fwd(0.35)
            >> up(TILE_D)
    };

    let out = top + bottom;

    out >> rotate([90.0, 180.0, 0.0])
}

fn peppermint_base() -> Object {
    let radius = 14.0 * 0.5;
    let circle = circle(radius) >> linear_extrude(0.1);
    circle.minkowski(sphere(3.5))
}

fn peppermint_swirl_pattern() -> Object {
    let p = svg!("peppermint.svg") >> back(14.0) >> right(1);

    let mut out = p.clone();
    let total = 5u64;
    for i in 1..total {
        let degree = i as f64 / total as f64 * 360.0;
        out += &p >> rotate([0.0, 0.0, degree])
    }

    out >> linear_extrude(5.0)
}

fn peppermint() -> Object {
    let mut candy = peppermint_base() - peppermint_swirl_pattern();

    candy -= c_magnet_cavity() >> down(2.0);

    candy
}

fn peppermint_swirl() -> Object {
    (peppermint_base() & peppermint_swirl_pattern()) >> scale([0.99, 0.99, 1.0])
}

fn main() {
    let settings = fragment_count(50).preview(25);

    let targets = &["stl"];
    // let targets = &[];

    [
        (tile_square(), "tile_square"),
        (tile_square_ornate(), "tile_square_ornate"),
        (tile_triangle(1.0, 1.0), "tile_triangle"),
        (tile_window(), "tile_window"),
        (tile_door(), "tile_door"),
        (tile_roof(), "tile_roof"),
        (tile_icicle(), "tile_icicle"),
        (tile_triangle(3.0, 3.0), "tile_roof_triangle"),
        (tile_triangle(2.0, 3.0), "tile_roof_triangle_shallow"),
        (roof_fascia(2.0, 3.0), "roof_fascia"),
        (roof_ridge(2.0, 3.0, true), "roof_ridge"),
        (pane(), "pane"),
        (wreath(), "wreath"),
        (gumdrop(), "gumdrop"),
        (snowman_ball(15.0), "snowman_large"),
        (snowman_ball(13.5), "snowman_medium"),
        (snowman_ball(11.0), "snowman_small"),
        (snowman_hat(), "snowman_hat"),
        (peppermint(), "peppermint"),
        (peppermint_swirl(), "peppermint_swirl"),
    ]
    .par_iter()
    .for_each(|(object, name)| {
        let start = std::time::Instant::now();
        let v = settings.apply(object);
        rsolid::export!(v, name, targets);
        eprintln!("{name} done in {:?}", start.elapsed());
    });
}
