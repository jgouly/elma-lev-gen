use elma::lev::Top10Save;
use elma_lev_gen::flat;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

fn main() {
    let seed = if let Ok(seed_str) = std::env::var("SEED") {
        let s1 = u128::from_str_radix(&seed_str[0..32], 16).unwrap();
        let s2 = u128::from_str_radix(&seed_str[32..64], 16).unwrap();
        let mut seed: <SmallRng as SeedableRng>::Seed = Default::default();
        seed[0..16].copy_from_slice(&s1.to_le_bytes());
        seed[16..32].copy_from_slice(&s2.to_le_bytes());
        seed
    } else {
        let mut seed: <SmallRng as SeedableRng>::Seed = Default::default();
        rand::thread_rng().fill(&mut seed);
        seed
    };
    let seed1 = u128::from_le_bytes(seed[0..16].try_into().unwrap());
    let seed2 = u128::from_le_bytes(seed[16..32].try_into().unwrap());

    println!("seed: {:x} {:x}", seed1, seed2);
    let mut rng = SmallRng::from_seed(seed);
    let mut level = flat::gen(&mut rng);
    level.save("gen.lev", Top10Save::No).unwrap();
}
