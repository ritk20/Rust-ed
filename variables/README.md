## Programming Concepts

### Variables

- Variables are immutable by default
- can't be declared globally

### Constants

- They are values bound to a name and can't be changed
- mut can't be used with constants (always immutable)
- type of the value must be annoted
- can be declared in global scope

### Shadowing

- diff between mut and shadow - shadow creates a new variable effectively (type can differ with the original) & basically the variable is immutable after the transformations using let is performed

### Notes on Variables

- rust is statically typed
- signed number stored using [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation
- Rust’s char type is four bytes in size and represents a Unicode scalar value

### Handling integer overflow

- Wrap in all modes with the wrapping\_\* methods, such as wrapping_add.
- Return the None value if there is overflow with the checked\_\* methods.
- Return the value and a Boolean indicating whether there was overflow with the overflowing\_\* methods.
- Saturate at the value’s minimum or maximum values with the saturating\_\* methods.

### Notes on Functions

- snake_case for function and variable naming
- we can define function before or after main fn
- type of parameter must be declared
- Functions = statements + expressions
- semicolon at end of expression turn it into statement (ie, it no longer returns a value)
- numbers are expression

### Notes on Control Statement

- conditions can only be bool
- if-else is an expression
- loop, while, for
