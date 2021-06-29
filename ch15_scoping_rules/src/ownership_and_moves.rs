mod moves {
    // This function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
    }

    #[test]
    fn main() {
        // _Stack_ allocated integer
        let x = 5u32;

        // *Copy* `x` into `y` - no resources are moved
        let y = x;

        println!("x is {}, y is {}", x, y);

        // `a` is a pointer to a _heap_ allocated integer
        let a = Box::new(5i32);
        println!("a contains: {}", a);

        // *Move* `a` into `b`
        let b = a;
        // The pointer address of `a` is copied (not the data) into `b`.
        // Both are now pointers to the same heap allocated data, but
        // `b` now owns it.

        // Error! `a` can no longer access the data, because it no longer owns the
        // heap memory
        // println!("a contains: {}", a);

        // This function takes ownership of the heap allocated memory from `b`
        destroy_box(b);

        // Since the heap memory has been freed at this point, this action would
        // result in dereferencing freed memory, but it's forbidden by the compiler
        // println!("b contains: {}", b);
    }
}

mod mutability {
    #[test]
    fn main() {
        let imm = Box::new(5u32);

        println!("imm box: {}", imm);

        // Mutability error
        // *imm = 4;

        // *Move* the box, changing the ownership (and mutability)
        let mut mu = imm;

        println!("mu box: {}", mu);

        *mu = 4;

        println!("mu box: {}", mu);
    }
}

mod partial_moves {
    #[test]
    fn main() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let person = Person {
            name: String::from("Alice"),
            age: 20,
        };

        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;

        println!("age is {}", age);
        println!("name is {}", name);

        // Error! borrow of partially moved value: `person` partial move occurs
        // println!("person is {:?}", person);

        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("person's age is {}", person.age);
    }
}