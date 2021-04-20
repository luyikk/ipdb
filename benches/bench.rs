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
                for _ in 0..$cap{
                    IPDB.find("58.250.137.36", "CN").unwrap();
                }
            });
        }

        #[bench]
        fn test_ipdb_rs(b: &mut Bencher){
            b.iter(move ||{
                  for _ in 0..$cap{
                    find("58.250.137.36", "CN").unwrap();
                  }
            });
        }

    }}

mod bench8000 {
    bench_all!(8000);
}
mod bench6000 {
    bench_all!(6000);
}
mod bench4000 {
    bench_all!(4000);
}
mod bench2000 {
    bench_all!(2000);
}
mod bench1000 {
    bench_all!(1000);
}
mod bench0500 {
    bench_all!(500);
}
