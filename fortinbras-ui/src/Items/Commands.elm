module Items.Commands exposing (..)

import Http
import Items.Messages exposing (..)
import Items.Models exposing (Item, Value)
import Json.Decode as Decode exposing ((:=))
import Task


fetchItem : String -> Cmd Msg
fetchItem key =
    Http.get collectionDecoder (fetchItemUrl key)
        |> Task.perform FetchItemFail FetchItemComplete


fetchItemUrl : String -> String
fetchItemUrl key =
    "http://localhost:7341/items?key=" ++ key


collectionDecoder : Decode.Decoder Item
collectionDecoder =
    Decode.object3 Item
        (Decode.maybe ("inputKey" := Decode.string))
        ("key" := Decode.string)
        (Decode.maybe ("val" := Decode.string))
