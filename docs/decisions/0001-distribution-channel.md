# ADR 0001 — Dağıtım kanalı: NSIS + GitHub Releases

## Durum

Kabul edildi — 2026-10-18

İlgili kararlar: D-01, D-02, D-03, D-04, D-05 (`.planning/phases/01-windows-kabu-u-ve-platform-altyap-s/01-CONTEXT.md`).
Bu kayıt, ROADMAP Faz 1 başarı kriteri 5'i ("dağıtım kanalı kararı yazılı olarak alınır") karşılar.

## Bağlam

Mornfold yalnızca Windows 11 masaüstünde çalışan, verilerini ve anahtarlarını yalnızca yerelde tutan
bir tepsi uygulamasıdır. İlk kullanıcı projenin sahibidir; ileride başkaları da indirip kendi
PC'lerinde kullanacaktır. Uygulamanın:

- yönetici izni istemeden kurulabilmesi,
- kendini güvenli (imzalı) biçimde güncelleyebilmesi,
- Windows açılışında tepsiye inerek başlayabilmesi,
- geliştirme ortamı Linux olduğu için kurulum paketinin CI'da (Windows runner) üretilmesi

gerekir. Seçenekler: NSIS kurulum dosyası + GitHub Releases + `tauri-plugin-updater`; MSI (WiX);
Microsoft Store (MSIX).

## Karar

### D-01 — Kanal

- Dağıtım kanalı **NSIS kurulum dosyası** + **GitHub Releases** + **`tauri-plugin-updater`** ile
  imzalı `latest.json`.
- NSIS ayarları: `installMode: currentUser` (kullanıcı başına kurulum, yönetici izni yok),
  kurulum dili Türkçe.
- **Microsoft Store (MSIX)** v2'de yeniden değerlendirilir. Store'a geçilirse Store'un kendi
  güncelleme mekanizması `tauri-plugin-updater`'ın, Store başlangıç görevi ise Run-key tabanlı
  otomatik başlatmanın yerini alır.
- MSI (WiX) yalnızca kurumsal/GPO dağıtımı için anlamlıdır, yalnızca Windows'ta üretilebilir ve
  kullanıcı başına kurulumda NSIS kadar rahat değildir; seçilmedi.

### D-03 — Sürümlerin yayımlandığı yer

- Sürümler **bu repoda** yayımlanır: `github.com/Emsal66/Tempo` (herkese açık). Ayrı bir sürüm
  reposu yoktur.
- Güncelleyici uç noktası:
  `https://github.com/Emsal66/Tempo/releases/latest/download/latest.json`
- `releases/latest/download/...` yalnızca **en son yayımlanmış, ön sürüm (prerelease) olmayan**
  sürüme çözülür. Taslak (draft) sürümler bu adreste görünmez.
- Sürümler **taslak olarak** oluşturulur; sahip kurulumu kendi PC'sinde kontrol ettikten sonra
  elle yayımlar. Böylece yayımlanmamış bir sürüm kurulu kopyalara asla ulaşmaz.

### D-02 — Güncelleme deneyimi (bağlam için)

- Yeni sürüm bulunduğunda arka planda indirilir; ardından hem Windows bildirimi hem tepsi
  menüsündeki "Yeniden başlat ve güncelle" öğesiyle kullanıcıya sorulur. Sessiz kurulum yoktur.

### D-04 — CI ve sürüm üretimi

- **Her push**, `windows-2025` CI işinde bir NSIS artifact'ı üretir (`mornfold-nsis-<sha>`).
- **Gerçek sürüm** yalnızca `v0.x.y` biçimindeki bir **etiket** ile, `tauri-apps/tauri-action` v1
  üzerinden oluşturulur.
- Push derlemeleri imza anahtarını **asla görmez**. `createUpdaterArtifacts: true` içeren
  `tauri.release.conf.json` yalnızca etiket işinde (`--config tauri.release.conf.json`) verilir;
  push derlemesi güncelleyici artifact'ı ve imza üretmez.

### D-05 — Güncelleyici imza anahtarının saklanması

- minisign güncelleyici anahtar çifti **sahibin kendi PC'sinde** üretilir — bulut konteynerinde
  asla üretilmez ve repoya asla girmez (`.gitignore` içinde `*.key` vardır).
- Özel anahtar ve parolası **üç yerde** yedeklenir:
  1. şifre yöneticisi,
  2. çevrimdışı USB bellek (`mornfold.key` + `mornfold.key.pub`),
  3. GitHub Actions secret'ları: `TAURI_SIGNING_PRIVATE_KEY` ve
     `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
- Bu belge yalnızca yedeklerin **nerede** durduğunu anlatır; anahtarı veya parolayı asla içermez.
- **Anahtarın kaybı geri döndürülemez:** kurulu kopyalar imza doğrulamasını kapatamaz, dolayısıyla
  bir daha asla otomatik güncellenemez; kullanıcıların yeni sürümü elle kurması gerekir.
- **Anahtar rotasyonu:** eski anahtarla imzalanmış son bir sürüm yayımlanır; bu sürümün yapılandırması
  yeni açık anahtara (pubkey) güvenir. Sonraki sürümler yeni anahtarla imzalanır.

### Dondurulmuş tanımlayıcılar

Aşağıdaki değerler ilk kurulu derlemeden sonra **değiştirilemez**:

| Tanımlayıcı | Değer |
|---|---|
| `identifier` | `com.mornfold.desktop` |
| `productName` | `Mornfold` |
| URL şeması | `mornfold` (`mornfold://`) |
| Kimlik Bilgisi Yöneticisi hizmeti / kullanıcısı | `Mornfold` / `vault-master-key-v1` |

Herhangi birinin değişmesi AUMID'i (bildirim kimliği), veri klasörünü
(`%LOCALAPPDATA%\com.mornfold.desktop`), anahtar kasasını ve güncelleyici güvenini bozar.

### İmzasız ikili dosyalar

- Yalnızca sahibin kullandığı v1 için imzasız ikili dosyalar kabul edilir: SmartScreen uyarısında
  "Ek bilgi → Yine de çalıştır".
- **Akıllı Uygulama Denetimi (Smart App Control) "Açık"** ise imzasız kod tamamen engellenir; bu
  durum smoke testindeki S-0 ile kontrol edilir. Açık çıkarsa kod imzalama engelleyici hale gelir.
- Kod imzalama v2 kapsamındadır (DIST-01): bulut HSM'li bir OV sertifikası (ör. Certum, SSL.com
  eSigner) `bundle.windows.signCommand` ile. Azure Artifact Signing, Türkiye'deki bir birey için
  kullanılamaz (bireysel uygunluk yalnızca ABD/Kanada).

### Sürüm numarasının tek kaynağı

- Sürümün tek doğruluk kaynağı `src-tauri/Cargo.toml` içindeki `package.version` alanıdır.
- `tauri.conf.json` içinde `version` alanı **yoktur**; Tauri sürümü Cargo.toml'dan okur.
- Sürüm iş akışı, etiket `v` + o sürüm değilse (ör. Cargo.toml `0.1.0` iken etiket `v0.1.0`
  değilse) **başarısız olur**.

## Sonuçlar

Olumlu:

- Küçük kurulum dosyası (WebView2 sistemde hazır), yönetici izni gerekmez.
- Güncellemeler imzalıdır; imza doğrulaması kapatılamaz.
- Sunucu yok: tüm dağıtım GitHub Releases üzerinden; "veri PC'den çıkmaz" ilkesiyle uyumlu
  (indirme analitiği yok).
- Taslak sürüm akışı, sahibin kontrolü olmadan hiçbir sürümün kullanıcılara ulaşmamasını sağlar.

Olumsuz / riskler:

- İmzasız ikili dosyalar SmartScreen uyarısı gösterir; Smart App Control açık bir PC'de çalışmaz.
- İmza anahtarının kaybı, kurulu kopyaların otomatik güncellenmesini kalıcı olarak bitirir —
  bu yüzden üç yerde yedek zorunludur.
- Dondurulmuş tanımlayıcılar ilk sürümden sonra değiştirilemez.
- Windows bildirimleri yalnızca kurulu uygulamada tam çalışır; geliştirme sırasında bildirim
  kimliği PowerShell olarak görünür.

## Gözden geçirme

- v2'de: Microsoft Store (MSIX) kanalı ve kod imzalama (DIST-01) yeniden değerlendirilir.
- Uygulama başka kullanıcılara dağıtılmaya başlamadan önce: imzalama zorunlu hale gelir.
- Smoke S-0 Akıllı Uygulama Denetimi'ni "Açık" bulursa: bu karar hemen yeniden açılır.
