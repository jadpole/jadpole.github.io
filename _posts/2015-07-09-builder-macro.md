---
layout: post
title:  "A Rust macro for the builder pattern"
categories: rust
---

Real-world(ish) problem: you are developing, in Rust, a
[Warcraft](https://www.wikiwand.com/fr/Warcraft_III:_Reign_of_Chaos)-like
game which uses very dense data structures for the buildings and the mobs. Your
team eventually decides to use the _builder pattern_ to ease the pain of
writing and maintaining constructors with 20-ish variables.

The two data structures which we are going to study today are reduced versions
of the Orc and of the Farm, which you use to verify that the builder pattern
really makes things easier in practice. Their code looks like this:

```rust
struct Orc {
    name: String,
    health: i32,
    level: i32,
    experience: i32,
    position: (i32, i32),
    weapon: Box<Weapon> // where Weapon is a trait
}

impl Orc {
    // ...
}

struct Farm {
    health: i32,
    level: i32,
    experience: i32,
    position: (i32, i32),
    kilograms_every_hour: i32,
}

impl Farm {
    // ...
}
```

Your objective is to be able to indirectly build these structures using method
chaining. For example, the Orc would be constructed as such:

```rust
let my_orc = OrcBuilder::new()
    .name("Greuh")
    .position((100, 200))
    .weapon(Box::new(Axe::new()))
    .build().unwrap();
```

You would also like to be able to define default values for some attributes,
but not all of them; here, for the `health`, `level`, and `experience`
attributes. `build()` checks that every attribute which doesn't have a default
value has been set, returns `Ok` if this is the case, and `Err` otherwise.
Because in general, we expect that we have provided valid arguments, we simply
`unwrap()` the value.

In practice, you would set the type of the `Err` to a custom `InstanceErr`
type, or something like that.

However, as you write the code for `OrcBuilder` and `FarmBuilder`, you realise
that you spend most of your time repeating yourself and you start to doubt of
the efficiency of this pattern in production code. You figure that, as your
team will add more types and more properties, the code for the builders will
continue to grow until it becomes a bigger problem than having a constructor
with twenty arguments.

In fact, most of the methods are just about setting attributes:

```rust
fn health(self, value: i32) -> Self {
    self.health = value;
    self
}
```

You feel like the only interesting method is `build()`, because then you can
do _constructor stuff_ and set properties based on the values provided by the
builder. The other methods are mostly just a way to pass named _arguments_ to
said constructor. Even then, the constructor is often basic enough that there
is no need to do complex computation: you just have to _stuff the arguments_
inside the data structure.

Then, you remember reading about this feature in Rust called _macros_. If you
recall correctly, they allow to programmatically generate Rust code at
compile-time. After a bit of research, you feel like you should be able to do
most of your work using only two macros.

The first would allow you to just _stuff the arguments_ inside the destination
structure. It would generate both the builder and the destination structures.
Calling it would look like:

```rust
builder!(BuilderName => StructName {
    attr_name : attr_type = Some(default_value) | None,
    ...
});

impl StructName {
    // ...
}
```

The second would allow to provide a constructor in which the attributes of the
field would be visible. It would only generate the builder structure, so that
the destination structure could have different fields. You could, for example,
do something like this:

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    adult: bool,
}

skinny_builder!(UserBuilder => User,
    attributes: {
        name: String = None,
        age: i32 = Some(0)
    },
    
    // constructor -> Result<User, &'static str>
    // You can change the Err type by modifying the macro, or extend it to
    // be able to specify the Err type manually every time. For the sake of
    // this demo, however, we keep it simple.
    constructor: {
        let err = "Argument missing";
        
        // `name` and `age` are automatically available:
        // Here, we redefine them by trying to get their value.
        let name = try!(name.ok_or(err));
        let age = try!(age.ok_or(err));
        
        Ok(User {
            name: name,
            age: age,
            adult: age >= 18
        })
    });
```

Let's write each one!


## The builder which _stuffs 'em in_

The way our simplified Orc is built is straightforward: you put the arguments
inside the structure. In practice, you would probably add a _target_ reference
and a maximum health... but for the sake of demonstration you keep the struct
small. And, unless the generated Orc is a special one, e.g. a level 11 boss
with a terrifying name, we would expect that it spawns with 100% of its health,
at level 1 and with no experience. Heck! We can even call it _Orc_ and give it
either a bow or an axe depending on its class.

So, the Orc structure is declared as such:

```rust
builder!(OrcBuilder => Orc {
    health: i32 = Some(100),
    level: i32 = Some(1),
    experience: i32 = Some(0),
    
    // This works because the provided expression will simply be inserted
    // inside the OrcBuilder constructor.
    name: String = Some("Orc".to_owned()),
    
    position: (i32, i32) = None,
    weapon: Box<Weapon>  = None
});

impl Orc {
    // ...
}
```

Which is quite close from your original attempt without macros. Now, it is time
to write the macro and, with the documentation at hand, you come up with the
following pattern to match:

```rust
($src_name:ident => $dest_name:ident {
    $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
})
```

The first thing you do is generate the destination structure:

```rust
struct $dest_name {
    $( $attr_name : $attr_type ),*
}
```

Next, you generate the builder's structure. In this case, though, you want to
have `Option<T>` attributes. When you call `ThingBuilder::new()`, attributes
which have a default value will be set to `Some(default_value)`, while the
others will be set to `None`. Basically, you set it to whatever you put on the
right side of the equals sign, in your pattern. When the time will come to
`build()` the instance, you require from all attributes to be `Some`.

Here's how it goes:

```rust
struct $src_name {
    $( $attr_name : Option<$attr_type> ),*
}

impl $src_name {
    pub fn new() -> $src_name {
        $src_name {
            $(
                $attr_name : $attr_default
            ),*
        }
    }

    pub fn build(self) -> Result<$dest_name, &'static str> {
        let err = "Argument missing";
        
        $(
            let $attr_name = try!(self.$attr_name.ok_or(err));
        )*
        
        Ok($dest_name {
            $( $attr_name : $attr_name ),*
        })
    }
}
```

Then, you only need to add methods to the builder to set the attributes'
values. So, at the end of `impl $src_name`, you insert:

```rust
    $(
        fn $attr_name(mut self, value: $attr_type) -> Self {
            self.$attr_name = Some(value);
            self
        }
    )*
```

So here you have it, the `builder!` macro:

```rust
macro_rules! builder {
    ( $src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    })
    => {
        struct $dest_name {
            $( $attr_name : $attr_type ),*
        }
        
        struct $src_name {
            $( $attr_name : Option<$attr_type> ),*
        }
        
        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $(
                        $attr_name : $attr_default
                    ),*
                }
            }
        
            pub fn build(self) -> Result<$dest_name, &'static str> {
                let err = "Argument missing";
                
                $(
                    let $attr_name = try!(self.$attr_name.ok_or(err));
                )*
                
                Ok($dest_name {
                    $( $attr_name : $attr_name ),*
                })
            }
            
            $(
                fn $attr_name(mut self, value: $attr_type) -> Self {
                    self.$attr_name = Some(value);
                    self
                }
            )*
        }
    }
}
```


## A skinny builder with a custom constructor

Your `Farm` type, in this example, is slightly more complicated than the `Orc`.
Not only do you want to store its level and its health, you also want to
generate a level-dependent, random number for the kilograms of wheat it produces
every hour, which depends on the level you provided. Still, you generally
create farms at the demand of the player; in which case, they are level 1 and
have full health.

What you do then, is generate the value randomly if it has not been set. You
_would_ set it, for example, if you loaded the farm's data from a save file.

So, you wish to write something of the sort:

```rust
struct Farm {
    health: i32,
    level: i32,
    experience: i32,
    position: (i32, i32),
    kilograms_every_hour: i32,
}

impl Farm {
    // ...
}

skinny_builder!(UserBuilder => User,
    attributes: {
        health     : i32 = Some(100),
        level      : i32 = Some(1),
        experience : i32 = Some(0),
        
        position   : (i32, i32) = None,
        kilograms_every_hour: i32 = None,
    },
    constructor: {
        let err = "Argument missing";
        
        // We know it will never be `None` because it has a default value.
        let level = level.unwrap();
        
        Ok(Farm {
            health: health.unwrap(),
            level: level,
            experience: experience.unwrap(),
            position: try!(position.ok_or(err)),
            kilograms_every_hour: match kilograms_every_hour {
                Some(value) => value,
                None => random_integer() % 10 + level * 10
            }
        })
    });
```

There you have it! You can now allow the player to spawn a brand new farm on
the map by writing something like this:

```rust
FarmBuilder::new()
    .position(requested_and_verified_position)
    .build().unwrap();
```

It's probably going to be more complicated in production code, and we loose the
whole guarantee that _all the arguments are provided to the constructor_ at
compile-time, but until we have named arguments in Rust it does, its job pretty
well.


## Conclusion

Alright, it probably wasn't _really_ a real-world example, but then it would be
much longer to write, and my point wouldn't be any clearer. You got to admit,
though, that we managed to get something pretty impressive with way less code
than it would have taken otherwise, thanks to macro the magnificent. I hope
that it got you as excited about Rust as I am myself, because this language is
incredible.

If you haven't checked it out already, I've been writing a [tutorial](/arcaders/arcaders-1-0/)
about how the different features of Rust fit together. To do so, we're creating
a game. And we're using macros, pattern matching, trait objects, and all kinds
of rusty goodies to do it. I've still got some caveats to fix and a lot of
articles to write until it is finished, but it's getting there.

And as always, until next time, keep rusting!
