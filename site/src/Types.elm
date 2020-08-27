module Types exposing (..)

import Account.Types
import Cats.Types
import Counter.Types
import Router.Types


type alias Model =
    { router : Router.Types.Model
    , cats : Cats.Types.Model
    , counter : Counter.Types.Model
    , account : Account.Types.Model
    }


type Msg
    = MsgForRouter Router.Types.Msg
    | MsgForCats Cats.Types.Msg
    | MsgForCounter Counter.Types.Msg
    | MsgForAccount Account.Types.Msg
