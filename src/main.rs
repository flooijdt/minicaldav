use std::io;

fn main() {
    let mut url = String::new();
    let mut username = String::new();

    println!("Enter the url for your caldav account: ");

    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read input");

    println!("Enter the username for your caldav account: ");

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");

    println!("Enter the password for your caldav account: ");

    // TODO have yet to confirm if this way of handling passwords is secure
    let password: String = rpassword::read_password().unwrap();
    println!("1");

    let agent = ureq::Agent::new();
    println!("2");
    let url = url::Url::parse(url.as_str()).expect("could not parse the given url.");
    println!("3");

    let credentials = minicaldav::Credentials::Basic(
        String::from(username.as_str().trim()),
        String::from(password.as_str().trim()),
    );
    println!("4");

    let calendars = minicaldav::get_calendars(agent.clone(), &credentials, &url).unwrap();
    println!("5");

    for calendar in calendars {
        println!("{:#?}", calendar);
    }
    println!("6");
}
