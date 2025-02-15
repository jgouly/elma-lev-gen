use elma::lev::Top10Save;
use elma_lev_gen::flat;

fn main() {
    let mut level = flat::gen();
    level.save("gen.lev", Top10Save::No).unwrap();
}
