module View exposing (..)

import Html exposing (Html, div, text)
import Html.App
import Html.Attributes exposing (class)
import Items.Delete
import Items.Models exposing (blankItem)
import Items.Read
import Items.Write
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
            Html.App.map ItemsMsg (Items.Write.view model.item)

        DeleteItemRoute ->
            Html.App.map ItemsMsg (Items.Delete.view model.item)

        NotFoundRoute ->
            notImplementedView


notImplementedView : Html msg
notImplementedView =
    div [ class "main" ]
        [ text "Not implemented"
        ]
