module Nav.Models exposing (..)

import Routing exposing (Route)


type alias Nav =
    { selectedRoute : Route
    }


newNav : Route -> Nav
newNav route =
    { selectedRoute = route }
