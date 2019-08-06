cd ../streamer-site-frontend
ng build --prod --aot
cd ../docker
cd ../tsquery_rs
cargo build --release --target=x86_64-unknown-linux-musl
cd ../docker

yes | cp -r ../streamer-site-frontend/dist .
yes | cp ../tsquery_rs/target/x86_64-unknown-linux-musl/release/tsquery_rs .
