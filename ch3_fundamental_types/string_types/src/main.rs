/*
    Byte string can't contain arbitrary Unicode characters. They must make do with
    ASCII and \xHH escape sequences.
*/

#[test]
fn test_byte_strings() {
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T'])
}

#[test]
fn test_vec_str() {
    let bits = vec!["veni", "vidi", "vici"];

    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(","), "veni,vidi,vici");

}

#[test]
fn test_str_functions() {
    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));

    assert_eq!("    clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}



fn str_literals() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);
    println!("In the room the women can come and go, Singing of Mount Abora");

    // If you need to write strings in multiple lines
    println!("It was a bright, cold day in April, and \
            there was four of us~\
            more or less");
}


/*
    String in Memory
    
    In stack-frame, there contains
    
    Fat Pointer for Strings
    - The pointer address
    - The capacity
    - The length

    The actual contents of the string are located at the heap memory

*/

fn str_in_memory() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    println!("{}", &noodles);
    println!("{}", &oodles);
    println!("{}", &poodles);

    println!(""); 
    println!("noodles.capacity() = {}", &noodles.capacity());
    println!("oodles.capacity() = {}", &oodles.to_string().capacity());
    println!("poodles.capacity() = {}", &poodles.to_string().capacity());

    println!(""); 
    println!("noodles.len() = {}", &noodles.len());
    println!("oodles.len() = {}", &oodles.len());
    println!("poodles.len() = {}", &poodles.len());
    
    println!(""); 
    println!("noodles.as_ptr() = {:p}", &noodles.as_ptr());
    println!("oodles.as_ptr() = {:p}", &oodles.as_ptr());
    println!("poodles.as_ptr() = {:p}", &poodles.as_ptr());
}


/*
    - &str is very much like &[T] (Slice): a fat pointer to some data.
    - String is analogous to Vec<T>
*/



fn raw_strings() {

    // Let's you parse backslash as it is (string tagged with lower case r)
    // No Escape Sequences needed
    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("Printing raw strings v1  = {}", default_win_install_path);
    
    // Another raw string format
    println!("{}", r###"
            This is the raw string started with 'r####'.
            Therefore it does not end until we reach a quote mark ('"')
            followed immediately by three pound signs ('###')
            "###);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/*
    Other String-Like Types
    - Rust guarantees that strings are valid UTF-8.
    - Sometimes a program needs to be able to deal with strings that are not valid Unicode.

    Rust's solution is to offer a few string-like types for these situations
    - Stick to String and &str for Unicode text.
    - When working with filenames, use std::path::PathBuf and &Path instead.
    - When working with binary data that isn't UTF-8 encoded, use Vec<u8> and &[u8]
    - When working with environment variable names and command-line arguments in the
      native form presented by the operating system, use OsString and &OsStr
    - When interoperating with C libraries that use null-terminated strings, use
      std::ffi::CString and &CStr

*/

/*
    Using Type Aliases
*/

fn type_aliases() {
    type Bytes = Vec<u8>;
    let v: Bytes = vec![0,1,2,3,4,5];

    println!("Data As Bytes = {:?}", &v);
}


fn main() {
    str_literals();
    raw_strings();

    str_in_memory();

    println!("");
    println!("Type of strings");

    print_type_of(&"hello this is conventional string.");
    print_type_of(&"hello this is String itself".to_string());

    println!("");
    type_aliases();

}
