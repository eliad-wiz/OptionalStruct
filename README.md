# OptionalStruct
[![Crates.io](https://img.shields.io/crates/v/optional_struct.svg)](https://crates.io/crates/optional_struct)

## Quick-start


```rust
#[optional_struct]
#[derive(Bar)]
struct Foo {
	meow: u32,
	woof: String,
}

// Generated:
#[derive(Clone, Debug, PartialEq, Bar)]
struct OptionalFoo {
	meow: Option<u32>,
	woof: Option<String>,
}

impl OptionalFoo {
	pub fn apply_to(self, foo: &mut OptionalFoo) {
        if let Some(meow) = self.meow {
            foo.meow = meow;
        }

        if let Some(woof) = self.woof {
            foo.woof = woof;
        }
	}
}

fn main() {
    let mut f = Foo {
        meow: 4,
        woof: "I am hungry rn".to_owned(),
    };

    let opt_f = OptionalFoo {
        meow: Some(10),
        woof: None,
    };

    opt_f.apply_to(&mut f);

    assert_eq!(f.meow, 10);
    assert_eq!(&f.woof, "I am hungry rn");
}
```

## Goal

Since rust does not have default arguments, and some tools are strict when
deserializing data (e.g. serde), missing configuration values can be quite
frustrating to deal with. For example:

```rust
#[derive(Deserialize)]
struct Config {
    log_file: PathBuf,
}
```

If we read the configuration from a file, and the `log_file` is not specified,
serde will e.g. fail to create the struct. While serde [offers ways to set the
default value for a field](https://serde.rs/attr-default.html) with e.g.

```rust
#[derive(Deserialize)]
struct Config {
    #[serde(default = "get_next_log_filename")]
    log_file: PathBuf,
}
```

there are obvious limitations. This crate aims to fill this gap by allowing
optional values, and providing an easy way to apply values obtained from
different sources to construct our configuration.

With `optional_struct`, one can define the required
configuration as it shall be used and only use the generated struct
to handle configuration/missing values/default values.


## How

The macro `optional_struct` generates a structure containing the same fields as the one it was tagged on, but wrapped by an `Option`.
A function on the new structure allows applying its values to the original one
(if the `Option`s are not `None`). This can be called multiple times, to apply
configuration from different source, while giving the caller complete control
over how to set the values, since the generated struct can be easily manipulated
and passed around before constructing the final configuration.

## Features

1. Rename the generated struct:

```rust
#[optional_struct(HeyU)]
struct Config();

let me = HeyU();
```

2. Handle recursive types:

```rust
#[optional_struct]
struct Foo {
    // Replaces Option<Bar> with Option<OptionalBar>
    #[optional_rename(OptionalBar)]
    bar: Bar,
}

#[optional_struct]
struct bar ();
```

3. Handle `Option`s in the original struct (by ignoring them):

```rust
#[optional_struct]
struct Foo {
    bar: Option<u8>,
}

let opt_f = OptionalFoo { bar: Some(1) }
```

4. Force wrapping (or not) of fields:

```rust
#[optional_struct]
struct Foo {
    #[optional_skip_wrap]
    bar: char,

    // Useless here since we wrap by default
    #[optional_wrap]
    baz: bool,
}

let opt_f = OptionalFoo { bar: Some(1) }
```

5. Change the default wrapping behavior:

```rust
#[optional_struct(OptionalFoo, false)]
struct Foo {
    bar: u8,
    #[optional_wrap]
    baz: i8,
}

let opt_f = OptionalFoo { bar: 1, baz: None };
```

## `apply_to`

The simplest way to understand how this function is generated is to look at the
`Applyable` helper trait we define in this crate:

```rust
pub trait Applyable<T> {
    fn apply_to(self, t: &mut T);
}

impl<T> Applyable<T> for Option<T> {
    fn apply_to(self, t: &mut T) {
        if let Some(s) = self {
            *t = s;
        }
    }
}

impl<T> Applyable<T> for T {
    fn apply_to(self, t: &mut T) {
        *t = self;
    }
}
```

Each structure generated by `optional_struct` implements `Applyable<Foo>` for
`OptionalFoo`. This implementation itself calls `apply_to` recursively on all
fields of the structure. Besides those implementations, the ones defined just
above tell you how this function behaves: values are applied when they exist,
options are simply overridden.

## TODOs / open questions

1. Allow *not* overridding `Option<T> -> Option<T>` if the original `Option` is
   set but not the applied one.

2. Allow wrapping behavior to be set without manually writing the struct name.

3. Automatically find out which fields have an associated `optional_struct`
   structure (and avoid the `optional_rename`). This might require better
   support for associated types? Could not get it to work with only constant
   generics :(

4. Make generated fields public by default? For now the fields have the same
   visibility as the original struct, and this is generated locally, so this
   seems to be a minor detail?

5. Control which `derive` attributes are tagged onto the generated struct. Right
   now, they are copied verbatim, and then some are added (`PartialEq / Default
   / Clone / Debug`). Problem: conflicting implementation, missing impl. (e.g.
   serde's Ser/Deser), invalid impl (e.g. if `Default` is missing on some
   `skip_wrap` field) etc.
