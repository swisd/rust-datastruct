use std::path::is_separator;
use std::ptr::null_mut;
use std::time::Duration;
use windows::Win32::System::EventLog::*;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};
use windows::Win32::Foundation::{HANDLE, WAIT_OBJECT_0, WIN32_ERROR};
use windows::Win32::System::EventLog::{
    EvtSubscribe, EvtNext, EvtClose, EVT_HANDLE, EVT_SUBSCRIBE_FLAGS,
    EvtSubscribeToFutureEvents, EvtSubscribeStartAtOldestRecord
};
use windows::core::{w, PCWSTR, PWSTR};
use crate::splog::{init_db, connect_db, ledger, LedgerType};
use std::slice;

// Helper function to convert &str to PCWSTR
fn to_wchar(s: &str) -> PCWSTR {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0); // Null-terminate the string
    PCWSTR::from_raw(v.as_ptr())
}

// You will need a new helper function for the pull model
fn process_events(h_subscription: EVT_HANDLE, h_event: HANDLE) -> windows::core::Result<()> {
    loop {
        let wait_result = unsafe { WaitForSingleObject(h_event, 500) };

        if wait_result != WAIT_OBJECT_0 {
            continue;
        }

        unsafe {
            let mut event_handles: [EVT_HANDLE; 10] = [EVT_HANDLE(0); 10];
            let handles_slice: &mut [isize] = slice::from_raw_parts_mut(
                event_handles.as_mut_ptr() as *mut isize,
                event_handles.len(),
            );

            loop {
                let mut returned_count: u32 = 0;

                let result = EvtNext(h_subscription, handles_slice, 0, 0, &mut returned_count);

                if result.is_err() {
                    let err = result.err().unwrap();
                    if err.code() == windows::Win32::Foundation::ERROR_NO_MORE_ITEMS.into() {
                        break;
                    }
                    return Err(err); // Return the error directly
                }

                for i in 0..returned_count as usize {
                    let h_event_record = event_handles[i];
                    let xml = render_event(h_event_record)?; // Use `?` to propagate errors

                    if let Some((pid, msg, ledger_type)) = parse_event_data(&xml) {
                        if let Ok(conn) = crate::splog::connect_db("records.db") {
                            let _ = crate::splog::ledger(
                                conn,
                                pid,
                                0,
                                "system",
                                &format!("/system/event"),
                                ledger_type,
                                &msg
                            );
                        }
                    }
                    EvtClose(h_event_record);
                }
            }
        }
    }
}
pub fn evlog_main() {
    let channels_to_subscribe = ["System", "Application"];

    unsafe {
        let h_event = CreateEventW(None, true, false, None).unwrap();
        // let query = to_wchar("*[Application[Provider[@Name='MyApplication']]]");
        let query = to_wchar("*");

        for channel_name in channels_to_subscribe.iter() {
            let channel = to_wchar(channel_name);
            if *channel_name == "System" {
                let _channel = w!("System");
            } else if *channel_name == "Application" {
                let _channel = w!("Application");
            }

            let subscription = EvtSubscribe(
                None,                                // session
                h_event,                             // signalevent
                channel,                             // channelpath
                query,                               // query
                None,                                // bookmark
                None,                                // context
                None,                                // callback
                EvtSubscribeToFutureEvents.0,   // flags
            );

            if subscription.is_err() {
                eprintln!("Error subscribing to channel '{}': {:?}", channel_name, subscription.err());
            } else {
                println!("Successfully subscribed to channel '{}'.", channel_name);
                let h_subscription = subscription.unwrap();
                std::thread::spawn(move || {
                    if let Err(e) = process_events(h_subscription, h_event) {
                        eprintln!("Error processing events for channel: {:?}", e);
                    }
                    EvtClose(h_subscription);
                });
            }
        }

        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

// Your event callback function (implement this to process events)
unsafe extern "system" fn event_callback(
    _action: EVT_SUBSCRIBE_NOTIFY_ACTION,
    _user_context: *const std::ffi::c_void,
    event_handle: EVT_HANDLE,
) -> u32 {
    let xml = render_event(event_handle);
    if let Ok(xml_str) = xml {
        if let Some((pid, msg, event)) = parse_event_data(&xml_str) {
            if let Ok(conn) = connect_db("records.db") {
                if &msg != "No Message" && pid != 0 {
                    let _ = ledger(conn, pid, 0, "system", "/system/event", event, &msg);
                }
            }
        }
    }
    0
}

fn render_event(event_handle: EVT_HANDLE) -> Result<String, windows::core::Error> {
    unsafe {
        let mut buffer_used: u32 = 0;
        let mut property_count: u32 = 0;

        // Get required buffer size
        let result = EvtRender(
            None,
            event_handle,
            EvtRenderEventXml.0,
            0,
            None,
            &mut buffer_used,
            &mut property_count,
        );

        if result.is_ok() {
            // This is unexpected for non-empty events, but we can handle it.
            return Ok("".to_string());
        }

        let mut buffer_size: u32 = buffer_used;
        let err = windows::core::Error::from_win32();
        if err.code() != windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.into() {
            // If the error is not "insufficient buffer," it's a real error.
            return Err(err);
        }
        let mut buffer: Vec<u8> = vec![0; buffer_used as usize];

        let result = EvtRender(
            None,
            event_handle,
            EvtRenderEventXml.0,
            buffer_size,
            Some(buffer.as_mut_ptr() as *mut std::ffi::c_void),
            &mut buffer_size,
            &mut property_count,
        );

        if result.is_ok() {
            let pwstr = PWSTR::from_raw(buffer.as_ptr() as *mut u16);
            Ok(pwstr.to_string()?)
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}

fn parse_event_data(xml: &str) -> Option<(i16, String, LedgerType)> {
    // Basic PID and Message parsing (unchanged)
    let pid = xml.find("<Execution ProcessID=\"")
        .and_then(|i| xml[i..].find("\"").map(|j| i + j))
        .and_then(|i| xml[i..].chars().next())
        .and_then(|c| c.to_string().parse::<i16>().ok())
        .unwrap_or(0);

    let msg = xml.find("<Message>")
        .and_then(|i| Some(&xml[i + 9..]))
        .and_then(|s| s.find("</Message>").map(|j| &s[..j]))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "No message".to_string());

    // New logic to find the Level
    let event_type = xml.find("<Level>")
        .and_then(|i| Some(&xml[i + 7..]))
        .and_then(|s| s.find("</Level>").map(|j| &s[..j]))
        .and_then(|s| s.parse::<u32>().ok())
        .map(|level| {
            match level {
                1 => LedgerType::F,
                2 => LedgerType::E, // Level 1 is Critical, Level 2 is Error
                3 => LedgerType::W,
                4 => LedgerType::I,
                _ => LedgerType::V,
            }
        })
        .unwrap_or(LedgerType::V);

    Some((pid, msg, event_type))
}