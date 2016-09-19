module Items.Models exposing (..)


type alias Item =
    { inputKey : ItemField
    , inputVal : ItemField
    , key : ItemField
    , val : ItemField
    }


type alias ItemField =
    Maybe String


newItem : ItemField -> ItemField -> Item
newItem key val =
    { inputKey = Nothing
    , inputVal = Nothing
    , key = key
    , val = val
    }


blankItem : Item
blankItem =
    { inputKey = Nothing
    , inputVal = Nothing
    , key = Nothing
    , val = Nothing
    }


inputAsItem : Item -> Item
inputAsItem item =
    { item
        | key = item.inputKey
        , val = item.inputVal
    }



-- Unwrap an item field (which is just a type aliased Maybe) into a printable string


unwrap : ItemField -> String
unwrap val =
    case val of
        Just x ->
            x

        Nothing ->
            ""
