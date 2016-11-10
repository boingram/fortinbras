module Nav.View exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, href)
import Html.Events exposing (onClick)
import Nav.Messages exposing (..)
import Nav.Models exposing (Nav)
import Routing exposing (..)


view : Nav -> Html Msg
view nav =
    div [ class "nav" ]
        [ links nav ]


links : Nav -> Html Msg
links nav =
    div []
        [ ul []
            [ li [] [ clickable nav CreateItemRoute "Create" ]
            , li [] [ clickable nav DeleteItemRoute "Delete" ]
            , li [] [ clickable nav ReadItemRoute "Read" ]
            ]
        ]


clickable : Nav -> Route -> String -> Html Msg
clickable nav route linkText =
    a
        [ href ("#" ++ (Routing.getPath route))
        , onClick (RouteSelected route)
        ]
        [ text linkText ]
