use super::state;
use crate::pyrocord::http::request::Request;
use crate::pyrocord::http::routes::Route;
use crate::pyrocord::payloads::gateway;

pub struct ClientController {}

impl ClientController {
    async fn get_gateway(&self) -> gateway::GatewayPayload {
        state::HTTP
            .get()
            .request::<gateway::GatewayPayload>(Request {
                body: None,
                headers: None,
                route: Route::GetGateway,
            })
            .await
            .expect("Replace with a Python exception.")
    }

    async fn login(&self, gateway_payload: gateway::GatewayPayload) {
        println!("{:?}", gateway_payload.url);

        // Replace with connection logic.
    }

    pub async fn launch(&self) {
        let gateway = self.get_gateway().await;
        self.login(gateway).await;
    }
}
