module Account.Update exposing (..)

import Account.Data exposing (createAccount)
import Account.Types exposing (..)
import Form exposing (Form)
import Form.Validate as Validate exposing (..)
import RemoteData exposing (..)
import Return exposing (Return, return)
import Types


init : Return Msg Model
init =
    return
        { newAccountForm = Form.initial [] validate
        , newAccountResponse = NotAsked
        }
        Cmd.none


update : Types.Msg -> Model -> Return Msg Model
update msgFor model =
    case msgFor of
        Types.MsgForAccount msg ->
            updateAccount msg model

        _ ->
            return model Cmd.none


updateAccount : Msg -> Model -> Return Msg Model
updateAccount msg model =
    let
        form =
            model.newAccountForm
    in
    case msg of
        NoOp ->
            return model Cmd.none

        UpdateNewAccountForm formMsg ->
            return { model | newAccountForm = Form.update validate formMsg form } Cmd.none

        SubmitNewAccount ->
            let
                submittedForm =
                    Form.update validate Form.Submit form
            in
            case Form.getOutput submittedForm of
                Just validForm ->
                    return
                        { model | newAccountForm = submittedForm, newAccountResponse = Loading }
                        (createAccount validForm)

                Nothing ->
                    return { model | newAccountForm = submittedForm } Cmd.none

        ResponseNewAccount response ->
            return { model | newAccountResponse = response } Cmd.none


validate : Validation () NewAccountForm
validate =
    Validate.succeed NewAccountForm
        |> Validate.andMap (field "productName" Validate.string)
        |> Validate.andMap (field "email" Validate.email)
        |> Validate.andMap
            (field "password"
                (Validate.string
                    |> Validate.andThen (Validate.minLength 3)
                )
            )
        |> Validate.andMap
            (Validate.map2 (\a b -> ( a, b )) (field "password" Validate.string) (field "passwordConfirmation" Validate.string)
                |> Validate.andThen
                    (\( a, b ) ->
                        if a == b then
                            Validate.succeed a

                        else
                            field "passwordConfirmation" (\_ -> Err <| customError ())
                    )
            )
