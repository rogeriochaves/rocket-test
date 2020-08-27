module Account.View exposing (..)

import Account.Types exposing (..)
import Element exposing (..)
import Element.Border as Border
import Element.Input as Input
import Element.Region as Region
import Form
import Form.Field
import RemoteData exposing (..)
import Styles exposing (..)
import Utils exposing (onEnter)


create : Model -> Element Msg
create model =
    column [ spacing 20, centerX ]
        [ paragraph (Styles.title ++ [ Region.heading 1, paddingXY 0 30 ])
            [ text "Create New Account"
            ]
        , field model
            "productName"
            (\value onChange attributes ->
                Input.text attributes
                    { onChange = onChange
                    , placeholder = Nothing
                    , text = value
                    , label = Input.labelAbove [] <| text "Name of your product"
                    }
            )
        , field model
            "email"
            (\value onChange attributes ->
                Input.email attributes
                    { onChange = onChange
                    , placeholder = Nothing
                    , text = value
                    , label = Input.labelAbove [] <| text "Your email"
                    }
            )
        , field model
            "password"
            (\value onChange attributes ->
                Input.newPassword attributes
                    { onChange = onChange
                    , placeholder = Nothing
                    , text = value
                    , label = Input.labelAbove [] <| text "New password"
                    , show = False
                    }
            )
        , field model
            "passwordConfirmation"
            (\value onChange attributes ->
                Input.newPassword attributes
                    { onChange = onChange
                    , placeholder = Nothing
                    , text = value
                    , label = Input.labelAbove [] <| text "Confirm password"
                    , show = False
                    }
            )
        , submitArea model
        ]


field : Model -> String -> (String -> (String -> Msg) -> List (Attribute Msg) -> Element Msg) -> Element Msg
field model name elem =
    let
        state =
            Form.getFieldAsString name model.newAccountForm

        onChange text =
            UpdateNewAccountForm (Form.Input name Form.Text (Form.Field.String text))

        attributes =
            case state.liveError of
                Just _ ->
                    [ onEnter SubmitNewAccount, Border.color (rgb 0.9 0 0) ]

                Nothing ->
                    [ onEnter SubmitNewAccount ]
    in
    case state.value of
        Just value ->
            elem value onChange attributes

        Nothing ->
            elem "" onChange attributes


submitArea : Model -> Element Msg
submitArea model =
    row [ spaceEvenly, width fill ]
        (case model.newAccountResponse of
            NotAsked ->
                [ submitButton ]

            Loading ->
                [ Input.button (Styles.button ++ [ alignRight, padding 10, width (px 150) ])
                    { onPress = Nothing
                    , label = image [ width (px 30) ] { src = "/loading.svg", description = "loading" }
                    }
                ]

            Failure _ ->
                [ el Styles.errorText (text "Something went wrong")
                , submitButton
                ]

            Success _ ->
                []
        )


submitButton : Element Msg
submitButton =
    Input.button (Styles.button ++ [ alignRight, padding 20, alignRight ])
        { onPress = Just SubmitNewAccount
        , label = text "Get started"
        }
