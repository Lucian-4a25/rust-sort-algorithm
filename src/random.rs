use rand::distributions::{Distribution, Uniform};

#[allow(dead_code)]
pub fn generate_random_data(n: usize, max: u32) -> Box<[u32]> {
    let mut rng = rand::thread_rng();
    let mut container = (0..(n as u32)).collect::<Vec<u32>>();
    let random_range = Uniform::from(0..max);

    for x in container.iter_mut() {
        *x = random_range.sample(&mut rng);
    }

    // println!("Generate: {:?}", container);
    // Box::into_array();
    container.into_boxed_slice()
}
