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



    



}
