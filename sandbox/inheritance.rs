    struct Person {
        age: u8,
    }

    struct Child {
        person: Person,
        has_toy: bool,
    }

    impl Person {
        fn new(age: u8) -> Self {
            Person { age: age }
        }

        fn age(&self) -> u8 {
            self.age
        }
    }

    impl Child {
        fn new(age: u8, has_toy: bool) -> Self {
            Child { person: Person::new(age), has_toy: has_toy }
        }

        fn age(&self) -> u8 {
            self.person.age()
        }
    }

    fn main() {
        let p = Person::new(42);
        let c = Child::new(7, true);

        println!("I am {}", p.age());
        println!("My child is {}", c.age());
    }
