module Items.Update exposing (..)

import Items.Commands exposing (deleteItem, fetchItem, writeItem)
import Items.Messages exposing (Msg(..))
import Items.Models exposing (blankItem, Item, unwrap)


update : Msg -> Item -> ( Item, Cmd Msg )
update message originalItem =
    case message of
        DeleteItem key ->
            ( originalItem, deleteItem (key) )

        DeleteItemSuccess item ->
            ( item, Cmd.none )

        DeleteItemFail error ->
            ( originalItem, Cmd.none )

        FetchItemComplete item ->
            ( item, Cmd.none )

        FetchItemFail error ->
            ( originalItem, Cmd.none )

        KeyInput key ->
            ( { originalItem | inputKey = Just key }, Cmd.none )

        ReadKey ->
            ( originalItem, fetchItem (unwrap originalItem.inputKey) )

        ValInput val ->
            ( { originalItem | inputVal = Just val }, Cmd.none )

        WriteItem item ->
            ( originalItem, writeItem item )

        WriteItemSuccess item ->
            ( item, Cmd.none )

        WriteItemFail error ->
            ( originalItem, Cmd.none )
