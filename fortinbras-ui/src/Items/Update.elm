module Items.Update exposing (..)

import Items.Commands exposing (fetchItem)
import Items.Messages exposing (Msg(..))
import Items.Models exposing (Item, unwrap)


update : Msg -> Item -> ( Item, Cmd Msg )
update message item =
    case message of
        FetchItemComplete item ->
            ( item, Cmd.none )

        FetchItemFail error ->
            ( item, Cmd.none )

        KeyInput key ->
            ( { item | inputKey = Just key }, Cmd.none )

        ReadKey ->
            ( item, fetchItem (unwrap item.inputKey) )
