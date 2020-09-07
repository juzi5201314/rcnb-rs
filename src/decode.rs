use crate::{SB, SC, SNB, SCNB, CRMAP, CCMAP, CNMAP, CBMAP};

macro_rules! index_of_and_char_at {
    ($rcnb:ident, $chars:expr, $index:expr) => {
        {
            let char = $chars.get($index);
            if char.is_none() {
                Some(&0usize)
            } else {
                $rcnb.get(char.unwrap())
            }
        }
    };
}

fn decode_byte(chars: &[char]) -> u8 {
    let mut nb = false;
    let idx = (|| Some([index_of_and_char_at!(CRMAP, chars, 0)?, index_of_and_char_at!(CCMAP, chars, 1)?]))()
        .or_else(|| {
            nb = true;
            (|| Some([index_of_and_char_at!(CNMAP, chars, 0)?, index_of_and_char_at!(CBMAP, chars, 1)?]))()
        })
        .expect("not rc/nb");
    let result = if nb { idx[0] * SB as usize + idx[1] } else { idx[0] * SC as usize + idx[1] };
    if result > 0x7E {
        panic!("rc/nb overflow")
    }

    (if nb { result | 0x80 } else { result }) as u8
}

fn decode_short(chars: &[char]) -> usize {
    let reverse = !CRMAP.contains_key(&chars[0]);
    let what_is_idx = |order: &[usize; 4]| Some([
        index_of_and_char_at!(CRMAP, chars, order[0])?,
        index_of_and_char_at!(CCMAP, chars, order[1])?,
        index_of_and_char_at!(CNMAP, chars, order[2])?,
        index_of_and_char_at!(CBMAP, chars, order[3])?
    ]);

    let idx = if !reverse {
        what_is_idx(&[0, 1, 2, 3])
    } else {
        what_is_idx(&[2, 3, 0, 1])
    }.expect("not rcnb");

    let result = idx[0] * SCNB as usize + idx[1] * SNB as usize + idx[2] * SB as usize + idx[3];

    if result > 0x7FFF {
        panic!("rcnb overflow")
    }

    if reverse { result | 0x8000 } else { result }
}

pub fn decode(s: &str) -> String {
    let len = s.chars().count();
    if (len & 1) != 0 {
        panic!("invalid length")
    }
    let mut v = Vec::with_capacity(len / 2);

    let mut i = 0;
    let vc = s.chars().collect::<Vec<char>>();
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