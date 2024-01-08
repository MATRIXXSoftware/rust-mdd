#![feature(test)]
extern crate test;
use test::black_box;
use test::Bencher;

use rust_mdd::cmdc::CMDC_CODEC;
use rust_mdd::codec::Codec;

#[bench]
fn bench_decode(b: &mut Bencher) {
    let data = test_data();

    b.iter(|| black_box(CMDC_CODEC.decode(data)));
}

#[bench]
fn bench_encode(b: &mut Bencher) {
    let data = test_data();
    let containers = CMDC_CODEC.decode(data).unwrap();
    b.iter(|| black_box(CMDC_CODEC.encode(&containers)));
}

fn test_data() -> &'static [u8] {
    sample_data_3()
}

fn sample_data_1() -> &'static [u8] {
    b"<1,18,0,-6,5222,2>[1,20,<1,2,0,452,5222,2>[100],4]"
}

fn sample_data_2() -> &'static [u8] {
    b"<1,8,0,-6,5222,2>[,,2,(5:AMF-1),(4:eMBB),(11:SouthWestUK),1]<1,1,0,-5,5222,2>[1000001]<1,7,0,263,5222,2>[2,{<1,5,1,330,5222,2>[4,200,1,17485760.0,17485824.0]},(6:555555),0,0.0,64.0,200]<1,11,0,626,5222,2>[{1,3,1},,{<1,17,1,624,5222,2>[17485824.0,200,1,(21:Data: Asset + Overage),0:1:5:277,(7:2000000),3,0,,,,,,,,0]},,{(13:HXS0:1:52:409)},{<1,8,1,1000,5222,2>[4,(18:Triple Play Bundle),1,0,,,,0]},{<1,5,1,1277,5222,2>[4,(17:999 - 1200TB Plan),1,<1,6,1,-11,5222,2>[800000.0,1200.0,300000.0,5000000000.0,100000.0,5000000000.0]<1,0,0,1257,5222,2>[]]},,{<1,3,1,1360,5222,2>[,(5:Usage),(5:Usage)]},{<1,2,1,627,5222,2>[(13:HXS0:1:52:408),1]<1,29,0,208,5222,2>[0:1:5:279,(7:1000001),0:1:5:283,,4,0:1:5:278,0:1:5:277,(7:2000000),,,,{<1,14,1,209,5222,2>[,1000,2,1,4,2021-09-07T08:00:25.000000Z,2021-10-07T08:00:25.000000Z,1,0.0,,,-1258291032.242187,,0]},{<1,12,1,567,5222,2>[17485824.0,200,0,0,1,0.0,,1,,1,0]},2021-09-09T16:37:19.000000Z,,(13:HXS0:1:52:409),,,,,1,,,0:1:5:281,,1,2]}]<1,29,0,208,5222,2>[0:1:5:279,(7:1000001),0:1:5:283,,0,0:1:5:280,0:1:5:279,(7:1000001),,,,,,2021-09-09T16:37:19.000000Z,0,(13:HXS0:1:52:408),,,,,1,,,0:1:5:281,,1,1,(26:00000000000000594134:00000)]"
}

fn sample_data_3() -> &'static [u8] {
    b"<1,8,0,-6,5222,2>[,,,(5:AMF-1),(4:eMBB),(11:SouthWestUK),1]<1,1,0,-5,5222,2>[1000001]<1,7,0,263,5222,2>[2,{<1,5,1,330,5222,2>[200,4,1],<1,5,1,330,5222,2>[202,6,2]},(6:555555)]<1,11,0,626,5222,2>[{1,3,1},,{<1,17,1,624,5222,2>[17485824.0,200,1,(21:Data: Asset + Overage),0:1:5:277,(7:2000000),3,0,,,,,,,,0]},0:1:5:144,{(13:HXS0:1:52:409)}]<1,29,0,208,5222,2>[]"
}

fn main() {}
