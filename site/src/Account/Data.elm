module Account.Data exposing (..)

import Account.Types exposing (..)
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import RemoteData exposing (..)


createAccount : NewAccountForm -> Cmd Msg
createAccount form =
    let
        url =
            "/api/account/create"

        returnMsg =
            RemoteData.fromResult >> ResponseNewAccount
    in
    Http.post
        { url = url
        , body = Http.jsonBody (newAccountEncoder form)
        , expect = Http.expectJson returnMsg decodeNewAccountResponse
        }


decodeNewAccountResponse : Decode.Decoder ()
decodeNewAccountResponse =
    Decode.succeed ()



-- Decode.at [ "data", "image_url" ] Decode.string


newAccountEncoder : NewAccountForm -> Encode.Value
newAccountEncoder form =
    Encode.object
        [ ( "product_name", Encode.string form.productName )
        , ( "email", Encode.string form.email )
        , ( "password", Encode.string form.password )
        ]
