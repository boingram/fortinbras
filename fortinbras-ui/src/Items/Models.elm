module Items.Models exposing (..)


type alias Item =
    { key : String
    , val : Maybe String
    }


type alias Value =
    Maybe String


newItem : String -> Value -> Item
newItem key val =
    { key = key
    , val = val
    }


unwrap : Value -> String
unwrap val =
    case val of
        Just x ->
            x

        Nothing ->
            ""
