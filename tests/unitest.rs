#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio;
    use deadpool_tiberius::SqlServerResult;
    use futures_lite::stream::StreamExt;

    #[tokio::test]
    async fn login() {
        async fn main() -> SqlServerResult<()> {
            let pool = deadpool_tiberius::Manager::new()
                .host("localhost") // default to localhost
                .port(1433) // default to 1433
                .basic_authentication("username", "password")
                .database("database")
                .trust_cert()
                .max_size(10)
                .wait_timeout(Duration::from_secs_f64(1.52))
                .pre_recycle_sync(|_client, _metrics| {
                    // do sth with client object and pool metrics
                    Ok(())
                })
                .create_pool()?;

            let mut conn = pool.get().await?;
            let mut rows = conn.simple_query("SELECT * FROM my_table").await?;
            while let Some(v) = rows.try_next().await? {
                dbg!(&v);
            }
            Ok(())
        }
        main().await.unwrap();
    }

    #[tokio::test]
    async fn t2() {
        async fn should_fail() -> SqlServerResult<()> {
            let pool = deadpool_tiberius::Manager::new()
                .create_pool()?;
            println!("pool created");
            let _ = pool.get().await?;
            Ok(())
        }
        assert!(should_fail().await.is_err());
    }

    #[tokio::test]
    async fn t3() -> SqlServerResult<()> {
        const CONN_STR: &str = "Driver={SQL Server};Integrated Security=True;\
                                Server=DESKTOP-TTTTTTT;Database=master;\
                                Trusted_Connection=yes;encrypt=DANGER_PLAINTEXT;";
        let pool = deadpool_tiberius::Manager::from_ado_string(CONN_STR)?
            .create_pool()?;
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
        Ok(())
    }
}