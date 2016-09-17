module Items.Models exposing (..)


type alias Item =
    { inputKey : Maybe String
    , key : String
    , val : Value
    }


type alias Value =
    Maybe String


newItem : String -> Value -> Item
newItem key val =
    { inputKey = Nothing
    , key = key
    , val = val
    }



-- Unwrap a value (which is just a type aliased Maybe) into a printable string


unwrap : Value -> String
unwrap val =
    case val of
        Just x ->
            x

        Nothing ->
            ""
