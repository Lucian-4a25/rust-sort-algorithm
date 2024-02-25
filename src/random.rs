use rand::Rng;

pub fn generate_random_data(n: usize, max: u32) -> Box<[u32]> {
    let mut rng = rand::thread_rng();
    let mut container = (0..(n as u32)).collect::<Vec<u32>>();

    for x in container.iter_mut() {
        *x = rng.gen_range(0..max);
    }

    // println!("Generate: {:?}", container);
    // Box::into_array();
    container.into_boxed_slice()
}
