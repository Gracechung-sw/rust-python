mod my_module; // mod 키워드로 ./rust/my_module.rs의 my_module을 스코프로 가져온다. 
mod bots; // will look for a file src/bots/mod.rs

use my_module::{greet, Person} // 그 다음 가져오고자 하는 함수와 구조체를 가져올 수 있습니다.
use bots::hello_bot::hello; // actually import the function from hello.rs

// 이렇게 가져온 함수와 구조체를 이제 main() 함수 내에서 사용할 수 있습니다
fn main() {
  hello(); 
  
  greet();

  let mut john = Person::new("john", 20);
  john.get_older(3);
  println!("{}", john.age);
}