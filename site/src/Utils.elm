module Utils exposing (..)

import Element
import Html exposing (Html, div)
import Html.Attributes exposing (property)
import Html.Events
import Json.Decode as Decode
import Json.Encode as Encode


onEnter : msg -> Element.Attribute msg
onEnter msg =
    Element.htmlAttribute
        (Html.Events.on "keyup"
            (Decode.field "key" Decode.string
                |> Decode.andThen
                    (\key ->
                        if key == "Enter" then
                            Decode.succeed msg

                        else
                            Decode.fail "Not the enter key"
                    )
            )
        )
