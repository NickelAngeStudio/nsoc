#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]

/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nsoc

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Nifty and Simple Overridable Constant provides neat macros to create constants that can be overriden compilation. 
//! 
//! It is based on [Lukas Kalbertodt](https://stackoverflow.com/users/2408867/lukas-kalbertodt) answer on [How can I override a constant via a compiler option?](https://stackoverflow.com/questions/37526598/how-can-i-override-a-constant-via-a-compiler-option/37526735#37526735).
//! 

#[macro_export]
/// Used in `build.rs` to quickly write constants in a file generated in the `OUT_DIR`.
/// 
/// # Usage
/// create_const_file! { {$filename,} $filehandle, $code}
/// 
/// - `$filename` *`Optional`* 
///     - Name of the file that will be created in the `OUT_DIR`. 
///     - The file `.rs` will be added to the end if not specified. 
///     - If not supplied, default filename is `{CARGO_PKG_NAME}_nsdcs.rs`.
/// - `$filehandle` 
///     - Variable used as file handle. 
///     - Must be provided to comply to [Rust macro hygiene](https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html).
/// - `$code` 
///     - Section where you write the [write_const!] macro.
/// 
/// # Example
/// In `build.rs` main
/// ```
/// fn main() {
///     write_file!{ f,
///         write_const!(f, DEFAULT_WIDTH, usize, 150, "Default frame width");
///         write_const!(f, DEFAULT_HEIGHT, usize, 50, "Default frame height")
///     }
/// }
/// ```
/// 
macro_rules! write_file {

    // Without filename supplied. Create default filename {CARGO_PKG_NAME}_nsdcs.rs.
    ($filehandle : expr, $code : block) => {
        create_const_file!(format!("{}_nsdcs.rs", env!("CARGO_PKG_NAME")), $filehandle, $code);
    };
    
    // With filename supplied
    ($filename : literal, $filehandle : expr, $code : block) => {
        // File destination
        let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(format!("{}.rs", $filename));

        // Create file
        let mut f = std::fs::File::create(&out_path).expect("Could not create file {}!", $filename);

        // Insert code block
        $code
    };
}

#[macro_export]
/// Write dynamic constants with optional comment and modifiers.
/// 
/// All contants have `pub` [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) unless specified via modifier.
/// 
/// # Usage
/// write_const! { {$modifiers,} $filehandle, $const_name, $const_type, $default {,$comment} }
/// 
/// - `$modifiers` *`Optional`* Literal used to modify certain parameters.  
///     - `nodoc` : No documentation will be generated for this constants.
///     - `cc` : Overwrite the default comment with a custom one.
///     - `priv` : Make contant visibility private.
///     - `pcrate` : Make contant visibility `pub (crate)`. 
///     - `pself` : Make contant visibility `pub (self)`. 
///     - `psuper` : Make contant visibility `pub (super)`. 
/// - `$filehandle` Filehandle specified in [create_const_file] macro.
/// - `$const_name` Name of the contant. Should be formatted as `SCREAMING_SNAKE_CASE` as specified in the [naming guideline](https://rust-lang.github.io/api-guidelines/naming.html).
/// 
/// 

macro_rules! write_const {
    // Call with no comments and no documentation
    ($filehandle : expr, $const_name : expr, $const_type : ty, $default : expr) => {
        write_const!("nodoc", $filehandle, $const_name, $const_type, $default, "")
    };

    // Call with no modifiers
    ($filehandle : expr, $const_name : expr, $const_type : ty, $default : expr, $comment : literal) => {
        write_const!("", $filehandle, $const_name, $const_type, $default, $comment)
    };

    // Full call which is usually not used directly.
    ($modifiers : literal, $filehandle : expr, $const_name : expr, $const_type : ty, $default : expr, $comment : literal) => {
        let const_value =  match option_env!(std::stringify!($const_name)){  // Try to get cargo argument specified
            Some(env_var) => {
                match env_var.parse::<$const_type>() {    // Parse cargo argument variable according to type.
                    Ok(value) => value, // Return value if valid
                    Err(err) => panic!("{}", err),  // Panic if invalid
                }
            },
            None => $default,   // Returns default value if not supplied in cargo arguments
        };
    
        println!("{}\n pub const {}: {} = {};\n", $comment, std::stringify!($const_name), std::stringify!($const_type), const_value);
        //std::write!(&mut $file, "{}\n {} const {}: {} = {};\n", $comment, $visibility, std::stringify!($const_name), std::stringify!($const_type), const_value)
        //    .expect("Could not write file `nslog_contants.rs`");
        println!("cargo:rerun-if-env-changed={}", std::stringify!($const_name));
    };

}


#[macro_export]
/// Add dynamic constants into your code.
macro_rules! load_const {
    () => {

    };
    ($filename : literal) => {

    }
}


#[cfg(test)]
mod tests {
    use super::write_const;

    #[test]
    fn it_works() {
        write_const!(f, DEFAULT_WIDTH, usize, 50);
    }
}
/*
#[macro_export]
macro_rules! write_const {

}

#[macro_export]
macro_rules! write_const_no_comment {

}


 write_env_var_to_file!( "NSLOG_ENTRY_COUNT", f, DEFAULT_ENTRY_COUNT, "
    /// Maximum count of entry the log can contain at once.
    ///
    /// # Default
    /// 50
    ///
    /// The default value can be overwritten when compiling or running cargo.
    /// 
    /// # Example
    /// This will set the log count to 100.
    /// ```
    /// NSLOG_ENTRY_COUNT=100 cargo run
    /// ```
    /// ");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/