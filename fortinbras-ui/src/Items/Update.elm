module Items.Update exposing (..)

import Items.Messages exposing (Msg(..))
import Items.Models exposing (Item)


update : Msg -> Item -> ( Item, Cmd Msg )
update message item =
    case message of
        NoOp ->
            ( item, Cmd.none )
