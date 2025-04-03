#[allow(warnings)]
mod bindings;
mod wallet;
mod utils;
mod musig_steps;

use bindings::Guest;
use klave;

struct Component;

impl Guest for Component {
    fn register_routes(){
        klave::router::add_user_transaction("register-user");
        klave::router::add_user_query("load-current-user");
        klave::router::add_user_query("is-registered");
        klave::router::add_user_query("get-all-users");
        klave::router::add_user_transaction("create-key");
        klave::router::add_user_query("load-keys");
        klave::router::add_user_query("load-key-name-ids");
        klave::router::add_user_query("load-key-pair");
        klave::router::add_user_transaction("create-musig-session-definition");
        klave::router::add_user_query("load-musig-session-ids");
        klave::router::add_user_transaction("update-musig-user-public-key");
        klave::router::add_user_query("load-musig-session");
        klave::router::add_user_transaction("update-musig-aggregation-session");
        klave::router::add_user_transaction("create-public-nonce");
        klave::router::add_user_transaction("create-partial-signature");
        klave::router::add_user_query("verify-final-signature");
    }

    fn register_user(cmd: String){
        wallet::register_user(cmd);
    }

    fn load_current_user(cmd: String){
        wallet::load_current_user(cmd);
    }

    fn is_registered(cmd: String){
        wallet::is_registered(cmd);
    }

    fn get_all_users(cmd: String){
        wallet::get_all_users(cmd);
    }

    fn create_key(cmd: String){
        wallet::create_key(cmd);
    }

    fn load_keys(cmd: String){
        wallet::load_keys(cmd);
    }

    fn load_key_name_ids(cmd: String){
        wallet::load_key_name_ids(cmd);
    }

    fn load_key_pair(cmd: String){
        wallet::load_key_pair(cmd);
    }

    fn create_musig_session_definition(cmd: String){
        wallet::create_musig_session_definition(cmd);
    }

    fn load_musig_session_ids(cmd: String){
        wallet::load_musig_session_ids(cmd);
    }

    fn update_musig_user_public_key(cmd: String){
        wallet::update_musig_user_public_key(cmd);
    }

    fn load_musig_session(cmd: String){
        wallet::load_musig_session(cmd);
    }

    fn update_musig_aggregation_session(cmd: String){
        wallet::update_musig_aggregation_session(cmd);
    }

    fn create_public_nonce(cmd: String) {
        musig_steps::create_public_nonce(cmd);
    }

    fn create_partial_signature(cmd: String) {
        musig_steps::create_partial_signature(cmd);
    }

    fn verify_final_signature(cmd: String) {
        musig_steps::verify_final_signature(cmd);
    }


}

bindings::export!(Component with_types_in bindings);
