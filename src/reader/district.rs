use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct  DistrictInfo<'a>{
    pub country_name:&'a str,
    pub region_name:&'a str,
    pub city_name:&'a str,
    pub owner_domain:&'a str,
    pub isp_domain:&'a str,
    pub latitude:&'a str,
    pub longitude:&'a str,
}

impl<'a> From<Vec<&'a str>> for  DistrictInfo<'a>{
    fn from(buff: Vec<&'a str>) -> Self {
        DistrictInfo {
            country_name: if buff.len() > 0 { buff[0] } else { "" },
            region_name: if buff.len() > 1 { buff[1] } else { "" },
            city_name: if buff.len() > 2 { buff[2] } else { "" },
            owner_domain: if buff.len() > 3 { buff[3] } else { "" },
            isp_domain: if buff.len() > 4 { buff[4] } else { "" },
            latitude: if buff.len() > 5 { buff[5] } else { "" },
            longitude: if buff.len() > 6 { buff[6] } else { "" }
        }
    }
}


impl<'a> ToString for DistrictInfo<'a>{
    fn to_string(&self) -> String {
        format!("{:?}",self)
    }
}