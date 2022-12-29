#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::serde::{json::Json, Deserialize};

mod profile;
use profile::{create_profile, delete_profile, update_profile, get_profile};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Persona<'r> {
    nombre: &'r str,
    edad: u8,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Rocket Overview"
    })
}

#[get("/api")]
fn get_api() -> &'static str {
  "Api get"
}

#[post("/api", data = "<persona>")]
fn api(persona: Json<Persona<'_>>) -> String {
    let nombre = persona.nombre;
    let edad = persona.edad;
    let mut message = format!("Hola Api");

    if nombre.len() < 1 && edad.to_string().len() < 1 {
        message = format!("Test");
    }
    if nombre != "" && edad > 18 {
        message = format!(
            "Hola {}, tu edad es {}, así que eres mayor de edad",
            nombre, edad
        );
    }
    if nombre != "" && edad < 18 {
        message = format!(
            "Hola {}, tu edad es {}, así que eres menor de edad",
            nombre, edad
        );
    }

    println!("{}", message);

    return message;
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("notFound", context! {
        msg: "Not Found"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, api, get_api])
        .mount(
            "/profile",
            routes![get_profile, create_profile, update_profile, delete_profile],
        )
        .attach(Template::fairing())
}
