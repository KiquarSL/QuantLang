use crate::lexer::TokenType;
use crate::parser::{AssignStmt, IfStmt, NewVarStmt, Parser, Stmt};

type StmtT = Box<dyn Stmt>;

// Assign
pub fn is_new_var_stmt(parser: &mut Parser) -> bool {
    let data_type = parser.get(0);
    let id = parser.get(1);
    matches!(
        (data_type.kind, id.kind),
        (TokenType::Type(_), TokenType::Id(_))
    )
}

pub fn parse_new_var_stmt(parser: &mut Parser) -> StmtT {
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

// Assign
pub fn is_assign_stmt(parser: &mut Parser) -> bool {
    let id = parser.get(0);
    let ass = parser.get(1);
    matches!((id.kind, ass.kind), (TokenType::Id(_), TokenType::Assign))
}

pub fn parse_assign_stmt(parser: &mut Parser) -> StmtT {
    let id = parser.get(0);
    let ass = parser.get(1);
    if let TokenType::Id(id) = id.kind
        && let TokenType::Assign = ass.kind
    {
        parser.next();
        parser.next();

        let value = parser.expression();
        parser.match_type(TokenType::Semicolon); // if you sooo want use ;, the use it :)
        return Box::new(AssignStmt::new(id, value));
    }
    panic!("Failed to make assign statement");
}

// If stmt
pub fn is_if_stmt(parser: &mut Parser) -> bool {
    let ifkw = parser.get(0);
    ifkw.kind == TokenType::Keyword("if".to_string())
}

pub fn parse_if_stmt(parser: &mut Parser) -> StmtT {
    parser.match_type(TokenType::Keyword("if".to_string()));
    let expr = parser.expression();
    parser.match_type(TokenType::LBracket);
    let thenb = parser.parse_block();
    let mut elseb = None;
    parser.match_type(TokenType::RBracket);
    if parser.match_type(TokenType::Keyword("else".to_string())) {
        parser.match_type(TokenType::LBracket);
        elseb = Some(parser.parse_block());
        parser.match_type(TokenType::RBracket);
    }
    Box::new(IfStmt::new(expr, thenb, elseb))
}
