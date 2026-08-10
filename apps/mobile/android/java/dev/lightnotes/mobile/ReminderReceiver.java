package dev.lightnotes.mobile;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.os.Build;

public class ReminderReceiver extends BroadcastReceiver {
  public static final String CHANNEL_ID = "lightnotes-reminders";
  public static final String EXTRA_ID = "lightnotes.id";
  public static final String EXTRA_TITLE = "lightnotes.title";
  public static final String EXTRA_BODY = "lightnotes.body";

  @Override
  public void onReceive(Context context, Intent intent) {
    NotificationManager manager =
        (NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE);
    if (manager == null) {
      return;
    }

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      NotificationChannel channel =
          new NotificationChannel(CHANNEL_ID, "Reminders", NotificationManager.IMPORTANCE_DEFAULT);
      manager.createNotificationChannel(channel);
    }

    String title = intent.getStringExtra(EXTRA_TITLE);
    String body = intent.getStringExtra(EXTRA_BODY);
    String id = intent.getStringExtra(EXTRA_ID);

    int icon = context.getApplicationInfo().icon;
    if (icon == 0) {
      icon = android.R.drawable.ic_dialog_info;
    }

    Notification.Builder builder =
        Build.VERSION.SDK_INT >= Build.VERSION_CODES.O
            ? new Notification.Builder(context, CHANNEL_ID)
            : new Notification.Builder(context);

    Notification notification =
        builder
            .setContentTitle(title == null ? "" : title)
            .setContentText(body == null ? "" : body)
            .setSmallIcon(icon)
            .setAutoCancel(true)
            .build();

    manager.notify(id == null ? 0 : id.hashCode(), notification);
  }
}
