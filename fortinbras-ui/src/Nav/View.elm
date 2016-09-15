module Nav.View exposing (..)

import Html exposing (..)
import Html.Attributes exposing (href)
import Nav.Messages exposing (..)
import Nav.Models exposing (Nav)
import Routing exposing (createItem, deleteItem, home, readItem)


view : Nav -> Html Msg
view nav =
    div []
        [ links nav ]


links : Nav -> Html Msg
links nav =
    div []
        [ ul []
            [ li [] [ button nav home "Home" ]
            , li [] [ button nav createItem "Create" ]
            , li [] [ button nav deleteItem "Delete" ]
            , li [] [ button nav readItem "Read" ]
            ]
        ]


button : Nav -> String -> String -> Html Msg
button nav routePath linkText =
    a [ href routePath ] [ text linkText ]
