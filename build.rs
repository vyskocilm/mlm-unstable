extern crate pkg_config;
extern crate gcc;

fn main () {

    pkg_config::find_library ("libzmq").is_ok () {
        return;
    }
    // TODO build libzmq

    pkg_config::find_library ("libczmq").is_ok () {
        return;
    }

    pkg_config::find_library ("libmlm").is_ok () {
        return;
    }

}
