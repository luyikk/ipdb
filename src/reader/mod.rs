pub mod meta;
mod city;
mod district;
mod idc;
mod base_station;

pub use meta::Meta;
pub use city::CityInfo;
pub use district::DistrictInfo;
pub use idc::IdcInfo;
pub use base_station::BaseStationInfo;
use anyhow::*;
use std::path::Path;
use std::convert::TryInto;
use std::net::{IpAddr};
use std::str::FromStr;
use std::collections::BTreeMap;


pub struct Reader{
    data:Vec<u8>,
    meta:Meta,
    v4offset:usize
}

impl Reader{
    pub fn open_file<T:AsRef<Path>>(file:T)->Result<Reader>{
        let path=file.as_ref();
        ensure!(path.exists(),"not found ipdb file:{:?}",path);
        let data= std::fs::read(path)?;
        let meta_length=u32::from_be_bytes((&data[..4]).try_into()?) as usize+4;
        let meta=serde_json::from_str::<Meta>(std::str::from_utf8(&data[4..meta_length] )?)?;
        ensure!(meta.total_size+meta_length==data.len(),"database file size error");
        let data=data[meta_length..].to_vec();
        let mut node=0usize;
        for i in 0..96 {
            if node >=meta.node_count{
                break;
            }
            if i>=80{
                let off=node*8+1*4;
                node= u32::from_be_bytes((&data[off..off+4]).try_into()?) as usize;
            }else{
                let off=node*8;
                node= u32::from_be_bytes((&data[off..off+4]).try_into()?) as usize;
            }
        }

        Ok(Reader{
            data,
            meta,
            v4offset:node
        })
    }

    #[inline]
    fn resolve(&self,node:usize)->Result<&str> {
        let resolved = node - self.meta.node_count +  self.meta.node_count * 8;
        ensure!(resolved<self.data.len(),"database resolve error,resolved:{}>file length:{}",resolved,self.data.len());
        let size=u32::from_be_bytes([0u8,0u8,self.data[resolved],self.data[resolved+1]]) as usize+resolved+2;
        ensure!(self.data.len()>size,"database resolve error,size:{}>file length:{}",size,self.data.len());
        unsafe {
            Ok(std::str::from_utf8_unchecked(&self.data[resolved + 2..size]))
        }
    }

    #[inline]
    fn read_node(&self,node:usize,index:usize)->Result<usize>{
        let off=node*8+index*4;
        Ok(u32::from_be_bytes((&self.data[off..off+4]).try_into()?) as usize)
    }
    #[inline]
    fn find_node(&self,binary:&[u8])->Result<usize>{
        let mut node=0;
        let bit=binary.len()*8;
        if bit ==32{
            node=self.v4offset;
        }
        for i in 0..bit {
            if node >self.meta.node_count{
                return Ok(node)
            }
            node=self.read_node(node, (1 & ((0xFF & binary[i / 8]) >> 7 - (i % 8))) as usize)?;
        }

        if node >self.meta.node_count{
            return Ok(node)
        }else{
            bail!("not found ip")
        }
    }

    #[inline(always)]
    pub fn is_ipv4(&self)->bool {
        self.meta.ip_version & 0x01 == 0x01
    }

    #[inline(always)]
    pub fn is_ipv6(&self)->bool {
        self.meta.ip_version & 0x02 == 0x02
    }

    #[inline]
    pub fn find(&self,addr:&str,language:&str)->Result<Vec<&str>>{
        let addr=IpAddr::from_str(addr)?;
        ensure!(!self.meta.fields.is_empty(),"fields is empty");
        let off=*self.meta.languages.get(language).ok_or_else(||anyhow!("not found language:{}",language))?;
        let mut _ipv4_buff;
        let mut _ipv6_buff;
        let ipv =match &addr{
                IpAddr::V4( v)=>{
                    ensure!(self.is_ipv4(),"error:ipdb is ipv6");
                    _ipv4_buff= v.octets();
                    &_ipv4_buff[..]
                },
                IpAddr::V6( v)=>{
                    ensure!(self.is_ipv6(),"error:ipdb is ipv4");
                    _ipv6_buff=  v.octets();
                    &_ipv6_buff[..]
                }
            };
        let node= self.find_node(ipv)?;
        let context=self.resolve(node)?;
        let sp:Vec<&str>=context.split('\t').skip(off).collect();
        Ok(sp)
    }

    #[inline]
    pub fn find_city_info(&self,addr:&str,language:&str)->Result<CityInfo>{
        Ok(self.find(addr,language)?.into())
    }

    #[inline]
    pub fn find_district_info(&self,addr:&str,language:&str)->Result<DistrictInfo>{
        Ok(self.find(addr,language)?.into())
    }

    #[inline]
    pub fn find_idc_info(&self,addr:&str,language:&str)->Result<IdcInfo>{
        Ok(self.find(addr,language)?.into())
    }

    #[inline]
    pub fn find_base_station_info(&self,addr:&str,language:&str)->Result<BaseStationInfo>{
        Ok(self.find(addr,language)?.into())
    }

    #[inline]
    pub fn find_map(&self,addr:&str,language:&str)->Result<BTreeMap<&str,&str>>{
        let v=self.find(addr,language)?;
        let k=&self.meta.fields;
        let mut map:BTreeMap<&str,&str>=BTreeMap::new();
        for i in 0..v.len() {
            let value=v[i];
            let key=k.get(i).ok_or_else(||anyhow!("keys len too small"))?;
            map.insert(key.as_str(),value);
        }
        Ok(map)
    }
}