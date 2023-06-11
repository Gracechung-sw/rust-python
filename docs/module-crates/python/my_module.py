from bots.hello_bot import BOT_NAME

def greet():
  print(f"Hi! I am hello bot")
  print(f"Hi I am {BOT_NAME}")

class Person:
  def __init__(self, name, age):
    self.name = name
    self.age = age

  def get_older(self, year):
    self.age += year