# Tutorial_NEAR_Rust

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/)

A step-by-step course for learning Smart Contract development using Rust. In this set of lessons, we will discuss the main features of the Rust language, 
as well as going over the NEAR platform. 

---

## Contact

[top](#tutorial_near_rust)

For any questions, complaints or suggestions, look me up on Discord On0n0k1#3800. If this course makes your life easier, feel free to buy me a cup of coffe by sending a bit of $NEAR to stiltztinkerstein.near . Thank you!

---

## Topics

[top](#tutorial_near_rust)

 - [What is the Rust language](#what-is-the-rust-language)
 - [Using Rust](#using-rust)
 - [Learning Rust](#learning-rust)
 - [Comparing Rust to Javascript and Python](#comparing-rust-to-javascript-and-python)
 - [Installing](#installing)
 - [Lessons](#lessons)

---

## What is the Rust language

[topo](#tutorial_near_rust)

In short, Rust is a low-level systems programming language with the following features:

 - Execution time in the likes of C or C++.
 - Doesn't have the risk involved with memory management as other low-level languages.
 - It has a steep learning curve (but is is well worth it!).
 - Doesn't have and doesn't need garbage collection. At compile time, the compiler can determine when variables are created and freed. 
 - Easy to do parallel programming. 
 - Async programming has about the same level of difficult as other popular languages.
 - Project and Dependency management is way easier than Python or JavaScript.
 - Has been, for years, the most loved language by the developer community!

---

## Using Rust

[top](#tutorial_near_rust)

A Rust developer can:

 - Create web3 applications using decentralized platforms such as NEAR.
 - Create applications that don't need a virtual machine to run them. Only the compiler is needed to produce an executable.
 - Create fast and compact server software hosted in Docker containers.
 - Create robust applications like Lambda functions that can be hosted on AWS (better web3 performance).
 - Create dynamic libs that can used from C.
 - Compile modules to WebAssembly, which can then be imported in a browser that supports javascript or a runtime (like node.js).
 - Compile robust and fast modules for Python using the PyO3 crate.
 - Compile code that target Embedded systems.
 - Have an edge in the job market where there's very few Rust developers worlwide.

---

## Learning Rust
[top](#tutorial_near_rust)

In my opinion, learning Rust is similar to taming a dragon in a fantasy-world. It is slow, difficult, with a lot of different, and simpler, alternatives. 
But if you do tame it, you will have a most powerful dragon by your side.

Studies show that it takes as much as 30 times more to write code in a low-level language (like C), than in a higher lever language (like Python or JavaScript). 
It is my experience that for a newcomer learning Rust, it is even slower than writing C. 

But, with practice, we get better at everything. With time, we learn what the compiler expects from us. We can leverage code snippets to generate "boilerplate" code automatically. Then, for the developer, everything just becomes a matter of understanding, memory and patience. There were times when I wrote 800 lines of Rust code
in just 2 days. 

We must always take a break to assess our process and make sure we are making the right calls. if we do, every future step will be easier than the one that came before it. 

---

## Comparing Rust to Javascript and Python

[top](#tutorial_near_rust)

A clever person might ask: "Why would I learn a difficult language if I can already solve the same problems in another language I already know?"... and that is a good question! if I already get good results writing a few lines of code in Python, why would I learn Rust? 

Easy of use and quick problem solving: that is the main focus of those languages. How to get to a solution for our problem the most easiest way. 
Processors were getting faster with each generation, so you could just buy new hardware that was faster and better and increase performance. 

But Moore's Law doesn't apply anymore. So, developers are needing better and more efficient algorithms. This need makes them take a closer look at the code and ask
"What is this instruction doing, exactly?". 

When we write ```"a = 3"``` in Python, a virtual machine is creating a "number" object, which entails creating a pointer to that number and then associating variable ```a``` to that pointer. That's why Python is generally limited to one processor core. When we want to take advantage of larger processing power on our machine, the code complexity increases exponentially in both JavaScript and Python.

Rust's focus isn't about the final result of that execution. It is about the path the processor and memory take to reach that result. An experienced Rust developer can take a look at a block of code and say: 
 - "That memory is going to be freed at this point in code";
 - "The processor is going to request freeing memory here and create a shallow copy of that variable here.";
 - "This function is going to borrow this address, use the value in this part of the code, and then give back that address to the owner.";

---

## Installing

[top](#tutorial_near_rust)

You need to install the folloowing before starting the lessons:

 - Install [near-cli](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/PT-BR/static/tutorials/setup-nearcli.md) to interact with the NEAR platform.
 - Install [rust](https://github.com/On0n0k1/Tutorial_NEAR_Rust/blob/main/PT-BR/static/tutorials/rust.md) to be able to compile and test projects.

---

## Lessons

[top](#tutorial_near_rust)

 - [Lesson 1: Smart Contracts](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_1_contract)
 - [Lesson 2: Ownership](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_2_ownership)
 - [Lesson 3: Structs](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_3_structs)
 - [Lesson 4: Modules](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_4_modules)
 - [Lesson 5: Using Macros](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_5_macro_usage)
 - [Lesson 6: Enums](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums)
 - Lesson 7: Traits
 - Lesson 8: Collections
