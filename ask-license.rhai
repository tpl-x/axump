let license = variable::get("license").to_lower();
while switch license {
    "none" => {
        debug("you selected to not use any license");
        file::delete("{{project-name}}\LICENSE-APACHE");
        file::delete("{{project-name}}\LICENSE-MIT");
        false
    }
    "mit" => {
        debug("you selected to use the MIT license");
        file::delete("{{project-name}}\LICENSE-APACHE");
        file::rename("{{project-name}}\LICENSE-MIT", "{{project-name}}\LICENSE");
        false
    }
    "apache" => {
        debug("you selected to use the APACHE license");
        file::rename("{{project-name}}\LICENSE-APACHE", "{{project-name}}\LICENSE");
        file::delete("{{project-name}}\LICENSE-MIT");
        false
    }
    _ => true,
} {
    license = variable::prompt("Which license do you want to use?", "none", ["none", "apache", "mit"]);
}
variable::set("license", license);