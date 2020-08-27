module Main exposing (initialize, main)

import Browser
import Html exposing (..)
import Router.Types
import Types exposing (..)
import Update exposing (init, update)
import View exposing (view)


main : Program () Model Msg
main =
    Browser.application initialize


initialize =
    { init = init
    , view = view
    , update = update
    , subscriptions = always Sub.none
    , onUrlChange = MsgForRouter << Router.Types.OnUrlChange
    , onUrlRequest = MsgForRouter << Router.Types.OnUrlRequest
    }
