use crate::{CN, SB, SC, CR, CC, CB, SNB, SCNB};

macro_rules! push_char {
    ($vec:expr, $($item:expr),*) => {
        $(
            $vec.push($item);
        )*
    };
}

#[inline(always)]
fn encode_byte(v: &mut String, mut i: u16) {
    if i > 0xFF {
        panic!("rc/nb overflow")
    }

    if i > 0x7F {
        i = i & 0x7F;
        push_char!(v, CN[(i / SB) as usize], CB[(i % SB) as usize]);
    } else {
        push_char!(v, CR[(i / SC) as usize], CC[(i % SC) as usize]);
    }
}

#[inline(always)]
fn encode_short(v: &mut String, mut i: u16) {
    let mut reverse = false;
    if i > 0x7FFF {
        reverse = true;
        i = i & 0x7FFF
    }
    if reverse {
        push_char!(v,
            CN[(i % SNB / SB) as usize],
            CB[(i % SB) as usize],
            CR[(i / SCNB) as usize],
            CC[(i % SCNB / SNB) as usize]
        );
    } else {
        push_char!(v,
            CR[(i / SCNB) as usize],
            CC[(i % SCNB / SNB) as usize],
            CN[(i % SNB / SB) as usize],
            CB[(i % SB) as usize]
        );
    }
}

#[inline(always)]
fn encode_short_o(mut i: u16) -> [char;4] {
    let mut reverse = false;
    if i > 0x7FFF {
        reverse = true;
        i = i & 0x7FFF
    }
    if reverse {
        [
            CN[(i % SNB / SB) as usize],
            CB[(i % SB) as usize],
            CR[(i / SCNB) as usize],
            CC[(i % SCNB / SNB) as usize]
        ]
    } else {
        [
            CR[(i / SCNB) as usize],
            CC[(i % SCNB / SNB) as usize],
            CN[(i % SNB / SB) as usize],
            CB[(i % SB) as usize]
        ]
    }
}

pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let data = data.as_ref();
    let len = data.len();
    let mut vc = String::with_capacity(len * 3);

    let mut i = 0;
    while i < (len >> 1) {
        encode_short(&mut vc, ((data[i * 2] as u16) << 8) | data[i * 2 + 1] as u16);
        i += 1;
    }

    if (data.len() & 1) != 0 {
        encode_byte(&mut vc, data[len - 1] as u16)
    }

    vc
    //vc.iter().collect::<String>()
}
