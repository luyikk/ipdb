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
    let r= IPDB.find_map("199.193.127.3","CN")?;
    println!("{:?}",r);
    Ok(())
}
```
```shell
cargo.exe run --color=always --package ipdb --example main
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\examples\main.exe`
["美国", "加利福尼亚州", "洛杉矶", "", "it7.net", "34.052234", "-118.243685", "America/Los_Angeles", "UTC-7", "", "1", "US", "NA"]
CityInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", latitude: "34.052234", longitude: "-118.243685", timezone: "America/Los_Angeles", utcoffset: "UTC-7", china_admin_code: "", idd_code: "1", country_code: "US", continent_code: "NA", idc: "", base_station: "", country_code3: "", european_union: "", currency_code: "", currency_name: "", any_cast: "" }
DistrictInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", latitude: "34.052234", longitude: "-118.243685" }
IdcInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", idc: "34.052234" }
BaseStationInfo { country_name: "美国", region_name: "加利福尼亚州", city_name: "洛杉矶", owner_domain: "", isp_domain: "it7.net", base_station: "34.052234" }
{"china_admin_code": "", "city_name": "洛杉矶", "continent_code": "NA", "country_code": "US", "country_name": "美国", "idd_code": "1", "isp_domain": "it7.net", "latitude": "34.052234", "longitude": "-118.243685", "owner_domain": "", "region_name": "加利福尼亚州", "timezone": "America/Los_Angeles", "utc_offset": "UTC-7"}

Process finished with exit code 0

```

### bench
```shell
running 12 tests
test bench0500::test_ipdb    ... bench:     261,625 ns/iter (+/- 13,556)
test bench0500::test_ipdb_rs ... bench:     318,980 ns/iter (+/- 9,253)
test bench1000::test_ipdb    ... bench:     522,450 ns/iter (+/- 9,787)
test bench1000::test_ipdb_rs ... bench:     635,570 ns/iter (+/- 17,337)
test bench2000::test_ipdb    ... bench:   1,048,630 ns/iter (+/- 71,659)
test bench2000::test_ipdb_rs ... bench:   1,273,910 ns/iter (+/- 43,754)
test bench4000::test_ipdb    ... bench:   2,096,190 ns/iter (+/- 33,412)
test bench4000::test_ipdb_rs ... bench:   2,554,980 ns/iter (+/- 36,575)
test bench6000::test_ipdb    ... bench:   3,140,700 ns/iter (+/- 52,065)
test bench6000::test_ipdb_rs ... bench:   3,837,500 ns/iter (+/- 49,432)
test bench8000::test_ipdb    ... bench:   4,191,190 ns/iter (+/- 49,141)
test bench8000::test_ipdb_rs ... bench:   5,121,350 ns/iter (+/- 47,059)

test result: ok. 0 passed; 0 failed; 0 ignored; 12 measured; 0 filtered out; finished in 7.99s
```