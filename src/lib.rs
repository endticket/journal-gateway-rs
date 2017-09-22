extern crate hyper;
extern crate url;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod error;

use std::io::Read;
use std::fmt;

use hyper::status::StatusCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum JournalMessage {
    String(String),
    Blob(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub message: Option<JournalMessage>,
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
    pub target: Option<String>,
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
    baseurl: url::Url,
    client: hyper::Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub skip: Option<i32>,
    pub length: Option<u32>,
}

impl fmt::Display for PaginationParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_data = String::new();
        if let Some(ref cursor_str) = self.cursor {
            str_data = str_data + cursor_str;
        }
        if let Some(ref skip_int) = self.skip {
            str_data = format!("{}:{}", str_data, skip_int);
        }
        if let Some(ref length_int) = self.length {
            str_data = format!("{}:{}", str_data, length_int);
        }
        write!(f, "{}", str_data)
    }
}

impl JournalGateway {
    pub fn new(baseurl_str: &str) -> error::Result<JournalGateway> {
        let baseurl = try!(url::Url::parse(baseurl_str));
        Ok(JournalGateway {
               baseurl: baseurl.to_owned(),
               client: hyper::Client::new(),
           })
    }

    pub fn get_all_entries(&self) -> error::Result<Vec<JournalEntry>> {
        self.get_entries(None, None)
    }

    pub fn get_first_entry(&self,
                           filters: Option<&Vec<(String, String)>>)
                           -> error::Result<JournalEntry> {
        let pagi = PaginationParams {
            cursor: None,
            skip: None,
            length: Some(1),
        };
        let list = try!(self.get_entries(filters, Some(pagi)));
        match list.is_empty() {
            true => Err(error::ErrorKind::NoEntries.into()),
            false => Ok(list[0].clone()),
        }
    }

    pub fn get_last_entry(&self,
                          filters: Option<&Vec<(String, String)>>)
                          -> error::Result<JournalEntry> {
        let pagi = PaginationParams {
            cursor: None,
            skip: Some(-1),
            length: Some(2),
        };
        let list = try!(self.get_entries(filters, Some(pagi)));
        match list.len() {
            0 => Err(error::ErrorKind::NoEntries.into()),
            1 => Ok(list[0].clone()),
            2 => Ok(list[1].clone()),
            len => panic!("Requested 2 elements, but got more: {}", len),
        }
    }

    pub fn get_entries(&self,
                       filters: Option<&Vec<(String, String)>>,
                       pagination: Option<PaginationParams>)
                       -> error::Result<Vec<JournalEntry>> {

        let mut url = try!(self.baseurl.join("entries"));
        if let Some(filters_unwrapped) = filters {
            if !filters_unwrapped.is_empty() {
                url.query_pairs_mut().extend_pairs(filters_unwrapped);
            }
        }

        let mut request = self.client
            .request(hyper::method::Method::Get, url.as_str())
            .header(hyper::header::Accept::json());

        if let Some(pagi) = pagination {
            request =
                request.header(hyper::header::Range::Unregistered("entries".to_string(),
                                                                  pagi.to_string()));
        }

        let mut response = try!(request.send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));

        if response.status != StatusCode::Ok {
            return Err(error::ErrorKind::RequestError(response.status, body).into());
        }

        let mut res: Vec<JournalEntry> = vec![];
        for line in body.split("\n") {
            if line.len() > 0 {
                let entry = try!(serde_json::from_str(&line));
                res.push(entry);
            }
        }

        Ok(res)
    }
}
