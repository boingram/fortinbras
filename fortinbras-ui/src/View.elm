module View exposing (..)

import Html exposing (Html, div, text)
import Html.App
import Items.Read
import Messages exposing (Msg(..))
import Models exposing (Model)
import Nav.View
import Routing exposing (..)


view : Model -> Html Msg
view model =
    div []
        [ nav model
        , page model
        ]


nav : Model -> Html Msg
nav model =
    Html.App.map NavMsg (Nav.View.view model.nav)


page : Model -> Html Msg
page model =
    case model.nav.selectedRoute of
        HomeRoute ->
            notImplementedView

        ReadItemRoute ->
            Html.App.map ItemsMsg (Items.Read.view model.item)

        CreateItemRoute ->
            notImplementedView

        DeleteItemRoute ->
            notImplementedView

        NotFoundRoute ->
            notImplementedView


notImplementedView : Html msg
notImplementedView =
    div []
        [ text "Not implemented"
        ]
