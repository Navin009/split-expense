#[catch(404)]
fn general_not_found() -> &'static str {
    "General 404"
}

#[catch(401)]
fn foo_not_found() -> &'static str {
    "Foo 404"
}
