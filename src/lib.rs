extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
//#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct JournalEntry {
    #[serde(rename = "__CURSOR")]
    pub cursor: String,
    #[serde(rename = "__REALTIME_TIMESTAMP")]
    pub realtime_timestamp: String,
    #[serde(rename = "__MONOTONIC_TIMESTAMP")]
    pub monotonic_timestamp: String,
    #[serde(rename = "_BOOT_ID")]
    pub boot_id: String,
    #[serde(rename = "_MACHINE_ID")]
    pub machine_id: String,
    #[serde(rename = "_HOSTNAME")]
    pub hostname: String,
    pub priority: Option<String>,
    #[serde(rename = "_UID")]
    pub uid: Option<String>,
    #[serde(rename = "_GID")]
    pub gid: Option<String>,
    #[serde(rename = "_SYSTEMD_SLICE")]
    pub systemd_slice: Option<String>,
    #[serde(rename = "_CAP_EFFECTIVE")]
    pub cap_effective: Option<String>,
    #[serde(rename = "_TRANSPORT")]
    pub transport: String,
    #[serde(rename = "_COMM")]
    pub command: Option<String>,
    #[serde(rename = "_EXE")]
    pub executable: Option<String>,
    #[serde(rename = "_SYSTEMD_CGROUP")]
    pub systemd_cgroup: Option<String>,
    #[serde(rename = "_SYSTEMD_UNIT")]
    pub systemd_unit: Option<String>,
    #[serde(rename = "_SYSTEMD_INVOCATION_ID")]
    pub systemd_invocation_id: Option<String>,
    #[serde(rename = "_SYSTEMD_OWNER_UID")]
    pub systemd_owner_uid: Option<String>,
    #[serde(rename = "_SYSTEMD_USER_SLICE")]
    pub systemd_user_slice: Option<String>,
    #[serde(rename = "_SYSTEMD_USER_UNIT")]
    pub systemd_user_unit: Option<String>,
    #[serde(rename = "_SYSTEMD_SESSION")]
    pub systemd_session: Option<String>,
    pub syslog_facility: Option<String>,
    pub syslog_identifier: Option<String>,
    #[serde(rename = "_CMDLINE")]
    pub command_line: Option<String>,
    #[serde(rename = "_AUDIT_LOGINUID")]
    pub audit_login_uid: Option<String>,
    pub message: String,
    pub code_file: Option<String>,
    pub code_line: Option<String>,
    pub code_function: Option<String>,
    pub unit: Option<String>,
    pub user_unit: Option<String>,
    pub userspace_usec: Option<String>,
    pub kernel_usec: Option<String>,
    pub message_id: Option<String>,
    pub result: Option<String>,
    pub user_id: Option<String>,
    pub seat_id: Option<String>,
    pub leader: Option<String>,
    pub journal_name: Option<String>,
    pub journal_path: Option<String>,
    pub current_use: Option<String>,
    pub current_use_pretty: Option<String>,
    pub max_use: Option<String>,
    pub max_use_pretty: Option<String>,
    pub disk_keep_free: Option<String>,
    pub disk_keep_free_pretty: Option<String>,
    pub disk_available: Option<String>,
    pub disk_available_pretty: Option<String>,
    pub available: Option<String>,
    pub available_pretty: Option<String>,
    pub limit: Option<String>,
    pub limit_pretty: Option<String>,
    pub session_id: Option<String>,
    pub syslog_pid: Option<String>,
    #[serde(rename = "_PID")]
    pub pid: Option<String>,
    #[serde(rename = "_AUDIT_SESSION")]
    pub audit_session: Option<String>,
    #[serde(rename = "_AUDIT_TYPE")]
    pub audit_type: Option<String>,
    #[serde(rename = "_AUDIT_FIELD_NAME")]
    pub audit_field_name: Option<String>,
    #[serde(rename = "_AUDIT_FIELD_APPARMOR")]
    pub audit_field_apparmor: Option<String>,
    #[serde(rename = "_AUDIT_FIELD_OPERATION")]
    pub audit_field_operation: Option<String>,
    #[serde(rename = "_AUDIT_FIELD_PROFILE")]
    pub audit_field_profile: Option<String>,
    #[serde(rename = "_AUDIT_ID")]
    pub audit_id: Option<String>,
    #[serde(rename = "_KERNEL_SUBSYSTEM")]
    pub kernel_subsystem: Option<String>,
    #[serde(rename = "_KERNEL_DEVICE")]
    pub kernel_device: Option<String>,
    #[serde(rename = "_UDEV_SYSNAME")]
    pub udev_sysname: Option<String>,
    #[serde(rename = "_UDEV_DEVNODE")]
    pub udev_devnode: Option<String>,
    #[serde(rename = "_UDEV_DEVLINK")]
    pub udev_devlink: Option<String>,
    #[serde(rename = "_SOURCE_REALTIME_TIMESTAMP")]
    pub source_realtime_timestamp: Option<String>,
    #[serde(rename = "_SOURCE_MONOTONIC_TIMESTAMP")]
    pub source_monotonic_timestamp: Option<String>,
}

pub struct JournalGateway {
    pub baseurl: url::Url,
    client: hyper::Client,
}

impl JournalGateway {
    pub fn new(baseurl_str: &str) -> JournalGateway {
        let baseurl = url::Url::parse(baseurl_str).expect("Error during parsing baseurl");
        JournalGateway {
            baseurl: baseurl.to_owned(),
            client: hyper::Client::new(),
        }
    }

    pub fn get_all_entries(&self) -> Vec<JournalEntry> {
        self.get_entries(None)
    }

    pub fn get_entries(&self, filters: Option<Vec<(String, String)>>) -> Vec<JournalEntry> {
        let mut url = self.baseurl.join("entries").expect("url join failed");
        if let Some(filters_unwrapped) = filters {
            url.query_pairs_mut().extend_pairs(filters_unwrapped);
        }

        let request = self.client
            .request(hyper::method::Method::Get, url.as_str())
            .header(hyper::header::Accept::json());

        let mut response = request.send().expect("request failure");
        let mut body = String::new();
        response
            .read_to_string(&mut body)
            .expect("body read failed");

        let mut res: Vec<JournalEntry> = vec![];
        for line in body.split("\n") {
            if line.len() > 0 {
                let deser_res = serde_json::from_str(&line);
                match deser_res {
                    Ok(entry) => res.push(entry),
                    Err(e) => println!("Error {}, skipping line: {}", e, line),
                }
            }
        }

        res
    }
}



#[cfg(test)]
mod tests {
    use JournalGateway;

    #[test]
    fn get_entries() {

        // TODO make this into a self contained test

        let journal_gw = JournalGateway::new("http://192.168.33.19:19531");
        let res = journal_gw.get_all_entries();
        for entry in res {
            println!("{}", entry.message);
        }
    }

    // TODO create another test with a filter, e.g.
    // let res = journal_gw.get_entries(Some(vec![("_COMM".to_string(), "cat".to_string())]));
}
