FROM rust:1.48.0

WORKDIR /usr/src/shakemon
COPY . .

RUN cargo install --path .

CMD ["shakemon"]
