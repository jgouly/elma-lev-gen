use elma::constants::OBJECT_RADIUS;
use elma::lev::*;
use elma::Position;

#[derive(Clone, Debug)]
pub struct FlatTrackConfig {
    width: f64,
    height: f64,
    num_segments: f64,
}

impl Default for FlatTrackConfig {
    fn default() -> Self {
        Self {
            width: 50.0,
            height: 7.0,
            num_segments: 40.0,
        }
    }
}

fn gen_spike(
    x_offset: f64,
    width: f64,
) -> [Position<f64>; 3] {
    [
        Position::new(x_offset + (width / 2.0), 0.0),
        Position::new(x_offset, 0.5),
        Position::new(x_offset - (width / 2.0), 0.0),
    ]
}

fn gen_polygons(config: &FlatTrackConfig) -> Vec<Polygon> {
    let width = config.width;
    let height = config.height;
    let mut vertices = vec![];

    // Left wall
    vertices.push(Position::new(0., 0.));
    vertices.push(Position::new(0., height));
    // Right wall
    vertices.push(Position::new(width, height));
    vertices.push(Position::new(width, 0.));

    let start_x = 3.0;
    let end_x = width - 2.0;

    let n_segments = config.num_segments;
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
            .flat_map(|&c| gen_spike(c, segment_width)),
    );
    vertices.push(Position::new(4.0, 0.0));
    vertices.dedup_by(|a, b| (a.x - b.x).abs() < 0.005 && (a.y - b.y).abs() < 0.005);

    vec![Polygon {
        grass: false,
        vertices,
    }]
}

pub fn gen() -> Level {
    let config = FlatTrackConfig::default();
    let mut level = Level::new();
    level.polygons = gen_polygons(&config);
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
