
#[derive(Debug)]
pub struct  BaseStationInfo<'a>{
    pub country_name:&'a str,
    pub region_name:&'a str,
    pub city_name:&'a str,
    pub owner_domain:&'a str,
    pub isp_domain:&'a str,
    pub base_station:&'a str,
}

impl<'a> From<Vec<&'a str>> for  BaseStationInfo<'a>{
    fn from(buff: Vec<&'a str>) -> Self {
        BaseStationInfo {
            country_name: if buff.len() > 0 { buff[0] } else { "" },
            region_name: if buff.len() > 1 { buff[1] } else { "" },
            city_name: if buff.len() > 2 { buff[2] } else { "" },
            owner_domain: if buff.len() > 3 { buff[3] } else { "" },
            isp_domain: if buff.len() > 4 { buff[4] } else { "" },
            base_station: if buff.len() > 5 { buff[5] } else { "" }
        }
    }
}


impl<'a> ToString for BaseStationInfo<'a>{
    fn to_string(&self) -> String {
        format!("{:?}",self)
    }
}