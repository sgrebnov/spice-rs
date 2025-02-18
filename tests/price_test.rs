#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use spice_rs::Client;
    use std::env;
    use std::path::Path;

    async fn new_client() -> Client {
        dotenv::from_path(Path::new(".env.local")).ok();
        let api_key = env::var("API_KEY").expect("API_KEY not found");
        Client::new(&api_key)
            .await
            .expect("Failed to create client")
    }

    #[tokio::test]
    async fn test_get_supported_pairs() {
        let spice_client = new_client().await;
        let result = spice_client.get_supported_pairs().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_prices() {
        let spice_client = new_client().await;
        let pair = "BTC-USD";
        let result = spice_client.get_prices(&[pair]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_historical_prices() {
        let spice_client = new_client().await;
        let pair1 = "BTC-USD";
        let pair2 = "ETH-USD";

        let start_time = Utc.timestamp_opt(1697669766, 0).single();
        let end_time = Utc.timestamp_opt(1697756166, 0).single();

        let result = spice_client
            .get_historical_prices(&[pair1, pair2], start_time, end_time, Some("1h"))
            .await;
        assert!(result.is_ok());
    }
}
