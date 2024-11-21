// This file handles conditional files for this specific sub-template.

let is_tailwind = variable::get("is_tailwind");
if !is_tailwind {
    file::delete("tailwind.config.js");
    file::delete("input.css");
}