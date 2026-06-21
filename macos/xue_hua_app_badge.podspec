#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
#
Pod::Spec.new do |s|
  s.name             = 'xue_hua_app_badge'
  s.version          = '0.0.1'
  s.summary          = 'Cross-platform Flutter app badge plugin.'
  s.description      = <<-DESC
Native macOS dock badge support via MethodChannel.
                       DESC
  s.homepage         = 'https://github.com/Matkurban/xue_hua_app_badge'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Matkurban' => 'https://github.com/Matkurban' }
  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'FlutterMacOS'
  s.platform = :osx, '10.11'
  s.swift_version = '5.0'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
end
