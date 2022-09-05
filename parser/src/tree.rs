use shork_lexer::tokens::Token;
use shork_error::{ShorkError, ErrorType};

/// Represents an Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq)]
pub struct AST{
    arena: Vec<Node>
}

/// A node with an ID
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Node{
    id: usize,
    val: Token,
    parent: Option<usize>,
    children: Vec<usize>
}

impl AST{
    /// create a new AST
    pub fn new() -> Self{
        Self { arena: Vec::new() }
    }

    /// add a node to the arena
    pub fn add(&mut self, n: Node) {
        self.arena.push(n);
    }

    /// add all nodes of this tree to another tree
    pub fn clone_into_tree(&self, other: &mut Self) {
        for n in self.arena.clone(){
            other.add(n)
        }
    }

    /// give all current root nodes the given id as a root
    pub fn set_root_all(&mut self, root_id: usize) {
        for cr in self.root(){
            let n = self.get_mut(cr).unwrap(); // unwrap should be safe
            n.set_parent(Some(root_id))
        }
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

    /// get a mutable node from an id
    pub fn get_mut(&mut self, id: usize) -> Result<&mut Node, ShorkError>{
        self.arena.sort();
        let index = self.arena.binary_search_by_key(&id, |n| n.id);
        if index.is_err(){
            return Err(
                ShorkError::generate_error(ErrorType::ParserError, 0, "".to_string(), "Failed to find Node in AST. This is an error by the interpreter and not in your source code".to_string())
            );
        }
        Ok(&mut self.arena[index.unwrap()])
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

    /// get the root node(s)
    pub fn root(&self) -> Vec<usize>{
        let mut n_vec = Vec::new();
        for n in &self.arena{
            if n.parent().is_none(){
                n_vec.push(n.id())
            }
        }

        n_vec
    }

    /// print the tree
    pub fn print(&mut self) {
        use ptree::{print_tree_with, Color, PrintConfig, Style, print_config};

        let config = {
            let mut config = PrintConfig::default();
            if std::env::var("NO_COLOR").is_ok(){
                config.branch = Style::default();
                config.leaf = Style::default();
                config.characters = print_config::ASCII_CHARS_TICK.into();
            } else {
                config.branch = Style {
                    foreground: Some(Color::Blue),
                    dimmed: true,
                    ..Style::default()
                };
                config.leaf = Style {
                    foreground: Some(Color::Green),
                    bold: true,
                    ..Style::default()
                };
                config.characters = print_config::UTF_CHARS.into();
            }
            config.indent = 4;
            config
        };

        let mut parena = Vec::new();

        for n in self.arena.clone(){
            parena.push(n.clone())
        }

        for n_id in self.root(){
            let n = self.get(n_id).expect("Failed to get node from id");
            let n_print = NodePrinter{
                node: n.clone(),
                arena: parena.clone()
            };
            print_tree_with(&n_print, &config).expect("Failed to print tree");
        }
    }
}

impl Node{
    /// create a new node
    pub fn new(id: usize, val: Token, parent: Option<usize>, children: Vec<usize>) -> Self{
        Self { id, val, parent, children}
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

#[derive(Debug, Clone)]
struct NodePrinter{
    node: Node,
    arena: Vec<Node>
}

impl ptree::TreeItem for NodePrinter{
    type Child = Self;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &ptree::Style) -> std::io::Result<()> {
        write!(f, "{} {}", style.paint(format!("{:?}", self.node.val().token_type())), style.paint(format!("{:?}", self.node.val().raw())))
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        let mut c_vec = Vec::new();

        for c in self.node.children(){
            for n in &self.arena{
                if n.id() == *c{
                    let n_print = Self {
                        node: n.clone(),
                        arena: self.arena.clone()
                    };
                    c_vec.push(n_print)
                }
            }
        }

        c_vec.into()
    }
}