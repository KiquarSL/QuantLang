use crate::lexer::TokenType;
use crate::parser::{NewVarStmt, Parser, Stmt};

type StmtT = Box<dyn Stmt>;

pub fn is_assign_stmt(parser: &mut Parser) -> bool {
    let data_type = parser.get(0);
    let id = parser.get(1);
    matches!(
        (data_type.kind, id.kind),
        (TokenType::Type(_), TokenType::Id(_))
    )
}

pub fn parse_assign_stmt(parser: &mut Parser) -> StmtT {
    let data_type = parser.get(0);
    let id = parser.get(1);
    if let TokenType::Type(_) = data_type.kind
        && let TokenType::Id(id) = id.kind
    {
        parser.next();
        parser.next();
        parser.next();

        let value = parser.expression();
        parser.match_type(TokenType::Semicolon); // if you sooo want use ;, the use it :)
        return Box::new(NewVarStmt::new(id, value));
    }
    panic!("Failed to make assign statement");
}
