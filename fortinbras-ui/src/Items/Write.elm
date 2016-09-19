module Items.Write exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, placeholder)
import Html.Events exposing (onClick, onInput)
import Items.Messages exposing (..)
import Items.Models exposing (inputAsItem, Item)
import Items.Read exposing (itemFields, itemHeader)


view : Item -> Html Msg
view item =
    div [ class "main" ]
        [ form item
        , showItemOnSuccess item
        ]


form : Item -> Html Msg
form item =
    div []
        [ input [ placeholder "Key", onInput KeyInput ] []
        , input [ placeholder "Value", onInput ValInput ] []
        , button [ onClick (WriteItem (inputAsItem item)) ]
            [ text "Write" ]
        ]


showItemOnSuccess : Item -> Html Msg
showItemOnSuccess item =
    if item.key /= Nothing then
        div [] [ itemHeader item, itemFields item ]
    else
        div [] []
