FROM rustlang/rust:nightly

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN curl -O -L https://npmjs.org/install.sh | sh
RUN rustup target add wasm32-unknown-unknown
RUN cargo install miniserve

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . /usr/src/app
EXPOSE 8080

CMD wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html