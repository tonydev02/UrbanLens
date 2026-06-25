use std::io;

use actix_web::{App, HttpServer, middleware::NormalizePath, web};
use urbanlens_api::{
    ApiConfig, RequestId, bounded_cors, build_schema, configure_routes, create_pool, request_logger,
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,urbanlens_api=debug"),
    );

    let config = ApiConfig::from_env().map_err(io::Error::other)?;
    let pool = create_pool(&config).await.map_err(io::Error::other)?;
    let schema = build_schema(pool.clone());
    let bind_address = config.bind_address();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(request_logger())
            .wrap(RequestId)
            .wrap(bounded_cors(&config))
            .wrap(NormalizePath::trim())
            .configure(configure_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
