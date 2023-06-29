use fcc_reporting::emerald::retrieve_subscriptions;

fn main() {
    let subscriptions = retrieve_subscriptions();
    println!("{subscriptions:#?}");
}
