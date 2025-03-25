// The use Keyword in Rust
//
// This file demonstrates how to use the 'use' keyword to bring
// items into scope, making it more convenient to reference them.

fn main() {
    println!("Understanding the 'use' Keyword in Rust!");
    
    //------------------------------------------------------
    // BASIC USE STATEMENTS
    //------------------------------------------------------
    println!("\n=== Basic Use Statements ===");
    
    // Without 'use', we need to specify the full path each time
    let mut map1 = std::collections::HashMap::new();
    map1.insert("key1", "value1");
    println!("Map created with full path: {:?}", map1);
    
    // With 'use', we can bring the item into scope
    use std::collections::HashMap;
    let mut map2 = HashMap::new();
    map2.insert("key2", "value2");
    println!("Map created with 'use' statement: {:?}", map2);
    
    //------------------------------------------------------
    // IDIOMATIC USE PATTERNS
    //------------------------------------------------------
    println!("\n=== Idiomatic Use Patterns ===");
    
    // For functions, bring the parent module into scope
    use std::fmt;
    fmt::println!("Using fmt::println after importing std::fmt");
    
    // For types (structs, enums), bring the full path into scope
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert("item");
    println!("HashSet: {:?}", set);
    
    //------------------------------------------------------
    // NESTED PATHS
    //------------------------------------------------------
    println!("\n=== Nested Paths ===");
    
    // Instead of:
    // use std::io;
    // use std::io::Write;
    
    // We can use nested paths:
    use std::io::{self, Write};
    
    // Now we can use both io and io::Write
    let _stdout = io::stdout();
    
    //------------------------------------------------------
    // MULTIPLE ITEMS
    //------------------------------------------------------
    println!("\n=== Multiple Items ===");
    
    // Import multiple items from the same module
    use std::collections::{BTreeMap, BTreeSet};
    
    let mut btree_map = BTreeMap::new();
    btree_map.insert("key", "value");
    
    let mut btree_set = BTreeSet::new();
    btree_set.insert("item");
    
    println!("BTreeMap: {:?}", btree_map);
    println!("BTreeSet: {:?}", btree_set);
    
    //------------------------------------------------------
    // RENAMING WITH AS
    //------------------------------------------------------
    println!("\n=== Renaming with 'as' ===");
    
    // Rename imports to avoid naming conflicts
    use std::fmt::Result;
    use std::io::Result as IoResult;
    
    // Now we can use both Result types without conflict
    fn _function_returning_fmt_result() -> Result {
        Ok(())
    }
    
    fn _function_returning_io_result() -> IoResult<()> {
        Ok(())
    }
    
    println!("Successfully defined functions with different Result types");
    
    //------------------------------------------------------
    // RE-EXPORTING WITH PUB USE
    //------------------------------------------------------
    println!("\n=== Re-exporting with 'pub use' ===");
    
    // Demonstrate re-exporting
    mod geometry {
        pub mod shapes {
            pub struct Circle {
                pub radius: f64,
            }
            
            impl Circle {
                pub fn new(radius: f64) -> Circle {
                    Circle { radius }
                }
                
                pub fn area(&self) -> f64 {
                    std::f64::consts::PI * self.radius * self.radius
                }
            }
        }
    }
    
    mod graphics {
        // Re-export the Circle from geometry::shapes
        pub use crate::geometry::shapes::Circle;
        
        pub fn draw_circle(circle: &Circle) {
            println!("Drawing a circle with area: {}", circle.area());
        }
    }
    
    // Now we can use Circle directly from graphics
    use graphics::Circle;
    let circle = Circle::new(5.0);
    graphics::draw_circle(&circle);
    
    //------------------------------------------------------
    // THE GLOB OPERATOR
    //------------------------------------------------------
    println!("\n=== The Glob Operator ===");
    
    // The glob operator (*) brings all public items into scope
    // Generally avoided in production code except for specific cases
    
    // Example with the prelude module (which is automatically imported)
    // use std::prelude::v1::*;
    
    // More common in tests
    mod test_helpers {
        pub fn setup() {
            println!("Setting up test");
        }
        
        pub fn teardown() {
            println!("Tearing down test");
        }
    }
    
    mod tests {
        // In tests, glob imports are more common
        use super::test_helpers::*;
        
        #[allow(dead_code)]
        fn run_test() {
            setup();
            println!("Running test");
            teardown();
        }
    }
    
    //------------------------------------------------------
    // SCOPED IMPORTS
    //------------------------------------------------------
    println!("\n=== Scoped Imports ===");
    
    // Use statements can be scoped to blocks
    {
        // This import is only valid in this block
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back("first");
        queue.push_back("second");
        println!("Queue: {:?}", queue);
    }
    
    // This would cause an error - VecDeque is not in scope here
    // let queue = VecDeque::new();
    
    // We can also use 'use' in functions, which scopes the import to that function
    fn demonstrate_binary_heap() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);
        println!("Binary heap: {:?}", heap);
    }
    
    demonstrate_binary_heap();
}

//------------------------------------------------------
// MODULE EXAMPLE FOR CHALLENGES
//------------------------------------------------------

// A module hierarchy for the challenges
mod data_structures {
    pub mod linear {
        pub struct LinkedList<T> {
            head: Option<Box<Node<T>>>,
        }
        
        struct Node<T> {
            value: T,
            next: Option<Box<Node<T>>>,
        }
        
        impl<T> LinkedList<T> {
            pub fn new() -> Self {
                LinkedList { head: None }
            }
            
            pub fn push(&mut self, value: T) {
                let new_node = Box::new(Node {
                    value,
                    next: self.head.take(),
                });
                self.head = Some(new_node);
            }
            
            pub fn is_empty(&self) -> bool {
                self.head.is_none()
            }
        }
        
        pub struct Queue<T> {
            items: Vec<T>,
        }
        
        impl<T> Queue<T> {
            pub fn new() -> Self {
                Queue { items: Vec::new() }
            }
            
            pub fn enqueue(&mut self, item: T) {
                self.items.push(item);
            }
            
            pub fn is_empty(&self) -> bool {
                self.items.is_empty()
            }
        }
    }
    
    pub mod tree {
        pub struct BinaryTree<T> {
            root: Option<Box<Node<T>>>,
        }
        
        struct Node<T> {
            value: T,
            left: Option<Box<Node<T>>>,
            right: Option<Box<Node<T>>>,
        }
        
        impl<T> BinaryTree<T> {
            pub fn new() -> Self {
                BinaryTree { root: None }
            }
            
            pub fn is_empty(&self) -> bool {
                self.root.is_none()
            }
        }
        
        pub struct Trie {
            root: TrieNode,
        }
        
        struct TrieNode {
            children: std::collections::HashMap<char, TrieNode>,
            is_end_of_word: bool,
        }
        
        impl Trie {
            pub fn new() -> Self {
                Trie {
                    root: TrieNode {
                        children: std::collections::HashMap::new(),
                        is_end_of_word: false,
                    }
                }
            }
            
            pub fn is_empty(&self) -> bool {
                self.root.children.is_empty()
            }
        }
    }
    
    pub mod graph {
        pub struct Graph {
            edges: std::collections::HashMap<usize, Vec<usize>>,
        }
        
        impl Graph {
            pub fn new() -> Self {
                Graph { edges: std::collections::HashMap::new() }
            }
            
            pub fn add_edge(&mut self, from: usize, to: usize) {
                self.edges.entry(from).or_insert_with(Vec::new).push(to);
            }
            
            pub fn is_empty(&self) -> bool {
                self.edges.is_empty()
            }
        }
    }
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Basic Use Statements
//
// Complete this function using appropriate 'use' statements to
// simplify the code.

mod challenge_1 {
    pub fn use_basic_imports() {
        // TODO: Add 'use' statements here to simplify the code below
        
        // Create a HashMap
        let mut scores = std::collections::HashMap::new();
        scores.insert("Blue Team", 10);
        scores.insert("Red Team", 5);
        
        // Create a HashSet
        let mut unique_words = std::collections::HashSet::new();
        unique_words.insert("hello");
        unique_words.insert("world");
        
        // Create a BinaryHeap
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(5);
        heap.push(10);
        
        println!("Challenge 1 completed!");
    }
}

// Challenge 2: Nested Paths
//
// Refactor this code to use nested paths for the imports.

mod challenge_2 {
    pub fn use_nested_paths() {
        // TODO: Replace these individual imports with nested path imports
        use std::collections::HashMap;
        use std::collections::HashSet;
        use std::collections::BTreeMap;
        use std::collections::BTreeSet;
        
        let mut hash_map = HashMap::new();
        hash_map.insert("key1", "value1");
        
        let mut hash_set = HashSet::new();
        hash_set.insert("item1");
        
        let mut btree_map = BTreeMap::new();
        btree_map.insert("key2", "value2");
        
        let mut btree_set = BTreeSet::new();
        btree_set.insert("item2");
        
        println!("Challenge 2 completed!");
    }
}

// Challenge 3: Renaming with 'as'
//
// Fix the code to handle the naming conflict using 'as'.

mod challenge_3 {
    // These modules both define a `Result` type
    mod module_a {
        pub struct Result {
            pub success: bool,
            pub value: String,
        }
        
        impl Result {
            pub fn new(success: bool, value: &str) -> Self {
                Result {
                    success,
                    value: value.to_string(),
                }
            }
        }
    }
    
    mod module_b {
        pub struct Result {
            pub is_ok: bool,
            pub error_code: i32,
        }
        
        impl Result {
            pub fn new(is_ok: bool, error_code: i32) -> Self {
                Result {
                    is_ok,
                    error_code,
                }
            }
        }
    }
    
    pub fn use_renaming() {
        // TODO: Fix these imports using 'as' to handle the naming conflict
        use module_a::Result;
        use module_b::Result;
        
        // Create results from both modules
        let result_a = Result::new(true, "Success!");
        let result_b = Result::new(false, 404);
        
        println!("Result A: {} with value '{}'", result_a.success, result_a.value);
        println!("Result B: {} with error code {}", result_b.is_ok, result_b.error_code);
        
        println!("Challenge 3 completed!");
    }
}

// Challenge 4: Re-exporting with 'pub use'
//
// Fix the code to create a cleaner API using 'pub use'.

mod challenge_4 {
    // This represents a complex module structure that we want to simplify
    mod internal {
        pub mod data {
            pub mod structures {
                pub struct User {
                    pub name: String,
                    pub email: String,
                }
                
                impl User {
                    pub fn new(name: &str, email: &str) -> Self {
                        User {
                            name: name.to_string(),
                            email: email.to_string(),
                        }
                    }
                }
                
                pub struct Product {
                    pub name: String,
                    pub price: f64,
                }
                
                impl Product {
                    pub fn new(name: &str, price: f64) -> Self {
                        Product {
                            name: name.to_string(),
                            price,
                        }
                    }
                }
            }
        }
    }
    
    // TODO: Add 'pub use' statements here to re-export User and Product
    // directly from the 'api' module
    
    pub mod api {
        // Add re-export here to simplify the API
    }
    
    pub fn use_reexporting() {
        // Right now we need this complex path:
        let user = internal::data::structures::User::new("Alice", "alice@example.com");
        let product = internal::data::structures::Product::new("Book", 29.99);
        
        println!("User: {} ({})", user.name, user.email);
        println!("Product: {} (${:.2})", product.name, product.price);
        
        // TODO: After your re-exports, uncomment these lines:
        // let user2 = api::User::new("Bob", "bob@example.com");
        // let product2 = api::Product::new("Notebook", 9.99);
        
        // println!("User2: {} ({})", user2.name, user2.email);
        // println!("Product2: {} (${:.2})", product2.name, product2.price);
        
        println!("Challenge 4 completed!");
    }
}

// Challenge 5: Using External Data Structures
//
// Use the data_structures module we defined earlier to complete this challenge.

mod challenge_5 {
    pub fn use_data_structures() {
        // TODO: Add appropriate 'use' statements to simplify accessing these types
        
        // Create a LinkedList
        let mut list = crate::data_structures::linear::LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        println!("LinkedList is empty: {}", list.is_empty());
        
        // Create a Queue
        let mut queue = crate::data_structures::linear::Queue::new();
        queue.enqueue("first");
        queue.enqueue("second");
        println!("Queue is empty: {}", queue.is_empty());
        
        // Create a BinaryTree
        let tree = crate::data_structures::tree::BinaryTree::<i32>::new();
        println!("Tree is empty: {}", tree.is_empty());
        
        // Create a Graph
        let graph = crate::data_structures::graph::Graph::new();
        println!("Graph is empty: {}", graph.is_empty());
        
        println!("Challenge 5 completed!");
    }
}

// A function to run all challenges
pub fn run_use_keyword_challenges() {
    println!("\nRunning 'use' keyword challenges...");
    
    challenge_1::use_basic_imports();
    challenge_2::use_nested_paths();
    challenge_3::use_renaming();
    challenge_4::use_reexporting();
    challenge_5::use_data_structures();
    
    println!("All 'use' keyword challenges completed!");
} 