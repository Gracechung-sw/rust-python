# rust-python
Learn Rust with Python examples. Referenced by 
- https://indosaram.github.io/rust-python-book/, 
- https://google.github.io/comprehensive-rust/welcome.html
- https://rinthel.github.io/rust-lang-book-ko/foreword.html


## Quick Start.
### 1. Install Rust compiler, system manager `cargo`.
```bash
# MacOS/Linux
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh  

# run
$ source "$HOME/.cargo/env"
```

### 2. Install VSCode Rust Extension.   
코드 자동완성, 에러 표시, 관련 문서 표시 등 다양한 기능이 있지만 가장 좋은 기능 중 하나는 변수의 타입을 추측해서 화면에 표시
- Go VSCode Marketplace
- Search `rust-analyzer` and Install. 

### 3. Create new Rust project folder
```bash
$ cargo new rust
```

### 4. then, you have 
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

### 5. Run rust code
#### 5-1. compile 
```rust
fn main() {
    println!("Hello, world!");
}
```

```bash
$ cd rust
$ cargo build

Compiling rust v0.1.0 (/Users/code/temp)
 Finished dev [unoptimized + debuginfo] target(s) in 2.07s
```

then, 러스트 폴더 밑에 target 이라는 폴더가 생성되고, 여기 밑에 debug 폴더를 열어보면 rust라는 바이너리 파일이 존재합니다.
```
.
├── README.md
├── python
│   └── main.py
└── rust
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── main.rs
    └── target
        ├── CACHEDIR.TAG
        └── debug
            ├── build
            ├── deps
            ├── examples
            ├── incremental
            ├── rust #  <- this!!
            └── rust.d
```
#### 5-2. run compiled binary file(./target/debug/rust)
```bash
$ ./target/debug/rust
Hello, world!

```
#### 5-3. Build for release
기본적으로 빌드는 디버그 모드로 수행됩니다. 디버그 모드는 좀더 빨리 컴파일이 수행되지만, 프로그램의 실행 속도는 느려질 수 있습니다. 하지만 실제로 프로그램을 배포할 때는 컴파일 단계에서 코드를 최적화해주어야 성능을 제대로 사용할 수 있습니다. 따라서 프로그램을 배포할 때에는 다음과 같이 --release 옵션을 추가로 사용합니다.
```bash
$ cargo build --release

Compiling notebook v0.1.0 (/Users/code/temp)
 Finished release [optimized] target(s) in 1.34s

```
#### 5-4. run build file (./target/release/rust )
```bash
$ ./target/release/rust
Hello, world!

```

If you run this code at once, use `cargo run` or   `cargo run --release`

### 6. Code formatting
러스트에는 내장 코드 포맷터인 rustfmt가 설치되어 있습니다. VSCode에서는 단축키를 사용해 코드를 포맷할 수 있습니다. 
맥의 경우는 Option + Shift + F를 누르면 됩니다. 또는 터미널에서 rustfmt src/main.rs 명령어를 사용해도 됩니다.
만약 현재 파일을 포함한 프로젝트의 전체 러스트 코드를 한꺼번에 포매팅하고 싶다면 아래 명령어를 사용합니다.

```bash
$ cargo fmt
```