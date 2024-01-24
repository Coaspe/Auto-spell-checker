import Cocoa
import SwiftUI
import UserNotifications

@main
struct AutoSpellCheckerApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        Window("Activity", id: "Activity") {
            GreetingView()
        }

        MenuBarExtra("Auto Spell Checker", systemImage: "list.bullet.clipboard") {
            MenuView()
        }
        .menuBarExtraStyle(.window)
    }
}

class AppDelegate: NSObject, NSApplicationDelegate {
    var keyMonitor: Any?

    func applicationDidFinishLaunching(_ notification: Notification) {
        _ = AXIsProcessTrustedWithOptions(
            [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true] as CFDictionary)
        getPassportKey { passportKey, error in
            if let error = error {
                sendNotification(flag: false, body: error.localizedDescription)
            }
            if let passportKey = passportKey {
                startMonitoringKeys(passportKey: passportKey)
            }
        }

        func startMonitoringKeys(passportKey: String) {
            keyMonitor = NSEvent.addGlobalMonitorForEvents(matching: .keyDown) { (event: NSEvent) in
                if event.modifierFlags.contains(.control) && event.characters == "\u{03}" {
                    getCheckedString(passportKey: passportKey)
                }
            }
        }

        func stopMonitoringKeys() {
            if let keyMonitor = keyMonitor {
                NSEvent.removeMonitor(keyMonitor)
                self.keyMonitor = nil
            }
        }

        func applicationWillTerminate(_ notification: Notification) {
            stopMonitoringKeys()
        }
    }
}

func getCheckedString(passportKey: String) {
    var urlComponents = URLComponents(string: BASE_URL)!
    let text = getTextFromClipboard()

    urlComponents.queryItems = [
        URLQueryItem(name: PASSPORT, value: passportKey),
        URLQueryItem(name: COLOR_BLINDNESS, value: COLOR_BLINDNESS_VAL),
        URLQueryItem(name: Q, value: text)
    ]

    let session = URLSession.shared
    var request = URLRequest(url: urlComponents.url!)

    request.addValue(USER_AGENT_VAL, forHTTPHeaderField: USER_AGENT)
    request.addValue(REFERER_VAL, forHTTPHeaderField: REFERER)

    request.httpMethod = GET

    let task = session.dataTask(with: request) { data, _, error in
        if let error = error {
            sendNotification(flag: false, body: error.localizedDescription)
        } else {
            let data = data!
            let decoder = JSONDecoder()
            do {
                let data = try decoder.decode(SpellChecker.self, from: data)
                let checked_string = data.message.result.notag_html
                copyTextToClipboard(text: checked_string)
                sendNotification(flag: true, body: checked_string)
            } catch {
                sendNotification(flag: false, body: error.localizedDescription)
            }
        }
    }

    task.resume()
}

func getPassportKey(completion: @escaping (String?, Error?) -> Void) {
    let urlComponents = URLComponents(string: PASSPORT_KEY_URL)!

    var request = URLRequest(url: urlComponents.url!)

    request.addValue(USER_AGENT_VAL, forHTTPHeaderField: USER_AGENT)
    request.httpMethod = GET

    URLSession.shared.dataTask(with: request) { data, _, error in

        if let error = error {
            completion(nil, error)
            return
        }

        guard let data = data else {
            completion(nil, NSError(domain: "YourDomain", code: 0, userInfo: [NSLocalizedDescriptionKey: "No data received"]))
            return
        }

        // Find passport key using (?i)passportKey=([^"'\s]+)
        do {
            let html = String(data: data, encoding: .utf8)!
            let regex = try NSRegularExpression(pattern: "(?i)passportKey=([^\"\'\\s]+)")
            let match = regex.firstMatch(in: html, range: NSRange(location: 0, length: html.count))

            if match == nil {
                completion(nil, NSError(domain: "YourDomain", code: 0, userInfo: [NSLocalizedDescriptionKey: "Passport key not found"]))
            } else {
                let passportKeyString = (String(data: data, encoding: .utf8)! as NSString).substring(with: match!.range(at: 1))
                completion(passportKeyString, nil)
            }
        } catch {
            completion(nil, error)
        }

    }.resume()
}

func copyTextToClipboard(text: String) {
    let pasteboard = NSPasteboard.general

    pasteboard.clearContents()
    pasteboard.writeObjects([text as NSPasteboardWriting])
}

func getTextFromClipboard() -> String {
    let pasteboard = NSPasteboard.general
    let text = pasteboard.string(forType: .string)
    return text ?? ""
}

struct Result: Codable {
    let errata_count: Int
    let origin_html: String
    let html: String
    let notag_html: String
}

struct Message: Codable {
    let result: Result
}

struct SpellChecker: Codable {
    let message: Message
}
