module Item.Commands exposing (..)

import Http
import Items.Messages exposing (..)
import Items.Models exposing (Item, Value)
import Json.Decode as Decode exposing ((:=))
import Task


fetchItem : Cmd Msg
fetchItem =
    Http.get collectionDecoder fetchItemUrl
        |> Task.perform FetchItemFail FetchAllComplete


fetchItemUrl : String -> String
fetchItemUrl key =
    "http://localhost:7341/items?key=" ++ key


collectionDecoder : Decode.Decoder Item
collectionDecoder =
    Decode.object2 Item
        ("key" := Decode.string)
        (Decode.maybe ("val" := Decode.string))
