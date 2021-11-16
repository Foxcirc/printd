
use printd::printd as dbgf;

#[derive(Debug)]
struct Bar {
    eggs: Vec<usize>,
}

impl Bar {
    pub(crate) fn new(count: usize) -> Self {
        Self { eggs: vec![69; count] }
    }
}

#[test]
fn printd() {
    
    let foo = String::from("Hello world!\n");
    let bar = Bar::new(4);

    dbgf!("Ma boys: ", foo, bar);
    dbgf!(1 + 2, vec![69, 69 * 2]);

    dbgf!("Bye!");

}
