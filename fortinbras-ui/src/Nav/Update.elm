module Nav.Update exposing (..)

import Nav.Messages exposing (Msg(..))
import Nav.Models exposing (Nav, newNav)


update : Msg -> Nav -> ( Nav, Cmd Msg )
update message prevNav =
    case message of
        RouteSelected route ->
            ( newNav route, Cmd.none )
