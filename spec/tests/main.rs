#[tokio::test]
async fn main() -> anyhow::Result<()> {
    use kube::api::PostParams;
    use kube_do_spec::*;

    let api = get_default_api().await;
    let ksvc = KnativeSpec::default().into_do();
    let _r = api.create(&PostParams::default(), &ksvc).await?;
    insta::assert_debug_snapshot!(_r);
    Ok(())
}
