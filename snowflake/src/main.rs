use rand::prelude::*;
use rand::Rng as _;
use rand_xoshiro::Xoroshiro128PlusPlus as Rng;
use rayon::prelude::*;
use rsolid::*;

const BRANCH_LEN: f64 = 25.0;
const HEIGHT: f64 = MAGNET_HEIGHT + 2.0;
const MAX_DEPTH: usize = 2;
const MAX_CHILDREN: usize = 6;
const MAGNET_HEIGHT: f64 = 2.0;
const MAGNET_WIDTH: f64 = 7.0;

#[derive(Clone, Copy, Debug)]
struct Config {
    initial_branch_len: f64,
    max_depth: usize,
    max_children: usize,
    max_width: f64,
    min_width: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            initial_branch_len: BRANCH_LEN,
            max_depth: MAX_DEPTH,
            max_children: MAX_CHILDREN,
            max_width: 4.0,
            min_width: 2.0,
        }
    }
}

impl Config {
    fn snowflake(&self, seed: u64) -> Object {
        let mut rng = Rng::from_rng(rand::rngs::StdRng::seed_from_u64(seed)).unwrap();
        let _count = rng.gen_range(5..=8);
        let count = 6;
        // let count = 1;
        let mut branch = self.branch(&mut rng, self.initial_branch_len, 0);
        for idx in 0..count {
            let fraction = idx as f64 / count as f64;
            let amount = 360.0 * fraction;
            branch += branch.clone().rotate([0.0, 0.0, amount]);
        }

        let mut branch = branch >> linear_extrude(HEIGHT) >> down(HEIGHT * 0.5);

        branch += magnet().scale([1.4, 1.4, 2.0]).down(MAGNET_HEIGHT);

        branch -= magnet();

        branch
    }

    fn branch(&self, rng: &mut Rng, len: f64, depth: usize) -> Object<2> {
        if depth == self.max_depth || len < 0.5 {
            return square(0).into();
        }

        let scale = self.depth_scale(depth);
        let width = rng.gen_range(self.min_width..=self.max_width) * scale;
        let mut b = branch_shape(len, width);

        for _ in 0..rng.gen_range(0..=self.max_children) {
            let offset = rng.gen_range(0.0..(len * 0.8));
            let len = rng.gen_range(0.0..(len * 0.9));
            let rot = rng.gen_range(30.0..170.0);
            let child = self.branch(rng, len, depth + 1);
            let child = child.rotate([0.0, 0.0, 180.0 - rot]).right(offset);

            b += &child;
            b += &child >> mirror([0.0, 1.0, 0.0]);
        }

        b.into()
    }

    fn depth_scale(&self, depth: usize) -> f64 {
        (self.max_depth - depth) as f64 / self.max_depth as f64
    }
}

fn magnet() -> Object {
    cylinder(MAGNET_HEIGHT, MAGNET_WIDTH * 0.5)
        .center(true)
        .up(MAGNET_HEIGHT * 0.5)
}

fn branch_shape(len: f64, width: f64) -> Object<2> {
    let mut b = square([len, width]).center(true).right(len * 0.5);

    let full = circle(width * 0.5).right(len);

    b += full;

    b
}

fn main() {
    let mut snowflakes = vec![];

    let large_seeds = [46, 43, 42, 36, 33, 31, 26, 25, 24, 23, 11, 2];

    for seed in large_seeds {
        snowflakes.push((
            "large",
            seed,
            Config {
                initial_branch_len: 60.0,
                max_depth: 4,
                max_children: 4,
                max_width: 8.0,
                min_width: 1.0,
            }
            .snowflake(seed),
        ));
    }

    let small_seeds = [
        1826, 1825, 1824, 1823, 1822, 1820, 1819, 1318, 1317, 1315, 1314, 1313, 1311, 1310, 1309,
        1307, 1306, 1304, 1303, 1302, 1301, 110, 109, 108, 107, 106, 105, 104, 100, 28, 26, 25, 24,
        21, 19, 18, 17, 14, 13, 12, 11, 10, 8, 7, 6, 4, 3, 2, 0,
    ];

    for seed in small_seeds {
        let config = Config {
            initial_branch_len: BRANCH_LEN * 0.5,
            min_width: 1.0,
            max_width: 3.0,
            ..Default::default()
        };
        snowflakes.push(("small", seed, config.snowflake(seed)));
    }

    let seeds = [
        1031, 1030, 1029, 1028, 1026, 1024, 1023, 1021, 1020, 1019, 1018, 1016, 1014, 1007, 1006,
        1005, 1004, 1002, 1001, 80, 79, 76, 75, 74, 73, 72, 71, 70, 69, 66, 65, 63, 61, 59, 57, 69,
        919, 23, 15, 30,
    ];

    for seed in seeds {
        snowflakes.push(("regular", seed, Config::default().snowflake(seed)));
    }

    let targets = &["amf"];
    // let targets = &[];

    let settings = fragment_count(50).preview(25);

    snowflakes.par_iter().for_each(|(name, idx, snowflake)| {
        let v = settings.apply(snowflake);
        rsolid::export!(v, format!("snowflake_{name}_{idx}"), targets);
    });
}
