use notify_rust::Notification;

/// Sends a notification to the user
pub fn notify(args: &crate::Args) 
{
    Notification::new()
        .summary(&args.notif_title)
        .show()
        .unwrap();
}