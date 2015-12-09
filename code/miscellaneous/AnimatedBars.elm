import Html exposing (button, div, h2, text)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)
import StartApp.Simple as StartApp


main =
  StartApp.start { model = model, view = view, update = update }


model =
  { name = "No character selected..."
  , str = 0
  , dex = 0
  , con = 0
  , int = 0
  , wis = 0
  , cha = 0
  }


update action model =
  case action of
    SelectJoshua  -> joshua
    SelectJebbeth -> jebbeth


view address model =
  div []
    [ button [ onClick address SelectJoshua ] [ text "Joshua" ]
    , button [ onClick address SelectJebbeth ] [ text "Jebbeth" ]
    , characterSheet model
    ]



-- MODEL

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



-- UPDATE

type Action
  = SelectJoshua
  | SelectJebbeth



-- VIEW

-- Generates a colored horizontal bar given a ratio between 0 and 1.
hBar ratio =
  let hue = toString (ratio * 120)
  in
    div
      []
      [ div
        [ style
          [ ("transition", "width 0.3s ease, background-color 0.3s ease")
          , ("backgroundColor", "hsl(" ++ hue ++ ", 80%, 56%)" )
          , ("width", (toString (ratio * 100)) ++ "%")
          , ("height", "32px")
          ]
        ] []
      ]


hStatBar fullValue label value =
  div
    [ style
      [ ("display", "flex")
      , ("flex-flow", "row nowrap")
      , ("align-items", "center")
      , ("margin", "1px")
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


abilityBar = hStatBar 20


characterSheet character =
  div
    [ style
      [ ("width", "calc(95% - 1rem)")
      , ("padding", "0.5rem")
      , ("border", "1px dashed #ccc")
      , ("border-radius", "0.4rem")
      , ("margin", "auto")
      ]
    ]
    [ div
      [ style
        [ ("font-size", "32px")
        , ("margin-bottom", "0.8rem")
        ]
      ]
      [ text character.name ]
    , abilityBar "STR" character.str
    , abilityBar "DEX" character.dex
    , abilityBar "CON" character.con
    , abilityBar "INT" character.int
    , abilityBar "WIS" character.wis
    , abilityBar "CHA" character.cha
    ]
