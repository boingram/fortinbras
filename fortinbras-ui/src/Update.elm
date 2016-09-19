module Update exposing (..)

import Items.Models exposing (blankItem)
import Items.Update
import Messages exposing (Msg(..))
import Models exposing (Model)
import Nav.Update


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ItemsMsg subMsg ->
            let
                ( updatedItem, cmd ) =
                    Items.Update.update subMsg model.item
            in
                ( { model | item = updatedItem }, Cmd.map ItemsMsg cmd )

        NavMsg navMsg ->
            let
                ( updatedNav, cmd ) =
                    Nav.Update.update navMsg model.nav
            in
                ( { model | nav = updatedNav, item = blankItem }, Cmd.map NavMsg cmd )
