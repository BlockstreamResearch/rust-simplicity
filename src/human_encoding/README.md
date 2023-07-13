# Simplicity Human-Readable Encoding

This module defines a human-readable encoding for Simplicity programs. This encoding
is intended to be the encoding used for storage and interchange of "commitment-time"
Simplicity programs, i.e. programs which are unpruned and have no witnesses.

The following parts of the encoding are incomplete/undesigned:

1. It does not support witness data; in future we would like to support partial/full
   witness population, as well as population of disconnected expressions.
2. `2^n` when it appears in types is a single lexer token and you cannot put spaces
   into it. I'm not sure if I want to fix this or not.

With that said, the rest of the document defines the encoding.

## Syntax

The syntax is defined in `src/human_encoding/parse/ast.rs`. It currently uses the
`santiago` parser generator, but we would like to move away from this, probably to
an ad-hoc parser, to avoid poor asymptotic behavior and to get better error messages.

Comments are started by `--` and end at the next newline. This is the only aspect
in which whitespace is significant.

Simplicity expressions are composed as a series of **definitions** of the form:

    NAME := EXPRESSION

and **type bounds** of the form

    NAME : TYPE -> TYPE

where these may be combined into the singular form `NAME := EXPRESSION : TYPE -> TYPE`.
Whitespace is not significant. Each definition or type bound is self-delimiting, so
there are no semicolons or other separators, but by convention each one should be
separated by at least one newline.

Here NAME is

* Any sequence matching the regex `[a-zA-Z_\-.'][0-9a-zA-Z_\-.']*`; i.e. combination
  of alphanumerics, `-`, `_`, `'`, and `.` that does not start with a numeral;
* EXCEPT for the following reserved symbols, which may not be used:
    * `_`;
    * `assertl`, `assertr`, `case`, `comp`, `const`, `disconnect`, `drop`, `fail`, `iden`, `injl`, `injr`, `pair`, `take`, `unit`, `witness`;
    * anything beginning with `prim`; and
    * anything beginning with `jet_`.

and EXPRESSION is

* a NAME;
* a HOLE (defined below);
* `unit`, `iden`, or `witness`;
* `injl`, `injr`, `take`, or `drop` followed by another EXPRESSION;
* `case`, `comp`, or `pair` followed by two EXPRESSIONs;
* `assertl` followed by an EXPRESSION, a literal `#`, and another EXPRESSION;
* `assertr` followed by a literal `#` and two EXPRESSIONs;
* a jet, which begins with `jet_` and must belong to the list of jets (FIXME define this list);
* `const` followed by a VALUE (defined below);
* `fail` followed by an ENTROPY (defined below); or
* `(` followed by another EXPRESSION followed by `)`.

Note that while we allow parenthesis to help group parts of expressions for human
understanding, they are never needed for disambiguation and are essentially
ignored by the parser.

A HOLE is the literal `?` followed by a NAME. It indicates an expression that has
yet to be defined. Holes have a different namespace than other names.

A VALUE is one of
* the literal `_`, which is interpreted as the empty value;
* a binary literal `0b[01]+` which is interpreted as a sequence of bits; or
* a hex literal `0x[0-9a-f]+` which is interpreted as a sequence of 4-bit nybbles

An ENTROPY is a VALUE whose size is between 128 and 512 bits inclusive. Internally
it is 0-padded out to 512 bits.

Finally, TYPE is

* a literal `_`, indicating no type bound;
* a NAME;
* a literal `1`, indicating the unit type;
* a literal `2`, indicating the bit type;
* `2^n`, where `n` is any power of two, in decimal with no spaces or punctuation;
* `(` followed by another TYPE followed by `)`;
* another TYPE, followed by `+`, followed by another TYPE; or
* another TYPE, followed by `*`, followed by another TYPE.

Here `*` has higher precedence than `+`, and both `+` and `*` are left-associative.

## Namespaces

There are three independent namespaces: one for NAMEs, one for HOLEs, and one for
TYPEs. They all have the same rules for valid symbols, except that `_` is reserved
(may not be used) for NAMEs and `_` has a special meaning (no type bound) for
TYPEs.

## Semantics: Definitions

As above, Simplicity expressions are a series of **definitions** of the form

    NAME := EXPRESSION

or

    NAME := EXPRESSION : TYPE -> TYPE

and **type bounds** of the form

    NAME : TYPE -> TYPE

We refer to the `NAME` part as the **name**, `EXPRESSION` as the **expression**,
and `TYPE -> TYPE` as the **type ascription**. If such a named expression appears anywhere
in a Simplicity encoding, then whenever that name appears in the expression of
any other named expression, its expression is substituted in place of it.

For a given name, it is permissible to have multiple type bounds, but any name
which appears must have exactly one definition. That is, it is not permitted to
have a type bound for a name which isn't defined elsewhere, and it is not permitted
to have multiple definitions for the same name.

This allows the user to provide any number of type bounds for a given name, each of
which may be helpful in clarifying a program.

For example, in the encoding

    node := unit : _ -> 1
    main := comp node node

the name `node` is substituted by `unit` both places that it appears. We can see
that starting from the name `main`, by recursive substitution, we obtain a single
Simplicity expression. The type checker will ensure that the target type of `node`
is 1.

In general, we do not need to obtain a single expression. It is permissible to
encode a "DAG forest".

The name `main` is special for several reasons:
* An expression named `name` implicitly has the type ascription `1 -> 1`. That is, it must always be a program.
* To generate a commitment-time program from an expression, it must be called `main`.
* Type ascriptions for `main` and its children are enforced **after** type inference has completed, so they act as sanity checks but cannot change the output of type inference. For other expressions, type ascriptions are enforced **before** and may guide inference.

No cycles are allowed; if a name occurs anywhere in the expansion of its expression,
this is an error.

## Semantics: Expressions

Expressions may be

* a NAME, which simply refers to another expression;
* a HOLE, which is described in the next section;
* one of the core combinators `unit`, `iden`, `comp`, `injl`, `injr`, `case`, `take`, `drop`, `pair`, followed by subexpression(s) as needed;
* the `disconnect` combinator followed by an expression and a hole;
* the `witness` combinator which currently allows no subexpressions;
* the assertions, `assertl` or `assertr`, which take two subexpressions, one of which will be hidden in the decoded program. The hidden subexpression should be prefixed by `#` which indicates to the parser to take the CMR of that expression, not the expression itself.
* `fail` followed by a 128-to-512-bit entropy value, which should occur only in the pruned branch of an assertion, though this is not enforced;
* `const` followed by a value, which is a "constant-word jet" and is equivalent to constructing the given value by a tree of `pair`s whose leaves are `injl unit` (0) or `injr unit` (1);

Expressions have an associated **type arrow**, which is inferred by the type checker as
expressions are built up. If a combinator's children's types are incompatible for that
combinator, an error is raised.

After the type arrow for a named expression is fully inferred, any type ascriptions for
that name are applied, and an error is raised if this fails. In this way, a user can
provide type ascriptions which act as sanity checks and documentation for sub-parts of
a Simplicity expression.

## Semantics: Holes

Holes are of the form `?NAME`; there may be whitespace after the `?` but by convention it is
omitted. Holes must have unique names, but live in a separate namespace from ordinary names,
so they cannot conflict with the names of expressions.

Holes have two meanings:
* When they occur as the right child of a `disconnect` combinator, they give a name to a disconnected expression. `disconnect` combinators are **required** to have holes for right children. Any other expression form is an error.
* In all other contexts, they indicate an incomplete part of the program which can be typechecked but not much else.

In all cases, holes are typechecked before any errors are reported, and the assembler will
report their types. This allows the use of holes as placeholders during development, where
users can learn the required type of the eventual expression by querying the typechecker.

When assembling or computing CMRs of incomplete programs (any program with a hole outside
of the right child of a `disconnect` node), errors will be reported for every hole. 
error messages will include the holes' type arrows.

## Semantics: Type Ascriptions

Type ascriptions are of the form `TYPE -> TYPE`. We refer to the first type as the **source
type** and the second as the **target type**. Here each TYPE is one of

* the literal `_`, which indicates that no additional checks should be done on the appropriate type;
* the literals `1`, `2` or `2^n` indicate that the appropriate type must be the unit, bit, or n-bit word type;
* an arbitrary NAME, which simply gives a name to the appropriate type. (If the same name is used in multiple places, the type-checker will check that the same type appears in each place.);
* any pair of TYPEs separated by `+` or `*`, which indicate a sum or product bound respectively; or
* any TYPE surrounded by `(` or `)`.

Note that the NAMEs used for types are in a separate namespace from the NAMEs used in
named expression, and from HOLEs. These three uses of names do not interact.

Note also that unlike the case for EXPRESSIONs, parentheses may be meaningful. Absent parentheses,
`*` has higher precedence than `+` and both operators are left-associative.

The interpretation of type ascriptions depends on whether they appear within the `main` expression.
If so, type inference is completed and all free types set to unit, and **then** all type ascriptions
are applied. In this case, the type ascriptions cannot change the final types, but if they are
incompatible with the final types, an error is raised.

Outside of `main`, type ascriptions are applied in parallel with type inference, and before free
types are set to unit. This means that type ascription can change types that would otherwise be
free, and the type of the resulting expression will not necessarily match its type if the
expression were to be pulled into `main`.

