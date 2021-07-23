use egg_mode::user::TwitterUser;
use futures::{StreamExt, TryStreamExt};
use std::io::{self, Write};

const CONSUMER_KEY: &str= "";
const CONSUMER_SECRET: &str = "";

struct Login {
    token: egg_mode::auth::Token,
    screen_name: String,
    user_id: u64,
}

impl Login {
    fn new(token: egg_mode::auth::Token, screen_name: String, user_id: u64) -> Self {
        Self {
            token,
            screen_name,
            user_id,
        }
    }
}

trait Trim {
    fn to_string_vec(&self) -> Vec<String>;
}

impl Trim for Vec<TwitterUser> {
    fn to_string_vec(&self) -> Vec<String> {
        let mut x: Vec<String> = vec![];
        for i in self {
            &x.push(i.screen_name.to_string());
        }
        x
    }
}

#[tokio::main]
async fn main() {
    let con_token = egg_mode::KeyPair::new(
        CONSUMER_KEY,
        CONSUMER_SECRET
    );
    let request_token = egg_mode::auth::request_token(&con_token, "oob")
        .await
        .unwrap();
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    println!("Url: {}", &auth_url);
    let mut value = String::new();
    print!("Type in PIN: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value).unwrap();
    let verifier = &value;

    let (token, user_id, screen_name) =
        egg_mode::auth::access_token(con_token, &request_token, verifier)
            .await
            .expect("Wrong PIN!");
    let login: Login = Login::new(token, screen_name, user_id);
    /* Prints user's login details.
    println!(
        "* Token: {:?}\n* User ID: {}\n* Screen Name: {}",
        login.token, login.user_id, login.screen_name
    );
    */
    println!("Verified users are NOT included in either of the lists.");
    println!("Made by Jean");
    let following: Vec<_> =
        egg_mode::user::friends_of(login.screen_name.to_owned(), &login.token)
            .map_ok(|r| r.response)
            .try_filter(|u| futures::future::ready(!u.verified))
            .try_collect::<Vec<TwitterUser>>()
            .await
            .unwrap()
            .to_string_vec();
    let followers: Vec<_> =
        egg_mode::user::followers_of(login.screen_name.to_owned(), &login.token)
            .map_ok(|r| r.response)
            .try_filter(|u| futures::future::ready(!u.verified))
            .try_collect::<Vec<TwitterUser>>()
            .await
            .unwrap()
            .to_string_vec();
    println!("Users who aren't following you back:");
    following
        .iter()
        .filter(|&v| !followers.contains(&v))
        .for_each(|v| println!("* @{}", v));
    println!("Users who YOU aren't following back:");
    followers
        .iter()
        .filter(|&v| !following.contains(&v))
        .for_each(|v| println!("* @{}", v));
}
