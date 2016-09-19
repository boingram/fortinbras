module Items.Commands exposing (..)

import Debug exposing (log)
import Http
import Items.Messages exposing (..)
import Items.Models exposing (Item, ItemField, unwrap)
import Json.Decode as Decode exposing ((:=))
import Json.Encode as Encode
import Task


fetchItem : String -> Cmd Msg
fetchItem key =
    Http.get itemDecoder (fetchItemUrl key)
        |> Task.perform FetchItemFail FetchItemComplete


fetchItemUrl : String -> String
fetchItemUrl key =
    "http://localhost:7341/items?key=" ++ key


writeItemTask : Item -> Task.Task Http.Error Item
writeItemTask item =
    let
        body =
            itemEncoded item
                |> Encode.encode 0
                |> Http.string

        config =
            { verb = "POST"
            , headers = [ ( "Content-Type", "application/json" ) ]
            , url = writeItemUrl
            , body = log "body: " body
            }
    in
        Http.send Http.defaultSettings config
            |> Http.fromJson itemDecoder


writeItem : Item -> Cmd Msg
writeItem item =
    writeItemTask item
        |> Task.perform WriteItemFail WriteItemSuccess


writeItemUrl : String
writeItemUrl =
    "http://localhost:7341/items"


itemEncoded : Item -> Encode.Value
itemEncoded item =
    let
        item =
            log "item: " item

        list =
            [ ( "key", Encode.string (unwrap item.key) )
            , ( "val", Encode.string (unwrap item.val) )
            ]
    in
        list
            |> Encode.object


itemDecoder : Decode.Decoder Item
itemDecoder =
    Decode.object4 Item
        (Decode.maybe ("inputKey" := Decode.string))
        (Decode.maybe ("inputVal" := Decode.string))
        (Decode.maybe ("key" := Decode.string))
        (Decode.maybe ("val" := Decode.string))
