use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::{now};
use crate::Result;
use crate::server::{Server, NewServer};
use crate::schema::server::dsl::*;

embed_migrations!();

fn conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn run_db_migrations() -> Result<()> {
    Ok(embedded_migrations::run(&conn())?)
}

pub fn next() -> Option<Server> {
    match server
        .filter(updated.le(now() - 60))
        .order(updated.asc())
        .limit(1)
        .first::<Server>(&conn()) {
            Ok(e) => Some(e),
             _ => None
        }
}

pub fn all() -> Result<Vec<Server>> {
    Ok(server
        .order(players.desc())
        .load::<Server>(&conn())?)
}

pub fn online() -> Result<Vec<Server>> {
    Ok(server
        .filter(ping.ge(0))
        .load::<Server>(&conn())?)
}

fn create(info: &Server) -> Result<Server> {
    let new_server = NewServer {
        real_ip: &info.real_ip,
        ip: &info.ip,
        updated: info.updated,
        ping: info.ping,
    
        host: &info.host,
        map: &info.map,
        players: info.players,
        wave: info.wave,
        version: info.version,
        vertype: &info.vertype,
        gamemode: info.gamemode,
        limit: info.limit,
        description: &info.description,

        fails: &info.fails,
        last_seen: info.last_seen,
    };

    Ok(diesel::insert_into(server)
        .values(&new_server)
        .get_result(&conn())?)
}

pub fn update(info: &Server) -> Result<Server> {
    use diesel::dsl::{select, exists};

    // delete duplicates (eg. when real_ip changes)
    diesel::delete(server
        .filter(ip.eq(&info.ip))
        .filter(real_ip.ne(&info.real_ip))
    ).execute(&conn())?;

    let exists = select(exists(server
        .filter(real_ip.eq(&info.real_ip))))
        .get_result(&conn())?;

    if exists {
        //Ok(info.save_changes(&conn())?)
        Ok(diesel::update(info)
            .set(info)
            .get_result(&conn())?)
    } else {
        create(info)
    }
}