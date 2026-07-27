// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "xue_hua_app_badge",
    platforms: [
        .macOS(.v10_14)
    ],
    products: [
        .library(
            name: "xue-hua-app-badge",
            targets: ["xue_hua_app_badge"]
        )
    ],
    dependencies: [
        .package(name: "FlutterFramework", path: "../FlutterFramework")
    ],
    targets: [
        .target(
            name: "xue_hua_app_badge",
            dependencies: [
                .product(name: "FlutterFramework", package: "FlutterFramework")
            ],
            resources: []
        )
    ]
)
