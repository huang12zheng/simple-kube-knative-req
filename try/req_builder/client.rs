use async_once::AsyncOnce;
use kube::Client;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> =
        AsyncOnce::new(async { Client::try_default().await.unwrap() });
}
