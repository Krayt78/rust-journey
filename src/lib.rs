//! # Rust Journey
//! 
//! A comprehensive learning journey through Rust programming concepts.

/// Module containing fundamental Rust concepts
pub mod fundamentals {
    /// Hello World examples and exercises
    pub mod hello_world {
        include!("../chapters/01_fundamentals/01_hello_world/challenge.rs");
    }
    
    /// Variables and mutability examples and exercises
    pub mod variables {
        include!("../chapters/01_fundamentals/02_variables/challenge.rs");
    }
    
    /// Data types examples and exercises
    pub mod data_types {
        /// Scalar types (integers, floats, booleans, characters)
        pub mod scalar_types {
            /// Integer examples and exercises
            pub mod integers {
                include!("../chapters/01_fundamentals/03_data_types/01_scalar_types/0_integers.rs");
            }
            
            /// Floating point examples and exercises
            pub mod floating_point {
                include!("../chapters/01_fundamentals/03_data_types/01_scalar_types/1_floating_point.rs");
            }
            
            /// Boolean examples and exercises
            pub mod boolean {
                include!("../chapters/01_fundamentals/03_data_types/01_scalar_types/2_boolean.rs");
            }
            
            /// Character examples and exercises
            pub mod character {
                include!("../chapters/01_fundamentals/03_data_types/01_scalar_types/3_character.rs");
            }
        }
        
        /// Compound types (tuples, arrays, slices)
        pub mod compound_types {
            /// Tuple examples and exercises
            pub mod tuples {
                include!("../chapters/01_fundamentals/03_data_types/02_compound_types/0_tuples.rs");
            }
            
            /// Array examples and exercises
            pub mod arrays {
                include!("../chapters/01_fundamentals/03_data_types/02_compound_types/1_arrays.rs");
            }
            
            /// Slice examples and exercises
            pub mod slices {
                include!("../chapters/01_fundamentals/03_data_types/02_compound_types/2_slices.rs");
            }
        }
    }
    
    /// Functions examples and exercises
    pub mod functions {
        /// Basic function examples and exercises
        pub mod basic_functions {
            include!("../chapters/01_fundamentals/04_functions/0_basic_functions.rs");
        }
        
        /// Function parameters examples and exercises
        pub mod function_parameters {
            include!("../chapters/01_fundamentals/04_functions/1_function_parameters.rs");
        }
        
        /// Return values examples and exercises
        pub mod return_values {
            include!("../chapters/01_fundamentals/04_functions/2_return_values.rs");
        }
        
        /// Function expressions examples and exercises
        pub mod function_expressions {
            include!("../chapters/01_fundamentals/04_functions/3_function_expressions.rs");
        }
        
        /// Advanced functions examples and exercises
        pub mod advanced_functions {
            include!("../chapters/01_fundamentals/04_functions/4_advanced_functions.rs");
        }
    }
    
    /// Control flow examples and exercises
    pub mod control_flow {
        /// If expressions examples and exercises
        pub mod if_expressions {
            include!("../chapters/01_fundamentals/05_control_flow/1_if_expressions.rs");
        }
        
        /// Loop expressions examples and exercises
        pub mod loop_expressions {
            include!("../chapters/01_fundamentals/05_control_flow/2_loop_expressions.rs");
        }
        
        /// While loops examples and exercises
        pub mod while_loops {
            include!("../chapters/01_fundamentals/05_control_flow/3_while_loops.rs");
        }
        
        /// For loops examples and exercises
        pub mod for_loops {
            include!("../chapters/01_fundamentals/05_control_flow/4_for_loops.rs");
        }
        
        /// Match expressions examples and exercises
        pub mod match_expressions {
            include!("../chapters/01_fundamentals/05_control_flow/5_match_expressions.rs");
        }
        
        /// If let and while let expressions examples and exercises
        pub mod if_while_let {
            include!("../chapters/01_fundamentals/05_control_flow/6_if_while_let.rs");
        }
    }
}

/// Module containing core Rust concepts
pub mod core_concepts {
    // This will be populated as needed when content is available
}

/// Module for testing-related examples and exercises
pub mod testing {
    // This will be populated with content from the testing chapter
}
