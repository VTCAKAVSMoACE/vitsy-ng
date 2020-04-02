use super::parse::Operation::*;
use super::*;
use std::ops::Deref;

#[test]
fn quine() -> Result<(), Box<dyn Error>> {
    let program = parse("../examples/quine.vt".to_string())?;
    let expected = Program {
        methods: vec![Method {
            ops: vec![
                SingleQuote,
                Reverse,
                Value(0xd),
                Value(0x3),
                Multiply,
                OutputAll,
            ],
        }],
        extends: vec![],
        uses: vec![],
    };

    assert_eq!(program.borrow().deref(), &expected);

    Ok(())
}
