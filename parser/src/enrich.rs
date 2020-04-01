use crate::parse::{
    parse, Extend as ParserExtend, Method as ParserMethod, Operation as ParserOperation,
    Program as ParserProgram, Use as ParserUse,
};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    ops: RefCell<Vec<Operation>>,
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
    Method,
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
    ProgramMethod,
    ShortProgramMethod,
    UseCount,
    UseName,
    Wait,
    File,
    Write,
    Shell,
    Eval,
}

pub fn enrich(name: String, program: ParserProgram) -> Result<Program, Box<dyn Error>> {
    let mut prog_cache = HashMap::new();
    prog_cache.insert(name.clone(), Rc::new(program));
    let method_cache = HashMap::new();

    Ok(Program {
        ops: real_enrich(name, 0, prog_cache, method_cache)?
            .as_ref()
            .clone(),
    })
}

fn real_enrich(
    name: String,
    index: usize,
    mut prog_cache: HashMap<String, Rc<ParserProgram>>,
    mut method_cache: HashMap<String, HashMap<usize, Rc<RefCell<Vec<Operation>>>>>,
) -> Result<Rc<RefCell<Vec<Operation>>>, Box<dyn Error>> {
    let program = match prog_cache.entry(name.clone()) {
        Entry::Occupied(o) => o.get().clone(),
        Entry::Vacant(v) => v
            .insert(Rc::new(
                File::open(Path::new(&name))
                    .map(|mut f| {
                        let mut contents = String::new();
                        match f.read_to_string(&mut contents) {
                            Ok(_) => Ok(contents),
                            Err(e) => Err(e),
                        }
                    })?
                    .map(|code| parse(code))??,
            ))
            .clone(),
    };
    let method = method_cache
        .entry(name.clone())
        .or_insert_with(|| HashMap::new())
        .entry(index);

    match method {
        Entry::Occupied(ops) => Ok(ops.get().clone()),
        Entry::Vacant(ops) => {
            let ops = ops.insert(Rc::new(RefCell::new(base_ops(
                &program
                    .methods
                    .get(index)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!(
                                "Tried to get method {} of {}, but it didn't exist",
                                index, name
                            ),
                        )
                    })?
                    .ops,
            ))));
            for op in ops.borrow_mut().iter_mut() {
                enrich_op(op)?;
            }
            Ok(ops.clone())
        }
    }
}

fn base_ops(ops: &Vec<ParserOperation>) -> Vec<Operation> {
    let mut ret = Vec::<Operation>::new();
    for op in ops {
        ret.push(match op {
            ParserOperation::IfNot => Operation::IfNot,
            ParserOperation::If => Operation::If,
            ParserOperation::BeginBlock => Operation::BeginBlock,
            ParserOperation::EndBlock => Operation::EndBlock,
            ParserOperation::Repeat => Operation::Repeat,
            ParserOperation::End => Operation::End,
            ParserOperation::Exit => Operation::Exit,
            ParserOperation::Teleport => Operation::Teleport,
            ParserOperation::MoveLeft => Operation::MoveLeft,
            ParserOperation::MoveRight => Operation::MoveRight,
            ParserOperation::DoubleQuote => Operation::DoubleQuote,
            ParserOperation::SingleQuote => Operation::SingleQuote,
            ParserOperation::CallMethod => Operation::Method,
            ParserOperation::Input => Operation::Input,
            ParserOperation::InputLength => Operation::InputLength,
            ParserOperation::Length => Operation::Length,
            ParserOperation::GetAllInput => Operation::GetAllInput,
            ParserOperation::OutputAll => Operation::OutputAll,
            ParserOperation::OutputChar => Operation::OutputChar,
            ParserOperation::OutputNumeric => Operation::OutputNumeric,
            ParserOperation::GetLine => Operation::GetLine,
            ParserOperation::Switch => Operation::Switch,
            ParserOperation::MultiSwitch => Operation::MultiSwitch,
            ParserOperation::RotateRight => Operation::RotateRight,
            ParserOperation::RotateLeft => Operation::RotateLeft,
            ParserOperation::Reverse => Operation::Reverse,
            ParserOperation::Part => Operation::Part,
            ParserOperation::Duplicate => Operation::Duplicate,
            ParserOperation::Remove => Operation::Remove,
            ParserOperation::Object => Operation::Object,
            ParserOperation::NewStack => Operation::NewStack,
            ParserOperation::RemoveStack => Operation::RemoveStack,
            ParserOperation::CloneStack => Operation::CloneStack,
            ParserOperation::RightStack => Operation::RightStack,
            ParserOperation::LeftStack => Operation::LeftStack,
            ParserOperation::Flatten => Operation::Flatten,
            ParserOperation::StackCount => Operation::StackCount,
            ParserOperation::TemporaryVariable => Operation::TemporaryVariable,
            ParserOperation::FinalVariable => Operation::FinalVariable,
            ParserOperation::Sine => Operation::Sine,
            ParserOperation::Arcsine => Operation::Arcsine,
            ParserOperation::Tangent => Operation::Tangent,
            ParserOperation::Arctangent => Operation::Arctangent,
            ParserOperation::Cosine => Operation::Cosine,
            ParserOperation::Arccosine => Operation::Arccosine,
            ParserOperation::Pi => Operation::Pi,
            ParserOperation::E => Operation::E,
            ParserOperation::Log => Operation::Log,
            ParserOperation::Random => Operation::Random,
            ParserOperation::Add => Operation::Add,
            ParserOperation::Subtract => Operation::Subtract,
            ParserOperation::Multiply => Operation::Multiply,
            ParserOperation::Divide => Operation::Divide,
            ParserOperation::Power => Operation::Power,
            ParserOperation::Equals => Operation::Equals,
            ParserOperation::Modulo => Operation::Modulo,
            ParserOperation::Truncate => Operation::Truncate,
            ParserOperation::Factorize => Operation::Factorize,
            ParserOperation::Prime => Operation::Prime,
            ParserOperation::Range => Operation::Range,
            ParserOperation::CallProgramMethod => Operation::ProgramMethod,
            ParserOperation::CallShortProgramMethod => Operation::ShortProgramMethod,
            ParserOperation::UseCount => Operation::UseCount,
            ParserOperation::UseName => Operation::UseName,
            ParserOperation::Wait => Operation::Wait,
            ParserOperation::File => Operation::File,
            ParserOperation::Write => Operation::Write,
            ParserOperation::Shell => Operation::Shell,
            ParserOperation::Eval => Operation::Eval,
        })
    }
    ret
}

fn enrich_op(op: &mut Operation) -> Result<(), Box<dyn Error>> {
    match op {
        Operation::IfNot => unimplemented!(),
        Operation::If => unimplemented!(),
        Operation::BeginBlock => unimplemented!(),
        Operation::EndBlock => unimplemented!(),
        Operation::Repeat => unimplemented!(),
        Operation::End => unimplemented!(),
        Operation::Exit => unimplemented!(),
        Operation::Teleport => unimplemented!(),
        Operation::MoveLeft => unimplemented!(),
        Operation::MoveRight => unimplemented!(),
        Operation::DoubleQuote => unimplemented!(),
        Operation::SingleQuote => unimplemented!(),
        Operation::Method => unimplemented!(),
        Operation::Input => unimplemented!(),
        Operation::InputLength => unimplemented!(),
        Operation::Length => unimplemented!(),
        Operation::GetAllInput => unimplemented!(),
        Operation::OutputAll => unimplemented!(),
        Operation::OutputChar => unimplemented!(),
        Operation::OutputNumeric => unimplemented!(),
        Operation::GetLine => unimplemented!(),
        Operation::Switch => unimplemented!(),
        Operation::MultiSwitch => unimplemented!(),
        Operation::RotateRight => unimplemented!(),
        Operation::RotateLeft => unimplemented!(),
        Operation::Reverse => unimplemented!(),
        Operation::Part => unimplemented!(),
        Operation::Duplicate => unimplemented!(),
        Operation::Remove => unimplemented!(),
        Operation::Object => unimplemented!(),
        Operation::NewStack => unimplemented!(),
        Operation::RemoveStack => unimplemented!(),
        Operation::CloneStack => unimplemented!(),
        Operation::RightStack => unimplemented!(),
        Operation::LeftStack => unimplemented!(),
        Operation::Flatten => unimplemented!(),
        Operation::StackCount => unimplemented!(),
        Operation::TemporaryVariable => unimplemented!(),
        Operation::FinalVariable => unimplemented!(),
        Operation::Sine => unimplemented!(),
        Operation::Arcsine => unimplemented!(),
        Operation::Tangent => unimplemented!(),
        Operation::Arctangent => unimplemented!(),
        Operation::Cosine => unimplemented!(),
        Operation::Arccosine => unimplemented!(),
        Operation::Pi => unimplemented!(),
        Operation::E => unimplemented!(),
        Operation::Log => unimplemented!(),
        Operation::Random => unimplemented!(),
        Operation::Add => unimplemented!(),
        Operation::Subtract => unimplemented!(),
        Operation::Multiply => unimplemented!(),
        Operation::Divide => unimplemented!(),
        Operation::Power => unimplemented!(),
        Operation::Equals => unimplemented!(),
        Operation::Modulo => unimplemented!(),
        Operation::Truncate => unimplemented!(),
        Operation::Factorize => unimplemented!(),
        Operation::Prime => unimplemented!(),
        Operation::Range => unimplemented!(),
        Operation::ProgramMethod => unimplemented!(),
        Operation::ShortProgramMethod => unimplemented!(),
        Operation::UseCount => unimplemented!(),
        Operation::UseName => unimplemented!(),
        Operation::Wait => unimplemented!(),
        Operation::File => unimplemented!(),
        Operation::Write => unimplemented!(),
        Operation::Shell => unimplemented!(),
        Operation::Eval => unimplemented!(),
    }
}
