import Dict exposing (Dict)
import Html exposing (Html)
import Html.Attributes as HA
import Html.Events as HE
import List
import String


main =
    Html.beginnerProgram
        { model = initModel
        , update = update
        , view = view
        }


initModel : Model
initModel =
    { board = initBoard
    , activePlayer = White
    }


{-| The initial state of the board whenever a game begins.
-}
initBoard : List (Location, Piece)
initBoard =
    let
        initRow rank color =
            List.indexedMap (\file kind -> (file, rank) => Piece kind color True)

        backRow =
            [ Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook ]
    in
        List.concat
            [ initRow 0 White backRow
            , initRow 1 White (List.repeat 8 Pawn)
            , initRow 6 Black (List.repeat 8 Pawn)
            , initRow 7 Black backRow
            ]


{-| Generate some configuration of the board that allows us to debug the moves
that are available to a piece of a given `Kind`.
-}
testBoard : Kind -> List (Location, Piece)
testBoard kind =
    [ (3, 3) => Piece kind White True
    , (1, 1) => Piece Pawn White True
    , (3, 5) => Piece Pawn Black True
    ]


(=>) : a -> b -> (a, b)
(=>) = (,)


-- MODEL


type alias Model =
    { board : List (Location, Piece)
    , activePlayer : Color
    }


{-| The color of a piece and of the player that owns it.
-}
type Color
    = White
    | Black


{-| The coordinates of a square on the board, represented by two integers
between 0 and 7 (inclusive). In chess-speak,

    ( column, row ) ~ ( file, rank )
-}
type alias Location =
    ( ColumnID, RowID )

type alias ColumnID = Int
type alias RowID = Int


type alias Piece =
    { kind : Kind
    , color : Color
    , alive : Bool
    }


type Kind
    = King
    | Queen
    | Bishop
    | Knight
    | Rook
    | Pawn



-- UPDATE


type Msg
    = NOP


update : Msg -> Model -> Model
update msg model =
    case msg of
        NOP ->
            model



-- VIEW


{-| The side of a square on the board, in pixels.
-}
squareSide : Int
squareSide =
    48


view : Model -> Html Msg
view model =
    let
        renderedSquares =
            List.map
                (\(file, rank) ->
                    let
                        tileColor =
                            if (file - rank) % 2 == 0 then "rgb(209, 139, 71)"
                            else "rgb(255, 206, 158)"
                    in
                        empty
                            [ ("position", "absolute")
                            , ("left", px (file * squareSide))
                            , ("bottom", px (rank * squareSide))
                            , ("width", px squareSide)
                            , ("height", px squareSide)
                            , ("background", tileColor)
                            ])
                boardLocations

        renderedPieces =
            List.map
                (\((file, rank), piece) ->
                    let
                        (opacity, pointerEvents) =
                            if piece.alive then ("1", "all")
                            else ("0", "none")
                    in
                        styled
                            [ ("position", "absolute")
                            , ("left", px (file * squareSide))
                            , ("bottom", px (rank * squareSide))

                            , ("display", "flex")
                            , ("align-items", "center")
                            , ("justify-content", "center")
                            , ("width", px squareSide)
                            , ("height", px squareSide)

                            , ("opacity", opacity)
                            , ("pointer-events", pointerEvents)
                            , ("transition", "opacity 0.15s, left 0.2s, bottom 0.2s")
                            ]
                            [ viewPiece (file, rank) piece ])
                model.board
    in
        styled
            [ ("min-height", "calc(100vh - 2 * 24px)")
            , ("padding", "24px 0")
            , ("background", "#f1f1f1")
            , ("font-family", "Helvetica")
            ]
            [ styled
                [ ("position", "relative")
                , ("margin", "0 auto")
                , ("width", px (8 * squareSide))
                , ("height", px (8 * squareSide))
                , ("box-shadow", shadow1)
                , ("border", "3px solid #fff")
                ]
                (renderedSquares ++ renderedPieces)
            ]



viewPiece : Location -> Piece -> Html Msg
viewPiece location piece =
    let
        -- FIXME Use an icon instead of plain text
        pieceIcon =
            Html.text (String.slice 0 2 (toString piece.kind))

        (background, textColor) =
            case piece.color of
                White -> ("#fafafa", "rgba(0, 0, 0, 0.7)")
                Black -> ("#303030", "rgba(255, 255, 255, 0.9)")
    in
        Html.div
            [ HA.style
                [ ("display", "flex")
                , ("align-items", "center")
                , ("justify-content", "center")
                , ("width", px (squareSide * 3 // 4))
                , ("height", px (squareSide * 3 // 4))
                , ("border-radius", "100%")
                , ("background", background)
                , ("color", textColor)
                , ("box-shadow", shadow1)
                ]
            ]
            [ pieceIcon ]



-- VIEW UTILITIES


px : Int -> String
px pixels =
    toString pixels ++ "px"


shadow1 : String
shadow1 = """
0px 2px 2px 0px rgba(0, 0, 0, 0.14),
0px 1px 5px 0px rgba(0, 0, 0, 0.12),
0px 3px 1px -2px rgba(0, 0, 0, 0.2)
"""


styled : List (String, String) -> List (Html a) -> Html a
styled cssAttrs =
    Html.div [ HA.style cssAttrs ]


empty : List (String, String) -> Html a
empty cssAttrs =
    styled cssAttrs []


boardLocations : List Location
boardLocations =
    List.range 0 7
    |> List.map (\file -> List.map (\rank -> (file, rank)) (List.range 0 7))
    |> List.concat
