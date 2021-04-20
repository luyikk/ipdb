# IPDB RS library

## example:
```rust
use ipdb::Reader;
use anyhow::*;
use lazy_static::*;

lazy_static!{
    static ref IPDB:Reader={
        Reader::open_file("ipipfree.ipdb").unwrap()
    };
}

fn main()->Result<()> {
    let r= IPDB.find("199.193.127.3","CN")?;
    println!("{:?}",r);
    let r= IPDB.find_city_info("199.193.127.3","CN")?;
    println!("{:?}",r);
    let r= IPDB.find_district_info("199.193.127.3","CN")?;
    println!("{:?}",r);
    let r= IPDB.find_idc_info("199.193.127.3","CN")?;
    println!("{:?}",r);
    let r= IPDB.find_base_station_info("199.193.127.3","CN")?;
    println!("{:?}",r);
    Ok(())
}
```
```shell
.../.cargo/bin/cargo.exe run --color=always --package ipdb --example main
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target\debug\examples\main.exe`
["美国", "加利福尼亚州", "洛杉矶", "", "it7.net", "34.052234", "-118.243685", "America/Los_Angeles", "UTC-7", "", "1", "US", "NA"]
CityInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", latitude: "34.052234", longitude: "-118.243685", timezone: "America/Los_Angeles", utcoffset: "UTC-7", china_admin_code: "", idd_code: "1", country_code: "US", continent_code: "NA", idc: "", base_station: "", country_code3: "", european_union: "", currency_code: "", currency_name: "", any_cast: "" }
DistrictInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", latitude: "34.052234", longitude: "-118.243685" }
IdcInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", idc: "34.052234" }
BaseStationInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", base_station: "34.052234" }

Process finished with exit code 0
```

### bench
```shell
running 12 tests
test bench05::test_ipdb    ... bench:       1,046 ns/iter (+/- 118)
test bench05::test_ipdb_rs ... bench:       1,134 ns/iter (+/- 34)
test bench10::test_ipdb    ... bench:         527 ns/iter (+/- 16)
test bench10::test_ipdb_rs ... bench:       1,130 ns/iter (+/- 66)
test bench20::test_ipdb    ... bench:         529 ns/iter (+/- 18)
test bench20::test_ipdb_rs ... bench:       1,130 ns/iter (+/- 58)
test bench40::test_ipdb    ... bench:         527 ns/iter (+/- 52)
test bench40::test_ipdb_rs ... bench:       1,129 ns/iter (+/- 37)
test bench60::test_ipdb    ... bench:         528 ns/iter (+/- 17)
test bench60::test_ipdb_rs ... bench:       1,133 ns/iter (+/- 74)
test bench80::test_ipdb    ... bench:         527 ns/iter (+/- 21)
test bench80::test_ipdb_rs ... bench:       1,128 ns/iter (+/- 49)
```