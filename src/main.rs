slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROPER: f64 = 0.05;
const OPPER: f64 = 0.10;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax = num * TAXPER;
        let owner = num * OWNERPER;
        let profit = num * PROPER;
        let operating = num * OPPER;
        let results = format!(
            "Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating: {:.2}",
            tax, owner, profit, operating
        );
        ui.set_results(results.into());
    });

    ui.run()
}
