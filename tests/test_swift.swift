import users

func test() throws {
  print("🔮 Swift program start.")
  defer { print("✅ Swift program end.") }

  let holder = Holder(environment: .prod)
  assert(holder.userCount() == 0)
  holder.addUser(named: "Foo")
  assert(holder.getUsers()[0].name == "Foo")
  holder.changeNameOfUser(at: 0, to: "Bar")
  assert(holder.getUsers()[0].name == "Bar")
}

try! test()
