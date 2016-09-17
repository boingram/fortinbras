module Nav.View exposing (..)

import Html exposing (..)
import Html.Attributes exposing (href)
import Html.Events exposing (onClick)
import Nav.Messages exposing (..)
import Nav.Models exposing (Nav)
import Routing exposing (..)


view : Nav -> Html Msg
view nav =
    div []
        [ links nav ]


links : Nav -> Html Msg
links nav =
    div []
        [ ul []
            [ li [] [ clickable nav HomeRoute "Home" ]
            , li [] [ clickable nav CreateItemRoute "Create" ]
            , li [] [ clickable nav DeleteItemRoute "Delete" ]
            , li [] [ clickable nav ReadItemRoute "Read" ]
            ]
        ]


clickable : Nav -> Route -> String -> Html Msg
clickable nav route linkText =
    button [ onClick (RouteSelected route) ] [ text linkText ]
