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



    


}
