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
    println!("{}", serde_json::to_string(&r)?);
    let r= IPDB.find_district_info("199.193.127.3","CN")?;
    println!("{}", serde_json::to_string(&r)?);
    let r= IPDB.find_idc_info("199.193.127.3","CN")?;
    println!("{}", serde_json::to_string(&r)?);
    let r= IPDB.find_base_station_info("199.193.127.3","CN")?;
    println!("{}", serde_json::to_string(&r)?);
    let r= IPDB.find_map("199.193.127.3","CN")?;
    println!("{}", serde_json::to_string(&r)?);
    Ok(())
}