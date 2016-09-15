module Routing exposing (..)

import Navigation
import String
import UrlParser exposing (..)


type Route
    = HomeRoute
    | ReadItemRoute
    | CreateItemRoute
    | DeleteItemRoute
    | NotFoundRoute


matchers : Parser (Route -> a) a
matchers =
    oneOf
        [ format HomeRoute (s home)
        , format ReadItemRoute (s readItem)
        , format CreateItemRoute (s createItem)
        , format DeleteItemRoute (s deleteItem)
        ]


hashParser : Navigation.Location -> Result String Route
hashParser location =
    location.hash
        |> String.dropLeft 1
        |> parse identity matchers


parser : Navigation.Parser (Result String Route)
parser =
    Navigation.makeParser hashParser


routeFromResult : Result String Route -> Route
routeFromResult result =
    case result of
        Ok route ->
            route

        Err string ->
            NotFoundRoute


createItem : String
createItem =
    "items/create"


deleteItem : String
deleteItem =
    "items/delete"


readItem : String
readItem =
    "items/read"


home : String
home =
    ""
