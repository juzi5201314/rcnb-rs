#![allow(dead_code)]
#[path = "encode.rs"]
mod en;
#[path = "decode.rs"]
mod de;

pub use en::*;
pub use de::*;

use phf::phf_map;

pub(crate) static CRMAP: phf::Map<char, usize> = phf_map! {
    'r' => 0,
    'R' => 1,
    'Ŕ' => 2,
    'ŕ' => 3,
    'Ŗ' => 4,
    'ŗ' => 5,
    'Ř' => 6,
    'ř' => 7,
    'Ʀ' => 8,
    'Ȑ' => 9,
    'ȑ' => 10,
    'Ȓ' => 11,
    'ȓ' => 12,
    'Ɍ' => 13,
    'ɍ' => 14
};
pub(crate) static CCMAP: phf::Map<char, usize> = phf_map! {
    'c' => 0,
    'C' => 1,
    'Ć' => 2,
    'ć' => 3,
    'Ĉ' => 4,
    'ĉ' => 5,
    'Ċ' => 6,
    'ċ' => 7,
    'Č' => 8,
    'č' => 9,
    'Ƈ' => 10,
    'ƈ' => 11,
    'Ç' => 12,
    'Ȼ' => 13,
    'ȼ' => 14
};
pub(crate) static CNMAP: phf::Map<char, usize> = phf_map! {
    'n' => 0,
    'N' => 1,
    'Ń' => 2,
    'ń' => 3,
    'Ņ' => 4,
    'ņ' => 5,
    'Ň' => 6,
    'ň' => 7,
    'Ɲ' => 8,
    'ƞ' => 9,
    'Ñ' => 10,
    'Ǹ' => 11,
    'ǹ' => 12,
    'Ƞ' => 13,
    'ȵ' => 14
};
pub(crate) static CBMAP: phf::Map<char, usize> = phf_map! {
    'b' => 0,
    'B' => 1,
    'ƀ' => 2,
    'Ɓ' => 3,
    'ƃ' => 4,
    'Ƅ' => 5,
    'ƅ' => 6,
    'ß' => 7,
    'Þ' => 8,
    'þ' => 9
};

pub(crate) const CR: &[char; 15] = &[
    'r', 'R', 'Ŕ', 'ŕ', 'Ŗ', 'ŗ', 'Ř', 'ř', 'Ʀ', 'Ȑ', 'ȑ', 'Ȓ', 'ȓ', 'Ɍ', 'ɍ',
];
pub(crate) const CC: &[char; 15] = &[
    'c', 'C', 'Ć', 'ć', 'Ĉ', 'ĉ', 'Ċ', 'ċ', 'Č', 'č', 'Ƈ', 'ƈ', 'Ç', 'Ȼ', 'ȼ',
];
pub(crate) const CN: &[char; 15] = &[
    'n', 'N', 'Ń', 'ń', 'Ņ', 'ņ', 'Ň', 'ň', 'Ɲ', 'ƞ', 'Ñ', 'Ǹ', 'ǹ', 'Ƞ', 'ȵ',
];
pub(crate) const CB: &[char; 10] = &['b', 'B', 'ƀ', 'Ɓ', 'ƃ', 'Ƅ', 'ƅ', 'ß', 'Þ', 'þ'];

pub(crate) const SR: u16 = CR.len() as u16;
pub(crate) const SC: u16 = CC.len() as u16;
pub(crate) const SN: u16 = CN.len() as u16;
pub(crate) const SB: u16 = CB.len() as u16;

pub(crate) const SRC: u16 = SR * SC;
pub(crate) const SNB: u16 = SN * SB;
pub(crate) const SCNB: u16 = SC * SNB;