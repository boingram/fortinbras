module Items.Delete exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, placeholder, value)
import Html.Events exposing (onClick, onInput)
import Items.Messages exposing (..)
import Items.Models exposing (Item, unwrap)
import Items.Read exposing (itemFields)


view : Item -> Html Msg
view item =
    div [ class "main" ]
        [ form item
        , showItemOnSuccess item
        ]


form : Item -> Html Msg
form item =
    div []
        [ input [ placeholder "Key", onInput KeyInput, value (unwrap item.inputKey) ] []
        , button [ onClick (DeleteItem (unwrap item.inputKey)) ] [ text "Delete" ]
        ]


showItemOnSuccess : Item -> Html Msg
showItemOnSuccess item =
    if item.key /= Nothing then
        div [] [ h3 [] [ text "Delete Successful" ], itemFields item ]
    else
        div [] []
