// Functions
// - Generate Html
// - Get Data Directory (for Dynamic Implementation)

// GET /gen/<template_name>?t=[access_token]
// GET /gen/<template_name>?t=[access_token]

#[get("/gen")]
pub fn gen() -> &'static str {
    "Generate HTML!!!!!"
}
