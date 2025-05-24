mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;
mod auth;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: u16,
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: status.code })
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Picsou Project!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .register("/", catchers![default_catcher])
        .mount("/", routes![index])
        .mount("/auth", routes![routes::session::login, routes::session::register, routes::session::validate]) // avoir comment adapter à sessions
        .mount("/user", routes![
            routes::user::create_user, // /!\ A voir comment intégrer le register à notre architecture REST

                //{user_id}
                routes::user::get_user_by_id_route,

                    //expenses
                    routes::user::expenses::get_all_expenses_with_user,

                        //{expense_id}
                        routes::user::expenses::get_expense_by_id_with_user,

                //email/{user_email}
                routes::user::get_user_by_email_route,


            routes::user::update_user // a bougerr dans /me
        ])
        .mount("/me", routes![
            routes::me::get_user_me,
            routes::me::update_user_me,
            routes::me::delete_user_me,
            
            //friends
            routes::me::friends::get_all_friends,
            routes::me::friends::add_friend,
            
                //{friend_id}
                routes::me::friends::delete_friend, 
                // PLUS TARD : ajout de la possibilité de gérer ses amis (accepter, refuser, supprimer, bloquer, debloquer, limiter)
                
            //groups
            routes::me::groups::get_all_groups,
            
            
        ])
        .mount("/groups", routes![
            routes::groups::create_groups,

            //expenses
            routes::groups::expenses::get_all_exepenses,

                //{expense_id}
                routes::groups::expenses::get_expense_by_id,
                routes::groups::expenses::update_expense,
                routes::groups::expenses::delete_expense,

            //{group_id}
            routes::groups::get_group_by_id,
            routes::groups::modify_group, // Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de modifier des infos ou non
            routes::groups::delete_group, // Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de supprimer le groupe ou non

                //users
                routes::groups::users::get_all_users_in_group,
                routes::groups::users::add_user_to_group,

                    //{user_id}
                    routes::groups::users::get_user_by_id_in_group, //UserGroup on récupère sont rôle et sont id de user
                    routes::groups::users::update_user_in_group, //Maj de son rôle
                    routes::groups::users::delete_user_in_group, //Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de supprimer un utilisateur ou non

            //expenses
                routes::groups::expenses::get_all_expenses_in_group,
                routes::groups::expenses::create_expense_to_group, //Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de créer une dépense ou non

                    //{expense_id}
                    routes::groups::expenses::get_expense_by_id_in_group,
                    routes::groups::expenses::update_expense_in_group, //Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de modifier une dépense ou non
                    routes::groups::expenses::delete_expense_in_group, //Il faudra prévoir la gestion du status de l'utilisateur dans le groupe pour lui permettre de supprimer une dépense ou non

        ])
}