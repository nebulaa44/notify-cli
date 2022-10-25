use notify_rust::Notification;

pub fn notify(args: &crate::Args) 
{
    Notification::new()
        .summary(&args.notif_title)
        .show()
        .unwrap();
}