# Variables

## Keywords

    - const
    - let
    - mut

## Difference between let & const

    - variables declared with const must be assigned at compile time. (their TYPE and VALUE must be known at compile time)
    - variables declared with let, can be assigned at runtime

    RULE: const are always immutable and cannot be changed after being defined.

## let variables

    RULE: You always need to define the variable.

    - you can declare let variables this way:

        1) let variable_name = value;
        2) let mut variable_name = value;

    - on 1) way, the variable is immutable.
    - on 2) way, the variable is mutable.

## Type inference

    - the variable type can be inferred, e.g.

        1) const name: str = "Jonas";
        2) let age: u8 = 21;

## VARIABLE SCOPE

    - variables follow block scope