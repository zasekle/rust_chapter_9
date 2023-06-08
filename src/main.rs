use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;

fn main() {

    //Rust features two types of errors. Recoverable and unrecoverable. panic! is the unrecoverable
    // error, however doing things like accessing beyond the end of an array is as well. Recoverable
    // errors are errors that take advantage of thing such as Option and Result.
    // unrecoverable_errors_with_panic();
    // recoverable_errors_with_result();
    to_panic_or_not_to_panic();
}

fn unrecoverable_errors_with_panic() {
    //A manual call to panic.
    // panic!("Unrecoverable error");

    let v = vec!['a', 'b', 'c'];

    //A panic caused by a bug in the code. Note that in C reading past the end of an array returns
    // undefined results. This can cause security vulnerabilities because it allows people to read
    // data they do not have access to.
    //Set the environment variable RUST_BACKTRACE=full to get the stacktrace.
    v[50];
}

fn result_return(err: bool) -> Result<isize, String> {
    return if err {
        Err(String::from("oopsie"))
    } else {
        Ok(123)
    }
}

fn propagate_error() -> Result<String, io::Error> {
    let mut open_file = File::open("hello.txt")?;
    Ok(String::from("hello"))
}

fn recoverable_errors_with_result() {
    //Result is a value that is meant to be returned in the case of a recoverable error.
    let result = result_return(true);

    match result {
        Ok(val) => println!("Ok returned {val}"),
        Err(val) => println!("Err returned {val}"),
    }

    //File open has an interesting structure, it has errors nested inside the Err return type. The
    // nested type is type ErrorKind. Not all values will be used, certain ones seem to be used in
    // certain situations. After that, `_` needs to be used to get the other results.
    // let no_file = File::open("the file");
    //
    // let the_file = match no_file {
    //     Ok(file) => file,
    //     Err(val) => match val.kind() {
    //         ErrorKind::NotFound => {
    //             panic!("file not found");
    //         }
    //         other => {
    //             panic!("problem opening {:?}", other)
    //         }
    //     }
    // };

    //Expect and unwrap can be used to panic if the result is Err. The only difference is that
    // expect allows a custom error message to be entered.
    // let v = vec![1,2,3];
    //
    // let value = v.get(14).unwrap();
    // let value = v.get(14).expect("failed to get index");

    //There is a shortcut for propagating errors in Rust, it is the ? operator. This is because it
    // is such a common thing to do. The `?` operator can also be chained just like in Kotlin.
    // For example,
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // The ? operator can also be used with `Option`.
    let result = propagate_error();

    //As an (seemingly odd) note, main can also return a `Result` type.
}

fn to_panic_or_not_to_panic() {
    //Basically this section covers when to return a Result or to panic! in a library. They lay out
    // some pretty good rules, not sure its worth memorizing though.
}