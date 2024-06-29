use gitpoap_rs::v1::get_gitpoaps_for_github_user;

#[tokio::main]
async fn main() {
    let github_handle = "woxjro";
    let response = get_gitpoaps_for_github_user(github_handle, None).await;
    match response {
        Ok(gitpoaps_response) => {
            dbg!("{:?}", gitpoaps_response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
