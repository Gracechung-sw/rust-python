# rust-python
Learn Rust with Python examples. Referenced by https://indosaram.github.io/rust-python-book/, https://google.github.io/comprehensive-rust/welcome.html


## Quick Start.
1. Install Rust compiler, system manager `cargo`.
```bash
# MacOS/Linux
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh  

# run
source "$HOME/.cargo/env"
```

2. Install VSCode Rust Extension.   
코드 자동완성, 에러 표시, 관련 문서 표시 등 다양한 기능이 있지만 가장 좋은 기능 중 하나는 변수의 타입을 추측해서 화면에 표시
- Go VSCode Marketplace
- Search `rust-analyzer` and Install. 

3. Create new Rust project folder
```bash
cargo new rust
```

3. then, you have 
```
.
├── README.md
└── rust
    ├── Cargo.toml
    └── src
        └── main.rs
```

`Cargo.toml`: 프로젝트의 모든 설정값을 가지고 있는 파일
```
[package]
name = "rust_part"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies] 
# 현재 프로젝트에서 설치하는 크레이트(러스트에서는 패키지를 크레이트(crate)라고 부릅니다)의 이름과 버전이 들어가게 됩니다. 
```
`src/main.rs`: src 폴더가 실제 러스트 소스코드가 들어가는 곳입니다. 현재는 코드의 시작 지점(entry point)인 main.rs 파일만 들어 있습니다. 해당 파일에는 main() 함수가 들어 있는데, main.rs 가 컴파일되고 바이너리가 실행될 때 바로 이 main() 함수가 실행됩니다. 따라서 반드시 main.rs 파일이 존재해야 하고, 이 파일 안에 main() 함수가 존재해야 코드가 컴파일되고 실행될 수 있습니다.