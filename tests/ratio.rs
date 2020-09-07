use paste::paste;
use rcnb_rs::encode_str;
use rand::Rng;
use byte_unit::Byte;

macro_rules! gen_ratio {
    ($($size:expr),+) => {
        $(
            paste!(
                #[test]
                fn [<test $size _ratio>]() {
                    let size = Byte::from_str($size).unwrap().get_bytes();
                    let data = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(size as usize).collect::<String>();
                    let res = encode_str(&data);
                    rcnb_rs::decode(&res);
                    let ratio = res.len() as f64 / data.len() as f64;
                    println!("{} data, ratio: {}", $size, ratio)
                }
            );
        )+
    };
}

gen_ratio!("1b", "1kib", "10kib", "100kib", "1mib", "10mib", "100mib");