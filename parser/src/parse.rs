use std::error::Error;

pub struct Program {
    pub methods: Vec<Method>,
    pub extends: Vec<Extend>,
    pub uses: Vec<Use>,
}

pub struct Method {
    pub ops: Vec<Operation>,
}

pub struct Extend {
    pub target: String,
}

pub struct Use {
    pub target: String,
}

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
}

peg::parser! {
    grammar program_parser() for str {
        use Operation::*;

        pub rule program() -> Program
            = methods:(m:method() "\n" { m })+
              extends:(e:extend_decl() "\n" { e })*
              uses:(u:use_decl() ** "\n" { u }) "\n"*
            {
                Program {
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
    }
}

#[inline]
pub fn parse(code: String) -> Result<Program, Box<dyn Error>> {
    Ok(program_parser::program(code.as_str())?)
}
