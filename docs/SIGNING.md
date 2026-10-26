# Firma y notarización de Apple — guía para el día del alta

> Decisión SD-F-017 (2026-10-26): cuenta **individual** de Apple Developer, contratada en **enero**
> (alta antes del 2027-01-05, por los días que tarda Apple en activarla), para publicar la primera
> versión firmada y notarizada antes del lanzamiento del 26-ene. Hasta entonces, cero gasto.
> Este repo es público: aquí no van nombres, IDs de equipo ni claves; los valores viven en `~/.secrets`.

## Qué está listo (sin cuenta)

- `scripts/macos-sign.sh` — firma el motor yt-dlp (103 librerías + ejecutable; Tauri no lo hace porque va en
  `Resources/`), firma + notariza + grapa los DMG, verifica (`verify`) y comprueba credenciales (`preflight`).
- `scripts/make-release.sh` — si existe `APPLE_SIGNING_IDENTITY`, hace la release firmada: compila fuera de
  iCloud (`~/Library/Caches/super-downloads/target`), firma **una copia** del motor y la empaqueta, y Tauri firma
  la app, ffmpeg y ffprobe con *hardened runtime*, notariza la `.app` y la grapa (el `.app.tar.gz` del updater
  sale después, así que también va notarizado). Sin esa variable, todo sigue como hoy (sin firma).
- `src-tauri/entitlements/engine.plist` — la única excepción del *hardened runtime*: memoria ejecutable para el
  Python del motor (ctypes/cffi de `curl_cffi`). La app no lleva ninguna.
- **Ensayo sin Apple:** `load-secrets && ./scripts/make-release.sh --sign-dry-run` (solo Apple Silicon, firma
  ad-hoc + *hardened runtime*, no toca `dist/`). Ensayo del 2026-10-26: bundle válido; el motor firmado descarga
  YouTube, TikTok (impersonación) y Vimeo restringido (plugin + ffmpeg).

Dos trampas encontradas en el ensayo, ya resueltas en los scripts:

1. **iCloud/Dropbox ensucian el bundle.** `codesign` rechaza la app («resource fork, Finder information, or
   similar detritus not allowed») por los atributos extendidos que iCloud pone a todo lo que se crea en este
   árbol y los de Dropbox en `ffmpeg`/`ffprobe`/icono → se compila fuera de iCloud y se hace `xattr -cr`.
2. **`Python.framework` del motor.** El zip oficial de yt-dlp trae tres copias reales del dylib de Python (sin
   enlaces simbólicos) y `codesign` lo rechaza como «bundle ambiguo». El motor carga `_internal/Python` y
   funciona sin el framework → `nested` lo quita de la copia firmada.

## El día del alta (Mario, ~30 min + la espera de Apple)

1. **Alta:** developer.apple.com/programs/enroll → *Individual* → tu Apple ID con doble factor → 99 USD/año.
   Apple verifica la identidad (normalmente 1–2 días). El nombre legal sale en el diálogo de instalación;
   en el resto de la marca, nada cambia.
2. **Certificado Developer ID Application:** Xcode → Settings → Accounts → (tu cuenta) → Manage Certificates →
   **+** → *Developer ID Application*. Queda en el llavero de sesión. Comprueba:
   `security find-identity -v -p codesigning` → una línea `"Developer ID Application: … (TEAMID)"`.
   Exporta una copia de seguridad (.p12 con contraseña) a tu gestor de contraseñas: si se pierde el Mac, se pierde.
3. **Clave para notarizar (API de App Store Connect):** appstoreconnect.apple.com → Users and Access →
   Integrations → App Store Connect API → *Team Keys* → **+**, acceso *Developer* → descarga `AuthKey_XXXX.p8`
   (solo se puede bajar una vez) → `mkdir -p ~/.appstoreconnect/private_keys && mv ~/Downloads/AuthKey_*.p8 ~/.appstoreconnect/private_keys/ && chmod 600 ~/.appstoreconnect/private_keys/*`.
   Apunta el *Key ID* y el *Issuer ID* de esa página.
4. **Añade a `~/.secrets`** (valores reales, nunca en el repo):
   ```bash
   export APPLE_SIGNING_IDENTITY="Developer ID Application: <nombre legal> (<TEAMID>)"
   export APPLE_API_ISSUER="<issuer id>"
   export APPLE_API_KEY="<key id>"
   export APPLE_API_KEY_PATH="$HOME/.appstoreconnect/private_keys/AuthKey_<key id>.p8"
   ```
5. **Comprobación:** `load-secrets && ./scripts/macos-sign.sh preflight` → `preflight OK`.

## La primera release firmada (Claude, con Mario delante)

1. Versión nueva (v1.6.0) como siempre (`docs/OPS.md` § Release pipeline, pasos 1–3).
2. `load-secrets && ./scripts/make-release.sh` — compila, firma, notariza (unos minutos por arquitectura) y
   verifica con `spctl`. Si Apple rechaza: `xcrun notarytool log <submission-id> --key … --key-id … --issuer …`
   dice qué fichero falla.
3. `load-secrets && ./scripts/make-release.sh --no-build --publish` — **con `load-secrets` también aquí**: sin él,
   el script no sabe que la build firmada está fuera de iCloud y empaquetaría la de `src-tauri/target`.
4. Después: bajar el DMG publicado en una carpeta nueva y comprobar
   `spctl --assess --type open --context context:primary-signature -v Super-Downloads_aarch64.dmg` y, montado,
   `spctl --assess --type execute -v "/Volumes/…/Super Downloads.app"` → `source=Notarized Developer ID`.
5. **Web, el mismo día y no antes:** quitar la guía «Open Anyway» (`web/src/pages/index.astro`, sección
   `#install` + FAQ `faq-macos`), quitar el *known issue* de `web/src/data/releases.js`, actualizar
   `docs/support/MACROS.md` («Opening Super Downloads for the first time») y la línea de `DESIGN.md`. Hasta
   que exista una build notarizada publicada, la web no dice «signed» ni «notarized».

## Notas

- **Updater:** la clave Ed25519 del updater no cambia; una 1.5.x sin firmar se actualiza a la firmada desde el
  aviso de siempre.
- **Motor que se actualiza solo:** la copia semanal que baja la app se ejecuta como proceso aparte; el
  *hardened runtime* de la app no la afecta.
- **¿Por qué no en GitHub Actions?** Los binarios empaquetados (~300 MB: motor, ffmpeg, ffprobe) no están en git
  y la clave del updater vive solo en este Mac; pasar la build a CI obligaría a descargarlos en cada ejecución y
  a subir la clave del updater y el certificado (.p12) como secretos de GitHub. Si algún día compensa, son los
  mismos pasos y las mismas variables (`APPLE_*` + `TAURI_SIGNING_PRIVATE_KEY*` como *repository secrets*, y un
  paso que importe el .p12 en un llavero temporal del runner).
