#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic] // expecting panic
    fn it_fails() {
        panic!("test failed -- test case not passed");
    }

    #[test]
    fn call_simple_add() {
        assert!(simple_add())
    }
}

// use 'cargo test' command to execute tests
// use 'cargo test call_simple_add' to execute a particular test
// use 'cargo test it' to run tests starting from it

fn simple_add() -> bool {
    if 2 + 2 == 4 {
        true
    } else {
        false
    }
}
