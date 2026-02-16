# Error handling

## Princicle

Every fallible function must return an error with enough data for the caller to retry the call.

## Guidelines

* Use `handle!` instead of `?` try operator to unwrap `Result` types
* Use `handle!` instead of `Result::map_err`
* Use `handle_opt!` instead of `?` try operator to unwrap `Option` types
* Use `handle_opt!` instead of `Option::ok_or` and `Option::ok_or_else`
* Use `handle_bool!` instead of `if condition { return Err(...) }` to return an error if some condition is true
* Use `handle_iter!` or `handle_iter_of_refs!` to collect and return errors from iterators
* Use `handle_into_iter!` to handle errors in collections that implement `IntoIterator` (including `Vec` and `HashMap`)
* Calls to macros that begin with `handle` must not contain calls to `clone` (must not contain `.clone()`)
  * Rationale: there is no need to clone the variables because the macros consume them only in the error branch, and the error branch contains a `return` statement. The variables are not consumed in the success branch, so you can always use them in the subsequent code.
* Don't convert a `Result` into an `Option`, always propagate the error up the call stack
* Don't use `unwrap` or `expect`
* Don't return strings as errors
* Every fallible function must return a unique error type, even if it contains only one fallible expression
* Every call to another fallible function must be wrapped in a unique error enum variant
* Every fallible function body must begin with `use ThisFunctionError::*;`, where `ThisFunctionError` must be the name of this function's error enum (for example: `use ParseConfigError::*;`)
* Every fallible function body must use the error enum variant names without the error enum name prefix (for example: `ReadFileFailed` instead of `ParseConfigError::ReadFileFailed`)
* Every error type must be an enum
* Every error type must derive `Error` via `thiserror` v2
* Every error type must be located below the function that returns it (in the same file)
* Every error enum variant must be a struct variant
* Every error enum variant must contain one field per owned variable that is relevant to the fallible expression that this variant wraps
  * The relevant variable is a variable whose value determines whether the fallible expression returns an `Ok` or an `Err`
* Every error enum variant must have fields only for [`data types`](#data-type), not for [`non-data types`](#non-data-type)
* Every error enum variant must have an `#[error]` attribute
  * The `#[error]` attribute must contain the error message displayed for the user
  * The `#[error]` attribute must not contain the `source` field
  * The `#[error]` attribute should contain only those fields that can be displayed on one line
  * If the `#[error]` attribute contains fields that implement `Display`, then those fields must be output using `Display` formatting (not `Debug` formatting)
    * Good:
      ```rust
      #[derive(Error, Debug)]
      pub enum QueryFailed {
          #[error("task not found for query '{query}'")]
          TaskNotFound { query: String }
      }
      ```
    * Bad:
      ```rust
      #[derive(Error, Debug)]
      pub enum QueryFailed {
          #[error("task not found for query '{query:?}'")]
          TaskNotFound { query: String }
      }
      ```
  * If the `#[error]` attribute contains fields, then those fields must be wrapped in single quotes. This is necessary to correctly display fields that may contain spaces.
    * Good: `#[error("user '{name}' not found")]`
    * Bad: `#[error("user {name} not found")]`
* If the error enum variant has a `source` field, then this field must be the first field
* If each field of each variant of the error enum implements `Copy`, then the error enum must implement `Copy` too
* Every error enum variant field must have an owned type (not a reference)
* Every error enum variant field must not have a `#[from]` attribute
* Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the `Debug` and `Display` impls (e.g. `secrecy::SecretBox`)
* The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
* If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
* If an argument of callee implements `Copy`, the callee should not include it in the list of error enum variant fields (the caller must include it because of the rule to include all relevant owned variables)
* If you see a function that returns a `Result` whose last argument is `()` (e.g. `Result<(), ()>`, `Result<T, ()>`, `Result<u32, ()>`), then you must fix the error handling in this function according to the guidelines and replace `()` with a proper error type

### Naming

* The name of the error enum must end with `Error` (for example: `ParseConfigError`)
* The name of the error enum variant should end with `Failed` or `NotFound` or `Invalid` (for example: `ReadFileFailed`, `UserNotFound`, `PasswordInvalid`)
* If the error variant name is associated with a child function call, the name of the error variant must be equal to the name of the function converted to CamelCase concatenated with `Failed` (for example: if the parent function calls `read_file`, then it should call it like this: `handle!(read_file(&path), ReadFileFailed, path)`
* The name of the error enum must include the name of the function converted to CamelCase
  * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
  * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
  * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
* If the error enum is defined for a `TryFrom<A> for B` impl, then its name must be equal to "Convert{A}To{B}Error"

## Definitions

### Fallible expression

An expression that returns a `Result`.

### Data type

A type that holds the actual data.

Examples:

* `bool`
* `String`
* `PathBuf`

### Non-data type

A type that doesn't hold the actual data.

Examples:

* `RestClient` doesn't point to the actual data, it only allows querying it.
* `DatabaseConnection` doesn't hold the actual data, it only allows querying it.
