#![feature(test)]

extern crate test;

use test::Bencher;

use paste::paste;
use rand::Rng;
use rcnb_rs::{encode, decode};
use byte_unit::Byte;

macro_rules! gen_bench {
    ($($size:expr),+) => {
        $(
            paste!(
                #[bench]
                fn [<rcnb_encode_ $size>](b: &mut Bencher) {
                    let size = Byte::from_str($size).unwrap().get_bytes();
                    let data = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(size as usize).collect::<String>();
                    b.iter(|| {
                        let _ = encode(test::black_box(data.as_bytes()));
                    });
                }

                #[bench]
                fn [<rcnb_decode_ $size>](b: &mut Bencher) {
                    let size = Byte::from_str($size).unwrap().get_bytes();
                    let data = encode(rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(size as usize).collect::<String>().as_bytes());
                    b.iter(|| {
                        let _ = decode(test::black_box(&data));
                    });
                }
            );
        )+
    };
}

gen_bench!("1b", "1kib", "10kib", "100kib", "1mib");