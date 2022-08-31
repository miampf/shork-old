use shork_lexer::tokens::Token;
use shork_error::{ShorkError, ErrorType};

/// Represents an Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq)]
pub struct AST<'a>{
    arena: Vec<&'a Node>
}

/// A node with an ID
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Node{
    id: usize,
    val: Token,
    parent: Option<usize>,
    children: Vec<usize>
}

impl<'a> AST<'a>{
    /// create a new AST
    pub fn new() -> Self{
        Self { arena: Vec::new() }
    }

    /// add a node to the arena
    pub fn add(&mut self, n: &'a Node) {
        self.arena.push(n);
    }

    /// get a node from an id
    pub fn get(&mut self, id: usize) -> Result<&Node, ShorkError>{
        self.arena.sort();
        let index = self.arena.binary_search_by_key(&id, |n| n.id);
        if index.is_err(){
            return Err(
                ShorkError::generate_error(ErrorType::ParserError, 0, "".to_string(), "Failed to find Node in AST. This is an error by the interpreter and not in your source code".to_string())
            );
        }

        Ok(&self.arena[index.unwrap()])
    }

    /// get the siblings of a node. Includes the node itself
    pub fn siblings(&mut self, n: &Node) -> Result<Vec<usize>, ShorkError>{
        let p = n.parent();
        if p.is_none(){
            let e = ShorkError::generate_error(ErrorType::ParserError, 0, "".to_string(), "Requested node siblings on root node. This is an error by the interpreter and not in your source code".to_string());
            return Err(e);
        }
        let p_n = self.get(p.unwrap())?;
        let siblings = p_n.children();
        Ok(siblings.clone())
    }
}

impl Node{
    /// create a new node
    pub fn new(id: usize, val: Token, parent: Option<usize>, children: Vec<usize>) -> Self{
        Self { id, val, parent, children }
    }

    /// set the parent
    pub fn set_parent(&mut self, parent: Option<usize>) {
        self.parent = parent
    }

    /// add a child node
    pub fn add_child(&mut self, id: usize) {
        self.children.push(id)
    }

    /// get the nodes value
    pub fn val(&self) -> &Token{
        &self.val
    }

    /// get the nodes ID
    pub fn id(&self) -> usize{
        self.id
    }

    /// get the nodes parent
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// get the nodes children
    pub fn children(&self) -> &Vec<usize>{
        &self.children
    }
}

impl Ord for Node{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}