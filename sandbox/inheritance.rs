
Modified 1 month ago
Viewed 53k times
97

Using rust 1.2.0

Problem

I'm still in the process of learning Rust (coming from a Javascript background) and am trying to figure out if it is possible for one struct StructB to extend an existing struct StructA such that StructB has all the fields defined on StructA.

In Javascript (ES6 syntax) I could essentially do something like this...

class Person {
    constructor (gender, age) {
        this.gender = gender;
        this.age = age;
    }
}
class Child extends Person {
    constructor (name, gender, age) {
        super(gender, age);
        this.name = name;
    }
}

Constraints

    StructA is from an external cargo package that I have no control over.

Current Progress

I found this blog post on single-inheritance which sounds like exactly what I need.

But trying to implement it resulted in this error message error: virtual structs have been removed from the language. Some searching later and I found out that it had been implemented and then removed per RFC-341 rather quickly.

Also found this thread about using traits, but since StructA is from an external cargo package I don't think it is possible for me to turn it into a trait.

So what would be the correct way to accomplish this in Rust?

    inheritancestructrust

Share
Edit
Follow
Flag
edited Sep 13, 2015 at 19:16
Shepmaster's user avatar
Shepmaster
344k7474 gold badges969969 silver badges12211221 bronze badges
asked Sep 13, 2015 at 17:36
drebabels's user avatar
drebabels
1,72222 gold badges1616 silver badges1313 bronze badges

    I think it will always be sub-optimal to answer this question without knowing the context in which you want to use the newly generated struct and why. In my opinion these design decisions highly depend on the specific use-case. – 
    Jan
    Sep 30, 2020 at 10:22

Add a comment
Start a bounty
5 Answers
Sorted by:
81

There is nothing that exactly matches that. There are two concepts that come to mind.

    Structural composition

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
