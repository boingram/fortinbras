module Items.Write exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, placeholder, value)
import Html.Events exposing (onClick, onInput)
import Items.Messages exposing (..)
import Items.Models exposing (inputAsItem, Item, unwrap)
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
        [ input [ placeholder "Key", onInput KeyInput, value (unwrap item.inputKey) ] []
        , input [ placeholder "Value", onInput ValInput, value (unwrap item.inputVal) ] []
        , button [ onClick (WriteItem (inputAsItem item)) ]
            [ text "Write" ]
        ]


showItemOnSuccess : Item -> Html Msg
showItemOnSuccess item =
    if item.key /= Nothing then
        div [] [ itemHeader item, itemFields item ]
    else
        div [] []
