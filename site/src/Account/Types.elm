module Account.Types exposing (..)

import Form exposing (Form)
import RemoteData exposing (WebData)


type alias Model =
    { newAccountForm : Form () NewAccountForm
    , newAccountResponse : WebData ()
    }


type alias NewAccountForm =
    { productName : String
    , email : String
    , password : String
    , passwordConfirmation : String
    }


type Msg
    = NoOp
    | UpdateNewAccountForm Form.Msg
    | SubmitNewAccount
    | ResponseNewAccount (WebData ())
