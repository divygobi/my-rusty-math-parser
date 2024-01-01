#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}


impl ParseNode {
    pub fn new(entry: GrammarItem) -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry,
        }
    }

    pub fn number(value: i32) -> ParseNode {
        ParseNode::new(GrammarItem::Number(value))
    }

    pub fn binary_operation(op: BinaryOp, left: ParseNode, right: ParseNode) -> ParseNode {
        let mut node = ParseNode::new(GrammarItem::BinaryOperation(op));
        node.children.push(left);
        node.children.push(right);
        node
    }

    pub fn parentheses(inner: ParseNode) -> ParseNode {
        let mut node = ParseNode::new(GrammarItem::Paren);
        node.children.push(inner);
        node
    }
}