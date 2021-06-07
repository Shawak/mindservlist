table! {
    server (real_ip) {
        real_ip -> Varchar,
        ip -> Varchar,
        updated -> Int8,
        ping -> Int4,
        host -> Varchar,
        map -> Varchar,
        players -> Int4,
        wave -> Int4,
        version -> Int4,
        vertype -> Varchar,
        gamemode -> Int2,
        limit -> Int4,
        description -> Varchar,
        fails -> Int2,
        last_seen -> Int8,
    }
}
