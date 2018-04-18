#![feature(test)]
#![feature(box_syntax)]
extern crate rand;
extern crate test;

use rand::Rng;
use rand::SeedableRng;
use std::iter::repeat;

const SIZE: usize = 1 << 20;

fn generate_random_stuff() -> Box<[usize; SIZE]> {
    let mut array = box ([0; SIZE]);
    let mut rng = rand::StdRng::from_seed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    for i in 0..SIZE {
        array[i] = rng.gen_range(0, SIZE);
    }
    return array;
}

fn fragment<T>(mut vector: Vec<T>) -> Vec<T> {
    let mut rng = rand::StdRng::new().unwrap();
    let mut permutated_vector = Vec::with_capacity(vector.len());
    while (!vector.is_empty()) {
        let vector_len = vector.len();
        let value = vector.swap_remove(rng.gen_range(0, vector_len));
        permutated_vector.push(value);
    }
    permutated_vector
}

#[bench]
fn pointer_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    bencher.iter(|| {
        for i in 0..SIZE / 2 {
            let alpha = test::black_box(*vector[i] + *vector[SIZE - i - 1]);
        }
    });
}

#[bench]
fn pointer_fragmented_heap_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    let vector = fragment(vector);

    bencher.iter(|| {
        for i in 0..SIZE / 2 {
            let alpha = test::black_box(*vector[i] + *vector[SIZE - i - 1]);
        }
    });
}

#[bench]
fn direct_bench(bencher: &mut test::Bencher) {
    let vector: Vec<u64> = rand::StdRng::new().unwrap().gen_iter().take(SIZE).collect();

    bencher.iter(|| {
        for i in 0..SIZE / 2 {
            let alpha = test::black_box(vector[i] + vector[SIZE - i - 1]);
        }
    });
}

#[bench]
fn pointer_access_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[i]);
        }
    });
}

#[bench]
fn pointer_access_fragmented_heap_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    let vector = fragment(vector);

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[i]);
        }
    });
}

#[bench]
fn direct_access_bench(bencher: &mut test::Bencher) {
    let vector: Vec<u64> = rand::StdRng::new().unwrap().gen_iter().take(SIZE).collect();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(vector[i]);
        }
    });
}

#[bench]
fn pointer_rand_access_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    let index_array = generate_random_stuff();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[index_array[i]]);
        }
    });
}

#[bench]
fn pointer_rand_access_fragmented_heap_bench(bencher: &mut test::Bencher) {
    let vector: Vec<Box<u64>> = rand::StdRng::new()
        .unwrap()
        .gen_iter()
        .map(Box::new)
        .take(SIZE)
        .collect();

    let vector = fragment(vector);
    let index_array = generate_random_stuff();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(*vector[index_array[i]]);
        }
    });
}

#[bench]
fn direct_rand_access_bench(bencher: &mut test::Bencher) {
    let vector: Vec<u64> = rand::StdRng::new().unwrap().gen_iter().take(SIZE).collect();

    let index_array = generate_random_stuff();

    bencher.iter(|| {
        for i in 0..SIZE {
            let alpha = test::black_box(vector[index_array[i]]);
        }
    });
}
