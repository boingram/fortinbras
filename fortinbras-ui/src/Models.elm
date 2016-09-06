module Models exposing (..)

import Items.Models exposing (Item, newItem)


type alias Model =
    { item : Item
    }


initialModel : Model
initialModel =
    { item = newItem "Default Key" Nothing }
