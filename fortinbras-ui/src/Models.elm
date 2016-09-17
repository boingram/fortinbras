module Models exposing (..)

import Items.Models exposing (Item, newItem)
import Nav.Models exposing (Nav, newNav)
import Routing exposing (Route)


type alias Model =
    { item : Item
    , nav : Nav
    }


initialModel : Routing.Route -> Model
initialModel route =
    { item = newItem "Default Key" Nothing
    , nav = newNav route
    }
