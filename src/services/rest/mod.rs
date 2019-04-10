pub mod connections;
pub mod routes;

pub fn launcher() {
    rocket::ignite().mount("/api", routes![routes::index]).launch();
}
