import Dict exposing (Dict)
import Html exposing (Html)
import Html.Attributes as HA
import Html.Events as HE
import Json.Decode as JD


main =
    Html.beginnerProgram
        { model = initModel
        , update = update
        , view = view
        }


initModel : Model
initModel =
    Empty



-- MODEL


{-| Start by only drawing one string/knot.
-}
type Model
    = Empty
    | Drawing (List (Point, Crossing))
    | ViewKnot Knot


type alias Knot =
    { path : List Point
    , crossings : Dict Point Crossing
    }


type alias Point =
    ( Int, Int )


type Crossing
    = Above
    | Below



-- UPDATE


type Msg
    = NOP
    | StartDrawing Int Int
    | EnterPoint Int Int
    | StopDrawing


update : Msg -> Model -> Model
update msg model =
    case msg of
        NOP ->
            model

        StartDrawing x y ->
            let
                squareX =
                    x // squareSide

                squareY =
                    y // squareSide
            in
                Drawing [ ((squareX, squareY), Above) ]

        EnterPoint x y ->
            let
                squareX =
                    x // squareSide

                squareY =
                    y // squareSide

                _ = Debug.log "Entering" (squareX, squareY)
            in
                case model of
                    Drawing path ->
                        case List.head (List.reverse path) of
                            Nothing ->
                                Debug.crash "Matched path without a beginning"

                            Just prevPoint ->
                                let
                                    newPoints =
                                        trace (Tuple.first prevPoint) (squareX, squareY)
                                        |> List.map (\pt -> (pt, Above))

                                    shadowedPrevious =
                                        List.map
                                            (\(point, crossing) ->
                                                if List.member (point, Above) newPoints then
                                                    (point, Below)
                                                else
                                                    (point, crossing))
                                            path
                                in
                                    Drawing (shadowedPrevious ++ newPoints)

                    _ ->
                        model

        StopDrawing ->
            case model of
                Empty ->
                    Empty

                Drawing path ->
                    let
                        firstPoint =
                            List.head path

                        lastPoint =
                            List.head (List.reverse path)
                    in
                        if firstPoint == lastPoint then
                            case buildKnot path of
                                Just knot ->
                                    ViewKnot knot

                                Nothing ->
                                    Empty
                        else
                            Empty

                ViewKnot data ->
                    ViewKnot data


{-| Construct a path of segments from `src` (excluded) to `dest` (included).
This is useful when "mousemove" skips some tiles; in this case, we must come up
with the most likely path between the last detected positions of the mouse.
-}
trace : Point -> Point -> List Point
trace (srcX, srcY) (destX, destY) =
    let
        nextPoint =
            (srcX + sign (destX - srcX), srcY + sign (destY - srcY))
    in
        if (srcX, srcY) == nextPoint then
            []
        else
            nextPoint :: trace nextPoint (destX, destY)


sign : Int -> Int
sign x =
    if x == 0 then 0
    else if x < 0 then -1
    else 1


buildKnot : List (Point, Crossing) -> Maybe Knot
buildKnot path =
    let
        checkOverlap remaining overlap =
            case remaining of
                a :: b :: tl ->
                    let
                        foundOverlap =
                            ridePath
                                (\prev current ->
                                    if prev == a && current == b then Just (a, b)
                                    else Nothing)
                                tl
                            |> List.filterMap identity
                    in
                        case foundOverlap of
                            [] ->
                                checkOverlap remaining Nothing


                _ ->
                    overlap


        points =
            List.map Tuple.first path

        noOverlap =
            List.foldl checkOverlap Nothing points == Nothing
    in
    Just
        { path = List.map Tuple.first path
        , crossings = Dict.empty -- TODO
        }


ridePath : (a -> a -> b) -> List a -> List b
ridePath visit xs =
    case xs of
        curr :: next :: tl ->
            visit curr next :: ridePath visit (next :: tl)

        _ ->
            []



-- VIEW


squareSide : Int
squareSide =
    24


segmentWidth : Int
segmentWidth =
    4


view : Model -> Html Msg
view model =
    let
        handleMouse =
            case model of
                Drawing _ ->
                    True

                _ ->
                    False

        backgroundGrid =
            List.map (viewGridSquare handleMouse) grid

        renderedKnot =
            case model of
                Empty ->
                    []

                Drawing path ->
                    viewPath path

                ViewKnot knot ->
                    [] -- TODO
    in
        Html.div
            [ HE.onWithOptions "mousedown"
                { stopPropagation = False, preventDefault = True }
                (JD.map2 StartDrawing
                    (JD.field "clientX" JD.int)
                    (JD.field "clientY" JD.int))

            , HE.onMouseUp StopDrawing

            , HA.style
                [ ("height", "100vh")
                , ("overflow", "hidden")
                ]
            ]
            (backgroundGrid ++ renderedKnot)


viewGridSquare : Bool -> (Int, Int) -> Html Msg
viewGridSquare handleMouse (x, y) =
    let
        tileColor =
            if (x - y) % 2 == 0 then "rgb(209, 139, 71)"
            else "rgb(255, 206, 158)"
    in
        Html.div
            [ HA.style
                [ ("position", "absolute")
                , ("left", px (x * squareSide))
                , ("top", px (y * squareSide))
                , ("width", px squareSide)
                , ("height", px squareSide)
                , ("background", tileColor)
                , ("opacity", "0.2")
                ]
            ]
            [ Html.div
                [ HE.on "mouseenter"
                    (JD.map2 EnterPoint
                        (JD.field "clientX" JD.int)
                        (JD.field "clientY" JD.int))

                , HA.style
                    [ ("width", "100%")
                    , ("height", "100%")
                    , ("border-radius", "100%")
                    ]
                ]
                []
            ]


grid =
    List.range 0 20
    |> List.map (\row -> List.map (\col -> (row, col)) (List.range 0 20))
    |> List.concat



-- KNOT RENDERING


viewPath : List (Point, Crossing) -> List (Html a)
viewPath path =
    case path of
        [] ->
            []

        [((startX, startY), _)] ->
            [ Html.div
                [ HA.style
                    [ ("position", "absolute")
                    , ("left", px (startX * squareSide + squareSide // 8))
                    , ("top", px (startY * squareSide + squareSide // 8))
                    , ("width", px (squareSide * 3 // 4))
                    , ("height", px (squareSide * 3 // 4))
                    , ("background", "#333")
                    , ("border-radius", "100%")
                    , ("pointer-events", "none")
                    ]
                ]
                []
            ]

        ((startX, startY), startCrossing) :: (nextPoint, nextCrossing) :: tl ->
            let
                step prevPoint remaining =
                    case remaining of
                        [] ->
                            []

                        [(point, crossing)] ->
                            [viewSegment crossing point prevPoint]

                        (point, crossing) :: (nextPoint, nextCrossing) :: tl ->
                            [ viewSegment crossing point prevPoint
                            , viewSegment crossing point nextPoint
                            ] ++ step point ((nextPoint, nextCrossing) :: tl)


                motionOrigin =
                    Html.div
                        [ HA.style
                            [ ("position", "absolute")
                            , ("left", px (startX * squareSide + squareSide // 4))
                            , ("top", px (startY * squareSide + squareSide // 4))
                            , ("width", px (squareSide // 2))
                            , ("height", px (squareSide // 2))
                            , ("background", "#333")
                            , ("border-radius", "100%")
                            , ("pointer-events", "none")
                            , ("transition", "width 0.1s, height 0.1s, left 0.1s, top 0.1s")
                            ]
                        ]
                        []

                segment =
                    viewSegment startCrossing (startX, startY) nextPoint
            in
                [motionOrigin, segment] ++ step (startX, startY) ((nextPoint, nextCrossing) :: tl)


{-| Trace a segment from the center of SRC to the edge of its square, in the
direction of DEST.
-}
viewSegment : Crossing -> Point -> Point -> Html a
viewSegment crossing (srcX, srcY) (destX, destY) =
    let
        toSide =
            squareSide // 2

        toCorner =
            ceiling (sqrt 2 * toFloat squareSide / 2)

        (length, angleFrac) =
            if destX > srcX && destY == srcY then
                (toSide, 0 / 8)
            else if destX > srcX && destY < srcY then
                (toCorner, 1 / 8)
            else if destX == srcX && destY < srcY then
                (toSide, 2 / 8)
            else if destX < srcX && destY < srcY then
                (toCorner, 3 / 8)
            else if destX < srcX && destY == srcY then
                (toSide, 4 / 8)
            else if destX < srcX && destY > srcY then
                (toCorner, 5 / 8)
            else if destX == srcX && destY > srcY then
                (toSide, 6 / 8)
            else if destX > srcX && destY > srcY then
                (toCorner, 7 / 8)
            else
                Debug.crash "All possibilities were already enumerated."
    in
            Html.div
                [ HA.style
                    [ ("position", "absolute")
                    , ("left", px (srcX * squareSide + squareSide // 2))
                    , ("top", px (srcY * squareSide + (squareSide - segmentWidth) // 2))
                    , ("width", px length)
                    , ("height", px segmentWidth)
                    , ("pointer-events", "none")
                    , ("transform-origin", "left")
                    , ("transform", "rotate(" ++ toString (-angleFrac * 2 * pi) ++ "rad)")
                    ]
                ]
                (case crossing of
                    Above ->
                        [ Html.div
                            [ HA.style
                                [ ("height", "100%")
                                , ("background", "#333")
                                ]
                            ]
                            []

                        , Html.div
                            [ HA.style
                                [ ("position", "absolute")
                                , ("left", px (-segmentWidth // 2))
                                , ("top", "0")
                                , ("width", px segmentWidth)
                                , ("height", px segmentWidth)
                                , ("background", "#333")
                                , ("border-radius", "100%")
                                ]
                            ]
                            []

                        , Html.div
                            [ HA.style
                                [ ("position", "absolute")
                                , ("right", px (-segmentWidth // 2))
                                , ("top", "0")
                                , ("width", px segmentWidth)
                                , ("height", px segmentWidth)
                                , ("background", "#333")
                                , ("border-radius", "100%")
                                ]
                            ]
                            []
                        ]

                    Below ->
                        [ Html.div
                            [ HA.style
                                [ ("position", "absolute")
                                , ("top", "0")
                                , ("bottom", "0")
                                , ("right", "0")
                                , ("width", "calc(100% - " ++ px segmentWidth ++ " - 2px)")
                                , ("background", "#333")
                                ]
                            ]
                            [ Html.div
                                [ HA.style
                                    [ ("position", "absolute")
                                    , ("left", px (-segmentWidth // 2))
                                    , ("top", "0")
                                    , ("width", px segmentWidth)
                                    , ("height", px segmentWidth)
                                    , ("background", "#333")
                                    , ("border-radius", "100%")
                                    ]
                                ]
                                []
                            ]

                        , Html.div
                            [ HA.style
                                [ ("position", "absolute")
                                , ("right", px (-segmentWidth // 2))
                                , ("top", "0")
                                , ("width", px segmentWidth)
                                , ("height", px segmentWidth)
                                , ("background", "#333")
                                , ("border-radius", "100%")
                                ]
                            ]
                            []
                        ])


type SegmentDest
    = ToSide
    | ToCorner



-- UTILITIES


px : Int -> String
px pixels =
    toString pixels ++ "px"
