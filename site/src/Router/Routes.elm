module Router.Routes exposing (..)

import Browser.Navigation
import Url.Parser exposing ((</>), Parser, map, oneOf, parse, s, top)


type Page
    = Home
    | NotFound
    | CatsPage
    | CounterPage
    | NewAccountPage


routes : Parser (Page -> a) a
routes =
    oneOf
        [ map Home top
        , map NotFound (s "404")
        , map CatsPage (s "cats")
        , map CounterPage (s "counter")
        , map NewAccountPage (s "account" </> s "create")
        ]


toPath : Page -> String
toPath page =
    case page of
        Home ->
            "/"

        NotFound ->
            "/404"

        CatsPage ->
            "/cats"

        CounterPage ->
            "/counter"

        NewAccountPage ->
            "/account/create"
