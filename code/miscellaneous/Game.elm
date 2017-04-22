module Game exposing (..)

import Html exposing (Html)
import Html.Attributes as HA
import Html.Events as HE
import Json.Decode as JD
import Json.Encode as JE
import List
import String
import WebSocket


init : (Model, Cmd Msg)
init =
    let
        model =
            { board = initBoard
            , localPlayer = White
            , activePlayer = White
            , selectedPiece = Nothing
            }
    in
        model ! []


initBoard : List (Piece, Location)
initBoard =
    List.concat
        [ initRow 0 White [ Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook ]
        , initRow 1 White (List.repeat 8 Pawn)
        , initRow 6 Black (List.repeat 8 Pawn)
        , initRow 7 Black [ Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook ]
        ]


testBoard : List (Piece, Location)
testBoard =
    [ { kind = Queen, owner = White, alive = True } => { rank = 3, file = 3 }
    , { kind = Pawn, owner = White, alive = True } => { rank = 6, file = 6 }
    , { kind = Pawn, owner = Black, alive = True } => { rank = 5, file = 3 }
    ]


initRow : RowID -> Color -> List Kind -> List (Piece, Location)
initRow rank owner kinds =
    let
        zip xs ys func =
            case (xs, ys) of
                (x :: xtl, y :: ytl) ->
                    func x y :: zip xtl ytl func

                _ ->
                    []

        initPiece file kind =
            { kind = kind, owner = owner, alive = True } => { rank = rank, file = file }
    in
        zip (List.range 0 7) kinds initPiece


(=>) : a -> b -> (a, b)
(=>) = (,)



-- MODEL


type alias Model =
    { board : List (Piece, Location)
    , localPlayer : Color
    , activePlayer : Color
    , selectedPiece : Maybe (Kind, Location)
    }


type alias Location =
    { rank : RowID
    , file : ColID
    }


{-| The number of the row, in the inclusive range [0..7].
-}
type alias RowID =
    Int


{-| The number of the column, in the inclusive range [0..7].
-}
type alias ColID =
    Int


{-| The representation of a piece on the board.
-}
type alias Piece =
    { kind : Kind
    , owner : Color
    , alive : Bool
    }


type Color
    = White
    | Black


type Kind
    = King
    | Queen
    | Bishop
    | Knight
    | Rook
    | Pawn



-- QUERY EXTRA INFORMATION


getPiece : Location -> List (Piece, Location) -> Maybe Piece
getPiece location board =
    find (Tuple.second >> (==) location) board
    |> Maybe.map Tuple.first


inBoard : Location -> Bool
inBoard { rank, file } =
    rank >= 0 && rank <= 7 && file >= 0 && file <= 7


squareLocations : List Location
squareLocations =
    List.range 0 7
    |> List.map (\rank -> List.range 0 7
        |> List.map (\file -> { rank = rank, file = file }))
    |> List.concat


squareColor : Location -> Color
squareColor location =
    if (location.rank - location.file) % 2 == 0 then
        Black
    else
        White


oppositeColor : Color -> Color
oppositeColor color =
    case color of
        White ->
            Black

        Black ->
            White


{-| Cannot move onto/over a piece of the same owner.
-}
availableMoves : List (Piece, Location) -> Piece -> Location -> List Location
availableMoves board piece { rank, file } =
    let
        condMove cond target =
            if cond target then
                [target]
            else
                []

        anyoneAt target =
            getPiece target board /= Nothing

        enemyAt target =
            case getPiece target board of
                Nothing ->
                    False

                Just targetPiece ->
                    targetPiece.owner /= piece.owner

        allyAt target =
            case getPiece target board of
                Nothing ->
                    False

                Just targetPiece ->
                    targetPiece.owner == piece.owner

        moveStep target nextLocation =
            if (not << inBoard) target || allyAt target then
                []
            else if enemyAt target then
                [target]
            else
                target :: moveStep (nextLocation target) nextLocation

        moveLine nextLocation =
            moveStep (nextLocation { rank = rank, file = file }) nextLocation

        moves =
            case piece.kind of
                King ->
                    List.map (condMove (not << allyAt))
                        [ { rank = rank, file = file + 1 }
                        , { rank = rank + 1, file = file + 1 }
                        , { rank = rank + 1, file = file }
                        , { rank = rank + 1, file = file - 1 }
                        , { rank = rank, file = file - 1 }
                        , { rank = rank - 1, file = file - 1 }
                        , { rank = rank - 1, file = file }
                        , { rank = rank - 1, file = file + 1 }
                        ]

                Queen ->
                    [ availableMoves board { piece | kind = Bishop } { rank = rank, file = file }
                    , availableMoves board { piece | kind = Rook } { rank = rank, file = file }
                    ]

                Bishop ->
                    [ moveLine (\{rank,file} -> { rank = rank + 1, file = file + 1 })
                    , moveLine (\{rank,file} -> { rank = rank + 1, file = file - 1 })
                    , moveLine (\{rank,file} -> { rank = rank - 1, file = file - 1 })
                    , moveLine (\{rank,file} -> { rank = rank - 1, file = file + 1 })
                    ]


                Knight ->
                    [ condMove (not << allyAt) { rank = rank + 1, file = file + 2 }
                    , condMove (not << allyAt) { rank = rank + 2, file = file + 1 }
                    , condMove (not << allyAt) { rank = rank + 2, file = file - 1 }
                    , condMove (not << allyAt) { rank = rank + 1, file = file - 2 }
                    , condMove (not << allyAt) { rank = rank - 1, file = file + 2 }
                    , condMove (not << allyAt) { rank = rank - 2, file = file + 1 }
                    , condMove (not << allyAt) { rank = rank - 2, file = file - 1 }
                    , condMove (not << allyAt) { rank = rank - 1, file = file - 2 }
                    ]

                Rook ->
                    [ moveLine (\{rank,file} -> { rank = rank, file = file + 1 })
                    , moveLine (\{rank,file} -> { rank = rank + 1, file = file })
                    , moveLine (\{rank,file} -> { rank = rank, file = file - 1 })
                    , moveLine (\{rank,file} -> { rank = rank - 1, file = file })
                    ]

                Pawn ->
                    case piece.owner of
                        White ->
                            [ condMove (not << anyoneAt) { rank = rank + 1, file = file }
                            , condMove enemyAt { rank = rank + 1, file = file - 1 }
                            , condMove enemyAt { rank = rank + 1, file = file + 1 }
                            , if rank == 1 && (not << anyoneAt) { rank = rank + 1, file = file } then
                                condMove (not << anyoneAt) { rank = rank + 2, file = file }
                              else
                                []
                            ]

                        Black ->
                            [ condMove (not << anyoneAt) { rank = rank - 1, file = file }
                            , condMove enemyAt { rank = rank - 1, file = file - 1 }
                            , condMove enemyAt { rank = rank - 1, file = file + 1 }
                            , if rank == 6 && (not << anyoneAt) { rank = rank - 1, file = file } then
                                condMove (not << anyoneAt) { rank = rank - 2, file = file }
                              else
                                []
                            ]
    in
        List.filter inBoard (List.concat moves)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- UPDATE


type Msg
    = NOP
    | UnselectPiece
    | SelectPiece Location
    | MovePiece { from : Location, to : Location }


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        NOP ->
            model ! []

        UnselectPiece ->
            { model | selectedPiece = Nothing } ! []

        SelectPiece location ->
            case find (Tuple.second >> (==) location) model.board of
                Nothing ->
                    Debug.crash "Selected piece that does not exist."

                Just (piece, location) ->
                    if model.localPlayer == model.activePlayer && piece.owner == model.localPlayer then
                        { model | selectedPiece = Just (piece.kind, location) } ! []
                    else
                        model ! []

        MovePiece { from, to } ->
            let
                removeEnemy =
                    findMap
                        (Tuple.second >> (==) to)
                        (\(piece, loc) -> ({ piece | alive = False }, loc))
                        model.board

                movedActive =
                    findMap
                        (Tuple.second >> (==) from)
                        (\(piece, _) -> (piece, to))
                        removeEnemy
            in
                -- TODO Send change to opponent
                { model |
                    board = movedActive,
                    activePlayer = oppositeColor model.activePlayer,
                    selectedPiece = Nothing
                } ! []



-- UTILITY FUNCTIONS


find : (a -> Bool) -> List a -> Maybe a
find cond xs =
    case xs of
        [] ->
            Nothing

        x :: tl ->
            if cond x then
                Just x
            else
                find cond tl

findMap : (a -> Bool) -> (a -> a) -> List a -> List a
findMap cond transform xs =
    case xs of
        [] ->
            []

        x :: tl ->
            if cond x then
                transform x :: findMap cond transform tl
            else
                x :: findMap cond transform tl



-- VIEW


squareSide : Int
squareSide =
    48


view : Model -> Html Msg
view model =
    let
        (activePlayerBackground, activePlayerTooltip) =
            case model.activePlayer of
                White ->
                    ("#fff", "C'est le tour des blancs.")

                Black ->
                    ("#444", "C'est le tour des noirs.")

        viewPiece_ (piece, location) =
            viewPiece model.localPlayer
                (model.localPlayer == model.activePlayer && piece.owner == model.localPlayer)
                (piece, location)
    in
        Html.div
            [ HA.style
                [ ("min-height", "calc(100vh - 2 * 24px)")
                , ("padding", "24px 0")
                , ("background", "#f1f1f1")
                ]
            ]
            [ Html.div
                [ HA.style
                    [ ("position", "relative")
                    , ("margin", "0 auto")
                    , ("width", px (8 * squareSide))
                    , ("height", px (8 * squareSide))
                    , ("box-shadow", shadow1)
                    , ("border", "3px solid #fff")
                    ]
                ]
                [ viewBoard
                , Html.div [] (List.map viewPiece_ model.board)
                , Html.div [] (viewTargets model.localPlayer model.board model.selectedPiece)
                ]

            , Html.div
                [ HA.style
                    [ ("display", "flex")
                    , ("align-items", "center")
                    , ("justify-content", "center")
                    , ("width", px (8 * squareSide + 48))
                    , ("height", "48px")
                    , ("padding", "8px 0")
                    , ("margin", "0 auto")
                    ]
                ]
                [ Html.div
                    [ HA.title activePlayerTooltip
                    , HA.style
                        [ ("width", "40px")
                        , ("height", "40px")
                        , ("background", activePlayerBackground)
                        , ("border", "2px solid #d0d0d0")
                        , ("border-radius", "100%")
                        ]
                    ]
                    []
                ]
            ]


viewBoard : Html Msg
viewBoard =
    let
        viewSquare rank file =
            let
                background =
                    case squareColor { rank = rank, file = file } of
                        Black ->
                            "rgb(209, 139, 71)"

                        White ->
                            "rgb(255, 206, 158)"
            in
                Html.div
                    [ HA.style
                        [ ("position", "absolute")
                        , ("bottom", px (rank * squareSide))
                        , ("left", px (file * squareSide))
                        , ("width", px squareSide)
                        , ("height", px squareSide)
                        , ("background", background)
                        ]
                    ]
                    []

        renderedSquares =
            List.map (\{rank, file} -> viewSquare rank file) squareLocations
    in
        Html.div [] renderedSquares


viewPiece : Color -> Bool -> (Piece, Location) -> Html Msg
viewPiece playerView selectable (piece, { rank, file }) =
    let
        pieceIcon =
            Html.text (String.slice 0 2 (toString piece.kind))

        (opacity, pointerEvents) =
            if piece.alive then ("1", "all")
            else ("0", "none")

        (background, textColor) =
            case piece.owner of
                White -> ("#fafafa", "rgba(0, 0, 0, 0.7)")
                Black -> ("#303030", "rgba(255, 255, 255, 0.9)")

        cursor =
            if selectable then "pointer"
            else "default"

        (rankAttr, fileAttr) =
            case playerView of
                White -> ("bottom", "left")
                Black -> ("top", "right")
    in
        Html.div
            [ HA.style
                [ ("position", "absolute")
                , (rankAttr, px (rank * squareSide))
                , (fileAttr, px (file * squareSide))

                , ("display", "flex")
                , ("align-items", "center")
                , ("justify-content", "center")
                , ("width", px squareSide)
                , ("height", px squareSide)

                , ("opacity", opacity)
                , ("pointer-events", pointerEvents)
                , ("font-family", "Helvetica")
                , ("transition", "bottom 0.2s, left 0.2s, right 0.2s, top 0.2s, opacity 0.1s")
                ]
            ]
            [ Html.div
                [ HE.onClick (SelectPiece { rank = rank, file = file })
                , HA.style
                    [ ("display", "flex")
                    , ("align-items", "center")
                    , ("justify-content", "center")
                    , ("width", px (squareSide * 3 // 4))
                    , ("height", px (squareSide * 3 // 4))
                    , ("border-radius", "100%")
                    , ("background", background)
                    , ("color", textColor)
                    , ("box-shadow", shadow1)
                    , ("cursor", cursor)
                    ]
                ]
                [ pieceIcon ]
            ]


viewTargets : Color -> List (Piece, Location) -> Maybe (Kind, Location) -> List (Html Msg)
viewTargets playerView board selectedPiece =
    let
        (rankAttr, fileAttr) =
            case playerView of
                White ->
                    ("bottom", "left")

                Black ->
                    ("top", "right")

        (legalMoves, location) =
            case selectedPiece of
                Nothing ->
                    ([], Nothing)

                Just (kind, location) ->
                    ( availableMoves board { kind = kind, owner = playerView, alive = True } location
                    , Just location
                    )
    in
        List.map
            (\possibleTarget ->
                let
                    (background, pointerEvents) =
                        if selectedPiece == Nothing then
                            ("rgba(0, 0, 0, 0)", "none")
                        else if List.member possibleTarget legalMoves || Just possibleTarget == location then
                            ("rgba(0, 0, 0, 0)", "all")
                        else
                            ("rgba(0, 0, 0, 0.4)", "none")

                    (onClick, cursor) =
                        case selectedPiece of
                            Nothing ->
                                (NOP, "default")

                            Just (_, location) ->
                                if List.member possibleTarget legalMoves then
                                    (MovePiece { from = location, to = possibleTarget }, "pointer")
                                else if possibleTarget == location then
                                    (UnselectPiece, "default")
                                else
                                    (NOP, "default")
                in
                    Html.div
                        [ HE.onClick onClick
                        , HA.style
                            [ ("position", "absolute")
                            , (rankAttr, px (squareSide * possibleTarget.rank))
                            , (fileAttr, px (squareSide * possibleTarget.file))
                            , ("width", px squareSide)
                            , ("height", px squareSide)
                            , ("background", background)
                            , ("pointer-events", pointerEvents)
                            , ("cursor", cursor)
                            , ("transition", "background 0.2s")
                            ]
                        ]
                        [])
            squareLocations


px : Int -> String
px pixels =
    toString pixels ++ "px"


shadow1 : String
shadow1 = """
0px 2px 2px 0px rgba(0, 0, 0, 0.14),
0px 1px 5px 0px rgba(0, 0, 0, 0.12),
0px 3px 1px -2px rgba(0, 0, 0, 0.2)
"""
