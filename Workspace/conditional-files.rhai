// This file handles conditional files for this specific sub-template.

const IS_WEB_VAR = "is_web";
const IS_DESKTOP_VAR = "is_desktop";
const IS_MOBILE_VAR = "is_mobile";
const IS_FULLSTACK_VAR = "is_fullstack";
const IS_ROUTER_VAR = "is_router";

let is_web = variable::get(IS_WEB_VAR);
if !is_web {
    file::delete("web");
}

let is_desktop = variable::get(IS_DESKTOP_VAR);
if !is_desktop {
    file::delete("desktop");
}

let is_mobile = variable::get(IS_MOBILE_VAR);
if !is_mobile {
    file::delete("mobile");
}

let is_fullstack = variable::get(IS_FULLSTACK_VAR);
if !is_fullstack {
    file::delete("server");
    file::delete("ui/src/calculator.rs");
    file::delete("ui/assets/styling/calculator.css");
}

let is_router = variable::get(IS_ROUTER_VAR);
if !is_router {
    file::delete("web/src/views");
    file::delete("web/assets/blog.css");

    file::delete("desktop/src/views");
    file::delete("desktop/assets/blog.css");

    file::delete("mobile/src/views");
    file::delete("mobile/assets/blog.css");

    file::delete("ui/src/navbar.rs");
    file::delete("ui/assets/styling/navbar.css");
    file::delete("ui/assets/styling/blog.css");
}