# Phase 1: Windows Kabuğu ve Platform Altyapısı - Context

**Gathered:** 2026-10-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Mornfold'un Windows 11 üzerindeki platform temeli: NSIS kurulum dosyası (Windows CI'da her push'ta üretilir), sistem tepsisinde yaşayan tek kopya uygulama, açılışta otomatik başlatma, Mornfold kimliğiyle tıklanabilir Windows bildirimleri, uykudan uyanınca kaçırılan işleri telafi eden kalıcı zamanlayıcı iskeleti, Windows Kimlik Bilgisi Yöneticisi'ne bağlı şifreli anahtar kasası, Türkçe i18n altyapısı ve imzalı uygulama içi güncelleme. Workspace yapısı (`crates/mornfold-core`, `src-tauri`, `ui/`) ve SQLite + migration altyapısı bu fazda kurulur.

Requirements: SHELL-01..SHELL-09. Tasarım sistemi / kart kütüphanesi (Faz 2), bağlayıcılar ve bildirim motoru kuralları (Faz 3) bu fazın dışında.

</domain>

<decisions>
## Implementation Decisions

### Dağıtım ve güncelleme
- **D-01:** Dağıtım kanalı **NSIS + GitHub Releases** (`tauri-plugin-updater`, imzalı `latest.json`). Microsoft Store (MSIX) v2'de yeniden değerlendirilir. Bu, roadmap'teki "dağıtım kanalı kararı yazılı olarak alınır" kriterini karşılar.
- **D-02:** Yeni sürüm bulununca **arka planda indir, sonra kullanıcıya "yeniden başlat" diye sor** — hem Windows bildirimi hem tepsi menüsünde bir öğe olarak. Sessiz kurulum yok.
- **D-03:** Sürümler **bu repoda (emsal66/tempo, herkese açık) GitHub Releases** olarak yayımlanır; ayrı sürüm reposu yok.
- **D-04:** **Her push'ta CI artifact** (NSIS kurulum dosyası) üretilir; **gerçek sürüm yalnızca `v0.x.y` etiketi** ile oluşturulur (tauri-action v1).
- **D-05:** Güncelleyici imza anahtarı (minisign) **şifre yöneticisi + çevrimdışı USB + GitHub Actions secret** olarak yedeklenir. Planlar anahtar üretimi ve yedekleme adımlarını kullanıcıya yaptırılacak manuel adımlar olarak içermeli (anahtar repoya asla girmez).

### Faz 1 penceresi ve bildirim
- **D-06:** Faz 1 sonunda pencere **sade iskelet**: Mica cam arka plan + özel başlık çubuğu (`decorations: false`, drag region) + Mornfold logosu + sistem durumu (son kontrol zamanı, sürüm, güncelleme durumu) + Ayarlar. Faz 2'de yerine gerçek pano gelir; bu ekran geçicidir, cila minimum ama "çirkin" olmamalı.
- **D-07:** **Elle başlatıldığında pencere açılır**; Windows açılışında otomatik başlatıldığında (`--minimized`) **sadece tepsiye iner**.
- **D-08:** Windows bildirimleri **butonlu**: tıklayınca ilgili ekran açılır + 1–2 hızlı buton (örn. "Aç", "Sessize al"). **Uygulama kapalıyken (cold start) de tıklama çalışmalı** → ToastActivatorCLSID / COM aktivasyon kaydı (NSIS) gerekir; araştırma bayrağı.
- **D-09:** **Özel Mornfold bildirim sesi.** Referans: **Discord'un bildirim sesinin karakteri** — kısa, yumuşak, tanınabilir bir "ping". Discord'un sesi birebir kopyalanmaz (telif); benzer hissi veren **özgün** bir ses dosyası üretilir veya uygun lisanslı bir ses kullanılır. Ses toast'a custom audio olarak bağlanır (installed app gerektirir).
- **D-10:** Ses **her bildirimde çalar**; Ayarlar'dan **tamamen veya servis bazında kapatılabilir**; **sessiz saatlerde çalmaz** (sessiz saatlerin kendisi Faz 3 bildirim motorunda; Faz 1'de ses aç/kapa ayarı ve altyapısı yeterli).

### Tepsi ve arka plan
- **D-11:** Tepsi ikonuna **sol tık pencereyi açar/gizler**; **sağ tık menü**: Aç / Tümünü duraklat / Çık (Discord gibi). Güncelleme hazırsa menüde "Yeniden başlat ve güncelle" öğesi (D-02).
- **D-12:** Tepsi ikonu **durum gösterir**: okunmamış bildirim sayısı için rozet; duraklatıldığında veya hata olduğunda ikon soluk/farklı görünür. Üzerine gelince ipucunda "son kontrol: X dk önce" (SHELL-06).
- **D-13:** "Tümünü duraklat" **süre seçmeli**: 1 saat / Yarına kadar / Ben açana kadar (Discord "rahatsız etme" menüsü gibi). Süre dolunca otomatik devam.
- **D-14:** Pencerenin X düğmesi **sessizce tepsiye indirir** — bilgi bildirimi yok. Tamamen kapatmak tepsi menüsündeki "Çık" ile.

### Anahtar kasası deneyimi
- **D-15:** Kayıtlı anahtar arayüzde **maskeli + son 4 hane** görünür (örn. `sk-ant-••••a1b2`); yanında Değiştir / Sil / Test et. Tam değer arayüze asla dönmez (SHELL-07) — maskeli gösterim çekirdekte hesaplanır.
- **D-16:** Değiştirme/silmede **ek doğrulama yok** (Windows Hello istenmez); Windows oturumu açıkken doğrudan yapılır.
- **D-17:** Anahtar girildiğinde **önce test, sonra kaydet**; geçersizse uyar. Test yalnızca sağlayıcısı bilinen anahtar türleri için; genel anahtarlarda test adımı atlanabilir.
- **D-18:** Faz 1'de Ayarlar'da **genel "Anahtarlar" bölümü**: ad + değer ekle / sil / test. Claude / Gemini / OpenRouter alanları Faz 5'te bu kasaya bağlanır. Kasa tasarımı araştırmadaki öneriyi izler: zarf şifreleme (Credential Manager'da ana anahtar + SQLite'ta AES-256-GCM şifreli değerler), `SecretStore` trait arkasında.

### Claude's Discretion
- Geçici iskelet ekranın tam yerleşimi ve tipografisi (D-06 sınırları içinde).
- Bildirim butonlarının tam metinleri ve hangi bildirimde hangi butonların çıkacağı (en fazla 2).
- Tepsi ikonunun rozet/soluk durum görselleri.
- Zamanlayıcı iskeletindeki test işinin ne olduğu (sağlık kontrolü/heartbeat gibi) — uyku telafisini kanıtlamaya yetmeli.
- AEAD crate seçimi ve kasa iç detayları (araştırma bayrağı).

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Proje kapsamı ve gereksinimler
- `.planning/PROJECT.md` — vizyon, Core Value, kısıtlar (yalnızca Windows 11, yerel veri)
- `.planning/REQUIREMENTS.md` — SHELL-01..SHELL-09 metinleri
- `.planning/ROADMAP.md` §Phase 1 — hedef ve 5 başarı kriteri

### Teknoloji ve mimari
- `.planning/research/STACK.md` — Tauri 2.12 + Rust core + React; plugin sürümleri; `tauri-winrt-notification` (tıklanabilir toast), `keyring-core`, `sqlx`, `croner`/`jiff`; Linux'tan Windows build/test planı; "What NOT to Use"
- `.planning/research/ARCHITECTURE.md` — çekirdek/kabuk/arayüz sınırları, secrets tasarımı (zarf şifreleme, 2560 bayt sınırı), kalıcı zamanlayıcı, bildirim kimliği (AUMID)
- `.planning/research/SUMMARY.md` — Electron→Tauri çeviri tablosu, Faz 1 kapsamı ve araştırma bayrakları (cold-start toast aktivasyonu, NSIS AUMID, AEAD crate, crash isolation)
- `.planning/research/PITFALLS.md` — Linux-geliştirme / Windows-gerçeklik farkı, scheduler'ın sessizce durması, imza/kanal kararı, updater anahtar yedeği

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Yok — greenfield. Repoda yalnızca `README.md`, GSD dosyaları (`.claude/`, `.planning/`) ve `CLAUDE.md` var.

### Established Patterns
- Henüz yok. Bu faz kalıpları belirler: Cargo workspace (`crates/mornfold-core` Tauri bağımlılığı olmadan, `src-tauri` ince kabuk + `#[cfg(windows)]` adaptörler, `ui/` React), `SecretStore` ve `Notifier` trait'leri, `tauri-specta` ile tipli IPC.

### Integration Points
- GitHub Actions `windows-2025` runner: build + test + NSIS bundle + artifact; etiketle release (tauri-action v1).
- `.claude/settings.json` SessionStart hook'u bulut oturumlarında `gsd-sdk` kurar; proje build araçları (Rust, pnpm, cargo-xwin) için ileride benzer bir kurulum gerekebilir.

</code_context>

<specifics>
## Specific Ideas

- Bildirim sesi: "Discord bildirim sesi gibi" — kısa, yumuşak, tanınabilir ping; özgün olmalı.
- Tepsi etkileşimi ve "rahatsız etme" süre menüsü: Discord'un tepsi davranışı referans.
- Tepside/Mica'da Apple / Linear zarafeti hedefi bu fazın geçici ekranında da hissedilmeli.
- Bulut ortamında doğrulanamayan davranışlar (Mica, toast + ses, tepsi, otomatik başlatma, uyku/uyanma, güncelleme uçtan uca) kullanıcının Windows 11 PC'sinde bir **smoke-test kontrol listesiyle** doğrulanır; plan bu listeyi üretmeli.

</specifics>

<deferred>
## Deferred Ideas

- Microsoft Store (MSIX) kanalı — v2'de yeniden değerlendirilecek (D-01).
- Sessiz saatler kuralının kendisi — Faz 3 bildirim motoru (Faz 1'de yalnızca ses aç/kapa).

</deferred>

---

*Phase: 01-windows-kabugu-ve-platform-altyapisi*
*Context gathered: 2026-10-18*
