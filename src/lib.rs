const ENDPOINT: &str = "https://public-api.gitpoap.io";
pub mod response;
pub mod status;
pub mod v1 {
    use super::response::*;
    use super::status::*;
    use super::ENDPOINT;
    use cfg_if::cfg_if;
    use std::error::Error;

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use gloo_net::http::Request;
        }
    }

    pub fn get_base_url() -> String {
        format!("{}/v1", ENDPOINT)
    }

    /// GET /v1/poap/:poapTokenId/is-gitpoap
    /// This endpoint allows users to query whether some poapTokenId is a GitPOAP or not.
    /// In the case that the poapTokenId corresponds to some claimed GitPOAP,
    /// the API will return something like:
    ///
    /// # Example
    ///
    /// ```
    /// use gitpoap_rs::v1::is_gitpoap;
    /// #[tokio::main]
    /// async fn main() {
    ///     let poap_token_id = "4003";
    ///     let response = is_gitpoap(poap_token_id).await;
    ///     assert!(response.is_ok());
    ///     let poap_response = response.unwrap();
    ///     assert_eq!(poap_response.is_gitpoap, false);
    /// }
    /// ```
    pub async fn is_gitpoap(poap_token_id: &str) -> Result<PoapResponse, Box<dyn Error>> {
        let url = format!("{}/poap/{poap_token_id}/is-gitpoap", get_base_url());

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let response = Request::get(&url).send().await?;
                let res = response.json::<PoapResponse>().await?;
                Ok(res)
            } else {
                let response = reqwest::get(&url).await?;
                let res = response.json::<PoapResponse>().await?;
                Ok(res)
            }
        }
    }

    /// GET /v1/poap/gitpoap-ids
    /// This endpoint retrieves all POAP token IDs that are recognized as GitPOAPs.
    ///
    /// # Example
    ///
    /// ```
    /// use gitpoap_rs::v1::get_gitpoap_ids;
    /// #[tokio::main]
    /// async fn main() {
    ///     let response = get_gitpoap_ids().await;
    ///     assert!(response.is_ok());
    ///     let poap_ids_response = response.unwrap();
    ///     assert!(!poap_ids_response.poap_token_ids.is_empty());
    /// }
    /// ```
    pub async fn get_gitpoap_ids() -> Result<PoapIdsResponse, Box<dyn Error>> {
        let url = format!("{}/poap/gitpoap-ids", get_base_url());

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let response = Request::get(&url).send().await?;
                let res = response.json::<PoapIdsResponse>().await?;
                Ok(res)
            } else {
                let response = reqwest::get(&url).await?;
                let res = response.json::<PoapIdsResponse>().await?;
                Ok(res)
            }
        }
    }

    /// GET /v1/poap-event/:poapEventId/is-gitpoap
    /// This endpoint checks whether a specific poapEventId is a GitPOAP event.
    ///
    /// # Example
    ///
    /// ```
    /// use gitpoap_rs::v1::is_gitpoap_event;
    /// #[tokio::main]
    /// async fn main() {
    ///     let poap_event_id = "12345";
    ///     let response = is_gitpoap_event(poap_event_id).await;
    ///     assert!(response.is_ok());
    ///     let event_response = response.unwrap();
    ///     assert_eq!(event_response.is_gitpoap, false);
    /// }
    /// ```
    pub async fn is_gitpoap_event(poap_event_id: &str) -> Result<EventResponse, Box<dyn Error>> {
        let url = format!("{}/poap-event/{poap_event_id}/is-gitpoap", get_base_url());

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let response = Request::get(&url).send().await?;
                let res = response.json::<EventResponse>().await?;
                Ok(res)
            } else {
                let response = reqwest::get(&url).await?;
                let res = response.json::<EventResponse>().await?;
                Ok(res)
            }
        }
    }

    /// GET /v1/github/user/:githubHandle/gitpoaps?status=<status>
    /// This endpoint allows users to query for minted GitPOAPs associated with
    /// a specified GitHub handle.
    /// The status query parameter is one of the following:
    /// claimed, unclaimed, pending, minting, and can be omitted completely.
    /// This returns data like:
    /// ```
    /// use gitpoap_rs::v1::get_gitpoaps_for_github_user;
    /// #[tokio::main]
    /// async fn main() {
    ///     let github_handle = "woxjro";
    ///     let response = get_gitpoaps_for_github_user(github_handle, None).await;
    ///     assert!(response.is_ok());
    ///     let gitpoaps_response = response.unwrap();
    ///     assert_eq!(gitpoaps_response.0.is_empty(), false);
    /// }
    /// ```
    pub async fn get_gitpoaps_for_github_user(
        github_handle: &str,
        status: Option<Status>,
    ) -> Result<GitpoapsResponse, Box<dyn Error>> {
        let url = format!(
            "{}/github/user/{github_handle}/gitpoaps?status={status}",
            get_base_url(),
            status = status.unwrap_or(Status::Claimed).to_string()
        );

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let response = Request::get(&url).send().await?;
                let res = response.json::<GitpoapsResponse>().await?;
                Ok(res)
            } else {
                let response = reqwest::get(&url).await?;
                let res = response.json::<GitpoapsResponse>().await?;
                Ok(res)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_deserialize_gitpoap_response() {
            let json = r#"{"isGitPOAP":true,"gitPOAPId":4003}"#;
            let response: PoapResponse = serde_json::from_str(json).unwrap();
            assert!(response.is_gitpoap);
            assert_eq!(response.gitpoap_id.unwrap(), 4003);
        }

        #[tokio::test]
        async fn test_is_gitpoap() {
            let poap_token_id = "4003";
            let response = is_gitpoap(poap_token_id).await;
            assert!(response.is_ok());
            let poap_response = response.unwrap();
            assert!(!poap_response.is_gitpoap);
        }

        #[tokio::test]
        async fn test_get_gitpoap_ids() {
            let response = get_gitpoap_ids().await;
            assert!(response.is_ok());
            let poap_ids_response = response.unwrap();
            assert!(!poap_ids_response.poap_token_ids.is_empty());
        }

        #[tokio::test]
        async fn test_is_gitpoap_event() {
            let poap_event_id = "166421";
            let response = is_gitpoap_event(poap_event_id).await;
            assert!(response.is_ok());
            let event_response = response.unwrap();
            assert!(event_response.is_gitpoap);
        }

        #[tokio::test]
        async fn test_get_gitpoaps_for_github_user() {
            let github_handle = "octocat";
            let response = get_gitpoaps_for_github_user(github_handle, None).await;
            assert!(response.is_ok());
            let gitpoaps_response = response.unwrap();
            assert!(gitpoaps_response.0.is_empty());
        }
    }
} /* v1 */
