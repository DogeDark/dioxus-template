// This file sets variables based on the results of the shared-init script.

const WORKSPACE_MEMBERS_VAR = "workspace_members";
const ROUTER_FEATURE_VAR = "router_feature";

const IS_WEB_VAR = "is_web";
const IS_DESKTOP_VAR = "is_desktop";
const IS_MOBILE_VAR = "is_mobile";
const IS_FULLSTACK = "is_fullstack";
const IS_ROUTER = "is_router";

let workspace_members = [];

if variable::get(IS_WEB_VAR) {
    workspace_members += "web"
}

if variable::get(IS_DESKTOP_VAR) {
    workspace_members += "desktop"
}

if variable::get(IS_WEB_VAR) {
    workspace_members += "mobile"
}

if variable::get(IS_FULLSTACK) {
    workspace_members += "server"
}

variable::set(WORKSPACE_MEMBERS_VAR, workspace_members);

// Router feature
if variable::get(IS_ROUTER) {
    variable::set(ROUTER_FEATURE_VAR, "\"router\"");
} else {
    variable::set(ROUTER_FEATURE_VAR, "");
}