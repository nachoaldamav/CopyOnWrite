FROM rust:1.73

WORKDIR /usr/src/myapp
COPY . .

CMD ["cargo", "test", "--release"]