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
        [ format HomeRoute (s "")
        , format GetItemRoute (s "items/read")
        , format CreateItemRoute (s "items/create")
        , format DeleteItemRoute (s "items/delete")
        ]


hashParser : Navigation.Loction -> Result String Route
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
