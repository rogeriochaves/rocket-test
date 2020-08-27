module AccountTest exposing (suite)

import Main exposing (initialize)
import ProgramTest exposing (clickButton, expectViewHas, simulateHttpResponse, start, withBaseUrl, withSimulatedEffects)
import Test exposing (..)
import Test.Html.Selector exposing (text)
import Test.Http
import Update exposing (initNoKey)


suite : Test
suite =
    todo "setup program test better"



-- test "create new account" <|
--     \() ->
--         ProgramTest.createApplication
--             { init = initNoKey
--             , update = initialize.update
--             , view = initialize.view
--             , onUrlChange = initialize.onUrlChange
--             , onUrlRequest = initialize.onUrlRequest
--             }
--             -- |> withSimulatedEffects
--             |> withBaseUrl "http://localhost/account/create"
--             |> start ()
--             |> simulateHttpResponse "POST"
--                 "http://localhost:4001/account/create"
--                 (Test.Http.httpResponse
--                     { statusCode = 500
--                     , headers = []
--                     , body = "internal server error"
--                     }
--                 )
--             |> clickButton "Get started"
--             |> expectViewHas
--                 [ text "Something went wrong, please try again"
--                 ]
