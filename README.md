# horba
An OOP programming language I am making by following Crafting Interpreters.
https://craftinginterpreters.com/

I intend it to have a somewhat C-style syntax, as it is very terse and pretty flexible, without having types before declarations, instead having keywords.
I also don't want semicolons, as they are visual noise. 

I want the language to look something like this:
```v
pub class Hello {
    let str
    let something
    new(...) {
        // Autogenerate constructor function with ..., will do `self.thing = thing` automatically.
        self.something = Number.from(something)
        ...
        
    } // Default init, does self.string = string automatically, can override and put ... for autogeneration
    pub method print() {
        // Method keyword for methods, don't need to use `self.` and distinguished from functions.
        println(str)
    }
    
    trait fn nice(); // Inheritors must define this, shared behaviour marker
}

class WhenThe inherits Hello {
    new(str, something, ...) {
        new super(Hello)(str, something)
        ...
    }
    
    fn nice() {
        println("Nice!")
    }
}

fn main() {
    let thing = new Hello("Hello, World", "3")
    thing.print()
    let array = new Array()
    let other_array = [1, 2, 3, 4]
    array.push(1)
}
```

I will eventually port it to bytecode (as the book goes), and try making a type system for it.
