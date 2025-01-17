// use std::time::Duration;
// use std::thread::sleep;
// use rand::thread_rng;
// use rand_distr::{Normal, Distribution};
// use std::fs::File;
use std::io::{self, Read};
// use stdf_rs::{file_has_record_header, file_has_record_footer, record_footer_size, file_has_record, get_record_from_file};

fn main() -> io::Result<()> {
//     let mean = 3.0;
//     let normal = Normal::new(mean, 1.0).unwrap();
//     let mut rng = thread_rng();

//     let file = File::open("example.stdf")?;

//     for _ in 0..10 {
//         let mut random_value: f64 = normal.sample(&mut rng);
//         if random_value < 0.0 { random_value = 0.0 };
//         println!("Random value from normal distribution: {}", random_value);
//         sleep(Duration::from_secs_f64(random_value));
        
//         let record = get_record_from_file(file.try_clone()?, 10)?;
//         println!("Read bytes: {:?}", record);
//     }

    Ok(())
}