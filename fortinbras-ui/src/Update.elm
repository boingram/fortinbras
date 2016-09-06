module Update exposing (..)

import Items.Update
import Messages exposing (Msg(..))
import Models exposing (Model)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ItemsMsg subMsg ->
            let
                ( updatedItem, cmd ) =
                    Items.Update.update subMsg model.item
            in
                ( { model | item = updatedItem }, Cmd.map ItemsMsg cmd )
