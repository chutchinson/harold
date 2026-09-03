#[derive(Debug)]
pub struct Document {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Instruction(InstructionDef),
    Label(LabelDef),
    Comment(CommentDef),
}

#[derive(Debug)]
pub enum Operand {
    None,
    Address(u16),
    Register(String),
    RegisterAddress(String),
    Integer(i16),
    Float(f32),
}

#[derive(Debug)]
pub struct InstructionDef {
    pub mnemonic: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub struct LabelDef {
    pub name: String,
}

#[derive(Debug)]
pub struct CommentDef {
    pub text: String,
}
