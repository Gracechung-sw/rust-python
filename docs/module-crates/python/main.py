from my_module import greet, Person
from bots.hello_bot import hello

if __name__ == '__main__':
  hello()

  greet()

  john = Person("john", 20)
  john.get_older(3)
  print(john.age)