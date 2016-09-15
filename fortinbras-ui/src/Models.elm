module Models exposing (..)

import Items.Models exposing (Item, newItem)
import Nav.Models exposing (Nav, newNav)
import Routing exposing (Route)


type alias Model =
    { item : Item
    , nav : Nav
    }


initialModel : Model
initialModel =
    { item = newItem "Default Key" Nothing
    , nav = newNav Routing.HomeRoute
    }
