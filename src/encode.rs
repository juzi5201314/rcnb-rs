use crate::{CN, SB, SC, CR, CC, CB, SNB, SCNB};

fn encode_byte(i: u16) -> [char;2] {
    if i > 0xFF {
        panic!("rc/nb overflow")
    }

    if i > 0x7F {
        let i = i & 0x7F;
        [CN[(i / SB) as usize], CB[(i % SB) as usize]]
    } else {
        [CR[(i / SC) as usize], CC[(i % SC) as usize]]
    }
}

fn encode_short(i: u16) -> [char;4] {
    let mut reverse = false;
    let mut i = i;
    if i > 0x7FFF {
        reverse = true;
        i = i & 0x7FFF
    }
    let chars = [
        CR[(i / SCNB) as usize],
        CC[(i % SCNB / SNB) as usize],
        CN[(i % SNB / SB) as usize],
        CB[(i % SB) as usize],
    ];
    if reverse {
        [chars[2], chars[3], chars[0], chars[1]]
    } else {
        chars
    }
}

pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let data = data.as_ref();
    let len = data.len();
    let mut vc = Vec::with_capacity(len * 2);

    let mut i = 0;
    while i < (len >> 1) {
        vc.extend_from_slice(&encode_short(((data[i * 2] as u16) << 8) | data[i * 2 + 1] as u16));
        i += 1;
    }

    if (data.len() & 1) != 0 {
        vc.extend_from_slice(&encode_byte(data[len - 1] as u16))
    }

    vc.iter().collect::<String>()
}
