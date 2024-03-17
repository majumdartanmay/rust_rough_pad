# Introduction

I want to write my journey while I learn rust. Its easy to revise the things I learnt about Rust from here.

## Ownership

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

### Rules

1. Every variable has an owner
2. When the owner goes out of scope. The variable value is dropped.
3. There can be atmost 1 owner

### Scope

1. Primitive data types are stored in stack since they are of known size. They can also be quickly copied to make a new independent instance when there is another part of the program in a new scope which needs the same variable.

**Handing strings**

1. We can either make string literals or make strings which can be mutated.
2. Mutable string is `let s = String::from("Hello World")`

## [Memory and allocation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

In the case of string literal we know the contents of the compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.

With the `String` type we allocate some memory on the heap which is unknown at compile time.

1. Memory is requested from the memory allocator at runtime.
2. The memory needs to be returned when the string is fulfilled its purpose.

The first part is done when we call `String::from`. The second part in some languages is don by GC. In languages where there is no GC, we are responsible to deallocate the memory ourselves when we do not need it anymore.

But in Rust, the memory is deallocated when the variables goes out of scope.

```
{
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
}                                  // this scope is now over, and s is no
                                       // longer valid
```

There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

## [Variables and data interacting with move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

```
let x = 5;
let y = x; // a copy of x is being make and assigned to y
```

This copying happens for primitive datatype.

```
 let s1 = String::from("hello");
 let s2 = s1;
```
To understand what's going on here, its better to read the full contents at [the original link](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

![String memory image](https://doc.rust-lang.org/book/img/trpl04-01.svg)


The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

When in the above example, when s1 is copied to s2, we only copy the contents of the stack and not the data of the heap.

![This is what happens](https://doc.rust-lang.org/book/img/trpl04-02.svg)

Now we can see that there are two variables which point to the same location in heap. Therefore when when both the variables go out of scope, they are gonna try to deallocate the same memory space twice. This will cause a memory corruption. This is also known as _double free error_

Due to this, rust won't consider s1 as valid once its location has already been copied. Due to this, this method is called _moving_ of a variable.

The way s1 is being copied wherein we are just creating another variables which points to the same memory address as s1, this method of copying is _shallow copying_

Rust will never automatically do a _deep_ copying of your data.


## [Variables and Data Interacting with Clone](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone)

We can do a `clone()` when we want to do deep copy.

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

```

## [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)

This section basically talks about integers who have a known size during compile time. These variables are always deep copied and also are copied on stack. There is a direct copy

>The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

Does this logic apply to every variable which is of known size?

Ans: 

You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

- All the integer types, such as u32.
- the Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

Rust has an annotation called `Copy` trait that we can place on types that are stored in the stack. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copy.

Rust won't let  use `Copy` if the type or any of its parts has also used `Drop`. 

## [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

Passing a variable in function will move or copy simply as assignment does.

```
fn main() {    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

//If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.
}
```
## Mutable references

1. If you have mutable reference to a value, you can have no other references to that value.

```
  let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```
 let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```
Users of an immutable reference don’t expect the value to suddenly change out from under them!

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

```
  let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

Also read about [Dangling references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

## [Slice type](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

 We can get part of a string using syntax like this

 ```
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
 ```

[Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

Defining

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.

```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

## [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-and-instantiating-structs)
** Function which returns a struct **

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

```
We can also use this shorthand

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

```

### Creating instances from other isntances with Struct update syntax
```rust
let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

### Creating tuple structs without named fields

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
### Unit-like Structs without Any Fields
``` rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

# Ownership of Struct Data

All the structs created until now owned their own data. To store data, which is owned by someone else, we need to use *lifetimes*. (We haven't read about it till now)
