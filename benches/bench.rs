#![feature(test)]
extern crate test;


use test::Bencher;
use ipdb::Reader;

#[macro_use]
extern crate lazy_static;
use ipdb_rs::find;

lazy_static!{
    static ref IPDB:Reader={
        Reader::open_file("ipipfree.ipdb").unwrap()
    };
}

macro_rules! bench_all {
    ($cap:literal) => {
        use super::*;
        #[bench]
        fn test_ipdb(b: &mut Bencher){
            b.iter(move ||{
                    IPDB.find("58.250.137.36", "CN").unwrap();
            });
        }

        #[bench]
        fn test_ipdb_rs(b: &mut Bencher){

            b.iter(move ||{
                    find("58.250.137.36", "CN").unwrap();
            });
        }

    }}

mod bench80 {
    bench_all!(80);
}
mod bench60 {
    bench_all!(60);
}
mod bench40 {
    bench_all!(40);
}
mod bench20 {
    bench_all!(20);
}
mod bench10 {
    bench_all!(10);
}
mod bench05 {
    bench_all!(5);
}
