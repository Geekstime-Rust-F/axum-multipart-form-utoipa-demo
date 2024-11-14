use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{response::IntoResponse, routing::post, Router};
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(upload),
    )]
    struct ApiDoc;

    let router = Router::new()
        .route("/upload", post(upload))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    println!("listening on {}", address);
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

#[derive(ToSchema)]
#[allow(unused)]
struct UploadedFile {
    file: Vec<u8>,
}

#[utoipa::path(
    post,
    path = "/upload",
    request_body(content_type = "multipart/formdata", content = UploadedFile, description = "File to upload"),
    responses(
        (status = 200, description = "OK")
    )
)]
async fn upload() -> impl IntoResponse {
    todo!()
}
