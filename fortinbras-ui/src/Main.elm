module Main exposing (..)

import Html.App
import Items.Commands exposing (fetchItem)
import Messages exposing (Msg(..))
import Models exposing (Model, initialModel)
import Nav.Models exposing (newNav)
import Navigation
import Routing exposing (Route)
import View exposing (view)
import Update exposing (update)


init : Result String Route -> ( Model, Cmd Msg )
init result =
    let
        currentRoute =
            Routing.routeFromResult result
    in
        ( initialModel currentRoute, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


urlUpdate : Result String Route -> Model -> ( Model, Cmd Msg )
urlUpdate result model =
    let
        currentRoute =
            Routing.routeFromResult result
    in
        ( { model | nav = newNav currentRoute }, Cmd.none )



-- MAIN


main : Program Never
main =
    Navigation.program Routing.parser
        { init = init
        , view = view
        , update = update
        , urlUpdate = urlUpdate
        , subscriptions = subscriptions
        }
