module Styles exposing (button, errorText, title)

import Element exposing (..)
import Element.Background as Background
import Element.Font as Font


title : List (Attr decorative msg)
title =
    [ Font.color (rgb 0 0 0)
    , Font.size 40
    ]


button : List (Attr decorative msg)
button =
    [ Background.color (rgb 0.9 0.9 0.9)
    ]


errorText : List (Attr decorative msg)
errorText =
    [ Font.color (rgb 0.9 0 0)
    ]
