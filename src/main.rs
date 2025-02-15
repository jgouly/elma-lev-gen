use elma::lev::Top10Save;
use elma_lev_gen::flat;

fn main() {
    let mut rng = rand::thread_rng();
    let mut level = flat::gen(&mut rng);
    level.save("gen.lev", Top10Save::No).unwrap();
}
