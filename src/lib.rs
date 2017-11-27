#![feature(test)]
#![feature(box_syntax)]
extern crate rand;
extern crate test;

use rand::Rng;
use rand::SeedableRng;

const SIZE: usize = 1 << 20;

fn generate_random_stuff() -> Box<[usize; SIZE]> {
    let mut array = box ([0; SIZE]);
    let mut rng = rand::StdRng::from_seed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    for i in 0..SIZE {
        array[i] = rng.gen_range(0, SIZE);
    }
    return array;
}

#[bench]
fn pointer_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<Box<u64>>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(Box::new(rng.next_u32() as u64));
    }

    bencher.iter(|| {
        for i in 0..SIZE / 2 {
            let alpha = test::black_box(*vector[i] + *vector[SIZE - i - 1]);
        }
    });
}

#[bench]
fn direct_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<u64>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(rng.next_u32() as u64);
    }

    bencher.iter(|| {
        for i in 0..SIZE / 2 {
            let alpha = test::black_box(vector[i] + vector[SIZE - i - 1]);
        }
    });
}

#[bench]
fn pointer_access_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<Box<u64>>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(Box::new(rng.next_u32() as u64));
    }

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[i]);
        }
    });
}

#[bench]
fn direct_access_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<u64>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(rng.next_u32() as u64);
    }

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(vector[i]);
        }
    });
}

#[bench]
fn pointer_rand_access_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<Box<u64>>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(Box::new(rng.next_u32() as u64));
    }

    let index_array = generate_random_stuff();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[index_array[i]]);
        }
    });
}

#[bench]
fn direct_rand_access_bench(bencher: &mut test::Bencher) {
    let mut vector = Vec::<u64>::with_capacity(SIZE);
    let mut rng = rand::StdRng::new().unwrap();

    for _ in 0..SIZE {
        vector.push(rng.next_u32() as u64);
    }

    let index_array = generate_random_stuff();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(vector[index_array[i]]);
        }
    });
}
