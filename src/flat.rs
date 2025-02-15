use elma::constants::OBJECT_RADIUS;
use elma::lev::*;
use elma::Position;

#[derive(Clone, Debug)]
pub struct FlatTrackConfig {
    width: f64,
    height: f64,
}

impl Default for FlatTrackConfig {
    fn default() -> Self {
        Self {
            width: 50.0,
            height: 7.0,
        }
    }
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
