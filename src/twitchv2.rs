use twitch_api2::{
    helix::channels::GetChannelInformationRequest, helix::streams::GetStreamsRequest, HelixClient,
    TwitchClient,
};
use twitch_oauth2::{
    client::reqwest_http_client, tokens::errors::TokenError, AccessToken, AppAccessToken, ClientId,
    ClientSecret, Scope, UserToken,
};

use crate::config::value_string_field_config;

pub async fn appaccesstoken_twitch() -> AppAccessToken {
    let client_id = value_string_field_config("twitch-api-client-id");
    let client_secret = value_string_field_config("twitch-api-client-secret");
    let client_id = ClientId::new(client_id);
    let client_secret = ClientSecret::new(client_secret);
    let token = match AppAccessToken::get_app_access_token(
        twitch_oauth2::client::reqwest_http_client,
        client_id,
        client_secret,
        twitch_oauth2::Scope::all(),
    )
    .await
    {
        Ok(t) => t,
        Err(TokenError::Request(e)) => panic!("got error: {:?}", e),
        Err(e) => panic!(e),
    };

    token
}

// pub async fn usnertoken_twitch() -> twitch_oauth2::UserToken {

//     let appaccesstoken_twitch = appaccesstoken_twitch();

//     let token = AccessToken
//     let usertoken =
//     match UserToken::from_existing(twitch_oauth2::client::reqwest_http_client, appaccesstoken_twitch.await, None).await {
//         Ok(t) => println!("user_token: {}", t.token().secret()),
//         Err(e) => panic!("got error: {}", e),
//     };

//     usertoken

// }

pub async fn get_channel_status() {
    let token = appaccesstoken_twitch().await;

    let client = HelixClient::new();

    let req = GetStreamsRequest::builder()
        .user_login(vec!["mistermv".into()])
        .build();

    let response = client.req_get(req, &token).await.unwrap();
    // println!("{:?}", &client.helix.req_get(req, &token).await.data[0].title);
}
