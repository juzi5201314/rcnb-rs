use std::io::Read;

use argh::FromArgs;

use rcnb_rs::{decode, encode};

#[derive(FromArgs)]
/// RCNB!
struct Args {
    #[argh(switch, short = 'd')]
    /// decode
    decode: bool,

    #[argh(switch, short = 'e')]
    /// encode
    encode: bool,

    #[argh(positional)]
    /// contents
    content: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

    if let Some(content) = &args.content {
        if args.decode {
            println!("{}", decode(content));
        } else if args.encode && args.decode {
            println!("RCNB! (错乱");
        } else {
            println!("{}", encode(content));
        }
    } else {
        let mut input = Vec::new();
        std::io::stdin().read_to_end(&mut input).unwrap();
        println!("{}", encode(&input));
    }

}
