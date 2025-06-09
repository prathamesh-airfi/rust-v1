#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(Credentials),
}

fn main() {
    let paypal_credentials = Credentials {
        username: String::from("pratham@gmail.com"),
        password: String::from("1234"),
    };

    let paypal_account = PaymentMethodType::PayPal(paypal_credentials);
    println!("{:#?}", paypal_account);
}
