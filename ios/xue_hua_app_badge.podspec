Pod::Spec.new do |s|
  s.name             = 'xue_hua_app_badge'
  s.version          = '2.0.0'
  s.summary          = 'Cross-platform Flutter app badge plugin.'
  s.description      = <<-DESC
Cross-platform Flutter app badge plugin supporting Android, iOS, macOS, Windows, and Linux via a unified set/remove/permission API.
                       DESC
  s.homepage         = 'https://github.com/Matkurban/xue_hua_app_badge'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Matkurban' => '3496354336@qq.com' }
  s.source           = { :path => '.' }
  s.source_files     = 'xue_hua_app_badge/Sources/xue_hua_app_badge/**/*'
  s.dependency 'Flutter'
  s.platform         = :ios, '12.0'

  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
end