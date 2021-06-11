#![allow(dead_code,unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait_v2)]
#[macro_use] extern crate async_std;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate serde;

extern crate dotenv;

use std::time::Duration;
use async_std::*;
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use std::thread;

mod server;
mod client;
mod mem_read;
mod schema;
mod db;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use crate::server::*;
use crate::client::*;

#[get("/all")]
fn all() -> JsonValue {
    match db::all() {
        Ok(data) => {
            json!({
                "server": data
            })
        },
        Err(err) => {
            json!({
                "error": err.to_string()
            })
        }
    }
}

#[get("/online")]
fn online() -> JsonValue {
    match db::online() {
        Ok(data) => {
            json!({
                "server": data
            })
        },
        Err(err) => {
            json!({
                "error": err.to_string()
            })
        }
    }
}

#[get("/server/<ip>")]
fn server(mut ip: String) -> JsonValue {
    ip = ip.to_lowercase();
    if !ip.contains(":") {
        ip =  ip + ":6567";
    }

    let result = task::block_on(get(&ip));
    let info = match result {
        Ok(info) => info,
        Err(err) => return json!({
            "error": err.to_string()
        })
    };

    match db::update(&info) {
        Ok(res) => json!({
            "success": "yes",
            "server": res
        }),
        Err(err) => json!({
            "error": err.to_string()
        })
    }
}

async fn update() -> Result<Option<Server>> {
    let mut next = match db::next() {
        Some(next) => next,
        None => return Ok(None)
    };

    // println!("updating {:?}", next.ip);
    Ok(Some(match get(&next.ip).await {
        Ok(mut info) => {
            next.fails = 0;
            info.last_seen = now();
            info.updated = now();
            db::update(&info)?
        },
        Err(_err) => {
            // println!("{:?}", err);
            if next.fails < 3 {
                next.fails += 1;
            } else {
                next.players = 0;
                next.wave = 0;
                next.ping = -1;
            }
            next.updated = now();
            db::update(&next)?
        }
    }))
}

#[async_std::main]
async fn main() -> Result<()> {
    db::run_db_migrations()?;

    let fn_update = async {
        println!("Starting update task..");
        loop {
            task::sleep(Duration::from_millis(100)).await;
            match update().await {
                // Ok(info) => println!("{:?}", info),
                Err(err) => println!("error {:?}", err),
                _ => ()
            }
        }        
    };

    let task_update = task::spawn(fn_update);

    let rocket_thread = thread::spawn(move || {
        println!("Starting rocket task..");
        rocket::ignite()
            .mount("/api/", routes![
                all,
                online,
                server
            ])
            .launch()
    });

    task_update.await;
    
    if let Err(err) = rocket_thread.join() {
        println!("{:?}", err);
    }

    println!("Done");
    Ok(())
}

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("could not get system time")
        .as_secs() as _
}

async fn get(address: &String) -> Result<Server> {
    let mut client = Client::new().await?;
    let res = client.get(address).await?;
    Ok(res)
}
