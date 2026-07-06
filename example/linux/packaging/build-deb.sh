#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLE_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PACKAGING_DIR="${SCRIPT_DIR}"
DIST_DIR="${EXAMPLE_DIR}/dist"

if ! command -v nfpm >/dev/null 2>&1; then
  echo "error: nfpm is not installed." >&2
  echo "Install with: go install github.com/goreleaser/nfpm/v2/cmd/nfpm@v2.41.3" >&2
  echo "Then ensure \$HOME/go/bin is in your PATH." >&2
  exit 1
fi

if ! command -v flutter >/dev/null 2>&1; then
  echo "error: flutter is not installed or not in PATH." >&2
  exit 1
fi

cd "${EXAMPLE_DIR}"
flutter pub get
flutter build linux --release

BUNDLE_DIR="${EXAMPLE_DIR}/build/linux/x64/release/bundle"
if [[ ! -x "${BUNDLE_DIR}/xue_hua_app_badge_example" ]]; then
  echo "error: release bundle not found at ${BUNDLE_DIR}" >&2
  exit 1
fi

mkdir -p "${DIST_DIR}"
(
  cd "${PACKAGING_DIR}"
  nfpm pkg \
    -f nfpm.yaml \
    -p deb \
    -t "${DIST_DIR}/"
)

echo "Built: ${DIST_DIR}/xue-hua-app-badge-example_1.0.0_amd64.deb"
