---
layout: post
title:  "Animated horizontal lines in Elm"
categories: elm
---

Today, I decided that it would be fun to see how long it would take me to make a
bunch of animated horizontal "progress" bars in Elm. Turns out that I got
something working in less than an hour without much trouble and the code looks
pretty damn good. This article explains how I did it.

You should note that this isn't a beginner's course in Elm, although you should
be able to follow along even if you've never seen Elm code. It was designed to
be obvious. The only requirement, really, is to know how HTML and CSS work.

You can find the code
[here](https://github.com/jadpole/jadpole.github.io/blob/master/code/miscellaneous/AnimatedBars.elm)
and you should be able to [try it out](http://elm-lang.org/try) on elm's website
as we progress.

Let's get going!


## Setting the foundations

We start by importing everything that we're going to need.

```elm
import Html exposing (button, div, h2, text)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)
import StartApp.Simple as StartApp
```

We will be using Evan's `StartApp` library to reduce the boilerplate. It hides
the complicated stuff behind a functional Model-View-Controller API. So here's
the _hello world_ program, Elm style:

```elm
--? The entry point of our program.
main =
  StartApp.start { model = model, view = view, update = update }


--? The default model of our program.
model =
    "Hello, World!"


--? Renders the content our our application. To do so, we build a tree of
--? components which Elm will render for us.
view address model =
  --? The root of our page will be a `div`, or a rectangle which we can fill
  --? with stuff.
  div
    --? Those are the properties of the element. We do not want to do anything
    --? special with our `div`, so we leave it empty.
    []
    --? This is a list of the element's contents. In this case, it shows the
    --? "Hello, World!" string which we assigned to `model`.
    [ text model ]


--? Because we do not yet react to user inputs, we replace the first argument
--? (the user's recent actions) with an underscore. This means: I don't care
--? about it.
--?
--? Because we're not yet reacting to any action, we directly return the model,
--? meaning that we do not change it.
update _ model =
  model
```

Notice that our program's behavior is dictated by `StartApp.start`. It takes in
a few functions, namely `view`, `model` and `update`. Although we give them the
same names as the fields of the object expected by `start`, this is simply a
matter of convention. If you're more game-oriented, you might write:

```elm
StartApp.start { model = gameWorld, view = render, update = tick }
```

And it would work insofar as we rename our functions accordingly... though you
probably won't be using StartApp to make a game (we may come back to this in a
later article).


## The horizontal bar

Now that we took care of the boilerplate, we can start to design our program.
The way we generally go about things in Elm is: as we progress, we make a bunch
of simple components which we combine in more complex components, until we have
a complete application.

The first component that we would like to think about is the animated bar
itself. Size-wise, it fills all the horizontal space it can and it has a fixed
height of 32 pixels. Let's start with the simplest implementation:

```elm
-- Generates a colored horizontal bar given a ratio covered/total between 0 and 1.
hBar ratio =
  div []
    --? Our horizontal bar contains another (blue) rectangle representing how
    --? much it is filled.
    [ div
      [ style
        [ ("backgroundColor", "blue" )
        , ("width", (toString (ratio * 100)) ++ "%")
        , ("height", "32px")
        ]
      ] []
    ]
```

Before we go any further, let us _render_ the component:

```elm
model =
  0.4


view address model =
  div []
    [ hBar model ]
```

![A simple, blue stat bar](/images/elm-1.png)

It works, but surely we're far from done. I mean, animating the size and the
color of our horizontal bar must be quite hard. Except that it isn't. Gaze upon
the power of CSS3!

```elm
hBar ratio =
  let hue = toString (ratio * 120)
  in div []
    [ div
      [ style
        [ ("transition", "width 0.3s ease, background-color 0.3s ease")
        , ("backgroundColor", "hsl(" ++ hue ++ ", 80%, 56%)" )
        , ("width", (toString (ratio * 100)) ++ "%")
        , ("height", "32px")
        ]
      ] []
    ]
```

The `transition` attribute takes care of animating everything for us, and the
`backgroundColor` now depends on the provided ratio. If you aren't familiar with
HSL (and I'm not either), you only need to know that it is a way to express
color values, akin to the RGB system you're probably familiar with.

You should now have the following result:

![A bar with changing color](/images/elm-2.png)

You can change the value of `model` if you want, and see for yourself how the
bar's color changes as the ratio gets bigger and smaller. Note, however, that
because the value doesn't change _as the program is running_, the progress bar
will not change smoothly. Do not worry, this should be fixed soon enough!


## General stat bar

Now that our horizontal bar works properly, we can compose it with other basic
components to build complexity. First, we will extend our horizontal bar with a
label and a number, in order to see what it _means_. We will call the result a
`hStatBar` (horizontal stat bar). To align everything properly, we will use the
[Flexbox API](https://css-tricks.com/snippets/css/a-guide-to-flexbox/).

```elm
hStatBar fullValue label value =
  div
    [ style
      [ ("display", "flex")
      , ("flex-flow", "row nowrap")
      , ("align-items", "center")
      , ("margin", "3px 0")
      ]
    ]
    [ div
      [ style [ ("width", "3rem"), ("text-align", "center") ] ]
      [ text label ]

    , div
      [ style [ ("flex", "1") ] ]
      [ hBar (value / fullValue) ]

    , div
      [ style [ ("width", "1rem"), ("text-align", "center") ] ]
      [ text (toString value) ]
    ]
```

Say our user got 13/20 on his exam. We would render it as:

```elm
model =
  13

view address model =
  div []
    [ hStatBar 20 "Score" model
    ]
```

Pretty simple, isn't it?


## D&D character sheet

It is now time to choose _what_ we want our bar to represent. For no reason in
particular, let's generate character sheets for our D&D player, and because
we're kinda lazy, we're only going to show their name and abilities. Our
players' characters will be called Joshua and Jebbeth, and are defined as such:

```elm
joshua =
  { name = "Joshua Devineer"
  , str = 10
  , dex = 12
  , con = 9
  , int = 17
  , wis = 15
  , cha = 12
  }

jebbeth =
  { name = "Jebbeth Truul"
  , str = 16
  , dex = 13
  , con = 14
  , int = 11
  , wis = 9
  , cha = 9
  }
```

By default, the model will not show any of the two characters, but instead an
empty character sheet with all abilities set to zero:

```elm
model =
  { name = "No character selected..."
  , str = 0
  , dex = 0
  , con = 0
  , int = 0
  , wis = 0
  , cha = 0
  }
```

Also, we will set the _full value_ of every stat bar to be 20. We could write it
down every for every ability, but it makes more sense to create a new function
instead, which will take care of this for us:

```elm
abilityBar label value =
    hStatBar 20 label value
```

You probably agree that this is ugly and repetitive. Fortunately, Elm offers a
functionality which fixes exactly that problem: partial applications.

```elm
abilityBar = hStatBar 20
```

Prettier, isn't it! Let us then render a character sheet:

```elm
characterSheet character =
  div
    [ style
      [ ("width", "calc(95% - 1rem)")
      , ("padding", "0.5rem")
      , ("border", "1px dashed #ccc")
      , ("border-radius", "0.4rem")
      --? Center the character sheet horizontally
      , ("margin", "auto")
      ]
    ]

    [ div   -- Render the character's name
      [ style
        [ ("font-size", "32px")
        , ("margin-bottom", "0.8rem")
        ]
      ]
      [ text character.name ]
      -- Render the character's abilities
    , abilityBar "STR" character.str
    , abilityBar "DEX" character.dex
    , abilityBar "CON" character.con
    , abilityBar "INT" character.int
    , abilityBar "WIS" character.wis
    , abilityBar "CHA" character.cha
    ]
```

We only need to add this to `view` in order to have a working character sheet:

```rust
view address model =
  div []
    [ characterSheet model
    ]
```

![An empty character sheet](#)

Pretty empty, huh... Let's fix that!


## Switching characters

This is where the `update` function comes into play. We would like to have two
buttons, each selecting one of the available characters. We first start by
defining what we mean by an _action_:

```elm
type Action
  = SelectJoshua
  | SelectJebbeth
```

Then, we handle it by modifying our model:

```elm
update action model =
  case action of
    SelectJoshua  -> joshua
    SelectJebbeth -> jebbeth
```

<!--TODO
You might notice that, we never change _model_ explicitly. Instead, we tell the
compiler: here's a new, updated model for ya!
-->

Finally, we add two buttons to our application to select the desired character:

```elm
view address model =
  div []
    [ button [ onClick address SelectJoshua ] [ text "Joshua" ]
    , button [ onClick address SelectJebbeth ] [ text "Jebbeth" ]
    , characterSheet model
    ]
```

This basically means: here's a button. When you click on it, call

And... we're done!

![A complete, animated character sheet](#)
