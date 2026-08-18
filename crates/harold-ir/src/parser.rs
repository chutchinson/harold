use super::ir::*;

peg::parser! {
  pub grammar asm_parser() for str {
    pub rule document() -> Document
        = __? statements:(statement() ** "\n") __? {
            Document {
                statements
            }
        }

    rule statement() -> Statement
        = _? v:label() _? { Statement::Label(v) }
        / _? v:instruction() _? { Statement::Instruction(v) }

    rule instruction() -> InstructionDef
        = mnemonic:identifier() _ operands:(operand() ** ",")
        { InstructionDef { mnemonic, operands }}

    rule operand() -> Operand
        = _? v:number() { Operand::Integer(v) }
        / _? v:register() { Operand::Register(v) }

    rule register() -> String
        = "rx" { "rx".to_string() }
        / "ry" { "ry".to_string() }
        / "rz" { "rz".to_string() }
        / "rw" { "rw".to_string() }
        / "sp" { "sp".to_string() }

    rule label() -> LabelDef
        = ident:identifier() ":" { LabelDef { name: ident } }

    rule identifier() -> String
        = ident:$(['a'..='z' | 'A'..='Z']+) { ident.to_string() }

    rule number() -> i16
        = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("i16")) }

    rule __()
        = [' ' | '\t' | '\r' | '\n']+

    rule _()
        = [' ' | '\t' | '\r']+
  }
}
