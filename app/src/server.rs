use crate::mem_read::*;
use std::io::{Error, Read};

use crate::schema::server;

#[derive(Queryable, Identifiable, AsChangeset, Serialize, PartialEq, Debug)]
#[table_name="server"]
#[primary_key(real_ip)]
pub struct Server {
    pub real_ip: String,
    pub ip: String,
    pub updated: i64,
    pub ping: i32,

    pub host: String,
    pub map: String,
    pub players: i32,
    pub wave: i32,
    pub version: i32,
    pub vertype: String,
    pub gamemode: i16,
    pub limit: i32,
    pub description: String,

    pub fails: i16,
    pub last_seen: i64,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name="server"]
pub struct NewServer<'a> {
    pub real_ip: &'a str,
    pub ip: &'a str,
    pub updated: i64,
    pub ping: i32,

    pub host: &'a str,
    pub map: &'a str,
    pub players: i32,
    pub wave: i32,
    pub version: i32,
    pub vertype: &'a str,
    pub gamemode: i16,
    pub limit: i32,
    pub description: &'a str,

    pub fails: &'a i16,
    pub last_seen: i64,
}

impl Server {
    // https://github.com/Anuken/Mindustry/blob/master/core/src/mindustry/net/NetworkIO.java#L82
    pub fn new<T: Read>(real_ip: String, ip: String, updated: i64, ping: i32, data: &mut T) -> Result<Server, Error> {
        Ok(Server {
            real_ip: real_ip,
            ip: ip,
            updated: updated,
            ping: ping,

            host: data.get_str_sized::<u8>()?,
            map: data.get_str_sized::<u8>()?,
            players: data.get_be()?,
            wave: data.get_be()?,
            version: data.get_be()?,
            vertype: data.get_str_sized::<u8>()?,
            gamemode: data.get_be::<u8>()? as _,
            limit: data.get_be()?,
            description: data.get_str_sized::<u8>()?,

            fails: 0,
            last_seen: updated,
        })
    }
}