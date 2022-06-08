mod request;
use clap::Command;
use dotenv::dotenv;
use nbot_twitter_rs::endpoints::Data;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Command::new("nbot")
        .version("0.1.0")
        .author("n0pj")
        .about("nbot is a scraping bot")
        .subcommand(Command::new("twitter"));

    let matches = app.get_matches();

    if let Some(_matches) = matches.subcommand_matches("twitter") {
        println!("twitter");

        let username = env::var("TWITTER_COLLECT_TARGET_USERNAME")
            .expect("TWITTER_COLLECT_TARGET_USERNAME not found");

        // get user_id
        let url = nbot_twitter_rs::endpoints::user::users_by_username::url(&username);
        let resp = request::request::<
            Data<nbot_twitter_rs::endpoints::user::users_by_username::User>,
        >(&url, None)
        .await;

        let user_id = resp.unwrap().data.id;

        // get user_follows
        let url = nbot_twitter_rs::endpoints::follows::users_a_user_id_is_following::url(&user_id);
        let mut user_follows = request::request::<
            Data<Vec<nbot_twitter_rs::endpoints::follows::users_a_user_id_is_following::User>>,
        >(&url, Some(&[("max_results", "200")]))
        .await;

        // if user_follows.meta.next_token is not None, loop until user_follows.meta.next_token is None
        while user_follows
            .as_ref()
            .unwrap()
            .meta
            .as_ref()
            .unwrap()
            .next_token
            .is_some()
        {
            let next_token = user_follows.unwrap().meta.unwrap().next_token.unwrap();
            let url =
                nbot_twitter_rs::endpoints::follows::users_a_user_id_is_following::url(&user_id);
            user_follows = request::request::<
                Data<Vec<nbot_twitter_rs::endpoints::follows::users_a_user_id_is_following::User>>,
            >(
                &url,
                Some(&[("max_results", "200"), ("pagination_token", &next_token)]),
            )
            .await;

            println!("{:?}", user_follows);
        }
    }
}
