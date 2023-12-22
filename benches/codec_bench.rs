#![feature(test)]
extern crate test;
use test::Bencher;

use rust_mdd::codec::{CmdcCodec, Codec};

#[bench]
fn bench_decode(b: &mut Bencher) {
    let codec = CmdcCodec {};
    let data = b"<1,18,0,-6,5222,2>[1,20,<1,2,0,452,5222,2>[100],4]";
    b.iter(|| codec.decode(data));
}

fn main() {}
