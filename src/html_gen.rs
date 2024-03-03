use askama::Template;
use url::Url;

#[derive(Template)]
#[allow(dead_code)]
// escape = "none": override the template's extension used
// for the purpose of determining the escaper for this template.
// {{ "Escape <>&"|e }} with escape will be this: Escape &lt;&gt;&amp;
// So we disable this
#[template(path = "init_payment_page.html", escape = "none")]
pub struct SubmitPaymentPage {
    price: i64,
    submit_payment_url: Url,
}

impl SubmitPaymentPage {
    pub fn new(price: i64, submit_payment_url: Url) -> Self {
        SubmitPaymentPage {
            price,
            submit_payment_url,
        }
    }
}

#[derive(Template)]
#[allow(dead_code)]
#[template(path = "card_token_registration_page.html", escape = "none")]
pub struct SubmitCardNumberPage {
    store_name: &'static str,
    submit_card_number_url: Url,
}

impl SubmitCardNumberPage {
    pub fn new(submit_card_number_url: Url) -> Self {
        SubmitCardNumberPage {
            submit_card_number_url,
            store_name: "Harmonysphere",
        }
    }
}

#[cfg(test)]
mod tests {
    use askama::Template;

    use super::SubmitPaymentPage;

    #[test]
    fn test_template_creation() {
        let page =
            SubmitPaymentPage::new(10, "http://mydomain/path".parse().unwrap());
        assert!(page.render().is_ok())
    }
}
