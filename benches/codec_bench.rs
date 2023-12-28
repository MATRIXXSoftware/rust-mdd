#![feature(test)]
extern crate test;
use test::black_box;
use test::Bencher;

use rust_mdd::cmdc::CMDC_CODEC;
use rust_mdd::codec::Codec;

#[bench]
fn bench_decode(b: &mut Bencher) {
    let data = b"<1,18,0,-6,5222,2>[1,20,<1,2,0,452,5222,2>[100],4]";
    b.iter(|| black_box(CMDC_CODEC.decode(data)));
}

#[bench]
fn bench_encode(b: &mut Bencher) {
    let containers = CMDC_CODEC
        .decode(b"<1,18,0,-6,5222,2>[1,20,<1,2,0,452,5222,2>[100],4]")
        .unwrap();
    b.iter(|| black_box(CMDC_CODEC.encode(&containers)));
}

fn main() {}
