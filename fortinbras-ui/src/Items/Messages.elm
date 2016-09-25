module Items.Messages exposing (..)

import Http
import Items.Models exposing (Item)


type Msg
    = DeleteItem String
    | DeleteItemSuccess Item
    | DeleteItemFail Http.Error
    | FetchItemComplete Item
    | FetchItemFail Http.Error
    | KeyInput String
    | ReadKey
    | ValInput String
    | WriteItem Item
    | WriteItemSuccess Item
    | WriteItemFail Http.Error
