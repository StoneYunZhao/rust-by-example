mod borrowing {
    // This function takes ownership of a box and destroys it
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying: {}", boxed_i32);
    }

    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("int is: {}", borrowed_i32);
    }

    #[test]
    fn main() {
        let boxed_i32 = Box::new(5_i32);
        let stacked_i32 = 6_i32;

        // Borrow the contents of the box. Ownership is not taken,
        // so the contents can be borrowed again.
        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            // Take a reference to the data contained inside the box
            let _ref_to_i32: &i32 = &boxed_i32;


            // eat_box_i32(boxed_i32);

            // Attempt to borrow `_ref_to_i32` after inner value is destroyed
            borrow_i32(_ref_to_i32);
        }

        eat_box_i32(boxed_i32);
    }
}

mod mutability {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` is a reference to a string allocated in read only memory
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!("immutably {} - {}", book.title, book.year);
    }

    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("mutably {} - {}", book.title, book.year);
    }

    #[test]
    fn main() {
        let imm = Book {
            author: "Douglas",
            title: "Bach",
            year: 1979,
        };

        // Create a mutable copy of `imm`
        let mut mutabook = imm;

        // Immutably borrow an immutable object
        borrow_book(&imm);

        // Immutably borrow a mutable object
        borrow_book(&mutabook);

        // Borrow a mutable object as mutable
        new_edition(&mut mutabook);

        // Error! Cannot borrow an immutable object as mutable
        // new_edition(&mut imm);
    }
}

mod aliasing {
    struct Point {
        x: i32,
        y: i32,
        z: i32,

    }

    #[test]
    fn main() {
        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        println!("coordinated: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // Error! Can't borrow `point` as mutable because it's currently
        // borrowed as immutable.
        // let mutable_borrow = &mut point;

        println!("coordinated: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // The immutable references are no longer used for the rest of the code so
        // it is possible to reborrow with a mutable reference.
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable.
        // let y = &point.y;

        // Error! Can't print because `println!` takes an immutable reference.
        // println!("z is {}", point.z);

        // Ok! Mutable references can be passed as immutable to `println!`
        println!("coordinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // The mutable reference is no longer used for the rest of the code so it
        // is possible to reborrow
        let new_borrowed_point = &point;

        println!("coordinates: ({}, {}, {})", new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
    }
}

mod ref_pattern {
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn main() {
        let c = 'Q';

        // A `ref` borrow on the left side of an assignment is equivalent to
        // an `&` borrow on the right side.
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("equals: {}", *ref_c1 == *ref_c2);

        let point = Point { x: 0, y: 0 };

        // `ref` is also valid when destructuring a struct.
        let _copy_of_x = {
            let Point { x: ref ref_to_x, y: _ } = point;

            // Return a copy of the `x` field of `point`.
            *ref_to_x
        };

        // A mutable copy of `point`
        let mut mutable_point = point;

        {
            // `ref` can be paired with `mut` to take mutable references.
            let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

            *mut_ref_to_y = 1;
        }

        println!("point is ({}, {})", point.x, point.y);
        println!("mutable point is ({}, {})", mutable_point.x, mutable_point.y);

        let mut mutable_tuple = (Box::new(5u32), 4u32);

        {
            let (_, ref mut last) = mutable_tuple;
            *last = 2u32;
        }

        println!("tuple is {:?}", mutable_tuple);
    }
}