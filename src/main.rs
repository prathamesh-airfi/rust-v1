#[allow(unused_variables)]

const TAX_RATE: f32 = 2.55;

fn main() {
    /*
     Variable Scope:
         A scope is the boundary or region of code where a name is valid.
    */

    let name = "Prathamesh";

    println!("{name}");

    {
        println!("{name}, {TAX_RATE}");
    }

    /* 
        TYPE ALIAS
            Is an alternate name that we can asssign to an existing type. 
    */

    type METER = u32;
    let mile_race_length: METER = 1600;
    println!("length of the race is {mile_race_length}");

    /*
        COMPILER DIRECTIVES
            A compiler directives is an annotation that tells the compiler 
            how to parse the code.

            #[fun(directive)] - for a single line
            #![fun(directive)] - for entire file

            e.g #![crate_name = "name"]     - Sets the name of the crate (useful for libraries).
            e.g #![crate_type = "type"]     - Specifies whether the crate is a bin (executable) or lib (library).
            e.g #![deny(warnings)]          - Converts warnings into errors.
            e.g #![allow(unused_variables)] - Suppresses warnings for unused variables. 
            e.g #![warn(missing_docs)]      - Issues warnings if public items lack documentation.
            e.g #![allow(dead_code)]        - No warnings

            e.g #[cfg(condition)]           - Compiles code only if the given condition is met.
                ```
                    #[cfg(target_os = "linux")]
                    fn platform_specific() {
                        println!("Running on Linux!");
                    }
                ```

            e.g #[cfg(feature = "my_feature")] - Enables code only if a Cargo feature is enabled.
                ```
                    #[cfg(feature = "experimental")]
                    fn experimental_feature() {
                        println!("Experimental feature enabled!");
                    }
                ```

            e.g #[derive(Trait)] - Automatically implements common traits like Debug, Clone, PartialEq, etc.
                ```
                    #[derive(Debug, Clone, PartialEq)]
                    struct Person {
                        name: String,
                        age: u32,
                    }
                ```
        
    */
    #[allow(unused_variables)]
    let a = 10;

}
