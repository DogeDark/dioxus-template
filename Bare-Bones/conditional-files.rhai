// This file handles conditional files for this specific sub-template.

const IS_TAILWIND_VAR = "is_tailwind";

let is_tailwind = variable::get(IS_TAILWIND_VAR);
if !is_tailwind {
    file::delete("tailwind.config.js");
    file::delete("input.css");
}