
//The defining the grammer
enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

fn parse_expression(tokens: &[GrammarItem]) -> Result<(Expression, &[GrammarItem]), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input".to_string());
    }

    let (first_expr, remaining_tokens) = parse_primary(tokens)?;

    parse_binary_op(first_expr, remaining_tokens)
}

fn parse_primary(tokens: &[GrammarItem]) -> Result<(Expression, &[GrammarItem]), String> {
    match tokens.first() {
        Some(GrammarItem::Number(n)) => Ok((Expression::Number(*n), &tokens[1..])),
        Some(GrammarItem::LeftParen) => {
            let (expr, remaining_tokens) = parse_expression(&tokens[1..])?;
            match remaining_tokens.first() {
                Some(GrammarItem::RightParen) => Ok((expr, &remaining_tokens[1..])),
                _ => Err("Expected closing parenthesis".to_string()),
            }
        },
        _ => Err("Expected number or open parenthesis".to_string()),
    }
}

fn parse_binary_op(expr: Expression, tokens: &[GrammarItem]) -> Result<(Expression, &[GrammarItem]), String> {
    if tokens.is_empty() {
        return Ok((expr, tokens));
    }

    match tokens.first() {
        Some(GrammarItem::Plus) => {
            let (right_expr, remaining_tokens) = parse_expression(&tokens[1..])?;
            Ok((Expression::Add(Box::new(expr), Box::new(right_expr)), remaining_tokens))
        },
        // Handle other operators (Minus, Multiply, Divide) similarly...
        _ => Ok((expr, tokens)),
    }
}
