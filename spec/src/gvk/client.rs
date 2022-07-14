use async_once::AsyncOnce;
use kube::Client;
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: AsyncOnce<Client> =
        AsyncOnce::new(async { Client::try_default().await.unwrap() });
}
/// static [Client]
///
/// No snap data due to [Client] is not Debug
pub async fn get_client() -> &'static Client {
    return CLIENT.get().await;
}
