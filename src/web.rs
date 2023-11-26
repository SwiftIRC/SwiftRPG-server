extern crate rocket;
extern crate utoipa;
extern crate utoipa_swagger_ui;

pub mod state;

use crate::map;
use state::RocketState;

use map::data::*;
use map::resources::Resources;
use rocket::routes;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn rocket(state: RocketState) {
    #[derive(OpenApi)]
    #[openapi(
        paths(routes::map_endpoint,),
        components(schemas(Point, Points, Coordinate, Resources))
    )]
    struct ApiDoc;

    match rocket::build()
        .manage(state)
        .mount("/api", routes![routes::map_endpoint])
        .mount(
            "/",
            SwaggerUi::new("/docs/api/<_..>").url("/docs/api/openapi.json", ApiDoc::openapi()),
        )
        .launch()
        .await
    {
        Ok(_) => println!("Rocket has shut down."),
        Err(e) => println!("Rocket has crashed: {}", e),
    };
}

mod routes {
    extern crate rocket;

    use crate::map;
    use crate::web::state::RocketState;

    use map::data::*;
    use map::fns::*;
    use map::resources::*;
    use rocket::{get, State};
    use serde_json::json;

    #[utoipa::path(get, path = "/api/map/{x}/{y}",
        responses(
            (status = 200, description = "Tile resources"),
            (status = 400, description = "Invalid coordinates"),
            (status = 401, description = "Invalid coordinates"),
        ),
        params(
            ("x", description = "X coordinate"),
            ("y", description = "Y coordinate"),
        ),
        security(
            ("api_key" = [])
        )
    )]
    #[get("/map/<x>/<y>")]
    pub async fn map_endpoint(x: i32, y: i32, state: &State<RocketState>) -> String {
        let point = Coordinate::new(x, y);

        let converted_point = convert_point(
            point.to_owned(),
            &state.noises,
            state.chunk.width,
            state.chunk.height,
            state.chunk.x1,
            state.chunk.x2,
            state.chunk.y1,
            state.chunk.y2,
            state.chunk.is_seamless,
        );

        let biome = get_biome(converted_point.to_owned());
        let height = get_biome_as_f64(biome.to_owned());

        let resources = Resources::new(&state.noises, [x as f64, y as f64], &biome);

        let point = Point::new(point, converted_point, height, biome, resources);

        json!(point).to_string()
    }
}
