// Hash Maps in Rust
//
// This file demonstrates how to use Rust's HashMap collection,
// which stores key-value pairs with efficient lookups by key.

use std::collections::HashMap;

fn main() {
    println!("Understanding Hash Maps in Rust!");
    
    //------------------------------------------------------
    // CREATING HASH MAPS
    //------------------------------------------------------
    println!("\n=== Creating Hash Maps ===");
    
    // Creating an empty hash map
    let mut scores = HashMap::new();
    println!("Empty hash map created: {:?}", scores);
    
    // Adding key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("After inserting values: {:?}", scores);
    
    // Creating a hash map from iterators
    let teams = vec![String::from("Yellow"), String::from("Green")];
    let initial_scores = vec![25, 55];
    
    // Using zip and collect to create a hash map
    let team_scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    println!("Hash map from iterators: {:?}", team_scores);
    
    // Creating with initial capacity
    let mut map_with_capacity = HashMap::with_capacity(10);
    map_with_capacity.insert("first", 1);
    map_with_capacity.insert("second", 2);
    println!("Map with capacity: {:?}", map_with_capacity);
    
    //------------------------------------------------------
    // ACCESSING VALUES
    //------------------------------------------------------
    println!("\n=== Accessing Values ===");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    // Using get - returns Option<&V>
    let blue_score = scores.get(&String::from("Blue"));
    match blue_score {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    // Using get with unwrap (panics if key doesn't exist)
    let red_score = scores.get(&String::from("Red")).unwrap();
    println!("Red team score: {}", red_score);
    
    // Safer access with if let
    if let Some(score) = scores.get(&String::from("Yellow")) {
        println!("Yellow team score: {}", score);
    } else {
        println!("Yellow team not found");
    }
    
    // Using the index operator (panics if key doesn't exist)
    let blue_score = scores[&String::from("Blue")];
    println!("Blue team score using index: {}", blue_score);
    
    // The following would panic:
    // let yellow_score = scores[&String::from("Yellow")];
    
    //------------------------------------------------------
    // UPDATING HASH MAPS
    //------------------------------------------------------
    println!("\n=== Updating Hash Maps ===");
    
    let mut scores = HashMap::new();
    
    // Inserting values
    scores.insert(String::from("Blue"), 10);
    println!("After inserting Blue: {:?}", scores);
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("After overwriting Blue: {:?}", scores);
    
    // Using entry API to insert only if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't change Blue's value
    println!("After using entry API: {:?}", scores);
    
    // Updating a value based on old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}", word_count);
    
    //------------------------------------------------------
    // REMOVING ENTRIES
    //------------------------------------------------------
    println!("\n=== Removing Entries ===");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Yellow"), 30);
    
    println!("Before removal: {:?}", scores);
    
    // Remove a key-value pair
    scores.remove(&String::from("Red"));
    println!("After removing Red: {:?}", scores);
    
    // Remove with return value
    if let Some(score) = scores.remove(&String::from("Blue")) {
        println!("Removed Blue with score: {}", score);
    }
    
    println!("After both removals: {:?}", scores);
    
    // Clear all entries
    scores.clear();
    println!("After clearing: {:?}", scores);
    
    //------------------------------------------------------
    // ITERATING OVER HASH MAPS
    //------------------------------------------------------
    println!("\n=== Iterating Over Hash Maps ===");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Yellow"), 30);
    
    // Iterating over key-value pairs
    println!("Key-value pairs:");
    for (key, value) in &scores {
        println!("  {} team: {}", key, value);
    }
    
    // Iterating over just keys
    println!("Keys only:");
    for key in scores.keys() {
        println!("  {}", key);
    }
    
    // Iterating over just values
    println!("Values only:");
    for value in scores.values() {
        println!("  {}", value);
    }
    
    // Iterating and modifying values
    println!("Modifying values during iteration:");
    for (_, value) in scores.iter_mut() {
        *value += 5;
    }
    println!("After modification: {:?}", scores);
    
    //------------------------------------------------------
    // OWNERSHIP AND HASH MAPS
    //------------------------------------------------------
    println!("\n=== Ownership and Hash Maps ===");
    
    // Types that implement Copy
    let k1 = 1;
    let v1 = 100;
    
    let mut map1 = HashMap::new();
    map1.insert(k1, v1);
    println!("k1: {}, v1: {}", k1, v1); // k1 and v1 are still valid
    
    // Types that don't implement Copy (like String)
    let k2 = String::from("key");
    let v2 = String::from("value");
    
    let mut map2 = HashMap::new();
    map2.insert(k2, v2);
    // The following lines would error because ownership has moved:
    // println!("k2: {}, v2: {}", k2, v2);
    
    // Referencing values
    let k3 = String::from("key");
    let v3 = String::from("value");
    
    let mut map3 = HashMap::new();
    map3.insert(&k3, &v3);
    println!("k3: {}, v3: {}", k3, v3); // k3 and v3 are still valid
    
    //------------------------------------------------------
    // COMPLEX VALUES IN HASH MAPS
    //------------------------------------------------------
    println!("\n=== Complex Values in Hash Maps ===");
    
    // Hash map with Vec values
    let mut scores_by_quarter = HashMap::new();
    scores_by_quarter.insert(
        String::from("Blue"),
        vec![7, 3, 10, 15],
    );
    scores_by_quarter.insert(
        String::from("Red"),
        vec![12, 8, 5, 10],
    );
    
    println!("Scores by quarter: {:?}", scores_by_quarter);
    
    // Hash map with struct values
    #[derive(Debug)]
    struct Player {
        name: String,
        position: String,
    }
    
    let mut team_players = HashMap::new();
    team_players.insert(
        String::from("Blue"),
        vec![
            Player {
                name: String::from("Alice"),
                position: String::from("Forward"),
            },
            Player {
                name: String::from("Bob"),
                position: String::from("Defense"),
            },
        ],
    );
    
    println!("Team players: {:?}", team_players);
    
    //------------------------------------------------------
    // HASH MAP PERFORMANCE AND CUSTOMIZATION
    //------------------------------------------------------
    println!("\n=== Hash Map Performance ===");
    
    // HashMap uses a default hashing algorithm
    let mut map = HashMap::new();
    
    // Basic stats
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    println!("Map size: {}", map.len());
    println!("Map capacity: {}", map.capacity());
    
    // Reserving space
    map.reserve(10);
    println!("Map capacity after reserve: {}", map.capacity());
    
    // Shrinking to fit
    map.shrink_to_fit();
    println!("Map capacity after shrink_to_fit: {}", map.capacity());
}

//------------------------------------------------------
// CHALLENGES
//------------------------------------------------------

// Challenge 1: Word Counter
//
// Implement the `count_words` function that takes a piece of text and
// returns a HashMap with each word as a key and its count as a value.
// Words should be case-insensitive (e.g., "The" and "the" count as the same word).
// Punctuation should be ignored.

fn count_words(text: &str) -> HashMap<String, u32> {
    // TODO: Implement this function.
    // Count the occurrences of each word in the text.
    // Words should be case-insensitive and punctuation should be ignored.
    // Return a HashMap with each word as a key and its count as a value.
    
    HashMap::new() // Replace this placeholder
}

// Challenge 2: Group By
//
// Implement the `group_by_first_letter` function that takes a vector of strings
// and returns a HashMap with the first letter as a key and a vector of strings
// that start with that letter as a value.
// The first letter should be lowercase.

fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    // TODO: Implement this function.
    // Group the words by their first letter.
    // The first letter should be lowercase.
    // Return a HashMap with each first letter as a key and a vector of strings as a value.
    
    HashMap::new() // Replace this placeholder
}

// Challenge 3: Merge HashMaps
//
// Implement the `merge_maps` function that takes two HashMaps and returns
// a new HashMap that contains all key-value pairs from both input maps.
// If a key exists in both maps, the value from the second map should be used.

fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    // TODO: Implement this function.
    // Merge the two maps into a new map.
    // If a key exists in both maps, use the value from map2.
    // Return the merged map.
    
    HashMap::new() // Replace this placeholder
}

// Challenge 4: Student Records
//
// Implement a function that manages student grades for different subjects.
// The function should support:
// - Adding a grade for a student in a subject
// - Getting the average grade for a student across all subjects
// - Getting the average grade for a subject across all students

struct GradeBook {
    // TODO: Define the structure to store student grades
    // Hint: You might want to use nested HashMaps
}

impl GradeBook {
    fn new() -> GradeBook {
        // TODO: Initialize and return a new GradeBook
        GradeBook {}
    }
    
    fn add_grade(&mut self, student: &str, subject: &str, grade: f32) {
        // TODO: Add a grade for a student in a subject
    }
    
    fn get_student_average(&self, student: &str) -> Option<f32> {
        // TODO: Calculate the average grade for a student across all subjects
        // Return None if the student doesn't exist
        None
    }
    
    fn get_subject_average(&self, subject: &str) -> Option<f32> {
        // TODO: Calculate the average grade for a subject across all students
        // Return None if the subject doesn't exist
        None
    }
}

//------------------------------------------------------
// TESTS
//------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_words() {
        let text = "The quick brown fox jumps over the lazy dog. The dog barks, but the fox runs away.";
        let counts = count_words(text);
        
        assert_eq!(counts.get("the").unwrap(), &3);
        assert_eq!(counts.get("fox").unwrap(), &2);
        assert_eq!(counts.get("dog").unwrap(), &2);
        assert_eq!(counts.get("jumps").unwrap(), &1);
        assert_eq!(counts.get("barks").unwrap(), &1);
        assert_eq!(counts.len(), 11); // 11 unique words
    }
    
    #[test]
    fn test_group_by_first_letter() {
        let words = vec![
            String::from("Apple"),
            String::from("Banana"),
            String::from("Apricot"),
            String::from("Cherry"),
            String::from("Blueberry"),
        ];
        
        let grouped = group_by_first_letter(words);
        
        assert_eq!(grouped.len(), 3); // 3 different first letters: a, b, c
        assert_eq!(grouped.get(&'a').unwrap().len(), 2); // "Apple", "Apricot"
        assert_eq!(grouped.get(&'b').unwrap().len(), 2); // "Banana", "Blueberry"
        assert_eq!(grouped.get(&'c').unwrap().len(), 1); // "Cherry"
    }
    
    #[test]
    fn test_merge_maps() {
        let mut map1 = HashMap::new();
        map1.insert(String::from("a"), 1);
        map1.insert(String::from("b"), 2);
        map1.insert(String::from("c"), 3);
        
        let mut map2 = HashMap::new();
        map2.insert(String::from("b"), 20);
        map2.insert(String::from("c"), 30);
        map2.insert(String::from("d"), 40);
        
        let merged = merge_maps(map1, map2);
        
        assert_eq!(merged.len(), 4); // a, b, c, d
        assert_eq!(*merged.get("a").unwrap(), 1);
        assert_eq!(*merged.get("b").unwrap(), 20); // From map2
        assert_eq!(*merged.get("c").unwrap(), 30); // From map2
        assert_eq!(*merged.get("d").unwrap(), 40);
    }
    
    #[test]
    fn test_grade_book() {
        let mut grade_book = GradeBook::new();
        
        // Add grades
        grade_book.add_grade("Alice", "Math", 90.0);
        grade_book.add_grade("Alice", "Science", 85.0);
        grade_book.add_grade("Bob", "Math", 80.0);
        grade_book.add_grade("Bob", "Science", 90.0);
        
        // Test student averages
        assert_eq!(grade_book.get_student_average("Alice").unwrap(), 87.5); // (90 + 85) / 2
        assert_eq!(grade_book.get_student_average("Bob").unwrap(), 85.0); // (80 + 90) / 2
        assert_eq!(grade_book.get_student_average("Charlie"), None); // Student doesn't exist
        
        // Test subject averages
        assert_eq!(grade_book.get_subject_average("Math").unwrap(), 85.0); // (90 + 80) / 2
        assert_eq!(grade_book.get_subject_average("Science").unwrap(), 87.5); // (85 + 90) / 2
        assert_eq!(grade_book.get_subject_average("History"), None); // Subject doesn't exist
    }
} 