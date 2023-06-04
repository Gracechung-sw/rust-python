# 1-1.**[파이썬 개발자가 러스트를 배워야 하는 이유](https://indosaram.github.io/rust-python-book/ch1-01.html#%ED%8C%8C%EC%9D%B4%EC%8D%AC-%EA%B0%9C%EB%B0%9C%EC%9E%90%EA%B0%80-%EB%9F%AC%EC%8A%A4%ED%8A%B8%EB%A5%BC-%EB%B0%B0%EC%9B%8C%EC%95%BC-%ED%95%98%EB%8A%94-%EC%9D%B4%EC%9C%A0)**

파이썬과 러스트의 어떤 차이점 때문에 파이썬 개발자들이 러스트를 배워야만 할까요?

1. **[CPU 연산이 많이 필요한 코드를 러스트로 교체하면 빠르게 동작하는 프로그램을 만들 수 있습니다.](https://indosaram.github.io/rust-python-book/ch1-01.html#%EC%B2%AB%EC%A7%B8%EB%A1%9C-cpu-%EC%97%B0%EC%82%B0%EC%9D%B4-%EB%A7%8E%EC%9D%B4-%ED%95%84%EC%9A%94%ED%95%9C-%EC%BD%94%EB%93%9C%EB%A5%BC-%EB%9F%AC%EC%8A%A4%ED%8A%B8%EB%A1%9C-%EA%B5%90%EC%B2%B4%ED%95%98%EB%A9%B4-%EB%B9%A0%EB%A5%B4%EA%B2%8C-%EB%8F%99%EC%9E%91%ED%95%98%EB%8A%94-%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%A8%EC%9D%84-%EB%A7%8C%EB%93%A4-%EC%88%98-%EC%9E%88%EC%8A%B5%EB%8B%88%EB%8B%A4)**
    1. 1) 인터프리터 언어이기 때문에 다른 컴파일 언어에 비해서 속도가 느릴 수밖에 없습니다.
    2. 데이터 분석이나 수치계산 분야에서는 이미 널리 쓰이는 pandas나 numpy와 같은 라이브러리가 C++로 작성되어 있습니다.
    3. 2) 러스트와 자주 비교되는 언어인 고(Go)와 다르게, 러스트에는 소유권(ownership) 모델을 사용해서  가비지 콜렉터가 없기 때문에 훨씬 좋은 성능을 내게 됩니다. 이러한 특징 때문에 퍼포먼스가 매우 중요한 서비스에 자주 사용됩니다.
    
2. **[멀티스레딩 구현이 훨씬 쉽습니다.](https://indosaram.github.io/rust-python-book/ch1-01.html#%EB%91%98%EC%A7%B8%EB%A1%9C-%EB%A9%80%ED%8B%B0%EC%8A%A4%EB%A0%88%EB%94%A9-%EA%B5%AC%ED%98%84%EC%9D%B4-%ED%9B%A8%EC%94%AC-%EC%89%BD%EC%8A%B5%EB%8B%88%EB%8B%A4)**
    1. 1) 파이썬에서 멀티스레딩 프로그램을 구현할 때 가장 많이 겪는 문제가 스레드 레이스 조건(race condition)입니다
    2. 러스트만의 독특한 타입 시스템과 소유권(ownership) 모델 덕분에 코드가 컴파일될 때 발생할 수 있는 메모리 혹은 스레드 문제를 미리 찾아낼 수 있기 때문에 훨씬 안정적인 프로그램을 만들 수 있습니다
    3. 2) 파이썬은 GIL(Global Interpreter Lock) 때문에 멀티스레딩이라 하더라도 한 번에 하나의 코어밖에 사용하지 않습니다.
    4. 러스트를 사용하면 GIL 락이 걸린 순간에 여러 스레드를 사용해 더 빠르게 계산을 완료할 수 있습니다.
    
    3. **[개발 도구가 매우 편리합니다.](https://indosaram.github.io/rust-python-book/ch1-01.html#%EB%A7%88%EC%A7%80%EB%A7%89%EC%9C%BC%EB%A1%9C-%EA%B0%9C%EB%B0%9C-%EB%8F%84%EA%B5%AC%EA%B0%80-%EB%A7%A4%EC%9A%B0-%ED%8E%B8%EB%A6%AC%ED%95%A9%EB%8B%88%EB%8B%A4)**
    
    1) 매우 친절한 컴파일러가 있습니다. 때로는 컴파일 시에 발생하는 오류에 대해 적절한 해결책을 컴파일러가 제시해 주기도 합니다.
    
    2) 러스트의 내장 패키지 매니저인 Cargo 덕분에 빌드, 테스트, 의존성 관리 등이 매우 간편합니다.
    
    ## [파이썬과 러스트의 차이점](https://indosaram.github.io/rust-python-book/ch1-01.html#%ED%8C%8C%EC%9D%B4%EC%8D%AC%EA%B3%BC-%EB%9F%AC%EC%8A%A4%ED%8A%B8%EC%9D%98-%EC%B0%A8%EC%9D%B4%EC%A0%90)
    
    ### [언어상의 차이](https://indosaram.github.io/rust-python-book/ch1-01.html#%EC%96%B8%EC%96%B4%EC%83%81%EC%9D%98-%EC%B0%A8%EC%9D%B4)
    
    먼저 기본적인 언어상의 차이를 살펴보면 다음과 같습니다.
    
    | 파이썬 | 러스트 |
    | --- | --- |
    | 인터프리터 언어 | 컴파일 언어 |
    | 강타입 언어이면서 동적 타입 언어 | 강타입 언어이면서 정적 타입 언어 |
    | 메모리 관리에 가비지 콜렉터 사용 | 메모리 관리에 소유권 모델 사용 |
    | 대부분의 경우 객체지향 프로그래밍 | 함수형 프로그래밍 |
    | 스타일 가이드가 유연함 | 명확한 스타일 가이드 존재 |
    
    **[툴 비교](https://indosaram.github.io/rust-python-book/ch1-01.html#%ED%88%B4-%EB%B9%84%EA%B5%90)**
    
    아래 표는 파이썬과 러스트의 기본 툴들을 비교한 표입니다. 파이썬의 경우, `pip` 를 제외한 툴들은 일반적으로 별도 설치가 필요합니다. 하지만 러스트는 `cargo` 라는 툴을 통해 대부분의 기능을 바로 사용할 수 있습니다.

    |  | 파이썬 | 러스트 |
    | --- | --- | --- |
    | 패키지 관리자 | pip | cargo |
    | 포매터 | black, yapf, autopep8 | cargo fmt |
    | 린터 | pylint, flake8 | cargo clippy |
    | 테스트 | pytest | cargo test |
    | 프로젝트 환경 관리 | virtualenv, pipenv, pyenv, conda | cargo new |
    | 문서화 | sphinx | cargo doc |
    | 벤치마크 | cProfile, pyspy | cargo bench |