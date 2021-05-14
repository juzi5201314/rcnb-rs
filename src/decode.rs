use crate::{SB, SC, SNB, SCNB, crmap, ccmap, cnmap, cbmap};

macro_rules! map_or {
    ($opt:expr, $map:expr) => {
        $opt.map_or(Some(0usize), |c|$map(*c))
    };
}

#[inline(always)]
fn decode_byte(chars: &[char]) -> u8 {
    let mut nb = false;

    let idx = (|| Some([map_or!(chars.get(0), crmap)?, map_or!(chars.get(1), ccmap)?]))()
        .or_else(|| {
            nb = true;
            (|| Some([map_or!(chars.get(0), cnmap)?, map_or!(chars.get(1), cbmap)?]))()
        })
        .expect("not rc/nb");
    let result = if nb { idx[0] * SB as usize + idx[1] } else { idx[0] * SC as usize + idx[1] };
    if result > 0x7E {
        panic!("rc/nb overflow")
    }

    (if nb { result | 0x80 } else { result }) as u8
}

#[inline(always)]
fn decode_short(chars: &[char]) -> usize {
    let reverse = !crmap(chars[0]).is_some();

    macro_rules! what_is_idx {
        ($chars:expr, $order0:expr, $order1:expr, $order2:expr, $order3:expr, $msg:expr) => {
            [
                map_or!($chars.get($order0), crmap).expect($msg),
                map_or!($chars.get($order1), ccmap).expect($msg),
                map_or!($chars.get($order2), cnmap).expect($msg),
                map_or!($chars.get($order3), cbmap).expect($msg)
            ]
        };
    }

    let idx = if !reverse {
        what_is_idx!(chars, 0, 1, 2, 3, "not rcnb")
    } else {
        what_is_idx!(chars, 2, 3, 0, 1, "not rcnb")
    };

    let result = idx[0] * SCNB as usize + idx[1] * SNB as usize + idx[2] * SB as usize + idx[3];

    if result > 0x7FFF {
        panic!("rcnb overflow")
    }

    if reverse { result | 0x8000 } else { result }
}

pub fn decode(s: &str) -> String {
    let vc =  s.chars().collect::<Vec<char>>();
    let len = vc.len();
    if (len & 1) != 0 {
        panic!("invalid length")
    }
    let mut v = Vec::with_capacity(len);

    let mut i = 0;

    while i < (len >> 2) {
        let start = i * 4;
        let short = decode_short(&vc[start..start + 4]);
        v.push((short >> 8) as u8);
        v.push((short & 0xFF) as u8);

        i += 1;
    }
    if (len & 2) != 0 {
        v.push(decode_byte(&vc[len-2..]))
    }

    unsafe { String::from_utf8_unchecked(v) }
}
