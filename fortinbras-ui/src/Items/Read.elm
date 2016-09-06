module Items.Read exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class)
import Items.Messages exposing (..)
import Items.Models exposing (Item, unwrap)


view : Item -> Html Msg
view item =
    div []
        [ itemHeader item
        , itemFields item
        ]



-- Print the item's key as a header


itemHeader : Item -> Html Msg
itemHeader item =
    div []
        [ h2 [] [ text item.key ] ]



-- Print the fields on an item into a table


itemFields : Item -> Html Msg
itemFields item =
    div []
        [ table []
            [ tr []
                [ th [] [ text "Key" ]
                , tr [] [ text item.key ]
                ]
            , tr []
                [ th [] [ text "Value" ]
                , tr [] [ text (unwrap item.val) ]
                ]
            ]
        ]
