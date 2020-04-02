use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

fn enriched_parse(name: String) -> Result<Rc<RefCell<Program>>, Box<dyn Error>> {
    let mut to_explore = Vec::<String>::new();
    to_explore.push(name.clone());
    let mut program_graph = HashMap::new();

    while let Some(target) = to_explore.pop() {
        match program_graph.entry(target) {
            Entry::Vacant(k) => {
                let program = program_parser::program(&File::open(Path::new(k.key())).and_then(
                    |mut f| {
                        let mut contents = String::new();
                        match f.read_to_string(&mut contents) {
                            Ok(_) => Ok(contents),
                            Err(e) => Err(e),
                        }
                    },
                )?)?;

                program
                    .extends
                    .iter()
                    .for_each(|e| to_explore.push(e.target.clone()));
                program
                    .uses
                    .iter()
                    .for_each(|u| to_explore.push(u.target.clone()));

                let ret = Rc::new(RefCell::new(Program {
                    methods: program.methods,
                    extends: vec![],
                    uses: vec![],
                }));

                let program_node = (
                    ret,
                    program
                        .uses
                        .iter()
                        .map(|u| u.target.clone())
                        .collect::<Vec<String>>(),
                    program
                        .extends
                        .iter()
                        .map(|e| e.target.clone())
                        .collect::<Vec<String>>(),
                );
                k.insert(program_node);
            }
            _ => {}
        }
    }

    Ok(program_graph.get(&name).unwrap().0.clone())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub methods: Vec<Method>,
    pub extends: Vec<Box<Program>>,
    pub uses: Vec<Box<Program>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserProgram {
    pub methods: Vec<Method>,
    pub extends: Vec<Extend>,
    pub uses: Vec<Use>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Method {
    pub ops: Vec<Operation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extend {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Use {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    IfNot,
    If,
    BeginBlock,
    EndBlock,
    Repeat,
    End,
    Exit,
    Teleport,
    MoveLeft,
    MoveRight,
    DoubleQuote,
    SingleQuote,
    CallMethod,
    Input,
    InputLength,
    Length,
    GetAllInput,
    OutputAll,
    OutputChar,
    OutputNumeric,
    GetLine,
    Switch,
    MultiSwitch,
    RotateRight,
    RotateLeft,
    Reverse,
    Part,
    Duplicate,
    Remove,
    Object,
    NewStack,
    RemoveStack,
    CloneStack,
    RightStack,
    LeftStack,
    Flatten,
    StackCount,
    TemporaryVariable,
    FinalVariable,
    Sine,
    Arcsine,
    Tangent,
    Arctangent,
    Cosine,
    Arccosine,
    Pi,
    E,
    Log,
    Random,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Equals,
    Modulo,
    Truncate,
    Factorize,
    Prime,
    Range,
    CallProgramMethod,
    CallShortProgramMethod,
    UseCount,
    UseName,
    Wait,
    File,
    Write,
    Shell,
    Eval,
    Value(u8),
}

peg::parser! {
    grammar program_parser() for str {
        use Operation::*;

        pub rule program() -> ParserProgram
            = methods:(m:method() ** "\n" { m })
              extends:("\n" e:extend_decl() { e })*
              uses:("\n" u:use_decl() { u })* "\n"*
            {
                ParserProgram {
                    methods,
                    extends,
                    uses
                }
            }

        rule method() -> Method
            = ops:op()+ { Method { ops } }

        rule extend_decl() -> Extend
            = ";e " f:filename() { unimplemented!() }

        rule use_decl() -> Use
            = ";u " f:filename() { unimplemented!() }

        rule filename() -> String
            = name:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '.' | '-' | '_' | std::path::MAIN_SEPARATOR]+) { name.to_string() }

        rule op() -> Operation
            = ")" { IfNot }
            / "(" { If }
            / "[" { BeginBlock }
            / "]" { EndBlock }
            / "\\" { Repeat }
            / ";" { End }
            / "x" { Exit }
            / "#" { Teleport }
            / "<" { MoveLeft }
            / ">" { MoveRight }
            / "\"" { DoubleQuote }
            / "'" { SingleQuote }
            / "m" { CallMethod }
            / "i" { Input }
            / "I" { InputLength }
            / "l" { Length }
            / "z" { GetAllInput }
            / "Z" { OutputAll }
            / "O" { OutputChar }
            / "N" { OutputNumeric }
            / "W" { GetLine }
            / "$" { Switch }
            / "%" { MultiSwitch }
            / "}" { RotateRight }
            / "{" { RotateLeft }
            / "r" { Reverse }
            / "@" { Part }
            / "D" { Duplicate }
            / "X" { Remove }
            / "o" { Object }
            / "&" { NewStack }
            / "Y" { RemoveStack }
            / ":" { CloneStack }
            / "?" { RightStack }
            / "|" { LeftStack }
            / "u" { Flatten }
            / "y" { StackCount }
            / "v" { TemporaryVariable }
            / "V" { FinalVariable }
            / "S" { Sine }
            / "s" { Arcsine }
            / "T" { Tangent }
            / "t" { Arctangent }
            / "C" { Cosine }
            / "s" { Arccosine }
            / "P" { Pi }
            / "E" { E }
            / "L" { Log }
            / "R" { Random }
            / "+" { Add }
            / "-" { Subtract }
            / "*" { Multiply }
            / "/" { Divide }
            / "^" { Power }
            / "=" { Equals }
            / "M" { Modulo }
            / "_" { Truncate }
            / "h" { Factorize }
            / "p" { Prime }
            / "H" { Range }
            / "k" { CallProgramMethod }
            / "K" { CallShortProgramMethod }
            / "g" { UseCount }
            / "G" { UseName }
            / "w" { Wait }
            / "`" { File }
            / "." { Write }
            / "," { Shell }
            / "n" { Eval }
            / v:value() { Value(v) }

        rule value() -> u8
            = "0" { 0 }
            / "1" { 1 }
            / "2" { 2 }
            / "3" { 3 }
            / "4" { 4 }
            / "5" { 5 }
            / "6" { 6 }
            / "7" { 7 }
            / "8" { 8 }
            / "9" { 9 }
            / "a" { 10 }
            / "b" { 11 }
            / "c" { 12 }
            / "d" { 13 }
            / "e" { 14 }
            / "f" { 15 }
    }
}

#[inline]
pub fn parse(name: String) -> Result<Rc<RefCell<Program>>, Box<dyn Error>> {
    Ok(enriched_parse(name)?)
}
