import Cocoa
import UserNotifications

class MyNotificationDelegate: NSObject, UNUserNotificationCenterDelegate {
    // 알람이 클릭될 때 호출되는 메서드
    func userNotificationCenter(_ center: UNUserNotificationCenter, didReceive response: UNNotificationResponse, withCompletionHandler completionHandler: @escaping () -> Void) {
        completionHandler()
    }
}

func sendNotification(flag: Bool, body: String) {
    // 알람 생성
    let content = UNMutableNotificationContent()
    content.title = APP_NAME
    content.subtitle = flag ? SUBTITLE_S : SUBTITLE_F
    content.body = body
    content.sound = UNNotificationSound.default

    // 알람 트리거 설정 (즉시 보여주기)
    let trigger = UNTimeIntervalNotificationTrigger(timeInterval: 1, repeats: false)

    // 알람 요청 생성
    let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: trigger)

    // 알람 센터 설정
    let notificationCenter = UNUserNotificationCenter.current()
    let delegate = MyNotificationDelegate()
    notificationCenter.delegate = delegate

    notificationCenter.requestAuthorization(options: [.alert, .badge, .sound]) { _, error in
        if let error = error {
            print("Error \(error.localizedDescription)")
        }
    }

    // 알람 센터에 알람 추가
    notificationCenter.add(request) { error in
        if let error = error {
            print("Error adding notification: \(error.localizedDescription)")
        }
    }
}
