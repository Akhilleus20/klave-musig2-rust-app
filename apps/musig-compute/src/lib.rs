#[allow(warnings)]
mod bindings;
mod musig_agg;

use bindings::Guest;
use klave;

struct Component;

impl Guest for Component {

    fn register_routes(){
        klave::router::add_user_transaction("initiate-musig-context");
        klave::router::add_user_transaction("initiatemusigcontext");
        klave::router::add_user_transaction("test-set-vec");
        klave::router::add_user_query("load-musig-session-ids");
        klave::router::add_user_transaction( "load-musig-session");
        klave::router::add_user_transaction( "submit-pub-nonce");
        klave::router::add_user_transaction( "submit-partial-signature");
        klave::router::add_user_transaction( "get-final-signature");
    }

    fn initiate_musig_context(cmd: String){
        musig_agg::initiate_musig_context(cmd);
    }

    fn initiatemusigcontext(cmd: String){
        musig_agg::initiate_musig_context(cmd);
    }

    fn load_musig_session_ids(_cmd: String){
        musig_agg::load_musig_session_ids(_cmd);
    }

    fn load_musig_session(cmd: String) {
        musig_agg::load_musig_session(cmd);
    }

    fn submit_pub_nonce(cmd: String){
        musig_agg::submit_pub_nonce(cmd);
    }

    fn submit_partial_signature(cmd: String) {
        musig_agg::submit_partial_signature(cmd);
    }

    fn get_final_signature(cmd: String) {
        musig_agg::get_final_signature(cmd);
    }
}

bindings::export!(Component with_types_in bindings);
