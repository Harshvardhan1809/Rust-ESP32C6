Try following the instructions in the documentation to get ESP-IDF on Rust working: https://docs.esp-rs.org/book/installation/index.html and then execute these commands: 
```
cargo install ldproxy
brew install espflash
cargo build
cargo run --example mpu6050 --target riscv32imac-esp-espidf
```

In case of ESP32C6 the target is `riscv32imac-esp-espidf` and in case of ESP323 the target is `riscv32imc-esp-espidf`. NE0-6M works partially (haven't implemented NMEA parsing yet).
