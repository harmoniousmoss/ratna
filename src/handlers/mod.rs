pub mod blacklist_handler;
pub use blacklist_handler::{
    add_blacklist_ip, delete_blacklist_ip_by_id, edit_blacklist_ip_by_id, get_all_blacklist_ip,
    get_blacklist_ip_by_id,
};

pub mod malicious_handler;
pub use malicious_handler::{
    add_blacklist_url, delete_blacklist_url_by_id, edit_blacklist_url_by_id, get_all_blacklist_url,
    get_blacklist_url_by_id,
};
