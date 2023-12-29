import users

func testHolder() throws {
  print("🔮 \(#function) start.")
  defer { print("✅ \(#function) end.") }

  let holder = Holder(environment: .prod)
  assert(holder.userCount() == 0)

  holder.addUser(named: "Foo")
  assert(holder.getUsers()[0].name == "Foo")
  holder.changeNameOfUser(at: 0, to: "Bar")
  assert(holder.getUsers()[0].name == "Bar")

  assert(holder.getUsers()[0].flags.flags == [])
  holder.addFlagDeletedByUser(at: 0)
  assert(holder.getUsers()[0].flags.flags == [.deletedByUser])

  holder.addUser(named: "Biz")
  holder.changeNameOfUser(at: 1, to: "Buz")
  assert(holder.getUsers().map(\.name) == ["Bar", "Buz"])
}

func test() throws {
  print("🔮 Swift program start.")
  defer { print("✅ Swift program end.") }

  try testHolder()
}

try! test()
