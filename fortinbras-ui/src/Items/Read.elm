module Items.Read exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, placeholder, value)
import Html.Events exposing (onClick, onInput)
import Items.Messages exposing (..)
import Items.Models exposing (Item, unwrap)


view : Item -> Html Msg
view item =
    div [ class "main" ]
        [ form item
        , itemHeader item
        , itemFields item
        ]


form : Item -> Html Msg
form item =
    div []
        [ input [ placeholder "Key", onInput KeyInput, value (unwrap item.inputKey) ] []
        , button [ onClick ReadKey ] [ text "Read" ]
        ]



-- Print the item's key as a header


itemHeader : Item -> Html Msg
itemHeader item =
    div []
        [ h2 [] [ text (unwrap item.key) ] ]



-- Print the fields on an item into a table


itemFields : Item -> Html Msg
itemFields item =
    div []
        [ table []
            [ tr []
                [ th [] [ text "Key" ]
                , tr [] [ text (unwrap item.key) ]
                ]
            , tr []
                [ th [] [ text "Value" ]
                , tr [] [ text (unwrap item.val) ]
                ]
            ]
        ]
