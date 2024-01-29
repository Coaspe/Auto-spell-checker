//
//  GreetingView.swift
//  AutoSpellChecker
//
//  Created by 이우람 on 1/24/24.
//

import SwiftUI

struct GreetingView: View {
    @Environment(\.openWindow) private var openWindow

    var body: some View {
        VStack(alignment: .leading) {
            Text("1. 원하는 텍스트를 클립 보드에 복사 (Command + C)")
            Text("2. Left Control + Left option + C를 순서대로 누르면 자동 맞춤법 검사가 진행됩니다.")
            Text("3. 자동 맞춤법 검사가 완료되면 클립보드에 자동으로 복사됩니다.")
            Text("4. 원하는 곳에 붙여넣기 하세요.")
            Text("")
            Text("해당 앱은 백그라운드로 실행됩니다.")
            Text("앱을 종료하거나 사용법을 다시 보고 싶다면 숨겨진 아이콘에서 우클릭하세요.")
            Text("문의 사항은 이우람에게 해주세요.")
        }.background {
            Image("background").opacity(0.2).aspectRatio(contentMode: .fit)
        }
    }
}

#Preview {
    GreetingView()
}
