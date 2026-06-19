use handlebars::Handlebars;

pub struct Template;
impl Template {
    pub fn engine() -> Handlebars<'static> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars
    }
}
