# Welcome to BryanScript! ❤️

BryanScript (bs for short 🤭) is the little language I'm working on for fun (... and profit 🧐)

It's just a little baby right now but, soon I'd like to have syntax looking something like what's below compiling down to the WebAssembly binary format:

```bs
use RuntimeError from core::error

struct GreetingError
  const message: String

expose struct Person
  const name: String
    mut age:  i32
    
decorate Person(Debug, PartialEq, Hash)
  fun greet(stranger: Person) -> Result<Ok, GreetingError>
    when stranger.name
      nil       => return GreetingError("No one to greet! 🙃")
      otherwise =>
        console.log `Well hello there, $stranger.name!`
        return Ok
    
fun main(args: List<string>) -> Result<Ok(int), RuntimeError>
  const me = Person
    name: "Bryan"
    age:   44
    
  return when me.greet stranger: Person(name: "Jumpy", age: 42)
    GreetingError(ge) => RuntimeError(...ge)
    otherwise         => Ok(0)
```
