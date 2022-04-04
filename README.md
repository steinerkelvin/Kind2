Kind2
=====

This is a prototype of [Kind-Lang](https://github.com/kindelia/kind)'s
successor. It does NOT include all of the Kind1 features yet, and won't for a
while, but it already has the building blocks to create programs and proofs. It
is designed to be maximally compatible with
[HVM](https://github.com/kindelia/hvm). New features were added with that goal
in mind.

Usage
-----

### 1. Install it

First, install [Rust](https://www.rust-lang.org/). Then, type:

```sh
cargo install kind2
```

### 2. Create a Kind2 file

Kind2 files look like [HVM](https://github.com/kind2/. Save the file below as `main.hvm`:

```javascript
// Booleans
Bool : Type
  True  : Bool
  False : Bool

// Negates a boolean
(Bool.Not b:Bool) : Bool
  (Bool.Not True)  = False
  (Bool.Not False) = True

// Double Negation Theorem
(Bool.DNT b:Bool) : (Bool.Not (Bool.Not b)) == b
  (Bool.DNT True)  = refl True
  (Bool.DNT False) = refl False
```


### 3. Type-check it

```sh
kind2 check main.kind2
```

### 4. Compile it to HVM

```
... TODO ...
```
