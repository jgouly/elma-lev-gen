use elma::constants::OBJECT_RADIUS;
use elma::lev::*;
use elma::Position;
use rand::Rng;

#[derive(Clone, Debug)]
struct SegmentConfig {
    num_segments: f64,
    spike_height_range: std::ops::Range<f64>,
}

impl Default for SegmentConfig {
    fn default() -> Self {
        Self {
            num_segments: 40.0,
            spike_height_range: -0.4..1.0,
        }
    }
}

#[derive(Clone, Debug)]
enum TypeConfig {
    SegmentConfig(SegmentConfig),
}

#[derive(Clone, Debug)]
pub struct FlatTrackConfig {
    width: f64,
    height: f64,
    type_config: TypeConfig,
}

impl Default for FlatTrackConfig {
    fn default() -> Self {
        Self {
            width: 50.0,
            height: 7.0,
            type_config: TypeConfig::SegmentConfig(SegmentConfig::default()),
        }
    }
}

fn gen_spike(
    config: &FlatTrackConfig,
    sc: &SegmentConfig,
    x_offset: f64,
    width: f64,
    rng: &mut impl Rng,
) -> [Position<f64>; 3] {
    [
        Position::new(x_offset + (width / 2.0), 0.0),
        Position::new(x_offset, rng.gen_range(sc.spike_height_range.clone())),
        Position::new(x_offset - (width / 2.0), 0.0),
    ]
}

fn gen_segments(config: &FlatTrackConfig, rng: &mut impl Rng) -> Vec<Polygon> {
    let width = config.width;
    let height = config.height;
    let TypeConfig::SegmentConfig(sc) = &config.type_config else {
        unreachable!()
    };
    let mut vertices = vec![];

    // Left wall
    vertices.push(Position::new(0., 0.));
    vertices.push(Position::new(0., height));
    // Right wall
    vertices.push(Position::new(width, height));
    vertices.push(Position::new(width, 0.));

    let start_x = 3.0;
    let end_x = width - 2.0;

    let n_segments = sc.num_segments;
    let full_segments_width = end_x - start_x;
    let segment_width = full_segments_width / n_segments;

    let mut centre_of_spikes = vec![];
    let mut centre = end_x - (segment_width / 2.0);
    while centre > start_x {
        centre_of_spikes.push(centre);
        centre -= segment_width;
    }
    centre_of_spikes.pop();

    vertices.extend(
        centre_of_spikes
            .iter()
            .flat_map(|&c| gen_spike(config, sc, c, segment_width, rng)),
    );
    vertices.push(Position::new(4.0, 0.0));
    vertices.dedup_by(|a, b| (a.x - b.x).abs() < 0.005 && (a.y - b.y).abs() < 0.005);

    vec![Polygon {
        grass: false,
        vertices,
    }]
}

pub fn gen(rng: &mut impl Rng) -> Level {
    let config = FlatTrackConfig::default();
    let mut level = Level::new();
    level.polygons = match config.type_config {
        TypeConfig::SegmentConfig(_) => gen_segments(&config, rng),
    };
    level.objects = vec![
        Object {
            position: Position::new(1.0, 0.0 + OBJECT_RADIUS),
            object_type: ObjectType::Player,
        },
        Object {
            position: Position::new(config.width - 3.0, 1.0 + OBJECT_RADIUS),
            object_type: ObjectType::Exit,
        },
    ];
    level.check_topology().unwrap();
    level
}
