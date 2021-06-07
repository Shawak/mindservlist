FROM rust:1.52-buster

RUN mkdir /app && cd /app
WORKDIR /app

RUN cargo install diesel_cli --version 1.4.0 --no-default-features --features postgres

RUN rustup default nightly && rustup update nightly

#ENTRYPOINT [ "cargo", "build" ]
#ENTRYPOINT [ "diesel", "migration", "run" ]

#CMD [ "tail", "-f", "/dev/null" ]
CMD [ "cargo", "run" ]
