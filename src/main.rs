use suart::cli::wizard::SerialWizard;

fn main() {
    let test_config = SerialWizard::run();

    println!("========================================");
    println!("     UI TEST: RESULT VALIDATION         ");
    println!("========================================");
    println!("Successfully captured user configuration context:");
    println!("{:#?}", test_config);
    println!("========================================");
}
