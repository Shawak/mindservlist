-- Your SQL goes here

create table server (
    real_ip varchar primary key not null,
    ip varchar not null,
    updated int8 not null,
    ping int4 not null,

    host varchar not null,
    map varchar not null,
    players int4 not null,
    wave int4 not null,
    version int4 not null,
    vertype varchar not null,
    gamemode int2 not null,
    "limit" int4 not null,
    description varchar not null
)
