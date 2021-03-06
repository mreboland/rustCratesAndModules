use std::usize;

fn main() {
    println!("Hello, world!");



    // Crates

    // Rust programs are made of crates. Each crate it a Rust project. All the source code for a single library or executable, plus any associated tests, examples, tool, configuration, and other junk.

    // Using the --verbose flag on cargo build will create a project that has some dependencies built in. This was done back in chapter 2, the Mandelbrot Program.

    // In that program, our main.rs contained three extern crate declarations:
    extern crate num;
    extern crate image;
    extern crate crossbeam;

    // These lines simply tell Rust that num, image, and crossbeam are external libraries, not part of the Mandelbrot program itself.

    // We also specified in our Cargo.toml file which version of each crate we wanted:
    [dependencies]
    num = "0.1.27"
    image = "0.6.1"
    crossbeam = "0.2.8"

    // The word dependencies just means other crates this project uses, code we're depending on. The crates were found on crates.io, Rust's community site for open source crates. For example, the image crate was found on the site by searching for an image library. The crates page provides links to documentation and source code, as well as a line of config like image = "0.6.1" that we can copy to our Cargo.toml. The numbers are the latest version as of the textbook writing.

    // The cargo transcript tells the story of how this info is used. When we run cargo build, Cargo starts by downloading source code for the specified versions of these crates from crates.io. Then, it reads those crates' Cargo.toml files, downloads their dependencies, and so on recursively. For example, the source code for vers 0.6.1 of the image crate contains a Cargo.toml file that includes:
    [dependencies]
    byteorder = "0.4.0"
    num = "0.1.27"
    enum_primitive = "0.1.0"
    glob = "0.2.10"

    // Seeing this, Cargo knows that before it can use image, it must fetch those crates as well. Cargo can also fetch source code from a Git repo or local filesystem which we'll cover later.

    // Once it has obtained all the source code, Cargo compiles all the crates. It runs rustc, the Rust compiler, once for each crate in the project's dependency graph. When compiling libraries, Cargo uses the --crate-type lib option. This tells rustc not to look for a main() function but instead to produce an .rlib file containing compiled code in a form that later rustc commands can use as input. When compiling a program, Cargo uses --crate-type bin, and the result is a binary executable for the target platform: mandelbrot.exe on Windows for example.

    // With each rustc command, Cargo passes --extern options giving the filename of each library the crate will use. That way, when rustc sees a line of code like extern crate crossbeam, it knows where to find that compiled crate on disk. The Rust compiler needs access to these .rlib files because they contain the compiled code of the library. Rust will statically link that code into the final executable. The .rlib also contains type info, so Rust can check that the library features we're using in our code actually exist in the crate, and that we're using them correctly. It also contains a copy of the crate's public inline functions, generics, and macros, features that can't be fully compiled to machine code until Rust sees how we use them.

    // cargo build supports all sorts of options, most of which are beyond this books scope. However one to mention is cargo build --release, it produces an optimized build. Release builds run faster, but they take longer to compile, they don't check for integer overflow, they skip debug_assert!() assertions, and the stack traces they generate on panic are generally less reliable.



    // Build Profiles

    // There are several config settings we can put in our Cargo.toml file that affect the rustc command lines that cargo generates.

    // Command line             Cargo.toml section used
    // cargo build              [profile.debug]
    // cargo build --release    [profile.release]
    // cargo test               [profile.test]

    // The defaults are usually fine, but one exception found is when you want to use a profiler. A tool that measures where our program is spending its CPU time. To get the best data from a profiler, we need both optimizations (usually enabled only in release builds) and debug symbols (usually enabled only in debug builds). To enable both, add the following line to our Cargo.toml:
    // [profile.release]
    // debug = true # enable debug symbols in release builds

    // The debug setting controls the -g option to rustc. With this config, when we type cargo build --release, we'll get a binary with debug symbols. The optimization settings are unaffected.

    // https://doc.rust-lang.org/cargo/reference/manifest.html
    // The above cargo doc lists many other settings we can adjust.



    // Modules

    // Modules are Rust's namespaces. They're containers for the functions, types, constants, and so on that make up our Rust program or library. Whereas crates are about code sharing between projects, modules are about code organization within a project. They look like this:
    mod spores {
        use cells::Cell;

        /// A cell made by and adult fern. It disperses on the wind as part of
        /// the fern life cycle. A spore grows into a prothallus - a whole
        /// separate organism, up to 5mm across -- which produces they zygote
        /// that grows into a new fern. (Plant sex is complicated).
        pub struct Spore {
            ...
        }

        /// Simulate the production of a spore by meiosis.
        pub fn produce_spore(factory: &mut Sporangium) -> Spore {
            ...
        }

        /// Mix genes to prepare for meiosis (part of interphase).
        fn recombine(parent: &mut Cell) {
            ...
        }

        ...
    }

    // A module is a collection of items, named features like the Spore struct and the two functions in the above example. The pub keyword makes an item public, so it can be accessed from outside the module. Anything that isn't marked pub is private.
    let s = spores::produce_spore(&mut factory); // ok
    spres::recombine(&mut cell); // error: `recombine` is private

    // Modules can next, and it's fairly common to see a module that's just a collection of submodules:
    mod plant_structures {
        pub mod roots {
            ...
        }

        pub mod stems {
            ...
        }

        pub mod leaves {
            ...
        }
    }

    // In this way, we could write out a whole program, with a huge amount of code and whole hierarchy of modules, all in a single source file. However working that way is difficult, so there's an alternative.



    // Modules in Separate Files

    // A module can also be written like this:
    mod spores;

    // Earlier, we included the body of the spores module, wrapped in curly braces. Here, we're instead telling the Rust compiler that the spores module lives in a separate file, called spores.rs:
    // spores.rs

    /// A cell made by an adult fern...
    pub struct Spore {
        ...
    }

    /// Simulate the production of a spore by meiosis.
    pub fn produce_spore(factory: &mut Sporangium) -> Spore {
        ...
    }

    /// Mix genes to prepare for meiosis (part of interphase).
    fn recombine(parent: &mut Cell) {
        ...
    }

    // spores.rs contains only the items that make up the module. It doesn't need any kind of boilerplate to declare that it's a module.

    // The location of the code is the only difference between this spores module and the version showed in the previous section (using the same code). The rules about what's public and what's private are exactly the same. Rust also never compiles modules separately, even if they're in separate files. When we build a Rust crate, we're recompiling all of its modules.

    // A module can have its own directory. When Rust sees mod spores, it checks for both spores.rs and spores/mod.rs. If neither files exists, or both exist, that's an error. For this example, we used spores.rs, because the spores module did not have any submodules. But consider the plant_structures module written out earlier. If we decide to split that module and its three submodules into their own files, the resulting project would look like this:
    // fern_sim/
        // Cargo.toml
        // src/
            // main.rs
            // spores.rs
            // plant_structures/
                // mod.rs
                // leaves.rs
                // roots.rs
                // stems.rs
    
    // In main.rs, we declare the plant_structures module:L
    pub mod plant_structures;

    // This causes Rust to load plant_structures/mod.rs, which declares the three submodules:
    // In plant_structures/mods.rs
    pub mod roots;
    pub mod stems;
    pub mod leaves;

    // The content of those three modules are stored in separate files names leaves.rs, roots.rs, and stems.rs, located alongside mod.rs in the plant_structures directory.



    // Paths and Imports

    // The :: operator is used to access features of a module. Code anywhere in our project can refer to any standard library feature by writing out its absolute path:
    if s1 > s2 {
        ::std::mem::swap(&mut s1, &mut s2);
    }

    // This function name, ::std::mem::swap, is an absolute path, because it starts with a double colon. The path ::std refers to the top-level module of the standard library. ::std::mem is a submodule within the standard library, and ::std::mem::swap is a public function in that module.

    // We could write all our code this way, spelling out ::std::f64::consts::Pl and ::std::collections::HashMap::new every time we want a circle or a dictionary, but it would be tedious to type and hard to read. The alternative is to import features into the modules where they're used:
    use std::mem;

    if s1 > s2 {
        mem::swap(&mut s1, &mut s2);
    }

    // The use declaration causes the name mem to be a local alias for ::std::mem throughout the enclosing block or module. Paths in use declarations are automatically absolute paths, so there is no need for a leading ::.

    // We could write use std::mem::swap to import the swap function itself instead of the mem module. However, what we did above is generally considered the best style. Import types, traits, and modules (like std::mem), then use relative paths to access the functions, constants, and other members within.

    // Several names can be imported at once:
    use std::collections::{HashMap, HashSet}; // import both
    use std::io::prelude::*; // import everything

    // The above is shorthand for writing our all the individual imports:
    use std::collections::HashMap;
    use std::collections::HashSet;

    // all the public items in std::io::prelude
    use std::io::prelude::Read;
    use std::io::prelude::Write;
    use std::io::prelude::BufRead;
    use std::io::prelude::Seek;

    // Modules do not automatically inherit names from their parent modules. For example, suppose we have this in our proteins/mods.rs:
    // proteins/mods.rs
    pub enum AminoAcid { ... }
    pub mod synthesis;

    // The code in synthesis.rs does not automatically see the type AminoAcid:
    // proteins/synthesis.rs
    pub fn synthesize(seq: &[AminoAcid]) // error: can't find type `AminoAcid`
    ...

    // Instead, each module starts with a blank slate and must import the names it uses:
    // proteins/synthesis.rs
    use super::AminoAcid; // explicitly import from parent

    pub fn synthesize(seq: &[AminoAcid]) // ok

    // The keyword super has a special meaning in imports. It's an alias for the parent module. Similarly, self is an alias for the current module.
    // In proteins/mod.rs

    // import from a submodule
    use self::synthesis::synthesize;

    // import names from an enum,
    // so we can write `Lys` for lysine, rather than `AminoAcid::Lys`
    use self::AminoAcid::*;

    // While paths in imports are treated as absolute paths by default, self and super let us override that and import from relative paths.

    // The AminoAcid example here breaks the style rule mentioned earlier about only importing types, traits, and modules. It can be broken if the naming gets extremely long (think DNA scientific name).

    // Submodules can access private items in their parent modules, but they have to import each one by name. Using super::* only imports items that are marked pub.

    // Modules aren't the same thing as files, but there is a natural analogy between modules and the files and directories of a Unix filesystem. The use keyword creates aliases, just as the In Command creates links. Paths, like filenames, come in absolute and relative forms. self and super are like the . and .. special directories. And extern crate grafts anther crate's root module into our project. It's a lot like mounting a filesystem.



    // The Standard Prelude

    // Previously we said each module starts with a "blank slate", as far as imported names are concerned. But the slate is not completely blank.

    // For one thing, the standard library std is automatically linked with every project. It's as though our lib.rs or main.rs contained an invisible declaration for it:
    extern crate std;

    // Furthermore, a few particularly handy names, like Vec and Result, are included in the standard prelude and automatically imported. Rust behaves as though every module, including the root module, started with the following import:
    use std::prelude::v1::*;

    // The standard prelude contains a few dozen commonly used traits and types. It does not contain std. So if our module refers to std, we'll have to import it explicitly, like so:
    use std;

    // Usually, it makes more sense to import the particular feature of std that we're using. In chapt 2, we mentioned that libraries sometimes provide modules names prelude. But std::prelude::v1 is the only prelude that is ever imported automatically. Naming a module prelude is just a convention that tells users it's meant to be imported using *.



    // Items, the Building Blocks of Rust

    // A module is made up of items. There are several kinds of item, and the list is really a list of the language's major features:

    // Functions
        // We've seen a lot of these already

    // Types
        // User-defined types are introduced using the struct, enum, and trait keywords. They will be covered in more detail in subsequent chapters. A simple struct looks like so:
        pub struct Fern {
            pub roots: RootSet,
            pub stems: StemSet
        }

        // A struct's fields, even private fields, are accessible throughout the module where the struct is declared. Outside the module, only public fields are accessible.

        // A single module can define several types that work closely together, such as perhaps frond::LeafMap and frond::LeafMapIter, accessing each other's private fields as needed, while still hiding those implementation details from the rest of our program.

    // Type aliases
        // As we've seen, the type keyword can be used like typedef in C++, to declare a new name for an existing type:
        type Table = HashMap<String, Vec<String>>;

        // The type Table that we're declaring here is shorthand for this particular kind of HashMap.
        fn show(table: &Table) {
            ...
        }

    // impl blocks
        // Methods are attached to types using impl blocks:
        impl Cell {
            pub fn distance_from_origin(&self) -> f64 {
                f64::hypot(self.x, self.y)
            }
        }

        // The syntax is covered in chapt 9. An impl block can't be marked pub. Instead, individual methods are marked pub to make them visible outside the current module.

        // Private methods, like private struct fields, are visible thoughout the module where they're declared.
    
    // Constants
        // The const keyword introduces a constant. The syntax is just like let except that it may be marked pub, and the type is required. Also, UPPERCASE_NAMES are conventional for constants:
        pub const ROOM_TEMPERATURE: f64 = 20.0; // degrees Celsius

        // The static keyword introduces a static item, which is nearly the same thing:
        pub static ROOM_TEMPERATURE: f64 = 68.0; // degrees Fahrenheit

        // A constant is a bit like a C++ #define. The value is compiled into our code every place it's used. A static is a variable that's set up before our program starts running and lasts until it exists. Use constant for magic numbers and string in our code. Use statics for larger amounts of data, or any time we'll need to borrow a ref to the constant value.

        // There are no mut constants. Statics can be marked mut, but as discussed in chapt 5, Rust has no way to enforce its rules about exclusive access on mut statics. They are, therefore, inherently non-thread-safe, and safe code can't use them at all:
        static mut PACKETS_SERVED: usize = 0;

        println!("{} served", PACKETS_SERVED); // error: use of mutable static

        // Rust discourages global mutable state. For a discussion of the alternatives, see "Global Variables" in chapt 19.

    // Modules
        // We've already covered this in details. As we've seen, a module can contain submodules, which can be public or private, like any other named item.

    // Imports
        // Use and extern crate declarations are items too. Even though tey're just aliases, they can by public:
        // in plant_structures/mods.rs
        ...
        pub use self::leaves::Leaf;
        pub use self::roots::Root;

        // This means that Leaf and Root are public items of the plant_structures module. They're still simple aliases for plant_structures::leaves::Leaf and plant_structures::roots:Root.
        // The standard prelude is written as just as a series of pub imports.
        
    // extern blocks
        // These declare a collection of functions written in some other language (typically C or C++), so that our Rust code can call them. These are covered in Chapt 21.

        // Rust warns about items that are declared, but never used:
        // warning: function is never used: `is_square`
        // --> src/crates_unused_items.rs...

        // This warning can be puzzling, because there are two very different possible causes. Perhaps this function really is dead code at the moment. Or, maybe, you meant to use it in other crates. In that case, we'd need to mark it and all enclosing modules as public.



    // Turning a Program into a Library

    // As our fern simulator starts to take off, we decide we need more than a single program. Suppose we've got one command-line program that runs the simulation and saves results in a file. Now, we want to write other programs for performing scientific analysis of the saved results, displaying 3D renderings of the growing plants in real time, rendering photorealistic pictures, and so on. ALl these programs need to share the basic fern simulation code. We need to make a library.

    // The first step is to factor our existing project into two parts. A library crate, which contains all the shared code, and an executable, which contains the code that's only needed for our existing command-line program.

    // To show how we can do this, let's use a grossly simplified example program:
    struct Fern {
        size: f64,
        growth_rate: f64
    }

    impl Fern {
        /// Simulate a fern growing for one day
        fn grow(&mut self) {
            self.size *= 1.0 + self.growth_rate;
        }
    }

    /// Run a fern simulation for some number of days.
    fn run_simulation(fern: &mut Fern, days: usize) {
        for _ in 0 .. days {
            fern.grow();
        }
    }

    fn main() {
        let mut fern = Fern {
            size: 1.0,
            growth_rate: 0.001
        };

        run_simulation(&mut fern, 1000);
        println!("final fern size: {}", fern.size);
    }

    // We'll assume that this program has a trivial Cargo.toml file:
    [package]
    name = "fern_sim"
    version = "0.1.0"
    authors = ["You <you@example.com>"]

    // Turning this program into a library is easy. The steps are:
    // 1. Rename the file src/main.rs to src/lib.rs
    // 2. Add the pub keyword to items in src/lib.rs that will be public features of our library.
    // 3. Move the main function to a temporary file somewhere. We'll expand on this in a minute.

    // The resulting src/lib.rs file looks like this:
    pub struct Fern {
        pub size: f64,
        pub growth_rate: f64
    }

    impl Fern {
        /// Simulate a fern growing for one day.
        pub fn grow(&mut self) {
            self.size *= 1.0 + self.growth_rate;
        }
    }

    /// Run a fern simulation for some number of days.
    pub fn run_simulation(fern: &mut Fern, days: usize) {
        for _ in 0 .. days {
            fern.grow();
        }
    }

    // We didn't need to change anything in Cargo.toml. This is because our minimal Cargo.toml files leaves Cargo to its default behaviour. By default, cargo build looks at the files in our source directory and figures out what to build. When it sees the file srd/lib.rs, it knows to build a library.

    // The code in src/lib.rs forms the root module of the library. Other crates that use our library can only access the public items of this root module.



    // The src/bin Directory

    // Getting the original command-line fern_sim program working again is also straightforward. Cargo has some built-in support for small programs that live in the same codebase as a library.

    // In fact, Cargo itself is written this way. The bulk of the code is in a Rust library. The cargo command-line program that we've been using throughout the book is a thin wrapper program that calls out to the library for all the heavy lifting. Both the library and the command-line program live in the same source repository.

    // We can put our program and ou library in the same codebase, too. Put this code into a file named src/bin/efern.rs:
    extern crate fern_sim;
    use fern_sim::{Fern, run_simulation};

    fn main() {
        let mut fern = Fern {
            size: 1.0,
            growth_rate: 0.001
        };

        run_simulation(&mut fern, 1000);
        println!("final fern size: {}", fern.size);
    }

    // The main function is the one we set aside earlier. We've added an extern crate declaration, since this program will use the fern_sim library crate, and we're importing Fern and run_simulation from the library.

    // Because we've put this file into src/bin, Cargo will compile both the fern_sim library and this program the next time we run cargo build. We can run the fern program using cargo run --bin efern.

    // We still didn't make any changes to Cargo.toml, because again, Cargo's default is to look at our source files and figure things out. It automatically treats .rs files in src/bin as extra programs to build.

    // Now that fern_sim is a library, we also have another option. We could have put this program in its own isolated project, in a completely separate directory, with its own Cargo.toml listing fern_sim as a dependency:
    [dependencies]
    fern_sim = { path = "../fern_sim"}

    // Perhaps that is what we'll do for other fern-simulating programs down the road. The src/bin directory is just right for a simple program like efern.



    // Attributes

    // Any item in a Rust program can be decorated with attributes. Attributes are Rust's catch-all syntax for writing misc instructions and advice to the compiler. For example, suppose we get this warning:
    // libgit2.rs: warning: type `git_revspec` should have a camel case name
        // such as `GitRevspec`, #[warn(non_camel_case_types)] on by default

    // But we chose this name for a reason, and you wish Rust would shut up about it. We can disable the warning by adding an #[allow] attribute on the type:
    #[allow(non_camel_case_types)]
    pub struct git_revspec {
        ...
    }

    // Conditional compilation is another feature that's writing using an attribute, the #[cfg] attribute:
    // Only include this module in the project if we're building for Android
    // #[cfg(target_os = "android")]
    // mod mobile;

    // The full syntax of #[cfg] is specified in the Rust Reference. See page 280-281 for the most commonly used options.

    // Occasionally, we need to micromanage the inline expansion of functions, an optimization that we're usually happy to leave to the compiler. We can use the #[inline] attribute for that:
    /// Adjust levels of ions etc. in two adjacent cells
    /// due to osmosis between them
    #[inline]
    fn do_osmosis(c1: &mut Cell, c2: &mut Cell) {
        ...
    }

    // There's one situation where inlining won't happen without #[inline]. When a function or method defined in one crate is called in another crate, Rust won't inline it unless it's generic (it has type parameters) or it's explicitly marked #[inline].

    // Otherwise, the compiler treats #[inline] as a suggestion. Rust also supports the more insistent #[inline(always)], to request that a function be expanded inline at every call site, and #[inline(never)], to ask that a function never be inlined.

    // Some attributes, like #[cfg] and #[allow], can be attached to a whole module and apply to everything in it. Others, like #[test] and #[inline], must be attached to individual items. Each attribute is custom-made and has its own set of supported arguments. Use the Rust Reference docs to see the full set of supported attributes.

    // To attach an attribute to a whole crate, add it at the top of the main.rs or lib.rs file, before any items, and write #! instead of #, like so:
    // libgit2_sys/lib.rs
    #![allow(non_camel_case_types)]

    pub struct git_revspec {
        ...
    }

    pub struct git_error {
        ...
    }

    // The #! tells Rust to attach an attribute to the enclosing item rather than whatever comes next. In this case, the #![allow] attribute attaches to the whole libgit2_sys crate, not just the struct git_revspec.

    // #! can also be used inside functions, structs, and so on, but it's only typically used at the beginning of a file, to attach an attribute to the whole module or crate. Some attributes always use the #! syntax because they can only be applied to a whole crate.

    // For example, the #![feature] attribute is used to turn on unstable features of the Rust language and libraries, features that are experimental, and therefore might have bugs or might be changed or removed in the future. For instance, as of writing, Rust has experimental support for 128-bit integer types i128 and u128. But since these types are experimental, we can only use them by 1. installing the Nightly version of Rust, and 2. explicitly declaring that our crate uses them:
    #![feature(i128_type)]

    fn main() {
        // Do my math homework, Rust!
        println!("{}", 9204093811595833589_u128 * 19973810893143440503_u128);
    }

    // Over time, the Rust team sometimes stabilizes an experimental feature, so that it becomes a standard part of the language. The #![feature] attribute then becomes superfluous, and Rust generates a warning advising us to remove it.



    // Tests and Documentation

    // Rust has a simple unit testing framework built in. Tests are ordinary functions marked with the #[test] attribute.
    #[test]
    fn math_works() {
        let x: i32 = 1;
        assert!(x.is_positive());
        assert_eq!(x + 1, 2);
    }

    // Running cargo test runs all the tests in our project.

    // This works the same whether our crate is an exe or library. We can run specific tests by passing arguments to Cargo. cargo test math runs all tests that contain math somewhere in their name.

    // The assert macros are commonly used. assert!(expr) succeeds if expr is true, otherwise it panics causing a fail. assert_eq!(v1, v2) is just like assert! (v1 == v2) except that if the assertion fails, the error message shows both values.
    // We can use the above macros in ordinary code, to check invariants, however they are included in release builds. Use debug_assert! and debug_assert_eq! instead to write assertions that are checked only in debug builds.

    // To test error cases, add the #[should_panic] attribute to our test:
    /// This test passes only if division by zero causes a panic
    /// as we claimed in the previous chapter.
    #[test]
    #[should_panic(expected="divide by zero")]
    fn test_divide_by_zero_error() {
        1 / 0; // should panic?
    }

    // Functions marked with #[test] are conditionally compile. When we run cargo test, Cargo builds a copy of our program with our tests and the test harness enable. A plain cargo build or cargo build --release skips the testing code. This means our unit tests can live right alongside the code they test, accessing internal implementation details if they need to, and yet there's no runtime cost. However, it can result in some warnings. For example:
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a -b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::Pl;
        assert!(roughly_equal(Pl.sin(), 0.0));
    }

    // In a testing build, this is fine. In a nontesting build, roughly_equal is unused, and Rust will complain:
    // Warning: function is never used: `roughly_equal` ...

    // Convention, when our tests get substantial enough to require support code, is to put them in a tests module and declare the whole module to be testing-only using the #[cfg] attribute:
    #[cfg(test)] // include this module only when testing
    mod tests {
        fn roughly_equal(a: f64, b: f64) -> bool {
            (a - b).abs() < 1e-6
        }

        #[test]
        fn trig_works() {
            use std::f64::consts::Pl;
            assert!(roughly_equal(Pl.sin(), 0.0));
        }
    }

    // Rust's test harness uses multiple threads to run several tests at a time, a nice side benefit of our Rust code being thread-safe by default. We can disable this, we can either run a single test, cargo test testname, or set the environment variable RUST_TEST_THREADS to 1.



    // Integration Tests

    // Our fern simulator continues to grow. We've decided to put all the major functionality into a library that can be used by multiple executables. It would be nice to have some tests that link with the library the way an end user would, using fern_sim.rlib as an external crate. Also, we have some tests that start by loading a saved simulation from a binary file, and it is awkward having those large test files in our src directory. Integration tests help with these two problems.

    // Integration tests are .rs files that live in a tests directory alongside our project's src directory. When we run cargo test, Cargo compile each integration test as a separate, standalone crate, linked with our library and the Rust test harness. Example:
    // tests/unfurl.rs - Fiddleheads unfurl in sunlight

    extern crate fern_sim;
    use fern_sim::Terrarium;
    use std::time::Duration;

    #[test]
    fn test_fiddlehead_unfurling() {
        let mut world = Terrarium::load("tests/unfurl_files/fiddlehead.tm");
        assert!(world.fern(0).is_furled());
        let one_hour = Duration::from_secs(60 * 60);
        world.apply_sunlight(one_hour);
        assert!(world.fern(0).is_fully_unfurled());
    }

    // The integration test includes an extern crate declaration since it uses fern_sim as a library. The point of integration tests is that they see our crate from the outside, just as a user would. They test the crate's public API.

    // cargo test runs both unit tests and integration tests. To run only the integration tests in a particular file, for example, tests/unfurl.rs, use the command cargo test --test unfurl.



    // Documentation

    // The command cargo doc creates HTML documentation for out library:
    // cargo doc -no-deps --open
    // Documenting fern_sim v0.1.0 (file:/// ...//fern_sim)

    // The --no-deps option tells Cargo to generate documentation only for fern_sim itself, and not for all the crates it depends on.

    // The --open option tells Cargo to open the documentation in our browser afterwards.

    // See page 288 for diagram. Cargo saves the new documentation files in target/doc. The starting page is target/doc/fern_sim/index.html.

    // The documentation is generated from the pub features of our library, plus any doc comments we've attached to them. We've seen a few doc comments in this chapter. They start with the /// tag which Rust treats as a #[doc] attribute. We could technically write it out as #[doc = "Simulate the production...."] which works out to be the same.

    // When we compile or test a library, these attributes are ignored. When we generate documentation, doc comments on public features are included in the output.

    // Likewise, comments starting with //! are treated as #![doc] attributes, and are attached to the enclosing feature, typically a module or crate. For example, our fern_sim/src/lib.rs file might begin like so:
    //! Simulate the growth...
    //! individual cells on up.

    // The content of a doc comment is treated ar Markdown, a shorthand notation for simple HTML formatting. Asterisks are used for *italics* and **bold type**, a blank line is treated as a paragraph break, and so on. However, we can also fall back on HTML. Any HTML tags in our doc comments are copied through verbatim into the documentation.

    // We can use `backticks` to set off bits of code in the middle of running text. In the output, these snippets will be formatted in a fixed-width font. Larger code samples can be added by indenting four spaces.

    /// A block of code in a doc comment
    ///
    ///    if everything().works() {
    ///         println!("ok");
    ///}     

    // We can also use Markdown fenced code blocks. This has the same effect.

    /// Another snippet, same code written differently
    ///
    /// ```
    /// if everything().works() {
    ///     println!("ok");
    ///}
    /// ```

    // Whichever format we use, an interesting thing happens when we include a block of code in a doc comment. Rust automatically turns it into a test.



    // Doc-Tests

    // When we run tests in a Rust library crate, Rust checks that all the code that appears in our documentation actually runs and works. It does this by taking each block of code that appears in a doc comment, compiling it as a separate executable crate, linking it with our library, and running it.

    // Below is a standalone example of a doc-test. Create a new project by running cargo new ranges and put the below code in ranges/src/lib.rs:

    use std::ops::Range;
    /// Return true if two ranges overlap.
    ///
    /// assert_eq!(ranges::overlap(0..7, 3..10), true);
    /// assert_eq!(ranges::overlap(1..5, 101..105), false);
    ///
    /// If either range is empty, they don't count as overlapping.
    ///
    /// assert_eq!(ranges::overlap(0..0, 0..10), false);
    ///
    pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end &&
        r1.start < r2.end && r2.start < r1.end
    }

    // The two small blocks of code in the doc comment appear in the documentation generated by cargo doc. See page 292 for a diagram.

    // They also become two separate tests (run cargo test)
    // running 2 tests
    // test overlap_0 ... ok
    // test overlap_1 ... ok

    // test result: ...

    // If we pass the --verbose flag to Cargo, we'll see that it's using rustdoc --test to run these two tests. Rustdoc stores each code sample in a separate file, adding a few lines of boilerplate code, to produce two programs. Here's the first:
    extern crate ranges;
    fn main() {
        assert_eq!(ranges::overlap(0..7, 3..10, true));
        assert_eq!(ranges::overlap(1..5, 101..105, false));
    }

    // The second:
    extern crate ranges;
    fn main() {
        assert_eq!(ranges::overlap(0..0, 0..10, false));
    }

    // The tests pass if these programs compile and run successfully.

    // In this case assertions make decent documentation. However, the idea behind doc-tests is not to put all our tests into comments. Rather, we write the best possible documentation, and Rust makes sure the code samples in our documentation actually compile and run.

    // Very often a minimal working example includes some details, such as imports or setup code, that are necessary to make the code compile, but just aren't important enough to show in the documentation. To hide a line of a code sample, put a # followed by a space at the beginning of that line:

    /// Let the sun shine in and run the simulation for a given
    /// amount of time.
    ///
    ///   # use fern_sim::Terrarium;
    ///   # use std::time::Duration;
    ///   # let mut tm = Terrarium::new();
    ///   tm.apply_sunlight(Duration::from_secs(60));
    ///
    pub fn apply_sunlight(&mut self, time: Duration) {
        ...
    }

    // Sometimes it's helpful to show a complete sample program in documentation, including a main function and an extern crate declaration. If those pieces of code appear in our code sample, we do not also want Rustdoc to add them automatically. The result wouldn't compile. Rustdoc therefore treats any code block containing the exact string fn main as a complete program, and doesn't add anything to it.

    // Testing can be disabled for specific blocks of code. To tell Rust to compile our example, but stop short of actually running it, we use a fenced code block with the no_run annotation:

    /// Upload all local terrariums to the online gallery.
    ///
    /// ```no_run
    /// let mut session = fern_sim::connect();
    /// session.upload_all();
    /// ```
    pub fn upload_all(&mut self) {
        ...
    }

    // If the code isn't even expected to compile, use ignore instead of no_run. If the code block isn't Rust code at all, use the name of the language, like c++ or sh, or text for plain text. Rustdoc treats any annotation it doesn't recognize as indicating that the code block isn't Rust. This disables code highlighting as well as doc-testing.



    // Specifying Dependencies

    // We've seen one way of telling Cargo where to get source code for crates our project depends on. By version number.
    image = "0.6.1"

    // There are several ways to specify dependencies, and some rather nuanced things we might want to say about which versions to use.

    // We may want to use dependencies that aren't published on crates.io at all. One way to do this is by specifying a Git repo URL and revision:
    image = { git = "https://github.com/piston/image.git", rev = "528f19c" }

    // This particular crate is open source, hosted on Github, but we could just as easily point to a private Git repo hosted on our corporate network. We can specify the particular rev, tag, or branch to use.

    // Another alternative is to specify a directory that contains the crate's source code:
    image = { path = "vendor/image" }

    // This is convenient when our team has a single version control repo that contains source code for several crates, or perhaps the entire dependency graph. Each crate can specify its dependencies using relative paths.

    // Having this level of control over our dependencies is powerful. If we ever decide that any of the open source crates being used isn't exactly to our liking, we can trivially for it. Hit the Fork button on GitHub and change one line in our Cargo.toml file. Our next cargo build will seamlessly use our fork of the crate instead of the official version.



    // Versions

    // When we write something like image = "0.6.1" in our Cargo.toml file, Cargo interprets this rather loosely. It uses the most recent version of image that is considered compatible with version 0.6.1.

    // The compatibility rules are adapted from Semantic Versioning.
    // 1. A version number starts with 0.0 is so raw that Cargo never assumes it's compatible with any other version.
    // 2. A version number that starts with 0.x, where x is nonzero, is considered compatible with other point releases in the 0.x series. We specified image version 0.6.1, but Cargo would use 0.6.3 if available.
    // 3. Once a project reaches 1.0, only new major versions break compatibility. So if we ask for version 2.0.1, Cargo might use 2.17.99 instead, but not 3.0.

    // Version numbers are flexible by default because otherwise the problem of which version to use would quickly become over constrained. Supposed one library, A, used num = "0.1.31", while B used num = "0.1.29". If versions required exact matches, no project would be able to use those two libraries together. Allowing Cargo to use any compatible version is a much more practical default.

    // We can specify an exact version or range of version by using operators:
    // Cargo.toml line             Meaning
    // image = "=0.10.0"           Use only the exact version 0.10.0
    // image = ">=1.0.5"           Use 1.0.5 or any higher version (even 2.9, if it???s available)
    // image = ">1.0.5 <1.1.9"     Use a version that???s higher than 1.0.5, but lower than 1.1.9
    // image = "<=2.7.10"          Use any version up to 2.7.10

    // Another version specification we'll occasionally see is the wildcard *. This tells Cargo that any version will do. Unless some other Cargo.toml file contains a more specific constraint, Cargo will use the latest available version.

    // The compatibility rules mean that version numbers can't be chosen purely for marketing reasons. They actually mean something. They're a contract between a crate's maintainers and its users. If we maintain a crate that's at version 1.7, an we decide to remove a function or make any other change that isn't fully backward compatible, we must bump our version number to 2.0. Calling it 1.8 would be claiming it works with 1.7 when it doesn't, and our users will have broken builds.



    // Cargo.lock

    // The version numbers in Cargo.toml are deliberately flexible, yet we don't want Cargo to upgrade us to the latest library versions every time we build. A cargo build that upgrades us to a new version of a library could have bad consequences say, during debugging which we do not want any changes being made. The unexpected change could cause more bugs.

    // Cargo has a built-in mechanism to prevent this. The first time we build a project, Cargo outputs a Cargo.lock file that records the exact version of every crate it used. Later builds will consult this file and continue to use the same versions. Cargo upgrades to newer versions only when we tell it to. Either by manually bumping up the version number in our Cargo.toml file, or by running cargo update.

    // cargo update only upgrades to the latest version that are compatible with what we've specified in Cargo.toml. If we have vers "0.6.1" and want "0.10.0" we'd have to manually change that in Cargo.toml. The next time we build, Cargo will update to the new version.

    // Something similar happens for dependencies that are stored in Git. Suppose our Cargo.toml file contains:
    image = { git = "https://github.com/Piston/image.git", branch = "master" }

    // cargo build will not pull new changes from the Git repo if it sees that we've got a Cargo.lock file. Instead, it eads Cargo.lock and uses the same revision as last time. But cargo update will pull from master, so that our next build uses the latest revision.

    // Cargo.lock is auto generated for us, and we'd normally not edit it by hand. If our project is an exe, we should commit Cargo.lock to version control. That way, everyone who builds our project will consistently get the same versions. The history of our Cargo.lock file will record our dependency updates.

    // If our project is an ordinary Rust library, don't bother committing Cargo.lock. Downstream users will have Cargo.lock files that contain version info for their entire dependency graph. They will ignore our library's Cargo.lock file. In the rare case that our project is a shared library (i.e., the ouput is a .dll, .dylib, or .so file), there is no such downstream cargo user, and we should therefore commit Cargo.lock



    // Publishing Crates to crates.io

    // We've decided to publish our fern-simulating library as open source software. Congrats! This part is easy.

    // First, make sure Cargo can pack the crate for us:
    // cargo package

    // The cargo package command creates a file containing all our library's source files, including Cargo.toml. This is the file that we'll upload to crates.io to share with the world (we can use cargo package --list to see which files are included). Cargo then double-checks its work by building our library from the .crate file, just as our eventual users will.

    // Cargo warns that the [package] section of Cargo.toml is missing some info that will be important to downstream users, such as the license under which we're distributing the code. The URL in the warning is an excellent resource, so we won't explain all the fields in detail. In short, we can fix the warnings by adding a few lines to Cargo.toml:
    [package]
    name = "fern_sim"
    version = "0.1.0"
    authors = ["You <you@example.com>"]
    license = "MIT"
    homepage = "https://fernsim.example.com/"
    repository = "https://gitlair.com/sporeador/fern_sim"
    documentation = "http://fernsim.example.com/docs"
    description = """
    Fern simulation, from the cellular level up.
    """

    // Of note, everyone who downloads our crate from crate.io can see the Cargo.toml. Leave personally info out of it.

    // another problem that sometimes arises at this stage is that our Cargo.toml file might be specifying the location of other crates by path:
    image = { path = "vendor/image" }

    // This may work fine for local production, but when others download it, they won't have the same files and directories we do. Cargo therefore ignores the path key in automatically downloaded libraries. This can cause build errors. The fix is straightforward. If our library is going to be published on crates.io, its dependencies should be on crates.io too. Specify a version number instead of a pth:
    image = "0.6.1"

    // If preferred, we can specify both a path, which takes precedence for our own local builds, and a version for all other users:
    image = { path = "vendor/image", version = "0.6.1"}

    // If we got the above route of both ways, it's now our responsibility that the two stay in sync.

    // Lastly, before publishing a crate, we'll need to log in to crates.io and get an API key. This step is straightforward. Once we have an account on crates.io, our "Account Settings" page will show a cargo login command, like:
    cargo login 5j0dV54BjlXBpUUbfIj7G9DvNl1vsWW1

    // Cargo saves the key in a config file, and the API key should be kept secret, like a password. So run this command only on a computer we control.

    // That done, the final step is to run cargo publish:
    // cargo publish

    // With this, our library joins thousands of others on crates.io.
    


    // Workspaces

    // As our project continues to grow, we end up writing many crates. They live side by side in a single source repo:
    fernsoft/
    ????????? .git/...
    ????????? fern_sim/
    ??? ????????? Cargo.toml
    ??? ????????? Cargo.lock
    ??? ????????? src/...
    ??? ????????? target/...
    ????????? fern_img/
    ??? ????????? Cargo.toml
    ??? ????????? Cargo.lock
    ??? ????????? src/...
    ??? ????????? target/...
    ????????? fern_video/
      ????????? Cargo.toml
      ????????? Cargo.lock
      ????????? src/...
      ????????? target/...

    // The way Cargo works, each crate has its own build directory, target which contains a separate build of all that crate's dependencies. These build directories are completely independent. Even if two crates have a common dependency, they can't share any compile code. This is wasteful.

    // We can save compilation time and disk space by using a Cargo workspace, a collection of crates that share a common build directory and Cargo.lock file.

    // All we need to do is create a Cargo.toml file in our repo's root directory and put these lines in:
    [workspace]
    members = ["fern_sim", "fern_img", "fern_video"]

    // Where fern_sim etc. are the names of the subdirectories containing our crates. Delete any leftover Cargo.lock files and target directories that exist in those subdirectories.

    // Once done, cargo build in any crate will automatically create and use a shared build directory under the root directory (in this case, fernsoft/target). The command cargo build --all builds all crates in the current workspace. cargo test and cargo doc accept the --all option as well.


}
