# Project file structure
* `main.rs` - Contains the entry point of the CLI tool and some functions and structs related to the commands of this tool.
    ## Structs
    - `CliInit` - Contains the program global options and is the entry point of the subcommands.
    - `CliDemangle` - Options and values for the **demangle** subcommand.
    - `CliGet` - Options for the **demangle** subcommand.
    ## Enums
    - `CliCommand` - Subcommands of the CLi tool. To add another one just create a new option in this Enum.
    ## Functions
    - `main` - Entry point of the program.
    - `demangle_cmd` - Logic of the **demangle** command.
    - `get_cmd` - Logic of the **get** command.

* `ompi.rs` - Functions about mangling/demangling hardware threds.
    ## Constants
    - `CORE_NUMBER` - Number of cores of each socket.
    ## Functions
    - `mangle` - Mangles a thread, giving the value the cluster will give if that thread is specified. This function has no use, just for development porposes.
    - `demangle` - Demangles a thread, giving the value that has to be introduced in the cluster to get the specified thread on this function.
    - `show_cmd` - Prints the recommended **mpirun**  command for the input of the program.
    - `format_vector` - Transforms a vector of display capable items into a comma separated string of said elements.

* `smt.rs` - Functions and implementations for the smt::SMT enum.
    ## Enums
    - `SMT` - The SMT enum. Can be SMT2 or SMT4. Implements the Copy, Clone, PartialEq and Debug traits.
    ## Implementations
    - `impl SMT` - Implementation of smt::SMT enum. Contains:
        + `get_jump` - Returns the jump between sockets. Hand calculated. Nonsense.
        +  `get_extra_threads` - Returns the total thread amount a single socket has with a determined SMT.
    - `impl FromStr` - Allows the conversion from &str (String slice) to smt::SMT.

* `filter.rs` - Contains the filtering options for the **get** command.
    ## Enums
    - `Thread` - Thread position per core. First, Second, Thrid or Fourth.
    - `Socket` - Socket 0 or 1.
    - `CoreParity` - Odd or even.
    ## Implementation
    - `impl Thread` - Some functions about threads related to smt::SMT.
        + `coherent` - Checks if a thread is coherent with a given smt::SMT.
        + `index` - Returns a 0 based index of the given thread. First -> 0, Sencond  -> 1, ...
    ## Functions
    - `get_threads` - Applies the filtering options to the table of threads.
    - `generate_table` - Generates the table of demangle threads.
        