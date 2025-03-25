// Traits in Rust
//
// This file demonstrates how to define and use traits to create
// shared behavior across different types.

use std::fmt::{Display, Debug};

fn main() {
    println!("Understanding Traits in Rust!");
    
    //------------------------------------------------------
    // DEFINING AND IMPLEMENTING TRAITS
    //------------------------------------------------------
    println!("\n=== Defining and Implementing Traits ===");
    
    let article = NewsArticle {
        headline: String::from("Rust 1.60 Released"),
        location: String::from("Mozilla"),
        author: String::from("Rust Team"),
        content: String::from("Today, the Rust team released version 1.60..."),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Excited to announce Rust 1.60!"),
        reply: false,
        retweet: false,
    };
    
    // Call the trait method on different types
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    //------------------------------------------------------
    // DEFAULT IMPLEMENTATIONS
    //------------------------------------------------------
    println!("\n=== Default Implementations ===");
    
    // Call the default method that uses the custom implementation
    println!("Article notify: {}", article.notify());
    
    // Call a method with default implementation
    println!("Tweet default summary: {}", tweet.summarize_author());
    
    //------------------------------------------------------
    // TRAITS AS PARAMETERS
    //------------------------------------------------------
    println!("\n=== Traits as Parameters ===");
    
    // Pass any type that implements Summary to a function
    notify(&article);
    notify(&tweet);
    
    // Function with multiple trait bounds
    notify_multiple(&article);
    
    //------------------------------------------------------
    // TRAIT BOUNDS WITH GENERICS
    //------------------------------------------------------
    println!("\n=== Trait Bounds with Generics ===");
    
    let tweet2 = Tweet {
        username: String::from("another_user"),
        content: String::from("Another tweet content"),
        reply: false,
        retweet: false,
    };
    
    // Pass different types to a generic function with trait bounds
    summarize_pair(&article, &tweet);
    summarize_pair(&tweet, &tweet2);
    
    //------------------------------------------------------
    // RETURNING TYPES THAT IMPLEMENT TRAITS
    //------------------------------------------------------
    println!("\n=== Returning Types That Implement Traits ===");
    
    let returns_summarizable = returns_summarizable(true);
    println!("Returned summary: {}", returns_summarizable.summarize());
    
    //------------------------------------------------------
    // TRAIT BOUNDS WITH WHERE CLAUSES
    //------------------------------------------------------
    println!("\n=== Trait Bounds with Where Clauses ===");
    
    // A more complex function using where clauses
    let pair = Pair { x: 5, y: 10 };
    pair.cmp_display();
    
    //------------------------------------------------------
    // TRAIT OBJECTS
    //------------------------------------------------------
    println!("\n=== Trait Objects ===");
    
    // Create a vector of trait objects
    let mut summary_objects: Vec<Box<dyn Summary>> = Vec::new();
    summary_objects.push(Box::new(article));
    summary_objects.push(Box::new(tweet));
    
    // Iterate through the vector and call trait methods
    for item in summary_objects {
        println!("Summary: {}", item.summarize());
    }
    
    //------------------------------------------------------
    // ASSOCIATED TYPES
    //------------------------------------------------------
    println!("\n=== Associated Types ===");
    
    let counter = Counter::new();
    for item in counter {
        println!("Counter item: {}", item);
    }
}

// Define a trait with a required method
trait Summary {
    // Required method (must be implemented)
    fn summarize(&self) -> String;
    
    // Method with default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
    
    // Default method that calls the required method
    fn notify(&self) -> String {
        format!("Breaking news! {}", self.summarize())
    }
}

// A struct for news articles
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Implement the Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // Override the default implementation
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

// A struct for tweets
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Implement the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// A function that takes any type that implements the Summary trait
fn notify(item: &impl Summary) {
    println!("Notification: {}", item.notify());
}

// A function with multiple trait bounds
fn notify_multiple(item: &(impl Summary + Display)) {
    println!("Notification Display: {}", item);
    println!("Notification Summary: {}", item.summarize());
}

// A generic function with trait bounds
fn summarize_pair<T, U>(t: &T, u: &U)
where
    T: Summary,
    U: Summary,
{
    println!("First summary: {}", t.summarize());
    println!("Second summary: {}", u.summarize());
}

// A function that returns a type that implements a trait
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Function returned an article"),
            location: String::from("Function"),
            author: String::from("Return Type"),
            content: String::from("Content of the returned article"),
        })
    } else {
        Box::new(Tweet {
            username: String::from("function_tweet"),
            content: String::from("This tweet was returned from a function"),
            reply: false,
            retweet: false,
        })
    }
}

// A struct that will use a where clause in implementation
struct Pair<T> {
    x: T,
    y: T,
}

// Implementation with complex trait bounds using where clause
impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// A custom iterator to demonstrate associated types
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Implementing the Iterator trait with an associated type
impl Iterator for Counter {
    // Associated type declaration
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Implementing Display for NewsArticle to satisfy trait bounds
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.headline)
    }
}

//------------------------------------------------------
// EXERCISES
//------------------------------------------------------

// Exercise 1: Drawable Trait
//
// Define a trait called `Drawable` with a required method `draw` that takes no arguments
// and returns nothing. Then implement this trait for at least two different shapes: 
// `Circle` and `Rectangle`. Each shape should print a message describing how it's drawn.

// TODO: Define the Drawable trait
// trait Drawable { ... }

// TODO: Define the Circle struct
// struct Circle { ... }

// TODO: Implement Drawable for Circle
// impl Drawable for Circle { ... }

// TODO: Define the Rectangle struct
// struct Rectangle { ... }

// TODO: Implement Drawable for Rectangle
// impl Drawable for Rectangle { ... }

// Exercise 2: Trait with Default Methods
//
// Define a trait called `Animal` with:
// - A required method `name` that returns a String
// - A required method `species` that returns a String
// - A default method `description` that combines the name and species
// - A default method `speak` that returns a static string

// Then implement this trait for at least two types: `Dog` and `Cat`.
// Override the `speak` method for each to return a specific sound.

// TODO: Define the Animal trait
// trait Animal { ... }

// TODO: Define and implement Dog
// struct Dog { ... }
// impl Animal for Dog { ... }

// TODO: Define and implement Cat
// struct Cat { ... }
// impl Animal for Cat { ... }

// Exercise 3: Trait Bounds
//
// Implement a function called `compare_and_print` that takes two values that
// implement both Display and PartialOrd. The function should print which value
// is larger. Use a where clause for the trait bounds.

// TODO: Implement compare_and_print
// fn compare_and_print<T, U>(t: T, u: U) ... { ... }

// Exercise 4: Associated Types in a Collection Trait
//
// Define a trait called `Collection` with:
// - An associated type `Item`
// - A required method `add` that takes an Item and returns nothing
// - A required method `get` that takes an index and returns an Option<&Item>
// - A required method `length` that returns usize

// Then implement this trait for a simple wrapper around Vec<T>.

// TODO: Define the Collection trait
// trait Collection { ... }

// TODO: Define the List struct and implement Collection for it
// struct List<T> { ... }
// impl<T> Collection for List<T> { ... }

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Exercise 1
    #[test]
    fn test_drawable() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle { width: 10.0, height: 20.0 };
        
        // These should compile and run without errors
        circle.draw();
        rectangle.draw();
    }

    // Tests for Exercise 2
    #[test]
    fn test_animal_traits() {
        let dog = Dog { name: String::from("Buddy"), breed: String::from("Golden Retriever") };
        let cat = Cat { name: String::from("Whiskers"), color: String::from("Tabby") };
        
        assert_eq!(dog.name(), "Buddy");
        assert_eq!(dog.species(), "Canis familiaris");
        assert_eq!(dog.speak(), "Woof!");
        
        assert_eq!(cat.name(), "Whiskers");
        assert_eq!(cat.species(), "Felis catus");
        assert_eq!(cat.speak(), "Meow!");
        
        // Test default implementation
        assert_eq!(dog.description(), "Buddy is a Canis familiaris");
    }

    // Tests for Exercise 3
    #[test]
    fn test_compare_and_print() {
        // This is mostly a compilation test, since the function prints to stdout
        compare_and_print(5, 10);
        compare_and_print("apple", "banana");
    }

    // Tests for Exercise 4
    #[test]
    fn test_collection() {
        let mut list = List::<i32>::new();
        
        assert_eq!(list.length(), 0);
        
        list.add(1);
        list.add(2);
        list.add(3);
        
        assert_eq!(list.length(), 3);
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(5), None);
    }
} 