# TODO

## Implement fallible groups

* Design an example in `crate::macros::tests` that would show how to handle the groups of fallible expressions
  * Recall the `FallibleMarket` example
  * Expand this example to include multiple stages (i.e. some expressions depending on the results of those expressions)
    * Notes:
      * Maybe the code can always be refactored in a way that there is only one fallible expression group per function (this can be done by refactoring the dependent expression chains into a separate function that would return a temporary struct or a tuple that is used only for aggregating the values)
