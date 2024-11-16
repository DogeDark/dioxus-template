// These are variables that are used in liquid
const NEEDS_DEFAULT_PLATFORM_VAR = "needs_default_platform";
const NEEDS_TAILWIND_PROMPT_VAR = "needs_tailwind_prompt";

const DIOXUS_DEP_FEAT_VAR = "dioxus_dep_features";
const DEFAULT_PLATFORM_VAR = "default_platform";

const IS_WEB_VAR = "is_web";
const IS_DESKTOP_VAR = "is_desktop";
const IS_MOBILE_VAR = "is_mobile";

const IS_ROUTER_VAR = "is_router";
const IS_FULLSTACK_VAR = "is_fullstack";
const IS_TAILWIND_VAR = "is_tailwind";

const FEATURES_VAR = "features";

// Figure out what platforms
let features = [];
let platforms = [];

let is_web = variable::prompt("Do you want to build for Dioxus Web?", true);
if is_web {
    platforms += "Web";
    features += "web = [\"dioxus/web\"]";
    variable::set(IS_WEB_VAR, true);
}
let is_desktop = variable::prompt("Do you want to build for Dioxus Desktop?", true);
if is_desktop {
    platforms += "Desktop";
    features += "desktop = [\"dioxus/desktop\"]";
    variable::set(IS_DESKTOP_VAR, true);
}

// Mobile
let is_mobile = variable::prompt("Do you want to build for Dioxus Mobile?", true);
if is_mobile {
    // Mobile can't currently be a default platform.
    //platforms += "Mobile";
    features += "mobile = [\"dioxus/mobile\"]";
    variable::set(IS_MOBILE_VAR, true);
}

// Make sure the user chose at least one platform.
if platforms.len() == 0 {
    if is_mobile {
        variable::set(DEFAULT_PLATFORM_VAR, "mobile");
    } else {
        abort("You must choose at least one platform to build for.");
    }
} else {
    // Ask the default platform from the selected platforms if the sub template requests it.
    if variable::get(NEEDS_DEFAULT_PLATFORM_VAR) {
        let default_platform = variable::prompt("Which platform do you want DX to serve by default?", platforms[0], platforms);
        variable::set(DEFAULT_PLATFORM_VAR, default_platform.to_lower());
    }
}


// Use fullstack?
let is_fullstack = variable::prompt("Do you want to use Dioxus Fullstack?", true);
variable::set(IS_FULLSTACK_VAR, is_fullstack);
if is_fullstack {
    features += "server = [\"dioxus/axum\"]";
}

// Use the router?
let is_router = variable::prompt("Do you want to use the Dioxus Router?", true);
variable::set(IS_ROUTER_VAR, is_router);

switch [is_router, is_fullstack] {
    [true, true] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"router\", \"fullstack\"]"),
    [true, false] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"router\"]"),
    [false, true] => variable::set(DIOXUS_DEP_FEAT_VAR, "[\"fullstack\"]"),
    _ => variable::set(DIOXUS_DEP_FEAT_VAR, "[]")
}

// Set features
variable::set(FEATURES_VAR, features);

// Assets
if variable::get(NEEDS_TAILWIND_PROMPT_VAR) {
    let is_tailwind = variable::prompt("Do you want to use Tailwind CSS?", false);
    variable::set(IS_TAILWIND_VAR, is_tailwind);
}

