use fcc_reporting::retrieve_subscriptions;

fn main() {
    let subscriptions = retrieve_subscriptions();
    println!("{subscriptions:#?}");
}
