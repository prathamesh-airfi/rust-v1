#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("1234"));
    let master_card = PaymentMethodType::DebitCard(String::from("1234"));
    let paypal = PaymentMethodType::PayPal(String::from("1234"), String::from("12345"));
    println!("{visa:#?}");
    println!("{master_card:#?}");
    println!("{paypal:#?}");
}
