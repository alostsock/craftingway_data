#! /bin/bash

OUTDIR="output_processed"

rm -rf $OUTDIR
mkdir -p "${OUTDIR}/action"
mkdir -p "${OUTDIR}/status"

ACTION_ICONS=(output/icon/action/*)
for icon in "${ACTION_ICONS[@]}"
do
  FILE="${OUTDIR}/action/$(basename "${icon}")"
  rusty_sr "${icon}" "${FILE}"

  WEBP_FILE="${OUTDIR}/action/$(basename -- "${icon}" .png).webp"
  cwebp -q 80 "$FILE" -o "$WEBP_FILE"
done

STATUS_ICONS=(output/icon/status/*)
for icon in "${STATUS_ICONS[@]}"
do
  FILE="${OUTDIR}/status/$(basename "${icon}")"
  rusty_sr "${icon}" "${FILE}"

  WEBP_FILE="${OUTDIR}/status/$(basename -- "${icon}" .png).webp"
  cwebp -q 80 "$FILE" -o "$WEBP_FILE"
done