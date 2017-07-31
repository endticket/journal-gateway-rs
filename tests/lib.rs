extern crate journal_gateway;

#[test]
fn get_entries() {

    // TODO make this into a self contained test

    let journal_gw = JournalGateway::new("http://192.168.33.19:19531")
        .expect("JournalGateway initialization failed");
    //let res = journal_gw.get_all_entries();

    let filter = vec![("SYSLOG_IDENTIFIER".to_string(), "wash-manager".to_string())];

    let res: Vec<JournalEntry> = journal_gw
        .get_entries(Some(&filter), None)
        .expect("should  have some entries");

    // TODO use asserts to check stuff

    println!("Received {} entries", res.len());
    for entry in res {
        println!("{}: {:?}",
                 entry.syslog_identifier.unwrap_or("N/A".to_string()),
                 entry.message);
    }
    println!("First entry: {:?}\n",
             journal_gw.get_first_entry(Some(&filter)));
    println!("Last entry: {:?}\n",
             journal_gw.get_last_entry(Some(&filter)));
}

// TODO create another test with a filter, e.g.
// let res = journal_gw.get_entries(Some(vec![("_COMM".to_string(), "cat".to_string())]));
